# Deskling Product Constitution

**Status:** Proposed architecture baseline; business sponsorship and product approval are not yet assigned.  
**Baseline:** `main` at `b41ff3daf29967fa0b35b290f6d9ae1218393ec6` (2026-11-06).  
**Scope:** The Deskling Linux desktop companion and its Avatar Factory/Studio authoring tools.  
**Purpose:** Govern the next two iterations and produce a credible, evidence-backed product discussion. Not a declaration of a commercial launch.

## Article I — Product mission

Deskling is a **local-first, user-controlled, customizable Linux desktop companion**. It should bring an expressive character to a user's desktop without requiring an account, cloud service, or AI model. Optional integrations may provide utility later, but an offline Deskling must be worth using on its own.

The value hypothesis, **not yet validated by customers**, is that expressive desktop presence plus a practical creator workflow is more differentiated than either a generic animation overlay or another embedded chat window.

## Article II — Non-negotiable engineering principles

1. **Offline is the default.** Startup, pack loading, animation, editing and export must function without Internet or agent access.
2. **The character engine is not an AI dependency.** No model or connector may be necessary for the core app to render, animate, pause or quit.
3. **User agency is explicit.** Users choose packs, can pause and quit, and must explicitly enable optional capabilities. No hidden background surveillance, desktop scraping, or implicit login/session access.
4. **Content is inert.** Runtime packs contain validated local JSON and PNG data, not executable scripts, remote URLs, actions or credentials. Preserve the existing strict size/path validation boundary.
5. **Separate concerns.** Rust core and rig renderer, Qt runtime, Qt authoring tools, character assets and optional future integrations remain separate architectural boundaries.
6. **Local-first privacy.** No telemetry, accounts, cloud dependency or automated remote loading in the initial product. Any future telemetry is opt-in with a separately approved privacy design.
7. **Character quality is a product requirement.** A synthetic QA fixture is not a marketable mascot. Art continuity, occlusion/seams, motion quality and real-desktop behavior require human visual acceptance.
8. **Preserve provenance and licenses.** MIT code does not imply that Kipwick art, the mascot image, icons or third-party characters can be commercially distributed. No general release before art usage rights and Qt distribution obligations are resolved.
9. **Accessibility and resource use matter.** Pause/motion controls, recoverable window positioning, size choices, keyboard access and test-device CPU/memory behavior must be demonstrated—not guessed.
10. **Evidence before confidence.** Claims require a reproducible test, artifact, documented inspection, screenshot/video or identifiable source. CI/offscreen tests do not prove X11 compositing or subjective animation quality.
11. **Specifications and tests precede risky implementation.** Define observable acceptance criteria; where a bug or behavior change is involved, preserve the first failing regression test before the fix; verify independently.
12. **Small reversible steps.** Prefer well-scoped, demonstrable changes. No silent merges, releases, installations, autostart, secret access or permanent settings changes outside the authorized workflow.

## Article III — Decision authority (do not invent a business owner)

| Decision | Accountable role | Operating rule |
| --- | --- | --- |
| Engineering architecture, interfaces, ADRs and technical tradeoffs | Lead architect (assistant in this engagement) | Can propose and prepare reversible changes; reviews against constitution and tests. |
| Engineering delivery and code quality | Engineering implementer / lead (assigned per issue) | Implements from approved criteria; independent reviewer verifies. Codex may fill this role where available. |
| Customer segment, positioning, pricing, funding, distribution and commercial licensing | **Product/business sponsor: TBD** | Must be decided by a human authorized by the organization. The repository owner is **not presumed** to be this sponsor. |
| Character art approval and reuse rights | Art/brand owner and legal/IP reviewer: TBD | Written provenance and distribution permission are required before release. |
| Privacy/security exceptions and agent capabilities | Assigned security/privacy reviewer and authorized approver: TBD | No permission expansion, surveillance or credential handling by architectural assumption. |
| Merge, rollout and externally visible release | Authorized repository/release maintainer | Follow explicit permissions, review and release gates. |

The lead architect should resolve normal technical choices without asking the user to act as a product manager. Unowned commercial choices are written as open decisions for an eventual business sponsor, not treated as blockers to safe architecture work.

## Article IV — Decision and escalation policy

- **Decide locally:** reversible component boundaries, testing strategy, code organization, docs, UX prototypes in isolated branches, and technical evaluation criteria.
- **Record as ADR:** material tradeoffs affecting security, packaging, runtime/authoring boundaries, target OS, character delivery or integration contracts.
- **Elevate to authorized humans:** product-market claims, target audience commitment, revenue model, artwork rights, distribution license, collection/processing of personal data, broad system permissions, release publication and irreversible changes.
- **Do not invent approval:** absence of an owner is `UNASSIGNED`, not implied consent. No AI service or cron principal may approve a human-only gate.

## Article V — Delivery discipline

Every feature travels through:

1. Product hypothesis or observed problem with a named user-facing outcome.
2. Short behavior specification: normal path, invalid input, interruption/rollback and quality thresholds.
3. Observable tests and a saved baseline failure for changes correcting a defect.
4. Implementation with no scope creep.
5. Automated verification, *plus* X11 desktop evidence for desktop UX claims.
6. Independent review, including licensing/privacy where relevant.
7. Demonstration and artifact handoff; only then consider merge or release.

**Definition of Done:** requirements trace to tests; automated suite for the exact commit passes; manual desktop checks are attached when necessary; known gaps are explicit; documentation and changelog/decision record are updated; release authority has not been bypassed.

## Article VI — Two-iteration product discussion gate

By the end of Iteration 2, the product conversation must have: (a) a running, recorded local demo; (b) a clear proposition and target-user hypotheses; (c) a reproducible install/build/test path; (d) C4 architectural views; (e) risks and licensing gaps; (f) explicit choices requiring sponsorship; and (g) evidence versus speculation called out separately. **This is a discovery/pilot decision gate, not a release promise.**

## Article VII — Amendment process

Change this constitution in a reviewed pull request with: rationale, impacted principles, alternatives, evidence, risk, and an owner of the approval. Security/privacy and distribution-rights constraints cannot be waived by a routine technical PR. Record the decision in an ADR. Product/business ownership remains unassigned until a legitimate sponsor accepts it.

**Companion documents:** [Product brief](PRODUCT_BRIEF.md) · [C4 architecture](ARCHITECTURE_C4.md) · [Two-iteration delivery](ITERATION_PLAN.md) · [Product discussion](PRODUCT_CONVERSATION.md).
