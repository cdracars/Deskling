# Deskling — Product Discussion & Organizational Utilization

**Meeting status:** Proposed decision conversation, not a sales pitch promising an existing product.  
**Target:** End of Iteration 2.  
**Meeting objective:** Decide whether an accountable sponsor should authorize a small, time-boxed user-discovery or pilot phase.

## Executive narrative (for a 5–10 minute meeting)

1. **What it is (30 seconds).** Deskling is a local-first Linux desktop companion with expressive custom characters. It does not require an account, online model or intrusive system permissions.
2. **Why anyone might care (45 seconds).** Personalization and desktop presence could make a small utility enjoyable. The premise is a hypothesis; we need to validate demand.
3. **See it (90 seconds).** Launch, wave, drag, blink, resize, pause, change a character; edit a joint in Avatar Studio, export and run the result.
4. **What's real (60 seconds).** Show the current source, merged Rust rig/Qt editor, exact-commit CI and recorded real-X11 evidence. Identify unfinished design and packaging work.
5. **How it's built (60 seconds).** Explain four C4 views, native inert JSON/PNG packs, optional separate creator workflow and no cloud dependency. Future integrations are possibilities, not current abilities.
6. **How an organization might use it (60 seconds).** Present three **candidate** utilization modes below and their owners/risks. Do not assert that a specific employer has endorsed it.
7. **The ask (60 seconds).** Identify business sponsor, art/IP rights owner and target audience; choose pilot/discovery, request more evidence, or stop.

## Organizational utilization choices — proposals, not approvals

| Mode | Who would use it | Potential outcome | What must be true before adoption |
| --- | --- | --- | --- |
| Personal/open-source maker project | Linux enthusiasts and character creators | A friendly, modifiable desktop experience and community-created packs | Supported distro scope, source distribution policy, art provenance, reproducible setup, maintenance owner |
| Organization-specific branded companion | Employees in a voluntary internal pilot | A branded desktop presence, optionally later linked to approved employee information | Authorized sponsor, security/privacy review, voluntary installation, helpdesk/IT support owner, brand/IP approval, no employer monitoring |
| Creator toolkit / pack ecosystem | Artists, builders, modding communities | Users author and distribute inert character packs | Safe importing, creator documentation, provenance/rights model, pack compatibility, quality standards, moderation/support decision if a marketplace is proposed |

**No organization has authorized deployment from these options.** Deskling currently has neither administrative controls for fleet rollout nor an enterprise account, permissions or management service. Treat organizational branding, enterprise integration and packaging as product proposals requiring new scope.

## Operating model proposal for any future pilot

- **Product/business sponsor (unassigned):** select one audience, own success criteria, budget, support and commercial decisions.
- **Architecture/engineering lead:** keep system boundaries and risks explicit; own technical recommendation and evidence.
- **User-experience/art reviewer (unassigned):** own approved character quality, visual acceptance and brand consistency.
- **Security/privacy/legal reviewers (unassigned):** establish approved usage and artwork/Qt distribution boundaries; no implied permission by silence.
- **Pilot participants:** opt in, can uninstall and pause at any point; no stealth instrumentation.
- **Support/release owner (unassigned):** document supported Ubuntu/X11 platform, maintenance, updates and rollback; no auto-install/autostart without consent.

### Suggested small pilot (requires sponsor approval)

- Start with **3–5 consenting users** in the chosen audience for qualitative learning; this is not a statistical product-market-fit measure.
- Give them a non-destructive, documented test build and a one-page task sheet; no privileged install.
- Observe: can they launch it without assistance, understand pause/quit, discover interactions, customize a pack, and recover from a problem?
- Ask (without covert tracking): what delighted or annoyed them, whether they'd retain it after a week, what feature would change their mind, and whether the tool fits their workflow.
- Collect opt-in interview notes and failure logs only with informed consent, clear retention and no sensitive desktop content.
- Follow-up *decision*, not automatic rollout: revise, narrow scope, expand authorized pilot or stop.

## Product versus technical readiness

| Topic | Engineering can prepare | Human business/organizational decision |
| --- | --- | --- |
| User journey | Working code, recordings, test evidence, measured friction | Which audience/job to prioritize |
| Architecture | C4, safe data boundaries, ADRs, operating costs/risks | Security exceptions or organizational deployment |
| Character content | Synthetic demo and technically valid packs | Rights, licensed art, branding and launch approval |
| Distribution | Reproducible test kit, documented dependency/runtime | Commercial model, release terms, maintenance funding |
| Integrations | Proposed contracts and threat model, **not implemented** | Whether integrations are desirable and consent/compliance model |
| Success | Trial completion, qualitative feedback templates | What KPI justifies further investment |

## Decisions to bring out of the meeting

| Decision ID | Choice | Current status |
| --- | --- | --- |
| P-001 | Name an accountable product/business sponsor | **UNASSIGNED** |
| P-002 | Choose one primary audience and core use case | **OPEN — hypothesis only** |
| P-003 | Choose open-source/personal, branded internal, or creator-tool positioning for discovery | **OPEN** |
| P-004 | Identify approved art/brand rights and licensing reviewer | **UNASSIGNED** |
| P-005 | Set pilot budget, support/IT and retention obligations | **OPEN** |
| P-006 | Authorize limited, opt-in discovery pilot after safety and rights gates | **NOT AUTHORIZED** |

## Evidence checklist for the presenter

- [ ] Show exact revision and recent green CI run.
- [ ] Show **real** Ubuntu 24.04/X11 video of launch, gestures, drag, controls and quit.
- [ ] Show edited rig -> saved -> exported pack -> native runtime.
- [ ] State which character images have explicit redistribution rights; mark others **not cleared**.
- [ ] Show easy-to-follow clean start and cleanup, with prerequisites.
- [ ] Show current limitations: no published release, no Wayland, no accounts/agent connectors, no automatic character art generation.
- [ ] Separate verified evidence from product hypotheses and unverifiable assumptions.
- [ ] Ask for a decision and named owners; do not translate meeting enthusiasm into launch approval.

## Meeting output template

```text
Date / attendees:
Audience and user job chosen:
Decision: [authorize discovery pilot | request evidence | stop]
Business sponsor:
Security/privacy reviewer:
Artwork and license rights reviewer:
Support/release owner:
Authorized scope / budget / duration:
Evidence accepted:
Open questions and risks:
Next gate / owner / date:
```

**Reference:** [Constitution](PRODUCT_CONSTITUTION.md) · [Brief](PRODUCT_BRIEF.md) · [C4](ARCHITECTURE_C4.md) · [Iteration plan](ITERATION_PLAN.md).
