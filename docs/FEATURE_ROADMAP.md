# Deskling opportunity roadmap

**Status:** Prioritized planning opportunities, not an implementation work order or release commitment.
**Governing plan:** [ITIL 4 and CSDM 5](SERVICE_MODEL.md), [two planning gates](ITERATION_PLAN.md).

## Existing foundation

Main contains the Rust/Qt X11 companion, validated local JSON/PNG packs, gestures and controls, opt-in Kipwick and Gremlin packs, Avatar Factory, the Rust rig renderer and Qt Avatar Studio. Launching without a pack displays diagnostic geometry. Live X11 quality, broad platform compatibility and artwork distribution rights remain separate acceptance questions. See [README](../README.md).

## Prioritized learning and preparation

| Priority | Opportunity | Question and useful evidence | Current boundary |
| --- | --- | --- | --- |
| 1 | First use and control | Can a new user launch, discover interactions, pause, quit and recover safely? Review existing trial/hint PRs and prepare live acceptance. | Preserve #12/#13; no merge or new feature implied. |
| 2 | Creator usefulness | Can a creator edit a rig, save, export and recognize the changed output without damaging source art? | Reuse merged Studio; prepare a reproducible evaluation before expanding tools. |
| 3 | Continued enjoyment | Do users want to keep the companion, and what motion or distraction problems matter? | Require observed feedback and a baseline before proposing art changes. |
| 4 | Workplace assistance | Which task benefits from context-aware help, and is a simpler existing tool sufficient? | Synthetic scenarios and conceptual alternatives only; #15 remains parked. |
| 5 | Platform and adoption feasibility | Which operating systems and support obligations does the target audience require? | Linux/X11 is the evidenced target; Windows, macOS and Wayland remain unverified. |
| 6 | Sustainable distribution | What rights, dependencies, maintenance and packaging obligations would a pilot create? | No shipping-art rights, public release or service owner inferred. |

## Candidate capabilities after an authorization decision

Local pack selection and recovery; motion refinement; a simpler creator walkthrough; undo/redo; reduced-motion controls; packaging; additional rights-cleared characters; platform adapters; and optional consented Assist.

These are alternatives to prioritize from evidence, not committed scope. Each needs a clear user outcome, compatibility and privacy boundaries, tests, an owner and explicit implementation authorization. A model or connector must not become a dependency of the offline companion.

The Assist alternatives are a manual handoff, a suitable local model, or an organization-approved external provider. No provider, retention policy, real-company-data use or continuous monitoring is authorized.

## Decision triggers

- Favor the smallest supported opportunity over a speculative subsystem.
- Preserve CLI behavior, pack schema, assets and privacy defaults unless an explicitly authorized change says otherwise.
- Preserve original failing evidence for defects before proposing a fix.
- Report automated checks, live desktop validation and consumer research separately.
- Treat unassigned sponsorship, art rights, support and security/privacy accountability as open decisions.
- At each planning gate, choose continued discovery, a bounded proposal or stop; do not automatically begin implementation.

See [product brief](PRODUCT_BRIEF.md), [team model](TEAM_EXECUTION.md) and [product discussion](PRODUCT_CONVERSATION.md).
