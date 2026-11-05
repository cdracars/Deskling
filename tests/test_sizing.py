"""Execute actual QML size expressions; this is not native desktop rendering."""
from pathlib import Path
import json
import re
import shutil
import subprocess
import unittest

QML = (Path(__file__).resolve().parents[1] / "crates/deskling-desktop/qml/Main.qml").read_text()
NODE = shutil.which("node")


@unittest.skipUnless(NODE, "Node unavailable; QML size expression tests skipped")
class SizingTests(unittest.TestCase):
    def size(self, width, height, scale):
        base = re.search(r"readonly property int baseHeight: (.+)", QML).group(1)
        width_expression = re.search(r"^    width: (.+)", QML, re.M).group(1)
        height_expression = re.search(r"^    height: (.+)", QML, re.M).group(1)
        script = (f"const pet = {{baseHeight: {base}}};\n"
                  f"const characterPack = {{cellWidth: {width}, cellHeight: {height}}};\n"
                  f"const bridge = {{petScale: {scale}}};\n"
                  f"console.log(JSON.stringify([{width_expression}, {height_expression}]));")
        result = subprocess.run([NODE, "-e", script], text=True, capture_output=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        return json.loads(result.stdout)

    def test_default_is_independent_of_sheet_resolution(self):
        for cell_size in (64, 160, 362, 443, 512):
            with self.subTest(cell_size=cell_size):
                self.assertEqual(self.size(cell_size, cell_size, 1), [160, 160])

    def test_saved_scale_and_aspect_ratio_are_preserved(self):
        self.assertEqual(self.size(443, 443, .75), [120, 120])
        self.assertEqual(self.size(443, 443, 2), [320, 320])
        self.assertEqual(self.size(128, 160, 1), [128, 160])
        self.assertEqual(self.size(256, 512, 1), [80, 160])


if __name__ == "__main__":
    unittest.main()
