---
name: aep-onboard
description: Environment setup and first-time orientation for Agentic Engineering Patterns (AEP). Use to install the plugin, verify tools, and configure project plugins, or to get the mental model when new to AEP. Always use this first when a user mentions AEP or this plugin for the first time.
---

# Onboard

Set up your environment for agentic TypeScript development and get oriented to how AEP thinks. Phase 0 points you to the mental-model tour; Phases 1–5 install the plugin, verify tools, and configure recommended plugins. Run once on first setup — returning users re-verifying their environment can skip Phase 0 and start at Phase 1.

---

## Phase 0 — Orient Yourself (first-timers only)

> **Returning user?** If you've run `/aep-onboard` before and are just re-verifying your environment, skip to Phase 1.

AEP is not a command runner — it's a workflow that separates _thinking_ (what to build, decided with the AI on the **control plane**) from _doing_ (agents building to precise specs on the **execution plane**), communicating only through structured artifacts (`product-context.yaml`, signal files in `.dev-workflow/signals/`) rather than shared code context. Installing the tools without that model leaves you staring at a blank terminal unsure which skill to run first.

Read **[docs/orientation.md](../../../docs/orientation.md)** — the canonical 10-minute first-hour tour: the three mental models (control vs execution plane, the Jeff Patton story map, the two-session main/workspace model), what every skill does, and the four concrete paths (new product / existing project / single feature / hands-free).

**Done when:** you've read orientation.md and can name which of the four paths in "Next Steps — Pick Your Path" (below) matches your situation. Then continue to Phase 1.

---

## Phase 1 — Install the Plugin

Install the AEP skills with the [`skills`](https://github.com/vercel-labs/skills) CLI at **project level**, once per agent your project uses. Pin to the latest release and commit the installed files so the version is frozen for your team:

```bash
# Claude Code (repeat with `-a codex` for Codex). Newest tag:
# https://github.com/memorysaver/agentic-engineering-patterns/releases/latest
npx skills add memorysaver/agentic-engineering-patterns@<latest-tag> -a claude-code --skill '*' -y
```

This installs every AEP skill (the `aep-*` names) plus a `skills-lock.json` manifest — **commit both**. For the full pinning + formatter guidance, see [Installing Skills](../../../README.md#installing-skills).

### Optional add-ons — always ask the user

AEP pairs with two project-level skills from [`memorysaver/skills`](https://github.com/memorysaver/skills). **Ask the user whether they want each**, and install only what they choose (newest tag at <https://github.com/memorysaver/skills/releases/latest>, once per agent):

- **Behavioral guidelines in `AGENTS.md`?** → install `project-behavior`, then run it to scaffold/extend `AGENTS.md`.
- **A project memory system (committed lessons + recall)?** → install `project-memory` (and `memory-forge`), run `project-memory` to bootstrap `project-memory/`, then add a concise `## Memory & Learning Loop` section to `AGENTS.md` that **layers** these onto AEP's native lessons loop instead of duplicating it. AEP already captures (`/aep-build` → `.dev-workflow/lessons.md`), archives (`/aep-wrap` → `lessons-learned/`), and recalls (`/aep-launch`); the supplement adds: `project-memory` recall at `/aep-dispatch` + persisting the archived lesson at `/aep-wrap` (qmd semantic recall), and `memory-forge` distilling settled lessons (≥7 days, ≥3 accrued) into skills at `/aep-reflect` / pre-PR.

```bash
npx skills add memorysaver/skills@<latest-tag> -a claude-code \
  --skill project-behavior --skill project-memory --skill memory-forge -y
```

> **Note:** This installs the AEP skills themselves. Recommended third-party Claude Code plugins are configured at the project level in Phase 4 via `.claude/settings.json`; browser automation is added only after its local smoke test passes.

---

## Phase 2 — Verify Required Tools

Each tool below earns its place in the agentic workflow — `git` provides version control and worktrees (one isolated working tree per parallel agent), `bun` runs the TypeScript monorepo, `openspec` powers spec-driven development, an **executor** (`claude` _or_ `codex`) runs the implementation agents, and `gh` publishes PRs. `tmux` is **optional**: launches are native-first (see `/aep-executor`); tmux only hosts the pinned **legacy** mode and the generic-host fallback.

Run this check:

```bash
# Required: at least one executor (claude OR codex)
command -v claude >/dev/null 2>&1 || command -v codex >/dev/null 2>&1 \
  && echo "executor:      OK" || echo "executor:      MISSING (install claude or codex)"

# Required: everything else
for cmd in bun git gh openspec; do
  printf "%-15s" "$cmd:"
  which $cmd >/dev/null 2>&1 && echo "OK ($(which $cmd))" || echo "MISSING"
done

# Optional (legacy/pinned-tmux mode only): tmux
printf "%-15s" "tmux:"
which tmux >/dev/null 2>&1 && echo "OK ($(which tmux))" || echo "MISSING (optional — only the legacy launch mode needs it)"
```

Install any missing tools:

| Tool       | Purpose                                       | Install                                          |
| ---------- | --------------------------------------------- | ------------------------------------------------ |
| `git`      | Version control + worktrees                   | `xcode-select --install` (macOS)                 |
| `bun`      | Package manager & runtime                     | `curl -fsSL https://bun.sh/install \| bash`      |
| `claude`   | Executor: Claude Code CLI                     | `npm install -g @anthropic-ai/claude-code`       |
| `codex`    | Executor: OpenAI Codex CLI                    | `npm install -g @openai/codex` _(alt to claude)_ |
| `gh`       | GitHub CLI for PRs                            | `brew install gh`                                |
| `openspec` | Spec-driven development (Node >= 20.19)       | `npm install -g @fission-ai/openspec@latest`     |
| `tmux`     | Terminal multiplexer (optional — legacy mode) | `brew install tmux`                              |

All **required** tools (executor + `bun`/`git`/`gh`/`openspec`) must show OK
before proceeding. You need **at least one executor** (claude or codex) — not
both. `tmux` may be MISSING; that's fine — launches are native-first.

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

These are optional — the workflow works without them but is enhanced by them. On macOS, do not enable `agent-browser` until a one-command smoke test can launch a page without crashing Chrome:

```bash
agent-browser navigate about:blank
```

If macOS shows a Google Chrome crash report with `_RegisterApplication`, `TransformProcessType`, or `abort() called`, leave `agent-browser` disabled and use non-browser checks (`curl`, unit tests, screenshots from the user, or the host agent's browser tool) until the local Chrome/agent-browser combination is healthy.

---

## Phase 4 — Configure Project Plugins

Configure recommended plugins at the project level. These are not cosmetic — `superpowers` provides the planning/TDD skills that `/aep-design` assumes exist, `mgrep` powers deeper search, `frontend-design` is assumed by visual calibration work, `code-review` is used by `/aep-build`, and the hooks enforce the concurrency protocol that keeps parallel workspace agents from corrupting `product-context.yaml`. Plugin roles are summarized in [references/plugins.md](references/plugins.md).

### What to write

Read `.claude/settings.json` if it exists, then merge **[references/settings-template.json](references/settings-template.json)** into it (create the file from the template if it's missing). The template carries three keys — `extraKnownMarketplaces`, `enabledPlugins`, and the two concurrency-protocol `hooks`.

**Verify** the two hooks landed:

```bash
jq '[.hooks.PreToolUse[].matcher]' .claude/settings.json
# expect an array containing both "Edit|Write" and "Bash"
```

> **Concurrency protocol hooks:** they block a workspace agent from editing, writing, or committing `product-context.yaml` and redirect it to signal files. This is defense-in-depth — the skill instructions also direct agents to use signals, but the hook catches model drift. Only the main session (`/aep-wrap`, `/aep-dispatch`, `/aep-reflect`) updates the YAML.

### Optional browser automation

Add `agent-browser` only after the Phase 3 smoke test succeeds — it launches a local Chrome that some macOS/Chrome combinations crash during application registration. Merge this extra block into `.claude/settings.json`:

```json
{
  "extraKnownMarketplaces": {
    "agent-browser": {
      "source": { "source": "github", "repo": "vercel-labs/agent-browser" }
    }
  },
  "enabledPlugins": {
    "agent-browser@agent-browser": true
  }
}
```

### Merging rules

- Merge new entries into existing keys — preserve any other settings (`permissions`, `env`, etc.); do not overwrite them.
- If `hooks.PreToolUse` already exists, **append** these hook entries rather than replacing existing hooks.
- If the file doesn't exist, create it from the template (all three keys).

---

## Phase 5 — Verify Environment

```bash
echo "=== Core Tools ==="
command -v claude >/dev/null 2>&1 || command -v codex >/dev/null 2>&1 \
  && echo "executor:      OK" || echo "executor:      MISSING (claude or codex)"
for cmd in bun git gh openspec; do
  printf "%-15s" "$cmd:"
  which $cmd >/dev/null 2>&1 && echo "OK" || echo "MISSING"
done
echo ""
echo "=== Optional Tools ==="
for cmd in tmux cmux agent-browser portless; do
  printf "%-15s" "$cmd:"
  which $cmd >/dev/null 2>&1 && echo "OK" || echo "MISSING (optional)"
done
echo ""
echo "=== Git Repo ==="
[ -d .git ] && echo "git repo: OK" || echo "Not a git repo — run: git init"
git worktree list 2>/dev/null | head -5
```

If all core tools show OK, the environment is ready.

### Integration branch (single- vs two-branch mode)

AEP integrates all feature work into one **integration branch** (`$BASE` across the skills). The standard cases are **auto-detected** — you configure nothing. Report which mode this repo is in:

```bash
# Auto-detect (same logic every skill uses): develop → two-branch; otherwise single-branch
if git show-ref --verify --quiet refs/heads/develop \
   || git show-ref --verify --quiet refs/remotes/origin/develop; then
  echo "Integration branch: develop  (two-branch mode — main is promote-only production)"
else
  echo "Integration branch: main  (single-branch mode)"
fi
```

For a **non-standard** integration branch name (not `main`/`develop`, e.g. `staging` or `integration`), set the repo-local override once — see `/aep-git-ref` → "Integration Branch" for the config command and why the standard `main`/`develop` cases stay unpinned (so a repo can grow from single- to two-branch mode with no reconfiguration).

### Enable the native launch modes (recommended)

`/aep-launch` and `/aep-autopilot` pick the launch mode automatically — native first, tmux only when pinned. **Claude Code needs no setup** (background subagents by default, falling back to background sessions). To unlock the best **Codex** mode, commit the two AEP role files (`aep-builder.toml`, `aep-evaluator.toml`) into the project's `.codex/agents/` — the TOML templates and the full launch-mode explainer live in `/aep-executor` (`references/codex-native.md`). Prefer the legacy tmux+cmux workflow instead? Pin it: `git config aep.executor-backend tmux`.

---

## Next Steps — Pick Your Path

Your next move depends on your situation. Pick the path that matches what you're trying to do. Full context for each path (including why each step is in the order it's in) is in [docs/orientation.md](../../../docs/orientation.md) section 4, "The Four Paths".

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

One command. Pauses only for design escalations or layer gate failures. Deep dive: [docs/workflow/autonomous-loop.md](../../../docs/workflow/autonomous-loop.md).

**Still unsure which path?** See the decision tree in [docs/skills-quick-reference.md](../../../docs/skills-quick-reference.md#decision-tree).

---

## Guardrails

- **Run from the project root** — tools and plugins are verified relative to the current environment.
- **Re-run anytime** — safe to re-run to verify the environment is still complete. Returning users can skip Phase 0 (orientation) and jump to Phase 1.
- **Checks only** — this skill verifies and installs tools; it does not scaffold projects or modify code.

---

## Learn More

Pointers for going deeper — check what's relevant when you need it.

**Mental models & concepts**

- [docs/orientation.md](../../../docs/orientation.md) — the canonical first-hour guide (mental models + every skill + four paths)
- [README.md "Why This Exists"](../../../README.md#why-this-exists) — the full argument for spec-precision-over-execution-speed
- [docs/glossary.md](../../../docs/glossary.md) — precise definitions for every AEP term (ubiquitous language)

**Skills decision guide**

- [docs/skills-quick-reference.md](../../../docs/skills-quick-reference.md) — cheat sheet + decision tree + common sequences

**Autonomous mode**

- [docs/workflow/autonomous-loop.md](../../../docs/workflow/autonomous-loop.md) — how `/aep-autopilot` orchestrates dispatch → launch → monitor → wrap

**v2 upgrades**

- [docs/aep-v2-improvement-guideline.md](../../../docs/aep-v2-improvement-guideline.md) — split-mode, capability maps, readiness scoring, outcome contracts, technical specs, grouped changes

**Git + worktree conventions**

- `/aep-git-ref` — AEP git + worktree reference (worktree lifecycle, branch naming, commit-per-task pattern, `$BASE` resolution, recovery), accessed on-demand.
