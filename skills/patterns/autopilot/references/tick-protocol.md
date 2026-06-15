# Tick Protocol

The 7-step state machine executed on each autopilot tick (with a ③.5 post-merge guard sub-step between wrap and guide-completion). Each tick is idempotent — running it twice with no external state change produces the same result and takes no duplicate actions.

**Target duration:** <60 seconds of work per tick (under the goal driver the
turn then waits the per-tick floor — step ⑦ — before ending)
**Invocation:** goal driver (default) — `/goal "<layer-N condition>"` re-fires
this tick each turn until the layer completes; loop driver (fallback) —
`/loop 5m /aep-autopilot tick`; or manual `/aep-autopilot tick`

> **BOUNDARY REMINDER:** The autopilot is an orchestrator. Every action on a workspace is `executor.nudge()` / `executor.liveness()` — autopilot runs only on **steerable, driver-compatible modes** (native-bg-subagent / claude-bg / codex-subagent / codex-exec / legacy; see the per-mode transport table in SKILL.md and `aep-executor/references/backends.md`). The nudge texts in this file are mode-independent — deliver each through the workspace's `backend` transport (`SendMessage(to: agentId)` / `feedback.md` / `send_input` / `codex exec resume` / `tmux send-keys`). Liveness is the [post-spawn liveness probe](../../executor/references/backends.md#post-spawn-liveness-probe) (process exists AND worktree active) — **never** roster/state membership. Never spawn code reviewers from main, never read workspace source code, never call `gh pr merge`. See SKILL.md "STOP — Orchestrator Boundaries".

**EXECUTION MODEL — CHECK → ACT** (see SKILL.md "Execution model"). A tick is two halves:

- **CHECK** — steps ①②⑤, the read-only/scoring parts of ④⑥, and the ⑦ state write. These run in a cheap, context-isolated agent via `executor.check()` (Claude Code Haiku subagent / Codex `codex exec`) and produce an **action list**. The CHECK reads signals only — never workspace code.
- **ACT** — the orchestrator performs the emitted actions: ③ wrap, ③.5 post-merge guard (dogfood / reflect / revert), ④/⑤ nudges, ⑥ launch, escalations.

The action-list schema is `{summary, state_written, actions[]}`, each action `{type, workspace, story_id, message, reason}` (full schema in `aep-executor/references/backends.md`). The step recipes below are both the content of the CHECK prompt and the templates the ACT executes.

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

**If `blocked_on == "human"` (or `needs-human.md` has an unresolved entry):**

- The workspace is at a **human gate**, not stuck — exempt it from stuck
  counting and emit an `escalate` action of type `human_gate`. The
  `expected_human_action` is hub-and-spoke: **the human answers in the main
  session**; the orchestrator relays it on the mode's channel —
  re-spawn a bg subagent into the worktree with the answer
  (native-bg-subagent, parked) / resume the session with the answer
  (claude-bg, parked) / `send_input` (codex-subagent) /
  `codex exec resume <agent_id> "<answer>"` (codex-exec, parked) /
  `executor.nudge()` (legacy). A **parked** worker (gate-and-park: its run
  ended cleanly after recording the gate) is resumed into the same worktree
  with the answer + recovery bootstrap — do not treat the exited process as
  crashed or stuck.
- Clear the escalation when the entry gains a `resolved:` line.

**Orphan check (session-bound modes — native-bg-subagent, codex-subagent) — by real liveness, not roster:**

- Apply the [post-spawn liveness probe](../../executor/references/backends.md#post-spawn-liveness-probe):
  the workspace is an **orphan** when its `agent_id` no longer appears in TaskList
  / `list_agents` (lead restarted, worker crashed, **or the spawn never actually
  started** — e.g. the removed claude-team's truncated-launch failure) **and/or**
  the worktree shows no live process — even if state/roster still says "active".
  If the worktree exists with progress, emit a `launch` action flagged
  `readopt: true` — the ACT re-spawns a worker **into the existing worktree** with
  the recovery bootstrap ("Run `bash .dev-workflow/init.sh` to recover state, read
  `.dev-workflow/signals/feedback.md`, then continue the /aep-build flow"), then
  updates `agent_id`. Do not mark the story failed; do not create a new worktree.
  **Never** accept roster/state membership as proof the worker is alive.

---

## Step ③: Wrap Completed Workspaces

For each workspace where `story_status == "completed"`:

1. Verify the workspace hasn't already been wrapped (`last_action != "wrapping"` and `last_action != "wrapped"`)
2. Run `/aep-wrap` for this workspace:
   - This runs on the integration branch (`$BASE`): `git fetch && git pull --ff-only origin "$BASE"`, archive OpenSpec change, sync story status to YAML, remove worktree
3. Set `last_action = "wrapping"`
4. After wrap completes:
   - Remove workspace entry from state
   - Increment `stats.stories_completed`
   - Add `cost_usd` to `stats.total_cost_usd`

**Max ONE wrap per tick.** Wraps modify `product-context.yaml` and involve git operations. Running multiple wraps risks conflicts. If multiple workspaces completed simultaneously, they get wrapped across consecutive ticks.

After wrapping, **skip to step ⑦** (write state). The next tick will handle dispatch of newly-ready stories (which the wrap may have unblocked via cascade).

---

## Step ③.5: Post-Merge Guard

For each recently-merged story (one Step ③ wrapped within the monitoring window — default applies per `post-merge-guard.md`), run the **post-merge guard**. The detail lives in `references/post-merge-guard.md`; this step defers to it. Within the monitoring window:

1. **Watch deploy health** — read deploy/CI signals and `gh` only (no workspace code, no `gh pr merge`). The orchestrator boundary holds.
2. **Run host-aware dogfood** — exercise the merged change per the host-aware recipe in `post-merge-guard.md`.

Two issue paths:

- **Dogfood UX / functional issue** → route the finding through the `/aep-reflect` classifier, which auto-creates a follow-up story.
- **Hard regression** (deploy health breaks / CI red on the integration branch) → apply the `post_merge_guard.auto_revert` policy:
  - **DEFAULT (conservative, `auto_revert: false`)** → **warn + escalate** for human decision; do not revert.
  - **`auto_revert: true`** (opt-in) → revert the merge.

Emit any follow-up (reflect story / escalation / revert) as an action; never read workspace source — signals / CI / `gh` only.

---

## Step ④: Guide Completion

**This is the most important step. ALL actions here use `executor.nudge()` (delivered via the workspace's mode transport). NEVER spawn Agent tools for review. NEVER call `gh pr merge`. Workspace agents own code review and merging.**

For each workspace, guide it through quality gates and toward merge completion. This step combines PR state detection, quality enforcement, and merge guidance.

### Decision Tree (quick reference)

```
For each workspace:
  Has pr_url?
  ├─ YES → ④a: check PR state
  │   ├─ MERGED/CLOSED → update state, done with this workspace
  │   └─ OPEN → ④b: has eval-response with PASS?
  │       ├─ NO  → trigger gen/eval via nudge (if not already triggered)
  │       └─ YES → ④c: guide to merge via nudge (if not already nudged)
  └─ NO, phase >= 5?
      ├─ YES → ④b: has eval-response with PASS?
      │   ├─ NO  → trigger gen/eval via nudge (if not already triggered)
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

**Autopilot NEVER calls `gh pr merge`.** That is the workspace agent's job (Phase 12 of `/aep-build`). This eliminates premature-merge bugs where autopilot merges before the workspace agent has finished its full flow.

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

**Trigger gen/eval via `executor.nudge()` (NEVER spawn an Agent tool):**

```
# First trigger (gentle)
executor.nudge(<workspace-name>,
  "Run Phase 5 code review now. Write eval-request.md, spawn the evaluator via executor.spawn_evaluator (your mode's recipe), and execute the gen/eval loop per the build skill Phase 5 protocol. Read .dev-workflow/signals/feedback.md for any additional context.")
```

Set in state: `code_review_triggered = true`, `code_review_triggered_at = now`, `last_action = "review_triggered"`.

**Re-trigger after 3 ticks (15 min) with no response:**

```
executor.nudge(<workspace-name>,
  "URGENT: Phase 5 code review has not started. If you had a context reset, read .dev-workflow/init.sh to recover state, then run Phase 5 immediately.")
```

Set: `last_action = "review_re_triggered"`.

**Send back (moved past Phase 5 without PASS):**

```
executor.nudge(<workspace-name>,
  "Your latest eval-response shows FAIL but you moved past Phase 5. Go back to Phase 5: fix the FAIL items identified in the eval-response, then re-run the gen/eval loop. Do not proceed to PR until eval passes.")
```

**Fresh review for PR (Phase 10+ with stale eval):**

```
executor.nudge(<workspace-name>,
  "Code has changed since your last evaluation. Re-run Phase 5 code review on the current state before proceeding with the PR. Write a new eval-request.md and spawn a fresh evaluator.")
```

**Escalation:** No eval-response after 6 ticks (30 min) post-trigger → before escalating, the workspace must climb the **recovery ladder** (`../../gen-eval/references/recovery-ladder.md`): nudge it to work the ladder's rungs (re-scope, decompose, relax non-essential criteria, etc.) first. Only emit the `"eval_not_converging"` escalation **after the ladder is exhausted** — i.e. the workspace has reported the ladder spent without a PASS.

### Sub-step ④c: Guide to Merge

For workspaces where the quality gate is satisfied (eval PASS exists) AND PR is OPEN.

**Guard: only nudge once.** Check `last_action` before sending — if already `"merge_nudged"`, do not re-send the nudge. The workspace received the instruction and is working on it. Re-nudging every tick floods the workspace with duplicate prompts.

**1. If `phase < 12` AND `last_action != "merge_nudged"` — workspace hasn't started merge yet:**

```
executor.nudge(<workspace-name>,
  "Your code review eval has PASSED. Proceed to Phase 12 now: run pre-merge checks (rebase on main, verify CI, check comments) then merge the PR. In autopilot mode you do not need user confirmation — merge when all Phase 12 checks pass.")
```

Set `last_action = "merge_nudged"`, `last_action_at = now`.

**2. If `phase == 12` AND `consecutive_stuck_ticks >= 2` — workspace started merge but is stuck:**

```
executor.nudge(<workspace-name>,
  "Complete Phase 12 merge now: 1) git fetch origin && git rebase origin/\"$(git config --get aep.integration-branch 2>/dev/null || (git show-ref --verify --quiet refs/remotes/origin/develop && echo develop || echo main))\" && git push --force-with-lease origin feat/<name> 2) Verify CI green 3) gh pr merge <number> --squash --delete-branch. Then update status.json with story_status completed.")
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

When signals are stale (same phase and completion_pct as previous tick), check whether the workspace agent is still actively working before counting it as stuck. **Exempt first:** if `blocked_on == "human"`, the workspace is gated, not stuck.

**Step 1 — Check mode-specific activity** and compare against
`last_liveness_hash` stored in the workspace state entry:

| Mode               | Activity probe                                                                   |
| ------------------ | -------------------------------------------------------------------------------- |
| native-bg-subagent | TaskList / `TaskOutput <agentId>` — task status / output changed?                |
| claude-bg          | `claude agents --json` status + `claude logs <agent_id> \| tail -20`             |
| codex-subagent     | `list_agents` status for `<agent_id>`                                            |
| codex-exec         | `tail -20 .feature-workspaces/<name>/.dev-workflow/worker.log`                   |
| legacy             | `tmux capture-pane -t <name>:0.0 -p -S -20` (a `zsh` pane = never-started spawn) |

- **`last_liveness_hash` is null** (first tick after launch or restart) → Populate it with the hash of the current probe output. Do NOT increment `consecutive_stuck_ticks`. The workspace gets benefit of the doubt on its first stale-signal tick.
- **The agent no longer exists** (bg subagent gone from TaskList, `list_agents` empty, `claude agents` shows exited, tmux session missing) → on a **session-bound mode** this is an **orphan**: emit the re-adoption `launch` action (see Step ②), do NOT count it stuck. On an **OS-bound mode** a missing process means a crashed/exited agent → increment `consecutive_stuck_ticks`.
- **Probe output differs from `last_liveness_hash`** → Agent is active, signals are lagging. Update `last_liveness_hash`. Do NOT increment `consecutive_stuck_ticks`.
- **Probe output matches `last_liveness_hash`** → Proceed to Step 2.

**Step 2 — Check for uncommitted code changes:**

```bash
git -C .feature-workspaces/<workspace-name> diff --stat
```

- **Has uncommitted changes** → Agent is writing code via tool use (file edits happen but no terminal output scrolls). Do NOT increment `consecutive_stuck_ticks`.
- **No uncommitted changes** → Agent is truly idle. Increment `consecutive_stuck_ticks`.

### Thresholds

| Stuck ticks | Duration | Action                                                        |
| ----------- | -------- | ------------------------------------------------------------- |
| 3           | 15 min   | Check if workspace has blockers. If yes, log but don't nudge. |
| 6           | 30 min   | Send nudge via `executor.nudge()` (see below). Log warning.   |
| 12          | 60 min   | Add escalation. Consider pausing if on critical path.         |

### Nudge Command (30 min stuck)

```
executor.nudge(<workspace-name>,
  "You appear stuck at Phase <N> (<phase_name>) for 30 minutes. Check for errors, read .dev-workflow/signals/feedback.md for any instructions, and continue. If you need help, update status.json with blockers.")
```

> **claude-bg:** 6 stuck ticks is the stop+respawn threshold — `claude stop
<agent_id>`, then respawn in the worktree with the recovery bootstrap and
> record the new `agent_id` (recipe in `aep-executor/references/claude-native.md`).

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

Reuse the dispatch scoring logic from `/aep-dispatch` steps 1-3:

1. **Determine active layer** — find the first layer with incomplete stories
2. **Layer gate check** — if active layer > 0, verify previous layer gate passed
3. **Wave ordering** — consult the `waves` section from `product-context.yaml`. Within the active layer, dispatch Wave 1 stories before Wave 2, etc. Only advance to the next wave when all stories in the current wave are completed or in_progress.
4. **Filter ready queue** — stories with `status: ready` in active layer and current wave, excluding file-conflict stories
5. **Compute readiness_score** per story (see `/aep-dispatch` Step 3):
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
4. Dispatch the entire group as one unit — one `/aep-launch`, one workspace, one OpenSpec change containing all grouped stories

### Check Routing

For the top-scored story (or group), use `readiness_score` for routing:

- **readiness_score >= 0.7** → dispatch to `/aep-launch`
- **readiness_score 0.5–0.7** → check `topology.routing.full_auto` / `auto_design`:
  - If `full_auto: true` (master switch) **or** `auto_design: true` → auto-route through the **non-interactive design resolver** (`/aep-design`, no pause), then `/aep-launch`
  - Otherwise → **ESCALATE** (pause for human design input)
- **readiness_score < 0.5** → check `topology.routing.full_auto` / `auto_design`:
  - If `full_auto: true` (master switch) **or** `auto_design: true` → auto-route through the **non-interactive design resolver** (`/aep-design`, no pause), then `/aep-launch`
  - Otherwise → **ESCALATE** (pause for human design input)
- **`attempt_count >= 2`** → always **ESCALATE** regardless of readiness (repeated failures need human attention)

If escalation triggers: follow the pause protocol from the main SKILL.md. Do not dispatch.

### Dispatch

If no escalation:

1. Run `/aep-dispatch` for the top story or group (autopilot acts as the "user" selecting the story)
2. If routed through `/aep-design` (auto_design mode): run `/aep-design` first, then `/aep-launch`
3. Run `/aep-launch` for the dispatched story (or group)
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
     "backend": "native-bg-subagent",
     "agent_id": "<bare-hex bg-subagent id | bg session id | codex agent id | exec session id | tmux session name>",
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
     "last_liveness_hash": null,
     "blockers": []
   }
   ```

   For grouped changes: `story_ids` contains all story IDs in the group, `compile_mode` is `"grouped_change"`, `change_group` is the group ID, and `story_id` is the first story in the group (used as the primary identifier).

**Max ONE launch per tick.** Launching involves creating a git worktree, spawning a worker (bg subagent / bg session / subagent / exec / tmux), running the post-spawn liveness probe, and delivering a bootstrap prompt — too slow for multiple per tick.

### Layer Completion

If all stories in the active layer are completed (after wraps):

1. Suggest running the layer gate integration test
2. If gate passes: update `layer_gates[layer].status: passed`
3. **Outcome contract check:** If `product.layers[active_layer].outcome_contract` exists, decide whether to auto-evaluate or pause:
   - **Quantitative auto-eval:** If `topology.routing.auto_outcome_eval: quantitative` **and** the contract's metric is quantitative (a measurable threshold) → first run `coverage_check([metric])` (`../../../product-context/reflect/references/telemetry-ingestion.md` §1.5); if the metric isn't bound to a telemetry source (the `/aep-map` Telemetry Binding step wasn't done) → **pause** and escalate "run /aep-map observability step" (do not claim auto-coverage). If covered → auto-evaluate via the telemetry-ingestion recipe (ingest the telemetry, compare against the threshold) and **advance without pausing** when it passes. If the metric is qualitative, fall through to the pause rule below.
   - **Qualitative / default pause:** Otherwise (no `auto_outcome_eval`, a qualitative metric, etc.) → **pause** and add an escalation requesting the user to evaluate the outcome contract before advancing — **UNLESS** `topology.routing.full_auto: true`, in which case auto-evaluate via the telemetry-ingestion recipe and advance without pause. Outcome evaluation otherwise requires human judgment (user testing, analytics, qualitative assessment). The user runs `/aep-reflect` which evaluates outcome contracts in Step 2.75. After `/aep-reflect` completes, resume autopilot.
   - Default (no `auto_outcome_eval` / `full_auto` false) preserves the current human pause.
4. If no outcome contract or outcome evaluation passes: advance to next layer
5. If gate fails: add escalation, pause autopilot (layer gate failures require human judgment)
6. If all layers complete: stop autopilot, notify human

### Orchestration Learning Checkpoint

At natural checkpoints (layer complete, escalation, or autopilot stop), run the orchestration learning protocol from `references/orchestration-learning.md`. This produces `.dev-workflow/autopilot-learnings.md` with cross-workspace findings that feed into `/aep-reflect`.

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

### Goal-driver tail (steps 6–7 — skip entirely under the loop driver)

The fixed-interval `/loop` driver stops here: the interval is the cadence and the
`/loop` skill re-fires the next tick. The **goal driver** instead ends each turn
with two extra actions so the `/goal` evaluator can decide whether to re-fire:

6. **Surface the AUTOPILOT status line** into the transcript — one compact,
   **signals-only** line (no workspace code, no file contents) the goal evaluator
   judges the completion condition against. Derive every field from
   `autopilot-state.json` + product-context layer data:

   ```
   AUTOPILOT layer=<N> wave=<W> stories=<total> done=<completed> in_progress=<n>
     wrapped=<n> ready_remaining=<n> paused=<true|false> escalations=<n> tick=<k>
   layer_complete=<true|false>   # true ⟺ done==total AND no worktrees remain
   ```

   `layer_complete=true` (or `paused=true`) is exactly what the goal condition
   keys on. Because the line is signals-only, the evaluator never reads workspace
   code — the orchestrator boundary holds.

7. **Wait the per-tick floor**, then end the turn. This is the anti-hot-loop
   floor — without it `/goal` re-fires the instant the turn ends. Default `5m`
   (`--floor`); implement with the host's sanctioned bounded wait:
   - **Claude Code:** the `Monitor` tool with a hard timeout (a raw foreground
     `sleep` is blocked inside a turn). An early wake on a `signals/` change is an
     allowed optimization but **not** required — the timeout alone guarantees a
     bounded, stable cadence.
   - **Codex:** a shell `sleep <floor>` (no restriction).

   Ending the turn returns control to `/goal`, whose evaluator reads the surfaced
   status line and either re-fires the next tick or stops (layer complete /
   paused). Step ⑤'s stuck detection still runs every tick, so a stalled layer
   escalates → `paused` → the goal stops for the human.

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
