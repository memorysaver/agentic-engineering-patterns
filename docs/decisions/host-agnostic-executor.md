# Host-Agnostic Executor Abstraction

> Decision record (2026-06-03).
> `/launch`, `/build`, and `/autopilot` were hardwired to a single execution
> path: a `claude --dangerously-skip-permissions` process, hosted in **tmux**,
> presented through **cmux**. This record introduces an executor abstraction so
> the same workflow runs under Claude Code or Codex, with or without tmux/cmux,
> and (on explicit opt-in) as a Claude Code dynamic workflow.

---

## 1. The Problem

The control plane was already tool-agnostic — `/dispatch` scores stories and
writes OpenSpec changes without caring what runs them, and the entire
inter-agent signals protocol (`.dev-workflow/signals/`) is file-based and
host-independent. The coupling lived entirely in the **execution plane**:

| Hardwired assumption                                                                                                      | Where                                                 |
| ------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------- |
| Executor is `claude --dangerously-skip-permissions --rc`                                                                  | `launch:99`, `build:356`, `gen-eval/eval-protocol:27` |
| Process host is **tmux** (`new-session`/`send-keys`/`split-window`/`capture-pane`)                                        | launch, build, autopilot (all ticks)                  |
| Review surface is **cmux** (`new-surface`/`send`/`rename-tab`) — and **required** in onboard's abort-on-missing tool gate | `launch:101–104`, `onboard:65`                        |

Consequences: the workflow could not run under Codex, could not run in Claude
Code Desktop / Codex Desktop (no attachable terminal multiplexer), and forced
cmux as a hard dependency even though nothing functional needs it.

Notably, `gen-eval/eval-protocol.md` **already** named three execution contexts
(A: tmux split spawning external `claude`; B: native Agent-tool subagents;
C: API/SDK). Launch and build only ever used Context A. The abstraction was
conceptualized but never wired into the lifecycle skills.

## 2. The Decision

Introduce a single **executor abstraction** — a new utility skill,
`aep-executor` — that every lifecycle skill consumes. It owns host/capability
detection and defines a uniform vocabulary of operations with a recipe per
backend. Backend selection is **fully automatic**; the dynamic-workflow backend
is the one exception, reached only on explicit user opt-in ("…with workflow")
when the host supports it.

This preserves the system's core value — **long-running, monitorable sessions
with a mid-flight feedback loop** — as the default, while degrading gracefully
where that's impossible and offering an autonomous fan-out where it's wanted.

### The operation contract

| Op                          | Purpose                                           | Host-independent?                                       |
| --------------------------- | ------------------------------------------------- | ------------------------------------------------------- |
| `detect()`                  | Resolve host + capabilities → select a backend    | recipe (env + `command -v`)                             |
| `spawn(ws, branch, prompt)` | Start an implementation agent bound to a worktree | backend-specific                                        |
| `nudge(ws, msg)`            | Send a mid-flight instruction                     | **session backends only**                               |
| `liveness(ws)`              | Is the agent actively working?                    | session backends; git-diff fallback is host-independent |
| `monitor(ws)`               | Read `.dev-workflow/signals/status.json`          | **already host-independent — unchanged**                |
| `present(ws)`               | Human review surface                              | backend-specific (cmux → tmux attach → headless)        |
| `teardown(ws)`              | Worktree/session cleanup                          | mostly host-independent                                 |

The pivotal realization: `monitor()` was already abstract. Progress is reported
through signal files at phase boundaries, independent of the executor. Only
spawn/nudge/liveness/present/teardown vary by backend, which is what makes this
refactor tractable rather than a rewrite.

### The four backends

| ID     | Backend                                                                           | Selected when                                        | Monitor                        | Mid-flight feedback          |
| ------ | --------------------------------------------------------------------------------- | ---------------------------------------------------- | ------------------------------ | ---------------------------- |
| **B1** | claude/codex session in tmux + cmux tab                                           | terminal host, tmux + cmux present _(prior default)_ | signals                        | yes (feedback.md + nudge)    |
| **B2** | session in tmux, **no cmux**                                                      | tmux present, cmux absent                            | signals                        | yes (`tmux attach` to watch) |
| **B3** | native subagent (CC Task tool / Codex subagent)                                   | **no tmux** (Desktop)                                | returned result + git/PR state | no (ephemeral)               |
| **B4** | dynamic-workflow fan-out (`pipeline(stories, build, verify)`, per-agent worktree) | **explicit opt-in** + Claude Code + Workflow tool    | `/workflows` view + signals    | no (no mid-run input)        |

B1 is byte-for-byte the prior behavior, so existing tmux+cmux installs see no
change. cmux is now purely the human's clickable live view — lose it and B2
keeps the full file-based monitor + nudge loop (`tmux attach` for a live view).
B3/B4 are the only backends that surrender mid-flight nudging, and B3 fires only
when there is genuinely no tmux.

### Detection recipe (grounded in real env markers)

```
HOST:      $CLAUDECODE set          → claude
           $CODEX_* (+ codex on PATH) → codex
           else                     → generic CLI (executor: ask/$AEP_EXECUTOR)
PRESENT:   $CMUX_SOCKET set          → cmux tab          (B1)
           else `command -v tmux`    → tmux session      (B2)
           else                      → native subagent   (B3)
WORKFLOW:  host==claude AND user opted in ("…with workflow") → B4 (overrides above)
```

Each host resolves **two** executor commands — interactive (for steerable B1/B2
sessions) and headless one-shot (for B3 / the evaluator / exec). Verified against
Claude Code 2.1.161 and Codex 0.130.0:

|            | interactive session (`$EXECUTOR`)                  | headless one-shot (`$EXECUTOR_EXEC`)                    |
| ---------- | -------------------------------------------------- | ------------------------------------------------------- |
| **claude** | `claude --dangerously-skip-permissions`            | `claude -p --dangerously-skip-permissions`              |
| **codex**  | `codex --dangerously-bypass-approvals-and-sandbox` | `codex exec --dangerously-bypass-approvals-and-sandbox` |

`--rc` was a bug (not a real Claude Code flag) and is removed. `codex exec` is
non-interactive, so the **session** backends use bare `codex` (its TUI); only the
headless paths use `codex exec`. The codex full-bypass flag is
`--dangerously-bypass-approvals-and-sandbox` (no `--yolo`/`--full-auto`).

`$CMUX_*` env vars are present when the session is hosted _inside_ a cmux
surface — a stronger signal than `command -v cmux` (which only proves it's
installed). `$TMUX` indicates we're already inside a tmux session.

## 3. Why This Shape

- **Shared reference, not inline per skill.** Host detection + backend recipes
  live in one place (`aep-executor/references/backends.md`). Launch, build, and
  autopilot link to it the way `/build` already links to
  `aep-gen-eval/references/`. No logic duplicated across three skills, no drift.
- **Sessions stay the default.** The monitor/feedback loop is the heart of the
  system and consistent with the orchestrator boundary (the main session never
  inspects workspace code; it only delegates and reads signals). The abstraction
  makes "delegate, don't inspect" the _only_ available verb.
- **Workflow is opt-in, not default.** Dynamic workflows are billed, run in the
  background, and accept no mid-run human input — the wrong default for
  interactive development, the right tool for hands-free batch. Gating it on
  explicit intent matches how the Workflow tool itself requires opt-in.
- **Autopilot is a session-backend orchestrator.** Its tick/nudge model only
  makes sense for B1/B2 (a running session you can instruct). B4 is an
  _alternative_ autonomous orchestrator that replaces the tick loop for batch
  runs, not a backend autopilot drives.

## 4. How — Per-Skill Changes

| Skill                    | Change                                                                                                                                           | Untouched                                                                                      |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------- |
| **`aep-executor`** (new) | Owns detect/spawn/nudge/liveness/present/teardown + the 4 backends + detection recipe + the worktree-context constraint                          | —                                                                                              |
| **launch**               | tmux/cmux/claude spawn + bootstrap → `executor.spawn()` + `executor.present()` (B1/B2 recipe shown inline, cmux conditional)                     | clean-worktree / push / calibration / orphan-cleanup guardrails; evaluator-criteria brainstorm |
| **build**                | Phase 5 `tmux split-window … claude` → `executor.spawn_evaluator()`, picking eval-protocol Context A/B/C by backend                              | all 13 phases, signals, verification JSON                                                      |
| **autopilot**            | `tmux send-keys`/`capture-pane` reframed as the **B1/B2 implementation** of `executor.nudge()`/`executor.liveness()`; requires a session backend | orchestrator boundary, tick protocol, escalation                                               |
| **dispatch**             | handoff resolves a backend; detects "…with workflow" → routes the wave through B4                                                                | all scoring, the dispatch lock, context assembly                                               |
| **gen-eval**             | note that the chosen execution Context tracks the executor backend                                                                               | scoring framework, contracts                                                                   |
| **onboard**              | cmux moved out of the abort-on-missing gate → optional/recommended; `codex` recognized as an executor; tmux still recommended                    | rest of onboarding                                                                             |

### Constraint baked into the executor reference

B3/B4 spawn their agents **bound to the worktree** (`isolation: worktree` or
cwd = workspace dir). This is mandatory: it gives the spawned agent the
workspace-local context that autopilot's "never spawn an Agent reviewer from
main" rule was protecting against. The gen/eval separation and orchestrator
boundary hold under every backend — only the spawn mechanism differs.

## 5. Versioning

- `marketplace.json`: **1.1.0 → 1.2.0**. Additive and backward-compatible — B1
  reproduces today's behavior exactly.
- New `aep-executor` skill joins the **`patterns`** plugin group.
- README skill catalog + install table updated. npx-pinned downstream projects
  upgrade explicitly (`npx skills update`), which is the point of pinning.

## 6. Alternatives Considered

- **Native subagent as the default executor.** Rejected: ephemeral subagents run
  to completion with no monitor or mid-flight feedback — it would discard the
  system's core loop. Kept as the B3 Desktop fallback only.
- **Inline abstraction per skill.** Rejected: triplicates host-detection/spawn
  logic across launch/build/autopilot with guaranteed drift.
- **Re-implement `/dispatch` as a workflow.** Rejected: dispatch is single-pass
  interactive scoring; wrapping it in a billed background workflow adds latency
  and removes the interactivity that is its purpose. The workflow path applies to
  the autonomous _build fan-out_, not to story scoring.
- **Runtime backend override flags / `.aep` pin.** Rejected: detection is
  reliable from env markers, and an escape hatch adds surface area for little
  gain. Workflow opt-in via natural language is the only manual lever.
