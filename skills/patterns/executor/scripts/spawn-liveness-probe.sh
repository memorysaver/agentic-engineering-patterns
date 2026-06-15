#!/usr/bin/env bash
# Post-spawn liveness probe for AEP executor spawns.
#
# A spawn call returning, a flag being set, or a roster/state entry saying
# "active" is NOT evidence a worker started. (The removed `claude-team` mode
# failed exactly here: the launch command was truncated in a detached tmux pane
# and never submitted, yet the team roster still showed the member "active".)
#
# A worker is LIVE only if BOTH hold within the timeout:
#   (1) the worker process/agent EXISTS — this is host-specific and must be
#       checked by the CALLER via the host tool, because an in-process
#       background subagent has no OS process to grep:
#         native-bg-subagent : TaskList shows <agent_id> (bare-hex id) running
#         claude-bg          : claude agents --json shows the session running
#         codex-subagent     : list_agents shows <agent_id>
#         codex-exec         : the codex exec PID is alive
#         legacy             : tmux pane_current_command == claude (NOT zsh)
#   (2) the worktree shows ACTIVITY — this script verifies the host-agnostic
#       half below.
#
# Usage: spawn-liveness-probe.sh <ws> <agent_id> [timeout_secs]
# Exit 0 = worktree active within timeout; 1 = dead spawn (tear down + fall back
#          to native-bg-subagent). The caller still confirms (1) above.
set -uo pipefail

WS="${1:?usage: spawn-liveness-probe.sh <ws> <agent_id> [timeout_secs]}"
AGENT_ID="${2:?missing agent_id}"
TIMEOUT="${3:-90}"

WT=".feature-workspaces/$WS"
SIG="$WT/.dev-workflow/signals/status.json"

worktree_active() {
  # status.json written by the worker, OR uncommitted edits in the worktree.
  [ -f "$SIG" ] && return 0
  [ -d "$WT" ] && [ -n "$(git -C "$WT" diff --stat 2>/dev/null)" ] && return 0
  return 1
}

deadline=$(( SECONDS + TIMEOUT ))
while [ "$SECONDS" -lt "$deadline" ]; do
  if worktree_active; then
    echo "LIVE: worktree '$WS' shows activity (agent_id=$AGENT_ID). Caller must still confirm the process/agent exists via the host tool."
    exit 0
  fi
  sleep 5
done

echo "DEAD: no worktree activity for '$WS' within ${TIMEOUT}s (agent_id=$AGENT_ID)." >&2
echo "  → Treat as a failed spawn: tear down the dead remnant (TeamDelete any team that got created)," >&2
echo "    then auto-fall-back to native-bg-subagent into the SAME worktree and probe again." >&2
echo "  → NEVER accept 'roster/state says active' as liveness." >&2
exit 1
