# Deskling planning and conditional implementation guidance

**Current phase: planning and strategy only.** ITIL 4 and CSDM 5 govern the product and service plan. Do not create implementation branches, change application code, launch development agents, merge or deploy based on brainstorming, an issue, or this file. Implementation requires an explicit scoped authorization.

When implementation is authorized, work as a **scoped engineer**, not a business owner, product sponsor, release manager or self-authorizing reviewer.

## Read first
- `docs/PRODUCT_CONSTITUTION.md` — non-negotiable behavior and decision authority.
- `docs/SERVICE_MODEL.md` — ITIL 4 value assessment, CSDM 5 conceptual model and decision register.
- `docs/TECHNICAL_BASELINE.md` — current code evidence and optional technical diagrams.
- `docs/FEATURE_ROADMAP.md` — prioritized discovery opportunities, not an implementation work order.
- `docs/TEAM_EXECUTION.md` — lane ownership, acceptance, independent verification.
- `docs/ITERATION_PLAN.md` — end-of-iteration gates.
- The issue or task assigned to you and the relevant source/tests.

## Working agreement after explicit implementation authorization
1. Confirm the exact branch/HEAD and existing functionality; do not claim existing features are missing.
2. Work only in your assigned lane and issue. Do not rewrite the architecture, pack schema or artwork to make a task easier.
3. Start from a measurable expected outcome. For a bug, preserve the first failing regression test/evidence. For a new feature, write observable acceptance tests before or alongside implementation and run them.
4. Favor minimal local-first, offline, user-consented implementations. Character packs stay inert JSON/PNG. Do not add a network call, agent dependency, telemetry, monitoring, remote asset fetch or dynamic pack code.
5. Keep existing `--pack`, strict validation and settings semantics compatible unless an approved ADR directs otherwise.
6. Run relevant tests and `sh scripts/verify.sh` where available. Provide exact pass/fail outputs and CI run URLs. Never equate Qt offscreen rendering with a real X11 desktop test.
7. Prepare one reviewable PR with clear scope, testing, screenshots/video where possible and known risks. Do not merge, publish releases, install/autostart or change user settings without authorization.
8. Decide routine reversible implementation details yourself; document tradeoffs. Escalate to the lead architect for cross-lane APIs, security/privacy, legal rights, irreversible changes, or missing authorization—not for ordinary coding choices.
9. Give the architect a structured handoff: issue, PR, commit, acceptance/tests, original failure, result, what remains, blockers, and reviewer action. Do not declare complete without evidence.
10. After each iteration, add short evidence-backed retrospective observations to the project review. Never invent human review, commercial approval, deployment or user-test results.

## Planning priorities
- First use, delight, customization, trust, and measurable feedback.
- Today's Deskling is an X11-first Rust/Qt desktop companion with existing Avatar Factory and Avatar Studio. **Use existing features before creating replacements.**
- Claude Haiku is the proposed primary implementation model when implementation is authorized; the lead architect/lead developer owns final architecture and integration decisions. Business/product sponsorship is unassigned.

## Stop conditions
Missing explicit rights to distribute artwork; unsafe requested permission/data access; release/merge authority not granted; inability to verify critical behavior; contradictory security or product requirements. Report the issue with a safe alternative, then continue on another independent, already-authorized task where possible.
