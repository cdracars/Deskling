# Deskling Product Constitution

**Status:** Planning and strategy only. ITIL 4 and CSDM 5 govern product and service planning; business sponsorship and product approval remain unassigned.  
**Baseline:** `main` at `b41ff3daf29967fa0b35b290f6d9ae1218393ec6` (2026-11-06).  
**Scope:** The Deskling Linux desktop companion and its Avatar Factory/Studio authoring tools.  
**Purpose:** Govern the next two iterations and produce a credible, evidence-backed product discussion. Not a declaration of a commercial launch.

## Governing framework and current authority

Use **ITIL 4** for value creation, service relationships, the four dimensions, relevant management practices and continual improvement. Use **CSDM 5** for the conceptual product, capability, ownership and lifecycle model. [Service model and decisions](SERVICE_MODEL.md) applies these frameworks to Deskling.

C4 is optional notation for technical diagrams only. The former C4-led product plan is retired. Current-system evidence is retained in [Technical baseline](TECHNICAL_BASELINE.md); it cannot authorize product scope.

**Remain in planning and strategy until implementation is explicitly authorized.** Do not create implementation branches, change application code, launch development agents, merge PRs or deploy based solely on brainstorming or a planning document. Authorized planning-document corrections do not open an implementation phase.

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
| Engineering architecture, interfaces, ADRs and technical tradeoffs | Lead architect (assistant in this engagement) | Makes routine architecture recommendations and prepares planning artifacts; code changes require explicit implementation authorization. |
| Engineering delivery and code quality | Engineering implementer / lead (assigned per issue) | Implements only within explicit authorization and approved criteria; independent reviewer verifies. |
| Customer segment, positioning, pricing, funding, distribution and commercial licensing | **Product/business sponsor: TBD** | Must be decided by a human authorized by the organization. The repository owner is **not presumed** to be this sponsor. |
| Character art approval and reuse rights | Art/brand owner and legal/IP reviewer: TBD | Written provenance and distribution permission are required before release. |
| Privacy/security exceptions and agent capabilities | Assigned security/privacy reviewer and authorized approver: TBD | No permission expansion, surveillance or credential handling by architectural assumption. |
| Merge, rollout and externally visible release | Authorized repository/release maintainer | Follow explicit permissions, review and release gates. |

The lead architect should resolve normal technical choices without asking the user to act as a product manager. Unowned commercial choices are written as open decisions for an eventual business sponsor, not treated as blockers to safe architecture work.

## Article IV — Decision and escalation policy

- **Decide locally during planning:** conceptual component boundaries, testing strategy, documentation, technical evaluation criteria and recommendations. Implemented UX prototypes, code changes and development branches require explicit authorization.
- **Record as ADR:** material tradeoffs affecting security, packaging, runtime/authoring boundaries, target OS, character delivery or integration contracts.
- **Elevate to authorized humans:** product-market claims, target audience commitment, revenue model, artwork rights, distribution license, collection/processing of personal data, broad system permissions, release publication and irreversible changes.
- **Do not invent approval:** absence of an owner is `UNASSIGNED`, not implied consent. No AI service or cron principal may approve a human-only gate.

## Article V — Delivery discipline

After explicit implementation authorization, every feature travels through:

1. Product hypothesis or observed problem with a named user-facing outcome.
2. Short behavior specification: normal path, invalid input, interruption/rollback and quality thresholds.
3. Observable tests and a saved baseline failure for changes correcting a defect.
4. Implementation with no scope creep.
5. Automated verification, *plus* X11 desktop evidence for desktop UX claims.
6. Independent review, including licensing/privacy where relevant.
7. Demonstration and artifact handoff; only then consider merge or release.

**Definition of Done:** requirements trace to tests; automated suite for the exact commit passes; manual desktop checks are attached when necessary; known gaps are explicit; documentation and changelog/decision record are updated; release authority has not been bypassed.

## Article VI — Two planning gates

**Iteration 1: Discovery and product definition.** Establish current state, consumer outcomes, audience hypotheses, opportunity backlog, platform and rights constraints, and prioritized questions.

**Iteration 2: Product readiness and adoption planning.** Refine proposed offerings, conceptual architecture, technical feasibility, operating responsibilities, support, privacy, cost drivers and an evidence-backed product discussion.

Exit with a product opportunity assessment, ITIL 4 service value assessment, CSDM 5 conceptual model, decision register, roadmap, adoption/support proposal and explicit choices requiring sponsorship. Record missing live-demo or consumer evidence rather than treating planning as permission to build it. These gates do not authorize implementation, research outreach, a pilot or release. See [Iteration plan](ITERATION_PLAN.md).

## Article VII — Amendment process

Change this constitution in a reviewed pull request with: rationale, impacted principles, alternatives, evidence, risk, and an owner of the approval. Security/privacy and distribution-rights constraints cannot be waived by a routine technical PR. Record the decision in an ADR. Product/business ownership remains unassigned until a legitimate sponsor accepts it.

**Companion documents:** [Product brief](PRODUCT_BRIEF.md) · [Service model](SERVICE_MODEL.md) · [Technical baseline](TECHNICAL_BASELINE.md) · [Two planning gates](ITERATION_PLAN.md) · [Product discussion](PRODUCT_CONVERSATION.md).
