# Dispatch Scoring

Consulted by `/aep-dispatch` Step 3. Compute two scores per ready story and write both into the story YAML:

- **`readiness_score`** — spec completeness, drives Step 7 routing.
- **`dispatch_score`** — priority ranking of the ready queue.

Determine the active layer, run the layer-gate check, and filter conflicts (all in Step 3) _before_ scoring; this file covers only the formulas.

---

## Readiness Score

```
readiness_score = (
  min(3, acceptance_criteria_count)      # 0-3
  + (interface_obligations_defined ? 2 : 0)  # 0 or 2
  + (files_affected_identified ? 1 : 0)      # 0 or 1
  + (verification_defined ? 2 : 0)           # 0 or 2
  + (no_relevant_open_questions ? 2 : 0)     # 0 or 2
) / 10
```

Write `readiness_score` to the story in YAML. It is used for routing in Step 7.

---

## Dispatch Score

```
dispatch_score = (business_value + unblock_potential + critical_path_urgency + reuse_leverage) / (complexity_cost + ambiguity_penalty + interface_risk)
```

### Business Value (1-10)

Use `story.business_value` if explicitly set. Otherwise derive from priority:

```
critical = 10
high     = 7
medium   = 4
low      = 1
```

### Unblock Potential (0-10)

```
unblock_potential = min(10, count of stories that directly depend on this one * 2)
```

A story that unblocks 5 others scores 10. A leaf story scores 0.

### Critical Path Urgency (0-10)

Compute the critical path through the dependency DAG (longest chain from any root to any leaf within the active layer). Stories on the critical path get maximum urgency:

```
If story is on critical path:
  critical_path_urgency = 10
Else:
  slack = latest_possible_start - earliest_possible_start
  critical_path_urgency = max(0, 10 - slack)
```

### Reuse Leverage (0-10)

Stories that produce shared enablers (auth middleware, base components, shared utilities) score higher:

```
reuse_leverage = min(10, count_of_modules_depending_on_output * 3)
```

Only applies to stories with `compile_mode: shared_enabler` or whose module appears in 2+ other modules' `depends_on`.

### Complexity Cost (denominator term)

```
S = 1    (fast feedback)
M = 2
L = 4    (slow, expensive)
```

### Ambiguity Penalty (0-5, denominator term)

```
ambiguity_penalty = 0
If acceptance_criteria count < 3:     +2
If interface_obligations empty:       +1
If relevant open_questions exist:     +1
If files_affected empty:              +1
```

Stories with high ambiguity get lower scores, biasing dispatch toward well-specified work.

### Interface Risk (0-3, denominator term)

```
interface_risk = min(3, count of interface contracts this story creates or modifies)
```

Cross-module interface changes carry integration risk in parallel execution.

---

## Worked Example

| Story                                                      | Value | Unblock | CP  | Reuse | Cost | Ambig | IFace | Score    |
| ---------------------------------------------------------- | ----- | ------- | --- | ----- | ---- | ----- | ----- | -------- |
| Auth middleware (critical path, high, unblocks 3, enabler) | 7     | 6       | 10  | 6     | S=1  | 0     | 1     | **14.5** |
| User model (not critical, medium, unblocks 2)              | 4     | 4       | 4   | 0     | S=1  | 0     | 0     | **12.0** |
| Dashboard layout (not critical, low, leaf, ambiguous)      | 1     | 0       | 2   | 0     | L=4  | 3     | 0     | **0.43** |

---

## Grouped Change Dispatch

For stories with `compile_mode: grouped_change` sharing the same `change_group`:

- **Readiness gate:** Use **min readiness_score** of any story in the group — if any story is under-specified, the group isn't ready.
- **Dispatch score:** Sum `business_value` and `unblock_potential` across the group; use max `critical_path_urgency` and max `reuse_leverage`; divide by sum of `complexity_cost` + max `ambiguity_penalty` + max `interface_risk`.
- Dispatch the entire group as one unit — one OpenSpec change, one workspace, one PR.
- Max 3 stories per group. Failure of any story fails the group.

---

## Routing Thresholds

Canonical `readiness_score` bands. The **actions** per band are mode-specific — interactive actions live in `/aep-dispatch` Step 7, autonomous actions in the autopilot tick Step ⑥ (Check Routing) — but the bands themselves are defined only here:

- **>= 0.7** — dispatch-ready: 3+ testable acceptance criteria, interface obligations defined, verification complete, files affected identified.
- **0.5–0.7** — borderline: spec mostly complete, but a design decision is still open.
- **< 0.5** — under-specified: vague or fewer than 3 acceptance criteria, missing interface details, relevant open questions.
- **`attempt_count >= 2`** — always escalate to a human, regardless of readiness: repeated failures need human attention.
