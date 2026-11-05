"""Exercise the opt-in launcher without a GUI or an installed Deskling binary."""
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]


class LauncherTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="deskling-launcher-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name) / "checkout with spaces"
        (self.root / "scripts").mkdir(parents=True)
        self.launcher = self.root / "scripts/run-gremlin.sh"
        shutil.copyfile(ROOT / "scripts/run-gremlin.sh", self.launcher)

    def test_missing_build_explains_next_step_without_installing(self):
        result = subprocess.run(["sh", str(self.launcher)], cwd=self.temp.name,
                                text=True, capture_output=True)
        self.assertEqual(result.returncode, 1)
        self.assertIn("cargo build --offline --locked --release", result.stderr)
        self.assertFalse((self.root / "target").exists())

    def test_launch_from_other_directory_preserves_quoted_arguments(self):
        binary = self.root / "target/release/deskling-desktop"
        binary.parent.mkdir(parents=True)
        binary.write_text('#!/bin/sh\nprintf "<%s>\\n" "$@"\n')
        binary.chmod(0o755)
        result = subprocess.run(["sh", str(self.launcher), "--reset-position",
                                 "--smoke-test", "--capture", "a capture.png"],
                                cwd=self.temp.name, text=True, capture_output=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout.splitlines(), [
            "<--pack>", f"<{self.root}/assets/characters/cyber-gremlin/pack.json>",
            "<--reset-position>", "<--smoke-test>", "<--capture>", "<a capture.png>"])


if __name__ == "__main__":
    unittest.main()
