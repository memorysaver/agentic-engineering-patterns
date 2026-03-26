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
**Output:** `product-context.yaml` with `opportunity` and `product` sections populated

**YAML Schema:** See `templates/product-context-schema.yaml` for the full structure and field definitions.

---

## Before Starting

Check if product context already exists:

```bash
ls product-context.yaml 2>/dev/null
```

- **No `product-context.yaml` exists:** Start from Phase 0. You will create the file at the end.
- **`product-context.yaml` exists:** The user is likely iterating. Read the existing file and ask whether they want to revise the existing product context (update mode) or start fresh. In update mode, read the existing YAML first, then focus on what's changed — preserve all sections you are not updating.

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

Write the finalized Opportunity Brief to the `opportunity` section of `product-context.yaml`.

---

## Phase 1: Product Framing

**Goal:** Transform the validated opportunity into a precise product definition that downstream agents can consume without ambiguity.

**Core premise:** The user carries dozens of implicit assumptions — about users, scope, technical constraints, success criteria. Every assumption left implicit will be resolved by a downstream agent through guesswork. This phase makes every assumption explicit.

### Stage 1: Diverge

Continue the conversation from Phase 0, now focused on product specifics. Lines of inquiry:

- **Problem statement:** Sharpen the problem. Not "developers need better tools" but "solo developers building SaaS on edge platforms lose 4+ hours per project setting up agent sandboxing because existing solutions assume AWS/GCP infrastructure."
- **Persona / JTBD:** Who is the primary user, concretely? What job are they hiring this product to do? What does success look like from their perspective?
- **MVP boundary:** What is the single most important end-to-end journey the user can complete? What is explicitly excluded, even if adjacent and tempting?
- **Technical constraints:** Non-negotiable stack choices, infrastructure requirements, hard dependencies.
- **Layered MVP contract:** Layer 0 is the walking skeleton — the thinnest possible end-to-end path. Each subsequent layer adds capabilities. Define what the user can accomplish at each layer.

### Stage 2: Structure

Organize everything into the **Context Document** (see `templates/context-document.md`). Present the draft to the user.

Quality standard: **every statement must be convertible into a verification condition.** "The system should be performant" fails. "API p95 latency < 200ms" passes. If a statement cannot be tested, it is not precise enough for agents to act on.

### Stage 3: Stress Test (Independent Agents)

Hand the Context Document to agents that did not participate in the conversation. They review it cold from three angles:

1. **Product viability:** Are the user and problem assumptions validated? What are the strongest counter-arguments?
2. **Technical feasibility:** Are technology choices compatible with each other and with the stated requirements? Known limitations?
3. **Scope control:** Is the MVP actually minimal? Can any layer be cut?

Each reviewer produces a challenge list. The user resolves each item — either by refining the document or marking it as an explicit `open_question` with a default assumption and a revisit trigger.

Record the stress test results in `product.stress_test` within the YAML.

### Phase 1 Output

Write the finalized Context Document to the `product` section of `product-context.yaml`. Always append an entry to the `changelog` section recording what was created or updated.

On first run — create `product-context.yaml` with `opportunity` + `product` sections, using `templates/product-context-schema.yaml` as the structural reference.

On subsequent runs — read the existing YAML, update the `opportunity` and/or `product` sections, and preserve all other sections (e.g., `architecture`, `stories`, `topology`).

Commit the artifact:

```bash
# Write product-context.yaml (opportunity + product sections)
jj describe -m "feat: add product context (opportunity brief + context document)"
jj new
jj git push --change @-
```

---

## For Iteration

When revisiting an existing product (triggered by `/reflect` or the user's own initiative):

1. Read the existing `product-context.yaml`
2. Identify what's changed — new learnings, invalidated assumptions, scope shifts
3. Update the relevant sections (`opportunity` and/or `product`)
4. Re-run the stress test on changed sections only
5. Append to the `changelog` section
6. Commit the updated version (version history is itself valuable)

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
