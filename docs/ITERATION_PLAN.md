# Deskling — Two-Iteration Architecture-to-Product Plan

**Planning status:** Proposed scope for engineering execution; no human business sponsor assigned.  
**Delivery goal:** A convincing **product conversation**, backed by a runnable and observable Linux desktop demonstration, within two iterations. **No commercial launch commitment.**  
**Baseline:** `main` merge commit `b41ff3d` (2026-11-06), successful [CI run](https://github.com/cdracars/Deskling/actions/runs/42234626006).  
**Constitution:** [PRODUCT_CONSTITUTION.md](PRODUCT_CONSTITUTION.md)

## Outcome, constraints and tracking

- **Outcome:** Show a genuinely enjoyable, local-first Linux companion and an end-to-end character-editing loop; explain the product opportunity, tradeoffs and what a pilot would take.
- **Timebox:** Two **iterations**; length is deliberately not assumed. Estimate and schedule after engineering capacity is known, but deliver an increment at each iteration boundary.
- **Target validation environment:** Ubuntu 24.04 in a real, composited X11 session, plus CI's offscreen test environment. A second distribution/WM test is desirable but not a hard claim until evidenced.
- **Do not silently:** merge, release binaries, install software, enable autostart, change permissions, access user data or represent unlicensed art as ready to ship.
- **Team operating rule:** architect owns decisions/tradeoffs; delegated coding lanes own scoped implementation and independent test evidence; design/art authority and product ownership remain **TBD**. One lane coordinates the overall release/verification ledger.
- **No waiting on routine prompts:** advance along scoped tasks and report progress at review points; stop and escalate only safety, rights, irreversible, commercial or release decisions.

## Iteration 1 — Experience and reliability baseline

**User-facing increment:** Running Deskling locally feels coherent and visibly responsive, and the engineering team can reproduce the same behavior.

| Priority | Workstream / specification | Verification and exit gate |
| --- | --- | --- |
| P0 | Inventory visual and interaction defects in Kipwick and diagnostic/synthetic character. Record baseline screen captures of real X11 idle, blink, wave, bounce, drag and transition. | Capture device/OS/WM/Qt versions, steps, expected/actual results and first failing cases before motion changes; publish before/after evidence. |
| P0 | First-run/selection behavior: choose an available local pack explicitly, display the active character, recover from a missing or invalid pack, and preserve CLI pack launch. | Unit/native tests for valid, malformed, moved, oversized and missing packs; transparent failure states; no secret/remote file paths; no unrequested executable pack code. |
| P0 | Desktop reliability and delight: animation transitions, natural pauses, coherent blink, window focus/drag/stacking and position reset. | Existing tests + exact-commit CI; independent hands-on review of real X11 behavior; tracked defects, no false claims from offscreen smoke. |
| P1 | Reproducible isolated demo kit and operator instructions for Ubuntu 24.04/X11. | Non-destructive fresh-user test: prerequisites, launch, stop, clean removal; no implicit app installation or autostart; verify instructions from an empty folder. |
| P1 | Confirm legal provenance and compatibility inventory (Kipwick, icon, Qt runtime, other art). | Rights checklist created and gaps flagged. Missing rights **block redistribution**, not local technical validation. |
| P1 | Product research hypothesis and rough cost/risk model. | A one-page brief with assumptions distinguished from verified outcomes; no fabricated demand/cost data. |

**Iteration 1 exit demo:** launch/gesture/pause/quit + switch/recover a pack + a recorded real-X11 test. If no approved artwork, mark visuals **development-only** and avoid representing them as shipping art.

## Iteration 2 — Creator experience and product-discussion package

**User-facing increment:** A creator can change a motion in Avatar Studio, export a new native pack, launch it in Deskling, and share a short recorded demonstration.

| Priority | Workstream / specification | Verification and exit gate |
| --- | --- | --- |
| P0 | End-to-end creator journey: open an original layered rig, change a pose, save atomically, export, launch and see the modified animation. | Regression test of saved/editable project round-trip, export validity and native pack loader; captured X11 demo, not solely offscreen test. |
| P0 | Make a second distinct approved character/rig or a visually acceptable original demo pack. Avoid claims of automatic portrait-to-avatar conversion. | Second-character fixture passes programmatic validators; independent visual review for continuity/seams/motion and explicit rights approval **before distribution**. |
| P0 | Clear setup/onboarding, user control, fallback behavior and a 90-second product walkthrough. | A person unfamiliar with the repository performs the journey without engineering intervention; friction documented, not hidden. |
| P1 | Runtime resource/performance assessment on target Linux hardware. | Repeatable startup/idle CPU and memory observations, packaging size, crash/restore behavior; record actual readings and environment rather than invent benchmarks. |
| P1 | Accessibility and reduced-motion evaluation: keyboard controls, pause, scale, focus, recovery. | Document findings, fixes and known limitations; route release-affecting issues to owners. |
| P0 | Assemble product conversation: pitch, current evidence, customer hypotheses, C4, demo, release risks, operating cost categories, sponsorship questions, pilot proposal. | Presentation-ready written brief and decision log, visible residual risks, explicit ask for a *pilot/discovery decision* (not launch). |

**Iteration 2 exit demo:** show the complete offline user and creator journeys; provide exact build/CI references, a tested live-desktop capture, rights ledger and unassigned sponsor decisions. Stop short of distribution without licenses/approvals.

## Feature acceptance scenarios — initial specification seeds

```gherkin
Feature: Safe local character selection
  Scenario: User selects a valid local pack
    Given Deskling is running without a remote account
    When the user chooses a validated local PNG/JSON character pack
    Then Deskling displays that pack's idle animation
    And the user can pause, resume, drag, and quit

  Scenario: A selected pack becomes invalid
    Given the selected local pack was previously usable
    When the pack is removed or no longer passes native validation
    Then the user receives a clear local error
    And Deskling remains recoverable without editing hidden files
    And no network request or executable pack content is invoked

  Scenario: CLI pack remains an explicit override
    Given a configured local default pack exists
    When the user starts Deskling with --pack PATH
    Then that explicit pack is selected for this run
    And persistent selection is not silently overwritten
```

```gherkin
Feature: Creator edits and exports a Deskling character
  Scenario: A changed rig is exported
    Given an original local rig with valid layer images
    When the creator edits an intermediate motion frame and saves
    And exports to a new output directory
    Then the input layer images remain byte-for-byte unchanged
    And the native pack passes the production loader
    And the exported gesture visibly differs at the edited pose

  Scenario: Export fails safely
    Given an occupied output location or invalid rig
    When the creator requests export
    Then the tool reports failure without clobbering original assets
    And the creator can correct the issue and retry
```

Implementation is not claimed until supported by tests and a demo. Maintain a **test matrix** with automated expected/actual, real-desktop checks, original defect evidence, environment details and resulting artifact links.

## Suggested staffing / parallel lanes (roles, not presumed people)

| Lane | Focus | Dependencies |
| --- | --- | --- |
| Lead architect / integrator | Constitution, ADRs, scope, interfaces, architecture reviews, product decision package | Reads all lanes; resolves technical conflicts |
| Runtime implementation | Pack selection, first run, QML animation/interactions, native window bugs | Pack contract; coordinates with QA |
| Creator implementation | Studio/Factory authoring experience, second rig and export | Rig contract; usable art rights |
| Verification and release engineering | Test harness, Linux/X11 recorded validation, isolated demo kit, dependency/rights ledger | Independent of implementation wherever possible |
| Art/design reviewer (TBD) | Character quality and rights evidence | Human approval required for publishable art |

Where capacity is limited, combine implementation work but keep independent review. No separate agents or completed work are asserted by this document.

## Risks and stop conditions

1. **Artwork rights unknown:** restrict redistribution until proven; a test fixture does not substitute for an approved character.
2. **Real X11 behavior uncertain:** no product-readiness claim without target-desktop recording.
3. **Qt distribution/dependencies:** distinguish source build/test kit from a licensed, maintainable installer.
4. **Scope pressure:** integrations, agent awareness, voice, chat, telemetry, auto-generated portrait art and Wayland are **explicitly out of scope** for the two-iteration gate.
5. **Business sponsor absent:** may still complete a product-discussion package; cannot claim investment approval, company adoption or product launch.

## Evidence ledger for the two iteration reviews

For each work item record: GitHub issue/PR and exact commit, linked spec/test, pass/fail results, manual environment and recording, known limitation, reviewer, and approval/ownership status. If a gate cannot be shown, label it **NOT VERIFIED** rather than claiming done.
