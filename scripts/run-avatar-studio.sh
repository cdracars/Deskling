#!/bin/sh
# Build locally without installing Deskling or modifying desktop settings.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$root"
if ! pkg-config --exists Qt6Widgets; then
  echo "Qt6 Widgets development libraries are required (Ubuntu: qt6-base-dev)." >&2
  exit 1
fi
cargo build --offline --locked --release -p deskling-avatar-factory
mkdir -p target/release
c++ -std=c++17 -O2 -Wall -Wextra -fPIC -pthread \
    tools/avatar-studio/main.cpp \
    $(pkg-config --cflags --libs Qt6Widgets) \
    -o target/release/deskling-avatar-studio
exec "$root/target/release/deskling-avatar-studio" \
    --renderer "$root/target/release/deskling-avatar-factory" \
    --deskling "$root/target/release/deskling-desktop" "$@"
