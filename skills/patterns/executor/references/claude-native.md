# Claude Code Native Backends — `claude-team` & `claude-bg`

Per-operation recipes for the two Claude Code native modes. Both replace tmux
with capabilities built into the Claude Code CLI; neither requires tmux, cmux,
or any hook. Detection and selection live in `backends.md` — read that first.

| Mode            | Mechanism                                  | Lifetime                                   | Steering                            | Human gate                                           |
| --------------- | ------------------------------------------ | ------------------------------------------ | ----------------------------------- | ---------------------------------------------------- |
| **claude-team** | agent teams (teammate per story)           | session-bound (dies with the lead session) | `SendMessage` (push)                | teammate→lead `HUMAN_GATE:` message                  |
| **claude-bg**   | native background sessions (`claude --bg`) | OS-bound (survives the lead session)       | `feedback.md` (pull) + stop/respawn | gate-and-park → main agent relays via session resume |

---

## Mode: `claude-team`

One **teammate per story**, spawned by the orchestrator (the team lead) with the
Agent tool. Each teammate is a separate Claude instance with its own context
window. The human can interact with a teammate directly (split pane or
`Shift+Down` cycling, per `teammateMode`).

### Prerequisites

- Agent teams is **experimental** (verified 2026-06): it requires the
  `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` env var (Claude Code ≥ 2.1.32). The
  recommended project-level enablement is the `env` block of
  `.claude/settings.json` (a settings entry, not a hook):

  ```json
  { "env": { "CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS": "1" } }
  ```

- The flag gates `TeamCreate`, `TeamDelete`, and `SendMessage`. If unset,
  detection falls through to `claude-bg`.
- Optional display setting: `"teammateMode": "auto" | "in-process" | "tmux"`
  (settings.json). `auto` uses split panes when tmux/iTerm2 is available,
  in-process otherwise. AEP does not set this for the user.

### Team lifecycle (the standing-team pattern)

Teams have hard constraints (per agent-teams docs): **one team at a time**,
team is **bound to the lead session** (`/resume` does not restore in-process
teammates), and `TeamDelete` requires all teammates shut down first. Teammates,
however, can be spawned and shut down **individually at any time**. Therefore:

```
orchestration start  → TeamCreate once (team name: "aep")
each story launch    → spawn ONE teammate into the standing team
each story wrap      → shutdown_request to THAT teammate (team persists)
orchestration stop   → shutdown all teammates, then TeamDelete
```

Never create a team per story — the one-team constraint would force destructive
cleanup between stories. Never expect the team to survive the lead session:
if the lead restarts, teammates are orphaned (see Orphan re-adoption in
`backends.md`); worker progress is safe because it lives in the worktree +
`.dev-workflow/signals/`, not in the team.

### `spawn(ws, branch, bootstrap_prompt)`

The worktree is created by AEP first (common recipe in `backends.md`). Then
spawn the teammate with the Agent tool:

```
Agent tool:
  name: <ws>                          # teammate addressable by workspace name
  team_name: aep                      # the standing team
  prompt: |
    You operate EXCLUSIVELY in <abs-repo-path>/.feature-workspaces/<ws>
    on branch feat/<ws>. cd there first; never edit files outside it.
    <bootstrap_prompt>                # the /aep-build bootstrap, incl. Prior Lessons
    Report progress through .dev-workflow/signals/status.json at phase
    boundaries. If you hit a decision only the human can make, follow the
    human-gate protocol: append to .dev-workflow/signals/needs-human.md,
    set "blocked_on": "human" in status.json, and SendMessage the team lead
    with a message starting "HUMAN_GATE:".
```

The worktree binding is a **prompt contract** (AEP's no-hooks decision: rely on
model capability + skill instructions, not a WorktreeCreate hook). The cwd
instruction is first in the prompt so it is the teammate's first action.

### `nudge(ws, msg)`

```
SendMessage(to: <ws>, message: <msg>)
```

Push-delivered; the teammate receives it without polling. Use the exact nudge
texts from the autopilot tick protocol. Also append the same message to
`signals/feedback.md` (file remains the durable record).

### `liveness(ws)`

```
TaskList / TaskGet — teammate task status
git -C .feature-workspaces/<ws> diff --stat     # host-independent corroboration
```

A teammate that no longer appears (lead restarted, teammate crashed) is an
**orphan**, not a failure — see the orphan re-adoption protocol in `backends.md`.

### `present(ws)`

Tell the human: the teammate is visible per `teammateMode` — split pane (click
into it) or in-process (`Shift+Down` to cycle). The human can type into the
teammate directly; this is the native replacement for the cmux review tab.

### `gate(ws)` — human gate (block-in-place, hub-and-spoke)

1. Worker appends the question to `signals/needs-human.md` + sets
   `"blocked_on": "human"` in `status.json` (always — the file is the
   host-agnostic record).
2. Worker sends `SendMessage` to the lead prefixed `HUMAN_GATE:`.
3. Lead asks the human (AskUserQuestion or plain text), then relays the answer
   with `SendMessage(to: <ws>, ...)`. The human may instead answer directly in
   the teammate's pane.
4. Worker appends `resolved: <answer summary>` to `needs-human.md`, clears
   `blocked_on`, continues.

### `spawn_evaluator(ws, role)`

The generator (teammate) spawns a **foreground Task subagent** in its own
session — it inherits the worktree cwd, and foreground means permission
prompts pass through rather than auto-denying. The evaluator prompt (from
`aep-gen-eval/references/agent-contracts.md`) **is** the spawn prompt; there is
no separate send step, no `sleep`, no pane to kill. The subagent returns on
completion — poll for `eval-response-<N>.md` only as a sanity check.

> Teammates can spawn subagents (one level); they cannot spawn nested teams.
> A foreground subagent is exactly right for the bounded eval round.

### `teardown(ws)`

```
SendMessage(to: <ws>, message: {type: "shutdown_request"})   # teammate exits; team persists
# then the common worktree removal from backends.md
```

At orchestration stop (not per story): shut down all teammates, then `TeamDelete`.

---

## Mode: `claude-bg`

One **native background session per story**. `claude --bg` starts a full Claude
instance detached from the current session; it is **OS-bound** — it survives
lead restarts and is attachable from any terminal. This is the GA fallback when
the teams flag is absent, and the required Claude Code mode for cron-style
drivers (see the driver × backend matrix in `backends.md`).

### Capability probe

```bash
claude --help 2>/dev/null | grep -q -- '--bg' && echo "claude-bg available"
```

### `spawn(ws, branch, bootstrap_prompt)`

The process cwd **is** the isolation — this mode hard-binds the worker to the
worktree, no prompt contract needed:

```bash
cd .feature-workspaces/<ws> && claude --bg --dangerously-skip-permissions "<bootstrap_prompt>"
# capture the printed session id → state agent_id
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

Same as claude-team: the bg session is a full Claude instance running in the
worktree — it spawns a **foreground Task subagent** with the evaluator prompt.

### `teardown(ws)`

```bash
claude stop <id> 2>/dev/null || true
claude rm <id>   2>/dev/null || true    # remove from the agents list (transcript kept)
# then the common worktree removal from backends.md
```
