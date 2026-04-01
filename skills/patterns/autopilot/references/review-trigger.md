# Workspace Gen/Eval Triggering Protocol

How the autopilot detects when a workspace needs code review and triggers the workspace's own gen/eval loop via tmux. The autopilot **never evaluates code itself** — it triggers and monitors.

> **Note:** Key trigger templates from this file are also inlined in `tick-protocol.md` Step ④ (GUIDE COMPLETION) to ensure the LLM sees them in context during tick execution.

---

## Principle: Trigger, Don't Execute

The workspace agent owns code quality evaluation. The autopilot's role is:

1. **Detect** when gen/eval should be running but isn't
2. **Trigger** the workspace to run its own Phase 5 gen/eval loop via `tmux send-keys`
3. **Monitor** the eval-response files for results
4. **Act** on results (guide workspace toward merge via tmux, or let workspace fix issues)

---

## Detection Logic

Each tick, for every active workspace, check:

### Condition 1: Phase 4 Complete, No Eval Started

```
workspace.phase >= 5
AND NOT exists(.feature-workspaces/<name>/.dev-workflow/signals/eval-response-*.md)
AND workspace.code_review_triggered == false
```

**Meaning:** Implementation is done (or past done) but the workspace hasn't run Phase 5.
**Likely cause:** Agent skipped Phase 5, had a context reset, or moved straight to Phase 9.

### Condition 2: Stuck at Phase 5

```
workspace.phase == 5
AND workspace.consecutive_stuck_ticks >= 2
AND workspace.code_review_triggered == true
```

**Meaning:** Workspace is at Phase 5 but making no progress for 10+ minutes after being triggered.
**Likely cause:** Evaluator spawn failed, tmux command didn't execute, or agent is in a loop.

### Condition 3: Phase 10+ Without Recent Eval

```
workspace.phase >= 10
AND workspace.pr_url is set
```

Check: does the latest `eval-response-*.md` file predate the latest PR commit?

```bash
# Get latest eval-response timestamp
EVAL_TIME=$(stat -f %m .feature-workspaces/<name>/.dev-workflow/signals/eval-response-*.md 2>/dev/null | sort -n | tail -1)

# Get latest PR commit timestamp
PR_COMMIT_TIME=$(gh pr view <number> --json commits --jq '.commits[-1].committedDate')
```

If eval is older than the latest commit, code has changed since review.

### Condition 4: Moved Past Phase 5 Without PASS

```
workspace.phase > 5
AND latest eval-response shows "Result: FAIL"
```

**Meaning:** Workspace moved to later phases despite failing evaluation.
**Action:** Send workspace back to Phase 5.

---

## Trigger Commands

### First Trigger (gentle)

```bash
tmux send-keys -t <workspace-name>:0.0 \
  "Run Phase 5 code review now. Write eval-request.md to .dev-workflow/signals/, spawn an evaluator via tmux split-window per the build skill Phase 5 protocol, and execute the gen/eval loop. Check .dev-workflow/signals/feedback.md for context." Enter
```

Set in state: `code_review_triggered = true`, `code_review_triggered_at = now`, `last_action = "review_triggered"`.

### Re-trigger (after 3 ticks / 15 min no response)

```bash
tmux send-keys -t <workspace-name>:0.0 \
  "URGENT: Phase 5 code review has not produced results. If you had a context reset, run bash .dev-workflow/init.sh to recover state. Then immediately: 1) Write eval-request.md 2) Run tmux split-window to spawn evaluator 3) Execute the gen/eval loop per build Phase 5." Enter
```

Set: `last_action = "review_re_triggered"`.

### Send Back (moved past without PASS)

```bash
tmux send-keys -t <workspace-name>:0.0 \
  "Your latest eval-response shows FAIL but you moved past Phase 5. Go back to Phase 5: fix the FAIL items identified in the eval-response, then re-run the gen/eval loop. Do not proceed to PR until eval passes." Enter
```

### Fresh Review for PR (Phase 10+ with stale eval)

```bash
tmux send-keys -t <workspace-name>:0.0 \
  "Code has changed since your last evaluation. Re-run Phase 5 code review on the current state before proceeding with the PR. Write a new eval-request.md and spawn a fresh evaluator." Enter
```

---

## Monitoring Protocol

Each tick after triggering, check for eval-response files:

```bash
ls .feature-workspaces/<name>/.dev-workflow/signals/eval-response-*.md 2>/dev/null
```

### If eval-response exists:

Read the latest response file. Parse the `## Result: PASS / FAIL` line.

**PASS:**

- Set `eval_rounds_completed` to the round number
- Workspace can proceed to Phase 9+ (it will do so autonomously)
- Tick step ④c will guide workspace toward Phase 12 merge via tmux

**FAIL:**

- Check if workspace is actively fixing (`phase == 5`, `completion_pct` changing) → let it work
- If stuck → re-trigger (see above)
- Track round count via `eval_rounds_completed`

### If no eval-response after trigger:

| Ticks since trigger | Action                                                     |
| ------------------- | ---------------------------------------------------------- |
| 1-2                 | Wait — workspace may be running eval                       |
| 3 (15 min)          | Re-trigger with URGENT message                             |
| 6 (30 min)          | Add escalation: "Workspace not responding to eval trigger" |

---

## Escalation

Escalate to human when:

- Workspace has completed 5 eval rounds without PASS (workspace's own max convergence)
- Workspace has not responded to 2 trigger attempts over 30 minutes
- Eval response shows the same findings 3+ consecutive rounds (not converging)

Escalation entry:

```json
{
  "type": "eval_not_converging",
  "story_id": "<id>",
  "workspace": "<name>",
  "reason": "Gen/eval loop failed to converge after 5 rounds",
  "details": "Persistent failures on [dimensions]. Generator cannot fix: [specific issues].",
  "expected_human_action": "Review the eval findings in .feature-workspaces/<name>/.dev-workflow/signals/eval-response-5.md and decide: fix manually, adjust the spec, or defer the story.",
  "created_at": "<ISO8601>",
  "acknowledged": false
}
```
