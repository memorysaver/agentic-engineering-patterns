---
name: onboard
description: Full environment onboarding and first-hour orientation for Agentic Engineering Patterns (AEP). Use when setting up a new machine, joining the project, installing the plugin, or when the user says "get started", "onboard", "setup environment", "install prerequisites", "what is this plugin", "help me understand AEP", "I'm new to agentic engineering", "introduce me to this workflow", or wants to prepare their dev environment AND learn the mental model. Always use this first when a user mentions AEP, agentic-engineering-patterns, or this plugin for the first time.
---

# Onboard

Set up your environment for agentic TypeScript development AND get oriented to how AEP thinks. Phase 0 gives you the 5-minute mental model; Phases 1–5 install the plugin, verify tools, and configure recommended plugins. Run once on first setup — returning users running for environment verification can skip Phase 0 and jump straight to Phase 1.

---

## Phase 0 — Orient Yourself (5 minutes, first-timers only)

> **Returning user?** If you've run `/onboard` before and you're just re-verifying your environment, skip to Phase 1.

Before installing tools, get the mental model. AEP is not a "command runner" — it's a workflow that separates _thinking_ (what to build) from _doing_ (building it). Installing the tools without understanding this will leave you staring at a blank terminal wondering which of 16 skills to run first.

**The three mental models you need:**

1. **Control plane vs execution plane.** You + AI make high-leverage decisions on the **control plane** (goals, decomposition, architecture, priorities). Agents receive precise specs and build on the **execution plane**. They never share code context directly — only structured artifacts like `product-context.yaml` and signal files. See [README.md "The Mental Model"](../../../README.md#the-mental-model).

2. **The story map.** Your product is organized as a [Jeff Patton story map](https://www.jpattonassociates.com/user-story-mapping/) — a grid with activities (columns, user journey left→right), layers (rows, enrichment top→down), waves (parallel batches within a layer), and release lines (what's shippable). Layer 0 is the **walking skeleton** — the thinnest end-to-end path. See [README.md "The Story Map"](../../../README.md#the-story-map).

3. **Two-session model.** The **main session** runs on your `main` branch where you + AI plan (`/envision`, `/map`, `/dispatch`, `/design`, `/wrap`, `/reflect`). The **workspace session** runs autonomously in an isolated git worktree on a `feat/<name>` branch where one agent implements a feature (`/build`). They communicate only through signal files in `.dev-workflow/signals/`. See [skills/product-context/README.md](../../product-context/README.md#single-source-of-truth-product-contextyaml).

**v2 split-mode (good to know):** Some projects store product context in two files — `product/index.yaml` (stable intent: opportunity, personas, capabilities, constraints) + `product-context.yaml` (mutable state: architecture, stories, cost, changelog). All skills auto-detect which mode a project uses. If you see only `product-context.yaml`, that's v1 single-file mode and it works exactly the same way. See [docs/aep-v2-improvement-guideline.md](../../../docs/aep-v2-improvement-guideline.md).

**Next step:** for the full 10-minute first-hour guide — including a table of all 16 skills, four concrete paths (new product / existing project / single feature / hands-free), and a glossary shortlist — read **[docs/orientation.md](../../../docs/orientation.md)**. Then come back to Phase 1.

---

## Phase 1 — Install the Plugin

Add the marketplace and install both plugin groups:

```bash
# Add the marketplace
/plugin marketplace add memorysaver/agentic-engineering-patterns

# Install plugin groups
/plugin install product-context@agentic-engineering-patterns
/plugin install project-setup@agentic-engineering-patterns
/plugin install agentic-development-workflow@agentic-engineering-patterns
```

### Plugin Groups

| Group                            | Skills                               | Purpose                                                                      |
| -------------------------------- | ------------------------------------ | ---------------------------------------------------------------------------- |
| **product-context**              | envision, map, dispatch, reflect     | Product-level planning and iteration                                         |
| **project-setup**                | onboard, scaffold                    | Scaffold projects, configure spec-driven development, environment onboarding |
| **agentic-development-workflow** | design, launch, build, wrap, git-ref | Full-lifecycle feature development with git worktrees                        |

> **Note:** This installs the agentic-engineering-patterns plugin itself. Recommended third-party plugins (superpowers, agent-browser, etc.) are configured at the project level in Phase 4 via `.claude/settings.json`.

---

## Phase 2 — Verify Required Tools

Each tool below earns its place in the agentic workflow — `git` provides version control and worktrees (one isolated working tree per parallel agent), `bun` runs the TypeScript monorepo, `openspec` powers spec-driven development, `tmux`/`cmux` host long-running autonomous agents, and `gh` publishes PRs. You need all of them before Phase 4.

Run this check:

```bash
for cmd in bun git gh claude openspec tmux cmux; do
  printf "%-15s" "$cmd:"
  which $cmd >/dev/null 2>&1 && echo "OK ($(which $cmd))" || echo "MISSING"
done
```

Install any missing tools:

| Tool       | Purpose                     | Install                                     |
| ---------- | --------------------------- | ------------------------------------------- |
| `git`      | Version control + worktrees | `xcode-select --install` (macOS)            |
| `bun`      | Package manager & runtime   | `curl -fsSL https://bun.sh/install \| bash` |
| `claude`   | Claude Code CLI             | `npm install -g @anthropic-ai/claude-code`  |
| `gh`       | GitHub CLI for PRs          | `brew install gh`                           |
| `openspec` | Spec-driven development     | `bun add -g openspec`                       |
| `tmux`     | Terminal multiplexer        | `brew install tmux`                         |
| `cmux`     | Claude Code tab multiplexer | `bun add -g cmux`                           |

All tools must show OK before proceeding.

> **Note on parallelism:** Each parallel feature agent runs in its own `git worktree` at `.feature-workspaces/<name>/` on its own `feat/<name>` branch. Worktrees share the underlying `.git/objects` (no history duplication) but each adds one full working-tree copy on disk — budget accordingly when running many agents in parallel.

---

## Phase 3 — Verify Optional Tools

```bash
for cmd in agent-browser portless; do
  printf "%-15s" "$cmd:"
  which $cmd >/dev/null 2>&1 && echo "OK ($(which $cmd))" || echo "MISSING (optional)"
done
```

| Tool            | Purpose                      | Install                                           |
| --------------- | ---------------------------- | ------------------------------------------------- |
| `agent-browser` | Browser automation testing   | Claude Code plugin: `agent-browser@agent-browser` |
| `portless`      | Port management (.localhost) | `bun add -g portless`                             |

These are optional — the workflow works without them but is enhanced by them.

---

## Phase 4 — Configure Project Plugins

Configure recommended plugins at the project level. These plugins are not optional cosmetics — `superpowers` provides the planning/TDD skills that `/design` assumes exist, `mgrep` powers deeper search, `frontend-design` is assumed by visual calibration work, `code-review` is used by `/build`, and the hooks enforce the concurrency protocol that keeps parallel workspace agents from corrupting `product-context.yaml`.

### What to write

Read `.claude/settings.json` if it exists. Merge the following keys into it (or create the file if missing):

```json
{
  "extraKnownMarketplaces": {
    "claude-plugins-official": {
      "source": { "source": "github", "repo": "anthropics/claude-plugins-official" }
    },
    "superpowers-marketplace": {
      "source": { "source": "github", "repo": "obra/superpowers-marketplace" }
    },
    "agent-browser": {
      "source": { "source": "github", "repo": "vercel-labs/agent-browser" }
    },
    "Mixedbread-Grep": {
      "source": { "source": "github", "repo": "mixedbread-ai/mgrep" }
    }
  },
  "enabledPlugins": {
    "superpowers@superpowers-marketplace": true,
    "agent-browser@agent-browser": true,
    "frontend-design@claude-plugins-official": true,
    "code-review@claude-plugins-official": true,
    "mgrep@Mixedbread-Grep": true,
    "skill-creator@claude-plugins-official": true
  },
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r '.tool_input.file_path // \"\"' | { read -r f; case \"$f\" in *product-context.yaml) if [[ \"$PWD\" == */.feature-workspaces/* ]]; then echo '{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"permissionDecision\":\"deny\",\"permissionDecisionReason\":\"CONCURRENCY PROTOCOL: Workspace sessions must not write to product-context.yaml. Write to .dev-workflow/signals/status.json instead. Only the main session (via /wrap, /dispatch, /reflect) updates the YAML.\"}}'; fi ;; esac; }",
            "statusMessage": "Checking concurrency protocol..."
          }
        ]
      },
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r '.tool_input.command // \"\"' | { read -r cmd; if [[ \"$PWD\" == */.feature-workspaces/* ]] && echo \"$cmd\" | grep -qE 'git\\s+(add|commit).*product-context\\.yaml'; then echo '{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"permissionDecision\":\"deny\",\"permissionDecisionReason\":\"CONCURRENCY PROTOCOL: Workspace sessions must not commit product-context.yaml. Write to .dev-workflow/signals/status.json instead.\"}}'; fi; }",
            "statusMessage": "Checking concurrency protocol..."
          }
        ]
      }
    ]
  }
}
```

> **Concurrency protocol hooks:** The `hooks` section enforces the rule that only the main session writes to `product-context.yaml`. When a workspace agent attempts to edit, write, or commit `product-context.yaml`, the hook blocks the action and explains how to use signals instead. This is defense-in-depth — the skill instructions also direct agents to use signals, but the hook catches any model drift.

### Plugin reference

| Plugin            | Marketplace               | Purpose                                         |
| ----------------- | ------------------------- | ----------------------------------------------- |
| `superpowers`     | `superpowers-marketplace` | Planning, debugging, TDD, code review workflows |
| `agent-browser`   | `agent-browser`           | Browser automation for testing                  |
| `frontend-design` | `claude-plugins-official` | High-quality UI generation                      |
| `code-review`     | `claude-plugins-official` | PR code review                                  |
| `mgrep`           | `Mixedbread-Grep`         | Semantic search (local + web)                   |
| `skill-creator`   | `claude-plugins-official` | Create and test new skills                      |

### Merging rules

- If `.claude/settings.json` already has these keys, merge new entries — do not overwrite other keys
- Preserve any existing settings (permissions, env, etc.)
- If `.claude/settings.json` already has a `hooks.PreToolUse` array, append these hook entries — do not replace existing hooks
- If the file doesn't exist, create it with all three keys (`extraKnownMarketplaces`, `enabledPlugins`, `hooks`)

---

## Phase 5 — Verify Environment

```bash
echo "=== Core Tools ==="
for cmd in bun git gh claude openspec tmux cmux; do
  printf "%-15s" "$cmd:"
  which $cmd >/dev/null 2>&1 && echo "OK" || echo "MISSING"
done
echo ""
echo "=== Optional Tools ==="
for cmd in agent-browser portless; do
  printf "%-15s" "$cmd:"
  which $cmd >/dev/null 2>&1 && echo "OK" || echo "MISSING (optional)"
done
echo ""
echo "=== Git Repo ==="
[ -d .git ] && echo "git repo: OK" || echo "Not a git repo — run: git init"
git worktree list 2>/dev/null | head -5
```

If all core tools show OK, the environment is ready.

---

## Next Steps — Pick Your Path

Your next move depends on your situation. Pick the path that matches what you're trying to do. Full context for each path (including why each step is in the order it's in) is in [docs/orientation.md](../../../docs/orientation.md) section 4, "The Four Paths".

### Path A — New product from scratch

You have an idea and a fresh repo.

```
/envision  →  /map  →  /validate  →  /scaffold  →  /autopilot
```

`/envision` validates the opportunity and extracts the activity backbone. `/map` decomposes it into a system map + story graph + agent topology. `/validate` runs gen/eval checks. `/scaffold` creates the monorepo + OpenSpec. `/autopilot` (optional) takes over hands-free — or drive it manually with `/dispatch → /design → /launch → /build → /wrap`.

### Path B — Onboarding an existing project

You have a codebase and want to add AEP workflows to it.

```
/scaffold  →  /dispatch  →  /design  →  /launch  →  /build  →  /wrap
```

`/scaffold` adds agentic infrastructure (OpenSpec, workspace hooks, E2E skeleton) to existing code. Then start a feature cycle with `/dispatch`. Use `/envision` later if you want to retrofit a product context.

### Path C — Single feature, no product context

You just want to ship one feature with AEP workflows.

```
/design  →  /launch  →  /build  →  /wrap
```

`/design` produces an OpenSpec change on `main`. `/launch` spawns an isolated git worktree on a `feat/<name>` branch and boots the agent. `/build` implements, tests, reviews, and merges. `/wrap` archives and removes the worktree.

### Path D — Hands-free autonomous mode

You have a validated product context and want to go grab coffee.

```
/autopilot
```

One command. Pauses only for design escalations or layer gate failures. Deep dive: [docs/workflow/autonomous-loop.md](../../../docs/workflow/autonomous-loop.md).

**Still unsure which path?** See the decision tree in [docs/skills-quick-reference.md](../../../docs/skills-quick-reference.md#decision-tree).

---

## Guardrails

- **Run from the project root** — tools and plugins are verified relative to the current environment
- **Re-run anytime** — safe to re-run to verify environment is still complete. Returning users can skip Phase 0 (orientation) and jump to Phase 1.
- **Checks only** — this skill verifies and installs tools, it does not scaffold projects or modify code

---

## Learn More

Pointers for going deeper. None of these are required reading — check what's relevant when you need it.

**Mental models & concepts**

- [docs/orientation.md](../../../docs/orientation.md) — the canonical first-hour guide (mental models + 16 skills + four paths)
- [README.md "Why This Exists"](../../../README.md#why-this-exists) — the full argument for spec-precision-over-execution-speed
- [docs/glossary.md](../../../docs/glossary.md) — precise definitions for every AEP term (ubiquitous language)

**Skills decision guide**

- [docs/skills-quick-reference.md](../../../docs/skills-quick-reference.md) — cheat sheet + decision tree + common sequences

**Autonomous mode**

- [docs/workflow/autonomous-loop.md](../../../docs/workflow/autonomous-loop.md) — how `/autopilot` orchestrates dispatch → launch → monitor → wrap

**v2 upgrades**

- [docs/aep-v2-improvement-guideline.md](../../../docs/aep-v2-improvement-guideline.md) — split-mode, capability maps, readiness scoring, outcome contracts, technical specs, grouped changes

**Git + worktree conventions**

- [skills/agentic-development-workflow/git-ref/SKILL.md](../../agentic-development-workflow/git-ref/SKILL.md) — AEP git + worktree reference (worktree lifecycle, branch naming, commit-per-task pattern, recovery), accessed on-demand via `/git-ref`
