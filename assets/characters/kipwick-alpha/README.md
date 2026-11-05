# Kipwick alpha animation pack

An **unfinished development pack** for the public Deskling mascot, separate from
its detailed static illustration. This simplified character uses a fixed source
image and controlled motion to reduce stationary texture flicker.

## Try it without installing

Build this branch using the repository instructions, then run:

```sh
sh scripts/run-kipwick.sh
```

For isolated test preferences, run from this checkout:

```sh
TEST_SETTINGS=$(mktemp -d)
XDG_CONFIG_HOME="$TEST_SETTINGS" sh scripts/run-kipwick.sh --reset-position
```

Neither command installs Deskling or replaces another application. The second
uses a fresh temporary settings directory. Quit another Deskling instance first.
Click to wave, drag to move, and right-click for individual animation previews,
Automatic behavior, Pause and Quit. Default launch without `--pack` still shows
the artwork-free diagnostic shape. The earlier `run-gremlin.sh` study remains
available separately; `run-kipwick.sh` selects this newer pack.

## Layout and motion

The RGBA atlas is 6912×2304: 24 columns and 8 rows of 288×288 cells. It fits the
existing 16,777,216-pixel and 16 MiB limits. Source motion frames were preserved at
362 pixels before this explicitly downsampled native derivative was assembled.

| Row | Animation | Frames | Requested fps | Nominal duration |
| --- | --- | --- | --- | --- |
| 0 | Breathing idle | 24 | 12 | 2 s |
| 1 | Curved blink | 24 | 40 | 0.6 s |
| 2 | Friendly wave | 24 | 20 | 1.2 s |
| 3 | Curious tilt | 24 | 12 | 2 s |
| 4 | Gentle bounce | 24 | 12 | 2 s |
| 5 | Sleepy nod | 24 | 10 | 2.4 s |
| 6 | Drag lean left | 24 | 12 | 2 s |
| 7 | Drag lean right | 24 | 12 | 2 s |

These are timeline frames, not 24 independent drawings: neutral endpoints and
some recurring positions repeat. The wave contains genuine warped intermediate
arm shapes. Blink, tilt and other motions reuse fixed artwork rather than
regenerating the entire character. Requested fps is not measured performance;
Qt timer scheduling and rendering load can stretch the nominal duration.

## Known alpha limitations

- Wave arm contours and palm/back artwork still change between source poses.
  Manual shoulder and occlusion seams need further cleanup.
- Closed-eye fill is smoother than surrounding face shading.
- Bounce is rigid translation, sleepy is a 2D dip, and drag leans rotate the feet.
- Interrupting breathing at another phase can produce a small transition snap.
- Native desktop smoothness, compositing, stacking and interaction need testing.
  Offscreen RGBA smoke tests do not establish live X11 behavior or frame rate.
- The atlas is near the pixel cap; lower-memory device performance needs review.

Tests cover manifest/layout limits, actual PNG decode, visible pixels,
transparent cell padding, opaque torso regions, fixed wave head/feet, and bounded
scheduler completion. They are not a judgment that the artwork is finished.
Keep the development PR in draft until the desktop test is reviewed.

## Artwork terms

The engine's MIT license does **not** grant reuse rights for this character's
artwork. No separate artwork redistribution/reuse license has been assigned.
The pack contains inert JSON and PNG data, with no code, links, credentials,
telemetry or private character assets.
