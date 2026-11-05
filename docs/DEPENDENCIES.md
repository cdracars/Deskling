# Dependencies and licensing

Reviewed: 2026-11-05. This is a practical compliance record, not legal advice or a certification of any future binary release.

## Scope and project license

Deskling's original Rust, C++ and QML source, including its code-drawn geometric test placeholder, is offered under the repository's MIT license. That license does **not** relicense Qt, language runtimes, system libraries, or externally supplied assets.

This repository distributes source, not compiled releases or Qt/runtime libraries. It includes a separately documented [static project mascot](../assets/mascot/README.md) and [Kipwick application icon](../assets/icons/README.md). The eight icon exports are embedded when the application is built; their reuse terms have not yet been specified and must be resolved before redistributing a binary containing them. The geometric placeholder remains a test fixture, not a runtime mascot pack. No fonts or runtime sprite sheets are bundled. Any future asset needs its own provenance and licensing review before inclusion.

## Dependency inventory

- **Rust:** no third-party Cargo dependencies are declared; the application uses the Rust standard library. Rust is generally MIT OR Apache-2.0, with separately licensed portions. A binary's standard-library content still needs review even with an empty Cargo dependency list. Use the exact toolchain release's `COPYRIGHT-library.html`, not a blanket claim that all Rust output is MIT. See [Rust's copyright record](https://github.com/rust-lang/rust/blob/main/COPYRIGHT).
- **Qt 6:** native shared libraries `Qt6Core`, `Qt6Gui`, `Qt6Qml`, and `Qt6Quick`; QML runtime modules include `QtQuick.Controls`, `QtQuick.Layouts`, `QtQuick.Templates`, `QtQuick.Window`, `QtQml.WorkerScript`, and `QtQml.Models`. These come from the Qt Core/GUI, QML, Quick, and Quick Controls families. The intended open-source route is **LGPLv3**, using dynamically linked, replaceable system Qt libraries. Qt also offers GPL alternatives and commercial licenses; the applicable package and source-file notices control.
- **C++ runtime:** dynamically linked. GNU `libstdc++` uses GPLv3 with the **GCC Runtime Library Exception 3.1**, which permits qualifying independently authored applications to retain their own license when its conditions are met. The runtime is not MIT. If another runtime is selected, review its actual license instead. See [GCC's license and exception](https://gcc.gnu.org/onlinedocs/libstdc++/manual/license.html).
- **Platform dependencies:** Qt may load platform, image, style, graphics, X11/XCB and other system plugins/libraries transitively. This source-level list is not a complete runtime SBOM.

Qt's official module license records: [Core](https://doc.qt.io/qt-6.8/qtcore-index.html#licenses-and-attributions), [GUI](https://doc.qt.io/qt-6.8/qtgui-index.html#licenses-and-attributions), [QML](https://doc.qt.io/qt-6.8/qtqml-index.html#licenses-and-attributions), [Quick](https://doc.qt.io/qt-6.8/qtquick-index.html#licenses-and-attributions), [Controls and Templates](https://doc.qt.io/qt-6.8/qtquickcontrols-index.html), and [Layouts](https://doc.qt.io/qt-6.8/qtquicklayouts-index.html). These references establish the license approach; they do not pin the installed Qt version. Verify the exact build's notices before release.

## Qt boundaries

MIT for Deskling-owned source is compatible with the intended LGPL approach only when applicable dependency obligations are met. Dynamic linking is helpful, but is not by itself complete compliance.

Do not assume every Qt module has an LGPL option. For example, Qt 6.8 lists Quick Timeline, Quick 3D, Lottie Animation and Qt Qml Compiler among GPLv3/commercial modules. Adding one needs a fresh review of the combined distribution; MIT on Deskling's own files does not override GPL conditions. Commercial Qt use depends on the license actually held. See the [Qt licensing overview](https://doc.qt.io/qt-6.8/licensing.html). Tool licensing and generated-output exceptions also need checking if the build tooling changes.

## Required review before distributing binaries

1. Record exact Rust, Qt, compiler/runtime, platform plugin and QML module versions and build options. Inspect the produced executable and loaded plugins, not just Cargo metadata. Use the installation's [Qt SBOM](https://doc.qt.io/qt-6/sbom.html) where available.
2. Include the required copyright notices and license texts for all included material. For an LGPL combined work, prominently identify Qt use and include both LGPLv3 and GPLv3 texts. If the app displays copyright notices, include the required Qt notice and license reference there too. Preserve applicable Rust/runtime notices.
3. Preserve users' ability to replace the LGPL libraries with compatible modified versions and run the resulting application. Do not impose terms that prohibit modification of those library portions or reverse engineering to debug such modifications. Supply installation information where the license requires it. Review static linking separately.
4. If shipping Qt libraries or plugins, arrange compliant delivery of their complete corresponding source, including modifications, using a method allowed by the applicable licenses. Merely linking to an upstream home page is not a release compliance plan. Distinguish a system-library dependency from a bundle that redistributes those libraries.
5. Audit Qt's embedded third-party components and every bundled runtime, font and asset. Carry the exact required notices and any source obligations for the selected build. Qt's [third-party inventory](https://doc.qt.io/qt-6.8/licenses-used-in-qt.html) is a starting point, not a substitute for examining the actual package.
6. Verify distribution terms do not restrict recipients' license rights. Complete this review before publishing installers, archives, AppImages, containers or other binary packages.

Controlling reference: [LGPLv3, especially section 4](https://www.gnu.org/licenses/lgpl-3.0.html). Practical guidance: [Qt's LGPL obligations](https://www.qt.io/development/open-source-lgpl-obligations). A source-only branch does not complete this future binary-release checklist.
