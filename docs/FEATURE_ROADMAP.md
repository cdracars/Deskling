# Deskling Feature Roadmap — Adoption First

**Status:** Proposed engineering roadmap (2026-11-10), not approved commercial scope or a release commitment.  
**Code baseline:** `main` at `b41ff3d` (CI [passed](https://github.com/cdracars/Deskling/actions/runs/42234626006)).  
**Architecture/decision authority:** [Constitution](PRODUCT_CONSTITUTION.md), [C4](ARCHITECTURE_C4.md).  
**Delivery and roles:** [Iteration plan](ITERATION_PLAN.md), [Team execution](TEAM_EXECUTION.md).

## Product goal and sequencing

**Get people to try it -> give them immediate delight -> let them personalize it -> make it trustworthy and easy to keep -> learn whether anyone actually wants it.**

Existing capabilities to exploit, **not rebuild:** local X11 Rust/Qt window; validated JSON/PNG packs and `--pack`; gestures, pause, resize and reset-position controls; bundled opt-in Kipwick alpha and Cyber Gremlin study; Rust Avatar Factory; layered rig/CPU animation; Qt Avatar Studio with open/edit/save/export/run; Ubuntu 24.04 CI test kit.

**Current friction:** default launch displays a diagnostic shape; switching characters is not a discoverable in-app experience; local install/setup still depends on tooling or a short-lived CI artifact; Kipwick animation is explicitly alpha; no live X11 evidence in CI; distribution rights for artwork are not cleared. Do not claim these are resolved until proven.

## Iteration 1 — Quick wins that create an adoption-quality demo

Relative size assumes an engineer familiar with the code and is **not a schedule guarantee**. Use one issue per vertical slice, written tests first, and one reviewable PR per change.

| Order | Feature / present gap | Minimum shippable change | Value | Size | Owner lane |
| --- | --- | --- | --- | --- | --- |
| 1 | **One-path trial / first launch**: setup has too many steps | A documented, non-destructive Ubuntu 24.04/X11 demo entry that detects missing prereqs, starts a clearly identified development character and tells users how to stop/clean up. No sudo or autostart from launcher. | First use | S–M | H4 (QA/tooling) |
| 2 | **Character choice**: only CLI `--pack` or alternate scripts | In-app local character selection (or a simple explicit relaunch workflow) with clear current pack, safe missing/invalid handling, and `--pack` precedence. Persist preference only by explicit action. | First use + ownership | M | H1 (runtime) |
| 3 | **Immediate delight**: Kipwick is a known alpha | Fix a small, evidence-ranked set of highest-visibility idle/blink/gesture/drag defects. Prioritize coherent motion over added animations. | Retention | M | H2 (art/animation) |
| 4 | **Discoverability + control**: existing pause/scale/reset/quit are behind controls | Add a succinct first-run hint and clearer interaction labels; prove pause/quit/position recovery work without knowing shortcuts. Avoid notification nags. | Trust + retention | S | H1 with H4 review |
| 5 | **Creator aha moment**: Studio is implemented but hard to approach | One written or in-app "five-minute rig edit": launch demo, modify joint/keyframe, save, export to a new directory, run output; demonstrate visible change. | Differentiation | S–M | H3 (creator tools) |

**Iteration-1 demo gate:** novice can start a demo, identify how to interact/pause/quit, select a local pack or safely recover; an engineer can show a real X11 recording and exact-commit tests. Rights of included *development-only* art are not silently assumed. **Do not present this as a public release.**

### First-iteration success checks (targets, not results)

- Activation: **4/5** voluntary qualitative testers (if available) can launch without live engineering help using the demo instructions.
- First delight: **4/5** discover one interaction within two minutes without a guided tour beyond the short in-app hint.
- Control/trust: **5/5** can find pause and quit; none are trapped off-screen without documented recovery.
- Creator path: a tester who follows the quickstart can create an exported change in **five minutes** once prerequisites are ready.
- Reliability: **zero known P0** crash/unsafe-content blockers on the documented test setup; existing test suite passes on exact demo commit.
- Evidence: each threshold gets an observed result with environment and tester consent; otherwise mark **NOT MEASURED**, not passed. No hidden analytics.

## Iteration 2 — Product conversation / differentiation

| Order | Feature | Outcome and evidence gate | Size | Main dependency |
| --- | --- | --- | --- | --- |
| 1 | **Polished avatar customization** | Consistent source -> Studio -> edited rig -> export -> desktop loop with live recording; address highest-impact editor friction | M | I1 Studio path |
| 2 | **Second distinct, distributable original character** | Two visibly different styles validate generic engine and creator tool; art/rights approval before external distribution | M–L | Art author + rights review |
| 3 | **Testable installation/packaging candidate** | Isolated, repeatable build/demo on Ubuntu 24.04/X11 and clean removal. Packaging/licensing gap list explicit | M | I1 launcher; Qt obligations |
| 4 | **Performance and UX acceptance** | Measured idle CPU/RAM, startup, responsiveness, focus/drag/stacking and accessibility; fixes driven by data | S–M | Target X11 session |
| 5 | **Product-discussion packet** | Demo video, positioning hypotheses, roadmap, C4, support/cost/risk outline and named sponsor decisions | S | Evidence from I1/I2 |

**Iteration-2 gate:** the group can have a defensible **fund/authorize-pilot / gather-more-evidence / stop** conversation. Nothing here presumes the user is business sponsor or has approved a launch.

## Later feature candidates — prioritize after observed usage

| Horizon | Candidate | Adoption / user value hypothesis | Complexity | Decision trigger |
| --- | --- | --- | --- | --- |
| Next | **Gallery and character themes** (local-only pack folders, previews, favorites) | Makes personalization easy without adding cloud | M | Users struggle to find/switch packs |
| Next | **Simple in-Studio templates** (expressions, blink and wave presets, reusable actions) | More creators complete first custom character | M–L | Creator usability tests show pose-authoring friction |
| Next | **Reduced-motion and do-not-disturb profiles** | Feels respectful during work; optional quiet hours | M | Feedback shows distraction or motion sensitivity |
| Next | **Undo/redo in Avatar Studio** | Reduces editing risk and rework | M | Observed save/recover friction |
| Explore | **Wayland support** | Expands addressable Linux environments | L–XL | Target-audience evidence and feasibility spike |
| Explore | **Opt-in local activity adapters** (e.g. user-selected timer/state) | Utility without requiring a model | L + security review | Clear user job, privacy threat model and consent design |
| Explore | **Local agent companion interface** | Expresses agent state through character without an embedded chat application | L–XL + security review | A pilot proves actual demand; explicit permission and protocol |
| Explore | **Pack sharing/community ecosystem** | Creator network effects | L–XL + legal/support | Content-rights, moderation and maintenance ownership |

**Explicitly not for Iterations 1–2:** always-listening assistant, remote model calls, implicit screen reading, account system, commerce, automatic web asset loading, complete image-to-character generation, full Wayland rewrite or large plugin architecture.

## Roadmap grooming rules

1. Every task must map to the **first use / delight / customization / trust / learn** funnel.
2. Prefer a small demonstrable change to a speculative framework or integration.
3. Haiku tasks should be scoped to **one clear ownership area** and ideally fit a single reviewable PR. If uncertain, spike/test first and report evidence.
4. Prioritize by observed user friction, consequence and implementation cost; record a precise before/after.
5. Preserve existing CLI, native pack schema, tests, source artwork and privacy defaults unless an ADR authorizes the breaking change.
6. The lead architect owns interface decisions and integration review; the business sponsor (unassigned) owns product strategy, distribution and funding.
7. No test or rights evidence -> **NOT VERIFIED**. No desktop footage -> do not claim live X11 quality.

**Next reading:** [Two-iteration plan](ITERATION_PLAN.md) · [Product discussion](PRODUCT_CONVERSATION.md) · [Team execution](TEAM_EXECUTION.md).
