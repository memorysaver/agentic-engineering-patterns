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

The gen/eval pattern can execute in three different contexts. The protocol is the same; the mechanics differ. When `/aep-build` runs the loop, the context tracks the **executor mode** in play (see `aep-executor/references/backends.md`): legacy (pinned tmux) → Context A; claude-team / claude-bg (foreground Task subagent) → Context B mechanism, worktree-bound; codex-subagent / codex-exec (`codex exec --cd` with the aep-evaluator role) and workflow (verify stage) → Context C mechanism, in-host. For the native modes the evaluator prompt is delivered **at spawn time** — no readiness wait, no separate send, no teardown.

### Context A: Tmux Split Panes (Workspace — used by /aep-build under legacy/pinned-tmux)

Generator runs in the top tmux pane. Evaluator is spawned as a separate agent instance in the bottom pane.

```bash
# Generator spawns evaluator in bottom pane. $EXECUTOR is the INTERACTIVE session command
# from detect() (claude → "claude --dangerously-skip-permissions";
# codex → "codex --dangerously-bypass-approvals-and-sandbox"); defaults to the claude form when unset.
tmux split-window -v -c "$(pwd)" "${EXECUTOR:-claude --dangerously-skip-permissions}"

# Generator returns focus to top pane
tmux select-pane -t :.0

# Generator waits for evaluator to initialize, then sends bootstrap prompt.
# Use -l for the (multi-line) prompt, then a single Enter to submit once.
sleep 10
tmux send-keys -t :.1 -l -- "<evaluator prompt>"
tmux send-keys -t :.1 Enter

# Generator polls for response file
while [ ! -f .dev-workflow/signals/eval-response-<N>.md ]; do sleep 15; done

# Generator reads response and kills evaluator pane
tmux kill-pane -t :.1
```

**When to use:** Autonomous workspace implementation. The evaluator needs to test a running application (agent-browser, curl, etc.).

**Note:** Use `tmux split-window`, not `cmux split`. The generator runs inside tmux but was not spawned by cmux, so it cannot use cmux socket commands.

### Context B: Parallel Agent Tool Calls (Main Session — used by /aep-validate)

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

> **Context B mechanism in /aep-build (claude-team / claude-bg):** the generator
> spawns a single **foreground** Task subagent with the evaluator prompt; it
> inherits the worktree cwd and returns on completion. Same mechanism as above,
> but worktree-bound and sequential — not the main-session read-only use.

### Context C: Subagent Spawning (CI/Automation)

Generator and evaluator are spawned via the Claude API or SDK as separate conversations.

**When to use:** Automated pipelines, CI checks, scheduled validation.

> **Context C mechanism in /aep-build (codex-subagent / codex-exec / workflow):**
> a bounded `codex exec --cd <worktree>` one-shot with the `aep-evaluator`
> role, or the workflow's worktree-isolated verify stage. Programmatic spawn,
> but worktree-bound review — not an API/SDK CI job.

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

| Artifact type                 | Mode        | Rationale                                            |
| ----------------------------- | ----------- | ---------------------------------------------------- |
| Product context / design docs | Single-pass | Document doesn't change between eval rounds          |
| Code implementation (active)  | Multi-round | Generator can fix issues between rounds              |
| Code review (PR)              | Single-pass | Code is already written; findings are for the author |
| Structured documents          | Single-pass | Documents are fixed; validation is a one-time check  |

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

[output of git diff --stat "$BASE"...HEAD (integration branch; see git-ref)]
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

### needs-human.md (worker writes — the human-gate record)

When the loop cannot converge (or any decision needs the human), the worker
appends to `.dev-workflow/signals/needs-human.md` and sets
`"blocked_on": "human"` in `status.json`:

```markdown
## <ISO8601> — Phase 5 (eval round <N>)

**Question:** <the decision needed, with options considered>
**Context:** <why the generator/evaluator pair can't resolve it>
```

After acting on the answer the worker appends `resolved: <summary>` and clears
`blocked_on`. How the question reaches the human is per launch mode — see the
Human-Gate Protocol in `aep-executor/references/backends.md`.

---

## Feature Verification JSON

Task-level tracking for code evaluation. Format is intentionally JSON — models tamper with JSON less than Markdown.

### Schema

```json
[
  {
    "task": "string — task description from tasks.md",
    "commit_sha": "string — git short SHA (8 chars), null until task is committed",
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

| Field                | Written by          | When                                                    |
| -------------------- | ------------------- | ------------------------------------------------------- |
| `task`               | Generator (Phase 0) | During initialization                                   |
| `commit_sha`         | Generator (Phase 4) | After committing each task; starts as `null` in Phase 0 |
| `verification_steps` | Generator (Phase 0) | Extracted from contracts/specs                          |
| `passes`             | **Evaluator only**  | After running verification steps                        |
| `evaluated_by`       | **Evaluator only**  | Agent identifier                                        |
| `round`              | **Evaluator only**  | Which eval round                                        |
| `notes`              | **Evaluator only**  | Detailed findings for this task                         |

**Critical rule:** The generator MUST NOT modify `verification_steps`, `passes`, `evaluated_by`, `round`, or `notes`. The generator may write `commit_sha` after each Phase 4 task commit. Only the evaluator or a human can update the verification fields. This ensures the generator cannot mark its own work as passing.

### Example (real-world, round 1)

```json
[
  {
    "task": "feat: expose WORKSPACE_CONTAINER DO binding in wrangler config",
    "commit_sha": "a1b2c3d4",
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
    "commit_sha": "e5f6g7h8",
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

| Condition                                              | Action                                       |
| ------------------------------------------------------ | -------------------------------------------- |
| All dimensions pass thresholds                         | **STOP — PASS**                              |
| Round N reaches max_rounds (default 5)                 | **STOP — ESCALATE** to human                 |
| Same findings appear 3+ consecutive rounds             | **STOP — ESCALATE** (generator can't fix it) |
| Evaluator finds new issues each round (not converging) | **STOP — ESCALATE** after max_rounds         |
| Generator and evaluator disagree on pass/fail          | **STOP — ESCALATE** for human judgment       |

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

| Artifact type         | Max rounds | Typical rounds  |
| --------------------- | ---------- | --------------- |
| Code (implementation) | 5          | 2-3             |
| Code (PR review)      | 1          | 1 (single-pass) |
| Product context       | 1          | 1 (single-pass) |
| Design artifacts      | 1-2        | 1               |
| Documents             | 1          | 1 (single-pass) |
