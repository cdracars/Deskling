# Deskling Avatar Factory (Rust alpha)

The **Avatar Factory** is an offline, CPU-based pack-authoring CLI, **not** a character/image generator yet. Its first milestone turns existing approved animation frames into loadable Deskling schema-1 packages, allows lossless frame extraction from older packs, and provides repeatable checks. The Deskling runtime is unchanged.

Source: `tools/avatar-factory/` | Native pack contract: [CHARACTER_PACKS.md](CHARACTER_PACKS.md)

## Build and run (Linux)

Requires Rust/Cargo and system zlib development/runtime (Ubuntu: `zlib1g-dev`), plus a working Deskling checkout. No external Rust crates, cloud services, image generation, or Qt are needed by the authoring CLI.

```sh
cargo test --offline --locked -p deskling-avatar-factory
cargo run --offline --locked -p deskling-avatar-factory -- \
  inspect assets/characters/kipwick-alpha/pack.json
```

Import an existing authored pack into an editable source project:

```sh
cargo run --offline --locked -p deskling-avatar-factory -- \
  extract assets/characters/kipwick-alpha/pack.json /tmp/kipwick-source
```

The source project contains `project.json` and individual RGBA8 images named
`frames/<animation-id>/0000.png`, `0001.png`, etc., all **the same dimensions**.
The first frame is index zero. Frames are always read in numeric order, not filesystem order.

Build a native pack and quality report from the source project:

```sh
cargo run --offline --locked -p deskling-avatar-factory -- \
  build /tmp/kipwick-source/project.json /tmp/kipwick-built

./target/release/deskling-desktop --pack /tmp/kipwick-built/pack.json
```

The export directory contains only `pack.json`, `sprites.png` and `quality-report.json`. It must be new or empty; **files are never overwritten** by the CLI. Do not reuse a directory from a previous build. As with the existing engine, validate on a target X11 desktop separately.

## Authoring contract

`project.json` is an authoring-only wrapper around the **exact native schema-1 manifest**:

```json
{
  "schemaVersion": 1,
  "pack": {
    "schemaVersion": 1,
    "id": "example-companion",
    "name": "Example Companion",
    "spriteSheet": "sprites.png",
    "cellWidth": 128,
    "cellHeight": 160,
    "idle": "idle",
    "animations": [
      {"id": "idle", "label": "Idle", "row": 0, "frames": 2, "fps": 8},
      {"id": "wave", "label": "Wave", "row": 1, "frames": 3, "fps": 12}
    ],
    "interactions": {"wave": "wave"}
  },
  "lockedRegions": {
    "wave": [{"x": 0, "y": 0, "width": 128, "height": 40}]
  }
}
```

Place **two** RGBA8 images in `frames/idle/0000.png` and `0001.png`; place **three** in `frames/wave/0000.png`, `0001.png` and `0002.png`. Each image must be 128×160. The `lockedRegions` entry requires the first forty rows of the wave sequence to remain **pixel-identical** across its frames; use this for body regions that must not flicker.

Every native manifest rule still applies: 16–512-pixel cells, 1–64 frames per animation, 1–60 fps, 1–32 animations, local PNG basename, bounded total atlas pixels, PNG bytes and manifest size, approved plain-text fields only. This wrapper is **never** copied into the native `pack.json`. Rows may be shared only when the same cell has identical pixel bytes in all overlapping animation references, as in the existing Cyber Gremlin idle/blink setup.

## Verification semantics

- **Structural hard failures:** bad/unknown JSON fields, bad dimensions, unsupported/invalid PNG, missing frames, invalid animation references, different pixels in a shared atlas cell, empty visible frames, locked-region changes, oversized atlas or output. Builder processes and validates all inputs before writing outputs.
- **Non-blocking warnings** in `quality-report.json`: touching any cell border, no fully transparent pixels, identical consecutive frames (often an intentional hold). Inspect prints warnings without altering the pack.
- **Not yet verified:** art quality, natural expressive motion, limb occlusion, consistency of unlocked textures, smooth animation transitions, native display compositor behavior or visual approval. A decoder pass is not artistic acceptance.
- **No guaranteed byte-identical imported PNG:** extracting and rebuilding decodes the source and deterministically re-encodes the atlas with RGBA8 and zlib level 6; decoded pixels and manifest semantics should match, though PNG file bytes and old right/bottom unused padding may differ.
- **Current codec restriction:** RGBA8, noninterlaced PNG only. Palette/RGB/16-bit/interlaced PNGs must be converted by the artist before import. Unlike the Qt loader, Avatar Factory intentionally does not support arbitrary PNG formats.

The saved Kipwick reproduction document explains why a fixed master texture, completed hidden anatomy, controlled geometry, per-pose approval and two-angle validation are important. Those artistic stages are future tooling, **not** features claimed by this CLI. The existing public Kipwick alpha and Cyber Gremlin packs are useful known-input fixtures. The approved private expressive-jump v0.4 is not imported or published by this change.

## Milestone 0.2: Rust layered rig renderer

The `rig` command renders **motion from one reusable set of RGBA image layers**, rather
than requiring a manually authored PNG for every frame. It resolves nested 2D
joints, interpolates keyframe transforms and composites layers in declared
front-to-back sequence. It produces the **same native Deskling schema-1**
`pack.json` and RGBA `sprites.png` as the earlier `build` command.

To see the complete workflow with original, procedurally drawn example art:

```sh
cargo run --offline --locked -p deskling-avatar-factory -- demo /tmp/deskling-rig-source
cargo run --offline --locked -p deskling-avatar-factory -- \
  rig /tmp/deskling-rig-source/rig.json /tmp/deskling-rig-built
cargo run --offline --locked -p deskling-avatar-factory -- \
  inspect /tmp/deskling-rig-built/pack.json
./target/release/deskling-desktop --pack /tmp/deskling-rig-built/pack.json
```

Choose **new empty** directories. `demo` generates a synthetic independent
character made of opaque body, arms, feet, antenna, eyes and mouth layers.
It exercises four motions: breathing, blink, waving and bounce. These are
engineering fixtures, not approved mascot designs and contain no private art.

### Rig contract

The `rig.json` format contains:

- `schemaVersion: 1`, `pack`: native schema-1 manifest as described above.
- `parts`: 1–64 ordered layers. Each is
  `{"id":"arm","image":"arm.png","parent":"body","pivot":[16,10],"at":[84,48]}`.
  `image` is a local RGBA8 PNG basename inside a sibling `layers/` folder.
  The `parent` field is optional or null for a root. Parents can appear
  later in the array, but cycles are rejected. **Array order determines
  draw order:** first part painted at the back, last at the front.
- `pivot`: the rotation/scaling center in that layer's image pixels.
  `at`: where that pivot is attached in the **parent image's pixel
  coordinates**, or canvas pixels for root layers. The image's untransformed
  local position is `at - pivot`. Nested transformations are inherited.
- `motions`: optional mapping from animation IDs to ordered keyframe
  arrays: `{"wave":[{"frame":0,"poses":{"arm":{"rotate":0}}},
  {"frame":7,"poses":{"arm":{"rotate":-90}}},
  {"frame":15,"poses":{"arm":{"rotate":0}}}]}`.
  Each motion must contain the first and last timeline frame. Angles
  are **degrees**, with shortest-arc interpolation. `dx`, `dy` are
  local translation in pixels; `rotate` is local angle; `scale`
  is uniform, while `scaleX` and `scaleY` override individual axes.
  Missing transforms at a keyframe mean the neutral pose, not hold-last.
- `lockedRegions`: optional named pixel rectangles that must match
  the first output frame across an animation.

Frames are independently composited with premultiplied-alpha bilinear
sampling. CPU worker threads process frames concurrently and one writer
assembles the atlas deterministically. The rig does **not** modify any
source PNG. It checks output visibility, transparency, image bounds,
locked regions and overlapping shared atlas cells. It rejects oversized
pack output and writes a separate quality report.

### Scope and important limits

This is a **2D affine cutout rig**, not an image-prompt-to-character service,
skeletal skin deformation engine, inverse kinematics solver, segmentation
tool or complete animation editor. Automatic mesh warping, seams/hidden
anatomy authoring, mesh weight painting, realistic cloth and expression
repaint are not implemented. All movable parts need complete clean art.
The source images remain authoritative; a distorted pose is not automatically
repaired by this generator.

The PNG codec accepts 8-bit RGBA noninterlaced input only. Pivots must be
within the image. Source and output filenames remain local to their own
directories. Runs use at most eight CPU workers; a later revision may
offer a configurable CPU count, animation curves, and smarter sampling.

## Next milestone

The layered renderer is implemented. Next, add an **image- and timeline-aware editor**, visual overlays against approved pose checkpoints, seam/opaque-anatomy authoring tools, and frame-level regression fixtures. Then apply the exact same rig contract to a **second independently approved character**. No reusable-avatar-generator claim until that second character and a live desktop test pass.

**Artwork rights remain independent** of Deskling's MIT code. The CLI does not change the copyright or license of the images it processes.
