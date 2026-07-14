---
name: aep-autopilot
description: >-
  Runs the dispatch-to-wrap cycle continuously for one layer from the main
  workspace. Use for "autopilot", "hands-free", or unattended multi-story
  execution; prefer /aep-dispatch for one story only.
---

# Autopilot

One command to go autonomous: initialize state, run the first tick, and keep
ticking until the current layer is complete. The default driver is
**goal-driven** (`/goal`, native on Claude Code and Codex) — each tick advances
the layer and the runtime re-fires only while a completion condition is unmet, so
autopilot **stops on its own** when every story in the **current layer** is
merged + wrapped, or a human-judgment gate is hit, then hands back for the layer
gate / `/aep-reflect` before you re-invoke it for the next layer. `--loop` selects
a fixed-interval fallback driver. Rationale:
`docs/decisions/goal-driven-autopilot.md`.

**Where this fits:** `/aep-envision → /aep-map → /aep-validate → /aep-autopilot
(goal: "layer N complete") → /aep-reflect`.

**Session:** Main session only (never from a feature workspace).
**State:** `.dev-workflow/autopilot-state.json` (machine-readable) +
`.dev-workflow/autopilot-status.md` (human-readable). Schema, atomic-write, and
tick-lock mechanics are canonical in `references/state-schema.md`.

---

## STOP — Orchestrator Boundaries

**Read this first; it overrides everything below.** You are an **orchestrator**,
not an executor. All code operations happen inside workspace agents. The main
session assesses progress **via signal files and git metadata only** and steers
workspaces through `executor.nudge()` — it never reads, reviews, edits,
evaluates, or merges workspace code. Violating this boundary is autopilot's most
common failure.

**Two hard prohibitions** (they cannot be phrased away — stated once, each paired
with its positive action):

- **Never read workspace source files** (no `Read`/`Grep`/`Bash` into a
  worktree) — **assess via signal files** (`.dev-workflow/signals/`) **and git
  metadata** (`gh pr view`, `git diff --stat`) only. Pulling implementation into
  the orchestrator's context makes it form code-quality opinions and act on them.
- **Never `gh pr merge` from the orchestrator** — **guide the workspace agent to
  merge.** The worker owns Phase 12 (rebase, CI, comment resolution) and **MUST**
  merge when its checks pass; "PR ready" is not a worker stop point. This binds
  MAIN only — never leak it into a worker prompt: telling a worker "do not merge"
  / "stop at ready" leaves a CLEAN, mergeable PR parked forever (the known
  `codex-subagent` shared-session failure). Every merge nudge says _"merge when
  Phase 12 passes."_

### What the orchestrator does (positive)

| Need                                         | Orchestrator action                                                                                                           |
| -------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Read workspace progress                      | Read `.feature-workspaces/<name>/.dev-workflow/signals/status.json`                                                           |
| Check PR state                               | `gh pr view <number> --json state` (observe only — never act on merge)                                                        |
| Trigger code review / gen-eval               | `executor.nudge(<ws>, "<trigger>")` — the workspace runs its **own** evaluator                                                |
| Send feedback / instructions                 | `executor.nudge()` or write `.feature-workspaces/<name>/.dev-workflow/signals/feedback.md`                                    |
| Nudge a stuck agent                          | `executor.nudge(<ws>, "<nudge>")`                                                                                             |
| Surface a human gate                         | Read `signals/needs-human.md`; relay the human's answer on the mode's channel                                                 |
| Get code fixed / tested / evaluated / merged | `executor.nudge()` — the workspace owns building, testing, eval, and Phase-12 merge; monitor `eval-response-*.md` for results |

The CHECK delegate (below) reading signals is **not** a boundary violation — it
offloads the orchestrator's own signal bookkeeping to a cheap context and never
reads code. Spawning a **code reviewer** from main stays categorically forbidden —
signals-only analysis ≠ code review, and "but I could give it the worktree" is not
a valid exception.

### Two gen/eval concerns — strictly separate

| Concern                    | Owner                    | Evaluates                                          | Runs                                                                       |
| -------------------------- | ------------------------ | -------------------------------------------------- | -------------------------------------------------------------------------- |
| **Code quality**           | Workspace agent          | correctness, security, completeness                | inside the workspace worker                                                |
| **Orchestration learning** | Autopilot (main session) | cross-workspace patterns: failures, costs, retries | main session → `/aep-reflect` (see `references/orchestration-learning.md`) |

Detection logic for _when_ a workspace's gen/eval must be triggered:
`references/review-trigger.md`.

### Executor transport

Every workspace action goes through three executor verbs:
`executor.nudge(ws, msg)` (deliver an instruction), `executor.liveness(ws)` (is
it really working — **process exists AND worktree active**, never roster/state
membership), and `executor.check(prompt, schema)` (cheap, context-isolated
read). Their per-mode transport (`SendMessage(to: agentId)` / `feedback.md` /
`send_input` / `codex exec resume` / `tmux send-keys`), the driver × backend
matrix, the post-spawn liveness probe, and the spawn / orphan-re-adoption recipes
are canonical in **/aep-executor** `references/backends.md` — consult it for the
mode you detected. Autopilot requires a **steerable, driver-compatible** mode; if
detection yields only workflow/headless (no mid-stage surface — that batch path
is the `/aep-dispatch … with workflow` orchestrator, not something autopilot
drives), report that autopilot needs a steerable mode and stop.

---

## `/aep-autopilot` — start

Initialize autopilot, run the first tick, and start the driver — one command, no
second step.

```
/aep-autopilot                  # goal driver (default) — drives the current layer to completion, then stops
/aep-autopilot --floor 3m       # goal driver, custom per-tick wait floor (default 5m)
/aep-autopilot --loop 10m       # fixed-interval loop driver (fallback), custom interval
/aep-autopilot status           # progress + escalations
/aep-autopilot stop             # gracefully stop the driver
```

**Driver selection:** the presence of `--loop <interval>` selects the loop
driver; its absence selects the goal driver (default). `--floor <dur>` applies
only to the goal driver (per-tick wait, default `5m`); `--max-turns <n>` caps the
goal driver as a runaway backstop (default `200`).

### Prerequisites

```bash
# 1. Must be on main workspace (not inside a feature workspace)
pwd | grep -q '.feature-workspaces' && echo "ABORT: Run from main workspace only" && exit 1

# 2. Product context must exist
[ -f product-context.yaml ] || echo "ABORT: Run /aep-envision and /aep-map first"

# 3. Autonomous routing must be enabled
# Check topology.routing.autonomous: true in product-context.yaml
```

Verify before proceeding:

- **Main workspace guard:** `pwd` must NOT contain `.feature-workspaces`
- **Product context exists:** `product-context.yaml` exists with a `stories` section
- **Autonomous enabled:** `topology.routing.autonomous: true` is set
- **Stories available:** at least one story is `ready` or `in_progress`
- **Validated:** product context has passed `/aep-validate` (both passes)

### Start Protocol

1. Create `.dev-workflow/` if absent:

   ```bash
   mkdir -p .dev-workflow
   ```

2. Initialize `.dev-workflow/autopilot-state.json` (full schema in
   `references/state-schema.md`):

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

3. Write initial `.dev-workflow/autopilot-status.md` (Status: Running, tick count
   0, no active workspaces yet — format in `references/state-schema.md`).

4. Resolve the **launch mode + driver pair** (executor `detect()` + the driver ×
   backend matrix). On Claude Code the default mode is **native-bg-subagent** (no
   team to create — if any agent-teams team is active, `TeamDelete` it first,
   since a live team poisons teamless background spawns). Announce the pair, e.g.
   "native-bg-subagent + /goal" or "codex-exec + launchd".

5. Run the first tick immediately (see `/aep-autopilot tick` below).

6. Start the driver per the selector below.

### Driver selector

Both drivers keep the orchestrator long-lived (so any steerable mode works); the
**goal driver additionally self-terminates** when the layer is done. The tick
body is identical under either driver — only how the next tick is triggered
differs. Full recipes (goal-condition text, per-host `/goal` / `/loop` commands,
Codex launchd `codex exec` snippet): `references/drivers.md`.

| Host / run                            | Driver                    | Start                                                                                                  |
| ------------------------------------- | ------------------------- | ------------------------------------------------------------------------------------------------------ |
| Claude Code / Codex, attended         | **goal** (default)        | `/aep-autopilot` sets `/goal "<layer-N condition>"`; re-fires each turn until layer complete or paused |
| Any host, fixed cadence or no `/goal` | **loop** (`--loop`)       | `/loop <interval> /aep-autopilot tick`                                                                 |
| OS-scheduled, unattended              | **loop** via cron/launchd | schedule `codex exec … "/aep-autopilot tick"` (OS-bound modes only)                                    |

**Driver × backend compatibility:** a long-lived driver (goal, or in-session
`/loop`) supports **any** steerable mode (session-bound workers live with the
orchestrator session). A cron/launchd **fresh-session-per-tick** run supports
**OS-bound modes only** (**claude-bg**, **codex-exec**, **legacy**) and only via
`/loop` or an external scheduler — `/goal` is in-session and cannot drive one.
**Goal-driver invariants:** scoped to ONE layer; the evaluator judges only the
**signals-only** surfaced status line (step ⑦); the per-tick wait floor is
mandatory (without it `/goal` hot-loops the instant a turn ends); and
`--max-turns` (plus a Codex `token_budget`) always bounds it.

---

## Strategic gates — `full_auto` and `auto_design`

Two `topology.routing` flags govern the **strategic** ("what to build" /
architecture) human gates; both default to keeping the human in control:

- **`auto_design`** (default `false`): a story in a non-dispatch-ready band, or
  one that hits the canonical repeated-attempt override, **pauses**
  for the human. `auto_design: true` instead routes it through `/aep-design`
  (non-interactive), re-computes readiness, then `/aep-launch` — no pause.
- **`full_auto`** (default `false`): the master switch **above** the finer-grained
  flags (`auto_design`, `auto_outcome_eval`, `watch.auto_create`) —
  `full_auto: true` **implies** all of them, so both the design-escalation pause
  and the qualitative outcome-contract evaluation auto-proceed via agent judgment.
  The default keeps every strategic pause with the human. The repeated-attempt
  override always escalates, even with these flags on.

Routing thresholds and the pause protocol live in the tick step that applies them
(`references/tick-protocol.md` Step ⑥); the escalation-entry and paused
`autopilot-status.md` shapes are in `references/state-schema.md`.

> **`full_auto` does not touch journey authoring.** The journey FILE is always a
> pre-merge build deliverable (`/aep-build` Phase 6 Step A authors it from the
> layer's acceptance criteria, committed with the feature). So under
> `full_auto: false` + `journey_timing: post-deploy` the journey must **already
> exist** before the post-deploy layer-gate handback — the handback is
> **EXECUTION + flip only, never authoring**. A layer reaching its gate with no
> journey file is a `/aep-wrap` COVERAGE FAILURE (the build should have authored
> it), not a human-authoring task at the gate.

---

## `/aep-autopilot tick`

The per-tick handler invoked by the driver (goal or loop); also runnable manually
at any time. **Idempotent** — re-running with no external state change produces no
duplicate actions.

**Execution model — CHECK → ACT.** Each tick is two halves:

- **CHECK (cheap, isolated):** `executor.check(prompt, schema)` — a Haiku
  subagent (Claude Code) or `codex exec` one-shot (Codex) — reads
  `autopilot-state.json` + every workspace `signals/` + `gh pr view` (**signals
  only, never workspace code**), computes transitions / stuck / dispatch capacity,
  **writes the updated `autopilot-state.json` + `autopilot-status.md`**, and
  returns the compact **action list**
  (`{summary, state_written, actions:[{type, workspace, story_id, message, reason}]}`
  — schema in /aep-executor `references/backends.md`). Token-heavy reading stays in
  the throwaway agent.
- **ACT (orchestrator):** execute the returned actions — `nudge`, `wrap` (max one
  per tick), `launch` (max one per tick), `escalate` / `design`. Few actions, so
  the main session stays cheap.
- On Codex the whole tick already runs as an isolated `codex exec`, so CHECK can
  run inline; on Claude Code the long-lived session is why CHECK is delegated.

**Tick summary (①–⑦, invariant per step).** Full step recipes — the CHECK prompt
content and the ACT templates (including the ④b/④c nudge prompt texts) — are
**canonical in `references/tick-protocol.md`**:

```
① READ STATE       cat autopilot-state.json; exit if status≠running or a tick lock is <4min old; else set the tick lock.
② SYNC SIGNALS     fold each workspace signals/status.json into state; blocked_on==human → escalate human_gate (NOT stuck); orphan (session-bound mode, by REAL liveness) → emit a re-adopt launch into the EXISTING worktree.
③ WRAP COMPLETED   story_status==completed → /aep-wrap (max ONE per tick, [one-wrap-per-tick]); remove from state; then skip to ⑦.
③.5 POST-MERGE GUARD  for each merged story, watch deploy health + host-aware dogfood across a window; UX finding → /aep-reflect story; confirmed hard regression → escalate (or revert if auto_revert). See references/post-merge-guard.md.
④ GUIDE COMPLETION per workspace: ④a check PR state; ④b quality gate — no eval PASS → trigger the workspace's OWN gen/eval via executor.nudge(); ④c eval PASS + PR open → nudge toward Phase-12 merge (once). NEVER spawn reviewers; NEVER gh pr merge.
⑤ DETECT STUCK     (phase, completion_pct) unchanged vs last tick → run executor.liveness() before counting; 6 ticks→nudge, 12→escalate; human-gate exempt.
⑥ DISPATCH NEW WORK  capacity? run /aep-dispatch scoring (steps 1-3); wave + layer-gate + grouped-change ordering; dispatch-ready → /aep-launch, otherwise escalate or auto-design; max ONE launch per tick ([one-launch-per-tick]); runs after ③ ([wrap-before-dispatch]).
⑦ WRITE STATE + SURFACE + WAIT  atomic write state (.tmp → rename); append autopilot-history.jsonl; update autopilot-status.md; release the tick lock. GOAL DRIVER ONLY: surface the signals-only AUTOPILOT status line, then wait the per-tick floor before ending the turn.
```

The `[bracketed]` tags are **mechanical ordering invariants** — deterministic
WHEN+SHAPE rules that drift under prose recall and hold behind typed gates. A
project scaling this loop should migrate them behind its own typed verbs:
`docs/decisions/deterministic-orchestration.md` (catalog in
`references/deterministic-orchestration.md`).

---

## `/aep-autopilot status`

Read and display current autopilot state.

```bash
cat .dev-workflow/autopilot-status.md
```

Also parse `.dev-workflow/autopilot-state.json` and present: **Status**
(running / paused / stopped); **Uptime** (since `started_at`, tick count);
**Active workspaces** (name, story_id, phase, completion_pct, last_action);
**Pending escalations** (type, story_id, reason); **Stats** (completed, failed,
total cost). **If paused:** why, what human feedback is expected, how to resume.

---

## `/aep-autopilot stop`

Gracefully stop autopilot and cancel the active driver.

1. Set `status: "stopped"` in `.dev-workflow/autopilot-state.json`.
2. Update `.dev-workflow/autopilot-status.md` with the stopped state.
3. Log the stop event to `.dev-workflow/autopilot-history.jsonl`.
4. Cancel the driver:
   - **Goal driver:** `/goal clear` (Claude Code / Codex). No further turn
     re-fires. (The goal driver also self-clears when the layer completes, so
     `stop` is mainly for early termination.)
   - **Loop driver:** cancel the `/loop` (loop skill's cancel mechanism), or
     remove the cron/launchd job for an OS-scheduled run.
5. **native-bg-subagent:** background subagents are session-bound and stop when
   the orchestrator session ends. To stop early while preserving work, let each
   in-flight worker reach a phase boundary (or `TaskStop <agentId>`); all state
   lives in the worktree + `.dev-workflow/`, so a later `/aep-autopilot` re-adopts
   the orphan via the liveness probe.

**Effect:** no more ticks; running workspaces continue autonomously (they don't
depend on autopilot — caveat: session-bound workers are orphaned when the lead
session closes and re-adopted on the next start). Product context is not
modified; no wraps or merges are triggered. To resume: `/aep-autopilot`.

---

## Guardrails

Rules with no natural step home (the ordering invariants, atomic write, and tick
lock live in `references/tick-protocol.md` / `references/state-schema.md`):

- **Require a real PASS before nudging a merge** — never treat SKIP-only test
  results or "no checks" as passing; for integration/test stories require at
  least one passing check OR an explicit eval-response PASS.
- **Respect WIP limits** — never exceed `topology.routing.concurrency_limit`.
- **Never dispatch a story with unmet dependencies** — even in autonomous mode.

---

## Next Steps

After autopilot completes a layer or is stopped:

| Action                  | When                                                                                |
| ----------------------- | ----------------------------------------------------------------------------------- |
| `/aep-reflect`          | After a layer completes — evaluate outcome contracts (Step 2.75), classify feedback |
| `/aep-autopilot status` | Anytime — check progress and escalations                                            |
| `/aep-autopilot`        | After resolving a pause — resume the driver (re-sets the layer goal)                |
| `/aep-dispatch`         | Manual mode — pick a specific story interactively                                   |
