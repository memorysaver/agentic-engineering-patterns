#!/usr/bin/env bash
# detect() — resolve the host, its two executor commands, native capabilities,
# the explicit pin, the presentation surface, and the selected MODE.
#
# This used to be forty lines of bash pasted into a reference file for an agent
# to retype. Retyped bash is bash that drifts: the probes are exact (an env-var
# test, a --help grep, a git config read), and exactness is the whole point of
# them. Run this instead.
#
#   eval "$(bash scripts/detect-backend.sh)"       # HOST, EXECUTOR, MODE, …
#   bash scripts/detect-backend.sh --json          # same, as JSON
#
# Orchestrator lifetime is NOT shell-probable — the calling agent knows it and
# passes it. You are long-lived as an interactive session, a /loop- or
# /goal-driven session, or a living Codex main thread; you are ephemeral when
# this invocation is a cron/launchd one-shot (a scheduled `codex exec` tick).
# Session-bound modes require a long-lived orchestrator, so the default is the
# conservative one only when you say so — it defaults to long because that is
# the interactive case, and a cron driver that forgets --lifetime ephemeral
# would otherwise select a worker that dies with its session.
#
#   --lifetime long|ephemeral   (default: long)
#   --opt-in workflow|tmux      the two natural-language opt-ins, when the user said so
#
# Exit 0 always; an unresolved executor is reported in EXECUTOR_UNRESOLVED=yes
# rather than by failing, so a caller can decide whether that is fatal.

LIFETIME=long
OPT_IN=""

while [ $# -gt 0 ]; do
  case "$1" in
    --lifetime) LIFETIME="${2:-long}"; shift 2 ;;
    --opt-in) OPT_IN="${2:-}"; shift 2 ;;
    --json) JSON=yes; shift ;;
    -h|--help) sed -n '2,26p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
    *) echo "unknown argument: $1 (try --help)" >&2; exit 2 ;;
  esac
done

# --- HOST + executor commands ---
#   EXECUTOR       interactive session (stays alive — legacy/tmux, evaluator panes)
#   EXECUTOR_EXEC  headless one-shot   (runs the given prompt to completion, exits)
if [ -n "${CLAUDECODE:-}" ]; then
  HOST=claude
  EXECUTOR="claude --dangerously-skip-permissions"            # interactive; NO -p
  EXECUTOR_EXEC="claude -p --dangerously-skip-permissions"    # -p/--print = non-interactive
  READY_GREP='❯'
elif command -v codex >/dev/null 2>&1 && { [ -n "${CODEX_HOME:-}" ] || env | grep -q '^CODEX_'; }; then
  HOST=codex
  EXECUTOR="codex --dangerously-bypass-approvals-and-sandbox"
  EXECUTOR_EXEC="codex exec --dangerously-bypass-approvals-and-sandbox"
  READY_GREP=''
else
  HOST=generic
  EXECUTOR="${AEP_EXECUTOR:-}"
  EXECUTOR_EXEC="${AEP_EXECUTOR_EXEC:-$EXECUTOR}"
  READY_GREP=''
fi
EXECUTOR_UNRESOLVED=no
[ -z "$EXECUTOR" ] && EXECUTOR_UNRESOLVED=yes

# --- NATIVE CAPABILITIES ---
# agent-teams is deliberately not consulted: claude-team was removed for silent
# spawn failure, so a mode is never selected from the teams flag.
BG_AVAILABLE=no
if [ "$HOST" = claude ] && claude --help 2>/dev/null | grep -q -- '--bg'; then
  BG_AVAILABLE=yes
fi
# On Claude Code >= 2.1.x the one-shot `claude --bg` flag was REMOVED, so
# BG_AVAILABLE=no and claude-bg is skipped. A cron/launchd driver then has no
# OS-bound Claude worker available — use Codex codex-exec or an in-session
# goal/loop driver instead.
MULTI_AGENT_AVAILABLE=no
if [ "$HOST" = codex ] && codex features list 2>/dev/null | grep -q 'multi_agent.*true'; then
  MULTI_AGENT_AVAILABLE=yes
fi
WORKFLOW_CAPABLE=no
[ "$HOST" = claude ] && WORKFLOW_CAPABLE=yes   # the host agent has the Workflow tool

# --- EXPLICIT PIN (the single manual lever besides "…with workflow") ---
PIN="$(git config --get aep.executor-backend 2>/dev/null || true)"

# --- PRESENTATION (legacy mode): cmux needs a reachable CLI and a target pane,
#     NOT $CMUX_SOCKET (the CLI drives cmux over its socket even when unset) ---
CMUX="$(command -v cmux || echo /Applications/cmux.app/Contents/Resources/bin/cmux)"
if [ -x "$CMUX" ] && { "$CMUX" tree 2>/dev/null | grep -q '◀ here' || [ -n "${CMUX_PANE_ID:-}" ]; }; then
  PRESENT=cmux
elif command -v tmux >/dev/null 2>&1; then
  PRESENT=tmux
else
  PRESENT=none
fi

# --- MODE SELECTION: apply in order, first match wins ---
if [ "$OPT_IN" = workflow ] && [ "$WORKFLOW_CAPABLE" = yes ]; then
  MODE=workflow
elif [ "$PIN" = tmux ] || [ "$OPT_IN" = tmux ]; then
  MODE=legacy
elif [ "$HOST" = claude ] && [ "$LIFETIME" = long ]; then
  MODE=native-bg-subagent
elif [ "$HOST" = claude ] && [ "$BG_AVAILABLE" = yes ]; then
  MODE=claude-bg
elif [ "$HOST" = codex ] && [ "$MULTI_AGENT_AVAILABLE" = yes ] && [ "$LIFETIME" = long ]; then
  MODE=codex-subagent
elif [ "$HOST" = codex ]; then
  MODE=codex-exec
elif [ "$PRESENT" = cmux ] || [ "$PRESENT" = tmux ]; then
  MODE=legacy
else
  MODE=headless
fi

if [ "${JSON:-no}" = yes ]; then
  printf '{\n'
  printf '  "host": "%s",\n' "$HOST"
  printf '  "executor": "%s",\n' "$EXECUTOR"
  printf '  "executor_exec": "%s",\n' "$EXECUTOR_EXEC"
  printf '  "executor_unresolved": %s,\n' "$([ "$EXECUTOR_UNRESOLVED" = yes ] && echo true || echo false)"
  printf '  "bg_available": %s,\n' "$([ "$BG_AVAILABLE" = yes ] && echo true || echo false)"
  printf '  "multi_agent_available": %s,\n' "$([ "$MULTI_AGENT_AVAILABLE" = yes ] && echo true || echo false)"
  printf '  "workflow_capable": %s,\n' "$([ "$WORKFLOW_CAPABLE" = yes ] && echo true || echo false)"
  printf '  "pin": "%s",\n' "$PIN"
  printf '  "present": "%s",\n' "$PRESENT"
  printf '  "lifetime": "%s",\n' "$LIFETIME"
  printf '  "mode": "%s"\n' "$MODE"
  printf '}\n'
else
  printf 'HOST=%s\n' "$HOST"
  printf 'EXECUTOR=%s\n' "$(printf '%q' "$EXECUTOR")"
  printf 'EXECUTOR_EXEC=%s\n' "$(printf '%q' "$EXECUTOR_EXEC")"
  printf 'EXECUTOR_UNRESOLVED=%s\n' "$EXECUTOR_UNRESOLVED"
  printf 'READY_GREP=%s\n' "$(printf '%q' "$READY_GREP")"
  printf 'BG_AVAILABLE=%s\n' "$BG_AVAILABLE"
  printf 'MULTI_AGENT_AVAILABLE=%s\n' "$MULTI_AGENT_AVAILABLE"
  printf 'WORKFLOW_CAPABLE=%s\n' "$WORKFLOW_CAPABLE"
  printf 'PIN=%s\n' "$(printf '%q' "$PIN")"
  printf 'PRESENT=%s\n' "$PRESENT"
  printf 'LIFETIME=%s\n' "$LIFETIME"
  printf 'MODE=%s\n' "$MODE"
fi
