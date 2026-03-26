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
**Output:** Classified feedback + updated product context artifacts

---

## Before Starting

Read the current product context:

```bash
cat product-context/context-document.md
cat product-context/system-map.md
cat product-context/story-graph.md
cat product-context/feedback-log.md 2>/dev/null
```

---

## Step 1: Gather Feedback

Collect observations from all sources:

- **User testing:** What worked? What confused people? What was missing?
- **Error logs / monitoring:** Runtime failures, performance issues, unexpected behavior
- **Cost data:** If agent execution traces exist, review per-story costs. Which story types were expensive? Where did retries concentrate?
- **Product instincts:** After seeing the thing work, what does the user's gut say? What feels right, what feels off?

Ask the user one source at a time. Don't rush — the quality of classification depends on the quality of input.

---

## Step 2: Classify Each Observation

Every piece of feedback becomes one of:

### Bug
Specified behavior that does not work.
- **Action:** Create a fix story in the current layer, route to `/design`
- **Update:** Add to story graph as a fix story with high priority

### Refinement
Working behavior that needs improvement.
- **Action:** Create a new story in the next layer, add to story graph
- **Update:** Add story to story graph with appropriate layer and dependencies

### Discovery
New requirement or invalidated assumption.
- **Action:** Revisit product context
  - If it's a product assumption → update Context Document via `/envision`
  - If it's an architecture issue → update System Map via `/map`
- **Update:** Mark the affected assumption in the Context Document as revised

### Opportunity Shift
Fundamentally changes the bet — the original opportunity hypothesis is wrong or has shifted.
- **Action:** Back to `/envision` Phase 0
- **This is rare** but critical to recognize. Signs: the problem you're solving isn't the problem users actually have, or a market shift made the opportunity moot.

Present the classification to the user for each observation. Let them override — they know their product better than any framework.

---

## Step 3: Cost Review

If cost data is available (from `.dev-workflow/` trace records or agent execution logs):

- **Which story types are consistently expensive?** Could they benefit from more precise context assembly, simpler decomposition, or a different agent role?
- **Where did retries concentrate?** Patterns in failure suggest either ambiguous specs or incorrect module boundaries.
- **Is the agent topology efficient?** Does the routing policy need adjustment?

Record cost observations and any topology adjustment recommendations.

---

## Step 4: Update Product Context

Based on the classified feedback:

1. **Append to feedback log** (`product-context/feedback-log.md`):
   ```markdown
   ## [Date] — Post-[feature/layer] Reflection

   ### Bugs
   - [description] → fix story added to story graph

   ### Refinements
   - [description] → story added to Layer N

   ### Discoveries
   - [description] → Context Document updated / System Map reviewed

   ### Opportunity Shifts
   - [none / description]

   ### Cost Observations
   - [observations and adjustments]
   ```

2. **Update story graph** with new stories (fixes, refinements)

3. **Update Context Document** if assumptions changed (version the changes)

4. **Update agent topology** if routing adjustments are needed

5. **Commit updates:**
   ```bash
   git add product-context/
   git commit -m "chore: reflect — classify feedback and update product context"
   ```

---

## Step 5: Decide Next Action

Based on the reflection, recommend the next step:

| Feedback type | Next action |
|---|---|
| Only bugs | Fix stories → `/design` → `/build` |
| Refinements | Next layer → `/design` → `/build` |
| Discovery (product) | `/envision` to update assumptions |
| Discovery (architecture) | `/map` to update system map |
| Opportunity shift | `/envision` Phase 0 (re-validate) |
| All clear | Next layer or ship to production |

---

## Key Principles

- **Without structured feedback ingestion, the system is open-loop** — you ship and hope. This phase makes the loop explicit.
- **Feedback classification is the decision** — the category determines where the feedback routes. Get the classification right and the routing follows.
- **Cost data matters from day one** — invisible spending is uncontrollable spending. Track it, review it, act on it.
- **Version the Context Document** — the history of changes is itself valuable. It shows how understanding evolved.

---

## Next Step

Based on the reflection outcome, proceed to one of:

- `/design` — execute new stories (bugs or refinements)
- `/envision` — update product assumptions
- `/map` — update system architecture
- Next layer execution cycle
