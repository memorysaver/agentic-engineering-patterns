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

## Phase 4 — Install Recommended Claude Code Plugins

These plugins complement the agentic engineering workflow:

| Plugin | Purpose | Install |
|--------|---------|---------|
| `superpowers@superpowers-marketplace` | Planning, debugging, TDD, code review workflows | `/plugin install superpowers@superpowers-marketplace` |
| `agent-browser@agent-browser` | Browser automation for testing | `/plugin install agent-browser@agent-browser` |
| `frontend-design@claude-plugins-official` | High-quality UI generation | `/plugin install frontend-design@claude-plugins-official` |
| `code-review@claude-plugins-official` | PR code review | `/plugin install code-review@claude-plugins-official` |
| `mgrep@Mixedbread-Grep` | Semantic search (local + web) | `/plugin install mgrep@Mixedbread-Grep` |
| `skill-creator@claude-plugins-official` | Create and test new skills | `/plugin install skill-creator@claude-plugins-official` |

Install all recommended plugins, or pick the ones relevant to your workflow.

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
