---
name: aep-autopilot
description: |-
  Orchestrate the full dispatch-launch-monitor-review-wrap-dispatch cycle autonomously. One command to go hands-free. Use when the user says "autopilot", "run autonomously", "auto dispatch loop", "hands-free mode", "start building everything", "go auto", "run the pipeline", "let it run", "manage workspaces", or wants to dispatch and monitor multiple stories without manual intervention. Always trigger this over /dispatch when the user wants continuous autonomous operation rather than a single story dispatch. Runs from the main workspace only.
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
       │  tick ①  read state                          │
       │  tick ②  sync signals                        │
       │  tick ③  wrap completed workspaces            │
       │  tick ④  GUIDE COMPLETION (quality + merge)   │
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

## STOP — Orchestrator Boundaries

**Read this section FIRST. It overrides everything below.**

You are an **ORCHESTRATOR**, not an **EXECUTOR**. All code operations happen inside workspace agents. The main session never reads, reviews, edits, or evaluates workspace code directly. The single most common autopilot failure is violating this boundary.

### Executor backend (read once, applies throughout)

Autopilot steers running workspace sessions, so it requires a **session backend
(B1/B2)** from the executor abstraction — a workspace you can instruct mid-flight.
Every "send to workspace" action in this skill is `executor.nudge(ws, msg)`, and
every liveness probe is `executor.liveness(ws)`. The `tmux send-keys` /
`tmux capture-pane` commands shown throughout the tick protocol are the **B1/B2
implementation** of those verbs (see `.claude/skills/aep-executor/references/backends.md`).

- **B1/B2:** proceed normally — `nudge` = `tmux send-keys`, `liveness` = `tmux capture-pane` (+ `git diff --stat`).
  - **Multi-line nudge form:** the nudge messages below are multi-line, so send each as
    `tmux send-keys -t <ws>:0.0 -l -- "<msg>"` followed by `tmux send-keys -t <ws>:0.0 Enter`.
    A bare `tmux send-keys "<msg>" Enter` lets embedded newlines submit the message
    line-by-line — always use the `-l` + separate-`Enter` form (this is `executor.nudge()`).
- **B3/B4 (no session to steer):** autopilot's tick/nudge model does not apply.
  Codex `/launch` is subagent-first (B3) by design: it is manual/headless and
  worktree-bound, but it is not an autopilot-steerable tmux session. If the user
  wants hands-free batch under Claude Code, that is the **dynamic-workflow path
  (B4)** reached via `/dispatch … with workflow`, which is its own orchestrator —
  not something autopilot drives. If detection yields B3/B4, report that
  autopilot needs a session backend and stop.

### The tick CHECK runs in a cheap delegate (token isolation)

Each tick splits into **CHECK** (read + analyze + write state) and **ACT**
(execute). The CHECK is delegated to a **cheap, context-isolated agent** via
`executor.check()` (Claude Code: a Haiku subagent; Codex: a `codex exec` cheap
one-shot) so the long-lived orchestrator session doesn't accumulate per-tick
reading. The CHECK reads **only** `autopilot-state.json` + workspace
`signals/` + `gh pr view` — **never workspace code** — and returns a compact
action list; the orchestrator then ACTs on it (nudge / wrap / launch / escalate).

> **This is NOT a violation of the next rule.** The CHECK delegate is the
> orchestrator offloading its own _signal reading and bookkeeping_ to a cheap
> context. It never reads workspace code and never reviews code. The forbidden
> thing is spawning a **code reviewer** (which would read the implementation) from
> main — that stays forbidden. Signals-only analysis ≠ code review.

### Never Do List

- **NEVER use the Agent tool to spawn code reviewers from the main session** — this is categorical, not a context problem you can engineer around. Even a worktree-bound reviewer spawned from main pulls workspace code and quality judgments into the orchestrator's context, which is exactly what the boundary forbids; "but I could give it the worktree" is **not** a valid exception. Instead: `executor.nudge()` (B1/B2 = `tmux send-keys`) to trigger the workspace's own gen/eval loop. (The executor's B3/B4 backends do spawn worktree-bound agents — but that is `/launch`/`/build` spawning the builder/evaluator _inside_ the workspace, never autopilot spawning a reviewer from main; autopilot runs only on session backends anyway.)
- **NEVER call `gh pr merge`** — workspace agents run pre-merge checks (rebase, CI verification, comment resolution) as part of Phase 12. Merging from main bypasses these checks and has caused premature merges where incomplete test results were accepted. Instead: send a tmux nudge telling the workspace to complete Phase 12.
- **NEVER read workspace source files** — only read signal files under `.dev-workflow/signals/`. The main session's job is to observe progress via signals, not to understand the code. If you need code reviewed, trigger the workspace's evaluator.
- **NEVER use `Read`, `Grep`, or `Bash` to inspect workspace code** — even "just checking" pulls implementation details into main session context, which leads to the main session forming opinions about code quality and then acting on them (spawning reviewers, suggesting fixes). Stay out of workspace code entirely.
- **NEVER write eval-response files** — evaluation integrity depends on separation between generator and evaluator. The main session is neither — it's the orchestrator. Writing eval responses breaks the trust model.

**If you are about to do any of the above: STOP. Send a tmux command to the workspace agent instead.**

### Allowed Actions (from main session)

| Action                | How                                                                     |
| --------------------- | ----------------------------------------------------------------------- |
| Read workspace status | Read `.feature-workspaces/<name>/.dev-workflow/signals/status.json`     |
| Trigger code review   | `tmux send-keys -t <session>:0.0 "..." Enter`                           |
| Send feedback         | Write to `.feature-workspaces/<name>/.dev-workflow/signals/feedback.md` |
| Nudge stuck agent     | `tmux send-keys -t <session>:0.0 "..." Enter`                           |
| Check PR state        | `gh pr view <number> --json state` (observe only — never act on merge)  |

### Forbidden Actions (from main session)

| Forbidden action     | Do this instead                           |
| -------------------- | ----------------------------------------- |
| Read workspace code  | Trigger workspace's gen/eval via tmux     |
| Spawn review agents  | Send review trigger via tmux send-keys    |
| Run tests            | Workspace handles its own test phases     |
| Edit workspace files | Send instructions via tmux or feedback.md |
| Evaluate code        | Monitor eval-response files for results   |
| Merge PRs            | Workspace agent merges via Phase 12       |

### Two Gen/Eval Concerns — Strictly Separate

| Concern                    | Owner                    | What it evaluates                                    | Where it runs                       |
| -------------------------- | ------------------------ | ---------------------------------------------------- | ----------------------------------- |
| **Code quality**           | Workspace agent          | Code correctness, security, completeness             | Inside workspace tmux session       |
| **Orchestration learning** | Autopilot (main session) | Patterns across workspaces: failures, costs, retries | Main session, feeds into `/reflect` |

The autopilot does NOT evaluate workspace code. It triggers and monitors the workspace's own gen/eval loop. See `references/review-trigger.md` for detection logic and `references/orchestration-learning.md` for meta-learning.

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

5. Start the recurring **periodic driver** that fires `/autopilot tick` every
   interval. The driver is host-specific (the tick itself is host-agnostic):

   **Claude Code — `/loop` (GA, in-session):**

   ```
   /loop <interval> /autopilot tick
   ```

   Where `<interval>` is from `--loop` flag (default: `5m`). `/loop` invokes
   `/autopilot tick` each interval; the tick keeps the main session cheap by
   delegating its CHECK to a Haiku subagent (see below).

   **Codex — no native `/loop`; schedule `codex exec` externally.** Run each tick
   as a fresh cheap one-shot (already context-isolated, so no nested CHECK needed).
   Recommended: a macOS `launchd` agent with `StartInterval=300` (or cron / a
   `while … sleep 300` loop) running:

   ```bash
   codex exec -m gpt-5.4-mini -c model_reasoning_effort=low \
     -C "$PWD" --skip-git-repo-check --dangerously-bypass-approvals-and-sandbox \
     "/autopilot tick" < /dev/null    # < /dev/null: exec hangs on stdin otherwise
   ```

   Tell the user the exact scheduler snippet for their platform; AEP does not
   install it for them.

---

## `/autopilot tick`

The per-tick handler invoked by the periodic driver. Can also be run manually at any time. **Idempotent** — safe to run multiple times with no state change producing no duplicate actions.

**Execution model — CHECK → ACT.** A tick is two halves:

- **CHECK (cheap, isolated):** run the read + analyze + write-state work via
  `executor.check(prompt, schema)` — a Haiku subagent (Claude Code) or a `codex exec`
  cheap one-shot (Codex). It reads `autopilot-state.json` + every workspace
  `signals/` + `gh pr view`, computes transitions / stuck / dispatch capacity,
  **writes the updated `autopilot-state.json` + `autopilot-status.md`**, and returns
  the compact **action list** (schema in `aep-executor/references/backends.md`:
  `{summary, state_written, actions:[{type, workspace, story_id, message, reason}]}`).
  All the token-heavy reading stays in the throwaway agent.
- **ACT (orchestrator):** execute the returned `actions` — `nudge` via
  `executor.nudge()`, `wrap` via `/wrap` (max one per tick), `launch` via `/launch`
  (max one per tick), `escalate`/`design` per the pause protocol. These are few, so
  the main session stays cheap.

> **On Codex** the whole tick already runs as an isolated cheap `codex exec`, so the
> CHECK can run inline (no nested `executor.check` needed) — the ACT still applies.
> **On Claude Code** the long-lived `/loop` session is exactly why the CHECK is
> delegated to a Haiku subagent.

The 7-step protocol below is the **content of the CHECK prompt** (steps ①②④a④b-detect
⑤⑥-scoring ⑦ = analysis + state write) plus the **ACT items** it emits (③ wrap,
④b/④c nudges, ⑥ launch, escalations). Full detail in `references/tick-protocol.md`.

**Before every tick, re-read the "STOP — Orchestrator Boundaries" section above.**

**Summary (annotated `[CHECK]` analysis vs `[ACT]` orchestrator action):**

```
① READ STATE [CHECK] → read .dev-workflow/autopilot-state.json
   - Exit if status != "running"
   - Exit if tick lock active (previous tick still running)
   - Set tick lock

② SYNC SIGNALS [CHECK] → for each workspace in state:
   - Read .feature-workspaces/<name>/.dev-workflow/signals/status.json
   - Update workspace entry in state (phase, story_status, completion_pct, pr_url, blockers)

③ WRAP COMPLETED [ACT] → for each workspace where story_status == "completed":
   (CHECK emits a `wrap` action per completed workspace; orchestrator runs it)
   - Run /wrap for this workspace (max ONE per tick — git operations serialize)
   - Remove workspace from state after wrap completes
   - Break to step ⑦

④ GUIDE COMPLETION [CHECK detect → ACT nudge] → for each workspace, guide toward quality and merge:
   (CHECK reads PR/eval state and decides; orchestrator sends the `nudge` actions)
   ALL NUDGE ACTIONS USE executor.nudge() (B1/B2 = tmux send-keys). NEVER spawn code reviewers.
   NEVER call gh pr merge. Workspace agents own merging.

   Decision tree:
     has pr_url? → ④a (check state) → OPEN? → ④b (quality gate) → PASS? → ④c (nudge merge)
     no pr_url, phase >= 5? → ④b (quality gate) → PASS? → leave alone (workspace creates PR)
     phase < 5? → skip

   ④a. CHECK PR STATE — for workspaces with pr_url set:
       - gh pr view <number> --json state
       - MERGED → update story_status to "completed" (Step ③ wraps next tick)
       - CLOSED → update story_status to "failed"
       - OPEN → proceed to ④b/④c

   ④b. QUALITY GATE — for ALL workspaces at phase >= 5 (pre-PR and post-PR):
       - Check for eval-response files in .feature-workspaces/<name>/.dev-workflow/signals/
       - If no eval-response with PASS exists → trigger gen/eval via tmux:
         tmux send-keys -t <workspace-name>:0.0 \
           "Run Phase 5 code review now. Write eval-request.md, spawn evaluator \
            via tmux split-window, and execute the gen/eval loop per the build \
            skill Phase 5 protocol." Enter
       - If stuck at Phase 5 (2+ ticks) → re-trigger via tmux
       - No response after 6 ticks (30 min) → add escalation
       - See references/review-trigger.md for full detection logic

   ④c. GUIDE TO MERGE — when eval PASSED AND pr_url set, nudge toward Phase 12:
       - Only nudge ONCE — skip if last_action is already "merge_nudged"
       - If eval PASSED but phase < 12 and not yet nudged:
         tmux send-keys -t <workspace-name>:0.0 \
           "Your code review eval has PASSED. Proceed to Phase 12: run pre-merge \
            checks (rebase on main, verify CI, check comments) then merge the PR. \
            In autopilot mode, merge when all checks pass without waiting for user \
            confirmation." Enter
       - If phase == 12 and stuck (2+ ticks):
         tmux send-keys -t <workspace-name>:0.0 \
           "Complete Phase 12 merge now: 1) git fetch origin && git rebase origin/main && \
            git push --force-with-lease origin feat/<name> 2) Verify CI green \
            3) gh pr merge <number> --squash --delete-branch. \
            Update status.json with story_status completed." Enter
       - If phase == 12 and progressing → leave alone

⑤ DETECT STUCK [CHECK detect → ACT nudge] → for each workspace:
   - Compare (phase, completion_pct) with previous tick
   - No change → run liveness check, then increment consecutive_stuck_ticks
   - Changed → reset to 0
   - 6 ticks (30 min) stuck → send tmux nudge
   - 12 ticks (60 min) stuck → add escalation, consider pausing

⑥ DISPATCH NEW WORK [CHECK score → ACT launch] → if capacity available:
   - Read product-context.yaml, run dispatch scoring logic (steps 1-3 from /dispatch)
   - available_slots = concurrency_limit - active_workspace_count
   - WAVE ORDERING: Dispatch Wave 1 before Wave 2 within each layer.
   - LAYER GATE: After completing all stories in a layer, check if a `.5` alignment
     layer exists for this layer. If yes, dispatch `.5` layer stories before
     advancing to the next integer layer.
     - Verify calibration artifacts exist before dispatching `.5` stories
       (check `calibration/<type>.yaml` or legacy `design-context.yaml`)
     - If missing → add escalation requesting the user to run `/calibrate <type>`
   - OUTCOME CONTRACT: If layer has outcome_contract, pause for /reflect evaluation
     before advancing to next layer.
   - GROUPED CHANGES: If top story has compile_mode: grouped_change, dispatch
     the entire change_group as one unit (one workspace, one PR).
   - For top-scored ready story (or group):
     - Route by readiness_score: >=0.7 → /launch, <0.5 → escalate or auto-design
     - If auto_design: true → route through /design automatically (no pause)
     - If auto_design: false and readiness < 0.7 → add escalation, PAUSE autopilot
     - If attempt_count >= 2 → always ESCALATE
     - If well-specified → run /launch (max ONE launch per tick)
   - Add new workspace entry to state (with story_ids, wave, readiness_score)

⑦ WRITE STATE [CHECK]
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

When autopilot encounters a story that needs design input, behavior depends on `topology.routing.auto_design`:

- **`auto_design: false` (default):** Autopilot **pauses entirely** — design decisions may affect other stories and require human judgment.
- **`auto_design: true`:** Autopilot routes the story through `/design` automatically, then `/launch`. No pause.

### Escalation Conditions (when `auto_design: false`)

A story triggers design escalation when ANY of:

1. **`readiness_score < 0.7`** — spec is not dispatch-ready (fewer than 3 acceptance criteria, missing interface obligations, unresolved open questions, etc.)
2. **`attempt_count >= 2`** — repeated failures suggest the spec is insufficient, not the implementation (always escalates, even with `auto_design: true`)

### Auto-Design Conditions (when `auto_design: true`)

Instead of pausing, autopilot:

1. Runs `/design` for the story to refine the spec
2. Re-computes `readiness_score` after `/design`
3. If readiness >= 0.7 → `/launch`
4. If readiness still < 0.5 → escalate (auto-design couldn't resolve the ambiguity)

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

## Guardrails

- **Main workspace only** — refuse to run if `pwd` contains `.feature-workspaces`
- **All code operations happen inside workspace agents** — the main session NEVER reads, reviews, edits, or evaluates workspace code directly. It only sends prompts via tmux and reads signal files
- **Never spawn Agent tools for code review** — all reviews run inside the workspace's tmux session via `tmux send-keys`. This is the #1 violation to watch for.
- **Never merge PRs** — workspace agents own Phase 12 merge; autopilot only detects already-merged PRs
- **Guide workspace agents to merge** — when eval passes and CI is green, send tmux nudge for Phase 12 completion; do not wait passively for workspace to figure it out
- **Never dispatch stories with unmet dependencies** — even under autonomous mode
- **Never treat SKIP-only test results as PASS** — at least 1 PASS required for test/integration stories
- **Never treat "no checks" as passing** — for integration/test stories, require at least one passing check OR explicit eval-response PASS
- **Never write eval-response files** — that's the workspace evaluator's job
- **One wrap per tick** — wraps involve git operations that must serialize
- **One launch per tick** — keeps tick duration under 60 seconds
- **Respect WIP limits** — never exceed `topology.routing.concurrency_limit`
- **Atomic state writes** — write to `.tmp` then rename to prevent corruption
- **Tick lock** — prevent overlapping ticks via `tick_in_progress` timestamp
- **Pause on design ambiguity** — unless `auto_design: true`, escalate to human when readiness < 0.7
- **Always escalate on repeated failures** — `attempt_count >= 2` always pauses, even with `auto_design: true`

---

## Next Steps

After autopilot completes a layer or is stopped:

| Action              | When                                                                              |
| ------------------- | --------------------------------------------------------------------------------- |
| `/reflect`          | After layer completes — evaluate outcome contracts (Step 2.75), classify feedback |
| `/autopilot status` | Anytime — check progress and escalations                                          |
| `/autopilot`        | After resolving a pause — resume the loop                                         |
| `/dispatch`         | Manual mode — pick a specific story interactively                                 |
