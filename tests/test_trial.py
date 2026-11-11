"""Fast isolated trial tests; never start Qt or install software."""
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]

class TrialTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix="deskling-trial-")
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name) / "checkout with spaces"
        (self.root / "scripts").mkdir(parents=True)
        shutil.copyfile(ROOT / "scripts/try-deskling.sh", self.root / "scripts/try-deskling.sh")
        self.env = dict(os.environ, DISPLAY=":77")
        self.env.pop("XDG_CONFIG_HOME", None)

    def run_trial(self, *args, env=None):
        return subprocess.run(["sh", str(self.root / "scripts/try-deskling.sh"), *args],
                              text=True, capture_output=True, cwd=self.tmp.name,
                              env=self.env if env is None else env)

    def stub(self, status=0):
        binary = self.root / "target/release/deskling-desktop"
        binary.parent.mkdir(parents=True)
        binary.write_text("#!/bin/sh\nprintf 'CONFIG=%s\\n' \"$XDG_CONFIG_HOME\"\n"
                          "printf 'ARG=%s\\n' \"$@\"\n"
                          "printf marker > \"$XDG_CONFIG_HOME/marker\"\n"
                          f"exit {status}\n")
        binary.chmod(0o755)
        for name in ("kipwick-alpha", "cyber-gremlin"):
            folder = self.root / "assets/characters" / name
            folder.mkdir(parents=True)
            (folder / "pack.json").write_text("{}")

    def test_help_no_display_or_build(self):
        env = self.env.copy()
        env.pop("DISPLAY")
        r = self.run_trial("--help", env=env)
        self.assertEqual(r.returncode, 0)
        self.assertIn("--character", r.stdout)
        self.assertFalse((self.root / "target").exists())

    def test_no_display_fails_before_build(self):
        env = self.env.copy()
        env.pop("DISPLAY")
        r = self.run_trial(env=env)
        self.assertEqual(r.returncode, 2)
        self.assertIn("X11 desktop", r.stderr)
        self.assertFalse((self.root / "target").exists())

    def test_invalid_input_fails_without_build(self):
        for args in (("--character", "wrong"), ("--pack", "thing"),
                     ("--character",), ("--character", "kipwick", "extra")):
            self.assertEqual(self.run_trial(*args).returncode, 2)
        self.assertFalse((self.root / "target").exists())

    def test_isolation_default_pack_and_cleanup(self):
        self.stub()
        r = self.run_trial()
        self.assertEqual(r.returncode, 0, r.stderr)
        self.assertIn("ARG=--pack", r.stdout)
        self.assertIn("kipwick-alpha/pack.json", r.stdout)
        self.assertIn("ARG=--reset-position", r.stdout)
        path = next(l.split("=", 1)[1] for l in r.stdout.splitlines() if l.startswith("CONFIG="))
        self.assertFalse(Path(path).exists())

    def test_alternate_character_and_diagnostic(self):
        self.stub()
        r = self.run_trial("--character", "gremlin")
        self.assertEqual(r.returncode, 0, r.stderr)
        self.assertIn("cyber-gremlin/pack.json", r.stdout)
        r = self.run_trial("--character", "diagnostic")
        self.assertEqual(r.returncode, 0, r.stderr)
        self.assertNotIn("ARG=--pack", r.stdout)

    def test_bad_runtime_exit_propagates_and_cleans(self):
        self.stub(status=8)
        r = self.run_trial()
        self.assertEqual(r.returncode, 8)
        path = next(l.split("=", 1)[1] for l in r.stdout.splitlines() if l.startswith("CONFIG="))
        self.assertFalse(Path(path).exists())

    def test_missing_pack_is_actionable(self):
        self.stub()
        (self.root / "assets/characters/kipwick-alpha/pack.json").unlink()
        r = self.run_trial()
        self.assertEqual(r.returncode, 1)
        self.assertIn("pack unavailable", r.stderr.lower())

if __name__ == "__main__":
    unittest.main()
