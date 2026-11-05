# Deskling

An X11-first Linux desktop character engine, built with Rust and Qt Quick/QML.

![Deskling mascot: a charcoal-and-purple maker creature with cyan eyes, goggles and glowing antennae, working beside a tiny laptop.](assets/mascot/deskling-mascot.png)

Deskling's **static project mascot**. This illustration is not an animated sprite sheet or a runtime character pack. See [mascot artwork](assets/mascot/README.md).

**Early source prototype.** The engine, character data, and optional integrations are separate concerns. This first branch implements the local window, declarative sprite packs, and decorative behavior. Cloud/local agent connectors and an in-app chat interface are **not implemented**.

The unfinished [Kipwick alpha animation pack](assets/characters/kipwick-alpha/README.md) is included for development testing, alongside the earlier [Cyber Gremlin study](assets/characters/cyber-gremlin/README.md). Both are opt-in local packs. Launching without a pack still shows a code-drawn **diagnostic shape**, not the mascot above. Engine and artwork licenses remain separate.

## Current slice

- Transparent frameless X11 window, drag to move, always-on-top hint.
- Local JSON + PNG character packs with strict validation and bounded animation behavior.
- Mostly idle, with occasional short weighted gestures. Manual selection and pause remain available.
- Right-click controls, size and position persistence, and an optional explicit HTTPS browser shortcut.
- Character size is 160 logical pixels high at 100%, independent of sprite resolution. Existing size percentages are preserved (75% = 120px, 200% = 320px); aspect ratio is retained.
- No telemetry, account/session access, API keys, background activity monitoring, executable pack code, or remote asset loading.

Only X11 is targeted. Compositor/window-manager behavior still needs desktop testing; this is not a claim of support for all Linux desktops. Wayland support, connectors, audio, packaging, roaming and click-through behavior are future work.

## Build without installing Deskling

Requirements: Rust/Cargo 1.70+, C++17 compiler, `ar`, `pkg-config`, Qt 6.4+ development tools (`moc`, `rcc`), and the Qt Quick/QML runtime modules used by the UI. Python 3 is used for source tests; Node.js is optional for scheduler tests.

On Ubuntu 24.04+ or Debian 13, install system prerequisites yourself if needed:

```sh
sudo apt update
sudo apt install cargo rustc build-essential pkg-config python3 \
  qt6-base-dev qt6-base-dev-tools qt6-declarative-dev qt6-declarative-dev-tools \
  qml6-module-qtquick qml6-module-qtquick-window qml6-module-qtquick-controls \
  qml6-module-qtquick-layouts qml6-module-qtquick-templates \
  qml6-module-qtqml qml6-module-qtqml-models qml6-module-qtqml-workerscript
```

Then, from the repository root:

```sh
sh scripts/check-prerequisites.sh
cargo test --offline --locked -p deskling-core
cargo build --offline --locked --release
./target/release/deskling-desktop
```

Run as your normal user inside an X11 session, not through sudo. No app installation or autostart configuration is performed. With system tools installed, there are no Cargo crates to download. The executable dynamically links Qt and is not a portable standalone binary.

For a character you supply:

```sh
./target/release/deskling-desktop --pack /path/to/character/pack.json
```

To try Kipwick after building:

```sh
sh scripts/run-kipwick.sh
```

It breathes, previews short gestures, and waves on a single click. Right-click opens
controls and Quit. See the [alpha pack's testing notes](assets/characters/kipwick-alpha/README.md)
for isolated trial settings and the native-desktop checks still needed.

See [character packs](docs/CHARACTER_PACKS.md). The separate example manifest is a template and intentionally has no accompanying artwork.

## Application icon and optional launcher

The selected **Kipwick portrait** is embedded as Deskling's application/window icon in eight sizes, from 16 to 512 pixels. It is separate from the static mascot illustration above and does not replace the diagnostic shape or load a character pack.

A standard Linux desktop entry and hicolor icon files are included for users who want an app-menu launcher. Nothing is installed automatically. See [icon files, previews and optional setup](assets/icons/README.md). Deskling keeps its existing tool-window behavior, so a taskbar or dock icon is not guaranteed by every window manager.

## Controls and behavior

- Drag to move. Single-click plays the pack's wave interaction. Double-click opens your configured link.
- Right-click opens controls. Quit is in that panel.
- **Automatic behavior** is on when the pack has behaviors. The first action starts around 5–10 seconds later; subsequent idle gaps vary from 20–45 seconds. Actions are bounded by the pack's loop counts and return to its idle animation. Timing may be slower under load.
- Choosing an animation previews it once, then returns to idle. It never changes the Automatic behavior switch. Turn that switch off to keep only idle; pause stops all playback and its countdown.
- Automatic behavior never moves the window, takes focus, or claims to represent an agent's activity.
- Ctrl+O opens the configured link; Ctrl+comma opens controls; Ctrl+Q quits, while the character window is focused. Space pauses. Escape closes controls.
- `--reset-position` recovers a window moved off-screen. Quit another running copy first; single-instance handling is not implemented.

The transparent rectangular window still receives mouse input within its bounds. Window managers can override the always-on-top hint.

## Browser shortcut, not a connector

Paste an ordinary HTTPS URL in controls and save it, or use the session-only `--link-url URL` argument. Empty Save clears the stored link. The system's normal browser handles navigation and login. Deskling cannot tell whether the destination loaded successfully.

The deliberately narrow validator accepts ASCII DNS hostnames and opaque URI paths. Ports, IP literals, credentials, query strings and fragments are rejected. Never include tokens or other secrets in a path: syntax validation cannot detect them. No URL is supplied by a character pack, and no request is sent until you explicitly open the shortcut.

Settings contain only the saved link, scale and position, normally in `$XDG_CONFIG_HOME/Deskling/desktop.ini` or `~/.config/Deskling/desktop.ini`. The link is plain text with private file permissions, not encrypted. Character packs remain separate local files. No settings or private packs belong in this repository.

## Verify

```sh
sh scripts/verify.sh
```

This runs source/scheduler and launcher checks, Rust tests, C++ pack-validation tests, a release build and offscreen **item RGBA** smoke tests for the diagnostic shape, earlier gremlin study and Kipwick alpha pack. Node-backed scheduler tests explicitly skip when Node is absent. Smoke tests render character items, not X11 desktop captures, and therefore do not verify native transparency, stacking, dragging or interaction. See [verification](docs/VERIFICATION.md).

If Qt reports missing modules, install the named runtime modules; having Qt libraries alone is insufficient. A missing DISPLAY means you are outside the desktop session. An opaque background needs compositor/renderer diagnosis, not root privileges. Use `QT_QUICK_BACKEND=software` as a rendering diagnostic if needed.

## Architecture and license

- `deskling-core`: dependency-free Rust CLI and browser-link rules.
- `deskling-desktop`: Rust entry point, narrow C ABI, native Qt window/config glue, validated pack loader and QML renderer/scheduler.
- Character packs: inert JSON + local PNG data, independently owned and licensed.
- Connectors: future boundary only. No provider SDK, local server, socket, credential flow or agent protocol is implemented in this branch.

Deskling-owned code and diagnostic geometry are MIT licensed. Qt and runtimes retain their own licenses. The [static mascot artwork](assets/mascot/README.md), [earlier runtime study](assets/characters/cyber-gremlin/README.md) and [Kipwick alpha pack](assets/characters/kipwick-alpha/README.md) and the embedded [Kipwick icon](assets/icons/README.md) have separate, currently unspecified artwork reuse terms. See [dependency obligations](docs/DEPENDENCIES.md) before distributing binaries. No binary releases are included.
