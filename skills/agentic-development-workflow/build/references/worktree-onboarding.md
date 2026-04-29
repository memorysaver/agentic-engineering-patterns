# Worktree Onboarding

This document is the bootstrap guide for spawned agents running in a git worktree. Read this first when entering a workspace session.

## Context

You are a Claude Code agent spawned in an isolated git worktree to implement a feature autonomously. The design phases (1-3) were completed on `main` by the user, and `/launch` created your worktree on a fresh `feat/<name>` branch. Your job is to execute Phases 0, 4-12.

## Bootstrap Sequence

### 1. Orient yourself

```bash
# Where am I? Which branch? What's the base?
pwd
git branch --show-current
git log --oneline -5

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

### 3. Initialize tracking and harness artifacts

```bash
mkdir -p .dev-workflow .dev-workflow/signals

# Copy progress template (named by base commit SHA for traceability)
cp skills/agentic-development-workflow/build/references/progress-template.md \
   .dev-workflow/progress-$(git rev-parse --short HEAD).md

# Ensure .dev-workflow is gitignored
grep -q '.dev-workflow' .gitignore || echo '\n.dev-workflow/' >> .gitignore
```

Edit the progress file:

- Fill in feature name, base commit SHA, date, change name, mode (full/light)
- Mark Phases 1-3 as `[x]` (pre-completed on main)

### 4. Set up environment

Run the project's setup hook if it exists:

```bash
SETUP_HOOK=.claude/hooks/workspace-setup.sh
if [ -f "$SETUP_HOOK" ]; then
  bash "$SETUP_HOOK"
else
  echo "No workspace setup hook found at $SETUP_HOOK"
  echo "Read the project README or ask the user for setup instructions."
fi
```

The setup hook handles project-specific concerns: package installation, dev server, port assignment, DB seeding, etc. It must write `.dev-workflow/ports.env` with `WEB_PORT`, `SERVER_PORT`, `BASE_URL`, `SERVER_URL`.

Verify setup produced ports.env:

```bash
[ -f .dev-workflow/ports.env ] && source .dev-workflow/ports.env
```

### 5. Generate harness artifacts

After reading `tasks.md` (see SKILL.md Phase 0 step 6), generate these additional artifacts:

- **Sprint contracts** — `.dev-workflow/contracts.md`: Per-task success criteria and verification steps extracted from OpenSpec specs. See `references/contract-template.md` for the format.
- **Feature verification list** — `.dev-workflow/feature-verification.json`: JSON verification list for evaluator scoring. `commit_sha` starts as `null` and is filled in after each task is committed in Phase 4. Generator must NOT modify `verification_steps` or `passes` fields.
- **Session recovery script** — `.dev-workflow/init.sh`: Auto-generated script for resuming after context resets. Make executable with `chmod +x`.
- **Inter-agent signals** — `.dev-workflow/signals/status.json`: Initialize with current phase. See `references/signals-spec.md` for the full specification.

### 6. Begin implementation

Now follow the workflow starting from **Phase 4: OpenSpec Apply**.

Read the full workflow at the `/build` skill (skills/agentic-development-workflow/build/SKILL.md) for phase details.

---

## Resuming a Session

If you are resuming an interrupted session (context reset, crash, manual restart):

1. **Check for init.sh** — if `.dev-workflow/init.sh` exists, run it:

   ```bash
   bash .dev-workflow/init.sh
   ```

   This restarts the dev server, shows the branch state, and displays progress. This is preferred over full context resets because it preserves structured state from previous sessions.

2. **Read the progress file** to find your current phase:

   ```bash
   cat .dev-workflow/progress-*.md
   ```

3. **Check for pending feedback** from the main session:

   ```bash
   cat .dev-workflow/signals/feedback.md 2>/dev/null
   ```

4. **Continue from where you left off** — pick up at the first unchecked phase. Inspect prior commits via `git log --oneline main..HEAD` to see what's already implemented.

> Do NOT re-run the full bootstrap if `.dev-workflow/` already exists. Use init.sh for recovery.

---

## Key Rules

- **Update the progress file** after completing each phase
- **Update signals** — write to `.dev-workflow/signals/status.json` at phase boundaries, check `feedback.md` for main session input
- **Never run `/opsx:archive`** — that happens on main after merge
- **Don't stage `openspec/specs/`** files in your commits
- **Ask for confirmation** before creating PRs or merging
- **The `.dev-workflow/` folder is ephemeral** — never commit it
- **Generator must not modify verification data** — never change `verification_steps` or `passes` in `feature-verification.json`. Only `commit_sha` is generator-writable.
- **One commit per task in Phase 4** — keeps the PR review readable. Squash-merge at PR-merge cleans up main history.
- **Don't rewrite already-pushed commits** — fixes go as follow-up commits. If you need to amend, do it before the next task's commit.
