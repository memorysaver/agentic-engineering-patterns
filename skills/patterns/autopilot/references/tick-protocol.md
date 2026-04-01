# Tick Protocol

The 8-step state machine executed on each autopilot tick. Each tick is idempotent — running it twice with no external state change produces the same result and takes no duplicate actions.

**Target duration:** <60 seconds per tick
**Invocation:** `/loop 5m /autopilot tick` or manual `/autopilot tick`

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

After wrapping, **skip to step ⑧** (write state). The next tick will handle dispatch of newly-ready stories (which the wrap may have unblocked via cascade).

---

## Step ④: Detect Merged PRs

For each workspace where `story_status == "in_review"` AND `pr_url` is set:

### Check PR State

```bash
gh pr view <number> --json state --jq '.state'
```

- **If state == `"MERGED"`:** Update workspace `story_status` to `"completed"`, set `completed_at` to current ISO8601 timestamp, set `last_action = "detected_merged"`. The next tick's Step ③ will wrap it.
- **If state == `"CLOSED"`:** Update workspace `story_status` to `"failed"`, add `failure_log` noting PR was closed without merge, set `last_action = "detected_closed"`.
- **If state == `"OPEN"`:** No action — the workspace agent owns the merge decision via Phase 12 of `/build`.

**Autopilot NEVER calls `gh pr merge`.** That is the workspace agent's job (Phase 12 of `/build`). This eliminates premature-merge bugs where autopilot merges before the workspace agent has finished its full flow (eval, CI verification, test validation).

---

## Step ⑤: Code Review Trigger

Detect workspaces that need gen/eval but haven't run it. See `references/review-trigger.md` for full protocol.

### Detection Logic

For each workspace, check:

1. **Phase 4 complete, no eval:** `phase >= 5` but no `eval-response-*.md` files exist in `.feature-workspaces/<name>/.dev-workflow/signals/`
   → Trigger: workspace should be running Phase 5 but hasn't started

2. **Stuck at Phase 5:** `phase == 5` AND `consecutive_stuck_ticks >= 2` (10 min with no progress)
   → Re-trigger: workspace started Phase 5 but appears stuck

3. **Phase 10+ without recent eval:** `phase >= 10` AND the latest `eval-response-*.md` is older than the latest PR commit
   → Trigger: code changed since last eval, need fresh review

4. **Moved past Phase 5 without PASS:** `phase > 5` AND latest eval-response shows FAIL
   → Send back: workspace skipped evaluation

### Trigger Command

```bash
tmux send-keys -t <workspace-name>:0.0 \
  "Run Phase 5 code review now. Write eval-request.md, spawn evaluator via tmux split-window, and execute the gen/eval loop per the build skill Phase 5 protocol. Read .dev-workflow/signals/feedback.md for any additional context." Enter
```

Set `code_review_triggered = true`, `code_review_triggered_at = now`, `last_action = "review_triggered"`.

**No response after 3 ticks (15 min):** Send stronger nudge:

```bash
tmux send-keys -t <workspace-name>:0.0 \
  "URGENT: Phase 5 code review has not started. If you had a context reset, read .dev-workflow/init.sh to recover state, then run Phase 5 immediately." Enter
```

---

## Step ⑥: Detect Stuck Workspaces

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

## Step ⑦: Dispatch New Work

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
3. **Filter ready queue** — stories with `status: ready` in active layer, excluding file-conflict stories
4. **Compute dispatch score** per story:
   ```
   dispatch_score = (critical_path_urgency + business_value + unblock_potential) / complexity_cost
   ```

### Check Design Escalation

For the top-scored story, check escalation conditions:

- Complexity L with fewer than 3 acceptance criteria → **ESCALATE**
- UI-heavy activity AND complexity M or L → **ESCALATE**
- `attempt_count >= 2` → **ESCALATE**
- Fewer than 3 acceptance criteria, missing interface details → **ESCALATE**

If escalation triggers: follow the pause protocol from the main SKILL.md. Do not dispatch.

### Dispatch

If no escalation:

1. Run `/dispatch` interactively for the top story (autopilot acts as the "user" selecting the story)
2. Run `/launch` for the dispatched story
3. Add workspace entry to state:

   ```json
   {
     "story_id": "<id>",
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

**Max ONE launch per tick.** Launching involves creating a jj workspace, starting a tmux session, and sending a bootstrap prompt — too slow for multiple per tick.

### Layer Completion

If all stories in the active layer are completed (after wraps):

1. Suggest running the layer gate integration test
2. If gate passes: update `layer_gates[layer].status: passed`, advance to next layer
3. If gate fails: add escalation, pause autopilot (layer gate failures require human judgment)
4. If all layers complete: stop autopilot, notify human

### Orchestration Learning Checkpoint

At natural checkpoints (layer complete, escalation, or autopilot stop), run the orchestration learning protocol from `references/orchestration-learning.md`. This produces `.dev-workflow/autopilot-learnings.md` with cross-workspace findings that feed into `/reflect`.

---

## Step ⑧: Write State

1. **Atomic write:** Write updated state to `.dev-workflow/autopilot-state.json.tmp`, then rename to `.dev-workflow/autopilot-state.json`

2. **Append tick summary** to `.dev-workflow/autopilot-history.jsonl`:

   ```json
   {
     "tick": 42,
     "at": "<ISO8601>",
     "actions": ["synced 3 workspaces", "merged PROJ-003", "triggered review for PROJ-004"],
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
