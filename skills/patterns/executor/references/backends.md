# Executor Backends

Detection, mode selection, and the cross-backend protocols that make
`/aep-launch`, `/aep-build`, `/aep-autopilot`, and `/aep-wrap` host-agnostic. Read this before
spawning or steering any workspace agent. Per-operation recipes live in three
sibling files:

| Recipe file                            | Modes                             |
| -------------------------------------- | --------------------------------- |
| [`claude-native.md`](claude-native.md) | `native-bg-subagent`, `claude-bg` |
| [`codex-native.md`](codex-native.md)   | `codex-subagent`, `codex-exec`    |
| [`tmux-session.md`](tmux-session.md)   | `legacy` (tmux + optional cmux)   |

---

## Table of Contents

1. [The Mode Matrix](#the-mode-matrix)
2. [Detection](#detection)
3. [Mode Selection](#mode-selection)
4. [Driver × Backend Compatibility](#driver--backend-compatibility)
5. [Common Recipes (all modes)](#common-recipes-all-modes)
6. [The Human-Gate Protocol](#the-human-gate-protocol)
7. [Mode: workflow (dynamic-workflow fan-out)](#mode-workflow-dynamic-workflow-fan-out)
8. [Orphan Re-adoption](#orphan-re-adoption)
9. [The Worktree-Context Constraint](#the-worktree-context-constraint)
10. [Legacy B1–B4 Mapping](#legacy-b1b4-mapping)

---

## The Mode Matrix

Native modes come first; tmux is the explicit-pin / generic-host fallback.
**Lifetime** is the axis that matters for orchestration: _session-bound_
workers (native-bg-subagents, Codex subagents) die with the orchestrator session;
_OS-bound_ workers (bg sessions, exec processes, tmux sessions) survive it.

| Mode                   | Backend                                 | Lifetime      | Spawn                                                                        | Nudge                                                        | Human gate                                                                | Present                            |
| ---------------------- | --------------------------------------- | ------------- | ---------------------------------------------------------------------------- | ------------------------------------------------------------ | ------------------------------------------------------------------------- | ---------------------------------- |
| **native-bg-subagent** | Agent tool `run_in_background`, no team | session-bound | Agent tool `run_in_background: true`, **no `team_name`**, **no active team** | `feedback.md` (pull); `SendMessage(to: agentId)` best-effort | gate-and-park → main agent (re-spawn w/ answer)                           | `TaskOutput` / JSONL `output_file` |
| **claude-bg**          | native background sessions              | OS-bound      | `cd <worktree> && claude --bg` _(only if `BG_AVAILABLE`; see note)_          | `feedback.md` (pull); stop/respawn if hard-stuck             | gate-and-park → main agent (resume w/ answer); `claude attach` optional   | `claude attach` / `claude logs`    |
| **codex-subagent**     | native multi_agent                      | session-bound | `spawn_agent(role=aep-builder)`                                              | `send_input` (push)                                          | approval overlay + `needs-human.md`                                       | `/agent` (CLI) / thread list (app) |
| **codex-exec**         | headless exec workers                   | OS-bound      | `codex exec --cd <worktree>` (bg process)                                    | `codex exec resume <id>`                                     | gate-and-park → main agent (`exec resume` w/ answer)                      | signals + PR                       |
| **legacy**             | tmux session (+ cmux tab)               | OS-bound      | `tmux new-session`                                                           | `tmux send-keys`                                             | `needs-human.md` + `tmux attach`                                          | cmux tab / `tmux attach`           |
| **workflow**           | CC dynamic workflow fan-out             | session-bound | Workflow tool pipeline                                                       | none mid-stage (steer at stage boundaries)                   | gate-and-park → main agent (structured `gated` result + `needs-human.md`) | `/workflows` + signals             |
| **headless**           | one-shot native subagent                | session-bound | Task/Agent tool, worktree-bound                                              | none                                                         | gate-and-park → main agent (re-spawn w/ answer)                           | signals + PR                       |

> **`claude-team` was removed (2026-06).** On Claude Code ≥ 2.1.x the agent-teams
> spawn path fails **silently**: the teams runtime pastes the long
> `claude … --agent-id <name>@<team> --settings '<big JSON>'` launch command into
> a detached `claude-swarm-<pid>` tmux pane, the `--settings` JSON is **truncated
> mid-string and never submitted**, so no worker process ever starts — yet the
> team roster still lists the member as "active". A live team also **poisons
> teamless background spawns** (they auto-route through the same broken tmux
> backend). `native-bg-subagent` replaces it as the Claude Code default. See
> `docs/decisions/remove-claude-team.md`.

**Announce the selection.** Before spawning, state which mode and why — e.g.
"Claude Code → `native-bg-subagent`: in-process background subagent (Agent tool,
`run_in_background`, no team); pull-steer via `feedback.md`; verified live by the
post-spawn liveness probe."

> **`native-bg-subagent` success signature.** A working spawn returns a
> **bare-hex `agentId`** (e.g. `adfb6cb206155a92e`) with a JSONL `output_file`,
> **not** an `@<team>` id. It is non-blocking and auto-notifies on completion.
> (A foreground in-process subagent also works but blocks the orchestrator turn.)
> **Pre-spawn:** if any team is active, `TeamDelete` it first — a live team
> re-routes teamless spawns into the broken agent-teams tmux backend.

---

## Detection

`detect()` resolves the **host**, its two **executor commands**, the **native
capabilities**, any **explicit pin**, and the **presentation surface**.

```bash
# --- HOST + executor commands ---
#   $EXECUTOR       interactive session (stays alive — legacy/tmux, evaluator panes)
#   $EXECUTOR_EXEC  headless one-shot   (runs the given prompt to completion, exits)
if [ -n "$CLAUDECODE" ]; then
  HOST=claude
  EXECUTOR="claude --dangerously-skip-permissions"            # interactive; NO -p
  EXECUTOR_EXEC="claude -p --dangerously-skip-permissions"    # -p/--print = non-interactive
  READY_GREP='❯'
elif command -v codex >/dev/null 2>&1 && { [ -n "$CODEX_HOME" ] || env | grep -q '^CODEX_'; }; then
  HOST=codex
  EXECUTOR="codex --dangerously-bypass-approvals-and-sandbox"
  EXECUTOR_EXEC="codex exec --dangerously-bypass-approvals-and-sandbox"
  READY_GREP=''
else
  HOST=generic
  EXECUTOR="${AEP_EXECUTOR:-}"; EXECUTOR_EXEC="${AEP_EXECUTOR_EXEC:-$EXECUTOR}"; READY_GREP=''
fi
[ -z "$EXECUTOR" ] && { echo "executor unresolved — set \$AEP_EXECUTOR or run under Claude Code / Codex"; }

# --- NATIVE CAPABILITIES ---
# NOTE: agent-teams (the old TEAMS_AVAILABLE / CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS
# gate) is NO LONGER consulted — claude-team was removed (silent spawn failure;
# see the mode-matrix note). Do not select a mode from the teams flag alone.
BG_AVAILABLE=$([ "$HOST" = claude ] && claude --help 2>/dev/null | grep -q -- '--bg' && echo yes || echo no)
# ^ On Claude Code ≥ 2.1.x the one-shot `claude --bg` spawn flag was REMOVED
#   (background agents are now the interactive `claude agents` view, not a
#   scriptable flag). On such builds BG_AVAILABLE=no and claude-bg is skipped —
#   the Claude Code default is native-bg-subagent (session-bound). If you need an
#   OS-bound Claude worker for a cron/launchd driver and `--bg` is gone, that
#   driver is unsupported on Claude Code — use Codex codex-exec or a long-lived
#   in-session goal/loop driver instead.
MULTI_AGENT_AVAILABLE=$([ "$HOST" = codex ] && codex features list 2>/dev/null | grep -q 'multi_agent.*true' && echo yes || echo no)
WORKFLOW_CAPABLE=$([ "$HOST" = claude ] && echo yes || echo no)   # the host agent knows it has the Workflow tool

# --- EXPLICIT PIN (the single manual lever besides "…with workflow") ---
PIN=$(git config --get aep.executor-backend 2>/dev/null || true)   # e.g. "tmux"

# --- PRESENTATION (for legacy mode): cmux needs a reachable CLI + a target pane,
#     NOT $CMUX_SOCKET (the CLI drives cmux over its socket even when unset) ---
CMUX="$(command -v cmux || echo /Applications/cmux.app/Contents/Resources/bin/cmux)"
if [ -x "$CMUX" ] && { "$CMUX" tree 2>/dev/null | grep -q '◀ here' || [ -n "$CMUX_PANE_ID" ]; }; then
  PRESENT=cmux
elif command -v tmux >/dev/null 2>&1; then
  PRESENT=tmux
else
  PRESENT=none
fi
```

> **Correct CLI invocations (verified against Claude Code 2.1.161+ / Codex 0.130.0):**
>
> |            | interactive session → `$EXECUTOR`                  | headless one-shot → `$EXECUTOR_EXEC`                    |
> | ---------- | -------------------------------------------------- | ------------------------------------------------------- |
> | **claude** | `claude --dangerously-skip-permissions`            | `claude -p --dangerously-skip-permissions`              |
> | **codex**  | `codex --dangerously-bypass-approvals-and-sandbox` | `codex exec --dangerously-bypass-approvals-and-sandbox` |
>
> `--rc` is **not** a real Claude Code flag. Codex's full-bypass flag is
> `--dangerously-bypass-approvals-and-sandbox` (no `--yolo` / `--full-auto`).

**Orchestrator lifetime is not shell-probable — the agent knows it.** You are
_long-lived_ when you're an interactive session or a `/loop`-driven session
(Claude Code) or a living Codex main thread (desktop or interactive CLI). You
are _ephemeral_ when this invocation is a cron/launchd-spawned one-shot (e.g. a
scheduled `codex exec` autopilot tick). Session-bound modes require a
long-lived orchestrator.

---

## Mode Selection

Apply in order; first match wins. `workflow` is the only natural-language
opt-in; `legacy` is the only pin.

```
workflow            IF user explicitly opted in ("…with workflow") AND WORKFLOW_CAPABLE
legacy              IF PIN == tmux, or user said "…with tmux"
native-bg-subagent  ELIF HOST == claude AND orchestrator is long-lived   # default on Claude Code
claude-bg           ELIF HOST == claude AND BG_AVAILABLE                  # OS-bound (cron); only if `claude --bg` exists
codex-subagent      ELIF HOST == codex AND MULTI_AGENT_AVAILABLE AND orchestrator is a living main thread
codex-exec          ELIF HOST == codex
legacy              ELIF PRESENT == cmux or PRESENT == tmux               (generic hosts)
headless            ELSE
```

**Behavior change (2026-06): `claude-team` removed.** The agent-teams spawn path
fails silently on Claude Code ≥ 2.1.x (see the mode-matrix note), so it is no
longer selectable. The Claude Code default is now **native-bg-subagent** (Agent
tool, `run_in_background`, no team). The teams env flag is ignored. There is no
"…with agent team" opt-in — the backend is broken, not merely de-prioritized.

**Behavior change vs v1.x:** Claude Code with tmux installed no longer
auto-selects tmux — native modes win. Users who want the tmux+cmux workflow
back pin it: `git config aep.executor-backend tmux`.

> **Post-spawn liveness probe is mandatory (never trust a flag or roster).**
> Selecting a mode does NOT mean its spawn worked. After ANY spawn, before
> declaring the worker "running", run the probe in
> [Post-Spawn Liveness Probe](#post-spawn-liveness-probe). On failure, tear the
> dead spawn down and **auto-fall-back to `native-bg-subagent`**.

---

## Driver × Backend Compatibility

An orchestrator (notably `/aep-autopilot`) is driven either by a **long-lived
session** or by a **cron/launchd scheduler** that starts a fresh session per
tick. The long-lived class has two in-session variants — the **goal driver**
(`/goal`, native on both hosts, the autopilot default, self-terminating per
layer) and the fixed-interval **loop driver** (`/loop`; a living Codex main
thread ticking in-thread). Both are equally compatible with every steerable
mode. Session-bound workers cannot outlive their parent, so:

| Driver                                                          | native-bg-subagent                   | claude-bg                         | codex-subagent                          | codex-exec                                 | legacy |
| --------------------------------------------------------------- | ------------------------------------ | --------------------------------- | --------------------------------------- | ------------------------------------------ | ------ |
| Long-lived session (`/goal` **or** `/loop`, living main thread) | ✅                                   | ✅                                | ✅                                      | ✅                                         | ✅     |
| Cron/launchd (fresh session per tick)                           | ❌ bg subagent dies with the session | ✅ OS-level, any session attaches | ❌ subagents invisible to a new session | ✅ `codex exec resume` works cross-process | ✅     |

A consumer that needs steering (autopilot) must pick a mode compatible with its
driver: on Claude Code, `/goal` (or `/loop`) + `native-bg-subagent` (or
`claude-bg` where `--bg` exists); on Codex, in-thread `/goal` (or manual ticks)

- `codex-subagent`, or cron ticks

* `codex-exec`. **The goal driver is in-session only** — it cannot drive a
  fresh-session-per-tick scheduler, so the cron/launchd row is always the
  `/loop`/`codex exec` path.

---

## Common Recipes (all modes)

### Worktree creation — always AEP-owned, before any spawn

```bash
# Resolve $BASE (integration branch) — see git-ref "Integration Branch" (override → develop → main)
BASE=$(git config --get aep.integration-branch 2>/dev/null || true)
[ -z "$BASE" ] && { git show-ref --verify --quiet refs/heads/develop \
  || git show-ref --verify --quiet refs/remotes/origin/develop; } && BASE=develop
BASE=${BASE:-main}

mkdir -p .feature-workspaces
git worktree add -b feat/<ws> .feature-workspaces/<ws> "$BASE"
```

Every mode points its worker at this directory. OS-bound modes bind by process
cwd (enforced); session-bound native modes bind by prompt contract (AEP's
no-hooks decision: rely on model capability + skill instructions — do not
install WorktreeCreate hooks or redirect host-managed worktree paths).

### `monitor(ws)` — host-independent, never changes

```bash
cat .feature-workspaces/<ws>/.dev-workflow/signals/status.json
ls  .feature-workspaces/<ws>/.dev-workflow/signals/ready-for-review.flag 2>/dev/null
ls  .feature-workspaces/<ws>/.dev-workflow/signals/needs-human.md 2>/dev/null
```

Mid-flight feedback is written the same way for every mode (push channels are
an acceleration layer, not a replacement — the file is the durable record):

```bash
cat >> .feature-workspaces/<ws>/.dev-workflow/signals/feedback.md <<'EOF'
## <date> <time>
Priority: high
<feedback>
EOF
```

### Worktree removal — `teardown()` tail, all modes

```bash
# (mode-specific session/agent teardown first — see the recipe files)
git worktree remove .feature-workspaces/<ws> \
  || git worktree remove --force .feature-workspaces/<ws>
git worktree prune
```

### Post-Spawn Liveness Probe

**Run after EVERY spawn, before declaring the worker "running".** A spawn call
returning, a flag being set, or a roster/state entry saying "active" is **NOT**
evidence the worker started. (The removed `claude-team` mode failed exactly here:
the launch command was truncated in a tmux pane and never submitted, yet the team
roster still showed the member "active" — a silently dead worktree the autopilot
only flagged 30+ minutes later.)

A worker is **live** only if BOTH hold within `N` seconds (default 90):

1. **Process / agent exists** — a real worker is running:
   - native-bg-subagent: the bg agent appears in `TaskList` AND its spawn returned
     a **bare-hex `agentId`** with a JSONL `output_file` (not an `@<team>` id)
   - claude-bg: `claude agents --json` shows the session `running`
   - codex-subagent: `list_agents` shows `<agent_id>`
   - codex-exec: the `codex exec` PID is alive
   - legacy: `pane_current_command` is `claude` (not `zsh`) — a `zsh` pane means the
     launch command never submitted
2. **Worktree shows activity** — `.dev-workflow/signals/status.json` was written
   **OR** `git -C .feature-workspaces/<ws> diff --stat` is non-empty.

```bash
bash .claude/skills/aep-executor/scripts/spawn-liveness-probe.sh <ws> <agent_id> [N]
# exit 0 = live; exit 1 = dead spawn (probe failed)
```

**On probe failure (dead spawn):**

1. Tear down the dead remnant — kill the stuck pane/process, and if a team got
   created during the attempt, `TeamDelete` it (a live team poisons the fallback).
2. Do **not** mark the story failed and do **not** leave the worktree for the
   autopilot to time-out on later.
3. **Auto-fall-back to `native-bg-subagent`** (Agent tool, `run_in_background`,
   no team) into the **existing** worktree, then probe again. native-bg-subagent
   is the terminal fallback — if it also fails the probe, only then escalate.

This contract is what `/aep-launch` Step 4 and the autopilot orphan/stuck checks
both consume — "roster/state says active" is never accepted as liveness.

### `check(prompt, schema)` — cheap, context-isolated analysis

Run a **read-only analysis** prompt in a throwaway, cheap-model agent and
return its structured JSON. The point is context isolation: the verbose
reading (state file + every workspace `signals/`, `gh pr view`, …) happens in
the cheap agent's own context — only the small JSON result crosses back. The
check **never reads workspace code** (signals only).

**Claude Code — Haiku subagent:**

```
Agent/Task tool: model haiku; tools Read, Bash, Glob;
prompt: <the analysis prompt; OUTPUT ONLY the JSON in `schema`>
```

**Codex — `codex exec` cheap one-shot:**

```bash
codex exec -m gpt-5.4-mini -c model_reasoning_effort=low \
  -C "$PWD" --skip-git-repo-check --dangerously-bypass-approvals-and-sandbox \
  --output-schema /tmp/aep-check.schema.json -o /tmp/aep-check.out.json \
  "<the analysis prompt>" < /dev/null
jq . /tmp/aep-check.out.json
```

**Result schema (the CHECK → ACT contract):**

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

---

## The Human-Gate Protocol

A worker mid-build can hit a decision only the human can make (design
ambiguity, eval non-convergence, Phase 11.5 manual QA). The record is
host-agnostic; the transport is per-mode — but the **canonical human console
is the main agent** (hub-and-spoke): the worker's question flows back to the
orchestrator, the orchestrator asks the human (AskUserQuestion / plain text in
the main session), and relays the answer to the worker. The human never _has_
to visit a worker's surface; per-mode direct interaction (`TaskOutput`,
`claude attach`, Codex thread, `tmux attach`) is an optional convenience.

**The record (always, every mode):** the worker appends to
`.dev-workflow/signals/needs-human.md` and sets `"blocked_on": "human"` in
`status.json`:

```markdown
## <ISO8601> — <phase>

**Question:** <the decision needed, with the options considered>
**Context:** <why the worker can't decide autonomously>
```

**Two gate styles.** The worker's behavior after recording the gate depends on
whether its mode has a push channel back into it:

- **Block-in-place** (steerable modes — codex-subagent, legacy):
  the worker raises the gate, keeps doing whatever doesn't depend on the
  answer, and waits. The answer arrives on the mode's push transport.
- **Gate-and-park** (batch/pull modes — native-bg-subagent, workflow, headless,
  codex-exec, and claude-bg): there is no push channel into a running worker, so the worker
  **parks**: commit WIP (or leave the tree clean), update `status.json`
  (`blocked_on: "human"`, current phase), then **end its run cleanly**. The
  orchestrator detects the gate, gets the human's answer, and **resumes a
  worker into the same worktree** — the same recipe as orphan re-adoption,
  with the answer prepended: "The human decided: <answer>. Run
  `bash .dev-workflow/init.sh` to recover state, mark the needs-human entry
  resolved, then continue the /aep-build flow." Parking is cheap because all
  state lives in the worktree + `.dev-workflow/`, not in the agent's context.

**The transport (per mode):**

| Mode               | Style          | Worker raises it via                                                    | Main agent relays the human's answer via                                                             | Optional direct surface            |
| ------------------ | -------------- | ----------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- | ---------------------------------- |
| native-bg-subagent | gate-and-park  | the file (orchestrator detects on next tick)                            | re-spawn a bg subagent into the same worktree with recovery bootstrap + answer                       | `TaskOutput` while it runs         |
| claude-bg          | gate-and-park  | the file (orchestrator detects on next tick)                            | resume the session with the answer (`claude -r <id> ...`), or respawn w/ recovery bootstrap + answer | `claude attach <id>` while it runs |
| codex-subagent     | block-in-place | native approval overlay (approvals) / ask the parent thread (decisions) | `send_input(<id>, "<answer>")`                                                                       | open the thread (`o` / app click)  |
| codex-exec         | gate-and-park  | the file (orchestrator detects on next tick)                            | `codex exec resume <id> "<answer>"`                                                                  | — (headless)                       |
| workflow           | gate-and-park  | stage returns a structured `gated` result + the file                    | continuation run for gated stories with the answer in the prompt (see Mode: workflow)                | — (batch)                          |
| headless           | gate-and-park  | the file; the one-shot subagent returns with a gated result             | re-spawn a one-shot into the same worktree with recovery bootstrap + answer                          | — (one-shot)                       |
| legacy             | block-in-place | the file                                                                | `executor.nudge()` (`tmux send-keys`) or `feedback.md`                                               | `tmux attach -t <ws>`              |

**Resolution:** after acting on the answer, the worker appends
`resolved: <summary>` under its entry and clears `blocked_on`. The autopilot
escalation queue consumes the same file — an unresolved `needs-human.md` entry
becomes an escalation whose `expected_human_action` is "answer in the main
session" plus the mode-specific relay recipe above. A parked workspace counts
as **waiting, not stuck and not failed**.

---

## Mode: workflow (dynamic-workflow fan-out)

> This mode is the **narrow** use of dynamic workflows — running one dispatched
> build wave as a fan-out. For the general dynamic-workflow pattern catalog
> (classify-route, fan-out/synthesize, adversarial verify, generate-filter,
> tournament, loop-until-done) and the judgment of _when a task warrants a workflow
> at all_, see [`/aep-workflow`](../../workflow/SKILL.md).

Claude Code's Workflow tool builds a whole dispatched wave as one deterministic
fan-out: one build agent per locked story, each with per-agent worktree
isolation, with `/aep-dispatch` authoring the script (the "…with workflow"
path bypasses `/aep-launch`). With hub-and-spoke gating this is a complete
backend, not just a fire-and-forget batch: gates park and return to the main
agent for confirmation, then gated stories resume.

```js
// sketch — one agent per story; gates surface in the structured result.
// `stories` is the dispatched wave: { change, worktree, bootstrap } per item.
const BUILD_RESULT = {
  type: "object",
  properties: {
    status: { enum: ["completed", "gated", "failed"] },
    question: { type: "string" }, // set when status == "gated" (mirror of needs-human.md)
    summary: { type: "string" },
  },
  required: ["status"],
};
const results = await pipeline(
  stories,
  (s) =>
    agent(
      `You operate EXCLUSIVELY in ${s.worktree}. Run /aep-build for OpenSpec change ${s.change}. ${s.bootstrap}
       If you hit a decision only the human can make: append it to .dev-workflow/signals/needs-human.md,
       set blocked_on:"human" in status.json, commit WIP, and RETURN status "gated" with the question —
       do not guess and do not wait.`,
      { phase: "Build", schema: BUILD_RESULT },
    ),
  (built, s) =>
    built.status === "completed"
      ? agent(`Adversarially verify the build for ${s.change} in ${s.worktree}.`, {
          phase: "Verify",
          schema: BUILD_RESULT,
        })
      : built,
);
return results;
```

**Gate handling (main agent, after the workflow returns):** collect
`status: "gated"` items, ask the human each `question` (AskUserQuestion), then
resume each gated story into its **existing** worktree — either a continuation
workflow over the gated subset or individual re-launches — with the recovery
bootstrap + the answer ("The human decided: <answer>…"). `Workflow
resumeFromRunId` makes the continuation cheap (completed agents return from
cache). Mid-stage there is still no push nudge — steering happens at stage
boundaries and through gates.

`monitor()` is unchanged (the build agents still write signals); progress is
also visible in the `/workflows` view. Autopilot does not drive this mode —
the workflow is its own orchestrator; its gates surface to the main agent that
authored it.

> AEP-created worktrees vs `isolation: 'worktree'`: prefer creating the
> `.feature-workspaces/<ws>` worktrees first (launch guardrails apply) and
> passing the path in the prompt, so `monitor()`/wrap paths stay standard. The
> Workflow tool's own `isolation: 'worktree'` puts agents in host-managed
> paths — acceptable for ad-hoc batches, but then signals live outside
> `.feature-workspaces/` and `/aep-wrap` does not apply.

---

## Orphan Re-adoption

Session-bound workers (native-bg-subagents, Codex subagents) die when the
orchestrator session dies, but their **work does not** — it lives in the worktree
and `.dev-workflow/`. When an orchestrator (re)starts and finds state claiming an
active workspace, **decide orphan-vs-live by the real-liveness probe, never by
roster/state membership** (a roster can show a never-started worker as "active" —
the `claude-team` failure mode). Treat as an orphan when the
[Post-Spawn Liveness Probe](#post-spawn-liveness-probe) fails — the agent process
is gone (TaskList / `list_agents` / `claude agents` empty) **or** the worktree
shows no activity:

1. Treat it as an **orphan, not a failure** — do not mark the story failed.
2. Read `signals/status.json` for the last known phase.
3. Re-launch a worker **into the existing worktree** with the current mode's
   spawn recipe and a recovery bootstrap:
   "Run `bash .dev-workflow/init.sh` to recover state, read
   `.dev-workflow/signals/feedback.md`, then continue the /aep-build flow from the
   current phase."
4. Record the new `agent_id` in orchestrator state.

This is why worker progress must always flow through signals + commits, never
live only in an agent's context.

---

## The Worktree-Context Constraint

**Every spawned worker and evaluator MUST be bound to the workspace worktree** —
by process cwd (claude-bg, codex-exec, legacy, evaluator execs) or by prompt
contract (native-bg-subagent, codex-subagent, headless).

This is not optional. The autopilot orchestrator boundary forbids spawning a
reviewer/agent "from main" precisely because such an agent lacks the
workspace's files, git state, and eval history. Binding the spawned agent to
the worktree gives it exactly that context, so the boundary's intent is
satisfied under every mode. The gen/eval separation (generator ≠ evaluator)
and the rule that the main session never reads workspace code both still hold —
only the spawn mechanism changes.

**Codex caveat:** `spawn_agent` has no cwd parameter, so the codex-subagent
binding is a directory contract hardened by the `aep-builder` role's
`developer_instructions` (see `codex-native.md`). The contract stays inside the
`workspace-write` sandbox because `.feature-workspaces/` is under the project
root. Hard enforcement is available via `codex-exec`.

---

## Legacy B1–B4 Mapping

For readers of v1.2–v1.5 docs and ADRs:

| Old                   | New                                                       |
| --------------------- | --------------------------------------------------------- |
| B1 (tmux + cmux tab)  | `legacy` with cmux present                                |
| B2 (tmux only)        | `legacy`                                                  |
| B3 (native subagent)  | `codex-subagent` (Codex) / `headless` (one-shot fallback) |
| B4 (dynamic workflow) | `workflow`                                                |

New in v1.6: `claude-bg`, `codex-exec`, the human-gate protocol, and orphan
re-adoption. See `docs/decisions/native-first-executor.md`.

**Removed 2026-06:** `claude-team` (silent agent-teams spawn failure) — replaced
by **`native-bg-subagent`** as the Claude Code default, plus the mandatory
[Post-Spawn Liveness Probe](#post-spawn-liveness-probe). See
`docs/decisions/remove-claude-team.md`.
