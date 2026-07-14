---
name: aep-reflect
description: Classify post-ship feedback and route it back to the right phase — story graph, architecture, or opportunity hypothesis. Use after /aep-wrap or user testing, or on "reflect", "what did we learn", "update the product context", "replan".
---

# Reflect

Close the feedback loop. Transform real-world observations into actionable changes to the product context — the story graph, the architecture, or even the opportunity hypothesis.

**Where this fits:**

```
/aep-envision → /aep-map → /aep-scaffold → [ /aep-design → /aep-launch → /aep-build → /aep-wrap ] → /aep-reflect
                                                                          ▲ you are here
```

**Session:** Main, interactive with user
**Input:** Observations after shipping (user testing, error logs, cost data, product instincts)
**Output:** Classified feedback + updated `product-context.yaml`; if product intent changed, also updated `product/index.yaml` (split mode)

---

## Before Starting

Mode semantics (split vs v1 vs new-project) are canonical in `references/file-resolution.md` — run the probe there. Reflect reads the product definition from `product/index.yaml` (split) or `product-context.yaml` (v1) for what was intended, and reads operational state (`cost`, `stories`, `changelog`) from `product-context.yaml` for what happened. It writes operational changes to `product-context.yaml`; product-intent changes go to `product/index.yaml` (split) or `product-context.yaml` (v1). If neither file exists there is nothing to reflect on — run `/aep-envision` then `/aep-map` first.

---

## Step 1: Gather Feedback

Collect observations from all sources:

- **User testing:** What worked? What confused people? What was missing?
- **Error logs / monitoring:** Runtime failures, performance issues, unexpected behavior
- **Cost data:** Review the `cost` section of `product-context.yaml`. If agent execution traces exist, review per-story costs. Which story types were expensive? Where did retries concentrate?
- **Product instincts:** After seeing the thing work, what does the user's gut say? What feels right, what feels off?
- **Dogfood reports:** Read `.dev-workflow/dogfood-*.md`. Normalize each `##` finding via the `dogfood_report` adapter (`references/telemetry-ingestion.md` → Dogfood-report adapter) into the same observation record Step 2 classifies — the same source `/aep-watch` ingests headlessly.
- **Lessons learned:** Read `lessons-learned/*.md` (observations captured by workspace agents during builds); summarize recurring errors, solutions that worked, missing documentation.
- **Layer distillations:** Read `lessons-learned/distillations/*.yaml` — the proposal-only synthesis written by `/aep-wrap` Reflect and Advance when a layer completes. Normalize each item via the `distillation` adapter (`references/telemetry-ingestion.md` → Distillation adapter): `refinements` → refinement (with `target_layer`), `skill_amendments` → process (proposed, never auto-applied), `weak_areas` → discovery/refinement.

**Automated ingestion (optional):** Error logs, analytics, and monitoring can be pulled in directly per `references/telemetry-ingestion.md`, normalized into the same observation format Step 2 classifies. Configure endpoints under `topology.routing.telemetry_sources`. Automation augments the interactive sources; ingested records merge with human input before classification, and the human still reviews each classification.

**Postcondition:** every source above is reviewed and its findings normalized into observation records for Step 2 (or the source is explicitly empty this cycle).

---

## Step 2: Classify Each Observation

Every piece of feedback becomes exactly one of the five categories below. Present each classification to the user and let them override — they know their product better than any framework. This section is the canonical classifier; `/aep-watch` applies it headlessly.

### Bug

Specified behavior that does not work.

- **Action:** Add a new story directly to the `stories` section of `product-context.yaml` with `priority: high` and `status: pending` in the current layer; route to `/aep-dispatch`.

### Refinement

Working behavior that needs improvement — or existing stories that need to move between layers.

- **Action:** Add a new story to the `stories` section in the next layer with `status: pending` (include layer assignment and dependencies). Alternatively, promote an existing story from a later layer to an earlier one if learning shows it is needed sooner.

**Sub-type — Calibration:** a gap between "works correctly" and "feels right." The code works as specified and the spec was correct, but the result doesn't match what the human wanted. Confirm: does the code work as specified (yes → not a bug)? Was the spec correct (yes → not a discovery)? Does the result feel right (no → calibration)? Which dimension is off (visual-design / ux-flow / api-surface / data-model / scope-direction / copy-tone / performance-quality)?

- **Heavy** dimensions (visual-design, ux-flow, copy-tone): create stories in the next `.5` alignment layer with `calibration_type: <dimension>`; run `/aep-calibrate <dimension>` before dispatching.
- **Light** dimensions (api-surface, data-model, scope-direction, performance-quality): route to `/aep-calibrate <dimension>` directly — it may create stories in the next integer layer or update product context inline. No `.5` layer needed.

### Discovery

New requirement or invalidated assumption.

- **Action:** Revisit product context. A product assumption → update the `product` section via `/aep-envision` (mark the affected assumption as revised). An architecture issue → update the `architecture` section via `/aep-map`.

### Opportunity Shift

Fundamentally changes the bet — the original opportunity hypothesis is wrong or has shifted.

- **Action:** Back to `/aep-envision` Phase 0. This is rare but critical to recognize. Signs: the problem you're solving isn't the problem users actually have, or a market shift made the opportunity moot.

### Process

Observations about the workflow itself, not the product (permission stalls, signal staleness, missing tooling, agent-config gaps).

- **Action:** Document the pattern in `lessons-learned/process/<observation>.md` (what happened — description, frequency, impact; root cause if known; proposed mitigation). Add a `process_learnings` entry (`pattern`, `mitigation`, `discovered_at`) to `topology.routing` in `product-context.yaml`.
- **Skill amendments:** If the pattern warrants a skill-file change, record it as a proposed amendment in the `changelog` — `type: process-improvement`, `summary`, and `proposed_changes` (`skill`, `change`, `rationale`). Skill files are reviewed and applied by a human; the classifier proposes amendments, never edits skill files.
- **Upstream routing:** For systematic capture of process and tech-stack observations across multiple downstream runs, use `/aep-workflow-feedback` (standardized format + downstream→AEP routing).

**Postcondition:** every observation carries exactly one user-confirmed category and its routing action is recorded — a story appended to `stories`, a `product`/`architecture` update queued, or a `process_learnings`/amendment entry written.

---

## Step 2.5: Re-slice the Map

After classifying all feedback, review the current layer assignments. Release lines are pencil marks — they shift based on what you learned. This is normal iteration.

For each layer not yet built: review story priorities against the classified feedback; promote stories from later layers when learning shows they're needed sooner and demote ones that turned out less critical; add new stories from classified Refinements to the appropriate layer; update the `layer` assignments in the `stories` section of `product-context.yaml`.

**Key rule:** Re-slicing does NOT require going back to `/aep-envision` — you route there only when the backbone (user activities) or product framing changes, not when layer assignments shift. See `docs/decisions/release-line-adjustments.md` for the full framework.

**Postcondition:** `layer` assignments in `stories` reflect the classified feedback (or are unchanged because nothing moved).

---

## Step 2.75: Evaluate Outcome Contracts

If the completed layer has an `outcome_contract` defined in `product.layers[]`:

1. **Present the hypothesis** to the user: "The hypothesis was: [hypothesis]. The success metric was: [type] [target]."
2. **Ask for evaluation** — outcome contracts are not automated tests; they may require user testing, analytics review, or qualitative assessment.
3. **Apply the decision rule:** `keep_if` condition met → record as passed, advance to the next layer; `otherwise` triggered → record as failed, recommend re-slicing (promote stories from later layers, adjust backbone if needed).
4. **Record the result** in the `changelog` — `type: outcome_evaluation`, `summary` noting passed/failed with the metric, actual value, and target.

**Auto-evaluation (optional, opt-in):** the pause can be skipped when the metric is telemetry-bound. See `references/telemetry-ingestion.md` for the `topology.routing.auto_outcome_eval` modes: `quantitative` runs the `coverage_check` gate (§1.5) and, if the metric is bound to a source, fetches the actual value and applies `keep_if`/`otherwise` mechanically; an unbound metric or a failed fetch falls back to the human pause. Qualitative metrics still pause — unless `topology.routing.full_auto: true`, where the agent evaluates by its own judgment. Default (`auto_outcome_eval: none`, `full_auto: false`) preserves the human-in-the-loop pause exactly.

If no outcome contract exists for the completed layer, skip this step.

**Postcondition:** the outcome result is recorded in the `changelog` as passed or failed, or the step was skipped because `product.layers[]` had no `outcome_contract`.

---

## Step 3: Cost Review

Review the `cost` section of `product-context.yaml` along with any execution traces from `.dev-workflow/`:

- **Which story types are consistently expensive?** Could they benefit from more precise context assembly, simpler decomposition, or a different agent role?
- **Where did retries concentrate?** Patterns in failure suggest either ambiguous specs or incorrect module boundaries.
- **Is the agent topology efficient?** Does the routing policy need adjustment?

**Postcondition:** cost observations and any topology-adjustment recommendations are captured for Step 4 (as `cost_observations` in the reflection `changelog` entry and/or `topology` edits) — or recorded as an explicit "none this cycle."

---

## Step 4: Update Product Context

Based on the classified feedback, update the appropriate file: operational changes (new stories, architecture amendments, topology, cost, changelog) → `product-context.yaml`; product-intent changes (opportunity shift, persona change, goals, mvp_boundary, layers, activities) → `product/index.yaml` (split mode) or `product-context.yaml` (v1 mode).

1. **Append a `changelog` entry** per the product-context schema — `date`, `type: reflection`, `summary`, and a `feedback` map holding `bugs` (`description`, `story_id`), `refinements` (`description`, `story_id`, `target_layer`), `discoveries` (`description`, `affected_section`), and `opportunity_shifts` (`description`), plus `cost_observations`.
2. **Update the `stories` section** with new stories (bug fixes get `priority: high`, refinements go to the next layer).
3. **Update the `product` section** if assumptions changed (version the changes).
4. **Update the `topology` section** if routing adjustments are needed.
5. **Validate YAML** (see `references/yaml-guardrails.md` for the full checklist and common fixes):

   ```bash
   npx js-yaml product-context.yaml > /dev/null && echo "YAML OK"
   ```

   Fix any error before committing.

6. **Commit** per `/aep-git-ref` "Control-Plane Commits" (resolve `$BASE` per `/aep-git-ref` "Resolving `$BASE`"):

   ```bash
   git pull --ff-only origin "$BASE"
   git add product-context.yaml product/
   git commit -m "chore: reflect — classify feedback and update product context"
   git push origin "$BASE"
   ```

**Postcondition:** `npx js-yaml` printed `YAML OK` and `git push` exited 0.

---

## Step 5: Decide Next Action

Based on the reflection, recommend the next step:

| Feedback type            | Next action                                                                       |
| ------------------------ | --------------------------------------------------------------------------------- |
| Only bugs                | Fix stories added to YAML → `/aep-dispatch` → `/aep-design` → `/aep-build`        |
| Refinements              | Next layer stories added to YAML → `/aep-dispatch` → `/aep-design` → `/aep-build` |
| Discovery (product)      | `/aep-envision` to update assumptions                                             |
| Discovery (architecture) | `/aep-map` to update system map                                                   |
| Opportunity shift        | `/aep-envision` Phase 0 (re-validate)                                             |
| Calibration (heavy)      | `.5` alignment stories created → `/aep-calibrate <dimension>` → `/aep-dispatch`   |
| Calibration (light)      | `/aep-calibrate <dimension>` (inline) → stories may update → `/aep-dispatch`      |
| All clear                | Next layer or ship to production                                                  |

**Postcondition:** a next action from the table is recommended to the user.
