"""Source-level contract for discoverable, non-disruptive character help.

Qt's offscreen smoke test in CI is the authoritative QML syntax/render check.
"""
from pathlib import Path
import unittest

QML = (Path(__file__).resolve().parents[1] / "crates/deskling-desktop/qml/Main.qml").read_text()

class DiscoverabilityTests(unittest.TestCase):
    def test_hover_only_and_delayed(self):
        self.assertIn("hoverEnabled: true", QML)
        self.assertIn("ToolTip.visible: containsMouse && !pressed && !pet.dragging && !controls.visible", QML)
        self.assertIn("ToolTip.delay: 1200", QML)
        self.assertIn("ToolTip.timeout: 4500", QML)

    def test_tooltip_explains_existing_actions(self):
        self.assertIn('ToolTip.text: "Click to wave · drag to move · right-click for controls"', QML)
        self.assertIn("else if (!moved) pet.wave()", QML)
        self.assertIn("if (mouse.button === Qt.RightButton) pet.showControls()", QML)
        self.assertIn("bridge.beginDrag()", QML)

    def test_existing_pause_and_quit_remain(self):
        self.assertIn('text: pet.paused ? "Resume" : "Pause"', QML)
        self.assertIn('Button { text: "Quit Deskling"; onClicked: Qt.quit() }', QML)
        self.assertIn('Shortcut { sequence: "Ctrl+Q"; onActivated: Qt.quit() }', QML)

if __name__ == "__main__":
    unittest.main()
