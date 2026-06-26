# Walking Skeleton as a Living Artifact

Why the walking skeleton rots while layers, waves, and release lines stay maintained — and how to make it a living, bet-anchored invariant. Grounded in Jeff Patton's _User Story Mapping_ and Alistair Cockburn's walking skeleton. Extends [release-line-adjustments.md](release-line-adjustments.md) and completes the Multi-Map Capability Structure proposed in [aep-v2-improvement-guideline.md](../aep-v2-improvement-guideline.md) §1.

> **Status:** Proposal (not yet implemented). This document records the design thinking precisely so that schema and skill changes can be made against it. It changes how AEP works, so it lives in `decisions/` per the [docs routing guide](../README.md).

> **Sourcing note:** The methodology synthesis below is grounded in the primary texts (Patton, _User Story Mapping_; Cockburn's walking-skeleton definition) and in AEP's own glossary and roadmap, which already encode the Patton model. Live web sourcing was unavailable when this was written (search quota exhausted); page-level citations can be back-filled.

---

## The Asymmetry

AEP's product-context concepts fall into two classes, and only one class survives contact with a long-running product:

| Concept                         | Who reads it, and when                                                                                                                                            | Outcome                                               |
| ------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------- |
| **Layer / Wave / Release Line** | `/aep-dispatch` reads them **every cycle** — picks the next story, batch-dispatches a wave, checks `layer_gates[N-1].status == passed` before advancing           | Load-bearing → continuously maintained                |
| **Walking Skeleton (Layer 0)**  | `/aep-envision` and `/aep-map` read it **at creation**; `/aep-validate` reads it once for **coverage** (every `layer_introduced: 0` activity has a Layer 0 story) | No **currency** reader → rots after the product moves |

The walking skeleton has a creation-time reader and a coverage reader. What it lacks is a **currency reader** — anything that re-asks "does Layer 0 still describe the product we are actually building?" Layers and waves are re-read on every dispatch, so drift is caught immediately. The skeleton is read once and then frozen, so drift is invisible.

**This is a framework-level gap, not a project-level discipline failure.** A downstream product (looplia) makes it concrete and unarguable.

---

## Diagnosis (looplia evidence)

Looplia maintains layer / wave / release-line faithfully across 23+ layers, yet its walking skeleton is lost. Three compounding root causes:

### A. The bet reshaped; the skeleton did not

`opportunity.bet` evolved on 2026-06-11 from "secure agent-to-remote-shell relay" to "auto-company / AI employees", with the relay demoted to a "power tier" (`looplia/product/index.yaml:9-13`, `evolved_at` at `:38`). But `Layer 0 — Walking Skeleton` (`:228-236`) still reads "sign up → install daemon → OAuth → connect via CLI → get a shell" — the **old** product's spine. AEP's [release-line-adjustments.md](release-line-adjustments.md) names the trigger ("Reshape (envision): change the backbone, reframe the product") and the glossary classifies it ("Opportunity Shift — fundamental bet is wrong → `/aep-envision` Phase 0"), but **neither path re-cuts the walking skeleton**. A reshape updates the bet and appends layers; Layer 0 is left frozen.

### B. One backbone assumed; ten capabilities shipped

The glossary defines the walking skeleton as "a horizontal slice across **the** Activity Backbone" (singular, `glossary.md:68-77`). Looplia has **10 capabilities**, each with its own `map_path → maps/<cap>/map.yaml`, and each map carries its own `backbone:` (e.g. `maps/auto-company/map.yaml` has six activities: hire-agents → assign-work → approve-decisions → oversee-company → direct-the-company → self-direct). The real backbone migrated **down** into capability maps, while the product-level `product.activities` (`index.yaml:478+`) degenerated into an **append-only list** — relay's six activities first, then everything ever added (launch-agent, hire-agents, watch-agents-learn, assign-work…). Read as a sentence it is now two products concatenated, not one coherent journey. There is no place in the model for a product-level skeleton that threads across capabilities.

### C. The framework's own quality gate can't see the rot

`/aep-validate` checks skeleton **coverage** (every `layer_introduced: 0` activity has a Layer 0 story) but never **currency** (whether Layer 0 still represents the current bet). So a frozen, obsolete Layer 0 keeps passing validation. The rot is invisible to the gate that exists precisely to catch product-context defects.

**Net:** "the thinnest end-to-end path of looplia _today_" — operator signs up → hires an AI employee → connects a tool → assigns real work → the agent does it → reports back — was never cut as a walking skeleton. It emerged piecemeal across L7 → L9 → L11 → L18 → L23. The user's observation is exact.

---

## What User Story Mapping actually says about multiple journeys

The user asked to re-examine how the methodology itself handles a product with multiple journeys / multiple walking skeletons. The discipline is clear, and it exposes AEP's misuse:

1. **One walking skeleton per map, not per product.** Cockburn's walking skeleton is "a tiny end-to-end implementation that links the main architectural components", after which architecture and functionality evolve in parallel. Patton adopts it as the **first release slice of one map**. So: one map = one journey = one backbone = one walking skeleton.

2. **Multiple journeys → multiple maps, threaded by altitude.** Patton's guidance is that a story map tells the story of one kind of user doing one big thing. When journeys genuinely diverge, you build **separate maps** (one per journey/persona) rather than forcing unrelated journeys onto one backbone — and for large products you use a "backbone of backbones": a coarse top-level backbone whose activities each expand into their own sub-map. This is exactly looplia's `capabilities[].map_path` shape; it had arrived at Patton's hierarchical map already.

3. **Every new journey is introduced skeleton-first.** "A product starts from one walking skeleton and grows new features and journeys" is the normal case. The discipline is: each new journey gets its **own** thinnest end-to-end slice **before** depth — not bolted on as deep features. Looplia's auto-company (a whole new journey) violated this: it was built as depth layers (L7→L23), so even that journey has no walking skeleton of its own.

4. **The map is a conversation artifact — and that is the root cause.** Patton is explicit that the real product of story mapping is _shared understanding_; the map itself is a byproduct, expected to go stale, often thrown away. **AEP repurposes a deliberately-ephemeral facilitation artifact as a durable product-context that drives autonomous dispatch** — a fundamentally different requirement. The methodology never promised a living artifact, so it never supplied the machinery to keep one alive. AEP needs that machinery and never added it. That is why the skeleton rots.

---

## What AEP already designed — and lost in implementation

The fix is mostly **completing an existing design**, not inventing a new one:

- The Multi-Map Capability Structure proposal already specifies that each capability `map.yaml` carries its own `layers:` starting at `layer: 0, name: "Walking Skeleton"` with an `outcome_hypothesis` (`aep-v2-improvement-guideline.md:104-133`, esp. `:116-121`). The glossary's "Capability Map" entry agrees: each map has "its own backbone, **layers**, and story stubs" (`glossary.md:668-678`).
- **But the shipped implementation dropped it.** Looplia's `maps/auto-company/map.yaml` lists `layers: [9, 10.5, 11, 11.5, 18, 18.5, 19, 20]` — **global** layer numbers — with no `layer: 0` and no per-capability walking skeleton. The "skeleton forest" was designed and then collapsed back onto global layers.
- **Two models in AEP's own docs were never reconciled.** The glossary still describes the v1 single-backbone model (the backbone lives in `product.activities`; `glossary.md:22-28,135-141`), while the v2 roadmap describes multi-map per-capability backbones. The walking skeleton fell through the seam between them.
- **The reshape trigger already exists but is inert for the skeleton.** "Opportunity Shift → `/aep-envision` Phase 0" (`glossary.md:499-514`) and Reshape in [release-line-adjustments.md](release-line-adjustments.md) both fire on a bet change, but neither re-baselines the walking skeleton.

So this proposal: (1) make the **per-capability walking skeleton** explicit and load-bearing — the half that was designed but lost; (2) add the **product spine** — the genuinely new piece, the cross-journey thread; (3) tie skeleton **currency to the bet**, so the reshape classification that already exists actually forces a re-cut.

---

## The Model: Skeleton Forest + Product Spine

Two levels, each given a reader so neither can silently rot.

### Level 1 — Capability Walking Skeleton (the forest)

Each `maps/<cap>/map.yaml` declares its own thinnest end-to-end slice. A new capability is introduced **skeleton-first**, then enriched. This is the design from §1.5, made explicit and durable:

```yaml
# maps/<cap>/map.yaml
capability: auto-company
walking_skeleton:
  status: declared # declared | green | stale
  introduced_in_layer: 9 # the layer that first makes this journey walk end-to-end
  thinnest_path: # the minimal activities strung together to walk
    - hire-agents
    - assign-work
    - oversee-company
  verification: >
    Operator hires one agent, assigns one task, the agent completes it with a
    mandatory comment, and the operator sees it done on the company surface.
backbone: # unchanged — the journey's local narrative
  - activity: hire-agents
    narrative: "…"
```

`status: green` means the journey actually walks end-to-end against the real product (an outcome statement, per [Outcome Contracts](../aep-v2-improvement-guideline.md) §4), not merely that its stories merged.

### Level 2 — Product Spine (the cross-journey thread)

`product/index.yaml` gains a curated `spine:` — "the thinnest end-to-end path a real user takes through the product as it stands today", threading across capabilities and **anchored to the bet**. This replaces the append-only semantics of `product.activities`:

```yaml
# product/index.yaml
product:
  spine:
    cut_for_bet: "2026-06-11" # MUST equal opportunity.bet.evolved_at (currency anchor)
    status: current # current | stale
    narrative: >
      Operator signs up → hires an AI employee → connects a company tool →
      assigns real work → the agent does it on their infra → reports back.
    path: # curated cross-capability thinnest path (NOT every activity)
      - { capability: agent-platform, activity: signup }
      - { capability: auto-company, activity: hire-agents }
      - { capability: company-context, activity: connect-tool }
      - { capability: auto-company, activity: assign-work }
      - { capability: auto-company, activity: oversee-company }
    superseded: # history: old spines archived, never deleted
      - cut_for_bet: "2026-04-03"
        narrative: "relay: signup → install daemon → connect → shell"
        archived_at: "2026-06-11"
```

`product.activities` is then reinterpreted as a **build-generated union** of the capability backbones (a derived index), not a hand-maintained list. The human-curated artifact is `spine`.

---

## Forcing functions (give the skeleton a currency reader)

Layers survived because `/aep-dispatch` reads them. The spine and the forest need readers too. **No new commands** (per Decision D1) — every change enhances an existing skill:

| Skill           | New responsibility                                                                                                                                                                                                                                                                                |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/aep-validate` | **Currency checks** (in addition to coverage): (1) `spine.cut_for_bet == opportunity.bet.evolved_at`, else WARN "spine stale vs current bet"; (2) every `capabilities[].status: active` has a `walking_skeleton`, and its enrichment layers were not dispatched before the skeleton went `green`. |
| `/aep-envision` | A **reshape re-baseline** step: on a bet change, push the current spine into `spine.superseded`, cut a new spine, and ensure the new center-of-gravity capability declares a `walking_skeleton`.                                                                                                  |
| `/aep-reflect`  | Split routing explicitly: re-slice (within the bet) vs **reshape** (bet change → triggers the spine re-baseline above). Today [release-line-adjustments.md](release-line-adjustments.md) names this boundary but takes no action on the skeleton.                                                 |
| `/aep-map`      | When introducing a new capability, **require** declaring its `walking_skeleton` before its enrichment layers — skeleton-first enforced at decomposition.                                                                                                                                          |
| `/aep-dispatch` | **Advisory** (not a hard block): when dispatching an enrichment story for capability C whose `walking_skeleton.status != green`, warn "building depth before C walks end-to-end."                                                                                                                 |

---

## Migration: v2 split → Skeleton Forest (additive-first)

Consistent with the existing additive philosophy (§1.7) and "capability maps are optional" (Decision D3). Each phase is independently shippable.

| Phase                      | Action                                                                                                                                                                                         | Breaking?                       |
| -------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------- |
| **P0 — additive schema**   | Add **optional** `spine` (product index) and `walking_skeleton` (capability map) fields to `product-context-schema.yaml`. Existing products keep working.                                      | No                              |
| **P1 — spine**             | Per product, curate `product.spine` (a one-time human pass selecting the thinnest cross-capability path). Point `/aep-validate` at it. Demote `product.activities` to a build-generated union. | Low                             |
| **P2 — skeleton backfill** | For each active capability, declare `walking_skeleton` (point at the layer that made it walk; if it was never cut skeleton-first — auto-company — declare it retroactively and flag the gap).  | Low                             |
| **P3 — forcing functions** | Wire `/aep-validate` currency, `/aep-envision` reshape re-baseline, `/aep-reflect` re-slice/reshape split, `/aep-map` skeleton-first, `/aep-dispatch` advisory.                                | Behavior change — dogfood first |

**Exact change sites (for the follow-up implementation PRs):**

- Schema (single source of truth — edit `_shared/`, then run `build-skills.sh`; never edit `.aep-generated` directly): `skills/product-context/_shared/templates/product-context-schema.yaml` (+ `object-map-schema.yaml` if cross-referenced).
- Skills: `skills/product-context/{envision,reflect,validate,map,dispatch}/SKILL.md`.
- Docs: `docs/glossary.md` (add "Product Spine", "Capability Walking Skeleton"; reconcile the single-backbone vs multi-map entries), `README.md` (two-level story-map diagram), [release-line-adjustments.md](release-line-adjustments.md) (add the "Reshape → re-cut the spine" action), and a registration pointer in `aep-v2-improvement-guideline.md` §1.

**Propagation discipline** (out of scope for any single upstream PR): new enum values (`spine.status`, `walking_skeleton.status`) must propagate to every listing and to the build-generated `_shared/references`; downstream consumers go live only after the upstream tag is cut and each consumer re-pins via the skills CLI.

---

## Worked example: looplia

- **Current state:** the relay spine has been stale since 2026-06-11; auto-company grew as depth (L7→L23) with no skeleton-first slice.
- **After applying the model:**
  - Archive the relay spine into `spine.superseded`.
  - Cut the current spine for the auto-company bet: signup → hire-agent → connect-tool → assign-work → oversee-company.
  - Backfill `walking_skeleton` per capability: remote-relay (`green`, it really was the original Layer 0), auto-company (declared retroactively at L9a, `green`), company-context (L10), agent-email (L4).
- **The proof:** under the new `/aep-validate` currency check, looplia **fails before migration** (relay spine's `cut_for_bet` ≠ the 2026-06-11 bet) and **passes after** — the forcing function catches exactly the rot that went silent in June.

---

## Anti-patterns this prevents

- **Adding a new journey as depth layers instead of a skeleton-first slice.** (looplia auto-company.) New capability → declare its walking skeleton first.
- **Treating Layer 0 as done-forever after the bet moves.** A reshape MUST re-baseline the spine; the old skeleton is archived, not silently kept.
- **Maintaining `product.activities` as an append-only aggregate.** The curated artifact is the spine; the flat list is derived.

---

## References

- Jeff Patton, _User Story Mapping_ — the backbone and "now / next / later" releases (Ch. 5–6), slicing strategies (Ch. 12), discovery and shared understanding (Ch. 14).
- Alistair Cockburn — "Walking Skeleton": a tiny end-to-end implementation linking the main architectural components, evolved in parallel with functionality.
- [release-line-adjustments.md](release-line-adjustments.md) — re-slice vs reshape; this proposal supplies the missing "reshape → re-cut the spine" action.
- [aep-v2-improvement-guideline.md](../aep-v2-improvement-guideline.md) §1 (Multi-Map Capability Structure — the per-capability `layer: 0` this completes), §4 (Outcome Contracts — what `status: green` means), Decisions D1 (no new commands) and D3 (capability maps additive).
- [glossary.md](../glossary.md) — Layer 0 (Walking Skeleton), Activity Backbone, Release Line, User Story Map, Capability Map, Feedback Classification (Opportunity Shift).
- Affected skills: `/aep-envision`, `/aep-map`, `/aep-validate`, `/aep-reflect`, `/aep-dispatch`.
