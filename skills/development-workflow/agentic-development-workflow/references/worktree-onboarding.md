# Worktree Onboarding

This document is the bootstrap guide for spawned agents running in a jj workspace. Read this first when entering a workspace session.

## Context

You are a Claude Code agent spawned in an isolated jj workspace to implement a feature autonomously. The design phases (1-3) were completed on `main` by the user. Your job is to execute Phases 0, 4-12.

## Bootstrap Sequence

### 1. Orient yourself

```bash
# Where am I?
pwd
jj log -r @ --no-graph -T 'change_id.short(8) ++ " " ++ description.first_line() ++ "\n"'

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

# Copy progress template
cp skills/development-workflow/agentic-development-workflow/references/progress-template.md \
   .dev-workflow/progress-$(jj log --no-graph -r @ -T 'change_id.short(8)').md

# Ensure .dev-workflow is gitignored
grep -q '.dev-workflow' .gitignore || echo '\n.dev-workflow/' >> .gitignore
```

Edit the progress file:
- Fill in feature name, change ID, date, change name, mode (full/light)
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

### 5. Generate harness artifacts

After creating the jj change stack (see SKILL.md Phase 0 step 6), generate these additional artifacts:

- **Sprint contracts** — `.dev-workflow/contracts.md`: Per-task success criteria and verification steps extracted from OpenSpec specs. See `references/contract-template.md` for the format.
- **Feature verification list** — `.dev-workflow/feature-verification.json`: JSON verification list for evaluator scoring. Generator must NOT modify `verification_steps` or `passes` fields.
- **Session recovery script** — `.dev-workflow/init.sh`: Auto-generated script for resuming after context resets. Make executable with `chmod +x`.
- **Inter-agent signals** — `.dev-workflow/signals/status.json`: Initialize with current phase. See `references/signals-spec.md` for the full specification.

### 6. Begin implementation

Now follow the workflow starting from **Phase 4: OpenSpec Apply**.

Read the full workflow at the agentic-development-workflow SKILL.md for phase details.

---

## Resuming a Session

If you are resuming an interrupted session (context reset, crash, manual restart):

1. **Check for init.sh** — if `.dev-workflow/init.sh` exists, run it:
   ```bash
   bash .dev-workflow/init.sh
   ```
   This restarts the dev server, shows the change stack, and displays progress. This is preferred over full context resets because it preserves structured state from previous sessions.

2. **Read the progress file** to find your current phase:
   ```bash
   cat .dev-workflow/progress-*.md
   ```

3. **Check for pending feedback** from the main session:
   ```bash
   cat .dev-workflow/signals/feedback.md 2>/dev/null
   ```

4. **Continue from where you left off** — pick up at the first unchecked phase.

> Do NOT re-run the full bootstrap if `.dev-workflow/` already exists. Use init.sh for recovery.

---

## Key Rules

- **Update the progress file** after completing each phase
- **Update signals** — write to `.dev-workflow/signals/status.json` at phase boundaries, check `feedback.md` for main session input
- **Never run `/opsx:archive`** — that happens on main after merge
- **Don't stage `openspec/specs/`** files in your commits
- **Ask for confirmation** before creating PRs or merging
- **The `.dev-workflow/` folder is ephemeral** — never commit it
- **Generator must not modify verification data** — never change `verification_steps` or `passes` in `feature-verification.json`
