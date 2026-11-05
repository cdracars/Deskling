"""Source checks only; native Qt validation is in pack_test.cpp and CI."""
from pathlib import Path
import json
import unittest
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[1]


class SourceTests(unittest.TestCase):
    def test_resource_graph_bundles_app_icons_but_no_character_pack(self):
        qrc = ROOT / "crates/deskling-desktop/native/resources.qrc"
        files = ET.fromstring(qrc.read_text()).findall(".//file")
        self.assertEqual([f.attrib["alias"] for f in files], ["Main.qml"] + [
            f"icons/deskling-{size}.png" for size in (16, 24, 32, 48, 64, 128, 256, 512)
        ])
        for entry in files:
            self.assertTrue((qrc.parent / entry.text).resolve().is_file())

    def test_template_is_explicit_data_not_complete_pack(self):
        pack = json.loads((ROOT / "examples/pack.template.json").read_text())
        self.assertEqual(pack["schemaVersion"], 1)
        self.assertIn(pack["idle"], [a["id"] for a in pack["animations"]])
        self.assertFalse((ROOT / "examples" / pack["spriteSheet"]).exists())

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
