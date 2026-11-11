# Deskling — Product Hypothesis and Brief

**Status:** Working hypothesis for a product discovery meeting. Not an approved business plan.  
**Product stage:** Source prototype with merged authoring pipeline; no public binary release.  
**Baseline evidence:** [README](../README.md) · [Avatar Factory](AVATAR_FACTORY.md) · [Avatar Studio](AVATAR_STUDIO.md) · [main CI](https://github.com/cdracars/Deskling/actions/runs/42234626006).

## The one-sentence pitch

**A small, expressive desktop companion you can actually make your own—offline, lightweight, and without surrendering your desktop to an AI service.**

## Problem and opportunity (hypotheses, not established market facts)

Linux users who like desktop customization and expressive character experiences face a gap between decorative widgets, complicated DIY sprite workflows, and large app/agent experiences that require accounts or services. Deskling could bridge that with a friendly desktop presence and a repeatable toolchain for creating custom characters.

**Initial audience hypotheses:** Linux desktop tinkerers; makers/creators comfortable with local files; character artists/streamers who want a desktop companion. We have not validated demand, willingness to pay, target size, or organization fit. Do not claim product-market fit.

## Proposed user journey for an Iteration-2 demonstration

1. Obtain a reproducible Ubuntu 24.04/X11 test build without altering an existing user setup.
2. Launch a visibly appealing, authorized character, sized sensibly on screen.
3. Watch fluid idle/gesture motion; click to interact, drag it, adjust size, pause and quit.
4. Open a simple pack-selection path or documented workflow and swap to a second approved character. The user can recover if a pack is invalid.
5. Launch Avatar Studio, change one rig keyframe and export a native pack. Run that pack in the desktop app.
6. Show everything operating offline. Explain that optional future agent connectors are deliberately **not** present.

The packaged candidate must not silently ship private/unlicensed art. The existing Kipwick alpha is a development asset, not automatically approved launch art. The synthetic rig is proof of a toolchain, not a production-quality character.

## Now / next / later

| Now — next two iterations | Next — only after product gate | Later — optional |
| --- | --- | --- |
| Great first-run experience, polished approved character, real desktop validation, reproducible demo/install path, pack switching, explicit human UX review | Creator workflow polish, additional approved characters, performance tuning, accessibility, distribution decision | Local-agent integration adapters, events, customization ecosystem, Wayland path after a separate feasibility decision |
| Keep local-first, no account, no telemetry | Validate audience, retention, cost, supportability | Permissioned plugin model, **never** silent desktop control |

## What currently exists versus does not

**Verified in repository:** Rust/Qt X11 runtime; validated inert PNG/JSON packs; Kipwick alpha and Cyber Gremlin study; Rust Avatar Factory; layered CPU rig renderer; Qt Avatar Studio; CI on merge commit `b41ff3d`.

**Not verified/implemented as a product claim:** general Ubuntu desktop interoperability beyond test setup; polished approved shipping character; packaged release or one-click installer; native X11 animation quality validated by actual desktop recording; automatic image-to-avatar creation; Wayland; AI connectors; commercial rights to artwork.

## What counts as success in the product conversation

An authorized sponsor can watch a 90-second demo, explain the core value in one sentence, name an intended first audience, understand the limits and cost/risk, and decide to fund/authorize a pilot, request evidence, or stop. Engineering can show evidence rather than architecture promises.

**Pilot feedback proposal (requires opt-in research):** have 3–5 prospective users independently launch and interact; record completion/friction and whether they would keep it installed. This small sample is *qualitative*, not statistical validation. Do not collect hidden telemetry.

## Hard questions we want a sponsor to answer

- Who is Deskling for, and what user job are we committing to?
- Is this an open-source community project, a licensed character product, an internal employee experiment, or a commercial offering?
- Who owns brand, art licenses and permission to distribute?
- Who funds support, maintenance, packaging and future compatibility?
- What does a successful small pilot look like, and who can approve it?

See the [product conversation](PRODUCT_CONVERSATION.md) for decision record and meeting structure.
