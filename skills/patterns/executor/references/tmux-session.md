# Legacy Backend — tmux Session (+ optional cmux tab)

Per-operation recipes for the **legacy** mode: a long-lived interactive
executor session hosted in tmux, optionally presented through a cmux review
tab. This was the v1.x default for Claude Code (backends B1/B2); it is now
selected only when the user **pins it explicitly** (`git config
aep.executor-backend tmux` or "…with tmux") or on a **generic host** where
tmux is the only session mechanism available. Detection and selection live in
`backends.md`.

OS-bound: tmux sessions survive the orchestrator session and work under both
the long-lived and cron driver models.

---

## `spawn(ws, branch, bootstrap_prompt)`

The worktree is created by AEP first (common recipe in `backends.md`). Then:

> `$EXECUTOR` is the **interactive** session command from `detect()` — bare
> `claude --dangerously-skip-permissions` / generic session command, never
> `-p` / `codex exec`. Guard first:
> `[ -z "$EXECUTOR" ] && { echo "run detect() — \$EXECUTOR unset"; exit 1; }`
> so an unset executor aborts loudly instead of launching a bare login shell.

**With cmux available (legacy+cmux):** spawn the tmux session **only**; the
cmux review tab is attached _after_ the bootstrap is sent (attaching a surface
focuses the tmux composer and blocks external `send-keys`).

```bash
tmux new-session -d -s <ws> -c .feature-workspaces/<ws> "$EXECUTOR"
```

**Without cmux:**

```bash
tmux new-session -d -s <ws> -c .feature-workspaces/<ws> "$EXECUTOR"
echo "Workspace running in tmux session '<ws>'. Watch it live with: tmux attach -t <ws>"
```

**Readiness + bootstrap send.** Wait for the agent to initialize, then send the
prompt. The readiness signal is executor-specific (`$READY_GREP` from
`detect()`); the send uses `-l` so a multi-line prompt is entered literally and
a single trailing `Enter` submits it (a bare `send-keys "$PROMPT" Enter` would
let embedded newlines submit the prompt line-by-line):

```bash
if [ -n "$READY_GREP" ]; then
  for _ in $(seq 1 12); do
    tmux capture-pane -t <ws>:0 -p -S -5 | grep -q "$READY_GREP" && break; sleep 2
  done
else
  sleep 8           # no readiness glyph configured — give the composer time to come up
fi
tmux send-keys -t <ws>:0.0 -l -- "$bootstrap_prompt"   # literal text (handles multi-line)
tmux send-keys -t <ws>:0.0 Enter                       # one submit
```

**Attach the cmux review tab (AFTER the bootstrap).** Open the tab as a
**sibling in the pane that holds the orchestrator's own tab** — never
`cmux new-workspace` (that makes a separate top-level workspace) and never a
bare `cmux new-surface` (it defaults to an unset `$CMUX_WORKSPACE_ID`). Resolve
the pane from `cmux tree` (the orchestrator's tab is marked `◀ here`), falling
back to the surface env vars when we're inside one:

```bash
# $CMUX is the CLI path resolved in detect(). Run this only when PRESENT == cmux.
read -r WS PANE < <("$CMUX" tree 2>/dev/null | awk '
  /workspace workspace:/ {for (i=1;i<=NF;i++) if ($i ~ /^workspace:/) ws=$i}
  /pane pane:/           {for (i=1;i<=NF;i++) if ($i ~ /^pane:/)      pane=$i}
  /◀ here/               {print ws, pane; exit}')
: "${WS:=$CMUX_WORKSPACE_ID}" "${PANE:=$CMUX_PANE_ID}"
if [ -n "$PANE" ]; then                                  # a target pane exists
  SREF=$("$CMUX" new-surface --type terminal --workspace "$WS" --pane "$PANE" --focus true \
         | grep -oE 'surface:[0-9]+' | head -1)
  "$CMUX" send --surface "$SREF" "tmux attach -t <ws>"$'\n'   # trailing newline submits
  "$CMUX" rename-tab --surface "$SREF" "<ws>"
else                                                     # reachable but no pane → tmux-only
  echo "cmux reachable but no target pane — watch with: tmux attach -t <ws>"
fi
```

### The cmux fallback ladder

cmux is a **convenience, never a requirement**. Nothing functional depends on
it; it is purely the human's clickable live-view tab.

```
cmux tab attachable → clickable review tab (sibling of the orchestrator's tab), live view
tmux present        → same session + monitor loop; `tmux attach` to watch
```

"cmux tab attachable" = the `cmux` CLI is reachable **and** a target pane
resolves (`cmux tree` shows `◀ here`, or `$CMUX_PANE_ID` is set) — it does
**not** require `$CMUX_SOCKET`. Reachable-but-no-pane degrades to tmux-only;
losing cmux costs only the tab UI. Skills must gate every cmux call on
detection and never abort merely because cmux is absent.

---

## `nudge(ws, msg)`

```bash
# -l sends the message literally (handles multi-line nudges); a separate Enter submits once.
tmux send-keys -t <ws>:0.0 -l -- "<msg>"
tmux send-keys -t <ws>:0.0 Enter
```

---

## `liveness(ws)`

```bash
# Session activity: capture the pane and compare to the last hash
tmux capture-pane -t <ws>:0.0 -p -S -20
# Host-independent fallback / corroboration: uncommitted work in the worktree
git -C .feature-workspaces/<ws> diff --stat
```

---

## `present(ws)`

| Surface | Recipe                                                                         |
| ------- | ------------------------------------------------------------------------------ |
| cmux    | the review tab attached at the end of `spawn()` already shows the live session |
| tmux    | tell the human: `tmux attach -t <ws>` (read-only: `tmux attach -t <ws> -r`)    |

---

## `gate(ws)` — human gate

Worker appends to `signals/needs-human.md` + sets `"blocked_on": "human"` in
`status.json` (the host-agnostic protocol). The orchestrator surfaces it:
"workspace `<ws>` needs a decision — `tmux attach -t <ws>`, answer in the
session (or write to `signals/feedback.md`), detach."

---

## `spawn_evaluator(ws, role)`

The generator spawns the evaluator in a bottom tmux pane (eval-protocol
**Context A**):

```bash
# Split current tmux window vertically (top=generator, bottom=evaluator). The evaluator
# needs to read files and write eval-response, so it runs the INTERACTIVE executor.
tmux split-window -v -c "$(pwd)" "${EXECUTOR:-claude --dangerously-skip-permissions}"
tmux select-pane -t :.0            # return focus to the generator pane

sleep 10
tmux send-keys -t :.1 -l -- "$EVAL_PROMPT"    # evaluator prompt from agent-contracts.md
tmux send-keys -t :.1 Enter

while [ ! -f .dev-workflow/signals/eval-response-<N>.md ]; do sleep 15; done
tmux kill-pane -t :.1
```

> Use `tmux split-window`, not `cmux split` — the generator runs inside tmux
> but was not spawned by cmux, so it cannot use cmux socket commands. Under
> cmux the attached surface displays both panes automatically.

---

## `teardown(ws)`

```bash
tmux kill-session -t <ws> 2>/dev/null || true
# then the common worktree removal from backends.md
```
