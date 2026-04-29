---
name: envision
description: Product-level opportunity and product framing. Use when starting a new product from scratch, revisiting product direction, or when the user says "new product idea", "validate this idea", "product framing", "what should we build", "revisit our assumptions". Walks through opportunity validation and produces a Context Document that feeds into /map and /design.
---

# Envision

Transform a fuzzy product idea into a precise, testable product definition. First validate the opportunity is worth pursuing, then frame the product with enough precision that downstream agents can work without ambiguity.

**Where this fits:**

```
/envision → /map → /scaffold → [ /design → /launch → /build → /wrap ] → /reflect
▲ you are here
```

**Session:** Main, interactive with user
**Input:** Product idea (vague or refined)
**Output:** `product/index.yaml` with `opportunity`, `personas`, `capabilities`, and `product` sections; `product-context.yaml` with `calibration` and `changelog` sections. In v1 fallback mode (no split), writes everything to `product-context.yaml`.

**YAML Schema:** See `templates/product-context-schema.yaml` for the full structure and field definitions.

---

## Before Starting

Check which mode to operate in:

```bash
ls product/index.yaml 2>/dev/null
ls product-context.yaml 2>/dev/null
```

**File Resolution:**

- If `product/index.yaml` exists → **split mode**. Product definition lives in `product/index.yaml`, operational state in `product-context.yaml`.
- If only `product-context.yaml` exists → **v1 mode** OR **migration candidate**. Ask the user: "Do you want to migrate to split mode (product/index.yaml + product-context.yaml) or keep the single file?"
- If neither exists → **new project**. Default to split mode. Create `product/` directory.

In update mode, read the existing file(s) and ask whether they want to revise or start fresh. Preserve all sections you are not updating.

---

## Phase 0: Opportunity Framing

**Goal:** Determine whether this idea is worth building at all, before investing in product design.

**Why this is separate from product framing:** Opportunity Framing answers "should we build this?" Product Framing answers "what exactly should we build?" Conflating them causes premature commitment — you start designing a product before validating the opportunity, and sunk-cost bias prevents you from killing a bad idea.

### How to run this phase

Let the user describe their idea freely. Do not impose structure yet. Your job is to extract the raw material:

- What triggered this idea? A personal pain point, a market gap, a technology capability?
- Who has this problem today? How do they currently solve it?
- What would change in the world if this product existed?
- What is the user's unique advantage in building this — technical skill, domain knowledge, existing audience?
- What are the strongest reasons this might fail or not matter?

After sufficient divergence, synthesize into an **Opportunity Brief** (see `templates/opportunity-brief.md`). The brief is deliberately short — one page. It captures the core bet: "I believe [target user] has [problem], and I can build [solution] because [advantage]."

### Kill Point

Present the Opportunity Brief back to the user. This is an explicit decision point: **proceed or kill.**

If the opportunity does not survive an honest five-minute challenge, it should not consume the resources that subsequent phases require. Killing early is the highest-ROI decision in the entire workflow.

- **Proceed** → Continue to Phase 1
- **Kill** → Stop here. The brief is still saved as a record.
- **Defer** → Save the brief with a revisit condition

### Phase 0 Output

**Split mode:** Write the finalized Opportunity Brief to the `opportunity` section of `product/index.yaml`.
**V1 mode:** Write to the `opportunity` section of `product-context.yaml`.

---

## Phase 1: Product Framing

**Goal:** Transform the validated opportunity into a precise product definition that downstream agents can consume without ambiguity.

**Core premise:** The user carries dozens of implicit assumptions — about users, scope, technical constraints, success criteria. Every assumption left implicit will be resolved by a downstream agent through guesswork. This phase makes every assumption explicit.

### Stage 1: Diverge

Continue the conversation from Phase 0, now focused on product specifics. Lines of inquiry:

- **Problem statement:** Sharpen the problem. Not "developers need better tools" but "solo developers building SaaS on edge platforms lose 4+ hours per project setting up agent sandboxing because existing solutions assume AWS/GCP infrastructure."
- **Persona / JTBD:** Who is the primary user, concretely? What job are they hiring this product to do? What does success look like from their perspective?
- **MVP boundary:** What is the single most important end-to-end journey the user can complete? What is explicitly excluded, even if adjacent and tempting?
- **User activities (story map backbone):** What does the user DO, step by step, in the core journey? Map the user's activities as a left-to-right narrative. Each activity is a verb phrase from the user's perspective: "Authenticate", "Create Profile", "Generate Content", "Track Progress", "Download Output". These form the backbone of the story map — the horizontal axis that layers cut across. The activities should read as a coherent sentence: "The user authenticates, then creates a profile, then generates content, then tracks progress, then downloads the output." This comes BEFORE layer definitions — build the backbone first, then draw release lines across it.
- **Technical constraints:** Non-negotiable stack choices, infrastructure requirements, hard dependencies.
- **Quality dimensions:** Which dimensions of this product require human judgment that agents cannot provide? Not every dimension needs calibration — only those where "correct but not right" is likely. Common dimensions:
  - **Visual design** — brand identity, color, typography, layout (nearly always needed for user-facing products)
  - **UX flow** — user journey, information architecture, page transitions
  - **API surface** — endpoint naming, grouping, error contracts (when external consumers exist)
  - **Data model** — entity naming, field semantics (when domain language matters)
  - **Copy/tone** — brand voice, error messages, empty states
  - **Scope/direction** — mid-build intent correction (common when PM and builder are different people)
  - **Performance/quality** — latency thresholds, retry behavior, caching strategy

  For each declared dimension: what layer is it most likely to first need calibration? Why?

- **Layered MVP contract:** Layer 0 is the walking skeleton — a horizontal slice across the activity backbone, picking the thinnest story from each activity. Each subsequent layer adds capabilities. Later layers may introduce new activities that extend the backbone to the right. Define what the user can accomplish at each layer.

  `.5` layers are **human alignment layers**, not just "UI polish." A `.5` layer is any point where the team pauses agent execution to calibrate human intent across one or more quality dimensions. Layer 0.5 might be visual design only. Layer 1.5 might be visual design extension + copy tone. The `calibration.plan` maps layers to expected calibration checkpoints.

### Stage 2: Structure

Organize everything into the **Context Document** (see `templates/context-document.md`). Present the draft to the user.

Populate `product.quality_dimensions` from the diverge conversation — for each dimension the user identified as needing human calibration, record the dimension, criticality, first calibration layer, and rationale.

Quality standard: **every statement must be convertible into a verification condition.** "The system should be performant" fails. "API p95 latency < 200ms" passes. If a statement cannot be tested, it is not precise enough for agents to act on.

### Stage 3: Stress Test (Independent Agents)

Hand the Context Document to agents that did not participate in the conversation. They review it cold from three angles:

1. **Product viability:** Are the user and problem assumptions validated? What are the strongest counter-arguments?
2. **Technical feasibility:** Are technology choices compatible with each other and with the stated requirements? Known limitations?
3. **Scope control:** Is the MVP actually minimal? Can any layer be cut?

Each reviewer produces a challenge list. The user resolves each item — either by refining the document or marking it as an explicit `open_question` with a default assumption and a revisit trigger.

> Note: The stress test is itself a form of pre-build calibration — independent agents check alignment before building. Post-build calibration (`/calibrate`) extends this to dimensions that only become visible after agents have produced output: visual design, UX flow, naming, tone.

Record the stress test results in `product.stress_test` within the YAML.

### Phase 1 Output

**Split mode:**

1. Write the finalized Context Document to `product/index.yaml`:
   - `opportunity` (from Phase 0)
   - `personas` (extracted from the persona work — use list format with `id`, `description`, `jtbd`)
   - `capabilities` (at least one entry; single-journey products get one capability)
   - `product` subsection: `problem`, `goals`, `non_goals`, `mvp_boundary`, `constraints`, `layers`, `activities`, `failure_model`, `security_model`, `success_criteria`, `quality_dimensions`, `open_questions`, `decisions`, `stress_test`
2. Write operational initialization to `product-context.yaml`:
   - Header: `schema: v1`, `project`, `version`, `updated_at`, `dispatch_epoch: 0`
   - `calibration.plan` (mapped from quality_dimensions)
   - `calibration.history: []`
   - `changelog` entry recording what was created
   - All other operational sections left empty (populated by `/map`)

**V1 mode:** Write everything to `product-context.yaml` using `templates/product-context-schema.yaml` as the structural reference.

On subsequent runs — read the existing file(s), update the relevant sections, and preserve all other sections (e.g., `architecture`, `stories`, `topology`).

If quality dimensions were declared, also write the initial `calibration.plan` section — mapping each dimension to the layer where calibration is expected. This plan is refined by `/map` (which has concrete layer definitions) and executed by `/calibrate`.

#### Capability Maps (for multi-journey products)

If the product has **2+ distinct user journeys**, also create capability map files:

1. Ensure `product/index.yaml` has multiple entries in `capabilities[]`
2. For each capability, create:
   - `product/maps/<capability-id>/frame.yaml` — scope, boundary, primary user, outcome contract
   - Story stubs are populated later by `/map`

Simple single-journey products get one capability entry but skip `frame.yaml` and `map.yaml`.

### Before Committing: Validate YAML

See `references/yaml-guardrails.md` for the full checklist. Run:

```bash
# Split mode
python3 -c "import yaml; [yaml.safe_load(open(f)) for f in ('product/index.yaml', 'product-context.yaml')]; print('YAML OK')"
# V1 mode
python3 -c "import yaml; yaml.safe_load(open('product-context.yaml')); print('YAML OK')"
```

If this fails, fix the YAML before committing. Common fixes: quote list items containing colons, flatten nested sub-lists, escape embedded double quotes.

### Commit

```bash
# Split mode: Write product/index.yaml (opportunity + personas + capabilities + product)
# Split mode: Write product-context.yaml (calibration + changelog, operational sections empty)
# V1 mode: Write product-context.yaml (all sections)
git pull --ff-only origin main
git add product-context.yaml product/ docs/
git commit -m "feat: add product context (opportunity brief + context document)"
git push origin main
```

---

## For Iteration

When revisiting an existing product (triggered by `/reflect` or the user's own initiative):

1. Read the existing product definition (`product/index.yaml` in split mode, `product-context.yaml` in v1 mode)
2. Identify what's changed — new learnings, invalidated assumptions, scope shifts
3. Update the relevant sections (`opportunity` and/or `product`)
4. Re-run the stress test on changed sections only
5. Append to the `changelog` section
6. Commit the updated version (version history is itself valuable)

### Boundary: When to Use `/envision` vs `/reflect`

Not every post-layer adjustment requires envision. Most learning leads to re-slicing (moving stories between layers), which is handled entirely in `/reflect`. For details, see `docs/decisions/release-line-adjustments.md`.

**What does NOT trigger `/envision`** (handle in `/reflect` instead):

- Moving stories between layers (e.g., promoting a Layer 2 story to Layer 1)
- Adding new stories to existing activities
- Re-prioritizing the next layer based on what you learned
- Adjusting release line boundaries without changing the backbone

**What DOES trigger `/envision`:**

- Backbone changes — new activities, removed activities, reordered user journey
- Product framing changes — persona, JTBD, or MVP boundary needs redefinition
- Opportunity hypothesis invalidation — the problem or market shifted
- New activities that extend the backbone to the right

---

## Key Principles

- **One question at a time** — Don't overwhelm with multiple questions
- **YAGNI ruthlessly** — Remove unnecessary features from all designs
- **Explain why, don't stack MUSTs** — Every instruction comes with its rationale
- **Explicit unknowns over implicit assumptions** — Documented unknowns cause agents to stop and ask. Undocumented unknowns cause agents to guess.
- **Kill early and often** — The best outcome from Phase 0 is sometimes "don't build this"

---

## Next Step

Product is envisioned. Proceed to:

```
/map
```

This decomposes the Context Document into a system map, layered story graph, and agent topology.
