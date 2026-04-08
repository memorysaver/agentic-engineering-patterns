---
name: reflect
description: Classify feedback and update product context after shipping features. Use after /wrap, after user testing, or when the user says "reflect", "what did we learn", "update the product context", "classify feedback", "replan". Closes the feedback loop by routing observations back to the right phase.
---

# Reflect

Close the feedback loop. Transform real-world observations into actionable changes to the product context — the story graph, the architecture, or even the opportunity hypothesis.

**Where this fits:**

```
/envision → /map → /scaffold → [ /design → /launch → /build → /wrap ] → /reflect
                                                                          ▲ you are here
```

**Session:** Main, interactive with user
**Input:** Observations after shipping (user testing, error logs, cost data, product instincts)
**Output:** Classified feedback + updated `product-context.yaml`

---

## Before Starting

Read the current product context:

```bash
cat product-context.yaml
```

If `product-context.yaml` does not exist, there is nothing to reflect on — run `/envision` and `/map` first.

---

## Step 1: Gather Feedback

Collect observations from all sources. Read from `product-context.yaml` to ground the conversation — review the `product` section for what was intended and the `cost` section for execution data.

- **User testing:** What worked? What confused people? What was missing?
- **Error logs / monitoring:** Runtime failures, performance issues, unexpected behavior
- **Cost data:** Review the `cost` section of `product-context.yaml`. If agent execution traces exist, review per-story costs. Which story types were expensive? Where did retries concentrate?
- **Product instincts:** After seeing the thing work, what does the user's gut say? What feels right, what feels off?
- **Lessons learned:** Read `lessons-learned/*.md` for observations captured by workspace agents during builds. Summarize patterns across recent lessons — recurring errors, solutions that worked, missing documentation.

Ask the user one source at a time. Don't rush — the quality of classification depends on the quality of input.

---

## Step 2: Classify Each Observation

Every piece of feedback becomes one of:

### Bug

Specified behavior that does not work.

- **Action:** Create a new story in `product-context.yaml` with `priority: high` and `status: pending` in the current layer, route to `/dispatch`
- **Update:** Add the story directly to the `stories` section of the YAML

### Refinement

Working behavior that needs improvement — or existing stories that need to move between layers.

- **Action:** Create a new story in the next layer with `status: pending`, add to the `stories` section of `product-context.yaml`. Alternatively, promote an existing story from a later layer to an earlier one if learning shows it's needed sooner.
- **Update:** Include appropriate layer assignment and dependencies

**Sub-type — Calibration:** A gap between "works correctly" and "feels right" in any quality dimension. The code works as specified and the spec was correct, but the result doesn't match what the human actually wanted.

Classification questions for calibration observations:

1. Does the code work as specified? (Yes → not a bug)
2. Was the spec correct? (Yes → not a discovery)
3. Does the result feel right? (No → calibration need)
4. What dimension feels off? (visual-design / ux-flow / api-surface / data-model / scope-direction / copy-tone / performance-quality)

For **heavy** dimensions (visual-design, ux-flow, copy-tone): create stories in the next `.5` alignment layer with `calibration_type: <dimension>`. Run `/calibrate <dimension>` before dispatching.

For **light** dimensions (api-surface, data-model, scope-direction, performance-quality): route to `/calibrate <dimension>` directly — may create stories in next integer layer or update product context inline. No `.5` layer needed.

### Discovery

New requirement or invalidated assumption.

- **Action:** Revisit product context
  - If it's a product assumption → update `product` section via `/envision`
  - If it's an architecture issue → update `architecture` section via `/map`
- **Update:** Mark the affected assumption in the `product` section as revised

### Opportunity Shift

Fundamentally changes the bet — the original opportunity hypothesis is wrong or has shifted.

- **Action:** Back to `/envision` Phase 0
- **This is rare** but critical to recognize. Signs: the problem you're solving isn't the problem users actually have, or a market shift made the opportunity moot.

### Process

Observations about the workflow itself, not the product. Examples: permission stalls, signal staleness, missing tooling, agent configuration gaps.

- **Action:** Document the pattern in `lessons-learned/process/<observation>.md`. Add a `process_learnings` entry to the `topology.routing` section of `product-context.yaml`.
- **Important:** If the pattern warrants a skill file change, record it as a proposed amendment in the changelog — **do not auto-edit skill files**. Skill changes are reviewed and applied by a human.
- **For systematic capture and upstream routing** of process and tech-stack observations — especially when reviewing multiple downstream project runs — use `/workflow-feedback` which provides a standardized format and downstream→AEP routing.

Present the classification to the user for each observation. Let them override — they know their product better than any framework.

---

## Step 2.5: Re-slice the Map

After classifying all feedback, review the current layer assignments. Release lines are pencil marks — they should shift based on what you learned. This is normal iteration, not a sign that something went wrong.

For each layer that has not yet been built:

1. **Review story priorities** in light of classified feedback. Do any stories need to move to an earlier layer? Are any stories in the next layer no longer relevant?
2. **Promote stories** from later layers to earlier ones when learning shows they're needed sooner. Demote stories that turned out to be less critical.
3. **Add new stories** from classified Refinements to the appropriate layer and activity.
4. **Update `product-context.yaml`** — change `layer` assignments in the `stories` section.

**Key rule:** Re-slicing does NOT require going back to `/envision`. You only route there when the backbone (user activities) or product framing changes — not when layer assignments shift. See `docs/decisions/release-line-adjustments.md` for the full decision framework.

---

## Step 2.75: Evaluate Outcome Contracts

If the completed layer has an `outcome_contract` defined in `product.layers[]`:

1. **Present the hypothesis** to the user: "The hypothesis was: [hypothesis]. The success metric was: [type] [target]."
2. **Ask for evaluation** — outcome contracts are not automated tests. They may require user testing, analytics review, or qualitative assessment.
3. **Apply the decision rule:**
   - If `keep_if` condition met → record as passed, advance to next layer
   - If `otherwise` triggered → record as failed, recommend re-slicing: promote stories from later layers, adjust backbone if needed
4. **Record the result** in the changelog:
   ```yaml
   - date: YYYY-MM-DD
     type: outcome_evaluation
     summary: "Layer N outcome contract: [passed/failed] — [metric] was [actual] vs target [target]"
   ```

If no outcome contract exists for the completed layer, skip this step.

---

## Step 3: Cost Review

Review the `cost` section of `product-context.yaml` along with any execution traces from `.dev-workflow/`:

- **Which story types are consistently expensive?** Could they benefit from more precise context assembly, simpler decomposition, or a different agent role?
- **Where did retries concentrate?** Patterns in failure suggest either ambiguous specs or incorrect module boundaries.
- **Is the agent topology efficient?** Does the routing policy need adjustment?

Record cost observations and any topology adjustment recommendations.

---

## Step 4: Update Product Context

Based on the classified feedback, update `product-context.yaml` directly:

1. **Append to the `changelog` section** with a full feedback classification entry:

   ```yaml
   - date: YYYY-MM-DD
     type: reflection
     summary: "Post-[feature/layer] reflection"
     feedback:
       bugs:
         - description: "..."
           story_id: "fix-xxx"
       refinements:
         - description: "..."
           story_id: "ref-xxx"
           target_layer: N
       discoveries:
         - description: "..."
           affected_section: "product|architecture"
       opportunity_shifts:
         - description: "..."
     cost_observations: "..."
   ```

2. **Update `stories` section** with new stories (bug fixes get `priority: high`, refinements go to next layer)

3. **Update `product` section** if assumptions changed (version the changes)

4. **Update `topology` section** if routing adjustments are needed

5. **Validate YAML** (see `references/yaml-guardrails.md`):

   ```bash
   npx js-yaml product-context.yaml > /dev/null && echo "YAML OK"
   ```

   If this fails, fix the YAML before committing. Common fixes: quote list items containing colons, flatten nested sub-lists, escape embedded double quotes.

6. **Commit updates:**
   ```bash
   jj describe -m "chore: reflect — classify feedback and update product context"
   jj new
   jj git push --change @-
   ```

---

## Step 5: Decide Next Action

Based on the reflection, recommend the next step:

| Feedback type            | Next action                                                             |
| ------------------------ | ----------------------------------------------------------------------- |
| Only bugs                | Fix stories added to YAML → `/dispatch` → `/design` → `/build`          |
| Refinements              | Next layer stories added to YAML → `/dispatch` → `/design` → `/build`   |
| Discovery (product)      | `/envision` to update assumptions                                       |
| Discovery (architecture) | `/map` to update system map                                             |
| Opportunity shift        | `/envision` Phase 0 (re-validate)                                       |
| Calibration (heavy)      | `.5` alignment stories created → `/calibrate <dimension>` → `/dispatch` |
| Calibration (light)      | `/calibrate <dimension>` (inline) → stories may update → `/dispatch`    |
| All clear                | Next layer or ship to production                                        |

---

## Step 5.5: Workflow Improvement

Review any observations classified as **Process** in Step 2. For each:

1. **Document the pattern** in `lessons-learned/process/<observation>.md` with:
   - What happened (description, frequency, impact)
   - Root cause (if known)
   - Proposed mitigation

2. **Update product context** — add a `process_learnings` entry to `topology.routing` in `product-context.yaml`:

   ```yaml
   process_learnings:
     - pattern: "<description>"
       mitigation: "<what to do differently>"
       discovered_at: "<date>"
   ```

3. **Propose skill amendments** — if the pattern warrants changes to skill files (e.g., adding a guardrail, changing a phase step), record the proposed amendment in the `changelog` section:

   ```yaml
   - date: YYYY-MM-DD
     type: process-improvement
     summary: "Proposed skill amendment: <description>"
     proposed_changes:
       - skill: "<skill name>"
         change: "<what to add/modify>"
         rationale: "<why>"
   ```

   **Do not auto-edit skill files.** Skill changes are sensitive — the human reviews and applies proposed amendments.

---

## Key Principles

- **Without structured feedback ingestion, the system is open-loop** — you ship and hope. This phase makes the loop explicit.
- **Feedback classification is the decision** — the category determines where the feedback routes. Get the classification right and the routing follows.
- **Cost data matters from day one** — invisible spending is uncontrollable spending. Track it, review it, act on it.
- **Version the product context** — the history of changes is itself valuable. It shows how understanding evolved.

---

## Next Step

Based on the reflection outcome, proceed to one of:

- `/dispatch` — pick and execute new stories (bugs or refinements enter the dispatch queue)
- `/envision` — update product assumptions
- `/map` — update system architecture
- `/calibrate` — recalibrate a specific quality dimension (visual design, UX flow, API surface, etc.)
- Next layer execution cycle
