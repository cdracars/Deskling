"""Dependency-free checks for icon exports and the Linux desktop entry."""
import configparser
import hashlib
from pathlib import Path
import struct
import unittest

ROOT = Path(__file__).resolve().parents[1]
SIZES = (16, 24, 32, 48, 64, 128, 256, 512)


def png_size(path):
    header = path.read_bytes()[:24]
    if header[:8] != b"\x89PNG\r\n\x1a\n" or header[12:16] != b"IHDR":
        raise ValueError(f"Not a PNG with an IHDR: {path}")
    return struct.unpack(">II", header[16:24])


class IconTests(unittest.TestCase):
    def test_selected_master_and_exports_have_expected_dimensions(self):
        master = ROOT / "assets/icons/source/kipwick-icon.png"
        self.assertEqual(png_size(master), (1254, 1254))
        self.assertEqual(hashlib.sha256(master.read_bytes()).hexdigest(),
                         "aeca1f3d2322b8de574ef3d8713cc658946e51f59f92420758d91f812110afb7")
        for size in SIZES:
            self.assertEqual(png_size(ROOT / f"assets/icons/hicolor/{size}x{size}/apps/deskling.png"),
                             (size, size))

    def test_desktop_entry_matches_embedded_application_identity(self):
        entry = configparser.ConfigParser(interpolation=None)
        entry.read(ROOT / "packaging/linux/deskling.desktop")
        desktop = entry["Desktop Entry"]
        self.assertEqual(desktop["Type"], "Application")
        self.assertEqual(desktop["Exec"], "deskling-desktop")
        self.assertEqual(desktop["TryExec"], "deskling-desktop")
        self.assertEqual(desktop["Icon"], "deskling")
        self.assertEqual(desktop["Terminal"], "false")
        source = (ROOT / "crates/deskling-desktop/native/bridge.cpp").read_text()
        self.assertIn('setDesktopFileName(QStringLiteral("deskling"))', source)
        self.assertIn("setWindowIcon(applicationIcon())", source)
        self.assertLess(source.index("setWindowIcon(applicationIcon())"), source.index("engine.load("))
        self.assertIn("window->icon().pixmap", source)
        build = (ROOT / "crates/deskling-desktop/build.rs").read_text()
        self.assertIn("cargo:rerun-if-changed=../../assets/icons/hicolor", build)


if __name__ == "__main__":
    unittest.main()
