# Tick Protocol

The 7-step state machine executed on each autopilot tick. Each tick is idempotent — running it twice with no external state change produces the same result and takes no duplicate actions.

**Target duration:** <60 seconds per tick
**Invocation:** `/loop 5m /autopilot tick` or manual `/autopilot tick`

> **BOUNDARY REMINDER:** The autopilot is an orchestrator. Every action on a workspace happens via `tmux send-keys`. Never spawn Agent tools, never read workspace source code, never call `gh pr merge`. See SKILL.md "STOP — Orchestrator Boundaries" section.

---

## Step ①: Read State

```bash
cat .dev-workflow/autopilot-state.json
```

**Exit conditions:**

- `status` is not `"running"` → log "autopilot not running, skipping tick" and exit
- `tick_in_progress` timestamp exists AND is less than 4 minutes old → log "previous tick still running, skipping" and exit (prevents overlapping ticks when `/loop` fires before the previous tick completes)

**If proceeding:**

- Set `tick_in_progress` to current ISO8601 timestamp
- Write state immediately (this is the tick lock)

---

## Step ②: Sync Signals

Read signal files from all active workspaces and update state:

```bash
for ws_name in $(jq -r '.workspaces | keys[]' .dev-workflow/autopilot-state.json); do
  signal=".feature-workspaces/$ws_name/.dev-workflow/signals/status.json"
  if [ -f "$signal" ]; then
    cat "$signal"
  fi
done
```

For each workspace in `state.workspaces`:

| Signal field     | State field to update             |
| ---------------- | --------------------------------- |
| `phase`          | `workspaces[name].phase`          |
| `phase_name`     | `workspaces[name].phase_name`     |
| `story_status`   | `workspaces[name].story_status`   |
| `completion_pct` | `workspaces[name].completion_pct` |
| `pr_url`         | `workspaces[name].pr_url`         |
| `blockers`       | `workspaces[name].blockers`       |
| `cost_usd`       | `workspaces[name].cost_usd`       |
| `completed_at`   | `workspaces[name].completed_at`   |
| `failure_log`    | `workspaces[name].failure_log`    |

**If signal file doesn't exist:** Keep previous state values. The workspace may not have written signals yet (still initializing).

**If `story_status` is `"failed"`:**

- Check `failure_log` for structured error info
- Add escalation if `attempt_count` exceeds `max_retries` (default 3)

---

## Step ③: Wrap Completed Workspaces

For each workspace where `story_status == "completed"`:

1. Verify the workspace hasn't already been wrapped (`last_action != "wrapping"` and `last_action != "wrapped"`)
2. Run `/wrap` for this workspace:
   - This runs on main: `jj git fetch`, rebase, archive OpenSpec change, sync story status to YAML, remove workspace
3. Set `last_action = "wrapping"`
4. After wrap completes:
   - Remove workspace entry from state
   - Increment `stats.stories_completed`
   - Add `cost_usd` to `stats.total_cost_usd`

**Max ONE wrap per tick.** Wraps modify `product-context.yaml` and involve git operations. Running multiple wraps risks conflicts. If multiple workspaces completed simultaneously, they get wrapped across consecutive ticks.

After wrapping, **skip to step ⑦** (write state). The next tick will handle dispatch of newly-ready stories (which the wrap may have unblocked via cascade).

---

## Step ④: Guide Completion

**This is the most important step. ALL actions here use `tmux send-keys`. NEVER spawn Agent tools. NEVER call `gh pr merge`. Workspace agents own code review and merging.**

For each workspace, guide it through quality gates and toward merge completion. This step combines PR state detection, quality enforcement, and merge guidance.

### Decision Tree (quick reference)

```
For each workspace:
  Has pr_url?
  ├─ YES → ④a: check PR state
  │   ├─ MERGED/CLOSED → update state, done with this workspace
  │   └─ OPEN → ④b: has eval-response with PASS?
  │       ├─ NO  → trigger gen/eval via tmux (if not already triggered)
  │       └─ YES → ④c: guide to merge via tmux (if not already nudged)
  └─ NO, phase >= 5?
      ├─ YES → ④b: has eval-response with PASS?
      │   ├─ NO  → trigger gen/eval via tmux (if not already triggered)
      │   └─ YES → leave alone (workspace will create PR autonomously)
      └─ NO (phase < 5) → skip, still implementing
```

### Sub-step ④a: Check PR State

For each workspace where `pr_url` is set:

```bash
gh pr view <number> --json state --jq '.state'
```

- **If state == `"MERGED"`:** Update workspace `story_status` to `"completed"`, set `completed_at` to current ISO8601 timestamp, set `last_action = "detected_merged"`. The next tick's Step ③ will wrap it.
- **If state == `"CLOSED"`:** Update workspace `story_status` to `"failed"`, add `failure_log` noting PR was closed without merge, set `last_action = "detected_closed"`.
- **If state == `"OPEN"`:** Proceed to sub-steps ④b and ④c.

**Autopilot NEVER calls `gh pr merge`.** That is the workspace agent's job (Phase 12 of `/build`). This eliminates premature-merge bugs where autopilot merges before the workspace agent has finished its full flow.

### Sub-step ④b: Quality Gate — Ensure Gen/Eval

**Applies to all workspaces at phase >= 5** — both pre-PR (phase 5-9) and post-PR (phase 10+). This is the universal quality gate.

**Check `topology.routing.skip_human_eval` first:**

- `skip_human_eval: all` → skip the quality gate entirely for all stories, proceed to ④c
- `skip_human_eval: backend` → skip the quality gate for stories in non-UI modules (check `story.activity` — if null or infrastructure, skip). UI stories still require eval.
- `skip_human_eval: none` (default) → apply the full quality gate below

Check whether a passing evaluation exists:

```bash
ls .feature-workspaces/<name>/.dev-workflow/signals/eval-response-*.md 2>/dev/null
```

**If latest eval-response shows "Result: PASS"** → quality gate satisfied, proceed to ④c.

**If no eval-response exists OR latest shows "Result: FAIL":**

Check detection conditions (see `references/review-trigger.md` for full logic):

1. **Phase >= 5, no eval-response, not yet triggered:** First trigger needed
2. **Phase == 5, stuck 2+ ticks, already triggered:** Re-trigger needed
3. **Phase >= 10, eval older than latest PR commit:** Fresh review needed
4. **Phase > 5, latest eval shows FAIL:** Send back to Phase 5

**Trigger gen/eval via tmux (NEVER spawn an Agent tool):**

```bash
# First trigger (gentle)
tmux send-keys -t <workspace-name>:0.0 \
  "Run Phase 5 code review now. Write eval-request.md, spawn evaluator via tmux split-window, and execute the gen/eval loop per the build skill Phase 5 protocol. Read .dev-workflow/signals/feedback.md for any additional context." Enter
```

Set in state: `code_review_triggered = true`, `code_review_triggered_at = now`, `last_action = "review_triggered"`.

**Re-trigger after 3 ticks (15 min) with no response:**

```bash
tmux send-keys -t <workspace-name>:0.0 \
  "URGENT: Phase 5 code review has not started. If you had a context reset, read .dev-workflow/init.sh to recover state, then run Phase 5 immediately." Enter
```

Set: `last_action = "review_re_triggered"`.

**Send back (moved past Phase 5 without PASS):**

```bash
tmux send-keys -t <workspace-name>:0.0 \
  "Your latest eval-response shows FAIL but you moved past Phase 5. Go back to Phase 5: fix the FAIL items identified in the eval-response, then re-run the gen/eval loop. Do not proceed to PR until eval passes." Enter
```

**Fresh review for PR (Phase 10+ with stale eval):**

```bash
tmux send-keys -t <workspace-name>:0.0 \
  "Code has changed since your last evaluation. Re-run Phase 5 code review on the current state before proceeding with the PR. Write a new eval-request.md and spawn a fresh evaluator." Enter
```

**Escalation:** No eval-response after 6 ticks (30 min) post-trigger → add escalation with type `"eval_not_converging"`.

### Sub-step ④c: Guide to Merge

For workspaces where the quality gate is satisfied (eval PASS exists) AND PR is OPEN.

**Guard: only nudge once.** Check `last_action` before sending — if already `"merge_nudged"`, do not re-send the nudge. The workspace received the instruction and is working on it. Re-nudging every tick floods the workspace with duplicate prompts.

**1. If `phase < 12` AND `last_action != "merge_nudged"` — workspace hasn't started merge yet:**

```bash
tmux send-keys -t <workspace-name>:0.0 \
  "Your code review eval has PASSED. Proceed to Phase 12 now: run pre-merge checks (rebase on main, verify CI, check comments) then merge the PR. In autopilot mode you do not need user confirmation — merge when all Phase 12 checks pass." Enter
```

Set `last_action = "merge_nudged"`, `last_action_at = now`.

**2. If `phase == 12` AND `consecutive_stuck_ticks >= 2` — workspace started merge but is stuck:**

```bash
tmux send-keys -t <workspace-name>:0.0 \
  "Complete Phase 12 merge now: 1) jj git fetch && jj rebase -d main@origin 2) Verify CI green 3) gh pr merge <number> --squash --delete-branch. Then update status.json with story_status completed." Enter
```

Set `last_action = "merge_stuck_nudged"`, `last_action_at = now`.

**3. If `phase == 12` AND progressing normally** → leave alone.

### Monitoring Protocol

Each tick after triggering gen/eval, check for eval-response files:

```bash
ls .feature-workspaces/<name>/.dev-workflow/signals/eval-response-*.md 2>/dev/null
```

**PASS:** Set `eval_rounds_completed` to the round number. Workspace can proceed to Phase 9+ (it will do so autonomously). Step ④c will guide toward merge next tick.

**FAIL:** Check if workspace is actively fixing (`phase == 5`, `completion_pct` changing) → let it work. If stuck → re-trigger.

| Ticks since trigger | Action                                                     |
| ------------------- | ---------------------------------------------------------- |
| 1-2                 | Wait — workspace may be running eval                       |
| 3 (15 min)          | Re-trigger with URGENT message                             |
| 6 (30 min)          | Add escalation: "Workspace not responding to eval trigger" |

---

## Step ⑤: Detect Stuck Workspaces

For each workspace, compare current `(phase, completion_pct)` with the values from the previous tick:

- **Different values:** Reset `consecutive_stuck_ticks` to 0
- **Same values:** Run liveness check before incrementing (see below)

### Liveness Check

When signals are stale (same phase and completion_pct as previous tick), check whether the workspace agent is still actively working before counting it as stuck:

**Step 1 — Check tmux pane activity:**

```bash
tmux capture-pane -t <workspace-name>:0.0 -p -S -20
```

Compare the captured output against `last_tmux_hash` stored in the workspace state entry.

- **`last_tmux_hash` is null** (first tick after launch or restart) → Populate `last_tmux_hash` with hash of current output. Do NOT increment `consecutive_stuck_ticks`. The workspace gets benefit of the doubt on its first stale-signal tick.
- **tmux session doesn't exist** (capture-pane fails) → Treat as no activity. Increment `consecutive_stuck_ticks`.
- **Output hash differs from `last_tmux_hash`** → Agent is active, signals are lagging. Update `last_tmux_hash`. Do NOT increment `consecutive_stuck_ticks`.
- **Output hash matches `last_tmux_hash`** → Proceed to Step 2.

**Step 2 — Check for uncommitted code changes:**

```bash
cd .feature-workspaces/<workspace-name> && jj diff --stat
```

- **Has uncommitted changes** → Agent is writing code via tool use (file edits happen but no terminal output scrolls). Do NOT increment `consecutive_stuck_ticks`.
- **No uncommitted changes** → Agent is truly idle. Increment `consecutive_stuck_ticks`.

### Thresholds

| Stuck ticks | Duration | Action                                                        |
| ----------- | -------- | ------------------------------------------------------------- |
| 3           | 15 min   | Check if workspace has blockers. If yes, log but don't nudge. |
| 6           | 30 min   | Send tmux nudge (see below). Log warning.                     |
| 12          | 60 min   | Add escalation. Consider pausing if on critical path.         |

### Nudge Command (30 min stuck)

```bash
tmux send-keys -t <workspace-name>:0.0 \
  "You appear stuck at Phase <N> (<phase_name>) for 30 minutes. Check for errors, read .dev-workflow/signals/feedback.md for any instructions, and continue. If you need help, update status.json with blockers." Enter
```

### Escalation (60 min stuck)

Add to `escalations[]`:

```json
{
  "type": "stuck",
  "story_id": "<story_id>",
  "workspace": "<name>",
  "reason": "Workspace stuck at Phase <N> for 60 minutes",
  "phase": <N>,
  "blockers": [...],
  "created_at": "<ISO8601>",
  "acknowledged": false
}
```

If the stuck workspace is on the critical path, consider pausing autopilot to get human attention.

---

## Step ⑥: Dispatch New Work

### Check Capacity

```
active_count = count of workspaces in state (not wrapped/completed)
concurrency_limit = topology.routing.concurrency_limit from product-context.yaml (default 5)
available_slots = concurrency_limit - active_count
```

If `available_slots <= 0`: skip dispatch, log "WIP limit reached".

### Run Dispatch Scoring

Reuse the dispatch scoring logic from `/dispatch` steps 1-3:

1. **Determine active layer** — find the first layer with incomplete stories
2. **Layer gate check** — if active layer > 0, verify previous layer gate passed
3. **Wave ordering** — consult the `waves` section from `product-context.yaml`. Within the active layer, dispatch Wave 1 stories before Wave 2, etc. Only advance to the next wave when all stories in the current wave are completed or in_progress.
4. **Filter ready queue** — stories with `status: ready` in active layer and current wave, excluding file-conflict stories
5. **Compute readiness_score** per story (see `/dispatch` Step 3):
   ```
   readiness_score = (min(3, acceptance_criteria_count) + interfaces_defined*2 + files_identified*1 + verification_defined*2 + no_open_questions*2) / 10
   ```
6. **Compute dispatch_score** per story:
   ```
   dispatch_score = (business_value + unblock_potential + critical_path_urgency + reuse_leverage) / (complexity_cost + ambiguity_penalty + interface_risk)
   ```
   Where `business_value` uses `story.business_value` if set, otherwise derived from priority (critical=10, high=7, medium=4, low=1).

### Grouped Change Handling

Before scoring individual stories, check for `compile_mode: grouped_change`:

1. Identify stories sharing the same `change_group`
2. Score the group as one unit: sum `business_value` and `unblock_potential` across the group; use max `critical_path_urgency` and max `reuse_leverage`; divide by sum of `complexity_cost` + max `ambiguity_penalty` + max `interface_risk`
3. Use **min readiness_score** of any story in the group as the group's readiness gate
4. Dispatch the entire group as one unit — one `/launch`, one workspace, one OpenSpec change containing all grouped stories

### Check Routing

For the top-scored story (or group), use `readiness_score` for routing:

- **readiness_score >= 0.7** → dispatch to `/launch`
- **readiness_score 0.5–0.7** → check `topology.routing.auto_design`:
  - If `auto_design: true` → route through `/design` automatically (no pause), then `/launch`
  - If `auto_design: false` → **ESCALATE** (pause for human design input)
- **readiness_score < 0.5** → check `topology.routing.auto_design`:
  - If `auto_design: true` → route through `/design` automatically (no pause), then `/launch`
  - If `auto_design: false` → **ESCALATE** (pause for human design input)
- **`attempt_count >= 2`** → always **ESCALATE** regardless of readiness (repeated failures need human attention)

If escalation triggers: follow the pause protocol from the main SKILL.md. Do not dispatch.

### Dispatch

If no escalation:

1. Run `/dispatch` for the top story or group (autopilot acts as the "user" selecting the story)
2. If routed through `/design` (auto_design mode): run `/design` first, then `/launch`
3. Run `/launch` for the dispatched story (or group)
4. Add workspace entry to state:

   ```json
   {
     "story_id": "<id>",
     "story_ids": ["<id>"],
     "compile_mode": "single_change",
     "change_group": null,
     "wave": 1,
     "readiness_score": 0.8,
     "routed_to": "launch",
     "phase": 0,
     "phase_name": "initializing",
     "story_status": "in_progress",
     "completion_pct": 0,
     "pr_url": null,
     "last_action": "launched",
     "last_action_at": "<ISO8601>",
     "code_review_triggered": false,
     "code_review_triggered_at": null,
     "eval_rounds_completed": 0,
     "consecutive_stuck_ticks": 0,
     "blockers": []
   }
   ```

   For grouped changes: `story_ids` contains all story IDs in the group, `compile_mode` is `"grouped_change"`, `change_group` is the group ID, and `story_id` is the first story in the group (used as the primary identifier).

**Max ONE launch per tick.** Launching involves creating a jj workspace, starting a tmux session, and sending a bootstrap prompt — too slow for multiple per tick.

### Layer Completion

If all stories in the active layer are completed (after wraps):

1. Suggest running the layer gate integration test
2. If gate passes: update `layer_gates[layer].status: passed`
3. **Outcome contract check:** If `product.layers[active_layer].outcome_contract` exists, add an escalation requesting the user to evaluate the outcome contract before advancing. Autopilot **pauses** — outcome evaluation requires human judgment (user testing, analytics, qualitative assessment). The user runs `/reflect` which evaluates outcome contracts in Step 2.75. After `/reflect` completes, resume autopilot.
4. If no outcome contract or outcome evaluation passes: advance to next layer
5. If gate fails: add escalation, pause autopilot (layer gate failures require human judgment)
6. If all layers complete: stop autopilot, notify human

### Orchestration Learning Checkpoint

At natural checkpoints (layer complete, escalation, or autopilot stop), run the orchestration learning protocol from `references/orchestration-learning.md`. This produces `.dev-workflow/autopilot-learnings.md` with cross-workspace findings that feed into `/reflect`.

---

## Step ⑦: Write State

1. **Atomic write:** Write updated state to `.dev-workflow/autopilot-state.json.tmp`, then rename to `.dev-workflow/autopilot-state.json`

2. **Append tick summary** to `.dev-workflow/autopilot-history.jsonl`:

   ```json
   {
     "tick": 42,
     "at": "<ISO8601>",
     "actions": [
       "synced 3 workspaces",
       "detected PROJ-003 merged",
       "triggered review for PROJ-004"
     ],
     "workspaces_active": 2,
     "stories_completed_total": 5
   }
   ```

3. **Update status file** `.dev-workflow/autopilot-status.md` — human-readable summary of current state

4. **Increment** `tick_count`, set `last_tick_at = now`

5. **Release tick lock** — set `tick_in_progress` to `null`

---

## Workspace State Derivation

The autopilot does NOT maintain a formal FSM enum. It derives the logical state from the combination of fields on each tick:

| Derived state  | Condition                                       |
| -------------- | ----------------------------------------------- |
| Initializing   | `phase == 0`                                    |
| Implementing   | `phase == 4`                                    |
| Reviewing      | `phase == 5`                                    |
| Testing        | `phase >= 6 AND phase <= 8`                     |
| PR created     | `phase == 10 AND story_status == "in_review"`   |
| CI/Review loop | `phase == 11 AND story_status == "in_review"`   |
| Awaiting merge | `story_status == "in_review"` AND `phase >= 11` |
| Completed      | `story_status == "completed"`                   |
| Failed         | `story_status == "failed"`                      |
| Stuck          | `consecutive_stuck_ticks >= 6`                  |
