# Verification boundaries

This source branch is an early engine extraction, not a stable release. Read the CI result for the exact commit you are using.

## Automated checks

- Python source/resource/schema checks and optional Node execution of extracted QML scheduler helpers.
- Rust core and C ABI boundary tests.
- Native QtCore/QtGui pack-loader tests using temporary generated test images, not bundled character artwork.
- A release build and offscreen item-rendering smoke test with detailed pixel/format diagnostics and a five-second total deadline.

The development workspace could execute Python/Node checks and source review but did not have Rust or Qt development tools. Native build, loader tests and smoke verification are delegated to CI/target runs, and must not be described as passed without their actual results.

The smoke test uses `QQuickItem::grabToImage()` because Qt 6.4's offscreen software window backing store is RGB32. It tests the rendered item, including alpha, rather than making an impossible alpha assertion on a non-alpha window screenshot. It does not validate X11 compositing or window-manager behavior.

## Manual desktop acceptance

1. On a composited X11 desktop, check the diagnostic shape has no opaque rectangular background, remains reachable, and does not repeatedly steal focus.
2. Drag quickly and across monitors. Release/cancel should leave it responsive. Test saved position, scale and `--reset-position` after monitor changes.
3. Wait through several automatic gestures. Each should be bounded and return to idle. Test manual selection, toggling automatic behavior, pause/resume, wave and drag interruptions.
4. Load a valid user-supplied pack with different cell sizes and animation order. Confirm its idle/interaction mappings and row crops. Invalid/missing/out-of-bounds packs must fail with an actionable error.
5. Open/close controls repeatedly. Test keyboard shortcuts only in the appropriate focused window. Quitting should leave no companion process behind.
6. Configure an ordinary HTTPS shortcut and verify the intended destination in your normal browser. Empty Save clears it. Confirm no provider connection or embedded login is implied.

Report the OS, desktop/window manager, Qt version, scale factor, exact command and error text. Do not include cookies, credentials, tokens or private artwork unless intentionally sharing them with the recipient.
