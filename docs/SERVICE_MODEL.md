# Deskling ITIL 4 and CSDM 5 planning model

**Status:** Governing product and service planning framework. Planning and strategy only; no implementation or organizational service is authorized.
**Revision:** 11 November 2026.
**Authority:** [Product constitution](PRODUCT_CONSTITUTION.md).

## ITIL 4 service value assessment

Use the Service Value System to connect demand and opportunity, governance, relevant management practices, delivery and continual improvement. Start with the consumer outcome, cost and risk rather than a feature list.

| Dimension | Deskling planning application |
| --- | --- |
| Organizations and people | Separate technical stakeholder, architect, sponsor, support, art and privacy/security responsibilities. Unassigned roles stay unassigned. |
| Information and technology | Preserve offline behavior and inert packs. Evaluate platform constraints and any future Assist data flow, retention and failure behavior. |
| Partners and suppliers | Record artwork rights, Qt distribution obligations, platform dependencies and potential provider obligations. |
| Value streams and processes | Trace discovery, authorized evaluation, evidence review, change decisions, readiness and support. A passing build does not establish adoption. |

Apply the guiding principles: focus on value, start where the product is, progress iteratively with feedback, collaborate visibly, think holistically, keep the process practical, and optimize before automating.

Relevant practices include relationship management, service design, portfolio management, change enablement, release/deployment management and continual improvement. Planning these responsibilities does not create an operational service or authorize a release.

## CSDM 5 conceptual model

| Domain | Conceptual application |
| --- | --- |
| Foundation | Product identity, capabilities, provenance, roles, ownership and lifecycle vocabulary. |
| Ideation and Strategy | Enthusiast, creator and workplace opportunity hypotheses and evidence. |
| Design and Planning | Proposed offerings, consumer journeys, information flows, platform options and solution decisions. |
| Build and Integration | Existing runtime, Factory, rig renderer, Studio, repository revisions, PRs and test evidence. |
| Service Delivery | Future supported instances, dependencies and operational accountability, if approved. No managed enterprise service is established by the repository. |
| Service Consumption | Candidate Companion, Creator and Assist offerings, intended consumers and adoption channels. |
| Manage Portfolio | Continue, narrow, fund a bounded pilot, investigate further or stop; ownership and lifecycle choices. |

Recommend one product with Companion and Creator as implemented capability areas and Assist as a hypothesis. This is a conceptual map, not created ServiceNow records, catalog entries or CMDB configuration items.

## Decisions and boundaries

| Decision | Status |
| --- | --- |
| ITIL 4 and CSDM 5 govern the product/service plan | Established |
| Retire the C4-led product plan | Applied in this documentation revision |
| Retain current-system diagrams only as optional technical evidence | Established boundary |
| Remain in planning until implementation is explicitly authorized | Established |
| One product with Companion, Creator and Assist capability areas | Recommendation |
| Validate enthusiasts first, creators second; explore workplace value | Recommendation |
| Enterprise platforms, support and service accountability | Open |
| Product/business sponsor and art rights approval | Unassigned or unresolved |
| Real-workplace data processing and model provider | Not authorized |
| Implementation, pilot, merge and release | Require separate scoped authorization |

The [technical baseline](TECHNICAL_BASELINE.md) retains useful component and trust-boundary evidence. C4 notation is optional for those diagrams; it is not a product plan.

## Existing work disposition

- Preserve [PR 12 isolated trial](https://github.com/cdracars/Deskling/pull/12) and [PR 13 discovery hint](https://github.com/cdracars/Deskling/pull/13) for later acceptance review.
- Keep [PR 15 Screen Assist](https://github.com/cdracars/Deskling/pull/15) parked as an unmerged prototype. Capturing and previewing an image does not interpret its contents.
- Issues 7–10, 14 and 16 are candidate future scope, not authority to begin development.
- Preserve exact-commit CI and existing code history. Automated checks, desktop observation, consumer research, approval and release are distinct evidence states.

## Change record

The former C4-led plan is superseded by the [two planning gates](ITERATION_PLAN.md). Historical documentation remains available at commit [9d0cd45e](https://github.com/cdracars/Deskling/commit/9d0cd45e8e2790e008e9dea26a1160c66bf8ce5b); it is not the current work order.

This revision changes planning documents only. It does not modify runtime code, tests, artwork, settings, pack schema or deployment configuration.
