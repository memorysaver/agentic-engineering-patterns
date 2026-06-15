---
name: aep-autopilot
description: |-
  Orchestrate the full dispatch-launch-monitor-review-wrap-dispatch cycle autonomously. One command to go hands-free. Use when the user says "autopilot", "run autonomously", "auto dispatch loop", "hands-free mode", "start building everything", "go auto", "run the pipeline", "let it run", "manage workspaces", or wants to dispatch and monitor multiple stories without manual intervention. Always trigger this over /aep-dispatch when the user wants continuous autonomous operation rather than a single story dispatch. Runs from the main workspace only.
---

# Autopilot

One command to go autonomous. Initializes state, runs the first tick, and keeps
itself ticking until the current layer is complete — all in one invocation. The
default driver is **goal-driven** (`/goal`, native on both Claude Code and
Codex): each tick advances the layer and then **the runtime decides whether to
re-fire** based on a completion condition, so autopilot **stops on its own** when
the layer is done or a human-judgment gate is hit. The fixed-interval `/loop`
driver remains available as a fallback (`--loop`).

```
/aep-autopilot                  # start: goal-driven driver (default) — drives the CURRENT LAYER to completion, then stops
/aep-autopilot --loop 10m       # start with the fixed-interval loop driver instead (custom interval)
/aep-autopilot --floor 3m       # goal driver with a custom per-tick wait floor (default 5m)
/aep-autopilot status           # check progress and escalations
/aep-autopilot stop             # gracefully stop the driver
```

> **Scope is one layer per run.** The goal driver completes when every story in
> the **current layer** is merged + wrapped, then hands control back so the human
> can run the layer gate / `/aep-reflect` and re-invoke `/aep-autopilot` for the
> next layer. This mirrors the existing pause-at-gate behavior with crisp
> termination instead of an infinite loop.

**Where this fits:**

```
/aep-envision → /aep-map → /aep-validate
  → /aep-autopilot   (goal: "layer N complete")
       ┌─────────────────────────────────────────────┐
       │  tick ①  read state                          │
       │  tick ②  sync signals                        │
       │  tick ③  wrap completed workspaces            │
       │  tick ④  GUIDE COMPLETION (quality + merge)   │
       │  tick ⑤  detect stuck workspaces              │
       │  tick ⑥  dispatch new work (/aep-launch)          │
       │  tick ⑦  write state + SURFACE status + WAIT  │
       └─────────────────────────────────────────────┘
            │ goal evaluator reads the surfaced status line:
            │   "is layer N complete, or is autopilot paused?"
            ├─ no            → re-fire next tick (after the wait floor)
            └─ yes / paused  → STOP (hand back to human)
  → /aep-reflect (after layer completes or autopilot stops)
```

**Session:** Main session only (never from a feature workspace)
**State:** `.dev-workflow/autopilot-state.json` (machine-readable), `.dev-workflow/autopilot-status.md` (human-readable)

---

## STOP — Orchestrator Boundaries

**Read this section FIRST. It overrides everything below.**

You are an **ORCHESTRATOR**, not an **EXECUTOR**. All code operations happen inside workspace agents. The main session never reads, reviews, edits, or evaluates workspace code directly. The single most common autopilot failure is violating this boundary.

### Executor mode + driver (read once, applies throughout)

Autopilot steers running workspace workers, so it requires a **steerable mode
whose lifetime is compatible with the driver** (see the driver ×
backend matrix in `.claude/skills/aep-executor/references/backends.md`). Every
"send to workspace" action in this skill is `executor.nudge(ws, msg)`, and
every liveness probe is `executor.liveness(ws)` — the table below is the
per-mode implementation of those verbs. Nudge texts shown in the tick protocol
are mode-independent; deliver them through your mode's transport.

| Op         | claude-team                                  | claude-bg                                                                  | codex-subagent           | codex-exec                           | legacy                                 |
| ---------- | -------------------------------------------- | -------------------------------------------------------------------------- | ------------------------ | ------------------------------------ | -------------------------------------- |
| `nudge`    | `SendMessage(<ws>, msg)`                     | append `feedback.md`; hard-stuck ≥6 ticks → stop+respawn (recovery prompt) | `send_input(<id>, msg)`  | `codex exec resume <id> "<msg>"`     | `tmux send-keys -l` + separate `Enter` |
| `liveness` | TaskList/TaskGet + `git -C <ws> diff --stat` | `claude agents --json` + `claude logs <id>` tail + git diff                | `list_agents` + git diff | signals + worker.log tail + git diff | `capture-pane` hash + git diff         |
| `present`  | teammate pane / `Shift+Down`                 | `claude attach <id>`                                                       | thread list / `/agent`   | signals + PR                         | cmux tab / `tmux attach`               |

- **Driver compatibility:** session-bound workers (claude-team teammates,
  codex-subagents) die with the orchestrator session. **Long-lived driver** —
  the **goal driver** (`/goal`, default) or the fixed-interval `/loop`, both
  running in a living Claude Code session or Codex main thread → any steerable
  mode. **Cron/launchd driver** (fresh session per tick) → OS-bound modes only
  (**claude-bg**, **codex-exec**, **legacy**) **and only via `/loop` or an
  external scheduler** — `/goal` is inherently in-session and cannot drive a
  fresh-session-per-tick scheduler, so unattended OS-scheduled runs keep using
  the `/loop`/`codex exec` path.
- **claude-team — the standing team:** at `/aep-autopilot` start, `TeamCreate`
  **once** (team "aep"). Each tick-⑥ launch spawns one teammate into it; each
  tick-③ wrap shuts that teammate down (team persists); `/aep-autopilot stop`
  shuts down all teammates then `TeamDelete`. Never one-team-per-story (one
  team at a time is a hard limit) and never expect the team to survive a lead
  restart.
- **Orphan re-adoption (session-bound modes):** if state lists an active
  workspace whose agent no longer exists (TaskList / `list_agents` empty for
  it), it is an **orphan, not a failure** — re-spawn a worker into the
  existing worktree with the recovery bootstrap (protocol in
  `aep-executor/references/backends.md`).
- **workflow / headless (no mid-stage surface):** autopilot's tick/nudge
  model does not apply. Hands-free batch under Claude Code is the
  **workflow** path reached via `/aep-dispatch … with workflow`, which is its own
  orchestrator — not something autopilot drives (its human gates park and
  return to the main agent that authored the workflow, not to autopilot). If
  detection yields only workflow/headless, report that autopilot needs a
  steerable mode and stop.
- **tmux nudge form (legacy only):** multi-line nudges are
  `tmux send-keys -t <ws>:0.0 -l -- "<msg>"` then `tmux send-keys -t <ws>:0.0 Enter` —
  a bare `send-keys "<msg>" Enter` would submit line-by-line.

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

- **NEVER use the Agent tool to spawn code reviewers from the main session** — this is categorical, not a context problem you can engineer around. Even a worktree-bound reviewer spawned from main pulls workspace code and quality judgments into the orchestrator's context, which is exactly what the boundary forbids; "but I could give it the worktree" is **not** a valid exception. Instead: `executor.nudge()` to trigger the workspace's own gen/eval loop. (Spawning a **builder teammate/worker at launch** and steering it with `SendMessage`/`send_input` is the nudge transport, not a reviewer spawn — the evaluator is always spawned by the _generator inside the workspace_, never by autopilot.)
- **NEVER call `gh pr merge`** — workspace agents run pre-merge checks (rebase, CI verification, comment resolution) as part of Phase 12. Merging from main bypasses these checks and has caused premature merges where incomplete test results were accepted. Instead: `executor.nudge()` telling the workspace to complete Phase 12.
- **NEVER read workspace source files** — only read signal files under `.dev-workflow/signals/`. The main session's job is to observe progress via signals, not to understand the code. If you need code reviewed, trigger the workspace's evaluator.
- **NEVER use `Read`, `Grep`, or `Bash` to inspect workspace code** — even "just checking" pulls implementation details into main session context, which leads to the main session forming opinions about code quality and then acting on them (spawning reviewers, suggesting fixes). Stay out of workspace code entirely.
- **NEVER write eval-response files** — evaluation integrity depends on separation between generator and evaluator. The main session is neither — it's the orchestrator. Writing eval responses breaks the trust model.

**If you are about to do any of the above: STOP. Send the instruction to the workspace agent via `executor.nudge()` instead.**

### Allowed Actions (from main session)

| Action                | How                                                                              |
| --------------------- | -------------------------------------------------------------------------------- |
| Read workspace status | Read `.feature-workspaces/<name>/.dev-workflow/signals/status.json`              |
| Trigger code review   | `executor.nudge(<ws>, "<trigger text>")` — per-mode transport table above        |
| Send feedback         | Write to `.feature-workspaces/<name>/.dev-workflow/signals/feedback.md`          |
| Nudge stuck agent     | `executor.nudge(<ws>, "<nudge text>")`                                           |
| Surface a human gate  | Read `signals/needs-human.md`; relay the human's answer via the mode's transport |
| Check PR state        | `gh pr view <number> --json state` (observe only — never act on merge)           |

### Forbidden Actions (from main session)

| Forbidden action     | Do this instead                                         |
| -------------------- | ------------------------------------------------------- |
| Read workspace code  | Trigger workspace's gen/eval via `executor.nudge()`     |
| Spawn review agents  | Send the review trigger via `executor.nudge()`          |
| Run tests            | Workspace handles its own test phases                   |
| Edit workspace files | Send instructions via `executor.nudge()` or feedback.md |
| Evaluate code        | Monitor eval-response files for results                 |
| Merge PRs            | Workspace agent merges via Phase 12                     |

### Two Gen/Eval Concerns — Strictly Separate

| Concern                    | Owner                    | What it evaluates                                    | Where it runs                           |
| -------------------------- | ------------------------ | ---------------------------------------------------- | --------------------------------------- |
| **Code quality**           | Workspace agent          | Code correctness, security, completeness             | Inside the workspace worker             |
| **Orchestration learning** | Autopilot (main session) | Patterns across workspaces: failures, costs, retries | Main session, feeds into `/aep-reflect` |

The autopilot does NOT evaluate workspace code. It triggers and monitors the workspace's own gen/eval loop. See `references/review-trigger.md` for detection logic and `references/orchestration-learning.md` for meta-learning.

---

## `/aep-autopilot` (default — start)

Initialize autopilot, run the first tick, and start the driver (goal-driven by default; `--loop` for the fixed-interval fallback). This is a single command — no second step needed.

**Usage:**

```
/aep-autopilot                  # default: goal driver — drives the current layer to completion, then stops
/aep-autopilot --floor 3m       # goal driver, custom per-tick wait floor (default 5m)
/aep-autopilot --loop 10m       # fixed-interval loop driver instead (custom interval) — the fallback
/aep-autopilot --loop 3m        # loop driver, faster for active development
```

**Driver selection rule:** the presence of `--loop <interval>` selects the
fixed-interval **loop driver**; its absence selects the **goal driver** (the
default). `--floor <dur>` only applies to the goal driver (the bounded per-tick
wait, default `5m`); `--max-turns <n>` caps the goal driver as a runaway
backstop (default `200`).

### Prerequisites

```bash
# 1. Must be on main workspace (not inside a feature workspace)
pwd | grep -q '.feature-workspaces' && echo "ABORT: Run from main workspace only" && exit 1

# 2. Product context must exist
[ -f product-context.yaml ] || echo "ABORT: Run /aep-envision and /aep-map first"

# 3. Autonomous routing must be enabled
# Check topology.routing.autonomous: true in product-context.yaml
```

Verify these conditions before proceeding:

- **Main workspace guard:** `pwd` must NOT contain `.feature-workspaces`
- **Product context exists:** `product-context.yaml` must exist with a `stories` section
- **Autonomous enabled:** `topology.routing.autonomous: true` must be set
- **Stories available:** At least one story must be `ready` or `in_progress`
- **Validated:** Product context should have passed `/aep-validate` (both passes)

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

4. Resolve the **launch mode + driver pair** (executor `detect()` + the driver ×
   backend matrix). If the mode is **claude-team**, create the standing team
   now: `TeamCreate` (team "aep") — once, for the whole autopilot run.
   Announce the pair, e.g. "claude-team + /loop" or "codex-exec + launchd".

5. Run the first tick immediately (see tick protocol below).

6. Start the driver. The default is the **goal driver**; `--loop` selects the
   fixed-interval **loop driver**. Both keep the orchestrator long-lived (so any
   steerable mode works); the goal driver additionally **self-terminates** when
   the layer is done. The tick body (the 7-step CHECK→ACT protocol below) is the
   same under either driver — only how the next tick is triggered differs.

   ### 6a. Goal driver (default)

   Build the **goal condition** for the current layer and hand it to the host's
   native `/goal` primitive. The condition is the success predicate the goal
   evaluator judges each turn — against the **status line the tick surfaces**
   (signals only — never workspace code) — plus a one-line per-turn directive:

   ```
   Layer <N> of this product is COMPLETE: every story in layer <N> is
   status=completed AND its worktree has been wrapped (none remain under
   .feature-workspaces/), as shown by the AUTOPILOT status line surfaced this
   turn — OR autopilot has entered status=paused requiring human input (design
   ambiguity, layer-gate failure, outcome contract, or repeated failure), as
   shown by the same status line. Judge ONLY from the surfaced status line,
   never from memory. Each turn, run exactly ONE `/aep-autopilot tick`, then end
   the turn; never run more than one tick per turn. Stop after <max-turns> turns
   if the layer has not completed.
   ```

   - **Claude Code (`/goal`, requires v2.1.139+):**

     ```
     /goal <the condition above>
     ```

     `/goal` starts a turn immediately and, after each turn, a small fast model
     (Haiku) checks the condition against the conversation; "no" auto-starts the
     next turn with the evaluator's reason as guidance, "yes" clears the goal and
     stops. Pair with auto mode so each turn runs unattended. The per-tick wait
     floor (step ⑦) is what prevents hot-looping — without it the evaluator
     re-fires the instant a turn ends. (The evaluator reads only the surfaced
     status line, so the orchestrator boundary holds: it never sees workspace
     code.)

   - **Codex (`goals` feature, experimental — enable with `--enable goals`):**

     ```
     /goal <the condition above>
     ```

     Set a `token_budget` as the hard runaway wall (on exhaustion Codex
     soft-stops to `budget_limited` with a wrap-up steer rather than dying).
     Codex continues the goal only when the thread is **idle**, the goal is
     active, and it is within budget; `/goal pause` · `/goal resume` ·
     `/goal check` · `/goal clear` manage it.

   Under either host, **keep delegating each tick's CHECK** to a cheap
   context-isolated agent (Haiku subagent / `codex exec` one-shot) — the goal
   session is long-lived, so the token-isolation reason from "Execution model"
   still holds.

   ### 6b. Loop driver (fallback — `--loop <interval>`)

   The fixed-interval driver, unchanged from earlier versions. Use it for hosts
   without `/goal`, for fully-unattended **OS-scheduled** runs (cron/launchd —
   `/goal` is in-session-only), or when you simply want a fixed cadence. The
   loop driver does not self-terminate; stop it with `/aep-autopilot stop`.

   **Claude Code — `/loop` (GA, in-session, long-lived):**

   ```
   /loop <interval> /aep-autopilot tick
   ```

   Where `<interval>` is from `--loop` flag (default: `5m`). `/loop` invokes
   `/aep-autopilot tick` each interval; the tick keeps the main session cheap by
   delegating its CHECK to a Haiku subagent. This is the driver
   that supports **claude-team** (the team lives in this session).

   **Codex — two driver options:**
   - **In-thread (long-lived):** the orchestrator is a living main thread
     (desktop app or interactive CLI) that runs `/aep-autopilot tick` on a cadence
     (self-paced or user-prompted). Supports **codex-subagent** — workers are
     this thread's subagents, steerable via `send_input`, visible as threads.
   - **Scheduled (ephemeral):** no native `/loop`; schedule `codex exec`
     externally — each tick is a fresh cheap one-shot (already
     context-isolated, so no nested CHECK needed). Workers must then be
     **codex-exec** (OS-bound; a fresh tick session steers them via
     `codex exec resume`). Recommended: a macOS `launchd` agent with
     `StartInterval=300` (or cron / a `while … sleep 300` loop) running:

     ```bash
     codex exec -m gpt-5.4-mini -c model_reasoning_effort=low \
       -C "$PWD" --skip-git-repo-check --dangerously-bypass-approvals-and-sandbox \
       "/aep-autopilot tick" < /dev/null    # < /dev/null: exec hangs on stdin otherwise
     ```

   Tell the user the exact scheduler snippet for their platform; AEP does not
   install it for them.

---

## `/aep-autopilot tick`

The per-tick handler invoked by the driver (goal or loop). Can also be run manually at any time. **Idempotent** — safe to run multiple times with no state change producing no duplicate actions.

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
  `executor.nudge()`, `wrap` via `/aep-wrap` (max one per tick), `launch` via `/aep-launch`
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
   - blocked_on == "human" OR needs-human.md has an unresolved entry → emit an
     `escalate` action (type human_gate) with the mode-specific answer recipe;
     do NOT count the workspace as stuck while gated
   - ORPHAN CHECK (session-bound modes): state says active but the agent is gone
     (TaskList / list_agents empty for it) → emit a `launch` action that re-spawns
     into the EXISTING worktree with the recovery bootstrap (re-adoption, not failure)

③ WRAP COMPLETED [ACT] → for each workspace where story_status == "completed":
   (CHECK emits a `wrap` action per completed workspace; orchestrator runs it)
   - Run /aep-wrap for this workspace (max ONE per tick — git operations serialize)
   - Remove workspace from state after wrap completes
   - Break to step ⑦

④ GUIDE COMPLETION [CHECK detect → ACT nudge] → for each workspace, guide toward quality and merge:
   (CHECK reads PR/eval state and decides; orchestrator sends the `nudge` actions)
   ALL NUDGE ACTIONS USE executor.nudge() — per-mode transport table above
   (SendMessage / feedback.md / send_input / exec resume / tmux send-keys).
   NEVER spawn code reviewers. NEVER call gh pr merge. Workspace agents own merging.

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
       - If no eval-response with PASS exists → trigger gen/eval via executor.nudge(<ws>):
           "Run Phase 5 code review now. Write eval-request.md, spawn the
            evaluator via executor.spawn_evaluator (your mode's recipe), and
            execute the gen/eval loop per the build skill Phase 5 protocol."
       - If stuck at Phase 5 (2+ ticks) → re-trigger via executor.nudge()
       - No response after 6 ticks (30 min) → add escalation
       - See references/review-trigger.md for full detection logic

   ④c. GUIDE TO MERGE — when eval PASSED AND pr_url set, nudge toward Phase 12:
       - Only nudge ONCE — skip if last_action is already "merge_nudged"
       - If eval PASSED but phase < 12 and not yet nudged → executor.nudge(<ws>):
           "Your code review eval has PASSED. Proceed to Phase 12: run pre-merge
            checks (rebase on the integration branch, verify CI, check comments) then merge the PR.
            In autopilot mode, merge when all checks pass without waiting for user
            confirmation."
       - If phase == 12 and stuck (2+ ticks) → executor.nudge(<ws>):
           "Complete Phase 12 merge now: 1) git fetch origin && git rebase origin/\"$(git config --get aep.integration-branch 2>/dev/null || (git show-ref --verify --quiet refs/remotes/origin/develop && echo develop || echo main))\" &&
            git push --force-with-lease origin feat/<name> 2) Verify CI green
            3) gh pr merge <number> --squash --delete-branch.
            Update status.json with story_status completed."
       - If phase == 12 and progressing → leave alone

⑤ DETECT STUCK [CHECK detect → ACT nudge] → for each workspace:
   - Compare (phase, completion_pct) with previous tick
   - No change → run executor.liveness() (mode table above), then increment consecutive_stuck_ticks
   - Changed → reset to 0
   - blocked_on == "human" → not stuck; it's a gate (handled in ②)
   - 6 ticks (30 min) stuck → executor.nudge() (claude-bg: this is the stop+respawn threshold)
   - 12 ticks (60 min) stuck → add escalation, consider pausing

⑥ DISPATCH NEW WORK [CHECK score → ACT launch] → if capacity available:
   - Read product-context.yaml, run dispatch scoring logic (steps 1-3 from /aep-dispatch)
   - available_slots = concurrency_limit - active_workspace_count
   - WAVE ORDERING: Dispatch Wave 1 before Wave 2 within each layer.
   - LAYER GATE: After completing all stories in a layer, check if a `.5` alignment
     layer exists for this layer. If yes, dispatch `.5` layer stories before
     advancing to the next integer layer.
     - Verify calibration artifacts exist before dispatching `.5` stories
       (check `calibration/<type>.yaml` or legacy `design-context.yaml`)
     - If missing → add escalation requesting the user to run `/aep-calibrate <type>`
   - OUTCOME CONTRACT: If layer has outcome_contract, pause for /aep-reflect evaluation
     before advancing to next layer.
   - GROUPED CHANGES: If top story has compile_mode: grouped_change, dispatch
     the entire change_group as one unit (one workspace, one PR).
   - For top-scored ready story (or group):
     - Route by readiness_score: >=0.7 → /aep-launch, <0.5 → escalate or auto-design
     - If auto_design: true → route through /aep-design automatically (no pause)
     - If auto_design: false and readiness < 0.7 → add escalation, PAUSE autopilot
     - If attempt_count >= 2 → always ESCALATE
     - If well-specified → run /aep-launch (max ONE launch per tick)
   - Add new workspace entry to state (with story_ids, wave, readiness_score)

⑦ WRITE STATE + SURFACE + WAIT [CHECK, then driver tail]
   - Write .dev-workflow/autopilot-state.json (atomic: write .tmp then rename)
   - Append tick summary to .dev-workflow/autopilot-history.jsonl
   - Update .dev-workflow/autopilot-status.md
   - Increment tick_count, set last_tick_at
   - Release tick lock
   - SURFACE the AUTOPILOT status line into the transcript (GOAL DRIVER ONLY) —
     signals-only, so the goal evaluator can judge "layer complete? paused?"
   - WAIT the per-tick floor before ending the turn (GOAL DRIVER ONLY) — the
     anti-hot-loop floor (default 5m, `--floor`). CC → Monitor with a hard
     timeout (a raw foreground sleep is blocked); Codex → shell sleep.
     Under the loop driver, neither the surface nor the wait runs (the `/loop`
     interval is the cadence).
```

---

### `/aep-autopilot status`

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

## `/aep-autopilot stop`

Gracefully stop the autopilot and cancel the active driver (goal or loop).

1. Set `status: "stopped"` in `.dev-workflow/autopilot-state.json`
2. Update `.dev-workflow/autopilot-status.md` with stopped state
3. Log stop event to `.dev-workflow/autopilot-history.jsonl`
4. Cancel the driver:
   - **Goal driver:** clear the active goal — `/goal clear` (Claude Code; aliases
     `stop`/`off`/`reset`/`none`/`cancel`) or `/goal clear` (Codex). No further
     turn re-fires. (The goal driver also self-clears when the layer completes,
     so `stop` is mainly for early termination.)
   - **Loop driver:** cancel the `/loop` (use the loop skill's cancel mechanism),
     or remove the cron/launchd job for an OS-scheduled run.
5. **claude-team only:** if no teammates are still building, shut them all down
   and `TeamDelete` the standing team. If workers are still mid-build, leave
   the team alive (deleting it would kill session-bound workers) and note in
   the status file that the team will be cleaned up after the last `/aep-wrap`.

**What happens:**

- The active driver is cancelled — no more ticks
- Running workspaces continue autonomously (they don't depend on autopilot)

**What does NOT happen:**

- Workspaces are NOT killed — they continue their `/aep-build` flow (caveat:
  session-bound workers do depend on the lead session staying open — closing
  the session orphans them; on the next `/aep-autopilot` start they are re-adopted
  per the orphan protocol)
- Product context is NOT modified
- No wraps or merges are triggered

```
Autopilot stopped. Active workspaces continue running independently.
To resume: /aep-autopilot
```

---

## Design Escalation

When autopilot encounters a story that needs design input, behavior depends on `topology.routing.auto_design`:

- **`auto_design: false` (default):** Autopilot **pauses entirely** — design decisions may affect other stories and require human judgment.
- **`auto_design: true`:** Autopilot routes the story through `/aep-design` automatically, then `/aep-launch`. No pause.

### Escalation Conditions (when `auto_design: false`)

A story triggers design escalation when ANY of:

1. **`readiness_score < 0.7`** — spec is not dispatch-ready (fewer than 3 acceptance criteria, missing interface obligations, unresolved open questions, etc.)
2. **`attempt_count >= 2`** — repeated failures suggest the spec is insufficient, not the implementation (always escalates, even with `auto_design: true`)

### Auto-Design Conditions (when `auto_design: true`)

Instead of pausing, autopilot:

1. Runs `/aep-design` for the story to refine the spec
2. Re-computes `readiness_score` after `/aep-design`
3. If readiness >= 0.7 → `/aep-launch`
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
     "expected_human_action": "Run /aep-design PROJ-010 to refine the spec...",
     "created_at": "<ISO8601>",
     "acknowledged": false
   }
   ```
3. Write `.dev-workflow/autopilot-status.md` with:
   - **Why paused** — the specific story and condition
   - **What needs human attention** — what's ambiguous, what decisions require human judgment
   - **Expected human feedback** — specific actions (run /aep-design, add acceptance criteria, etc.)
   - **Current state** — active workspaces still running, stories completed, what's blocked
   - **Detailed guidelines** — why this story can't be auto-designed (e.g., "UI layout decisions require visual design judgment that the agent cannot make autonomously")
4. Log to `.dev-workflow/autopilot-history.jsonl`

### Resuming After Pause

After the human resolves the design issue:

```
/aep-autopilot
```

This re-reads the product context (now with refined specs) and re-initializes
the driver — under the goal driver it sets a fresh goal for the current layer;
under the loop driver it restarts the `/loop` — then resumes ticking.

---

## Guardrails

- **Main workspace only** — refuse to run if `pwd` contains `.feature-workspaces`
- **All code operations happen inside workspace agents** — the main session NEVER reads, reviews, edits, or evaluates workspace code directly. It only sends instructions via `executor.nudge()` and reads signal files
- **Never spawn Agent tools for code review** — all reviews run inside the workspace worker, triggered via `executor.nudge()`. This is the #1 violation to watch for.
- **Never merge PRs** — workspace agents own Phase 12 merge; autopilot only detects already-merged PRs
- **Guide workspace agents to merge** — when eval passes and CI is green, nudge for Phase 12 completion via `executor.nudge()`; do not wait passively for workspace to figure it out
- **Never dispatch stories with unmet dependencies** — even under autonomous mode
- **Never treat SKIP-only test results as PASS** — at least 1 PASS required for test/integration stories
- **Never treat "no checks" as passing** — for integration/test stories, require at least one passing check OR explicit eval-response PASS
- **Never write eval-response files** — that's the workspace evaluator's job
- **One wrap per tick** — wraps involve git operations that must serialize
- **One launch per tick** — keeps tick duration under 60 seconds
- **Respect WIP limits** — never exceed `topology.routing.concurrency_limit`
- **Atomic state writes** — write to `.tmp` then rename to prevent corruption
- **Tick lock** — prevent overlapping ticks via `tick_in_progress` timestamp
- **Goal driver is scoped to ONE layer** — the goal condition completes (or
  pauses) at the current layer boundary; never widen it to the whole backlog
- **Goal evaluator sees signals only** — the per-tick surfaced status line MUST
  be signals-only (no workspace code, no file contents), preserving the
  orchestrator boundary; the evaluator never reads code
- **Per-tick wait floor is mandatory under the goal driver** — without it
  `/goal` re-fires the instant a turn ends and hot-loops, burning tokens
- **Always bound the goal** — `--max-turns` (default 200) and, on Codex, a
  `token_budget`, so a non-converging layer can never run forever
- **Pause on design ambiguity** — unless `auto_design: true`, escalate to human when readiness < 0.7
- **Always escalate on repeated failures** — `attempt_count >= 2` always pauses, even with `auto_design: true`

---

## Next Steps

After autopilot completes a layer or is stopped:

| Action                  | When                                                                              |
| ----------------------- | --------------------------------------------------------------------------------- |
| `/aep-reflect`          | After layer completes — evaluate outcome contracts (Step 2.75), classify feedback |
| `/aep-autopilot status` | Anytime — check progress and escalations                                          |
| `/aep-autopilot`        | After resolving a pause — resume the driver (re-sets the layer goal)              |
| `/aep-dispatch`         | Manual mode — pick a specific story interactively                                 |
