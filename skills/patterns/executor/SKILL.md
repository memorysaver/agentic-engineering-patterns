---
name: aep-executor
description: |-
  Host-agnostic executor abstraction for spawning and steering implementation agents. This is a utility skill — it defines how /launch, /build, and /autopilot start a worktree-bound agent, send it mid-flight instructions, check liveness, present it for human review, and tear it down, across Claude Code or Codex, with or without tmux/cmux, or (on explicit opt-in) as a Claude Code dynamic workflow. Reference its files from any skill that needs to run work in an isolated workspace. Triggers on "executor", "which backend", "spawn workspace", "run under codex", "no tmux", "cmux optional", "run as a workflow", "host detection".
---

# Executor Abstraction

A reusable abstraction for **running implementation work in an isolated
workspace**, independent of which agent host (Claude Code, Codex) or which
process/presentation tools (tmux, cmux, native subagents, dynamic workflows) are
available. Lifecycle skills speak one vocabulary of operations; this skill maps
each operation to a concrete recipe per backend.

**This skill is both a utility library and a standalone skill:**

- **As a library:** `/launch`, `/build`, and `/autopilot` reference its
  `references/backends.md` for detection, backend selection, and per-operation
  recipes.
- **As a standalone skill:** Invoke directly to detect the current host and
  report which backend would be selected (useful when debugging "why did it pick
  X").

---

## Why This Exists

The control plane (`/dispatch` scoring, the `.dev-workflow/signals/` protocol) is
already host-independent. The coupling lived in the execution plane — a
`claude --dangerously-skip-permissions` process, hosted in tmux, presented
through cmux. This abstraction isolates that coupling so the same workflow runs
under Claude Code or Codex, in a terminal or a Desktop app, with cmux as an
optional convenience rather than a hard dependency.

See [`docs/decisions/host-agnostic-executor.md`](../../../docs/decisions/host-agnostic-executor.md)
for the full decision record.

---

## How Other Skills Use This

| Skill            | What it uses                                             | Operations                             |
| ---------------- | -------------------------------------------------------- | -------------------------------------- |
| `/launch`        | Start the implementation agent + expose it for review    | `detect`, `spawn`, `present`           |
| `/build` Phase 5 | Spawn the evaluator in the right execution context       | `detect`, `spawn_evaluator`            |
| `/autopilot`     | Run the periodic tick check cheaply; steer workspaces    | `detect`, `check`, `nudge`, `liveness` |
| `/wrap`          | Tear down the session + worktree after merge             | `teardown`                             |
| `/dispatch`      | Resolve the handoff backend; route "…with workflow" runs | `detect`                               |

### Cross-skill reference path

After sync with the `aep-` prefix, the reference is at:

```
.claude/skills/aep-executor/references/backends.md
```

When a skill references this path, read it before spawning or steering any
workspace agent.

---

## The Operation Contract

Every consumer speaks these verbs. `references/backends.md` supplies the recipe
for each, per backend.

| Op                          | Purpose                                                                                                                                                  |
| --------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `detect()`                  | Resolve host + capabilities, select a backend                                                                                                            |
| `spawn(ws, branch, prompt)` | Start an implementation agent bound to a worktree                                                                                                        |
| `spawn_evaluator(ws, role)` | Start an evaluator agent (worktree-bound) in the backend's eval context                                                                                  |
| `nudge(ws, msg)`            | Send a mid-flight instruction _(session backends only)_                                                                                                  |
| `liveness(ws)`              | Is the agent actively working? _(session backends; git-diff fallback otherwise)_                                                                         |
| `check(prompt, schema)`     | Run a read-only analysis prompt in a **cheap, context-isolated** agent; return its JSON result — keeps a long-lived orchestrator session's context small |
| `monitor(ws)`               | Read `.dev-workflow/signals/status.json` — **host-independent, never changes**                                                                           |
| `present(ws)`               | Human review surface (cmux tab → tmux attach → headless)                                                                                                 |
| `teardown(ws)`              | Worktree/session cleanup                                                                                                                                 |

> **`monitor()` is already abstract.** Progress is reported through signal files
> at phase boundaries regardless of the executor. Consumers read signals exactly
> as they do today; only spawn/nudge/liveness/present/teardown vary by backend.

---

## The Four Backends (summary)

| ID     | Backend                                         | Selected when                                 |
| ------ | ----------------------------------------------- | --------------------------------------------- |
| **B1** | claude/codex session in tmux + cmux tab         | terminal host, tmux + cmux present            |
| **B2** | session in tmux, no cmux                        | tmux present, cmux absent                     |
| **B3** | native subagent (CC Task tool / Codex subagent) | no tmux (Desktop)                             |
| **B4** | dynamic-workflow fan-out, per-agent worktree    | explicit opt-in + Claude Code + Workflow tool |

Read `references/backends.md` for the detection recipe, the full per-operation
recipes, the fallback ladder, and the worktree-context constraint.

---

## Reference Files

| File                                               | Contents                                                                                                                                                                      | When to read                                    |
| -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------- |
| [`references/backends.md`](references/backends.md) | Detection recipe, backend selection table, per-operation recipes (spawn/nudge/liveness/present/teardown) for B1–B4, the worktree-context constraint, the cmux fallback ladder | Before spawning or steering any workspace agent |

---

## Standalone Usage

Invoked directly, this skill reports what would happen:

1. Run the detection recipe from `references/backends.md`.
2. Print: host (claude/codex/generic), executor binary, tmux present?, cmux
   present? (env vs installed), workflow-capable?, and the **selected backend**
   with the reason.
3. If the user asked "why not workflow", explain the opt-in + host gate.

This does not spawn anything — it is a dry-run of `detect()`.

---

## Design Decisions

**Why a utility skill, not just a reference file:**

- It is invocable for ad-hoc host detection / debugging backend selection.
- It appears in the skill list, making the abstraction discoverable.
- Its `references/` directory is path-accessible to launch/build/autopilot.

**Why backend selection is automatic (no flags/pins):**

- Detection is reliable from env markers (`$CLAUDECODE`, `$CODEX_*`,
  `$CMUX_SOCKET`, `$TMUX`) plus `command -v`. An override mechanism adds surface
  area for little gain.
- The single manual lever is the dynamic-workflow opt-in, expressed in natural
  language ("…with workflow"), consistent with the Workflow tool's own opt-in.

**Why autopilot only drives session backends:**

- `nudge()`/`liveness()` presuppose a _running session you can instruct_. B3/B4
  collapse a build into one autonomous unit with no mid-flight surface. Autopilot
  therefore requires B1/B2; the B4 workflow path is an alternative orchestrator,
  not a backend autopilot steers.

---

## Next Step

After detecting/spawning, control returns to the calling skill:

- `/launch` → sends the bootstrap prompt, then `/build` runs in the workspace
- `/autopilot` → resumes its tick loop
- `/dispatch` → completes the handoff
