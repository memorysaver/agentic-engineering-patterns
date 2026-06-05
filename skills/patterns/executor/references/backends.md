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

`detect()` resolves the **host**, its two **executor commands** (interactive
session vs headless one-shot — they are different invocations, see below), the
**presentation surface**, and **workflow capability** — using env markers plus
`command -v`. No guessing, no hardcoded executor.

```bash
# --- HOST + executor commands ---
# Two modes, because the session backends (B1/B2) need a LONG-LIVED, steerable
# process while evaluator/check execs may need a ONE-SHOT runner — and the two are
# different invocations per CLI:
#   $EXECUTOR       interactive session (stays alive, accepts tmux send-keys)
#   $EXECUTOR_EXEC  headless one-shot   (runs the given prompt to completion, exits)
if [ -n "$CLAUDECODE" ]; then
  HOST=claude
  EXECUTOR="claude --dangerously-skip-permissions"            # interactive is the default; NO -p
  EXECUTOR_EXEC="claude -p --dangerously-skip-permissions"    # -p/--print = non-interactive
  READY_GREP='❯'                                              # pane shows ❯ when ready
elif command -v codex >/dev/null 2>&1 && { [ -n "$CODEX_HOME" ] || env | grep -q '^CODEX_'; }; then
  HOST=codex
  EXECUTOR="codex --dangerously-bypass-approvals-and-sandbox"           # session fallback only
  EXECUTOR_EXEC="codex exec --dangerously-bypass-approvals-and-sandbox" # cheap/check one-shots, not coding launch
  READY_GREP=''                                                         # codex TUI has no ❯ — use a timed wait
else
  HOST=generic
  EXECUTOR="${AEP_EXECUTOR:-}"; EXECUTOR_EXEC="${AEP_EXECUTOR_EXEC:-$EXECUTOR}"; READY_GREP=''
fi
# Guard: never spawn an empty command (an unset executor would start a bare login shell).
[ -z "$EXECUTOR" ] && { echo "executor unresolved — set \$AEP_EXECUTOR or run under Claude Code / Codex"; }

# --- PRESENTATION: how a human watches a running session ---
# cmux can host a review tab whenever its CLI is reachable AND we can resolve a
# target pane — it does NOT require $CMUX_SOCKET. The cmux CLI drives cmux over its
# Unix socket even when $CMUX_SOCKET is unset (e.g. Claude Code inside a
# cmux-managed tmux session does not inherit it). Resolve the binary robustly, then
# confirm a target pane exists: `cmux tree` marks the orchestrator's own pane
# "◀ here", or $CMUX_PANE_ID is set when we're already inside a surface.
CMUX="$(command -v cmux || echo /Applications/cmux.app/Contents/Resources/bin/cmux)"
if [ -x "$CMUX" ] && { "$CMUX" tree 2>/dev/null | grep -q '◀ here' || [ -n "$CMUX_PANE_ID" ]; }; then
  PRESENT=cmux
elif command -v tmux >/dev/null 2>&1; then
  PRESENT=tmux
else
  PRESENT=none          # Desktop / no multiplexer
fi

# --- WORKFLOW CAPABILITY: only Claude Code has the dynamic-workflow (Workflow) tool ---
# Not shell-probable. The host agent knows: if you are Claude Code, you have it.
WORKFLOW_CAPABLE=$([ "$HOST" = claude ] && echo yes || echo no)
```

> **Correct CLI invocations (verified against Claude Code 2.1.161 / Codex 0.130.0):**
>
> |            | interactive session (B1/B2) → `$EXECUTOR`          | headless one-shot (B3 / exec) → `$EXECUTOR_EXEC`        |
> | ---------- | -------------------------------------------------- | ------------------------------------------------------- |
> | **claude** | `claude --dangerously-skip-permissions`            | `claude -p --dangerously-skip-permissions`              |
> | **codex**  | `codex --dangerously-bypass-approvals-and-sandbox` | `codex exec --dangerously-bypass-approvals-and-sandbox` |
>
> `--rc` is **not** a real Claude Code flag (it was a bug; removed). `codex exec`
> is **non-interactive** and reserved for cheap/read-only check-style one-shots;
> Codex coding launches use the native subagent/worktree path (B3) first, even
> when tmux is installed. Codex's full-bypass flag is
> `--dangerously-bypass-approvals-and-sandbox` (there is no `--yolo` /
> `--full-auto`).

Notes:

- **cmux does not require `$CMUX_SOCKET`.** The `cmux` CLI controls cmux over its
  Unix socket even when `$CMUX_SOCKET` is unset (Claude Code running inside a
  cmux-managed tmux session does not inherit it). What you actually need to open a
  _sibling_ tab is a **target pane**: `cmux tree` marks the orchestrator's own pane
  `◀ here`, and `$CMUX_PANE_ID` / `$CMUX_WORKSPACE_ID` are set when you're inside a
  surface. Reachable CLI **and** a resolvable pane ⇒ B1; reachable but no pane ⇒ B2.
- `$TMUX` (set when already inside a tmux session) and `$CLAUDE_CODE_*` are
  available for finer decisions but are not required for backend selection.
- **Host knows itself.** A skill is executed by whatever agent loaded it. If you
  are Claude Code, `$CLAUDECODE` is set and the Workflow tool is available to you.
  If you are Codex, the `CODEX_*` markers are set and `/launch` uses Codex native
  subagents for coding work (`codex exec` only for cheap/check one-shots).
  Detection confirms what the executing agent already is.

---

## Backend Selection

Apply in order. The first match wins. B4 is the only explicit opt-in path; Codex
coding launches are subagent-first so they keep Codex-native execution while
retaining AEP's git worktree isolation.

```
B4  dynamic-workflow   IF user explicitly opted in ("…with workflow")
                       AND WORKFLOW_CAPABLE == yes        → select B4, stop
B3  codex-subagent     ELIF HOST == codex                 → select B3, stop
B1  session+cmux       ELIF PRESENT == cmux               → select B1
B2  session+tmux       ELIF PRESENT == tmux               → select B2
B3  native-subagent    ELSE (PRESENT == none)             → select B3
```

| ID     | Backend                                         | Monitor                        | Mid-flight nudge | Notes                                                  |
| ------ | ----------------------------------------------- | ------------------------------ | ---------------- | ------------------------------------------------------ |
| **B1** | claude/generic session in tmux, cmux tab        | signals                        | yes              | Prior default for session-capable non-Codex hosts.     |
| **B2** | claude/generic session in tmux, no cmux         | signals                        | yes              | Full loop; human runs `tmux attach` to watch live.     |
| **B3** | native subagent (CC Task tool / Codex subagent) | returned result + git/PR state | no               | Codex default; non-Codex fallback when no tmux exists. |
| **B4** | dynamic-workflow fan-out, per-agent worktree    | `/workflows` view + signals    | no               | Opt-in, billed, background. Autonomous batch.          |

**Announce the selection.** Before spawning, state which backend and why — e.g.
"Codex host → native subagent (B3): I'll create the AEP worktree first, then run
the build in a Codex worker bound to that worktree. There is no live tmux monitor
or mid-flight feedback in this mode."

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

> `$EXECUTOR` is the **interactive** session command from `detect()` for B1/B2 —
> bare `claude` / generic session command, never `-p` / `codex exec`. Guard first:
> `[ -z "$EXECUTOR" ] && { echo "run detect() — \$EXECUTOR unset"; exit 1; }`
> so an unset executor aborts loudly instead of launching a bare login shell.

**B1 — session in tmux + cmux review tab:**

Spawn the tmux session **only**. The cmux review tab is attached _after_ the
bootstrap is sent (see "Attach the cmux review tab" below): attaching a surface
focuses the tmux composer and blocks external `send-keys`, so the prompt must land
first.

```bash
tmux new-session -d -s <ws> -c .feature-workspaces/<ws> "$EXECUTOR"
```

**B2 — session in tmux, no cmux:**

```bash
tmux new-session -d -s <ws> -c .feature-workspaces/<ws> "$EXECUTOR"
echo "Workspace running in tmux session '<ws>'. Watch it live with: tmux attach -t <ws>"
```

**Readiness + bootstrap send (B1/B2).** Wait for the agent to initialize, then
send the prompt. The readiness signal is executor-specific (`$READY_GREP` from
`detect()`); the send uses `-l` so a multi-line prompt is entered literally and a
single trailing `Enter` submits it (a bare `send-keys "$PROMPT" Enter` would let
embedded newlines submit the prompt line-by-line):

```bash
if [ -n "$READY_GREP" ]; then
  for _ in $(seq 1 12); do
    tmux capture-pane -t <ws>:0 -p -S -5 | grep -q "$READY_GREP" && break; sleep 2
  done
else
  sleep 8           # codex TUI has no ❯ glyph — give the composer time to come up
fi
tmux send-keys -t <ws>:0.0 -l -- "$bootstrap_prompt"   # literal text (handles multi-line)
tmux send-keys -t <ws>:0.0 Enter                       # one submit
```

**Attach the cmux review tab (B1 only, AFTER the bootstrap).** Open the tab as a
**sibling in the pane that holds the orchestrator's own tab** — never
`cmux new-workspace` (that makes a separate top-level workspace) and never a bare
`cmux new-surface` (it defaults to an unset `$CMUX_WORKSPACE_ID`). Resolve the pane
from `cmux tree` (the orchestrator's tab is marked `◀ here`), falling back to the
surface env vars when we're inside one:

```bash
# $CMUX is the CLI path resolved in detect(). Run this only when PRESENT == cmux.
read -r WS PANE < <("$CMUX" tree 2>/dev/null | awk '
  /workspace workspace:/ {for (i=1;i<=NF;i++) if ($i ~ /^workspace:/) ws=$i}
  /pane pane:/           {for (i=1;i<=NF;i++) if ($i ~ /^pane:/)      pane=$i}
  /◀ here/               {print ws, pane; exit}')
: "${WS:=$CMUX_WORKSPACE_ID}" "${PANE:=$CMUX_PANE_ID}"
if [ -n "$PANE" ]; then                                  # genuine B1 — a target pane exists
  SREF=$("$CMUX" new-surface --type terminal --workspace "$WS" --pane "$PANE" --focus true \
         | grep -oE 'surface:[0-9]+' | head -1)
  "$CMUX" send --surface "$SREF" "tmux attach -t <ws>"$'\n'   # trailing newline submits
  "$CMUX" rename-tab --surface "$SREF" "<ws>"
else                                                     # reachable but no pane → degrade to B2
  echo "cmux reachable but no target pane — headless B2: tmux attach -t <ws>"
fi
```

**B3 — native subagent (Codex default; non-Codex fallback when no tmux):**

- **Claude Code host:** use the Task/Agent tool with `isolation: worktree` (or
  cwd set to `.feature-workspaces/<ws>/`), passing `bootstrap_prompt` as the
  agent prompt. The subagent runs `/build` to completion and returns its result.
  Prefer background mode so the main session can poll signals between turns.
- **Codex host:** spawn a Codex worker/subagent after the common worktree setup.
  The worker prompt must include the absolute worktree path and this contract:
  "Operate only in `<repo>/.feature-workspaces/<ws>` on branch `feat/<ws>`.
  Run the bootstrap prompt from that directory. Do not edit the main checkout.
  Report progress through `.dev-workflow/signals/` and finish with the usual
  `/build` result/PR state." Use the native Codex subagent mechanism available in
  the host; if the host exposes a separate Codex thread that can be started in an
  existing directory, start it in `.feature-workspaces/<ws>/`.
- **Do not use `codex exec` for coding launch.** Keep `codex exec` for
  cheap/read-only checks and other bounded one-shot analysis, not long-running
  implementation.
- No `nudge()`/`liveness()` — the subagent runs to completion. `monitor()`
  degrades to reading any signals the subagent wrote, plus final git/PR state.

**B4 — dynamic workflow (Claude Code, explicit opt-in):**

Author a workflow whose stage(s) build (and verify) each story with per-agent
worktree isolation. One agent per story; the build stage runs `/build` for that
story's OpenSpec change.

```js
// sketch — the build agent gets the worktree via isolation:'worktree'.
// `stories` is the dispatched wave: each item = { change: "<openspec-change-id>", bootstrap: "<prompt tail>" }.
const VERDICT = {
  type: "object",
  properties: { real: { type: "boolean" }, summary: { type: "string" } },
  required: ["real"],
};
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

### `spawn_evaluator(ws, role)` — evaluator, worktree-bound, per backend

Spawn a **separate** evaluator agent (never the generator grading itself), always
**bound to the workspace worktree** so it sees the build's files and git state.
`/build` Phase 5 calls this; the request/response signal files and convergence
rules are identical across backends — only the spawn mechanism differs:

| Backend   | Evaluator spawn                                                                           | eval-protocol context                                                                                                  |
| --------- | ----------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| **B1/B2** | `tmux split-window -v -c .feature-workspaces/<ws> "$EXECUTOR"` (interactive, bottom pane) | Context A                                                                                                              |
| **B3**    | sibling worktree-bound subagent/evaluator using the host's native agent mechanism         | Context B **mechanism** (Agent-tool/subagent) — but worktree-bound, **not** the main-session read-only `/validate` use |
| **B4**    | the workflow's `verify` stage (already worktree-isolated, in-host)                        | Context C **mechanism** (programmatic subagent) — in-host dynamic-workflow, not API/SDK CI                             |

> The eval-protocol Context labels describe the _spawn mechanism_; under the
> executor the evaluator is **always worktree-bound** (per the Worktree-Context
> Constraint below), which is what keeps it consistent with the generator's build
> and with autopilot's "never spawn a reviewer from main" boundary.

### `nudge(ws, msg)` — session backends (B1/B2) only

```bash
# -l sends the message literally (handles multi-line nudges); a separate Enter submits once.
tmux send-keys -t <ws>:0.0 -l -- "<msg>"
tmux send-keys -t <ws>:0.0 Enter
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

### `check(prompt, schema)` — cheap, context-isolated analysis

Run a **read-only analysis** prompt in a throwaway, cheap-model agent and return
its structured JSON. The point is **context isolation**: the verbose reading
(state file + every workspace `signals/`, `gh pr view`, …) happens in the cheap
agent's own context and is discarded — only the small JSON result crosses back,
so a long-lived orchestrator session (e.g. `/autopilot` under `/loop`) doesn't
accumulate per-tick tokens. The check **never reads workspace code** (signals
only), so it does not cross the orchestrator boundary.

The host's cheap model and isolation mechanism (model names drift — these are the
current defaults, override as needed):

**Claude Code — Haiku subagent (own context window; only its final message returns):**

```
Use the Agent/Task tool with:
  model: haiku
  tools: Read, Bash, Glob        # signals + jq + gh; no workspace-code reads
  prompt: <the analysis prompt; instruct it to OUTPUT ONLY the JSON in `schema`>
Capture the returned text as the JSON result.
```

> A subagent cannot spawn further subagents (one level). That's fine — the check
> only reads and decides; it returns actions for the orchestrator to perform.

**Codex — `codex exec` cheap one-shot (fresh context per call; only stdout returns):**

```bash
codex exec -m gpt-5.4-mini -c model_reasoning_effort=low \
  -C "$PWD" --skip-git-repo-check --dangerously-bypass-approvals-and-sandbox \
  --output-schema /tmp/aep-check.schema.json \
  -o /tmp/aep-check.out.json \
  "<the analysis prompt>" < /dev/null      # < /dev/null: exec hangs on stdin otherwise
jq . /tmp/aep-check.out.json               # read the structured result
```

`--output-schema` constrains the final message to your JSON Schema; `-o` writes
just that message to a file. For CHECK, prefer `codex exec` because it gives the
orchestrator a fresh cheap context and structured stdout. This is different from
coding launch, where Codex uses the worktree-bound native subagent path (B3).

**Result schema (the CHECK → ACT contract).** The check returns an action list the
orchestrator executes; it may write its own state files (e.g. `autopilot-state.json`)
but does not mutate workspaces:

```json
{
  "summary": "string — one-line human-readable status",
  "state_written": true,
  "actions": [
    {
      "type": "nudge | wrap | launch | escalate | design",
      "workspace": "string | null",
      "story_id": "string | null",
      "message": "string | null — exact text for a nudge",
      "reason": "string | null — for escalate/design"
    }
  ]
}
```

### `present(ws)` — human review surface

| Surface       | Recipe                                                                              |
| ------------- | ----------------------------------------------------------------------------------- |
| cmux (B1)     | the cmux review tab attached at the end of `spawn()` already shows the live session |
| tmux (B2)     | tell the human: `tmux attach -t <ws>` (read-only: `tmux attach -t <ws> -r`)         |
| none (B3)     | headless — review via `monitor()` signals and the PR when it lands                  |
| workflow (B4) | the `/workflows` view; plus signals + PR                                            |

### `teardown(ws)`

```bash
# Stop the session if one exists (B1/B2)
tmux kill-session -t <ws> 2>/dev/null || true
# Remove the worktree (all backends; /wrap normally owns this).
# Try a clean remove first; fall back to --force only if leftover files block it
# (don't blanket-swallow the error — a silently-skipped removal leaves an orphan).
git worktree remove .feature-workspaces/<ws> \
  || git worktree remove --force .feature-workspaces/<ws>
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
cmux tab attachable → B1: clickable review tab (sibling of the orchestrator's tab), live view
tmux present        → B2: same session + monitor loop; `tmux attach` to watch
no multiplexer      → B3: headless autonomous subagent; review via signals + PR
```

"cmux tab attachable" = the `cmux` CLI is reachable **and** a target pane resolves
(`cmux tree` shows `◀ here`, or `$CMUX_PANE_ID` is set) — it does **not** require
`$CMUX_SOCKET`. Reachable-but-no-pane degrades to B2; losing cmux costs only the
tab UI. The file-based monitor loop and mid-flight feedback survive in B2
unchanged. Skills must therefore gate every cmux call on detection and never abort
merely because cmux is absent.
