# Workspace Gen/Eval Triggering Protocol

How the autopilot **detects** when a workspace needs code review, so it can trigger the workspace's own gen/eval loop via `executor.nudge()` — delivered through the workspace's mode transport (see the Executor transport summary in SKILL.md). The autopilot **never evaluates code itself** — it triggers and monitors.

> **Scope of this file = the detection conditions.** The nudge/trigger **prompt texts** and the per-tick monitoring recipe are canonical in `tick-protocol.md` Step ④ (GUIDE COMPLETION) — this file names _when_ each trigger fires and points there for _what_ to send.

---

## Principle: Trigger, Don't Execute

The workspace agent owns code quality evaluation. The autopilot's role is:

1. **Detect** when gen/eval should be running but isn't
2. **Trigger** the workspace to run its own Phase 5 gen/eval loop via `executor.nudge()`
3. **Monitor** the eval-response files for results
4. **Act** on results (guide workspace toward merge via `executor.nudge()`, or let workspace fix issues)

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
**Likely cause:** Evaluator spawn failed, the nudge never reached the worker, or agent is in a loop.

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

## Trigger and Monitoring — canonical in tick-protocol.md

Each detection condition above maps to a nudge in `tick-protocol.md` Step ④b, and
the per-tick monitoring recipe (check for `eval-response-*.md`; PASS → guide to
merge next tick via ④c; FAIL → let it fix or re-trigger; the 3-tick / 6-tick
escalation ladder) is Step ④'s **Monitoring Protocol**. The prompt texts and
`last_action` state writes live there — this file does not restate them.

| Detection condition                          | Trigger (tick-protocol.md Step ④b) |
| -------------------------------------------- | ---------------------------------- |
| 1 — Phase ≥ 5, no eval started               | First Trigger (gentle)             |
| 2 — Stuck at Phase 5, already triggered      | Re-trigger (URGENT)                |
| 3 — Phase 10+, eval older than latest commit | Fresh Review for PR                |
| 4 — Moved past Phase 5 with a FAIL eval      | Send Back to Phase 5               |

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
