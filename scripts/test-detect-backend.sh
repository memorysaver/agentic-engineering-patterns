#!/usr/bin/env bash
# Fixture tests for skills/patterns/executor/scripts/detect-backend.sh.
#
# The mode matrix is a table people reason about and a selection order agents
# depend on; these cases are the executable half. Each builds a fake host (stub
# `claude` / `codex` / `tmux` on PATH, a throwaway git repo for the pin) and
# asserts the MODE that falls out — including the two that are easy to get
# wrong: a workflow opt-in on a host without the Workflow tool must NOT select
# workflow, and an ephemeral orchestrator must never select a session-bound mode.

set -uo pipefail

REPO="$(cd "$(dirname "$0")/.." && pwd)"
DETECT="$REPO/skills/patterns/executor/scripts/detect-backend.sh"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

pass=0
fail=0

make_host() {
  # make_host <bin-dir> <has-claude:yes|no> <claude-has-bg:yes|no> <has-codex:yes|no> <multi-agent:yes|no> <has-tmux:yes|no>
  local bin="$1" has_claude="$2" claude_bg="$3" has_codex="$4" multi="$5" has_tmux="$6"
  rm -rf "$bin"; mkdir -p "$bin"
  if [ "$has_claude" = yes ]; then
    { echo '#!/bin/sh'
      if [ "$claude_bg" = yes ]; then echo 'echo "  --bg   run in background"'; else echo 'echo "  --print"'; fi
    } > "$bin/claude"
    chmod +x "$bin/claude"
  fi
  if [ "$has_codex" = yes ]; then
    { echo '#!/bin/sh'
      if [ "$multi" = yes ]; then echo 'echo "multi_agent: true"'; else echo 'echo "multi_agent: false"'; fi
    } > "$bin/codex"
    chmod +x "$bin/codex"
  fi
  if [ "$has_tmux" = yes ]; then
    printf '#!/bin/sh\nexit 0\n' > "$bin/tmux"; chmod +x "$bin/tmux"
  fi
  # git is needed for the pin probe; expose the real one.
  ln -sf "$(command -v git)" "$bin/git"
  ln -sf "$(command -v grep)" "$bin/grep"
  ln -sf "$(command -v env)" "$bin/env"
  ln -sf "$(command -v sed)" "$bin/sed"
}

run_case() {
  # run_case <name> <expected-mode> <env-assignments...> -- <detect args...>
  local name="$1" expected="$2"; shift 2
  local envs=() args=()
  while [ $# -gt 0 ]; do
    if [ "$1" = "--" ]; then shift; args=("$@"); break; fi
    envs+=("$1"); shift
  done
  local actual
  actual=$(cd "$WORK/repo" && env -i PATH="$WORK/bin:/usr/bin:/bin" HOME="$WORK" "${envs[@]}" \
    bash "$DETECT" "${args[@]}" 2>/dev/null | sed -n 's/^MODE=//p')
  if [ "$actual" = "$expected" ]; then
    echo "PASS: $name → $actual"
    pass=$((pass + 1))
  else
    echo "FAIL: $name → expected $expected, got ${actual:-<none>}"
    fail=$((fail + 1))
  fi
}

mkdir -p "$WORK/repo" && git -C "$WORK/repo" init -q 2>/dev/null

# --- Claude Code host ---
make_host "$WORK/bin" yes yes no no yes
run_case "claude, long-lived"                 native-bg-subagent CLAUDECODE=1 --
run_case "claude, ephemeral, --bg available"  claude-bg          CLAUDECODE=1 -- --lifetime ephemeral
run_case "claude, workflow opt-in"            workflow           CLAUDECODE=1 -- --opt-in workflow
run_case "claude, tmux opt-in beats default"  legacy             CLAUDECODE=1 -- --opt-in tmux

make_host "$WORK/bin" yes no no no yes
run_case "claude >=2.1.x (no --bg), ephemeral" legacy            CLAUDECODE=1 -- --lifetime ephemeral

# --- Codex host ---
make_host "$WORK/bin" no no yes yes yes
run_case "codex, multi-agent, living thread"  codex-subagent     CODEX_HOME="$WORK" --
run_case "codex, multi-agent, cron tick"      codex-exec         CODEX_HOME="$WORK" -- --lifetime ephemeral
run_case "codex, workflow opt-in is refused"  codex-subagent     CODEX_HOME="$WORK" -- --opt-in workflow

make_host "$WORK/bin" no no yes no yes
run_case "codex without multi-agent"          codex-exec         CODEX_HOME="$WORK" --

# --- Generic host ---
make_host "$WORK/bin" no no no no yes
run_case "generic host with tmux"             legacy             AEP_EXECUTOR=myagent --
make_host "$WORK/bin" no no no no no
run_case "generic host, no surface"           headless           AEP_EXECUTOR=myagent --

# --- The pin is the only manual lever ---
make_host "$WORK/bin" yes yes no no yes
git -C "$WORK/repo" config aep.executor-backend tmux
run_case "pin wins over the native default"   legacy             CLAUDECODE=1 --
git -C "$WORK/repo" config --unset aep.executor-backend

echo ""
echo "detect-backend fixtures: $pass passed, $fail failed"
[ "$fail" -eq 0 ]
