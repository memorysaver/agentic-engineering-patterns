---
name: development-onboarding
description: Full environment onboarding for agentic engineering. Use when setting up a new machine, joining the project, or when the user says "get started", "onboard", "setup environment", "install prerequisites", or wants to prepare their dev environment for this plugin.
---

# Development Onboarding

Set up your environment for agentic TypeScript development. This skill takes you from a bare machine to fully configured — install the plugin, verify tools, and configure recommended plugins. Run it once on first setup, or re-run anytime to verify your environment.

---

## Phase 1 — Install the Plugin

Add the marketplace and install both plugin groups:

```bash
# Add the marketplace
/plugin marketplace add memorysaver/agentic-engineering-patterns

# Install plugin groups
/plugin install project-scaffold@agentic-engineering-patterns
/plugin install development-workflow@agentic-engineering-patterns
```

### Plugin Groups

| Group | Skills | Purpose |
|-------|--------|---------|
| **project-scaffold** | monorepo-setup, openspec-setup, development-onboarding | Scaffold projects, configure spec-driven development, environment onboarding |
| **development-workflow** | agentic-development-workflow | Full-lifecycle feature development with worktree isolation |

> **Note:** This installs the agentic-engineering-patterns plugin itself. Recommended third-party plugins (superpowers, agent-browser, etc.) are configured at the project level in Phase 4 via `.claude/settings.json`.

---

## Phase 2 — Verify Required Tools

Run this check:

```bash
for cmd in bun git gh claude openspec tmux cmux; do
  printf "%-15s" "$cmd:"
  which $cmd >/dev/null 2>&1 && echo "OK ($(which $cmd))" || echo "MISSING"
done
```

Install any missing tools:

| Tool | Purpose | Install |
|------|---------|---------|
| `bun` | Package manager & runtime | `curl -fsSL https://bun.sh/install \| bash` |
| `git` | Version control + worktrees | `xcode-select --install` (macOS) |
| `claude` | Claude Code CLI | `npm install -g @anthropic-ai/claude-code` |
| `gh` | GitHub CLI for PRs | `brew install gh` |
| `openspec` | Spec-driven development | `bun add -g openspec` |
| `tmux` | Terminal multiplexer | `brew install tmux` |
| `cmux` | Claude Code tab multiplexer | `bun add -g cmux` |

All tools must show OK before proceeding.

---

## Phase 3 — Verify Optional Tools

```bash
for cmd in agent-browser portless; do
  printf "%-15s" "$cmd:"
  which $cmd >/dev/null 2>&1 && echo "OK ($(which $cmd))" || echo "MISSING (optional)"
done
```

| Tool | Purpose | Install |
|------|---------|---------|
| `agent-browser` | Browser automation testing | Claude Code plugin: `agent-browser@agent-browser` |
| `portless` | Port management (.localhost) | `bun add -g portless` |

These are optional — the workflow works without them but is enhanced by them. `agent-browser` enables dogfood testing (Phase 6 of the development workflow). `portless` provides clean `.localhost` URLs for dev servers.

---

## Phase 4 — Configure Project Plugins

Configure recommended plugins at the project level so anyone who clones this repo gets prompted to install them automatically.

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
  }
}
```

### Plugin reference

| Plugin | Marketplace | Purpose |
|--------|------------|---------|
| `superpowers` | `superpowers-marketplace` | Planning, debugging, TDD, code review workflows |
| `agent-browser` | `agent-browser` | Browser automation for testing |
| `frontend-design` | `claude-plugins-official` | High-quality UI generation |
| `code-review` | `claude-plugins-official` | PR code review |
| `mgrep` | `Mixedbread-Grep` | Semantic search (local + web) |
| `skill-creator` | `claude-plugins-official` | Create and test new skills |

### How it works

- **`extraKnownMarketplaces`** declares where to find each plugin marketplace (GitHub repos)
- **`enabledPlugins`** declares which plugins should be active for this project
- When committed to git, team members are prompted to install these marketplaces and plugins on first use
- Users can decline specific plugins — their choice is stored in their user settings

### Merging rules

- If `.claude/settings.json` already has `extraKnownMarketplaces` or `enabledPlugins`, merge new entries into the existing objects — do not overwrite other keys
- Preserve any existing settings (permissions, hooks, env, etc.)
- If the file doesn't exist, create it with just these two keys

---

## Phase 5 — Verify Environment

Run a final comprehensive check:

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
```

If all core tools show OK, the environment is ready.

---

## Next Steps

| Command | What it does |
|---------|-------------|
| `/monorepo-setup` | Scaffold a full-stack TypeScript monorepo via Better-T-Stack |
| `/openspec-setup` | Initialize spec-driven development with explore/propose/apply/archive commands |
| `/agentic-development-workflow` | Start the full development lifecycle — design, implement, test, PR |

Typical flow: `/monorepo-setup` → `/openspec-setup` → `/agentic-development-workflow`

---

## Guardrails

- **Run from the project root** — tools and plugins are verified relative to the current environment
- **Re-run anytime** — safe to re-run to verify environment is still complete
- **Checks only** — this skill verifies and installs tools, it does not scaffold projects or modify code
- **Complements runtime checks** — the `agentic-development-workflow` skill re-verifies tools at workflow start; this skill ensures the baseline is in place upfront
