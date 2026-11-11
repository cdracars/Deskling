"""Security/UX contracts for manually invoked local screen review.

The full native Qt build/smoke in CI is authoritative for C++/QML integration.
A real X11 run remains mandatory to validate the live desktop experience.
"""
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]
CPP = (ROOT / "crates/deskling-desktop/native/bridge.cpp").read_text()
HEADER = (ROOT / "crates/deskling-desktop/native/bridge.hpp").read_text()
QML = (ROOT / "crates/deskling-desktop/qml/Main.qml").read_text()

class ScreenReviewContracts(unittest.TestCase):
    def test_capture_is_only_exposed_from_explicit_review_button(self):
        self.assertIn('objectName: "screenReviewButton"', QML)
        self.assertIn("onClicked: pet.beginScreenReview()", QML)
        self.assertIn("bridge.captureScreenReview()", QML)
        self.assertEqual(QML.count("bridge.captureScreenReview()"), 1)
        self.assertIn("if (screenReviewCapturePending) return", QML)
        self.assertIn("screenReviewDelay.start()", QML)
        self.assertIn("pet.hide()", QML)
        self.assertIn("controls.hide()", QML)
        self.assertIn("pet.show()", QML)

    def test_preview_precedes_explicit_clipboard_copy(self):
        self.assertIn('objectName: "screenReviewWindow"', QML)
        self.assertIn("bridge.screenReviewReady ?", QML)
        self.assertIn('objectName: "copyScreenReviewButton"', QML)
        self.assertIn("onClicked: bridge.copyScreenReview()", QML)
        self.assertEqual(QML.count("bridge.copyScreenReview()"), 1)
        self.assertIn('objectName: "discardScreenReviewButton"', QML)
        self.assertIn("bridge.clearScreenReview()", QML)
        self.assertIn("onClosing: bridge.clearScreenReview()", QML)
        self.assertIn("cache: false", QML)

    def test_screen_is_local_and_not_uploaded(self):
        self.assertIn("screen->grabWindow(0)", CPP)
        self.assertIn("m_screenReview = image", CPP)
        self.assertIn("QGuiApplication::clipboard()->setImage", CPP)
        self.assertNotIn("QNetworkAccessManager", CPP)
        self.assertNotIn("QFile::write", CPP)
        self.assertIn("16000000LL", CPP)
        self.assertIn("Screenshot copied to the desktop clipboard", CPP)
        self.assertIn("only if your chosen assistant is approved", QML)

    def test_smoke_mode_blocks_capture_and_copy(self):
        self.assertIn("if (m_smokeTest)", CPP)
        self.assertIn("if (smokeTest && (bridge.captureScreenReview() || bridge.copyScreenReview()", CPP)
        self.assertIn("screenReviewReady()", HEADER)
        self.assertIn("clearScreenReview()", HEADER)

    def test_existing_pack_and_animation_contract_is_unchanged(self):
        self.assertIn('source: characterPack.source', QML)
        self.assertIn('onTriggered: pet.advanceAnimation()', QML)
        self.assertIn('if (mouse.button === Qt.RightButton) pet.showControls()', QML)
        self.assertIn('setContextProperty("characterPack", characterPack)', CPP)

if __name__ == "__main__":
    unittest.main()
