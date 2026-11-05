"""Source checks only; native Qt validation is in pack_test.cpp and CI."""
from pathlib import Path
import json
import struct
import unittest
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[1]


class SourceTests(unittest.TestCase):
    def test_resource_graph_has_no_bundled_artwork(self):
        qrc = ROOT / "crates/deskling-desktop/native/resources.qrc"
        files = ET.fromstring(qrc.read_text()).findall(".//file")
        self.assertEqual([f.attrib["alias"] for f in files], ["Main.qml"])
        for entry in files:
            self.assertTrue((qrc.parent / entry.text).resolve().is_file())

    def test_template_is_explicit_data_not_complete_pack(self):
        pack = json.loads((ROOT / "examples/pack.template.json").read_text())
        self.assertEqual(pack["schemaVersion"], 1)
        self.assertIn(pack["idle"], [a["id"] for a in pack["animations"]])
        self.assertFalse((ROOT / "examples" / pack["spriteSheet"]).exists())

    def test_first_character_pack_has_real_sheet_and_safe_layout(self):
        directory = ROOT / "assets/characters/cyber-gremlin"
        pack = json.loads((directory / "pack.json").read_text())
        self.assertEqual(pack["schemaVersion"], 1)
        self.assertEqual(pack["id"], "cyber-gremlin")
        self.assertEqual(pack["spriteSheet"], "sprites.png")
        png = (directory / pack["spriteSheet"]).read_bytes()
        self.assertTrue(png.startswith(b"\x89PNG\r\n\x1a\n"))
        width, height = struct.unpack(">II", png[16:24])
        self.assertEqual((width, height), (1774, 887))
        self.assertEqual((pack["cellWidth"], pack["cellHeight"]), (443, 443))
        animations = {a["id"]: a for a in pack["animations"]}
        self.assertEqual(animations[pack["idle"]]["frames"], 1)
        self.assertEqual(animations["blink"]["frames"], 4)
        self.assertEqual(animations["wave"]["frames"], 4)
        self.assertEqual(pack["interactions"]["wave"], "wave")
        for animation in animations.values():
            self.assertLessEqual(animation["frames"] * pack["cellWidth"], width)
            self.assertLessEqual((animation["row"] + 1) * pack["cellHeight"], height)

    def test_first_character_launcher_is_opt_in(self):
        launcher = (ROOT / "scripts/run-gremlin.sh").read_text()
        self.assertIn('exec "$binary" --pack "$root/assets/characters/cyber-gremlin/pack.json" "$@"', launcher)
        verify = (ROOT / "scripts/verify.sh").read_text()
        self.assertIn("./test-output/pack-tests assets/characters/cyber-gremlin/pack.json", verify)
        self.assertIn("--pack assets/characters/cyber-gremlin/pack.json", verify)
        self.assertIn("--capture test-output/cyber-gremlin-smoke.png", verify)

    def test_renderer_reads_validated_pack_dimensions_and_rows(self):
        qml = (ROOT / "crates/deskling-desktop/qml/Main.qml").read_text()
        self.assertIn("characterPack.cellWidth", qml)
        self.assertIn("pet.animations[pet.activeAnimation].row", qml)
        self.assertIn('source: characterPack.source', qml)
        self.assertIn('objectName: "desklingSprite"', qml)

    def test_smoke_is_item_capture_not_window_alpha_claim(self):
        source = (ROOT / "crates/deskling-desktop/native/bridge.cpp").read_text()
        self.assertIn("sprite->grabToImage()", source)
        self.assertNotIn("window->grabWindow()", source)
        self.assertIn('window->setProperty("paused", true)', source)
        self.assertIn("Native X11 transparency", source)

    def test_license_and_source_only_distribution(self):
        self.assertTrue((ROOT / "LICENSE").read_text().startswith("MIT License"))
        for folder in [ROOT / "crates", ROOT / "examples"]:
            self.assertFalse(any(p.suffix.lower() in {".png", ".jpg", ".zip", ".so"} for p in folder.rglob("*")))


if __name__ == "__main__":
    unittest.main()
