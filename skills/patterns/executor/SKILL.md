---
name: aep-executor
description: |-
  Host-agnostic executor for spawning and steering workspace agents across
  Claude Code and Codex backends. Consulted by /aep-launch, /aep-build,
  /aep-autopilot; invoke directly for "which backend", "launch mode",
  "host detection", running under Codex, with/without tmux (legacy),
  "as a workflow", or why "claude-team" was removed.
---

# Executor Abstraction

A reusable abstraction for **running implementation work in an isolated
workspace**, independent of which agent host (Claude Code, Codex) or which
mechanism (native background subagents, background sessions, native subagents,
exec workers, tmux, dynamic workflows) is available. Lifecycle skills speak one
vocabulary of operations; this skill maps each operation to a concrete recipe
per mode. It is consumed as a library by `/aep-launch`, `/aep-build`, and
`/aep-autopilot`, and can be invoked standalone to dry-run detection (see
[Standalone Usage](#standalone-usage)).

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

| Mode                   | Backend                                 | Lifetime      | Selected when                                                          |
| ---------------------- | --------------------------------------- | ------------- | ---------------------------------------------------------------------- |
| **native-bg-subagent** | Agent tool `run_in_background`, no team | session-bound | **Claude Code default** + long-lived orchestrator                      |
| **claude-bg**          | native background sessions              | OS-bound      | Claude Code, `claude --bg` present (cron driver / OS-bound need)       |
| **codex-subagent**     | native multi_agent (`spawn_agent`)      | session-bound | Codex with a living main thread (desktop app or interactive CLI)       |
| **codex-exec**         | headless `codex exec --cd` workers      | OS-bound      | Codex + cron driver, or hard isolation demanded                        |
| **legacy**             | tmux session (+ optional cmux tab)      | OS-bound      | explicit pin (`aep.executor-backend tmux`), or generic host w/ tmux    |
| **workflow**           | CC dynamic-workflow fan-out             | session-bound | explicit opt-in ("…with workflow") + Claude Code (see `/aep-workflow`) |
| **headless**           | one-shot native subagent                | session-bound | last resort                                                            |

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
   gates.

This does not spawn anything — it is a dry-run of `detect()`.

---

## Rationale

Why native-first (tmux demoted to a pinned `legacy` mode), why AEP still owns the
worktree, why session-bound vs OS-bound is a first-class axis, why human gates are
hub-and-spoke, and why autopilot drives only steerable modes are recorded in
`docs/decisions/native-first-executor.md`,
`docs/decisions/host-agnostic-executor.md`, and
`docs/decisions/remove-claude-team.md`.

---

## Next Step

After detecting/spawning, control returns to the calling skill:

- `/aep-launch` → the bootstrap was the spawn prompt (native modes) or sent over
  tmux (legacy), then `/aep-build` runs in the workspace
- `/aep-autopilot` → resumes its tick loop
- `/aep-dispatch` → completes the handoff
