# Executor Backends

Detection, backend selection, and the per-operation recipes that make
`/launch`, `/build`, and `/autopilot` host-agnostic. Read this before spawning
or steering any workspace agent.

---

## Table of Contents

1. [Detection](#detection)
2. [Backend Selection](#backend-selection)
3. [Operation Recipes](#operation-recipes)
4. [The Worktree-Context Constraint](#the-worktree-context-constraint)
5. [The cmux Fallback Ladder](#the-cmux-fallback-ladder)

---

## Detection

`detect()` resolves three things from the environment — **host**, **presentation
surface**, and **workflow capability** — using env markers plus `command -v`. No
guessing, no hardcoded executor.

```bash
# --- HOST: which agent runtime is reading this skill, and what binary it spawns ---
if [ -n "$CLAUDECODE" ]; then
  HOST=claude; EXECUTOR="claude --dangerously-skip-permissions --rc"
elif [ -n "$CODEX_HOME" ] || env | grep -q '^CODEX_'; then
  HOST=codex;  EXECUTOR="codex exec"
else
  HOST=generic; EXECUTOR="${AEP_EXECUTOR:-}"   # ask the user if empty
fi

# --- PRESENTATION: how a human watches a running session ---
if [ -n "$CMUX_SOCKET" ] || command -v cmux >/dev/null 2>&1; then
  PRESENT=cmux          # prefer $CMUX_SOCKET — proves we're INSIDE a cmux surface
elif command -v tmux >/dev/null 2>&1; then
  PRESENT=tmux
else
  PRESENT=none          # Desktop / no multiplexer
fi

# --- WORKFLOW CAPABILITY: only Claude Code has the dynamic-workflow (Workflow) tool ---
# Not shell-probable. The host agent knows: if you are Claude Code, you have it.
WORKFLOW_CAPABLE=$([ "$HOST" = claude ] && echo yes || echo no)
```

Notes:

- `$CMUX_SOCKET` (and the other `CMUX_*` vars) are set when the session is hosted
  _inside_ a cmux surface — a stronger signal than `command -v cmux`, which only
  proves cmux is installed somewhere. Prefer the env var when deciding whether
  you can spawn a _sibling_ cmux tab.
- `$TMUX` (set when already inside a tmux session) and `$CLAUDE_CODE_*` are
  available for finer decisions but are not required for backend selection.
- **Host knows itself.** A skill is executed by whatever agent loaded it. If you
  are Claude Code, `$CLAUDECODE` is set and the Workflow tool is available to you.
  If you are Codex, the `CODEX_*` markers are set and `codex exec` is your spawn
  binary. Detection confirms what the executing agent already is.

---

## Backend Selection

Apply in order. The first match wins, except B4 which overrides on explicit
opt-in.

```
B4  dynamic-workflow   IF user explicitly opted in ("…with workflow")
                       AND WORKFLOW_CAPABLE == yes        → select B4, stop
B1  session+cmux       ELIF PRESENT == cmux               → select B1
B2  session+tmux       ELIF PRESENT == tmux               → select B2
B3  native-subagent    ELSE (PRESENT == none)             → select B3
```

| ID     | Backend                                         | Monitor                        | Mid-flight nudge | Notes                                                     |
| ------ | ----------------------------------------------- | ------------------------------ | ---------------- | --------------------------------------------------------- |
| **B1** | claude/codex session in tmux, cmux tab          | signals                        | yes              | Prior default. Zero behavior change.                      |
| **B2** | claude/codex session in tmux, no cmux           | signals                        | yes              | Full loop; human runs `tmux attach` to watch live.        |
| **B3** | native subagent (CC Task tool / Codex subagent) | returned result + git/PR state | no               | Desktop fallback only. Tell the user the limits up front. |
| **B4** | dynamic-workflow fan-out, per-agent worktree    | `/workflows` view + signals    | no               | Opt-in, billed, background. Autonomous batch.             |

**Announce the selection.** Before spawning, state which backend and why — e.g.
"No tmux in this host → native subagent (B3): I'll build the story end-to-end and
report back, but there's no live monitor or mid-flight feedback in this mode."

---

## Operation Recipes

### `spawn(ws, branch, bootstrap_prompt)`

Start an implementation agent on `feat/<branch>` in
`.feature-workspaces/<ws>/`. The worktree is created the same way for every
backend; only the agent-start differs.

```bash
# Common to all backends — create the worktree (outside .claude/ — see launch guardrails)
mkdir -p .feature-workspaces
git worktree add -b feat/<ws> .feature-workspaces/<ws> main
```

**B1 — session in tmux + cmux tab:**

```bash
tmux new-session -d -s <ws> -c .feature-workspaces/<ws> "$EXECUTOR"
GEN_SURFACE=$(cmux new-surface --type terminal | grep -o 'surface:[0-9]*')
cmux send --surface "$GEN_SURFACE" "tmux attach -t <ws>\n"
cmux rename-tab --surface "$GEN_SURFACE" "<ws>"
# then send bootstrap_prompt via: cmux send --surface "$GEN_SURFACE" "<prompt>"
```

**B2 — session in tmux, no cmux:**

```bash
tmux new-session -d -s <ws> -c .feature-workspaces/<ws> "$EXECUTOR"
# then send bootstrap_prompt via: tmux send-keys -t <ws>:0.0 "<prompt>" Enter
echo "Workspace running in tmux session '<ws>'. Watch it live with: tmux attach -t <ws>"
```

For both B1 and B2: wait for the agent to initialize before sending the prompt
(`tmux capture-pane -t <ws>:0 -p -S -5 | grep -q '❯'`, or the Codex ready
indicator).

**B3 — native subagent (no tmux):**

- **Claude Code host:** use the Task/Agent tool with `isolation: worktree` (or
  cwd set to `.feature-workspaces/<ws>/`), passing `bootstrap_prompt` as the
  agent prompt. The subagent runs `/build` to completion and returns its result.
  Prefer background mode so the main session can poll signals between turns.
- **Codex host:** spawn a Codex subagent bound to the worktree directory with the
  same prompt.
- No `nudge()`/`liveness()` — the subagent runs to completion. `monitor()`
  degrades to reading any signals the subagent wrote, plus final git/PR state.

**B4 — dynamic workflow (Claude Code, explicit opt-in):**

Author a workflow whose stage(s) build (and verify) each story with per-agent
worktree isolation. One agent per story; the build stage runs `/build` for that
story's OpenSpec change.

```js
// sketch — the build agent gets the worktree via isolation:'worktree'
await pipeline(
  stories,
  (s) =>
    agent(`Run /build for OpenSpec change ${s.change}. ${s.bootstrap}`, {
      isolation: "worktree",
      phase: "Build",
    }),
  (built, s) =>
    agent(`Adversarially verify the build for ${s.change}.`, { phase: "Verify", schema: VERDICT }),
);
```

Monitoring is via the `/workflows` view; the build agents still write signals.
No mid-run human input — this is the hands-free batch path.

### `nudge(ws, msg)` — session backends (B1/B2) only

```bash
tmux send-keys -t <ws>:0.0 "<msg>" Enter
```

There is no `nudge` for B3 (subagent already returned or is non-interactive) or
B4 (workflows take no mid-run input). A consumer that requires nudging — notably
`/autopilot` — must run on a session backend. If detection yields B3/B4 and the
consumer needs `nudge`, surface that and stop.

### `liveness(ws)` — session backends (B1/B2)

```bash
# Session activity: capture the pane and compare to the last hash
tmux capture-pane -t <ws>:0.0 -p -S -20
# Host-independent fallback / corroboration: uncommitted work in the worktree
git -C .feature-workspaces/<ws> diff --stat
```

Pane-capture is the B1/B2 signal; the `git diff --stat` check is host-independent
and is the only liveness signal available under B3/B4 (where it corroborates the
returned result rather than a live pane).

### `monitor(ws)` — host-independent, all backends

```bash
cat .feature-workspaces/<ws>/.dev-workflow/signals/status.json
ls  .feature-workspaces/<ws>/.dev-workflow/signals/ready-for-review.flag 2>/dev/null
```

Unchanged from today. Mid-flight feedback is written the same way for B1/B2:

```bash
cat >> .feature-workspaces/<ws>/.dev-workflow/signals/feedback.md <<'EOF'
## <date> <time>
Priority: high
<feedback>
EOF
```

### `present(ws)` — human review surface

| Surface       | Recipe                                                                      |
| ------------- | --------------------------------------------------------------------------- |
| cmux (B1)     | the cmux tab created in `spawn()` already shows the live session            |
| tmux (B2)     | tell the human: `tmux attach -t <ws>` (read-only: `tmux attach -t <ws> -r`) |
| none (B3)     | headless — review via `monitor()` signals and the PR when it lands          |
| workflow (B4) | the `/workflows` view; plus signals + PR                                    |

### `teardown(ws)`

```bash
# Stop the session if one exists (B1/B2)
tmux kill-session -t <ws> 2>/dev/null || true
# Remove the worktree (all backends; /wrap normally owns this)
git worktree remove .feature-workspaces/<ws> 2>/dev/null || true
git worktree prune
```

B3 subagents and B4 workflow agents that used `isolation: worktree` are cleaned
up by their runtime; only an explicitly created `.feature-workspaces/<ws>`
worktree needs `git worktree remove`.

---

## The Worktree-Context Constraint

**B3 and B4 MUST spawn their agents bound to the workspace worktree** — via
`isolation: worktree` or by setting the agent's working directory to
`.feature-workspaces/<ws>/`.

This is not optional. The autopilot orchestrator boundary forbids spawning a
reviewer/agent "from main" precisely because such an agent lacks the workspace's
files, git state, and eval history. Binding the spawned agent to the worktree
gives it exactly that context, so the boundary's intent is satisfied under every
backend. The gen/eval separation (generator ≠ evaluator) and the rule that the
main session never reads workspace code directly both still hold — only the
spawn mechanism changes.

---

## The cmux Fallback Ladder

cmux is a **convenience, never a requirement**. Nothing functional depends on
it; it is purely the human's clickable live-view tab.

```
cmux present   → B1: clickable tab, live view
tmux present   → B2: same session + monitor loop; `tmux attach` to watch
no multiplexer → B3: headless autonomous subagent; review via signals + PR
```

Losing cmux costs only the tab UI. The file-based monitor loop and mid-flight
feedback survive in B2 unchanged. Skills must therefore gate every cmux call on
detection and never abort merely because cmux is absent.
