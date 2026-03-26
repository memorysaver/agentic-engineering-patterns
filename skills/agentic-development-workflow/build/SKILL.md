---
name: build
description: Autonomous feature implementation in a workspace session. Use when a workspace agent starts building, or when the user says "build", "implement", "execute implementation". Covers the full autonomous flow — initialize harness, implement via jj change stack, review, test, create PR, handle review feedback, and merge. Runs in an isolated jj workspace without user interaction.
---

# Build

Autonomous feature implementation inside an isolated jj workspace. Initialize the harness, implement each task via jj change stack, review, test, create a PR, handle feedback, and merge — all without user interaction.

**Where this fits:**

```
/onboard → /scaffold → [ /design → /launch → /build → /wrap ]
                                              ▲ you are here
```

**Session:** Workspace session, autonomous
**Input:** OpenSpec artifacts on disk (committed to main by `/design`)
**Output:** Merged PR

---

## Phase 0: Initialize Tracking

Before any work begins, set up the tracking infrastructure, environment, and jj change stack.

1. **Read the worktree-onboarding guide** at `skills/agentic-development-workflow/build/references/worktree-onboarding.md`.

2. **Discover the OpenSpec change:**
   - List `openspec/changes/` to find the active change
   - Read all artifacts: `proposal.md`, `design.md`, `specs/**/*.md`, `tasks.md`

3. **Create the tracking folder:**
   ```bash
   mkdir -p .dev-workflow
   ```

4. **Add `.dev-workflow/` to `.gitignore`** if not already present:
   ```bash
   grep -q '.dev-workflow' .gitignore || echo '\n# Development workflow tracking (per-workspace)\n.dev-workflow/' >> .gitignore
   ```

5. **Create the progress file** from the template:
   ```bash
   cp skills/agentic-development-workflow/build/references/progress-template.md \
      .dev-workflow/progress-$(jj log --no-graph -r @ -T 'change_id.short(8)').md
   ```
   Fill in feature name, change ID, date, and OpenSpec change name.
   **Mark design phases as pre-completed** (they were done on main via `/design`).

6. **Materialize the OpenSpec tasks as a jj change stack:**

   Read `tasks.md` and create one change per task (skeleton-first pattern):

   ```bash
   jj new main
   jj describe -m "<task-1-description>"
   jj new
   jj describe -m "<task-2-description>"
   jj new
   jj describe -m "<task-3-description>"
   # ... one change per OpenSpec task
   ```

   Record the change IDs in the progress file:
   ```bash
   jj log --no-graph -T 'change_id.short(8) ++ " " ++ description.first_line() ++ "\n"'
   ```

7. **Install dependencies:**
   ```bash
   bun install
   ```

8. **Start the dev server:**
   ```bash
   bun run dev
   ```

9. **Set up port configuration** (if using portless):
   ```bash
   echo "WEB_PORT=3000\nSERVER_PORT=3001\nBASE_URL=http://localhost:3000\nSERVER_URL=http://localhost:3001" > .dev-workflow/ports.env
   ```

10. **Generate sprint contracts:**

    Read `specs/*.md`, `design.md`, and `tasks.md`. For each task in the change stack, generate a contract entry in `.dev-workflow/contracts.md` using the template at `references/contract-template.md`:

    ```markdown
    ## Task: <task-description>
    **Change ID:** <id>
    **Source spec:** <matching spec file>

    ### What will be built
    - [specific files/components]

    ### Success criteria
    - [extracted from matching spec]

    ### Verification steps
    1. [concrete, executable step]
    2. [what to check]
    ```

11. **Generate feature verification list:**

    Extract the verification steps from contracts into `.dev-workflow/feature-verification.json`:

    ```json
    [
      {
        "task": "<task description>",
        "change_id": "<jj change short ID>",
        "verification_steps": ["step 1", "step 2", "step 3"],
        "passes": false,
        "evaluated_by": null,
        "round": null
      }
    ]
    ```

    **Rules:**
    - JSON format is intentional — models tamper with JSON less than Markdown
    - The generator agent **MUST NOT** modify `verification_steps` or `passes` — only the evaluator (or human) does

12. **Generate session recovery script:**

    Create `.dev-workflow/init.sh` for resuming after context resets:

    ```bash
    #!/bin/bash
    # Session recovery script — run this to resume after context reset
    set -e
    cd "$(dirname "$0")/.."

    # Environment
    source .dev-workflow/ports.env 2>/dev/null

    # Dependencies
    bun install --silent

    # Dev server (if not running)
    if ! lsof -ti :${SERVER_PORT:-3001} >/dev/null 2>&1; then
      bun run dev &
      echo "Dev server starting on ${BASE_URL:-http://localhost:3000}"
    else
      echo "Dev server already running on port ${SERVER_PORT:-3001}"
    fi

    # Current state
    echo "=== Change Stack ==="
    jj log --no-graph -T 'change_id.short(8) ++ " " ++ description.first_line() ++ "\n"'

    echo "=== Progress ==="
    grep '\[x\]' .dev-workflow/progress-*.md 2>/dev/null | tail -10

    echo "=== Next Phase ==="
    grep '\[ \]' .dev-workflow/progress-*.md 2>/dev/null | head -3
    ```

    Make executable: `chmod +x .dev-workflow/init.sh`

13. **Initialize inter-agent signals:**

    ```bash
    mkdir -p .dev-workflow/signals
    ```

    Create `.dev-workflow/signals/status.json`:

    ```json
    {
      "phase": 0,
      "phase_name": "initializing",
      "task_current": null,
      "task_index": 0,
      "task_total": 0,
      "started_at": "<ISO 8601 timestamp>",
      "blockers": [],
      "completion_pct": 0,
      "last_updated": "<ISO 8601 timestamp>"
    }
    ```

    Check for feedback from main session:
    ```bash
    cat .dev-workflow/signals/feedback.md 2>/dev/null
    ```

    See `skills/agentic-development-workflow/launch/references/signals-spec.md` for the full signal file specification.

Update the Phase 0 checkbox in the progress file when done.

> **Signal update:** Update `.dev-workflow/signals/status.json` with `"phase": 0, "phase_name": "initialized", "completion_pct": 10`.

---

## Story Status Tracking

If this feature corresponds to a story in `product-context.yaml` (check if the OpenSpec change name matches a story's `openspec_change` field):

- **Phase 0 start:** Confirm story status is `in_progress` in the YAML
- **Phase 12 merge:** Update story in YAML:
  - `status: completed`
  - `completed_at: <ISO 8601 now>`
  - `pr_url: <PR URL>`
  - `cost_usd: <accumulated cost>`
- **On failure (escalation):** Update story:
  - `status: failed`
  - Append to `failure_logs`
- **After status update:** Check if any `pending` stories should transition to `ready` (all dependencies now completed)

Update the YAML and commit:
```bash
# Update product-context.yaml with new story status
git add product-context.yaml
git commit -m "chore: update story <id> status to completed"
```

If `product-context.yaml` doesn't exist, skip this tracking (standalone feature mode).

---

## Phase 4: OpenSpec Apply

Implement each task by editing its corresponding jj change:

```bash
# For each task in the change stack:
jj edit <change-id>       # Jump to the task's change
# ... implement the task ...
jj diff                   # Verify the change matches the task
# ... run targeted tests for this change ...
# Move to next task
```

Invoke the apply skill for guidance on implementing each task:

```
/opsx:apply
```

The agent works **one change at a time**. Auto-rebase keeps dependent changes consistent — when you edit an earlier change, all later changes automatically update.

Update the progress file checkbox for each completed task, and mark the Phase 4 checkbox when all tasks are done.

> **Signal update:** Update `.dev-workflow/signals/status.json` with `"phase": 4` at start, update `task_current`, `task_index`, `task_total` as tasks progress, and `completion_pct` proportionally.

---

## Phase 5: Code Review & Verification

After implementation, verify the code before moving to testing.

### Completeness check (always done by generator)

1. Re-read the proposal (including any design review adjustments)
2. Walk through each change in the stack, reviewing with `jj diff -r <change>` against its task description
3. Check `.dev-workflow/contracts.md` — verify each task's success criteria are met
4. If any task is incomplete, `jj edit <change>` and loop back to Phase 4

### Quality review

**With separate evaluator (full mode):**

If an evaluator agent was set up via `/launch`, delegate quality review to it:

1. Write `.dev-workflow/signals/eval-request.md` describing what to evaluate
2. Wait for `.dev-workflow/signals/eval-response-1.md`
3. Read the response — fix any FAIL items
4. Write a new eval-request for the next round
5. Repeat until all dimensions pass (see `skills/agentic-development-workflow/launch/references/evaluator-criteria.md` for thresholds)
6. Max 5 rounds — escalate to human if not converging

The evaluator also updates `.dev-workflow/feature-verification.json` with pass/fail results.

**Without evaluator (light mode):**

Self-review with awareness of its limitations:

1. **Correctness** — Logic errors, off-by-one bugs, missing edge cases?
2. **Security** — Input validation, auth checks, SQL parameterization?
3. **Performance** — N+1 queries, missing indexes, unbounded loops?
4. **Conventions** — Naming, file structure, error handling, imports?

> **Note:** Self-review tends to be lenient. If using light mode, be extra critical and walk through `feature-verification.json` steps manually.

Document findings in `.dev-workflow/code-review-<feature>.md`. Fix any issues found.

Update the Phase 5 checkbox in the progress file when complete.

> **Signal update:** Update `.dev-workflow/signals/status.json` with `"phase": 5, "phase_name": "code-review"`.

---

## Phase 6: Browser Testing (Dogfood)

> Skip if `agent-browser` is not installed. **Light mode:** Skip this phase.

**Port configuration:** Source `.dev-workflow/ports.env` to get the correct URLs:

```bash
source .dev-workflow/ports.env
```

Use agent-browser to systematically explore and test the application:

```
/agent-browser:dogfood
```

Document results in `.dev-workflow/dogfood-<feature>.md`.

> **Signal update:** Update `.dev-workflow/signals/status.json` with `"phase": 6, "phase_name": "dogfood-testing"`.

---

## Phase 7: E2E Test Script Generation

> Skip if E2E testing is not set up for this project. **Light mode:** Skip this phase.

Generate a reusable E2E test script if the project has an E2E testing setup. The script should:

- Source `.dev-workflow/ports.env` for dynamic ports
- Use `$BASE_URL` and `$SERVER_URL` (never hardcoded ports)
- Cover the key user flows from the feature

---

## Phase 8: Review Results

> **Light mode:** Skip this phase.

1. Source `.dev-workflow/ports.env` for correct ports
2. Run any E2E test scripts to verify they pass
3. Present to the user (or note in progress file):
   - Code review from Phase 5
   - Dogfood report from Phase 6 (if run)
   - E2E test results from Phase 7 (if run)
4. If tests fail, loop back to the appropriate phase

> **Signal update:** Update `.dev-workflow/signals/status.json` with `"phase": 8, "phase_name": "review-results"`.

---

## Phase 9: Cleanup & Publish

> **Note:** Do NOT run `/opsx:archive` here. Archive runs on `main` after merge (via `/wrap`).

### 1. Clean up the change stack

```bash
jj log   # Review the full stack
```

- **Split** any change that mixed multiple concerns:
  ```bash
  jj edit <change>
  jj split
  ```

- **Squash** any fix that belongs in an earlier change:
  ```bash
  jj squash
  ```

### 2. Rebase onto latest main

```bash
jj git fetch
jj rebase -d main@origin
```

### 3. Create bookmark and push

```bash
jj bookmark create feat-<name> -r @-
jj git push --bookmark feat-<name>
```

---

## Phase 10: Create PR

```bash
gh pr create --title "<title>" --body "<body>"
```

Include in the PR body:
- Summary of changes (from proposal)
- Test coverage notes
- Link to manual test plan (if created)

---

## Phase 11: PR Review & CI Feedback Loop

Monitor for CI and review feedback.

### Triage review comments

**Fix** — correctness issues, CI failures, convention violations, security.
**Acknowledge but skip** — style preferences, over-engineering, cosmetic suggestions.
**Discuss** — architectural suggestions that expand scope, conflicting comments.

### Fix loop

1. Triage all comments
2. Create fix plan at `.dev-workflow/pr-fix-plan-<round>.md`
3. Reply to skipped/discussed comments
4. **Edit the correct change** — identify which change owns each fix:
   ```bash
   jj edit <change-that-needs-fixing>
   # ... make the fix ...
   jj squash   # if fix is in a new change, fold into the right parent
   ```
5. Re-run tests
6. Re-push:
   ```bash
   jj git push --bookmark feat-<name>
   ```
7. Repeat until CI green and reviews resolved

---

## Phase 11.5: Human Evaluation & Iteration

After PR review fixes are resolved, the human tester evaluates the feature — typically by running the app from the workspace. If they find minor issues (UX tweaks, missing edge cases, behavior that doesn't match intent), this phase handles the iteration loop.

> **If no issues found:** Skip this phase and proceed to Phase 12.

### Iteration round

1. **Document findings** — Write to `.dev-workflow/human-eval-round-<N>.md`:
   - What was found (description, steps to reproduce)
   - Severity (minor / moderate)
   - Category (UX, logic, edge case, visual)

2. **Fix in the correct change** — Identify which jj change owns each fix:
   ```bash
   jj edit <change-that-needs-fixing>
   # ... make the fix ...
   jj squash   # if fix is in a new change, fold into the right parent
   ```

3. **Align OpenSpec change** — Update `openspec/changes/<name>/` artifacts:
   - Add completed tasks to `tasks.md` for the work just done
   - Update `specs/` if behavior changed
   - Update `design.md` only if approach details shifted
   - Keep `proposal.md` scope as-is (direction unchanged)

4. **Re-test** — Re-run Phase 5 (code review) and Phase 6 (dogfood) on the changed areas.

5. **Push** — Update the PR:
   ```bash
   jj git push --bookmark feat-<name>
   ```

6. **Repeat** — If the human tester finds more issues, start a new round.

> **Signal update:** Create `.dev-workflow/signals/ready-for-review.flag` when ready for human evaluation. Update `status.json` with `"phase": 11.5, "phase_name": "human-evaluation"`.

---

## Phase 12: Pre-merge Checks & Merge

1. Up-to-date with main: `jj git fetch && jj rebase -d main@origin`
2. CI checks green
3. No unresolved review comments
4. E2E tests passed (if applicable)
5. Present final status summary
6. **Ask user for confirmation** to merge

Merge:

```bash
gh pr merge <number> --squash --delete-branch
```

---

## Guardrails

- **Never skip the tracking initialization** (Phase 0). Every workflow needs a progress file and jj change stack.
- **Never run `/opsx:archive` from a workspace** — it writes to `openspec/specs/` and causes conflicts. Archive always runs on `main` via `/wrap`.
- **Always confirm with the user** before creating PRs, merging, or pushing to shared branches.
- **The `.dev-workflow/` folder is ephemeral** — gitignored, local to each workspace.
- **Resume support**: If returning to an in-progress workflow, run `.dev-workflow/init.sh` if it exists, then read the progress file.
- **Phase skipping**: Users may ask to skip phases. Update progress file accordingly.
- **Always use `jj` for local changes** — never use raw `git commit` or `git add` in a colocated repo.
- **Bookmarks are publish-time only** — create them in Phase 9 when ready to push, not during implementation.
- **Signal updates are required** — update `.dev-workflow/signals/status.json` at the start and end of every phase. Check `.dev-workflow/signals/feedback.md` for main session feedback at phase boundaries.
- **Generator must not modify verification data** — never modify `verification_steps` or `passes` in `feature-verification.json`. Only the evaluator or human updates these fields.
- **Evaluator loop max 5 rounds** — if the generator-evaluator loop hasn't converged after 5 rounds, escalate to human.

---

## Next Step

After merge, signal the main session to run:

```
/wrap
```
