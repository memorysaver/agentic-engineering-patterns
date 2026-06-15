# Remove `claude-team`; default to `native-bg-subagent` + mandatory liveness probe

**Status:** Accepted (2026-06-15)
**Supersedes:** the `claude-team` parts of [`native-first-executor.md`](./native-first-executor.md)

## Context

`claude-team` (agent teams, one teammate per story) was the Claude Code default
executor mode. On Claude Code ≥ 2.1.x (reproduced on **v2.1.177**, macOS) its
spawn path fails **silently**:

- The agent-teams runtime opens a detached tmux session on socket
  `claude-swarm-<lead-pid>` and pastes the launch command
  `claude … --agent-id <name>@<team> --settings '<large hooks JSON>'` into the pane.
- The long `--settings` JSON is **truncated mid-string and never submitted** —
  `pane_current_command` stays `zsh`, **no `--agent-id` process ever starts**, and
  the worktree shows zero activity indefinitely.
- **The team roster (`~/.claude/teams/<team>/config.json`) still lists the member
  as "active"**, so anything that checks roster membership thinks the worker is
  alive. It is not.
- A **live team also poisons teamless background spawns** — they auto-route through
  the same broken tmux backend. You must `TeamDelete` before a clean spawn.

This reproduced on two consecutive spawns. It is the same root-cause class as the
older "bootstrap paste needs a second Enter to submit" lesson, now hitting the
native agent-teams backend, not just legacy tmux.

Separately, on the same build the `claude --bg` one-shot spawn flag (the
`claude-bg` recipe) is **absent** from `claude --help` — background agents are now
the interactive `claude agents` view, not a scriptable flag.

## Decision

1. **Remove `claude-team`** as a selectable mode. The agent-teams env flag is no
   longer consulted; there is no "…with agent team" opt-in (the backend is broken,
   not merely de-prioritized).

2. **`native-bg-subagent` is the Claude Code default** — the Agent tool with
   `run_in_background: true`, **no `team_name`**, spawned with **no active team**.
   Session-bound, non-blocking, auto-notifies. A working spawn returns a
   **bare-hex `agentId`** + JSONL `output_file` (not an `@<team>` id). Steer via
   `SendMessage(to: agentId)` + `feedback.md`; present via `TaskOutput`; human gate
   is gate-and-park.

3. **A post-spawn liveness probe is mandatory** (`scripts/spawn-liveness-probe.sh`):
   after any spawn, before declaring "running", require **(a)** the worker
   process/agent exists (host tool) **AND** **(b)** the worktree shows activity
   (`status.json` written or non-empty `git diff`) within N seconds. On failure:
   tear down the dead remnant (`TeamDelete` any team), then **auto-fall-back to
   `native-bg-subagent`** into the same worktree. **"Roster/state says active" is
   never accepted as liveness.**

4. **Orphan/stuck detection uses real liveness, not roster membership** — a
   never-started worker is caught on the next tick, not after a 30-minute stuck
   timeout.

5. **`claude-bg` is gated on `BG_AVAILABLE`** (`claude --help | grep -- '--bg'`).
   Where the flag is gone it is skipped; `native-bg-subagent` is used. A
   cron/launchd driver that needs an OS-bound Claude worker is unsupported when
   `--bg` is absent — use Codex `codex-exec` or a long-lived in-session driver.

## Consequences

- One worktree + **one `native-bg-subagent` per story** per launch is preserved
  (the launch unit is unchanged; only the spawn mechanism changed).
- Claude Code loses push-via-`SendMessage`-to-a-teammate-pane and the in-process
  team display. Steering is `SendMessage(to: agentId)` + `feedback.md`; watching is
  `TaskOutput`. Acceptable — the team display never worked reliably here anyway.
- Claude Code has no OS-bound mode on builds without `claude --bg`; cron autopilot
  on Claude Code is unsupported there (documented in `backends.md` detection).

## Affected files

`aep-executor` (`SKILL.md`, `references/backends.md`, `references/claude-native.md`,
`scripts/spawn-liveness-probe.sh`), `aep-launch`, `aep-autopilot`
(`SKILL.md`, `references/tick-protocol.md`, `references/state-schema.md`,
`references/review-trigger.md`), and incidental mentions across `aep-build`,
`aep-wrap`, `aep-dispatch`, `aep-onboard`, `gen-eval`, glossary, quick-reference.
