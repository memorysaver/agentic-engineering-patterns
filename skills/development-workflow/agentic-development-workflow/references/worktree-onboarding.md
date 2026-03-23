# Worktree Onboarding

This document is the bootstrap guide for spawned agents running in a git worktree. Read this first when entering a worktree session.

## Context

You are a Claude Code agent spawned in an isolated git worktree to implement a feature autonomously. The design phases (1-3) were completed on `main` by the user. Your job is to execute Phases 0, 4-12.

## Bootstrap Sequence

### 1. Orient yourself

```bash
# Where am I?
pwd
git branch --show-current

# What's the OpenSpec change?
ls openspec/changes/
```

### 2. Read all change artifacts

```bash
# Read the full change context
cat openspec/changes/<change-name>/proposal.md
cat openspec/changes/<change-name>/design.md
cat openspec/changes/<change-name>/tasks.md
ls openspec/changes/<change-name>/specs/ 2>/dev/null
```

### 3. Initialize tracking

```bash
mkdir -p .dev-workflow

# Copy progress template
cp skills/development-workflow/agentic-development-workflow/references/progress-template.md \
   .dev-workflow/progress-$(git branch --show-current | tr '/' '-').md

# Ensure .dev-workflow is gitignored
grep -q '.dev-workflow' .gitignore || echo '\n.dev-workflow/' >> .gitignore
```

Edit the progress file:
- Fill in feature name, branch name, date, change name
- Mark Phases 1-3 as `[x]` (pre-completed on main)

### 4. Set up environment

```bash
# Install dependencies
bun install

# Start dev server
bun run dev &

# Write port config
echo "WEB_PORT=3000\nSERVER_PORT=3001\nBASE_URL=http://localhost:3000\nSERVER_URL=http://localhost:3001" > .dev-workflow/ports.env
```

### 5. Begin implementation

Now follow the workflow starting from **Phase 4: OpenSpec Apply**.

Read the full workflow at the agentic-development-workflow SKILL.md for phase details.

## Key Rules

- **Update the progress file** after completing each phase
- **Never run `/opsx:archive`** — that happens on main after merge
- **Don't stage `openspec/specs/`** files in your commits
- **Ask for confirmation** before creating PRs or merging
- **The `.dev-workflow/` folder is ephemeral** — never commit it
