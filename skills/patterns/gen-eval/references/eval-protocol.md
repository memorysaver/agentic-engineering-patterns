# Evaluation Protocol

The request→response→fix loop for running generator/evaluator cycles. Covers execution contexts, signal files, verification tracking, and convergence rules.

---

## Table of Contents

1. [Execution Contexts](#execution-contexts)
2. [The Eval Loop](#the-eval-loop)
3. [Signal Files](#signal-files)
4. [Feature Verification JSON](#feature-verification-json)
5. [Convergence Rules](#convergence-rules)

---

## Execution Contexts

The gen/eval pattern can execute in three different contexts. The protocol is the same; the mechanics differ.

### Context A: Tmux Split Panes (Workspace — used by /build)

Generator runs in the top tmux pane. Evaluator is spawned as a separate Claude Code instance in the bottom pane.

```bash
# Generator spawns evaluator in bottom pane
tmux split-window -v -c "$(pwd)" "claude --dangerously-skip-permissions --rc"

# Generator returns focus to top pane
tmux select-pane -t :.0

# Generator waits for evaluator to initialize, then sends bootstrap prompt
sleep 10
tmux send-keys -t :.1 "<evaluator prompt>" Enter

# Generator polls for response file
while [ ! -f .dev-workflow/signals/eval-response-<N>.md ]; do sleep 15; done

# Generator reads response and kills evaluator pane
tmux kill-pane -t :.1
```

**When to use:** Autonomous workspace implementation. The evaluator needs to test a running application (agent-browser, curl, etc.).

**Note:** Use `tmux split-window`, not `cmux split`. The generator runs inside tmux but was not spawned by cmux, so it cannot use cmux socket commands.

### Context B: Parallel Agent Tool Calls (Main Session — used by /validate)

Generator and evaluator are spawned as parallel agents using the Agent tool. They work independently and their results are consolidated after both complete.

```
Launch in parallel:
  Agent(subagent_type="Plan", prompt="<generator prompt>")
  Agent(subagent_type="Plan", prompt="<evaluator prompt>")
  Agent(subagent_type="Plan", prompt="<protocol checker prompt>")  # optional

Wait for all agents to return.
Consolidate findings.
```

**When to use:** Validating artifacts on the main branch. No running application needed. Agents need read access to the codebase but don't modify it.

### Context C: Subagent Spawning (CI/Automation)

Generator and evaluator are spawned via the Claude API or SDK as separate conversations.

**When to use:** Automated pipelines, CI checks, scheduled validation.

---

## The Eval Loop

### Single-pass mode (validate, design review)

```
1. Assemble context for each agent role
2. Spawn all agents in parallel
3. Wait for all agents to complete
4. Consolidate findings (see findings-format.md)
5. Present findings to user
6. Apply approved fixes
7. Done
```

### Multi-round mode (build, code review)

```
Round 1:
  1. Generator writes eval-request.md
  2. Evaluator reads request + artifacts + criteria
  3. Evaluator scores, writes eval-response-1.md
  4. Generator reads response
  5. If PASS → done
  6. If FAIL → generator fixes issues

Round 2:
  1. Generator writes updated eval-request.md (notes what was fixed)
  2. Evaluator re-evaluates
  3. If PASS → done
  4. If FAIL → generator fixes again

...repeat up to max_rounds (default 5)...

If not converged → escalate to human
```

### Choosing the mode

| Artifact type | Mode | Rationale |
|--------------|------|-----------|
| Product context / design docs | Single-pass | Document doesn't change between eval rounds |
| Code implementation (active) | Multi-round | Generator can fix issues between rounds |
| Code review (PR) | Single-pass | Code is already written; findings are for the author |
| Structured documents | Single-pass | Documents are fixed; validation is a one-time check |

---

## Signal Files

Used in multi-round mode (workspace context). Files live in `.dev-workflow/signals/`.

### eval-request.md (generator writes)

```markdown
# Evaluation Request — Round <N>

## What to evaluate
- [summary of implementation state]
- [which tasks are complete]

## Changes since last round
- [what was fixed since previous evaluation, or "first evaluation"]

## Known issues
- [anything the generator is aware of but hasn't fixed yet]

## Files changed
[output of jj diff --stat or git diff --stat]
```

### eval-response-N.md (evaluator writes)

```markdown
# Evaluation Round <N>

## Findings

### [PASS/FAIL]: [Finding title] ([Dimension]: [Score])
- Steps to reproduce: [concrete steps]
- Expected: [what should happen]
- Actual: [what actually happens]
- Impact: [why this matters]
- Fix: [specific, actionable suggestion]

## Scores
- Completeness: [1-5] — [justification]
- Correctness: [1-5] — [justification]
- UX Quality: [1-5] — [justification]
- Security: [1-5] — [justification]
- Code Quality: [1-5] — [justification]

## Result: PASS / FAIL
[If FAIL: which hard failure thresholds were violated]

## Verification Updates
[Which items in feature-verification.json were updated, with new pass/fail status]
```

### status.json (generator updates at phase boundaries)

```json
{
  "phase": 5,
  "phase_name": "code-review",
  "eval_round": 2,
  "eval_result": "fail",
  "completion_pct": 75,
  "updated_at": "2026-03-30T12:00:00Z"
}
```

---

## Feature Verification JSON

Task-level tracking for code evaluation. Format is intentionally JSON — models tamper with JSON less than Markdown.

### Schema

```json
[
  {
    "task": "string — task description from tasks.md",
    "change_id": "string — jj change short ID (8 chars)",
    "verification_steps": [
      "string — concrete, executable verification step",
      "string — another step"
    ],
    "passes": false,
    "evaluated_by": null,
    "round": null,
    "notes": null
  }
]
```

### Field ownership

| Field | Written by | When |
|-------|-----------|------|
| `task` | Generator (Phase 0) | During initialization |
| `change_id` | Generator (Phase 0) | When creating jj change stack |
| `verification_steps` | Generator (Phase 0) | Extracted from contracts/specs |
| `passes` | **Evaluator only** | After running verification steps |
| `evaluated_by` | **Evaluator only** | Agent identifier |
| `round` | **Evaluator only** | Which eval round |
| `notes` | **Evaluator only** | Detailed findings for this task |

**Critical rule:** The generator MUST NOT modify `verification_steps`, `passes`, `evaluated_by`, `round`, or `notes`. Only the evaluator or a human can update these fields. This ensures the generator cannot mark its own work as passing.

### Example (real-world, round 1)

```json
[
  {
    "task": "feat: expose WORKSPACE_CONTAINER DO binding in wrangler config",
    "change_id": "motvylon",
    "verification_steps": [
      "wrangler.jsonc includes workspace_container durable_object binding",
      "WorkspaceContainer class is exported from the entrypoint",
      "wrangler dev starts without binding errors"
    ],
    "passes": true,
    "evaluated_by": "evaluator-round-1",
    "round": 1,
    "notes": "All three steps verified. Binding present, class exported, dev server starts clean."
  },
  {
    "task": "feat: add source_type column to marketplace_plugins table",
    "change_id": "llzqmpto",
    "verification_steps": [
      "Drizzle schema includes source_type column with enum constraint",
      "Migration generated and applies cleanly",
      "Existing rows default to 'github'"
    ],
    "passes": false,
    "evaluated_by": "evaluator-round-1",
    "round": 1,
    "notes": "Schema column added but migration not generated (db:generate not run). Deployment will fail."
  }
]
```

---

## Convergence Rules

### When to stop the loop

| Condition | Action |
|-----------|--------|
| All dimensions pass thresholds | **STOP — PASS** |
| Round N reaches max_rounds (default 5) | **STOP — ESCALATE** to human |
| Same findings appear 3+ consecutive rounds | **STOP — ESCALATE** (generator can't fix it) |
| Evaluator finds new issues each round (not converging) | **STOP — ESCALATE** after max_rounds |
| Generator and evaluator disagree on pass/fail | **STOP — ESCALATE** for human judgment |

### Escalation format

```markdown
# Escalation — Eval Loop Not Converging

## Round history
- Round 1: FAIL (Correctness 2, Security 2) — 4 findings
- Round 2: FAIL (Correctness 3, Security 2) — 3 findings (1 fixed, 0 new)
- Round 3: FAIL (Security 2) — 2 findings (1 fixed, 0 new)

## Persistent issues
1. [Issue that generator cannot fix — explain why]
2. [Issue that requires architectural decision]

## Recommendation
[What the human should decide or do]
```

### Round budgets

| Artifact type | Max rounds | Typical rounds |
|--------------|------------|----------------|
| Code (implementation) | 5 | 2-3 |
| Code (PR review) | 1 | 1 (single-pass) |
| Product context | 1 | 1 (single-pass) |
| Design artifacts | 1-2 | 1 |
| Documents | 1 | 1 (single-pass) |
