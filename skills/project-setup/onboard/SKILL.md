---
name: aep-onboard
description: >-
  Installs, verifies, and explains AEP itself: environment, tools, plugin
  config. Use for first contact with AEP; creating a project is
  /aep-scaffold.
---

# Onboard

Set up your environment for agentic TypeScript development and get oriented to how AEP thinks. Phase 0 points you to the mental-model tour; Phases 1–5 install AEP, verify tools, and configure AEP's project guardrails. Run once on first setup — returning users re-verifying their environment can skip Phase 0 and start at Phase 1.

---

## Phase 0 — Orient Yourself (first-timers only)

> **Returning user?** If you've run `/aep-onboard` before and are just re-verifying your environment, skip to Phase 1.

AEP is not a command runner — it's a workflow that separates _thinking_ (what to build, decided with the AI on the **control plane**) from _doing_ (agents building to precise specs on the **execution plane**), communicating only through structured artifacts (`product-context.yaml`, signal files in `.dev-workflow/signals/`) rather than shared code context. Installing the tools without that model leaves you staring at a blank terminal unsure which skill to run first.

Read **[the installed orientation guide](references/orientation.md)** — the self-contained 10-minute first-hour tour: the three mental models (control vs execution plane, the Jeff Patton story map, the two-session main/workspace model), what every skill does, and the four concrete paths (new product / existing project / single feature / hands-free).

**Done when:** you've read orientation.md and can name which of the four paths in "Next Steps — Pick Your Path" (below) matches your situation. Then continue to Phase 1.

---

## Phase 1 — Install the Plugin

Install AEP v4.1.0 with the [`skills`](https://github.com/vercel-labs/skills) CLI at **project level**, once per agent your project uses. Commit the installed files so the version is frozen for your team:

```bash
# Run the command for each agent runtime this project uses.
npx -y skills@1.5.17 add memorysaver/agentic-engineering-patterns@v4.1.0 -a claude-code --skill '*' -y
npx -y skills@1.5.17 add memorysaver/agentic-engineering-patterns@v4.1.0 -a codex        --skill '*' -y
```

This installs every AEP skill (the `aep-*` names) plus a `skills-lock.json` manifest — **commit both**. The v4 layout is plain per-agent copies; shared or canonical symlink layouts are legacy compatibility only. The committed skill bytes are the durable project pin; upgrade intentionally by rerunning the same command with the desired tag, then review and commit the resulting skill and lockfile diff.

Phase 1 installs only AEP and adds no companion project-level skill bundle. AEP's project record is self-contained: `/aep-build` captures lessons in `.dev-workflow/lessons.md`, `/aep-wrap` archives them in `lessons-learned/`, and `/aep-launch` recalls them. Host memory, when available, is only an accelerator over that repository record.

> **Note:** Baseline onboarding installs AEP only. Optional third-party integrations are documented in [references/plugins.md](references/plugins.md) and stay outside this setup unless the user requests them separately.

---

## Phase 1.5 — Wire the Agent Instruction Files

The instruction files are versioned with AEP. Three templates ship with this skill under `templates/`: `AGENTS.md.tmpl` (the generic working agreement, the `## AEP Workflow` section with the pinned release and the `/aep-easy-explain` register, and a `## Project Context` pointer to `project-convention/`), `CLAUDE.md.tmpl` (`@AGENTS.md`), and `project-convention/README.md.tmpl` (the convention index: how the project runs on AEP, the `docs/` routing table, the directories AEP owns, how to add a convention). The first line of `AGENTS.md` is the marker `<!-- aep-agents-template: vX.Y.Z -->`; `/aep-scaffold`'s audit reads it, and [references/migrations.md](references/migrations.md) keys its upgrade steps to it.

- **No `AGENTS.md` yet:** copy the three templates into place as-is (`project-convention/README.md` at the repository root) and commit them with the install.
- **Existing `AGENTS.md`:** read its marker (none ⇒ pre-v4.1.0) and apply every section of `references/migrations.md` between it and the pinned release, in order. Project-specific content moves into `project-convention/<topic>.md`, one file per topic, linked from the README; nothing project-specific stays in `AGENTS.md`. A hand-authored `CLAUDE.md` is merged into `AGENTS.md` by hand before it becomes `@AGENTS.md`.
- **Codex-only repo:** skip `CLAUDE.md`.

**Verify:** `head -1 AGENTS.md` prints the marker for the pinned release; `head -1 CLAUDE.md` prints `@AGENTS.md` where Claude Code is installed; `project-convention/README.md` exists.

---

## Phase 2 — Verify Required Tools

Each tool below earns its place in the agentic workflow — `git` provides version control and worktrees (one isolated working tree per parallel agent), Node/npm run the pinned installer and OpenSpec, `bun` runs the TypeScript monorepo, `openspec` powers spec-driven development, an **executor** (`claude` _or_ `codex`) runs the implementation agents, and `gh` publishes PRs. Claude Code projects also need `jq` for AEP's concurrency hooks. `tmux` is **optional**: launches are native-first (see `/aep-executor`); tmux only hosts the pinned **legacy** mode and the generic-host fallback.

Run this check:

```bash
# Required: at least one executor (claude OR codex)
command -v claude >/dev/null 2>&1 || command -v codex >/dev/null 2>&1 \
  && echo "executor:      OK" || echo "executor:      MISSING (install claude or codex)"

# Required: everything else
for cmd in node npm bun git gh openspec; do
  printf "%-15s" "$cmd:"
  command -v "$cmd" >/dev/null 2>&1 && echo "OK ($(command -v "$cmd"))" || echo "MISSING"
done
node -e 'const [M,m]=process.versions.node.split(".").map(Number); process.exit(M>20||(M===20&&m>=19)?0:1)' \
  && echo "node version:   OK (>=20.19)" || echo "node version:   TOO OLD (need >=20.19)"

# Required only when Claude Code skills are installed: concurrency hooks parse tool input with jq
if [ -e .claude/skills/aep-onboard/SKILL.md ]; then
  printf "%-15s" "jq:"
  command -v jq >/dev/null 2>&1 && echo "OK ($(command -v jq))" || echo "MISSING (required for Claude hooks)"
fi

# Optional (legacy/pinned-tmux mode only): tmux
printf "%-15s" "tmux:"
command -v tmux >/dev/null 2>&1 && echo "OK ($(command -v tmux))" || echo "MISSING (optional — only the legacy launch mode needs it)"
```

Install any missing tools:

| Tool       | Purpose                                       | Install                                                  |
| ---------- | --------------------------------------------- | -------------------------------------------------------- |
| `node/npm` | Pinned skills installer + OpenSpec runtime    | Node >= 20.19 via the platform's preferred version tool  |
| `git`      | Version control + worktrees                   | `xcode-select --install` (macOS)                         |
| `bun`      | Package manager & runtime                     | `curl -fsSL https://bun.sh/install \| bash`              |
| `claude`   | Executor: Claude Code CLI                     | `npm install -g @anthropic-ai/claude-code`               |
| `codex`    | Executor: OpenAI Codex CLI                    | `npm install -g @openai/codex` _(alt to claude)_         |
| `gh`       | GitHub CLI for PRs                            | `brew install gh`                                        |
| `openspec` | Spec-driven development                       | `npm install -g @fission-ai/openspec@latest`             |
| `jq`       | Claude Code concurrency-hook JSON parsing     | `brew install jq` / `apt-get install jq` _(Claude only)_ |
| `tmux`     | Terminal multiplexer (optional — legacy mode) | `brew install tmux`                                      |

All **required** tools (Node >=20.19, npm, executor, `bun`/`git`/`gh`/`openspec`, plus `jq` for a Claude install) must show OK before proceeding. You need **at least one executor** (claude or codex) — not both. `tmux` may be MISSING; that's fine — launches are native-first.

> **Native-first launches:** the executor abstraction picks the host's native mode automatically (Claude Code background subagents/sessions, or Codex native subagents/exec workers) with live monitoring and steering, no tmux required — which is why `tmux` may show MISSING. See `/aep-executor`.

> **Note on parallelism:** Each parallel feature agent runs in its own `git worktree` at `.feature-workspaces/<name>/` on its own `feat/<name>` branch. Worktrees share the underlying `.git/objects` (no history duplication) but each adds one full working-tree copy on disk — budget accordingly when running many agents in parallel.

---

## Phase 3 — Verify Optional Tools

```bash
for cmd in cmux agent-browser portless; do
  printf "%-15s" "$cmd:"
  which $cmd >/dev/null 2>&1 && echo "OK ($(which $cmd))" || echo "MISSING (optional)"
done
```

| Tool            | Purpose                                                                                                        | Install                                           |
| --------------- | -------------------------------------------------------------------------------------------------------------- | ------------------------------------------------- |
| `cmux`          | Clickable tab multiplexer for watching legacy-mode tmux sessions — **optional**; only used when tmux is pinned | `bun add -g cmux`                                 |
| `agent-browser` | Browser automation testing                                                                                     | Claude Code plugin: `agent-browser@agent-browser` |
| `portless`      | Port management (.localhost)                                                                                   | `bun add -g portless`                             |

> **cmux is a convenience, not a requirement.** It only adds clickable tabs for
> watching legacy-mode tmux sessions (when `aep.executor-backend tmux` is
> pinned). Without it, pinned workspaces still run in tmux with the full
> monitor + mid-flight-feedback loop — attach with `tmux attach -t <name>`.
> Skills auto-detect cmux and never abort when it's absent. See `/aep-executor`.

These are optional — the workflow works without them but is enhanced by them. On macOS, enable `agent-browser` once a one-command smoke test can launch a page without crashing Chrome:

```bash
agent-browser navigate about:blank
```

If macOS shows a Google Chrome crash report with `_RegisterApplication`, `TransformProcessType`, or `abort() called`, leave `agent-browser` disabled and use non-browser checks (`curl`, unit tests, screenshots from the user, or the host agent's browser tool) until the local Chrome/agent-browser combination is healthy.

---

## Phase 4 — Configure AEP Project Guardrails (Claude Code only)

If this project has no `.claude/skills/aep-*` install, skip this phase. Codex enforces the workflow through its installed AEP instructions and native agent roles; it does not need `.claude/settings.json`.

For Claude Code projects, install AEP's two concurrency hooks. They keep parallel workspace agents from corrupting `product-context.yaml`. Optional third-party plugins are outside baseline onboarding; AEP's core workflow and fallback paths operate without them.

### What to write

Read `.claude/settings.json` if it exists, then merge the `hooks.PreToolUse` entries from **[references/settings-template.json](references/settings-template.json)** into it (create the file from the template if it's missing). Preserve every existing setting and hook.

**Verify** the two hooks landed:

```bash
jq '[.hooks.PreToolUse[].matcher]' .claude/settings.json
# expect an array containing both "Edit|Write" and "Bash"
```

> **Concurrency protocol hooks:** they block a workspace agent from editing, writing, or committing `product-context.yaml` and redirect it to signal files. This is defense-in-depth — the skill instructions also direct agents to use signals, but the hook catches model drift. Only the main session (`/aep-wrap`, `/aep-dispatch`, `/aep-reflect`) updates the YAML.

### Merging rules

- Merge new entries into existing keys, preserving any other settings (`permissions`, `env`, etc.).
- If `hooks.PreToolUse` already exists, **append** these hook entries rather than replacing existing hooks.
- If the file doesn't exist, create it from the hook-only template.

---

## Phase 5 — Verify Environment

```bash
echo "=== Core Tools ==="
command -v claude >/dev/null 2>&1 || command -v codex >/dev/null 2>&1 \
  && echo "executor:      OK" || echo "executor:      MISSING (claude or codex)"
for cmd in node npm bun git gh openspec; do
  printf "%-15s" "$cmd:"
  command -v "$cmd" >/dev/null 2>&1 && echo "OK" || echo "MISSING"
done
node -e 'const [M,m]=process.versions.node.split(".").map(Number); process.exit(M>20||(M===20&&m>=19)?0:1)' \
  && echo "node version:   OK (>=20.19)" || echo "node version:   TOO OLD (need >=20.19)"
if [ -e .claude/skills/aep-onboard/SKILL.md ]; then
  printf "%-15s" "jq:"
  command -v jq >/dev/null 2>&1 && echo "OK" || echo "MISSING (required for Claude hooks)"
fi
echo ""
echo "=== Optional Tools ==="
for cmd in tmux cmux agent-browser portless; do
  printf "%-15s" "$cmd:"
  command -v "$cmd" >/dev/null 2>&1 && echo "OK" || echo "MISSING (optional)"
done
echo ""
echo "=== Git Repo ==="
git rev-parse --is-inside-work-tree >/dev/null 2>&1 && echo "git repo: OK" || echo "Not a git repo — run: git init"
git worktree list 2>/dev/null | head -5
```

If all core tools show OK, the environment is ready.

### Integration branch (single- vs two-branch mode)

AEP integrates all feature work into one **integration branch** (`$BASE` across the skills). Resolve `$BASE` per `/aep-git-ref` → "Integration Branch" (config override first, then auto-detect — the standard `main`/`develop` cases need no configuration) and report which mode this repo is in: `develop` → two-branch (main is promote-only production); otherwise single-branch on `main`.

For a **non-standard** integration branch name (not `main`/`develop`, e.g. `staging` or `integration`), set the repo-local override once — see the same `/aep-git-ref` section for the config command and why the standard cases stay unpinned (so a repo can grow from single- to two-branch mode with no reconfiguration).

### Enable the native launch modes (recommended)

`/aep-launch` and `/aep-autopilot` pick the launch mode automatically — native first, tmux only when pinned. **Claude Code needs no setup** (background subagents by default, falling back to background sessions). To unlock the best **Codex** mode, commit the two AEP role files (`aep-builder.toml`, `aep-evaluator.toml`) into the project's `.codex/agents/` — the TOML templates and the full launch-mode explainer live in `/aep-executor` (`references/codex-native.md`). Prefer the legacy tmux+cmux workflow instead? Pin it: `git config aep.executor-backend tmux`.

---

## Next Steps — Pick Your Path

Your next move depends on your situation. Pick the path that matches what you're trying to do. Full context for each path (including why each step is in the order it's in) is in [the installed orientation guide](references/orientation.md#4-pick-one-of-four-paths).

### Path A — New product from scratch

You have an idea and a fresh repo.

```
/aep-envision  →  /aep-map  →  /aep-validate  →  /aep-scaffold  →  /aep-autopilot
```

`/aep-envision` validates the opportunity and extracts the activity backbone. `/aep-map` decomposes it into a system map + story graph + agent topology. `/aep-validate` runs gen/eval checks. `/aep-scaffold` creates the monorepo + OpenSpec. `/aep-autopilot` (optional) takes over hands-free — or drive it manually with `/aep-dispatch → /aep-design → /aep-launch → /aep-build → /aep-wrap`.

### Path B — Onboarding an existing project

You have a codebase and want to add AEP workflows to it.

```
/aep-scaffold  →  /aep-dispatch  →  /aep-design  →  /aep-launch  →  /aep-build  →  /aep-wrap
```

`/aep-scaffold` adds agentic infrastructure (OpenSpec, workspace hooks, E2E skeleton) to existing code. Then start a feature cycle with `/aep-dispatch`. Use `/aep-envision` later if you want to retrofit a product context.

### Path C — Single feature, no product context

You just want to ship one feature with AEP workflows.

```
/aep-design  →  /aep-launch  →  /aep-build  →  /aep-wrap
```

`/aep-design` produces an OpenSpec change on the integration branch (`$BASE`). `/aep-launch` spawns an isolated git worktree on a `feat/<name>` branch and boots the agent. `/aep-build` implements, tests, reviews, and merges. `/aep-wrap` archives and removes the worktree.

### Path D — Hands-free autonomous mode

You have a validated product context and want to go grab coffee.

```
/aep-autopilot
```

One command. Pauses only for design escalations or layer gate failures. Run `/aep-autopilot` directly for its full tick and pause protocol.

**Still unsure which path?** Use the decision rule in [the installed orientation guide](references/orientation.md#4-pick-one-of-four-paths).

---

## Guardrails

- **Run from the project root** — tools and plugins are verified relative to the current environment.
- **Re-run anytime** — safe to re-run to verify the environment is still complete. Returning users can skip Phase 0 (orientation) and jump to Phase 1.
- **Checks only** — this skill verifies and installs tools; it does not scaffold projects or modify code.

---

## Learn More

- [Installed orientation guide](references/orientation.md) — mental models, skill map, and the four entry paths; always available offline with this skill.
- `/aep-git-ref` — worktree lifecycle, branch naming, commit-per-task pattern, `$BASE` resolution, and recovery.
- `/aep-autopilot` — the autonomous tick, monitoring, escalation, and pause protocol.
- [Public AEP documentation](https://github.com/memorysaver/agentic-engineering-patterns/tree/main/docs) — extended glossary, decisions, and architecture material when network access is available.
