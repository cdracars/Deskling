"""Public Kipwick source/layout and actual-manifest scheduler regressions.

Native PNG/frame validation and offscreen item rendering run in verify.sh.
"""
from pathlib import Path
import json
import struct
import unittest
import shutil
import subprocess
import tempfile
import test_scheduler

ROOT = Path(__file__).resolve().parents[1]
PACK = json.loads((ROOT / 'assets/characters/kipwick-alpha/pack.json').read_text())


class KipwickSourceTests(unittest.TestCase):
    def test_atlas_and_eight_rows_fit_existing_limits(self):
        png = (ROOT / 'assets/characters/kipwick-alpha/sprites.png').read_bytes()
        self.assertEqual(png[:8], b'\x89PNG\r\n\x1a\n')
        self.assertEqual(struct.unpack('>II', png[16:24]), (6912, 2304))
        self.assertEqual(tuple(png[24:26]), (8, 6))  # 8-bit RGBA, not opaque RGB.
        self.assertLessEqual(len(png), 16 * 1024 * 1024)
        self.assertLessEqual(6912 * 2304, 16777216)
        self.assertEqual((PACK['cellWidth'], PACK['cellHeight']), (288, 288))
        self.assertEqual(PACK['id'], 'kipwick-hybrid-alpha')
        entries = PACK['animations']
        self.assertEqual([a['row'] for a in entries], list(range(8)))
        self.assertTrue(all(a['frames'] == 24 for a in entries))
        self.assertEqual({a['id']: a['fps'] for a in entries}, {
            'idle': 12, 'blink': 40, 'wave': 20, 'curious': 12,
            'bounce': 12, 'sleepy': 10, 'drag-left': 12, 'drag-right': 12})
        ids = {a['id'] for a in entries}
        self.assertEqual(PACK['idle'], 'idle')
        self.assertTrue(set(PACK['interactions'].values()) <= ids)
        self.assertTrue(all(b['animation'] in ids and b['loops'] == 1 for b in PACK['behaviors']))

    def test_launcher_and_actual_pack_ci_are_wired(self):
        launcher = (ROOT / 'scripts/run-kipwick.sh').read_text()
        self.assertIn('exec "$binary" --pack "$root/assets/characters/kipwick-alpha/pack.json" "$@"', launcher)
        verify = (ROOT / 'scripts/verify.sh').read_text()
        self.assertIn('./test-output/pack-tests assets/characters/cyber-gremlin/pack.json assets/characters/kipwick-alpha/pack.json', verify)
        self.assertIn('--pack assets/characters/kipwick-alpha/pack.json', verify)
        self.assertIn('--capture test-output/kipwick-alpha-smoke.png', verify)


@unittest.skipUnless(test_scheduler.NODE, "Node unavailable; actual-pack scheduler check skipped")
class KipwickSchedulerTests(unittest.TestCase):
    js = test_scheduler.SchedulerTests.js
    def test_actual_kipwick_manifest_actions_return_idle(self):
        self.js('''
const manifest = ''' + json.dumps(PACK) + ''';
const ids = manifest.animations.map(a => a.id);
characterPack = {idleIndex: ids.indexOf(manifest.idle),
  waveIndex: ids.indexOf(manifest.interactions.wave),
  animations: manifest.animations.map(a => ({frames:a.frames, intervalMs:Math.round(1000/a.fps)})),
  behaviors: manifest.behaviors.map(b => ({animation:ids.indexOf(b.animation), loops:b.loops, weight:b.weight}))};
animations = characterPack.animations;
frameCounts = animations.map(a => a.frames);
frameIntervals = animations.map(a => a.intervalMs);
automaticBehaviors = characterPack.behaviors;
assert.equal(frameCounts[1] * frameIntervals[1], 600);
for (const b of automaticBehaviors) {
  startGesture(b.animation, b.loops);
  for (let tick=0; tick<24; ++tick) advanceAnimation();
  assert.equal(activeAnimation, 0); assert.equal(frame, 0);
  assert.equal(gestureLoopsRemaining, 0);
}
for (const automatic of [true, false]) {
  setAutonomous(automatic);
  for (let index=0; index<8; ++index) {
    chooseAnimation(index);
    for (let tick=0; tick<24; ++tick) advanceAnimation();
    assert.equal(activeAnimation, 0); assert.equal(frame, 0);
    assert.equal(autonomous, automatic);
  }
}
''')


class KipwickLauncherTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="kipwick-launcher-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name) / "checkout with spaces"
        (self.root / "scripts").mkdir(parents=True)
        self.launcher = self.root / "scripts/run-kipwick.sh"
        shutil.copyfile(ROOT / "scripts/run-kipwick.sh", self.launcher)

    def test_missing_build_only_explains_next_step(self):
        result = subprocess.run(["sh", str(self.launcher)], cwd=self.temp.name, text=True, capture_output=True)
        self.assertEqual(result.returncode, 1)
        self.assertIn("cargo build --offline --locked --release", result.stderr)
        self.assertFalse((self.root / "target").exists())

    def test_launch_preserves_pack_path_and_quoted_arguments(self):
        binary = self.root / "target/release/deskling-desktop"
        binary.parent.mkdir(parents=True)
        binary.write_text('#!/bin/sh\nprintf "<%s>\\n" "$@"\n')
        binary.chmod(0o755)
        result = subprocess.run(["sh", str(self.launcher), "--reset-position", "--capture", "a capture.png"],
                                cwd=self.temp.name, text=True, capture_output=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout.splitlines(), ["<--pack>",
            f"<{self.root}/assets/characters/kipwick-alpha/pack.json>",
            "<--reset-position>", "<--capture>", "<a capture.png>"])
