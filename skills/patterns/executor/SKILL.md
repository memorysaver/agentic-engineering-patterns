---
name: aep-executor
description: |-
  Host-agnostic executor abstraction for spawning and steering implementation agents. This is a utility skill — it defines how /aep-launch, /aep-build, and /aep-autopilot start a worktree-bound agent, send it mid-flight instructions, check liveness, surface human gates, present it for review, and tear it down, across Claude Code (native background subagents / background sessions) or Codex (native subagents / exec workers), with tmux+cmux as the legacy pinned fallback. Reference its files from any skill that needs to run work in an isolated workspace. Triggers on "executor", "which backend", "launch mode", "spawn workspace", "run under codex", "native-bg-subagent", "agent teams", "claude-team", "no tmux", "with tmux", "run as a workflow", "host detection".
---

# Executor Abstraction

A reusable abstraction for **running implementation work in an isolated
workspace**, independent of which agent host (Claude Code, Codex) or which
mechanism (native background subagents, background sessions, native subagents,
exec workers, tmux, dynamic workflows) is available. Lifecycle skills speak one vocabulary of
operations; this skill maps each operation to a concrete recipe per mode.

**Native-first:** Claude Code launches use a native in-process background subagent
(`native-bg-subagent`, the default) or — where the `claude --bg` flag exists —
native background sessions (`claude-bg`); Codex launches use native subagents
(`codex-subagent`) or headless exec workers (`codex-exec`). tmux+cmux is the
**`legacy`** mode — selected only by explicit pin
(`git config aep.executor-backend tmux`) or on generic hosts. Every mode runs its
worker in an AEP-created git worktree at `.feature-workspaces/<ws>`.

> **`claude-team` removed (2026-06):** the agent-teams spawn path fails silently
> on Claude Code ≥ 2.1.x (truncated launch command in a detached tmux pane; roster
> still shows the worker "active"). Replaced by `native-bg-subagent` + a mandatory
> post-spawn liveness probe. See `docs/decisions/remove-claude-team.md`.

**This skill is both a utility library and a standalone skill:**

- **As a library:** `/aep-launch`, `/aep-build`, and `/aep-autopilot` reference its
  `references/` files for detection, mode selection, and per-operation recipes.
- **As a standalone skill:** Invoke directly to detect the current host and
  report which mode would be selected (useful when debugging "why did it pick
  X").

---

## Why This Exists

The control plane (`/aep-dispatch` scoring, the `.dev-workflow/signals/` protocol)
is host-independent. The coupling lived in the execution plane — historically a
`claude` process hosted in tmux, presented through cmux. This abstraction
isolates that coupling so the same workflow runs under Claude Code or the Codex
desktop app/CLI, using each host's **native** parallel-agent machinery, with
tmux as a pinned fallback rather than a default.

See [`docs/decisions/native-first-executor.md`](../../../docs/decisions/native-first-executor.md)
(and the earlier [`host-agnostic-executor.md`](../../../docs/decisions/host-agnostic-executor.md))
for the decision records.

---

## How Other Skills Use This

| Skill                | What it uses                                          | Operations                                     |
| -------------------- | ----------------------------------------------------- | ---------------------------------------------- |
| `/aep-launch`        | Start the implementation agent + expose it for review | `detect`, `spawn`, `present`                   |
| `/aep-build` Phase 5 | Spawn the evaluator in the right execution context    | `detect`, `spawn_evaluator`                    |
| `/aep-build`         | Raise a human decision mid-build                      | `gate`                                         |
| `/aep-autopilot`     | Run the periodic tick check cheaply; steer workspaces | `detect`, `check`, `nudge`, `liveness`, `gate` |
| `/aep-wrap`          | Tear down the worker + worktree after merge           | `teardown`                                     |
| `/aep-dispatch`      | Resolve the handoff mode; route "…with workflow" runs | `detect`                                       |

### Cross-skill reference path

After sync with the `aep-` prefix, the references are at:

```
.claude/skills/aep-executor/references/backends.md       # detection, selection, cross-mode protocols
.claude/skills/aep-executor/references/claude-native.md  # native-bg-subagent, claude-bg recipes
.claude/skills/aep-executor/references/codex-native.md   # codex-subagent, codex-exec recipes + role TOMLs
.claude/skills/aep-executor/references/tmux-session.md   # legacy recipes
```

Read `backends.md` first, then the recipe file for the selected mode.

---

## The Operation Contract

Every consumer speaks these verbs. The recipe files supply the implementation
per mode.

| Op                          | Purpose                                                                                                                                                     |
| --------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `detect()`                  | Resolve host + native capabilities + pin, select a mode                                                                                                     |
| `spawn(ws, branch, prompt)` | Start an implementation agent bound to the AEP worktree                                                                                                     |
| `spawn_evaluator(ws, role)` | Start an evaluator agent (worktree-bound) in the mode's eval context                                                                                        |
| `nudge(ws, msg)`            | Send a mid-flight instruction _(steerable modes; pull-based under claude-bg)_                                                                               |
| `liveness(ws)`              | Is the agent actively working? _(mode-specific signal + git-diff corroboration)_                                                                            |
| `gate(ws)`                  | Surface a worker's human decision: `needs-human.md` + the mode's transport, answered hub-and-spoke through the main agent (block-in-place or gate-and-park) |
| `check(prompt, schema)`     | Run a read-only analysis prompt in a **cheap, context-isolated** agent; return its JSON result — keeps a long-lived orchestrator session's context small    |
| `monitor(ws)`               | Read `.dev-workflow/signals/status.json` — **host-independent, never changes**                                                                              |
| `present(ws)`               | Human review surface (`TaskOutput` / `claude attach` / Codex thread / cmux tab / signals)                                                                   |
| `teardown(ws)`              | Worker + worktree cleanup                                                                                                                                   |

> **`monitor()` is already abstract.** Progress is reported through signal files
> at phase boundaries regardless of the executor. Native push channels
> (SendMessage, send_input) are an acceleration layer — the signal files remain
> the durable, host-agnostic source of truth.

---

## The Modes (summary)

| Mode                   | Backend                                 | Lifetime      | Selected when                                                       |
| ---------------------- | --------------------------------------- | ------------- | ------------------------------------------------------------------- |
| **native-bg-subagent** | Agent tool `run_in_background`, no team | session-bound | **Claude Code default** + long-lived orchestrator                   |
| **claude-bg**          | native background sessions              | OS-bound      | Claude Code, `claude --bg` present (cron driver / OS-bound need)    |
| **codex-subagent**     | native multi_agent (`spawn_agent`)      | session-bound | Codex with a living main thread (desktop app or interactive CLI)    |
| **codex-exec**         | headless `codex exec --cd` workers      | OS-bound      | Codex + cron driver, or hard isolation demanded                     |
| **legacy**             | tmux session (+ optional cmux tab)      | OS-bound      | explicit pin (`aep.executor-backend tmux`), or generic host w/ tmux |
| **workflow**           | CC dynamic-workflow fan-out             | session-bound | explicit opt-in ("…with workflow") + Claude Code                    |
| **headless**           | one-shot native subagent                | session-bound | last resort                                                         |

Read `references/backends.md` for the detection recipe, the full selection
order, the driver × backend compatibility matrix, the human-gate protocol, and
orphan re-adoption.

---

## Reference Files

| File                                                         | Contents                                                                                                    | When to read                                  |
| ------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------- | --------------------------------------------- |
| [`references/backends.md`](references/backends.md)           | Mode matrix, detection, selection order, driver compatibility, gate protocol, orphan re-adoption, `check()` | Always, before spawning or steering           |
| [`references/claude-native.md`](references/claude-native.md) | `native-bg-subagent` (default) + `claude-bg` recipes, `--bg` availability note                              | When the selected mode is a Claude native one |
| [`references/codex-native.md`](references/codex-native.md)   | `codex-subagent` + `codex-exec` recipes, `aep-builder`/`aep-evaluator` role TOMLs, desktop app mapping      | When the selected mode is a Codex one         |
| [`references/tmux-session.md`](references/tmux-session.md)   | `legacy` recipes (tmux spawn/nudge/liveness, cmux tab ladder)                                               | When `legacy` is pinned or selected           |

---

## Standalone Usage

Invoked directly, this skill reports what would happen:

1. Run the detection recipe from `references/backends.md`.
2. Print: host (claude/codex/generic), executor commands, native capabilities
   (`BG_AVAILABLE`, `MULTI_AGENT_AVAILABLE`), pin, tmux/cmux presence,
   orchestrator lifetime, and the **selected mode** with the reason.
3. If the user asked "why not workflow / why not tmux", explain the opt-in/pin
   gates. (There is no agent-teams mode — `claude-team` was removed; see
   `docs/decisions/remove-claude-team.md`.)

This does not spawn anything — it is a dry-run of `detect()`.

---

## Design Decisions

**Why native-first, tmux demoted:**

- Claude Code's native in-process background subagent (Agent tool,
  `run_in_background`, no team) gives each story its own context window,
  re-activation steering (`SendMessage(to: agentId)`), task-output visibility
  (`TaskOutput`), and auto-notify on completion — without tmux, cmux, or the
  agent-teams machinery (whose spawn path is broken; see `remove-claude-team.md`).
  Native background sessions (`claude --bg`/`attach`/`logs`/`stop`/`respawn`),
  where the flag exists, add an OS-bound option for cron drivers. Codex
  multi_agent gives push steering (`send_input`) and a native approval overlay in
  both the CLI and the desktop app.

**Why AEP still owns the worktree:**

- Host-managed worktrees pin their paths (`.claude/worktrees/`,
  `$CODEX_HOME/worktrees`) and hide them from the orchestrator's `monitor()`
  path. AEP's `git worktree add .feature-workspaces/<ws>` keeps the location
  stable and main-visible; native workers are pointed at it by process cwd
  (enforced) or prompt contract (no hooks — see backends.md).

**Why the single `legacy` pin exists (a narrow exception to "no pins"):**

- Detection can't distinguish "tmux is installed" from "the user wants the
  tmux+cmux workflow". Since native modes now outrank tmux on Claude Code, the
  users who _prefer_ cmux's clickable tabs need one explicit lever:
  `git config aep.executor-backend tmux` (or "…with tmux"). Everything else
  remains automatic.

**Why session-bound vs OS-bound is a first-class axis:**

- native-bg-subagents and Codex subagents die with their parent session; bg
  sessions, exec workers, and tmux sessions don't. An orchestrator's periodic
  driver (long-lived `/loop` vs cron one-shots) therefore constrains the mode —
  the compatibility matrix in `backends.md` makes that explicit, and orphan
  re-adoption (via the real-liveness probe, not roster membership) makes lead
  restarts non-fatal.

**Why human gates are hub-and-spoke (main agent as the console):**

- The human shouldn't have to chase worker surfaces. Every mode records the
  gate in `needs-human.md`; the question flows to the **main agent**, which
  asks the human and relays the answer. Steerable modes deliver the answer by
  push (**block-in-place**); batch/pull modes (`native-bg-subagent`, `workflow`,
  `headless`, `codex-exec`, `claude-bg`) use **gate-and-park** — the worker
  commits WIP, returns cleanly, and is resumed into the same worktree with the
  answer. Parking is cheap because all worker state lives in the worktree +
  `.dev-workflow/`, never only in agent context. Direct surfaces (`TaskOutput`,
  attach, threads) remain optional conveniences.

**Why autopilot needs a steerable, driver-compatible mode:**

- `nudge()` presupposes a worker you can reach mid-flight. `workflow` and
  `headless` collapse a build into one autonomous unit with no mid-stage
  surface — autopilot does not drive them (the workflow is its own
  orchestrator; gate-and-park still gives both a human-gate path through the
  main agent that launched them). All other modes are steerable —
  native-bg-subagent via `SendMessage(to: agentId)` + `feedback.md`, claude-bg
  degraded to pull-based nudging.

---

## Next Step

After detecting/spawning, control returns to the calling skill:

- `/aep-launch` → the bootstrap was the spawn prompt (native modes) or sent over
  tmux (legacy), then `/aep-build` runs in the workspace
- `/aep-autopilot` → resumes its tick loop
- `/aep-dispatch` → completes the handoff
