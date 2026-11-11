# Deskling two planning gates

**Status:** Planning and strategy only. No implementation, development-agent launch, merge, pilot or deployment is authorized by this plan.
**Foundation:** [ITIL 4 and CSDM 5 service model](SERVICE_MODEL.md), [constitution](PRODUCT_CONSTITUTION.md).
**Goal:** Prepare a credible product and organizational adoption conversation within two planning iterations. Iteration duration and capacity are not assumed.

## Iteration 1 Discovery and product definition

**Outcome:** An evidence-backed recommendation about which consumer problem to investigate next.

1. Reconcile the existing runtime, creator tools, open PRs and test evidence. Reuse merged capabilities instead of proposing replacements.
2. Compare enthusiast, creator and workplace-assistance hypotheses by consumer outcome, current fit, cost, risk and cost of learning.
3. Identify onboarding, control, creator-workflow and platform constraints from available evidence. Separate observed defects from proposed improvements.
4. Prepare an opt-in qualitative research plan, including success criteria and synthetic Assist scenarios. Do not recruit participants, contact organizations or process real company screenshots without authorization.
5. Prioritize an opportunity backlog and record ownership, rights and evidence gaps.

**Exit evidence:** current-state assessment; value proposition and audience recommendations; prioritized opportunities; platform, privacy and rights constraints; a clear next learning question. Mark unmeasured outcomes as unmeasured. A proposed 3–5 person qualitative study is not statistical validation or permission to run it.

## Iteration 2 Product readiness and adoption planning

**Outcome:** A product conversation that can result in a bounded pilot, more discovery or a stop decision.

1. Refine the strongest proposed offering and the consumer outcome it should support.
2. Apply ITIL 4's four dimensions to people, technology, partners and the value stream.
3. Use the CSDM 5 conceptual model to distinguish ideas, product capabilities, build components, future service delivery and consumer offerings. Do not invent operational records or accountable owners.
4. Define technical feasibility questions, supported-platform options, adoption and withdrawal experience, support obligations, privacy boundaries, rights and cost drivers.
5. Prepare the product discussion with an explicit proposed scope, success criteria, risks and decisions for authorized stakeholders.

**Exit evidence:** product opportunity assessment; ITIL 4 service value assessment; CSDM 5 conceptual model; decision register; prioritized roadmap; adoption/support model; ownership gaps; and an explicit proceed, investigate or stop proposal.

A planning gate does not authorize implementation, installation, a public release or an organizational pilot.

## Existing evidence and future acceptance

Main at `b41ff3daf29967fa0b35b290f6d9ae1218393ec6` includes Avatar Factory, the Rust rig renderer and Qt Studio. See [README](../README.md), [Studio](AVATAR_STUDIO.md), [technical baseline](TECHNICAL_BASELINE.md) and [verification](VERIFICATION.md).

Preserve the existing quick-win PRs: [isolated trial #12](https://github.com/cdracars/Deskling/pull/12) and [interaction hint #13](https://github.com/cdracars/Deskling/pull/13). Keep [Screen Assist #15](https://github.com/cdracars/Deskling/pull/15) parked as an unmerged prototype. Their existence does not authorize further implementation.

For a future authorized desktop evaluation, record exact revision, OS, Qt, compositor/window manager, steps and expected/actual behavior. Cover launch, transparency, drag/focus/stacking, repeated and interrupted interactions, pause/quit, position recovery, settings isolation and cleanup. For Studio, cover edit, save, reopen, export, visible runtime output and safe failure without modifying source artwork.

Offscreen CI cannot establish live desktop behavior or consumer acceptance. Missing desktop evidence and unresolved redistribution rights remain visible in the product discussion.

## Scope control

The architect owns routine technical recommendations and consolidates the decision package. Product sponsorship, service/support ownership, art rights, privacy/security approval and release authority remain explicit roles rather than assumed approvals.

Existing issues and acceptance scenarios are candidate future scope. Start a development assignment only after its concrete outcome, boundaries, owner, tests and authorization are established. Preserve independent review and exact-commit evidence.
