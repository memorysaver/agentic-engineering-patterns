---
name: aep-envision
description: >-
  Validates and frames a product opportunity for downstream agents. Use to
  start or revisit an idea, validate product direction, answer "what should we
  build", or create the Context Document for /aep-map and /aep-design.
---

# Envision

Transform a fuzzy product idea into a precise, testable product definition. First validate the opportunity is worth pursuing (Phase 0), then frame the product with enough precision that downstream agents work without ambiguity (Phase 1).

**Where this fits:**

```
/aep-envision → /aep-map → /aep-scaffold → [ /aep-design → /aep-launch → /aep-build → /aep-wrap ] → /aep-reflect
▲ you are here
```

- **Session:** Main, interactive with user
- **Input:** Product idea (vague or refined)
- **Output:** `product/index.yaml` (`opportunity`, `personas`, `capabilities`, `product` sections) + `product-context.yaml` (`calibration`, `changelog`). V1 fallback (no split): everything in `product-context.yaml`.
- **YAML schema:** field structure and definitions live in `templates/product-context-schema.yaml`.

## Before Starting

Run the probe, then apply the matching case. Detection is automatic — never ask the user which mode is active.

```bash
ls product/index.yaml 2>/dev/null && echo "SPLIT MODE" || echo "V1 MODE"
ls product-context.yaml 2>/dev/null
```

Split vs V1 vs new-project semantics (including the one-time V1→split migration offer) are canonical in `references/file-resolution.md` — read it to resolve the case. In update mode, read the existing file(s), ask whether to revise or start fresh, and preserve every section you are not updating.

---

## Phase 0: Opportunity Framing

**Goal:** Decide whether this idea is worth building at all, before investing in product design. Opportunity Framing answers "should we build this?"; Phase 1 answers "what exactly?" — separating them prevents premature commitment and sunk-cost bias against killing a bad idea.

### How to run this phase

Let the user describe the idea freely; do not impose structure yet. Extract the raw material:

- What triggered this idea — a personal pain, a market gap, a technology capability?
- Who has this problem today, and how do they solve it now?
- What changes in the world if this product exists?
- What is the user's unfair advantage — technical skill, domain knowledge, existing audience?
- What are the strongest reasons this might fail or not matter?

After sufficient divergence, synthesize an **Opportunity Brief** (one page, per `templates/opportunity-brief.md`) capturing the core bet: "I believe [target user] has [problem], and I can build [solution] because [advantage]."

### Kill Point

Present the Opportunity Brief back and put three yes/no challenge questions to the user — an honest five-minute challenge. Any **no** stops the phase (Kill or Defer):

1. Is the problem real and painful for a specific, named user?
2. Is there a genuine "why now" and an unfair advantage that is yours?
3. Do the strongest counter-arguments fail to sink it, and is the impact worth the downstream build cost?

Killing early is the highest-ROI decision in the workflow. Record the decision in the brief's Decision section:

- **Proceed** → Phase 1
- **Kill** → stop; the brief is still saved as a record
- **Defer** → save the brief with a revisit condition

### Phase 0 Output

Write the finalized Opportunity Brief to the `opportunity` section — of `product/index.yaml` (split mode) or `product-context.yaml` (v1 mode).

**Postcondition:** the `opportunity` section exists in the mode's target file, and the Kill Point decision (proceed/kill/defer) is recorded.

---

## Phase 1: Product Framing

**Goal:** Turn the validated opportunity into a precise product definition downstream agents consume without ambiguity. Every assumption left implicit gets resolved by a downstream agent through guesswork — this phase makes each one explicit.

### Stage 1: Diverge

Continue the conversation from Phase 0, now on product specifics:

- **Problem statement:** Sharpen it. Not "developers need better tools" but "solo developers on edge platforms lose 4+ hours per project on agent sandboxing because existing solutions assume AWS/GCP."
- **Persona / JTBD:** Who is the primary user, concretely? What job are they hiring this product to do? What does success look like to them?
- **MVP boundary:** The single most important end-to-end journey the user can complete — and what is explicitly excluded, even if adjacent and tempting.
- **User activities (story-map backbone):** What the user DOES, step by step, as a left-to-right narrative of verb phrases ("Authenticate", "Create Profile", "Generate Content", "Track Progress", "Download Output"). Build this backbone BEFORE layer definitions — layers cut across it.
- **Technical constraints:** Non-negotiable stack, infrastructure, hard dependencies.
- **Quality dimensions:** Which dimensions need human judgment agents cannot provide — only those where "correct but not right" is likely. The seven taste dimensions (visual-design, ux-flow, copy-tone, api-surface, data-model, scope-direction, performance-quality) are cataloged in `/aep-calibrate` (`references/calibration-types.md`) — declare from there rather than re-listing. **`object-model` is separate:** a structural gate (not a taste calibration) that `/aep-map` auto-drafts and `/aep-model` approves. **Declare `object-model` by default for any UI-facing product/capability** — it stops build agents from inventing one-step-one-screen wizard UIs (skip only for pure-backend/CLI). For each declared dimension, record the layer where calibration is first likely and why.
- **Layered MVP contract:** Layer 0 is the walking skeleton — the thinnest story from each activity, sliced horizontally across the backbone. Each later layer adds capabilities (and may extend the backbone rightward with new activities). `.5` layers are **human alignment layers**: points where agent execution pauses to calibrate intent across one or more quality dimensions (canonical definition: `/aep-map` `references/alignment-layers.md`). `calibration.plan` maps layers to expected checkpoints.

### Stage 2: Structure

Organize everything into the **Context Document** (`templates/context-document.md`) and present the draft. Populate `product.quality_dimensions` — for each declared dimension record dimension, criticality, first calibration layer, and rationale.

Quality standard: **every statement must be convertible into a verification condition.** "The system should be performant" fails; "API p95 latency < 200ms" passes. Untestable statements are not precise enough for agents to act on.

### Stage 3: Stress Test (independent agents)

Hand the Context Document to agents that did not join the conversation. They review it cold from three angles, each producing a challenge list:

1. **Product viability** — are user/problem assumptions validated? Strongest counter-arguments?
2. **Technical feasibility** — are technology choices mutually compatible and adequate? Known limits?
3. **Scope control** — is the MVP actually minimal? Can any layer be cut?

The user resolves each item — by refining the document, or by marking it an explicit `open_question` with a default assumption and a revisit trigger. Record results in `product.stress_test`. (This stress test is pre-build calibration; `/aep-calibrate` extends it to dimensions only visible after agents produce output.)

### Phase 1 Output

**Split mode:**

1. Write the Context Document to `product/index.yaml`:
   - `opportunity` (from Phase 0)
   - `personas` (list with `id`, `description`, `jtbd`)
   - `capabilities` (≥1 entry; single-journey products get one)
   - `product`: `problem`, `goals`, `non_goals`, `mvp_boundary`, `constraints`, `layers`, `activities`, `failure_model`, `security_model`, `success_criteria`, `quality_dimensions`, `open_questions`, `decisions`, `stress_test`
2. Write operational init to `product-context.yaml`: header (`schema: v1`, `project`, `version`, `updated_at`, `dispatch_epoch: 0`), `calibration.plan` (mapped from quality_dimensions), `calibration.history: []`, a `changelog` entry; leave other operational sections empty (populated by `/aep-map`).

**V1 mode:** write everything to `product-context.yaml` using `templates/product-context-schema.yaml` as the structural reference. If quality dimensions were declared, also write `calibration.plan`. On subsequent runs, update the relevant sections and preserve all others (e.g. `architecture`, `stories`, `topology`).

**Postcondition:** the `product` sections above are written to the mode's target file, and (split mode) `calibration.plan` + a `changelog` entry exist in `product-context.yaml`.

#### Capability Maps (for multi-journey products)

If the product has **2+ distinct user journeys**: ensure `capabilities[]` has multiple entries and, for each capability, create `product/maps/<capability-id>/frame.yaml` (scope, boundary, primary user, outcome contract) per the schema in `templates/product-context-schema.yaml`. Story stubs are populated later by `/aep-map`. Single-journey products keep one capability entry and skip `frame.yaml`.

### Before Committing: Validate YAML

Run the gate — see `references/yaml-guardrails.md` for the full checklist and common fixes:

```bash
# Split mode
npx js-yaml product/index.yaml > /dev/null && npx js-yaml product-context.yaml > /dev/null && echo "YAML OK"
# V1 mode
npx js-yaml product-context.yaml > /dev/null && echo "YAML OK"
```

**Postcondition:** the command prints `YAML OK` (exit 0). If it fails, fix the YAML before committing.

### Commit

Resolve `$BASE` per `/aep-git-ref` "Integration Branch", then commit to it per `/aep-git-ref` "Control-Plane Commits":

```bash
git add product-context.yaml product/ docs/
git commit -m "feat: add product context (opportunity brief + context document)"
```

**Postcondition:** the commit is pushed to `$BASE`.

---

## For Iteration

When revisiting an existing product (from `/aep-reflect` or the user's own initiative):

1. Read the existing definition (`product/index.yaml`, or `product-context.yaml` in v1 mode)
2. Identify what changed — new learnings, invalidated assumptions, scope shifts
3. Update the relevant sections (`opportunity` and/or `product`)
4. Re-run the stress test on changed sections only
5. Append to `changelog`
6. Commit the updated version (version history is itself valuable)

**Boundary — `/aep-envision` vs `/aep-reflect`:** `/aep-envision` frames a NEW or shifted opportunity — trigger it for backbone changes (added/removed/reordered activities) or product-framing changes (persona, JTBD, MVP boundary, invalidated opportunity hypothesis). `/aep-reflect` classifies feedback on SHIPPED work — it handles re-slicing (moving stories between layers, adding stories, re-prioritizing) without touching envision. Details: `docs/decisions/release-line-adjustments.md`.

---

## Next Step

Product is envisioned. Proceed to `/aep-map` — it decomposes the Context Document into a system map, layered story graph, and agent topology.
