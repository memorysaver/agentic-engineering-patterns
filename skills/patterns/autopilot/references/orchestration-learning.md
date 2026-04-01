# Orchestration Learning

How the autopilot uses the gen/eval pattern to evaluate its own orchestration quality — not individual code, but cross-workspace patterns. This is the main session's gen/eval concern, strictly separate from workspace-level code evaluation.

---

## Principle: Meta-Evaluation, Not Code Review

| Workspace gen/eval                            | Orchestration gen/eval                          |
| --------------------------------------------- | ----------------------------------------------- |
| "Is this code correct?"                       | "Is our process working?"                       |
| Evaluates one story's implementation          | Evaluates patterns across all stories           |
| Runs inside workspace (tmux session)          | Runs in main session (Agent tool, Context B)    |
| Triggered by autopilot, executed by workspace | Triggered and executed by autopilot             |
| Feeds into: fix code → re-eval                | Feeds into: `/reflect` → update product context |

---

## What to Observe

The orchestration learning protocol examines data from across all workspaces in the current autopilot run:

### Completion Patterns

- Which stories completed successfully vs failed vs got stuck
- Average time-to-completion by complexity (S/M/L)
- Which modules have higher failure rates

### Cost Analysis

- Cost per story (from `status.json` signals)
- Cost per module — are some modules consistently expensive?
- Cost correlation with complexity rating — is L really 4x S?

### Retry Patterns

- Which stories needed multiple attempts (`attempt_count` from product-context.yaml)
- Common failure reasons (from `failure_log` in signals)
- Whether retries with fresh agents succeeded (fresh-agent-retry effectiveness)

### Eval Convergence

- How many eval rounds per story (from `eval_rounds_completed` in state)
- Which scoring dimensions consistently fail (parsed from eval-response files)
- Whether certain feature types (UI, API, security) have worse convergence

### Escalation Analysis

- Escalation frequency and causes
- How long before escalations were resolved
- Whether escalation conditions could have been predicted earlier

### Time Patterns

- Time spent per phase (derived from signal `last_updated` timestamps)
- Bottleneck phases (where workspaces spend the most time)
- Correlation between context package size and completion time

---

## When to Run

Orchestration learning runs at natural checkpoints:

### 1. Layer Complete

When all stories in a layer are completed and the layer gate passes. This is the primary learning checkpoint — a full cycle of dispatch-build-merge is done.

### 2. After Escalation

When an escalation is created. Examine: could this have been predicted? Should dispatch scoring or design escalation thresholds change?

### 3. On Autopilot Stop

When `/autopilot stop` is called or all layers complete. Summary of the entire run.

---

## How to Run

Use the gen/eval pattern's **Context B: Parallel Agent Tool Calls** from the main session:

```
Launch Agent(subagent_type="Plan", prompt="<orchestration evaluator prompt>")
```

### Evaluator Prompt Template

```markdown
You are an ORCHESTRATION EVALUATOR. Analyze the autopilot run data and identify
patterns that should inform future product context and dispatch decisions.

## Data Sources

Read these files:

1. .dev-workflow/autopilot-state.json — current state with all workspace data
2. .dev-workflow/autopilot-history.jsonl — tick-by-tick audit trail
3. product-context.yaml — story specs, complexity ratings, dependencies
4. For each completed workspace:
   - .feature-workspaces/<name>/.dev-workflow/signals/status.json (if still exists)
   - .feature-workspaces/<name>/.dev-workflow/signals/eval-response-\*.md (if exists)

## Analysis Dimensions

1. **Accuracy of estimates:** Did complexity ratings (S/M/L) match actual effort?
2. **Spec quality:** Did stories with more acceptance criteria complete faster/more reliably?
3. **Module patterns:** Are certain modules consistently problematic?
4. **Eval effectiveness:** Did the gen/eval loop catch real issues or just slow things down?
5. **Dispatch efficiency:** Was the scoring formula (CP + value + unblock / complexity) optimal?
6. **Cost efficiency:** Where was money well-spent vs wasted?

## Output Format

Write findings to .dev-workflow/autopilot-learnings.md using this structure:

### Finding: [Title]

**Category:** [estimate_accuracy | spec_quality | module_pattern | eval_effectiveness | dispatch_efficiency | cost_efficiency]
**Evidence:** [specific data points]
**Recommendation:** [actionable change to product context or process]
**Severity:** [info | suggestion | important]
```

---

## Output Format

`.dev-workflow/autopilot-learnings.md`:

```markdown
# Autopilot Learnings — Layer N

**Generated:** <timestamp>
**Stories analyzed:** N completed, N failed, N in-progress
**Total cost:** $XX.XX

---

## Findings

### Finding: Complexity L stories take 4x longer and fail 2x more than rated

**Category:** estimate_accuracy
**Evidence:** L stories averaged 3.2 hours vs S at 0.8 hours (4x, not 4x as rated).
L stories had 40% failure rate vs S at 20%. PROJ-007 and PROJ-012 both failed on
first attempt.
**Recommendation:** Consider splitting L stories into 2-3 M stories before dispatch.
Update /map guidance to discourage L complexity.
**Severity:** important

### Finding: Auth module stories consistently fail on Security dimension

**Category:** module_pattern
**Evidence:** PROJ-003 and PROJ-008 both received Security score 2 on first eval
round. Both eventually passed after 3 rounds. No other module had security failures.
**Recommendation:** Add explicit security acceptance criteria to auth module stories
in product-context.yaml. Consider adding security-focused evaluator criteria preset
for auth stories.
**Severity:** suggestion

### Finding: Stories with 5+ acceptance criteria completed 30% faster

**Category:** spec_quality
**Evidence:** Stories with ≥5 criteria averaged 1.1 hours. Stories with exactly 3
criteria averaged 1.6 hours. Hypothesis: more criteria = less ambiguity = fewer
false starts.
**Recommendation:** During /map, aim for 5+ criteria per story. Flag stories with
exactly 3 criteria for potential spec refinement.
**Severity:** info
```

---

## Integration with `/reflect`

The learnings file is consumed by `/reflect` during its feedback classification step:

1. `/reflect` reads `.dev-workflow/autopilot-learnings.md`
2. Each finding is classified: Bug, Refinement, Discovery, or Opportunity Shift
3. Findings become updates to `product-context.yaml`:
   - Estimate accuracy → update complexity ratings
   - Spec quality → add acceptance criteria to future stories
   - Module patterns → update module definitions in architecture section
   - Dispatch efficiency → update topology.routing settings
   - Cost efficiency → update cost.alerts thresholds

This closes the meta-learning loop:

```
/autopilot runs → workspaces build → autopilot observes →
learnings → /reflect → product-context updated →
next /autopilot run benefits from improved context
```
