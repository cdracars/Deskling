# Cyber Gremlin: first animation preview

The first opt-in runtime character pack for Deskling's selected public mascot.
It contains an unchanged AI-generated transparent PNG sheet and a schema-1
manifest. No executable code or provider configuration is part of the pack.

## Try it

Build the engine using the [repository instructions](../../../README.md), then
run from the repository root inside a composited X11 desktop:

```sh
sh scripts/run-gremlin.sh
```

The equivalent direct command is:

```sh
./target/release/deskling-desktop --pack assets/characters/cyber-gremlin/pack.json
```

Keep `pack.json` and `sprites.png` together. The launcher works from any current
directory and accepts normal engine options, for example `--reset-position`.
It does not build, install, configure autostart, or open a browser. Launching the
engine without a pack still shows its diagnostic shape.

For an isolated trial that does not read or change existing Deskling settings:

```sh
trial_config=$(mktemp -d)
printf 'Trial settings: %s\n' "$trial_config"
XDG_CONFIG_HOME="$trial_config" sh scripts/run-gremlin.sh --reset-position
```

Quit from the right-click controls. Trial settings stay in the printed system
temporary directory if you choose to inspect or remove them afterward; no
existing settings are deleted.

## What to expect

- Resting idle holds the first frame still.
- Blink uses four distinct poses at 8 fps.
- A single click plays four wave poses at 5 fps, then returns to idle.
- Automatic behavior favors blinks over waves, starts after about 5–10 seconds,
  and waits 20–45 seconds between later gestures. It does not move the window.
- Right-click controls allow manual animation, pause and size selection. Choosing
  an animation manually disables automatic behavior until it is re-enabled.

The sheet is 1774 × 887 pixels, with four columns and two rows of 443 × 443
cells. Two right-edge pixel columns and one bottom pixel row are unused padding. Row 0
contains the blink sequence and row 1 the wave sequence. The visible foot
baseline is aligned across all eight poses. There are small generated changes
in details, so this is a first testable animation, not final polished artwork.

## Verification boundary

The verification script loads this real pack through the native Qt loader,
checks decoded frame visibility/transparency and distinct frame data, and runs
an offscreen RGBA smoke render of its first idle frame. That smoke render is
not a desktop recording and does not verify native X11 compositing, dragging,
stacking, or the appearance of moving frames. Those need an on-desktop check.

## Artwork provenance and reuse

Generated with AI from Deskling's selected [static mascot](../../mascot/README.md).
The sheet is included for testing this project mascot. Artwork reuse terms have
not yet been specified. The repository's MIT license covers the engine and its
diagnostic geometry, not this artwork; do not assume the PNG is MIT licensed.
