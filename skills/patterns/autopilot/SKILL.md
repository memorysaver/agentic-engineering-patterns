---
name: autopilot
description: |-
  Orchestrate the full dispatch-launch-monitor-review-wrap-dispatch cycle autonomously. One command to go hands-free. Use when the user says "autopilot", "run autonomously", "auto dispatch loop", "hands-free mode". Runs from the main workspace only. Usage: /autopilot [--loop 5m] [status|stop]
---

# Autopilot

One command to go autonomous. Initializes state, runs the first tick, and starts a recurring loop — all in one invocation.

```
/autopilot                  # start with default 5m interval
/autopilot --loop 10m       # start with custom interval
/autopilot status           # check progress and escalations
/autopilot stop             # gracefully stop the loop
```

**Where this fits:**

```
/envision → /map → /validate
  → /autopilot
       ┌─────────────────────────────────────────────┐
       │  tick ①  sync signals                        │
       │  tick ②  wrap completed workspaces            │
       │  tick ③  merge ready PRs                      │
       │  tick ④  trigger code review via tmux         │
       │  tick ⑤  detect stuck workspaces              │
       │  tick ⑥  dispatch new work (/launch)          │
       │  tick ⑦  write state                          │
       │  ... repeat every 5 min ...                   │
       └─────────────────────────────────────────────┘
  → /reflect (after layer completes or autopilot stops)
```

**Session:** Main session only (never from a feature workspace)
**State:** `.dev-workflow/autopilot-state.json` (machine-readable), `.dev-workflow/autopilot-status.md` (human-readable)

---

## `/autopilot` (default — start)

Initialize autopilot, run the first tick, and start the recurring loop. This is a single command — no second step needed.

**Usage:**

```
/autopilot                  # default: 5 minute tick interval
/autopilot --loop 10m       # custom interval
/autopilot --loop 3m        # faster for active development
```

### Prerequisites

```bash
# 1. Must be on main workspace (not inside a feature workspace)
pwd | grep -q '.feature-workspaces' && echo "ABORT: Run from main workspace only" && exit 1

# 2. Product context must exist
[ -f product-context.yaml ] || echo "ABORT: Run /envision and /map first"

# 3. Autonomous routing must be enabled
# Check topology.routing.autonomous: true in product-context.yaml
```

Verify these conditions before proceeding:

- **Main workspace guard:** `pwd` must NOT contain `.feature-workspaces`
- **Product context exists:** `product-context.yaml` must exist with a `stories` section
- **Autonomous enabled:** `topology.routing.autonomous: true` must be set
- **Stories available:** At least one story must be `ready` or `in_progress`
- **Validated:** Product context should have passed `/validate` (both passes)

### Start Protocol

1. Create `.dev-workflow/` if it doesn't exist:

   ```bash
   mkdir -p .dev-workflow
   ```

2. Initialize `.dev-workflow/autopilot-state.json` — see `references/state-schema.md` for the full schema:

   ```json
   {
     "version": 1,
     "status": "running",
     "started_at": "<ISO8601>",
     "last_tick_at": null,
     "tick_count": 0,
     "workspaces": {},
     "escalations": [],
     "stats": {
       "stories_completed": 0,
       "stories_failed": 0,
       "total_ticks": 0,
       "total_cost_usd": 0
     }
   }
   ```

3. Write initial `.dev-workflow/autopilot-status.md`:

   ```markdown
   # Autopilot Status

   **Status:** Running
   **Started:** <timestamp>
   **Tick count:** 0

   ## Active Workspaces

   None yet.

   ## Next Action

   First tick will sync signals and dispatch work.
   ```

4. Run the first tick immediately (see tick protocol below).

5. Start the recurring loop using the `/loop` skill:

   ```
   /loop <interval> /autopilot tick
   ```

   Where `<interval>` is from `--loop` flag (default: `5m`). This starts the `/loop` skill which will invoke `/autopilot tick` on the specified interval automatically.

---

## `/autopilot tick`

The per-tick handler invoked by `/loop` on each interval. Can also be run manually at any time. **Idempotent** — safe to run multiple times with no state change producing no duplicate actions.

Follow the 8-step tick protocol documented in `references/tick-protocol.md`.

**Summary:**

```
① READ STATE → read .dev-workflow/autopilot-state.json
   - Exit if status != "running"
   - Exit if tick lock active (previous tick still running)
   - Set tick lock

② SYNC SIGNALS → for each workspace in state:
   - Read .feature-workspaces/<name>/.dev-workflow/signals/status.json
   - Update workspace entry in state (phase, story_status, completion_pct, pr_url, blockers)

③ WRAP COMPLETED → for each workspace where story_status == "completed":
   - Run /wrap for this workspace (max ONE per tick — git operations serialize)
   - Remove workspace from state after wrap completes
   - Break to step ⑧

④ MERGE READY → for each workspace where story_status == "in_review" AND pr_url set:
   - Check CI: gh pr checks <number>
   - Check reviews: gh pr view <number> --json reviewDecision
   - Check workspace eval: read latest eval-response file — must show PASS
   - If all green → gh pr merge <number> --squash --delete-branch
   - If CI failed → tmux send-keys to workspace requesting fix

⑤ CODE REVIEW → detect workspaces needing gen/eval:
   - Phase 4 complete but no eval-response files → trigger via tmux
   - Phase 10+ but no recent eval → trigger via tmux
   - Stuck at Phase 5 → re-trigger via tmux
   - See references/review-trigger.md for tmux command templates

⑥ DETECT STUCK → for each workspace:
   - Compare (phase, completion_pct) with previous tick
   - No change → increment consecutive_stuck_ticks
   - Changed → reset to 0
   - 6 ticks (30 min) stuck → send tmux nudge
   - 12 ticks (60 min) stuck → add escalation, consider pausing

⑦ DISPATCH NEW WORK → if capacity available:
   - Read product-context.yaml, run dispatch scoring logic (steps 1-3 from /dispatch)
   - available_slots = concurrency_limit - active_workspace_count
   - For top-scored ready story:
     - Check design escalation conditions (see below)
     - If needs design → add escalation, PAUSE autopilot
     - If well-specified → run /launch (max ONE launch per tick)
   - Add new workspace entry to state

⑧ WRITE STATE
   - Write .dev-workflow/autopilot-state.json (atomic: write .tmp then rename)
   - Append tick summary to .dev-workflow/autopilot-history.jsonl
   - Update .dev-workflow/autopilot-status.md
   - Increment tick_count, set last_tick_at
   - Release tick lock
```

---

### `/autopilot status`

Read and display the current autopilot state.

```bash
cat .dev-workflow/autopilot-status.md
```

Also parse `.dev-workflow/autopilot-state.json` and present:

- **Status:** running / paused / stopped
- **Uptime:** since started_at, tick count
- **Active workspaces:** table with name, story_id, phase, completion_pct, last_action
- **Pending escalations:** each with type, story_id, reason
- **Stats:** stories completed, failed, total cost
- **If paused:** Why paused, what human feedback is expected, how to resume

---

## `/autopilot stop`

Gracefully stop the autopilot and cancel the recurring loop.

1. Set `status: "stopped"` in `.dev-workflow/autopilot-state.json`
2. Update `.dev-workflow/autopilot-status.md` with stopped state
3. Log stop event to `.dev-workflow/autopilot-history.jsonl`
4. Cancel the `/loop` (use the loop skill's cancel mechanism)

**What happens:**

- The recurring loop is cancelled — no more ticks
- Running workspaces continue autonomously (they don't depend on autopilot)

**What does NOT happen:**

- Workspaces are NOT killed — they continue their `/build` flow
- Product context is NOT modified
- No wraps or merges are triggered

```
Autopilot stopped. Active workspaces continue running independently.
To resume: /autopilot
```

---

## Design Escalation

The autopilot **pauses entirely** when it encounters a story that needs human design input. This is not a per-story skip — it's a full pause because design decisions may affect other stories.

### Escalation Conditions

A story triggers design escalation when ANY of:

1. **Complexity L with fewer than 3 acceptance criteria** — large and underspecified
2. **UI-heavy activity AND complexity M or L** — UI stories need interactive design review (layout, theme, interaction patterns)
3. **`attempt_count >= 2`** — repeated failures suggest the spec is insufficient, not the implementation
4. **Story would route to `/design` per dispatch Step 7** — fewer than 3 acceptance criteria, missing interface details, open questions

### Pause Protocol

When escalation triggers:

1. Set `status: "paused"` in `.dev-workflow/autopilot-state.json`
2. Add escalation entry with detailed context:
   ```json
   {
     "type": "design_needed",
     "story_id": "PROJ-010",
     "reason": "Complexity L with 1 acceptance criterion, UI-heavy activity 'Settings'",
     "details": "The story 'Add settings page' lacks specificity...",
     "expected_human_action": "Run /design PROJ-010 to refine the spec...",
     "created_at": "<ISO8601>",
     "acknowledged": false
   }
   ```
3. Write `.dev-workflow/autopilot-status.md` with:
   - **Why paused** — the specific story and condition
   - **What needs human attention** — what's ambiguous, what decisions require human judgment
   - **Expected human feedback** — specific actions (run /design, add acceptance criteria, etc.)
   - **Current state** — active workspaces still running, stories completed, what's blocked
   - **Detailed guidelines** — why this story can't be auto-designed (e.g., "UI layout decisions require visual design judgment that the agent cannot make autonomously")
4. Log to `.dev-workflow/autopilot-history.jsonl`

### Resuming After Pause

After the human resolves the design issue:

```
/autopilot
```

This re-reads the product context (now with refined specs), re-initializes the loop, and resumes ticking.

---

## Code Review: Separation of Concerns

The autopilot maintains strict separation between two gen/eval concerns:

| Concern                    | Owner                    | What it evaluates                                    | Where it runs                       |
| -------------------------- | ------------------------ | ---------------------------------------------------- | ----------------------------------- |
| **Code quality**           | Workspace agent          | Code correctness, security, completeness             | Inside workspace tmux session       |
| **Orchestration learning** | Autopilot (main session) | Patterns across workspaces: failures, costs, retries | Main session, feeds into `/reflect` |

**Autopilot does NOT evaluate workspace code.** It triggers and monitors the workspace's own gen/eval loop. See `references/review-trigger.md` for the triggering protocol and `references/orchestration-learning.md` for the meta-learning pattern.

---

## Guardrails

- **Main workspace only** — refuse to run if `pwd` contains `.feature-workspaces`
- **Never dispatch stories with unmet dependencies** — even under autonomous mode
- **Never merge without green CI** — always verify `gh pr checks` before merge
- **Never write eval-response files** — that's the workspace evaluator's job
- **One wrap per tick** — wraps involve git operations that must serialize
- **One launch per tick** — keeps tick duration under 60 seconds
- **Respect WIP limits** — never exceed `topology.routing.concurrency_limit`
- **Atomic state writes** — write to `.tmp` then rename to prevent corruption
- **Tick lock** — prevent overlapping ticks via `tick_in_progress` timestamp
- **Pause on design ambiguity** — do not auto-design; escalate to human

---

## Next Steps

After autopilot completes a layer or is stopped:

| Action              | When                                                              |
| ------------------- | ----------------------------------------------------------------- |
| `/reflect`          | After layer completes — classify feedback, update product context |
| `/autopilot status` | Anytime — check progress and escalations                          |
| `/autopilot`        | After resolving a pause — resume the loop                         |
| `/dispatch`         | Manual mode — pick a specific story interactively                 |
