#!/bin/sh
# Run from any directory. All build/test output stays in this source tree.
set -eu
cd "$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
python3 -m unittest discover -s tests -v
cargo test --offline --locked -p deskling-core
cargo test --offline --locked -p deskling-desktop
mkdir -p test-output
# Standard system Qt paths; use the same compiler family as the application.
${CXX:-c++} -std=c++17 -fPIC -Wall -Wextra \
    tests/pack_test.cpp crates/deskling-desktop/native/pack.cpp \
    $(pkg-config --cflags --libs Qt6Core Qt6Gui) -o test-output/pack-tests
./test-output/pack-tests
cargo build --offline --locked --release
QT_QPA_PLATFORM=offscreen QT_QUICK_BACKEND=software \
    ./target/release/deskling-desktop --smoke-test --capture test-output/deskling-smoke.png
