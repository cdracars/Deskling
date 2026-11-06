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

## Next milestone

Implement an independent **Rust rig and pose engine** that can render from immutable RGBA layers, enforce complete opaque anatomy beneath moving limbs, interpolate pivots and transforms, and compare output pixels against frozen accepted poses. Then add a visual timeline editor that calls the Rust core. Only after a second character uses the same rigging path successfully should we call this a reusable avatar generator.

**Artwork rights remain independent** of Deskling's MIT code. The CLI does not change the copyright or license of the images it processes.
