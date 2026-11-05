"""Run the real QML scheduler helpers in Node, not a reimplemented model.

This does not execute Qt bindings, rendering, or input. Node is optional.
"""
from pathlib import Path
import re
import shutil
import subprocess
import unittest

QML = (Path(__file__).resolve().parents[1] / "crates/deskling-desktop/qml/Main.qml").read_text()
NODE = shutil.which("node")


def source():
    # Deliberately use a nonzero idle index to catch character-specific assumptions.
    setup = """
var characterPack = {
  idleIndex: 1, waveIndex: 0,
  animations: [
    {frames: 4, intervalMs: 150}, {frames: 6, intervalMs: 180},
    {frames: 3, intervalMs: 200}, {frames: 5, intervalMs: 130}
  ],
  behaviors: [{animation: 0, loops: 1, weight: 2}, {animation: 2, loops: 2, weight: 3}]
};
var visible = true;
"""
    names = ("paused", "selectedAnimation", "activeAnimation", "frame", "dragging", "waving",
             "animations", "frameCounts", "frameIntervals", "automaticBehaviors", "autonomous",
             "gestureLoopsRemaining", "automaticWaitMs")
    for name in names:
        expression = re.search(r"(?:readonly )?property \w+ " + name + r": (.*)", QML).group(1)
        if expression == "{":
            body = QML.split("property var " + name + ": {", 1)[1].split("\n    }", 1)[0]
            expression = "(function(){" + body + "})()"
        setup += "var " + name + " = " + expression + ";\n"
    helpers = QML.split("    function randomWait(first) {", 1)[1].split("    function showControls()", 1)[0]
    return setup + "function randomWait(first) {" + helpers


class WiringTests(unittest.TestCase):
    def test_timer_binding_and_no_automatic_system_actions(self):
        self.assertIn("running: !pet.paused && pet.visible", QML)
        self.assertIn("onTriggered: pet.advanceAnimation()", QML)
        self.assertNotIn("animationClock.restart", QML)
        for token in ("bridge.", "requestActivate", ".raise(", "keyboard"):
            self.assertNotIn(token, source())


@unittest.skipUnless(NODE, "Node unavailable; JavaScript behavior tests skipped")
class SchedulerTests(unittest.TestCase):
    def js(self, body):
        result = subprocess.run([NODE, "-e", "const assert = require('assert');\n" + source() + body], text=True, capture_output=True)
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_wait_bounds(self):
        self.js("""
assert(automaticWaitMs >= 5000 && automaticWaitMs <= 10000);
Math.random = () => 0; assert.equal(randomWait(false), 20000);
Math.random = () => 0.999999; assert.equal(randomWait(false), 45000);
""")

    def test_every_behavior_returns_pack_idle(self):
        self.js("""
for (const b of automaticBehaviors) {
  startGesture(b.animation, b.loops);
  for (let i = 0; i < frameCounts[b.animation] * b.loops; ++i) advanceAnimation();
  assert.equal(activeAnimation, characterPack.idleIndex);
  assert.equal(gestureLoopsRemaining, 0);
  assert(automaticWaitMs >= 20000);
}
""")

    def test_high_rate_24_frame_preview_is_bounded(self):
        self.js("""
frameCounts[2] = 24; frameIntervals[2] = 25;
chooseAnimation(2);
let nominalElapsed = 0;
for (let i = 1; i < 24; ++i) {
  nominalElapsed += frameIntervals[activeAnimation]; advanceAnimation();
  assert.equal(activeAnimation, 2);
  assert.equal(frame, i);
  assert.equal(gestureLoopsRemaining, 1);
}
nominalElapsed += frameIntervals[activeAnimation]; advanceAnimation();
assert.equal(nominalElapsed, 600);
assert.equal(activeAnimation, characterPack.idleIndex);
assert.equal(frame, 0);
assert.equal(gestureLoopsRemaining, 0);
assert(autonomous);
""")

    def test_pause_and_hide_freeze_state(self):
        self.js("""
startGesture(2, 2);
const before = [frame, gestureLoopsRemaining, automaticWaitMs];
paused = true; for (let i = 0; i < 100; ++i) advanceAnimation();
assert.deepEqual([frame, gestureLoopsRemaining, automaticWaitMs], before);
paused = false; visible = false; advanceAnimation();
assert.deepEqual([frame, gestureLoopsRemaining, automaticWaitMs], before);
""")

    def test_preview_returns_idle_and_preserves_automatic_behavior(self):
        self.js("""
for (const automatic of [true, false]) {
  setAutonomous(automatic);
  chooseAnimation(2);
  assert.equal(autonomous, automatic);
  assert.equal(activeAnimation, 2);
  assert.equal(selectedAnimation, 2);
  assert.equal(gestureLoopsRemaining, 1);
  for (let i = 0; i < frameCounts[2]; ++i) advanceAnimation();
  assert.equal(activeAnimation, characterPack.idleIndex);
  assert.equal(selectedAnimation, characterPack.idleIndex);
  assert.equal(gestureLoopsRemaining, 0);
  assert.equal(autonomous, automatic);
}
""")

    def test_selecting_idle_does_not_disable_automatic_gestures(self):
        self.js("""
Math.random = () => 0;
setAutonomous(true);
chooseAnimation(characterPack.idleIndex);
assert(autonomous);
assert.equal(activeAnimation, characterPack.idleIndex);
automaticWaitMs = 1;
advanceAnimation();
assert.equal(gestureLoopsRemaining, 1);
assert.notEqual(activeAnimation, characterPack.idleIndex);
""")

    def test_preview_pause_repeat_and_wave_interruption(self):
        self.js("""
chooseAnimation(2); advanceAnimation();
assert.equal(frame, 1);
chooseAnimation(2);
assert.equal(frame, 0);
assert.equal(gestureLoopsRemaining, 1);
paused = true;
advanceAnimation(); assert.equal(frame, 0);
paused = false;
wave(); assert.equal(activeAnimation, characterPack.waveIndex);
for (let i = 0; i < frameCounts[characterPack.waveIndex]; ++i) advanceAnimation();
assert.equal(activeAnimation, characterPack.idleIndex);
assert.equal(gestureLoopsRemaining, 0);
assert(autonomous);
""")

    def test_disabled_automatic_behavior_never_restarts_after_preview(self):
        self.js("""
setAutonomous(false); chooseAnimation(2);
for (let i = 0; i < frameCounts[2]; ++i) advanceAnimation();
automaticWaitMs = 1;
for (let i = 0; i < 100; ++i) advanceAnimation();
assert.equal(activeAnimation, characterPack.idleIndex);
assert.equal(gestureLoopsRemaining, 0);
assert(!autonomous);
assert.equal(automaticWaitMs, 1);
""")

    def test_drag_preempts_and_suppresses_automatic_start(self):
        self.js("""
startGesture(2, 2); beginGestureDrag(); automaticWaitMs = 1;
for (let i = 0; i < 100; ++i) advanceAnimation();
assert.equal(gestureLoopsRemaining, 0); assert.equal(automaticWaitMs, 1);
dragging = false; returnToBase(); assert.equal(activeAnimation, characterPack.idleIndex);
assert(automaticWaitMs >= 20000);
""")

    def test_empty_behavior_list_stays_idle(self):
        self.js("""
automaticBehaviors = []; automaticWaitMs = 0; startAutomaticGesture();
assert.equal(activeAnimation, characterPack.idleIndex);
assert.equal(gestureLoopsRemaining, 0); assert(automaticWaitMs >= 20000);
""")


if __name__ == "__main__":
    unittest.main()
