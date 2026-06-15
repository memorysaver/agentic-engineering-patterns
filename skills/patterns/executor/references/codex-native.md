# Codex Native Backends — `codex-subagent` & `codex-exec`

Per-operation recipes for the two Codex modes. Both apply to the Codex CLI
**and** the Codex desktop app — they share the same Rust runtime, and
`multi_agent` is stable and on by default from runtime 0.130.0 (no app-side
toggle). Detection and selection live in `backends.md` — read that first.

| Mode               | Mechanism                          | Lifetime                                    | Steering                 | Human gate                                          |
| ------------------ | ---------------------------------- | ------------------------------------------- | ------------------------ | --------------------------------------------------- |
| **codex-subagent** | native multi_agent (`spawn_agent`) | session-bound (dies with the parent thread) | `send_input` (push)      | native approval overlay + `needs-human.md`          |
| **codex-exec**     | headless `codex exec --cd` workers | OS-bound (independent processes)            | `codex exec resume <id>` | gate-and-park → main agent relays via `exec resume` |

---

## The worktree reality (read before choosing)

`spawn_agent` has **no cwd/worktree parameter** — subagents share the parent's
workspace and sandbox. The Codex app's own "Worktree" environment pins worktrees
under `$CODEX_HOME/worktrees` (path not configurable). So for AEP's invariant —
worktree at `.feature-workspaces/<ws>` — the binding under `codex-subagent` is a
**directory contract** (prompt + role instructions), not enforcement.

Why the contract is safe in practice: with the parent thread rooted at the repo
and `workspace-write` sandboxing, the writable boundary is the **project root** —
`.feature-workspaces/<ws>` is inside it, and so is the linked worktree's git
metadata (`.git/worktrees/...`). The worker can do all its git work in the
worktree without leaving the sandbox. When the user demands _hard_ cwd
enforcement, use `codex-exec` instead (the process cwd is the worktree).

## Custom agent roles (ship with the project)

Commit these to the project's `.codex/agents/` — they are project-scoped, so
both the CLI and the desktop app discover them in any checkout or worktree.

`.codex/agents/aep-builder.toml`:

```toml
name = "aep-builder"
description = "AEP workspace builder — implements one story inside its assigned git worktree"
developer_instructions = """
You are an AEP workspace builder. Your FIRST action is to cd into the absolute
worktree path given in your prompt (under .feature-workspaces/). You operate
EXCLUSIVELY inside that directory on its feat/<ws> branch. Never edit the main
checkout or any other worktree. Report progress through
.dev-workflow/signals/status.json at phase boundaries; read
.dev-workflow/signals/feedback.md at phase starts. If you hit a decision only
the human can make, append it to .dev-workflow/signals/needs-human.md and set
"blocked_on": "human" in status.json, then ask the parent thread.
"""
```

`.codex/agents/aep-evaluator.toml`:

```toml
name = "aep-evaluator"
description = "AEP evaluator — scores a workspace build against its criteria, never fixes"
developer_instructions = """
You are an AEP EVALUATOR. You work inside the worktree directory given in your
prompt. Read evaluator-criteria.md, the eval-request, the OpenSpec change, and
the git diff against the integration branch. Score honestly per the criteria;
apply hard failure thresholds strictly; never modify code and never modify
verification_steps. Write your findings to
.dev-workflow/signals/eval-response-<N>.md and update pass/fail fields in
.dev-workflow/feature-verification.json, then stop.
"""
```

---

## Mode: `codex-subagent`

One native subagent per story, spawned from the orchestrator's **living main
thread** (desktop thread or interactive CLI session). Session-bound: subagents
die with the parent thread, so this mode requires the orchestrator itself to be
long-lived (see the driver × backend matrix in `backends.md`).

Concurrency: `agents.max_threads` (config.toml, default 6) caps concurrent
subagents — effective WIP limit is `min(concurrency_limit, max_threads)`.

### `spawn(ws, branch, bootstrap_prompt)`

AEP creates the worktree first (common recipe in `backends.md`), then:

```
spawn_agent(
  agent_type: "aep-builder",
  message: "Worktree: <abs-repo-path>/.feature-workspaces/<ws> (branch feat/<ws>).
            <bootstrap_prompt>"
)
# record the returned agent id → state agent_id
```

### `nudge(ws, msg)`

```
send_input(agent: <agent_id>, message: <msg>)
```

Also append to `signals/feedback.md` (durable record).

### `liveness(ws)`

```
list_agents                                      # thread status
git -C .feature-workspaces/<ws> diff --stat      # corroboration
```

### `present(ws)`

- **CLI:** `/agent` switches between agent threads — tell the human which
  thread id belongs to `<ws>`.
- **Desktop app:** threads run side by side; subagent diff stats appear in the
  composer and each subagent has a stable identicon. The human clicks into the
  thread to watch or steer.

### `gate(ws)` — human gate (block-in-place, hub-and-spoke)

Two native channels, plus the file. In both, the **parent thread (main agent)
is the human's console** — the human answers there; opening the worker thread
directly is optional:

- **Approvals** (sandbox/permission requests): surface natively in the active
  thread labeled with the source thread — CLI: press `o` to open that thread
  and approve; app: contextual permission prompt, click into the owning thread.
- **Non-approval decisions** (design ambiguity, eval non-convergence): worker
  appends to `needs-human.md` + `blocked_on: human` and asks the parent thread;
  the parent asks the human in the main conversation and relays the answer via
  `send_input(<id>, "<answer>")`.

### `spawn_evaluator(ws, role)`

Use a bounded headless one-shot with **enforced** worktree cwd — review is
exactly the "bounded analysis" case `codex exec` is reserved for:

```bash
codex exec --cd "<abs>/.feature-workspaces/<ws>" \
  --dangerously-bypass-approvals-and-sandbox \
  "<evaluator prompt from agent-contracts.md, customized with the workspace paths>" < /dev/null
```

The prompt is the spawn — no sleep, no send step, no pane to kill. The exec
returns when `eval-response-<N>.md` is written.

### `teardown(ws)`

```
close_agent(agent: <agent_id>)      # if still running
# then the common worktree removal from backends.md
```

---

## Mode: `codex-exec`

One **headless `codex exec` process per story**, cwd hard-bound to the
worktree. OS-bound: workers survive the orchestrator session, and a _fresh_
session can steer them via `codex exec resume`. This is the Codex mode for
**cron/launchd-driven autopilot** (each tick is a new `codex exec` session that
cannot see another session's subagents) and for users who demand enforced
isolation.

### `spawn(ws, branch, bootstrap_prompt)`

```bash
nohup codex exec --cd ".feature-workspaces/<ws>" \
  --dangerously-bypass-approvals-and-sandbox \
  "<bootstrap_prompt>" < /dev/null > ".feature-workspaces/<ws>/.dev-workflow/worker.log" 2>&1 &
# Recover the session id from the worker log / `codex exec resume --last`
# bookkeeping and record it → state agent_id
```

### `nudge(ws, msg)`

```bash
codex exec resume <session-id> --dangerously-bypass-approvals-and-sandbox \
  "<msg>" < /dev/null
```

`resume` continues the worker's own session with the new instruction — this is
the OS-bound steering channel; it works from any later orchestrator session.
Also append to `signals/feedback.md`.

### `liveness(ws)`

```bash
cat .feature-workspaces/<ws>/.dev-workflow/signals/status.json   # primary signal
git -C .feature-workspaces/<ws> diff --stat                      # corroboration
tail -5 .feature-workspaces/<ws>/.dev-workflow/worker.log        # process output
```

### `present(ws)` / `gate(ws)` — gate-and-park

Headless — review via signals + the PR. For a gate the worker **parks**: write
`needs-human.md` + `blocked_on: human`, commit WIP, finish the run. The
orchestrator asks the human in the main session and relays the answer:

```bash
codex exec resume <session-id> --dangerously-bypass-approvals-and-sandbox \
  "The human decided: <answer>. Mark the needs-human entry resolved, clear blocked_on, and continue the /aep-build flow." < /dev/null
```

### `spawn_evaluator(ws, role)`

Same as codex-subagent: `codex exec --cd <worktree>` with the evaluator prompt.

### `teardown(ws)`

The exec process exits on its own when the build completes; nothing to kill.
Then the common worktree removal from `backends.md`.

---

## Dogfood / post-deploy validation (Codex)

Host-aware dogfood (`dogfood_method()`) for Codex resolves by mode, per
[`dogfood-validation.md`](dogfood-validation.md):

- **codex-subagent** (desktop, GPT-5.4 multimodal): use the **native in-app
  browser + computer-use** to drive the app and capture screenshots (computer-use
  is desktop-only). Fallback: the Playwright skill, then agent-browser CLI.
- **codex-exec** (headless): **write and run a Playwright script** (no computer-use
  off the desktop app). Fallback: agent-browser CLI → API/curl checks.

Screenshots feed the multimodal evaluator's Visual Design dimension
(`aep-gen-eval/references/scoring-framework.md`). Full selection + `target_url()`
resolution live in `dogfood-validation.md`.
