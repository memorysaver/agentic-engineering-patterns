---
name: onboard
description: Full environment onboarding for agentic engineering. Use when setting up a new machine, joining the project, or when the user says "get started", "onboard", "setup environment", "install prerequisites", or wants to prepare their dev environment for this plugin.
---

# Onboard

Set up your environment for agentic TypeScript development. Install the plugin, verify tools, and configure recommended plugins. Run once on first setup, or re-run anytime to verify your environment.

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

| Group | Skills | Purpose |
|-------|--------|---------|
| **product-context** | envision, map, dispatch, reflect | Product-level planning and iteration |
| **project-setup** | onboard, scaffold | Scaffold projects, configure spec-driven development, environment onboarding |
| **agentic-development-workflow** | design, launch, build, wrap, jj-ref | Full-lifecycle feature development with jj workspaces |

> **Note:** This installs the agentic-engineering-patterns plugin itself. Recommended third-party plugins (superpowers, agent-browser, etc.) are configured at the project level in Phase 4 via `.claude/settings.json`.

---

## Phase 2 — Verify Required Tools

Run this check:

```bash
for cmd in jj bun git gh claude openspec tmux cmux; do
  printf "%-15s" "$cmd:"
  which $cmd >/dev/null 2>&1 && echo "OK ($(which $cmd))" || echo "MISSING"
done
```

Install any missing tools:

| Tool | Purpose | Install |
|------|---------|---------|
| `jj` | Change-oriented local VCS | `brew install jj` or `cargo install jj-cli` |
| `bun` | Package manager & runtime | `curl -fsSL https://bun.sh/install \| bash` |
| `git` | Remote collaboration + GitHub | `xcode-select --install` (macOS) |
| `claude` | Claude Code CLI | `npm install -g @anthropic-ai/claude-code` |
| `gh` | GitHub CLI for PRs | `brew install gh` |
| `openspec` | Spec-driven development | `bun add -g openspec` |
| `tmux` | Terminal multiplexer | `brew install tmux` |
| `cmux` | Claude Code tab multiplexer | `bun add -g cmux` |

All tools must show OK before proceeding.

---

## Phase 2.5 — Initialize jj (Colocated Mode)

If the project has a `.git/` directory but no `.jj/`, initialize jj in colocated mode:

```bash
# Check if jj is already initialized
[ -d .jj ] && echo "jj already initialized" || jj git init --colocate
```

This creates a colocated jj+git repo:
- **jj** manages local changes, history, workspaces
- **git** handles remote push/fetch, GitHub PRs, CI/CD
- Both `.jj/` and `.git/` coexist in the same repo

Add `.jj/` to `.gitignore` if not already present:

```bash
grep -q '\.jj' .gitignore 2>/dev/null || echo '\n# jj local state\n.jj/' >> .gitignore
```

> **Rule:** After initialization, use `jj` commands for all local work. Use `jj git` subcommands for remote operations. Never use raw `git commit` or `git add` in a colocated repo.

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

These are optional — the workflow works without them but is enhanced by them.

---

## Phase 4 — Configure Project Plugins

Configure recommended plugins at the project level.

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
            "command": "jq -r '.tool_input.command // \"\"' | { read -r cmd; if [[ \"$PWD\" == */.feature-workspaces/* ]] && echo \"$cmd\" | grep -qE '(git\\s+(add|commit)|jj\\s+(describe|new|commit)).*product-context\\.yaml'; then echo '{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"permissionDecision\":\"deny\",\"permissionDecisionReason\":\"CONCURRENCY PROTOCOL: Workspace sessions must not commit product-context.yaml. Write to .dev-workflow/signals/status.json instead.\"}}'; fi; }",
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

| Plugin | Marketplace | Purpose |
|--------|------------|---------|
| `superpowers` | `superpowers-marketplace` | Planning, debugging, TDD, code review workflows |
| `agent-browser` | `agent-browser` | Browser automation for testing |
| `frontend-design` | `claude-plugins-official` | High-quality UI generation |
| `code-review` | `claude-plugins-official` | PR code review |
| `mgrep` | `Mixedbread-Grep` | Semantic search (local + web) |
| `skill-creator` | `claude-plugins-official` | Create and test new skills |

### Merging rules

- If `.claude/settings.json` already has these keys, merge new entries — do not overwrite other keys
- Preserve any existing settings (permissions, env, etc.)
- If `.claude/settings.json` already has a `hooks.PreToolUse` array, append these hook entries — do not replace existing hooks
- If the file doesn't exist, create it with all three keys (`extraKnownMarketplaces`, `enabledPlugins`, `hooks`)

---

## Phase 5 — Verify Environment

```bash
echo "=== Core Tools ==="
for cmd in jj bun git gh claude openspec tmux cmux; do
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
echo "=== jj Colocated ==="
[ -d .jj ] && echo "jj initialized: OK" || echo "jj not initialized — run: jj git init --colocate"
```

If all core tools show OK, the environment is ready.

---

## Next Steps

| Command | What it does |
|---------|-------------|
| `/envision` | Validate a product idea and create the context document |
| `/scaffold` | Scaffold a full-stack TypeScript monorepo + initialize OpenSpec |
| `/dispatch` | Pick the next story from the map and start building |
| `/design` | Start designing a feature (explore + propose + review) |

Typical flow: `/envision` → `/map` → `/scaffold` → `/dispatch` → `/design` → `/launch` → `/build` → `/wrap` → `/reflect`

---

## Guardrails

- **Run from the project root** — tools and plugins are verified relative to the current environment
- **Re-run anytime** — safe to re-run to verify environment is still complete
- **Checks only** — this skill verifies and installs tools, it does not scaffold projects or modify code
