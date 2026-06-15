# Native-First Executor Backends

> **Superseded in part (2026-06):** `claude-team` was removed (silent agent-teams spawn failure on Claude Code ≥ 2.1.x) and replaced by `native-bg-subagent` as the Claude Code default, with a mandatory post-spawn liveness probe. See [`remove-claude-team.md`](./remove-claude-team.md).

> Decision record (2026-06-10). Supersedes parts of
> [`host-agnostic-executor.md`](host-agnostic-executor.md): the B1–B4 backend
> ladder is replaced by named launch modes, and tmux+cmux is demoted from
> "default for Claude Code" to an explicitly-pinned legacy mode. The executor
> abstraction itself (the operation vocabulary, the signals protocol, AEP-owned
> worktrees) is unchanged — this record re-targets its recipes at the hosts'
> native parallel-agent machinery.

---

## 1. The Problem

The v1.2–v1.5 backend ladder (B1 tmux+cmux, B2 tmux, B3 native subagent,
B4 workflow) was designed when "a steerable worker with its own context window
in our worktree" required an external terminal multiplexer. By mid-2026 both
hosts ship that capability natively:

- **Claude Code** has **agent teams** (experimental: teammate = a separate
  Claude instance with its own context window, `SendMessage` push steering,
  `teammateMode` display, teammate→lead messaging) and **native background
  sessions** (`claude --bg` / `agents` / `attach` / `logs` / `stop` /
  `respawn` — functionally the tmux verb set, GA).
- **Codex** (CLI 0.130+ and the desktop app, same Rust runtime) has stable
  **multi_agent** subagents (`spawn_agent` / `send_input` / `wait_agent` /
  `list_agents` / `close_agent`), custom roles via `.codex/agents/*.toml`
  (project-scoped), the `/agent` thread switcher, and a native approval
  overlay that surfaces a subagent's permission requests in the active
  thread/app UI.

Meanwhile the old B3 had no mid-flight steering and no human gate, so Codex
users lost autopilot; and Claude Code auto-selected tmux whenever it was
installed, forcing external tooling on users whose host no longer needed it.

## 2. Key Research Findings (2026-06, verified against live docs)

1. **Codex `spawn_agent` has no cwd/worktree parameter.** Subagents share the
   parent's workspace and sandbox. The Codex app's own Worktrees feature pins
   paths under `$CODEX_HOME/worktrees` (not configurable). So binding a Codex
   subagent to `.feature-workspaces/<ws>` is a **directory contract** (prompt +
   role `developer_instructions`), not enforcement — but the contract stays
   inside the `workspace-write` sandbox because the worktree (and its
   `.git/worktrees/...` metadata) lives under the project root. Hard
   enforcement exists only via `codex exec --cd <worktree>`.
2. **Agent teams is experimental, not GA** (`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS`,
   Claude Code ≥ 2.1.32; enableable per-project via the settings.json `env`
   block). Constraints: one team at a time; team bound to the lead session
   (`/resume` does not restore in-process teammates); `TeamDelete` requires all
   teammates shut down. But teammates **can** be spawned and shut down
   individually at any time — which makes a long-running orchestrator viable.
3. **Background Claude Code subagents auto-deny permission prompts** — a pure
   `background: true` subagent has no human gate. Teams (teammate→lead
   message) and attachable bg sessions do.
4. **Worker lifetime splits into two classes.** _Session-bound_: teammates and
   Codex subagents die with the lead session/thread. _OS-bound_: `claude --bg`
   sessions, `codex exec` processes, and tmux sessions survive it (and
   `codex exec resume <id>` steers a worker from a _different, later_ session).

## 3. The Decision

### Named launch modes, native first

| Mode                    | Mechanism                          | Lifetime      |
| ----------------------- | ---------------------------------- | ------------- |
| `claude-team`           | agent teams, teammate per story    | session-bound |
| `claude-bg`             | native background sessions         | OS-bound      |
| `codex-subagent`        | multi_agent + `aep-builder` role   | session-bound |
| `codex-exec`            | headless `codex exec --cd` workers | OS-bound      |
| `legacy`                | tmux (+ cmux tab)                  | OS-bound      |
| `workflow` / `headless` | unchanged from B4 / one-shot B3    | session-bound |

Selection: natural-language opt-in (`workflow`) → explicit pin (`legacy`) →
native by host capability → tmux only for generic hosts → headless. **Claude
Code with tmux installed no longer auto-selects tmux** — the one behavior
change; cmux fans pin with `git config aep.executor-backend tmux` (a narrow,
deliberate exception to the "no pins" stance of the previous ADR).

### AEP keeps worktree ownership

Every mode points its worker at the AEP-created
`.feature-workspaces/<ws>` worktree. Host-managed worktrees would break the
stable path that `monitor()`, dispatch signal-sync, and `/aep-wrap` depend on.
Binding is by process cwd where possible (claude-bg, codex-exec, legacy,
evaluator execs) and by **prompt contract** otherwise (claude-team,
codex-subagent) — per the project's no-hooks decision (hooks have poor
cross-platform reach; model capability + skill instructions carry the
contract). No WorktreeCreate hook, no `isolation: worktree` redirection.

### The standing team (claude-team × long-running orchestration)

One team for the whole autopilot run: `TeamCreate` at start, spawn one
teammate per dispatched story (tick ⑥), shut that teammate down at wrap
(tick ③), `TeamDelete` at stop. This fits inside "one team at a time" while
allowing incremental grow/shrink. Because the team is session-bound, the
**driver matters**: `/loop` (long-lived lead) supports claude-team; a
cron/launchd driver (fresh session per tick) does not — it requires OS-bound
modes (claude-bg, codex-exec, legacy). The same logic gives Codex two
autopilot shapes: in-thread ticks → codex-subagent; scheduled `codex exec`
ticks → codex-exec.

### Host-agnostic human gate: hub-and-spoke, two styles

New signal: `.dev-workflow/signals/needs-human.md` + `"blocked_on": "human"`
in `status.json` — the durable record on every mode. The **main agent is the
canonical human console** (hub-and-spoke): the worker's question flows back to
the orchestrator, which asks the human in the main session and relays the
answer; visiting a worker's surface (teammate pane, `claude attach`, Codex
thread, `tmux attach`) is an optional convenience, never required. Two styles:

- **Block-in-place** (steerable modes — claude-team, codex-subagent, legacy):
  the worker waits; the answer arrives by push (`SendMessage` / `send_input` /
  nudge).
- **Gate-and-park** (batch/pull modes — workflow, headless, codex-exec,
  claude-bg): no push channel reaches a running worker, so the worker commits
  WIP, records the gate, and **ends its run cleanly**; the orchestrator resumes
  a worker into the same worktree with the answer (the orphan re-adoption
  recipe, with the answer prepended). Parking is cheap because all worker state
  lives in the worktree + `.dev-workflow/`.

Gated workspaces are _waiting_, not stuck and not failed; autopilot surfaces
them as `human_gate` escalations answered in the main session.

### Dynamic workflow as a complete backend

Gate-and-park is what elevates **workflow mode** (the Claude Code
dynamic-workflow fan-out) from fire-and-forget batch to a complete backend:
each build agent is prompted to return a structured `gated` result (mirroring
`needs-human.md`) instead of guessing or stalling; the main agent collects
gated stories after the run, asks the human, and resumes them into their
existing worktrees (continuation workflow via `resumeFromRunId`, or individual
re-launches). Mid-stage push steering still doesn't exist — steering happens
at stage boundaries and through gates — and autopilot still doesn't drive
workflow mode (it is its own orchestrator).

### Orphan re-adoption

A lead restart orphans session-bound workers but not their work (worktree +
signals + commits). Orchestrators treat "state says active, agent list says
gone" as re-adoption: re-spawn into the existing worktree with the recovery
bootstrap (`bash .dev-workflow/init.sh` path), update `agent_id`, continue.

### Gen-eval in the worktree

The evaluator remains a bounded, worktree-cwd, generator-spawned agent.
claude-team/claude-bg: a **foreground Task subagent** (permission prompts pass
through; the prompt is the spawn). codex modes: `codex exec --cd <worktree>`
with the `aep-evaluator` role (enforced cwd). legacy: the tmux split, as
before. Request/response files and convergence rules unchanged.

## 4. What Did Not Change

The operation vocabulary (`detect/spawn/spawn_evaluator/nudge/liveness/check/
monitor/present/teardown`, plus new `gate`); the signals protocol; `$BASE`
resolution; launch guardrails; one-commit-per-task; the orchestrator boundary
(main never reads workspace code); CHECK-in-cheap-delegate. Downstream projects
migrate by re-pinning skills — old environments without any native capability
land exactly where v1.5.0 left them (legacy/headless).

## 5. Rejected Alternatives

- **Host-managed worktrees** (`isolation: worktree`, Codex app Worktrees):
  paths not ours, invisible to `monitor()`, and (Codex) not available to
  subagents at all.
- **WorktreeCreate hook to enforce teammate isolation at our path:** works,
  but hooks are the least portable layer across hosts and the user explicitly
  chose model-capability-over-hooks.
- **One team per story:** violates "one team at a time" and forces destructive
  TeamDelete churn between stories.
- **`codex exec --cd` as the Codex default:** hard isolation, but headless —
  loses the approval overlay, thread UI, and push steering that make the
  desktop app experience native. Kept as the cron-driver / hard-isolation mode.
