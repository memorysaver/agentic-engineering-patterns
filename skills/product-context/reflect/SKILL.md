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

Ask the user one source at a time. Don't rush — the quality of classification depends on the quality of input.

---

## Step 2: Classify Each Observation

Every piece of feedback becomes one of:

### Bug
Specified behavior that does not work.
- **Action:** Create a new story in `product-context.yaml` with `priority: high` and `status: pending` in the current layer, route to `/dispatch`
- **Update:** Add the story directly to the `stories` section of the YAML

### Refinement
Working behavior that needs improvement.
- **Action:** Create a new story in the next layer with `status: pending`, add to the `stories` section of `product-context.yaml`
- **Update:** Include appropriate layer assignment and dependencies

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

Present the classification to the user for each observation. Let them override — they know their product better than any framework.

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

5. **Commit updates:**
   ```bash
   jj describe -m "chore: reflect — classify feedback and update product context"
   jj new
   jj git push --change @-
   ```

---

## Step 5: Decide Next Action

Based on the reflection, recommend the next step:

| Feedback type | Next action |
|---|---|
| Only bugs | Fix stories added to YAML → `/dispatch` → `/design` → `/build` |
| Refinements | Next layer stories added to YAML → `/dispatch` → `/design` → `/build` |
| Discovery (product) | `/envision` to update assumptions |
| Discovery (architecture) | `/map` to update system map |
| Opportunity shift | `/envision` Phase 0 (re-validate) |
| All clear | Next layer or ship to production |

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
- Next layer execution cycle
