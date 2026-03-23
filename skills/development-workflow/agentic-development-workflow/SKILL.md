---
name: agentic-development-workflow
description: Full-lifecycle feature development workflow with progress tracking. Use when starting a new feature, implementing a change end-to-end, or when the user says "start development", "new feature", "develop", or wants to follow the full scaffold → explore → propose → apply → test → archive → PR pipeline. Orchestrates Better-T-Stack scaffold, OpenSpec skills, agent-browser testing, worktree isolation, and checkpoint tracking.
---

# Agentic Development Workflow

A structured, end-to-end workflow for feature development — from project scaffold through spec creation, implementation, browser-based testing, PR creation, and post-merge archival. Every step is tracked with checkboxes in a local temp folder so you can resume, skip, or re-run phases at any time.

This workflow uses a **two-session model**:

- **Main session** (interactive) — scaffold, design phases with the user on `main`
- **Worktree session** (autonomous) — implementation phases in an isolated worktree

Split into five parts:

- **Part A — Scaffold** (optional): Create a new project if none exists
- **Part B — Design:** Explore, propose, review (on `main`, interactive with user)
- **Part C — Launch Worktree:** Spawn autonomous Claude Code session in worktree
- **Part D — Implementation:** Apply, test, PR (in worktree, autonomous)
- **Part E — Post-Merge:** Archive + cleanup (on `main` after PR merges)

---

## Onboarding: "teach me" / "how does this work?"

If the user asks to learn the workflow:

1. **Read `README.md`** from this skill's directory to load the visual diagrams
2. **Print the Five-Part Workflow** diagram — show it to the user as-is
3. **Walk through each part** briefly, explaining what happens and why
4. **Print the Two-Session Model** diagram — explain how main and worktree sessions work
5. **Show where they are now** — check if a progress file exists in `.dev-workflow/` and highlight the current phase, or indicate they haven't started yet
6. **Ask** if they want to start a feature or dive deeper into any specific phase

---

## Prerequisites

Before starting the workflow, verify these dependencies are available.

### CLI Tools

Run this check:

```bash
for cmd in git bun tmux cmux claude openspec gh; do
  printf "%-15s" "$cmd:"
  which $cmd >/dev/null 2>&1 && echo "OK ($(which $cmd))" || echo "MISSING"
done
# Optional tools:
for cmd in agent-browser portless; do
  printf "%-15s" "$cmd:"
  which $cmd >/dev/null 2>&1 && echo "OK ($(which $cmd))" || echo "MISSING (optional)"
done
```

| Tool | Used in | Required? |
|------|---------|-----------|
| `git` | Worktree creation, commits, push | Required |
| `bun` | Install deps, run dev server | Required |
| `tmux` | Part C: launch worktree session | Required for Part C |
| `cmux` | Part C: create tab, send keys | Required for Part C |
| `claude` | Part C: spawned agent session | Required for Part C |
| `openspec` | Part B: explore/propose, Part D: apply | Required |
| `gh` | Phase 10: create PR | Required |
| `agent-browser` | Phase 6: dogfood testing | Optional |
| `portless` | Port management, .localhost URLs | Optional |

If any required tool is missing, install it before proceeding.

### Required Skills

Check that OpenSpec skills exist:

```bash
for skill in openspec-explore openspec-propose openspec-apply-change openspec-archive-change; do
  printf "%-35s" "$skill:"
  [ -f ".claude/skills/$skill/SKILL.md" ] && echo "OK" || echo "MISSING"
done
```

If OpenSpec skills are missing, run `/openspec-setup` first.

---

## Part A — Scaffold (optional, on main)

> Skip this part if the project already exists.

If no project exists yet, scaffold one:

1. **Invoke `/monorepo-setup`** to create the project via Better-T-Stack
2. **Invoke `/openspec-setup`** to initialize spec-driven development
3. **Verify** the project builds and OpenSpec is ready:
   ```bash
   bun install && turbo build && openspec list
   ```

---

## Part B — Design (on main, interactive with user)

> Run these phases in the **main workspace** on `main` branch. These are interactive conversations with the user to clarify requirements and create the OpenSpec change.

---

### Phase 1: OpenSpec Explore

Invoke the explore skill to think through the feature:

```
/opsx:explore
```

Use this phase to:

- Clarify requirements and scope with the user
- Investigate the codebase for relevant patterns
- Identify risks or unknowns
- Build shared understanding
- Create architecture documentation in `docs/` if the feature warrants it

---

### Phase 2: OpenSpec Propose

Invoke the propose skill to generate a full proposal:

```
/opsx:propose
```

This creates the OpenSpec change with all artifacts:

- `proposal.md` — what and why
- `design.md` — how, key decisions, risks
- `specs/**/*.md` — detailed requirements and scenarios
- `tasks.md` — implementation checklist

---

### Phase 3: Design Review

Before implementation, review the proposal from non-functional angles:

1. **Security** — Auth gaps, injection surfaces, data exposure?
2. **Performance** — N+1 queries, large payloads, blocking operations?
3. **Existing patterns** — Does it follow codebase conventions?
4. **Edge cases** — Concurrency issues, race conditions, failure modes?

**What NOT to review:** Business logic (decided in Phase 1), cosmetic preferences.

If adjustments are needed, update the OpenSpec change files directly.

### Commit to main

After Phase 3, commit all artifacts to `main`:

```bash
git add docs/ openspec/changes/<change-name>/
git commit -m "feat: add <change-name> architecture doc and OpenSpec change"
```

This ensures the worktree will have all artifacts when it's created from `main`.

---

## Part C — Launch Worktree (on main, automated)

> Run this from the **main workspace** (on `main` branch) to spawn a new autonomous implementation session.

### Guardrail: verify main is clean

```bash
git status --short
```

**If any files are staged or modified — ABORT.** Commit or stash first.

### Launch commands

```bash
# 1. Create the worktree
git worktree add .claude/worktrees/<name> \
  -b worktree-<name> origin/main

# 2. Fast-forward to include latest main commits
cd .claude/worktrees/<name> && git merge main --ff-only && cd -

# 3. Start Claude Code in a tmux session
tmux new-session -d -s <name> \
  -c .claude/worktrees/<name> \
  "claude --dangerously-skip-permissions --rc"

# 4. Create a cmux tab and attach
SURFACE_REF=$(cmux new-surface --type terminal | grep -o 'surface:[0-9]*')
cmux send --surface "$SURFACE_REF" "tmux attach -t <name>\n"
cmux rename-tab --surface "$SURFACE_REF" "<name>"
```

Replace `<name>` with a short feature name (e.g., `add-auth`).

### Send initial prompt to spawned session

After the cmux tab is attached and Claude Code is ready, send the bootstrap instruction:

```bash
cmux send --surface "$SURFACE_REF" "/agentic-development-workflow execute implementation for openspec change <change-name>. Read the worktree-onboarding reference at skills/development-workflow/agentic-development-workflow/references/worktree-onboarding.md for full setup instructions. Phases 1-3 are pre-completed on main.
"
```

### Managing parallel sessions

The main workspace stays on `main` and can:

- Launch multiple worktree sessions (one tab per feature)
- See all sessions as named cmux tabs
- Switch between sessions by clicking tabs
- Handle Part E (archive + cleanup) after each PR merges

---

## Part D — Implementation (in worktree, autonomous)

> Run these phases inside a git worktree on a feature branch. The spawned agent follows these steps autonomously after reading the worktree-onboarding reference.

---

### Phase 0: Initialize Tracking

Before any work begins, set up the tracking infrastructure and environment.

1. **Read the worktree-onboarding guide** at `skills/development-workflow/agentic-development-workflow/references/worktree-onboarding.md`.

2. **Discover the OpenSpec change:**
   - List `openspec/changes/` to find the active change
   - Read all artifacts: `proposal.md`, `design.md`, `specs/**/*.md`, `tasks.md`

3. **Create the tracking folder:**
   ```bash
   mkdir -p .dev-workflow
   ```

4. **Add `.dev-workflow/` to `.gitignore`** if not already present:
   ```bash
   grep -q '.dev-workflow' .gitignore || echo '\n# Development workflow tracking (per-worktree)\n.dev-workflow/' >> .gitignore
   ```

5. **Create the progress file** from the template:
   ```bash
   cp skills/development-workflow/agentic-development-workflow/references/progress-template.md \
      .dev-workflow/progress-$(git branch --show-current | tr '/' '-').md
   ```
   Fill in feature name, branch name, date, and OpenSpec change name.
   **Mark Phase 1-3 as pre-completed** (they were done on main).

6. **Install dependencies:**
   ```bash
   bun install
   ```

7. **Start the dev server:**
   ```bash
   bun run dev
   ```

8. **Set up port configuration** (if using portless):
   ```bash
   # Scan for available ports and write config
   mkdir -p .dev-workflow
   echo "WEB_PORT=3000\nSERVER_PORT=3001\nBASE_URL=http://localhost:3000\nSERVER_URL=http://localhost:3001" > .dev-workflow/ports.env
   ```

Update the Phase 0 checkbox in the progress file when done.

---

### Phase 4: OpenSpec Apply

Invoke the apply skill to implement the tasks from the proposal:

```
/opsx:apply
```

Work through each task in the proposal. Update the progress file checkbox for each completed task, and mark the Phase 4 checkbox when all tasks are done.

---

### Phase 5: Code Review & Verification

After implementation, verify the code before moving to testing.

#### Completeness check

1. Re-read the proposal (including any Phase 3 adjustments)
2. Walk through each task and verify it was fully implemented
3. If any task is incomplete, loop back to Phase 4

#### Code quality review

1. **Correctness** — Logic errors, off-by-one bugs, missing edge cases?
2. **Security** — Input validation, auth checks, SQL parameterization?
3. **Performance** — N+1 queries, missing indexes, unbounded loops?
4. **Conventions** — Naming, file structure, error handling, imports?

Document findings in `.dev-workflow/code-review-<feature>.md`. Fix any issues found.

Update the Phase 5 checkbox in the progress file when complete.

---

### Phase 6: Browser Testing (Dogfood)

> Skip if `agent-browser` is not installed.

**Port configuration:** Source `.dev-workflow/ports.env` to get the correct URLs:

```bash
source .dev-workflow/ports.env
```

Use agent-browser to systematically explore and test the application:

```
/agent-browser:dogfood
```

This produces a structured report with:
- Step-by-step screenshots
- Bug reproduction steps
- UX issues found

Document results in `.dev-workflow/dogfood-<feature>.md`.

Update the Phase 6 checkbox in the progress file when complete.

---

### Phase 7: E2E Test Script Generation

> Skip if E2E testing is not set up for this project.

Generate a reusable E2E test script if the project has an E2E testing setup. The script should:

- Source `.dev-workflow/ports.env` for dynamic ports
- Use `$BASE_URL` and `$SERVER_URL` (never hardcoded ports)
- Cover the key user flows from the feature

Update the Phase 7 checkbox in the progress file when complete.

---

### Phase 8: Review Results

1. Source `.dev-workflow/ports.env` for correct ports
2. Run any E2E test scripts to verify they pass
3. Present to the user (or note in progress file):
   - Code review from Phase 5
   - Dogfood report from Phase 6 (if run)
   - E2E test results from Phase 7 (if run)
4. If tests fail, loop back to the appropriate phase

Update the Phase 8 checkbox in the progress file when complete.

---

### Phase 9: Git Commit & Push

> **Note:** Do NOT run `/opsx:archive` here. Archive runs on `main` after merge (Phase 13).

1. **Verify you're on a feature branch** (not `main`/`master`).
2. **Stage code changes** — be selective, avoid secrets or temp files. Do not stage `openspec/specs/` files:
   ```bash
   git add <relevant-files>
   ```
3. **Commit:**
   ```bash
   git commit -m "feat: <description of the change>"
   ```
4. **Push:**
   ```bash
   git push -u origin <branch-name>
   ```

Update the Phase 9 checkbox in the progress file when complete.

---

### Phase 10: Create PR

```bash
gh pr create --title "<title>" --body "<body>"
```

Include in the PR body:
- Summary of changes (from proposal)
- Test coverage notes
- Link to manual test plan (if created)

Update the Phase 10 checkbox in the progress file when complete.

---

### Phase 11: PR Review & CI Feedback Loop

Monitor for CI and review feedback.

#### Triage review comments

**Fix** — correctness issues, CI failures, convention violations, security.
**Acknowledge but skip** — style preferences, over-engineering, cosmetic suggestions.
**Discuss** — architectural suggestions that expand scope, conflicting comments.

#### Fix loop

1. Triage all comments
2. Create fix plan at `.dev-workflow/pr-fix-plan-<round>.md`
3. Reply to skipped/discussed comments
4. Implement fixes, re-run tests
5. Commit and push
6. Repeat until CI green and reviews resolved

Update the Phase 11 checkbox per fix round.

---

### Phase 12: Pre-merge Checks & Merge

1. Branch up-to-date with target: `git fetch && git diff HEAD..origin/main --stat`
2. CI checks green
3. No unresolved review comments
4. E2E tests passed (if applicable)
5. Present final status summary
6. **Ask user for confirmation** to merge

Merge:

```bash
gh pr merge <number> --squash --delete-branch
```

Update the Phase 12 checkboxes.

---

## Part E — Post-Merge on Main (Phase 13)

> Run on `main` after the PR merges. Do not run from the feature branch worktree.

---

### Phase 13: Archive & Cleanup on Main

1. **Switch to main and pull:**
   ```bash
   git checkout main && git pull
   ```

2. **Stop the dev server** (from the worktree, if still running):
   ```bash
   source <worktree-path>/.dev-workflow/ports.env 2>/dev/null
   lsof -ti :$SERVER_PORT | xargs kill 2>/dev/null
   lsof -ti :$WEB_PORT | xargs kill 2>/dev/null
   ```

3. **Run archive:**
   ```
   /opsx:archive <change-name>
   ```

4. **Commit the archive:**
   ```bash
   git add openspec/
   git commit -m "chore: archive <change-name>"
   git push
   ```

5. **Remove the worktree:**
   ```bash
   git worktree remove <worktree-path>
   ```

6. **Delete the merged branch:**
   ```bash
   git branch -d <branch-name>
   ```

7. **Prune stale worktree references:**
   ```bash
   git worktree prune
   ```

Update the Phase 13 checkboxes — workflow complete!

---

## Guardrails

- **Never skip the tracking initialization** (Phase 0). Every workflow needs a progress file.
- **Part C main-clean check is mandatory** — abort if `main` has staged or modified files.
- **Never run `/opsx:archive` on the feature branch** — it writes to `openspec/specs/` and causes conflicts when parallel worktrees are active. Archive always runs on `main` in Phase 13.
- **Always confirm with the user** before creating PRs, merging, or pushing to shared branches.
- **The `.dev-workflow/` folder is ephemeral** — gitignored, local to each worktree.
- **Resume support**: If returning to an in-progress workflow, read the progress file.
- **Phase skipping**: Users may ask to skip phases. Update progress file accordingly.
