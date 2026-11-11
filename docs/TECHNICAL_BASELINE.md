# Deskling technical baseline

**Status:** Current-system evidence and optional technical diagrams. This is not a product plan or implementation authorization. ITIL 4 and CSDM 5 govern planning through the [service model](SERVICE_MODEL.md) and [constitution](PRODUCT_CONSTITUTION.md).  
**Code baseline:** `main` `b41ff3daf29967fa0b35b290f6d9ae1218393ec6`.  
**Optional diagram notation:** [C4 architecture](https://c4model.com/introduction): Context, Container, Component, Code. Containers in C4 mean applications/data stores, **not Docker**.

## Level 1 — System context (current)

```mermaid
flowchart LR
  user["Linux desktop user"] -->|runs and interacts| desktop["Deskling desktop companion"]
  artist["Character creator"] -->|edits/export rigs| studio["Deskling Avatar Studio / Factory"]
  studio -->|exports local PNG + JSON pack| store[("Local character files")]
  store -->|user selects pack| desktop
  desktop -->|native window, input, persisted settings| os["Linux X11 / Qt runtime"]
  desktop -->|only explicit open action| browser["System default browser"]
```

**System boundary:** the local desktop program, local authoring programs and inert local character assets. No account server, background monitoring, model, cloud service or integration adapter is implemented.

## Level 2 — Containers (current)

```mermaid
flowchart TB
  author["Creator"] --> studio["Avatar Studio — Qt6 Widgets GUI"]
  studio -->|invokes pose, rig, demo, export| factory["Avatar Factory — Rust CLI"]
  factory -->|reads| rig[("Authoring rig.json + layer PNGs")]
  factory -->|writes| pack[("Native pack.json + sprites.png")]
  user["User"] --> desktop["Deskling Desktop — Rust entry + Qt Quick/QML"]
  desktop -->|validates/reads| pack
  desktop -->|position, scale, optional shortcut| settings[("Local user settings")]
  desktop -->|X11 display/input| linux["Linux X11 session"]
  desktop -->|user requested URL| browser["Default HTTPS browser"]
```

These are local programs and files, not network microservices. The **Factory** uses system zlib for PNG handling; the **desktop** dynamically links Qt. The editor is authoring-only; it does not need to remain open for runtime animation.

## Level 3 — Desktop runtime components (current)

```mermaid
flowchart LR
  cli["Rust CLI parser + URL rules"] --> ffi["Narrow C ABI entry"]
  ffi --> boot["Qt application/bootstrap"]
  boot --> loader["Strict native pack loader"]
  boot --> bridge["QSettings/window bridge"]
  boot --> qml["QML window + animation scheduler"]
  loader --> qml
  bridge <--> qml
  bridge --> sys["X11 window manager / browser"]
```

**Data boundary:** `--pack` is supplied explicitly. The loader validates the local manifest and PNG; pack content does not reach the Rust CLI as an executable instruction. QML owns decorative scheduling; the bridge owns native UI/settings behavior and the optional browser action.

## Level 4 — Code and concrete evidence

| Architectural responsibility | Verified repository location |
| --- | --- |
| CLI, URL parsing, core Rust rules | [`crates/deskling-core/src/lib.rs`](../crates/deskling-core/src/lib.rs) |
| Desktop Rust entry, C ABI invocation | [`crates/deskling-desktop/src/main.rs`](../crates/deskling-desktop/src/main.rs) |
| Qt boot, native bridge, local state and smoke capture | [`crates/deskling-desktop/native/bridge.cpp`](../crates/deskling-desktop/native/bridge.cpp) |
| Manifest validation / image loading | [`crates/deskling-desktop/native/pack.cpp`](../crates/deskling-desktop/native/pack.cpp) |
| Animation state, interactions, controls | [`crates/deskling-desktop/qml/Main.qml`](../crates/deskling-desktop/qml/Main.qml) |
| Rust rig/export/PNG CLI | [`tools/avatar-factory/`](../tools/avatar-factory/) |
| Qt visual editor + authoring model | [`tools/avatar-studio/`](../tools/avatar-studio/) |
| Automation and native smoke verification | [`scripts/verify.sh`](../scripts/verify.sh) and [CI](../.github/workflows/ci.yml) |

No invented classes/services are required to tell the current code story. A deep code diagram is not warranted until a component is complex enough to justify it.

## Trust boundaries and architectural risks

- Local pack input is untrusted data. Continue strict local-only paths, manifest schema limits, PNG validation and resource caps. Avoid live code/plugin execution inside a pack.
- The native renderer and Qt image stack are not OS sandboxes. An invalid file must fail closed rather than fall through to a dangerous path.
- `QSettings` currently persists link, scale and position. Proposed selected-pack persistence needs validation against missing/symlink/changed files and clear behavior for personal data exposure in error messages.
- Real X11 transparency, stacking, keyboard focus, dragging and smoothness are **not proven** by offscreen CI. Capture a real desktop test and environment details.
- Character assets have separate licensing from MIT application code. Record rights before distribution.
- Pack format schema 1 is the current compatibility boundary; preserve loader behavior and regression fixtures.

## Planning references

Potential changes are evaluated in the [opportunity roadmap](FEATURE_ROADMAP.md) and [service decision register](SERVICE_MODEL.md). They are not assigned to implementation iterations. Existing code diagrams and repository links remain useful evidence without making C4 the governing plan.
