# Claude Code Native Backends — `native-bg-subagent` & `claude-bg`

Per-operation recipes for the two Claude Code native modes. Both replace tmux —
and the **removed `claude-team`** — with capabilities built into Claude Code;
neither requires tmux, cmux, agent teams, or any hook. Detection and selection
live in `backends.md` — read that first, including the mandatory
[Post-Spawn Liveness Probe](backends.md#post-spawn-liveness-probe).

> **`claude-team` was removed (2026-06).** On Claude Code ≥ 2.1.x the agent-teams
> spawn path fails **silently** — the launch command is truncated in a detached
> `claude-swarm-<pid>` tmux pane and never submitted, so no worker starts, yet the
> team roster still reports the member "active". A live team also **poisons
> teamless background spawns** (they re-route through the same broken backend).
> `native-bg-subagent` is the replacement default. See
> `docs/decisions/remove-claude-team.md`.

| Mode                   | Mechanism                                              | Lifetime                                   | Steering                                   | Human gate                                           |
| ---------------------- | ------------------------------------------------------ | ------------------------------------------ | ------------------------------------------ | ---------------------------------------------------- |
| **native-bg-subagent** | Agent tool `run_in_background`, no team                | session-bound (dies with the orchestrator) | `SendMessage(to: agentId)` / `feedback.md` | gate-and-park → main agent re-spawns w/ answer       |
| **claude-bg**          | native background sessions (`claude --bg`, if present) | OS-bound (survives the lead session)       | `feedback.md` (pull) + stop/respawn        | gate-and-park → main agent relays via session resume |

---

## Mode: `native-bg-subagent` (Claude Code default)

A **native in-process background subagent**: the Agent tool with
`run_in_background: true`, **no `team_name`**, spawned while **no team is active**.
It runs asynchronously in the orchestrator session, is non-blocking, and
auto-notifies on completion. This is the Claude Code default — it needs no tmux,
no agent-teams flag, and no `--bg` CLI flag.

### Success signature (how to know the spawn actually worked)

A working spawn returns a **bare-hex `agentId`** (e.g. `adfb6cb206155a92e`) with a
JSONL `output_file` — **not** an `@<team>` id. Record that `agentId` as the
workspace `agent_id`. Then **run the [Post-Spawn Liveness Probe](backends.md#post-spawn-liveness-probe)**
before declaring the worker running.

### Prerequisite: no active team

```
# A live agent-teams team re-routes EVEN teamless background spawns through the
# broken agent-teams tmux backend. If a team exists, shut its members down and
# TeamDelete it BEFORE spawning.
list_agents / TaskList → if a team "aep" exists: shutdown members, then TeamDelete
```

### `spawn(ws, branch, bootstrap_prompt)`

The worktree is created by AEP first (common recipe in `backends.md`). Then spawn
a background subagent with the Agent tool — **no `team_name`**:

```
Agent tool:
  run_in_background: true
  # NO team_name — a team (active or newly created) routes through the broken backend
  prompt: |
    You operate EXCLUSIVELY in <abs-repo-path>/.feature-workspaces/<ws>
    on branch feat/<ws>. cd there first; never edit files outside it.
    <bootstrap_prompt>                # the /aep-build bootstrap, incl. Prior Lessons
    Report progress through .dev-workflow/signals/status.json at phase
    boundaries. If you hit a decision only the human can make, follow the
    human-gate protocol: append to .dev-workflow/signals/needs-human.md,
    set "blocked_on": "human" in status.json, commit WIP, and end your run.
```

Capture the returned **bare-hex `agentId`** + `output_file` → state `agent_id`.
The worktree binding is a **prompt contract** (AEP's no-hooks decision). Then run
the liveness probe; on failure follow the re-dispatch-on-failure contract (since
this mode is itself the terminal fallback, a probe failure here escalates).

### `nudge(ws, msg)`

```
SendMessage(to: <agentId>, message: <msg>)   # continues the background subagent
```

`SendMessage` re-activates a previously spawned background subagent with the
message. Always **also** append the same text to `signals/feedback.md` (the file
is the durable, host-agnostic record the worker reads at phase boundaries). If the
agent is unreachable / already exited, fall back to the hard-stuck path below.

**Hard-stuck** (no progress ≥ 6 ticks): `TaskStop <agentId>`, then re-spawn into
the worktree with a recovery prompt — the worktree and `.dev-workflow/` carry all
state:

```
Agent tool: run_in_background: true, no team_name, prompt:
  "Run bash .dev-workflow/init.sh to recover state, read
   .dev-workflow/signals/feedback.md, then continue the /aep-build flow from the
   current phase."   # record the NEW agentId in state
```

### `liveness(ws)`

```
TaskList                                     # is <agentId> still running?
git -C .feature-workspaces/<ws> diff --stat  # host-independent corroboration
```

Apply the [Post-Spawn Liveness Probe](backends.md#post-spawn-liveness-probe):
the agent must exist in `TaskList` AND the worktree must show activity. **Never**
accept "state says active" as liveness. A `TaskList`-absent agent with worktree
progress is an **orphan** (session restarted) → re-adopt per `backends.md`.

### `present(ws)`

```
TaskOutput <agentId>     # the JSONL output_file — recent worker output
```

Plus `signals/status.json` + the PR. There is no live pane; this mode is a
background worker surfaced through its task output and signals.

### `gate(ws)` — human gate (gate-and-park)

There is no guaranteed push channel into a mid-run background subagent, so the
worker **parks**: append to `needs-human.md` + `blocked_on: human`, commit WIP,
end the run cleanly. The orchestrator detects the gate on its next tick, asks the
human in the **main session** (hub-and-spoke), and **re-spawns a bg subagent into
the same worktree** with the answer + recovery bootstrap:

```
Agent tool: run_in_background: true, no team_name, prompt:
  "The human decided: <answer>. Run bash .dev-workflow/init.sh to recover state,
   mark the needs-human entry resolved, clear blocked_on, and continue the
   /aep-build flow."   # record the new agentId
```

### `spawn_evaluator(ws, role)`

The background subagent spawns a **foreground Task subagent** in its own context
(one level — a subagent may spawn one subagent) with the evaluator prompt; it
inherits the worktree via the prompt contract. The evaluator prompt (from
`aep-gen-eval/references/agent-contracts.md`) **is** the spawn prompt. Poll for
`eval-response-<N>.md` as a sanity check.

### `teardown(ws)`

```
TaskStop <agentId> 2>/dev/null || true    # stop the bg subagent if still running
# then the common worktree removal from backends.md
```

---

## Mode: `claude-bg`

One **native background session per story**. `claude --bg` starts a full Claude
instance detached from the current session; it is **OS-bound** — it survives lead
restarts and is attachable from any terminal. This is the only Claude Code mode
that survives a cron/launchd fresh-session-per-tick driver (see the driver ×
backend matrix in `backends.md`).

> **`--bg` availability (verify per build).** On Claude Code ≥ 2.1.x the one-shot
> `claude --bg` spawn flag was **removed** — `claude agents` is now an interactive
> _agent view_, not a scriptable one-shot spawn. The capability probe below gates
> this mode: when `--bg` is absent, `BG_AVAILABLE=no` and detection skips
> claude-bg, leaving **native-bg-subagent** (session-bound) as the Claude Code
> default. If a build re-introduces a scriptable background-spawn flag, update the
> spawn recipe here accordingly.

### Capability probe

```bash
claude --help 2>/dev/null | grep -q -- '--bg' && echo "claude-bg available"
```

### `spawn(ws, branch, bootstrap_prompt)`

The process cwd **is** the isolation — this mode hard-binds the worker to the
worktree, no prompt contract needed:

```bash
cd .feature-workspaces/<ws> && claude --bg --dangerously-skip-permissions "<bootstrap_prompt>"
# capture the printed session id → state agent_id; then run the liveness probe
cd - >/dev/null
```

Record the session id in orchestrator state (`agent_id`). List/inspect at any
time:

```bash
claude agents --json          # all background sessions + status
claude logs <id> | tail -40   # recent output
```

### `nudge(ws, msg)` — degraded (pull)

Background sessions take no push input mid-turn. Two-tier nudge:

1. **Normal:** append to `signals/feedback.md` (workers read it at phase
   boundaries — the existing protocol).
2. **Hard-stuck** (no progress ≥ 6 ticks): stop and respawn with a recovery
   prompt — the worktree and `.dev-workflow/` carry all state:

   ```bash
   claude stop <id>
   cd .feature-workspaces/<ws> && claude --bg --dangerously-skip-permissions \
     "Run bash .dev-workflow/init.sh to recover state, read .dev-workflow/signals/feedback.md, then continue the /aep-build flow from the current phase."
   cd - >/dev/null   # record the NEW session id in state
   ```

### `liveness(ws)`

```bash
claude agents --json | jq '.[] | select(.id=="<id>")'   # running / exited
claude logs <id> | tail -5                              # output still moving?
git -C .feature-workspaces/<ws> diff --stat             # corroboration
```

Apply the [Post-Spawn Liveness Probe](backends.md#post-spawn-liveness-probe):
process exists AND worktree shows activity — never roster/state alone.

### `present(ws)`

```bash
claude attach <id>     # interactive attach — the native replacement for tmux attach
```

### `gate(ws)` — human gate (gate-and-park)

There is no push channel into a running bg session, so the worker **parks**:
append to `needs-human.md` + `blocked_on: human`, commit WIP, end the run
cleanly. The orchestrator detects the gate on its next tick, asks the human in
the **main session** (hub-and-spoke — the human does not need to attach), and
relays the answer by resuming the worker:

```bash
# Resume the same session with the answer (preferred — context intact):
claude -r <agent_id> --bg --dangerously-skip-permissions \
  "The human decided: <answer>. Mark the needs-human entry resolved, clear blocked_on, and continue the /aep-build flow."
# Fallback (session not resumable): respawn in the worktree with the recovery bootstrap + answer.
# Record the (new) session id as agent_id.
```

Optional direct surface: while the worker is running, `claude attach <id>`
also works (a blocking permission prompt holds the session and attach surfaces
it) — a convenience, not the protocol.

### `spawn_evaluator(ws, role)`

The bg session is a full Claude instance running in the worktree — it spawns a
**foreground Task subagent** with the evaluator prompt.

### `teardown(ws)`

```bash
claude stop <id> 2>/dev/null || true
claude rm <id>   2>/dev/null || true    # remove from the agents list (transcript kept)
# then the common worktree removal from backends.md
```
