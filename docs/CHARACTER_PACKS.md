# Local character packs, schema 1

A pack is a JSON manifest and one PNG beside it. It contains data, not QML, JavaScript, commands, URLs or provider configuration. Keep your own/private packs outside the repository. The engine's license does not grant rights to anyone else's character or artwork.

The repository includes one opt-in [Cyber Gremlin preview](../assets/characters/cyber-gremlin/README.md), launched after building with `sh scripts/run-gremlin.sh`. It is separate from the engine's resource bundle and has its own artwork notice. The diagnostic shape remains the default when no pack is selected.

Copy `examples/pack.template.json` into your own directory, supply your own `sprites.png`, and update the values to match its real pixel layout. The template alone is not a complete runnable pack; no corresponding artwork is bundled. Running without `--pack` uses the built-in diagnostic shape instead.

Required fields:

- `schemaVersion`: 1.
- `id`: lowercase slug, up to 64 characters, such as `my-character`.
- `name`: display text, up to 128 characters.
- `spriteSheet`: PNG basename in the same directory. Paths, URLs and symlinks resolving outside that directory are rejected.
- `cellWidth`, `cellHeight`: integer pixels, 16–512 each. All frames use this cell size.
- `animations`: 1–32 entries. Each has a unique lowercase `id`, display `label`, zero-based `row` (0–255), `frames` (1–64) and integer `fps` (1–30). Frames start at column zero on each row. Every referenced cell must fit the actual PNG.
- `idle`: an existing animation ID.

Optional fields:

- `interactions`: `wave`, `dragLeft`, `dragRight`, each referencing an existing animation ID. Missing interactions fall back to idle.
- `behaviors`: up to 32 `{animation, loops, weight}` entries. `loops` is 1–10; `weight` is 1–100. Omit this array for manual/idle-only operation. Weights select decorative actions after quiet gaps; they do not grant any system capability.
- `author` and `license`, or those same string fields under `metadata`: provenance supplied by the pack author. The engine does not verify or infer rights from them.

Unknown fields and invalid numeric types are rejected. Display text cannot contain control characters or rich-text angle brackets. Manifest size is limited to 64 KiB. PNG data is limited to 16 MiB, 16384 pixels on either side and 16777216 total pixels. The loader checks the PNG signature/dimensions before decoding, then requires a successful full decode.

Transparent padding is needed for a shaped desktop character. The smoke test pauses on the idle animation's first frame and requires at least one visibly opaque pixel and one fully transparent pixel in its rendered item. A fully blank or fully opaque first frame will fail that test even if the file is otherwise a valid PNG.

```sh
./target/release/deskling-desktop --pack /path/to/my-character/pack.json
```

Changing a pack does not execute code. Nevertheless, use trusted files and an up-to-date Qt image stack; image decoding is not a separate operating-system sandbox. Pack editing while the app is running is not supported. Restart after replacing a manifest or sheet.
