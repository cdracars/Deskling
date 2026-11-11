# Deskling — 3–4 Haiku Developer Delivery Operating Model

**Status:** Proposed execution contract, effective when an implementation team begins work.  
**Working model:** Claude Haiku is the primary implementation model for three or four independent development lanes. ChatGPT acts as **lead architect, solution architect and lead developer**: sets architecture and acceptance contracts, owns cross-lane integration decisions, reviews risky changes and curates evidence for product discussion. No external agent is claimed to be running merely because this document exists.

## Team map and strict file ownership

| Lane | Primary responsibility | Normal owned paths | Avoid editing without coordination |
| --- | --- | --- | --- |
| **H1 — Runtime UX** | Character selection, first-run hint, interaction controls, safe fallback | `crates/deskling-desktop/native/bridge.{hpp,cpp}`, `crates/deskling-desktop/qml/Main.qml`, `crates/deskling-desktop/src/main.rs`, relevant runtime tests | Factory and Studio internals; animation artwork |
| **H2 — Motion and character QA** | Baseline motion tests, animation improvements, source sprite/pack checks | `assets/characters/`, `tests/test_kipwick.py`, animation QA fixtures and documented review evidence | QML/bridge **unless** H1 agrees to interface contract |
| **H3 — Creator path** | Avatar Studio usability and rig export journey | `tools/avatar-studio/`, `tools/avatar-factory/`, `tests/avatar_studio_test.cpp`, creator docs | Native runtime loader or pack schema without architect ADR |
| **H4 — Trial, QA and release evidence** | Reproducible Ubuntu demo, test harness and independent X11 validation | `scripts/`, `.github/workflows/`, `docs/VERIFICATION.md`, `tests/test_launcher.py`, evidence ledger | Runtime/artist source edits; H1–H3 feature branches |

**If only three developers:** H4 owns first-use demo and QA; H2 temporarily owns motion plus visual tests; H1 owns runtime; H3 owns Studio. Lead developer coordinates independent verification and product evidence. If capacity is insufficient, defer lower-priority features rather than mixing file ownership.

## Lead architect/developer operating contract

- Own and maintain constitution, C4, ADRs, public API/pack boundaries and product-to-feature traceability.
- Convert product hypotheses into acceptance criteria; make routine reversible technical decisions without asking the repository owner to act as the business.
- Assign non-overlapping feature tasks and define integration order. Prefer one scoped branch and PR per lane; no direct-to-main work.
- Set test evidence expectations and verify results instead of trusting agent summaries. Preserve first failing regression for defect fixes.
- Resolve cross-boundary changes (pack format, native Rust/Qt API, persistence semantics), review tests and own final integration recommendation.
- Consolidate reports into an evidence ledger and product-discussion packet. No automatic merging, deploying, publishing or licensing approval.

## Haiku task contract — required for every assigned issue

Each assigned Haiku thread must receive:

```text
Deskling development task
Repository: cdracars/Deskling
Base: latest main (verify HEAD before editing)
Constitution: docs/PRODUCT_CONSTITUTION.md
C4: docs/ARCHITECTURE_C4.md
Roadmap: docs/FEATURE_ROADMAP.md
Assigned issue: [GitHub link]
Owner lane / allowed paths: [one lane]
User-facing outcome: [observable behavior]
Current evidence: [existing code/tests/known defect]
Do not do: [explicit out-of-scope or locked paths]
First: inspect source and write acceptance/regression test (for a bug,
preserve first failure). Implement only then. Do not claim live desktop
validation from Qt offscreen output. Provide exact commit/CI refs.
Report to lead architect when: blocked by interface, art rights,
security/privacy, release authority or a genuinely irreversible choice.
Otherwise make conservative implementation decisions and continue.
Deliver: PR, tests/CI evidence, X11 results if relevant, before/after,
risks, and a short reviewer checklist. No merge or release.
```

## Delivery states and independent verification

- `ready`: scope, owner, acceptance and test method identified.
- `working`: branch/PR opened; first test/fixture evidence captured.
- `review`: code plus exact-commit tests and artifacts are linked.
- `verified`: independent reviewer checked expected behavior and residual risks.
- `done`: merge approved/executed by authorized maintainer; issue closed with reference.
- `blocked`: named dependency, failed evidence or missing approval; propose lowest-risk alternative.

The implementation agent must not be its sole acceptance reviewer. H4 can provide verification for H1/H2/H3; H1 or H3 independently reviews H4 changes where appropriate. The architect decides interfaces and reviews risk.

## Branch and integration discipline

- Use `feat/deskling-i1-<short-name>` or `fix/deskling-i1-<short-name>`.
- Start from current `main`; recheck current head and PR diff before integration.
- Keep change size reviewable. Tests in the same PR; do not alter large artwork casually.
- Avoid concurrent writes to `Main.qml`. H2 documents desired motion changes and hands requirements to H1 if runtime behavior must change.
- Run `sh scripts/verify.sh` in capable environment/CI; record what executed locally versus remotely.
- Native desktop requirements need X11 human/manual evidence. Offscreen CI is not equivalent.
- PR must include screenshots/video for UI changes, failing/passing regression, risk and rollback notes.
- No merge, actual deployment, binary release, user installation, autostart, credential access or business approval without authority.

## Escalation and retrospective

**Escalate only:** scope/architecture conflicts, legal rights, security/privacy changes, inability to verify, irreversible user/system actions, product decision needing sponsorship or cross-lane dependency that cannot be resolved from documented contracts. Otherwise proceed and document the choice.

At the end of each iteration, record: what users demonstrably accomplished, cycle time/defects, original failures preserved, friction in developer/Haiku workflow, technical debt, what was deferred and what to adjust. Feed changes into the roadmap or a reviewed constitution amendment. The retrospective is evidence, not self-congratulation.

## Immediate work order

1. H4: isolated user-facing demo entry and real X11 validation matrix.
2. H1: safe local character selection/first-run controls, protecting `--pack` and current loader.
3. H2: baseline and top visible Kipwick motion defects, original art unchanged until approved.
4. H3: five-minute visual editor walkthrough and evidence of edit -> save -> export -> run.
5. Lead architect: review each handoff, run integration gate and prepare Iteration-1 demo; update product brief from actual evidence.

A small number of fully verified user-visible improvements is better than many partially complete speculative features.
