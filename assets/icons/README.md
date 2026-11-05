# Kipwick application icon

![Kipwick application icon](hicolor/128x128/apps/deskling.png)

The selected Kipwick portrait is Deskling's application icon. It is distinct from the wider [static project mascot](../mascot/README.md) and is not a runtime character pack.

- `source/kipwick-icon.png`: the unchanged 1254 × 1254 RGB source image.
- `hicolor/<size>x<size>/apps/deskling.png`: Linux icon-theme exports at 16, 24, 32, 48, 64, 128, 256 and 512 pixels.
- The original dark background and rounded neon frame are preserved. These are opaque PNGs, not transparent cutouts.

The eight exports are compiled into Qt resources and assigned as the application/window icon before the QML windows are created. No external icon path or current working directory is needed for a direct build launch. The original master is kept for future exports, not embedded in the binary.

The face and cyan/purple silhouette remain recognizable at small sizes; fine fur, goggles and antenna detail naturally diminishes at 16–24 pixels. No simplified or redesigned mark has been substituted.

## Re-export

With Pillow available in an asset-maintenance Python environment, run:

```sh
python3 scripts/export-icons.py
```

This uses Lanczos downsampling from the master at each size, without cropping, recoloring or redrawing. Pillow is not required to build, run or test Deskling. Initial exports used Pillow 13.0.0.

## Optional Linux app-menu entry

`packaging/linux/deskling.desktop` uses the icon name `deskling`, matching the hicolor filenames and Qt desktop-file identity. Building or running Deskling does **not** install this entry or enable autostart.

After building, users who want a menu entry can run the following commands themselves from the repository root. They copy the binary, desktop entry and icons into the current user's local directories; no sudo is needed:

```sh
data_home="${XDG_DATA_HOME:-$HOME/.local/share}"
install -Dm755 target/release/deskling-desktop "$HOME/.local/bin/deskling-desktop"
install -Dm644 packaging/linux/deskling.desktop "$data_home/applications/deskling.desktop"
for size in 16 24 32 48 64 128 256 512; do
    install -Dm644 "assets/icons/hicolor/${size}x${size}/apps/deskling.png" \
        "$data_home/icons/hicolor/${size}x${size}/apps/deskling.png"
done
```

The desktop session's `PATH` must include `$HOME/.local/bin`; otherwise the launcher may be hidden by `TryExec` or fail to start. Logging out and back in may be needed after changing the session's `PATH` or for an app menu to refresh. Runtime Qt libraries and QML modules are still required.

The launcher starts the normal no-pack diagnostic shape. It does not select or install a character pack. The existing `Qt.Tool` windows are preserved, so window managers may omit Deskling from the taskbar, dock or Alt-Tab list. An application icon does not change that behavior, and final menu/window-icon appearance needs testing on a real desktop.

## Artwork licensing

Reuse terms for the selected image and its resized derivatives have not yet been specified. The repository's MIT license covers Deskling-owned code and diagnostic geometry, not this artwork. Do not assume these icons are MIT licensed. The compiled application includes the resized artwork, so resolve its reuse terms before redistributing binaries containing it.
