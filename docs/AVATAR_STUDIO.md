# Deskling Avatar Studio v0.3 (Linux/X11)

Avatar Studio is a **local Qt6 Widgets visual editor** for the Rust Avatar Factory. It is a standalone authoring tool, not a runtime character pack and not a cloud service. The Rust engine renders every preview and export; the editor never repaints original layer PNGs.

## Launch it

Requirements: Linux/X11, Rust/Cargo, C++17 compiler, system zlib and Qt6Widgets development libraries (Ubuntu: qt6-base-dev).

From the feature branch feat/avatar-studio-qt-v0:

    cargo run --offline --locked -p deskling-avatar-factory -- demo /tmp/deskling-demo-source
    sh scripts/run-avatar-studio.sh --rig /tmp/deskling-demo-source/rig.json

Or load your own project:

    sh scripts/run-avatar-studio.sh --rig /absolute/path/to/rig.json

The startup script builds the authoring tools locally; it performs no installation or sudo operation.

## Ready-to-run Linux test artifact

Each successful GitHub Actions run for PR #6 publishes a short-lived artifact named
**deskling-avatar-studio-linux-test-kit** containing the compiled Studio, the Rust
renderer, the Deskling desktop binary, an original editable rig under example/,
a generated native example-export/ pack, a screenshot, and a run.sh launcher.

On Ubuntu 24.04 x86-64 with Qt6 runtime modules and zlib installed:

1. Download the artifact ZIP from the PR workflow run's Artifacts section.
2. Extract it anywhere you can write, then open a terminal in the extracted folder.
3. Run the included launcher using sh run.sh. This works even when ZIP extraction
   clears executable bits: the launcher restores permissions for the included binaries.
4. Select Wave, drag the orange attachment joint for the right arm, adjust a rotation
   at an intermediate frame, Save, and Export Pack to a new destination.
5. Use File > Run exported pack in Deskling to inspect it live on X11.

The CI binaries dynamically link your system Qt and zlib libraries and were
built on Ubuntu 24.04. They are **test artifacts, not portable AppImages**:
they may not run on Ubuntu 22.04 or different distributions because of
library and glibc compatibility. In those environments, build locally
using the commands below or test in a supported Ubuntu 24.04 session.

## Test the complete editing loop

1. Open a rig project, or choose New Demo to create a wholly synthetic test character in a new folder.
2. Select a part in the left layer list. Drag its colored joint handle in the canvas, or change attachment/pivot coordinates precisely in the property panel. The source image remains unchanged.
3. Choose an animation and scrub frames with the slider or frame number. Press Play/Pause for motion; Rust renders frames from the authoritative rig.
4. Change an animation pose using translation, rotation, or independent X/Y scale. Editing an in-between frame inserts a keyframe, while preserving all other part poses at the same time.
5. Add or delete intermediate keyframes. The first and last frames are required.
6. Save (Ctrl+S) writes rig.json atomically; until then, previews use temporary JSON linked to the original layers.
7. Export Pack creates a fresh character-id-export folder containing native pack.json and sprites.png plus quality-report.json. The source rig and layer images remain untouched.
8. Use the File menu to start the exported pack in Deskling when its desktop binary exists. Verify actual native playback separately inside an X11 desktop.

## Rust preview command

The editor invokes the fast single-pose command rather than rebuilding the whole atlas after each edit:

    cargo run --offline --locked -p deskling-avatar-factory -- \
      pose /tmp/deskling-demo-source/rig.json wave 6 /tmp/wave-frame-6.png

The Rust rig pipeline retains CPU-parallel export for full sprite sheets. PNG+JSON character packs remain inert data, and the editor does not run code from packs.

## Automated verification

    sh scripts/verify.sh

CI tests Qt authoring transformations, insertion and removal of keyframes, save/reopen, Rust single-frame rendering, native rig and pack generation, and an offscreen Qt Studio initial preview. CI artifacts contain an editor screenshot and the editable synthetic rig. This is not a claim of live X11 compositing or end-user visual approval.

## Boundaries

The first release provides a practical visual editor, not a commercial illustration suite. It does not yet include image painting, automatic layer extraction, skinned meshes, inverse kinematics, arbitrary curve editors, automated seam repair, undo/redo, or an approved second mascot. This is an editing and packaging tool for artists who supply complete clean layer artwork.

**PR stacking:** Studio PR #6 is based on rig PR #5, which is based on pack-builder PR #4. Review and merge in order. The main branch is not modified.
