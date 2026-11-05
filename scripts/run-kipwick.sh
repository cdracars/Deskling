#!/bin/sh
# Launch the bundled Kipwick alpha pack; no build, install or settings migration.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
binary="$root/target/release/deskling-desktop"
if [ ! -x "$binary" ]; then
    printf '%s\n' 'Deskling is not built yet. From the repository root, run:' >&2
    printf '%s\n' '  sh scripts/check-prerequisites.sh' '  cargo build --offline --locked --release' >&2
    exit 1
fi
exec "$binary" --pack "$root/assets/characters/kipwick-alpha/pack.json" "$@"
