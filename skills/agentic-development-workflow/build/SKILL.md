---
name: aep-build
description: Autonomous feature implementation in a workspace session. Use when a workspace agent starts building, or when the user says "build", "implement", "execute implementation". Covers the full autonomous flow — initialize harness, implement linearly with one commit per task, review, test, create PR, handle review feedback, and merge. Runs in an isolated git worktree on a feature branch without user interaction.
---

# Build

Autonomous feature implementation inside an isolated git worktree on a fresh `feat/<name>` branch. Initialize the harness, implement tasks linearly (one commit per `tasks.md` row), review, test, create a PR, handle feedback, and merge — all without user interaction.

> **Phase numbering note:** Phases 1-3 (explore, propose, review) were completed on main via `/aep-design`. This skill begins at Phase 0 (workspace init) and continues from Phase 4 (implementation).

**Where this fits:**

```
/aep-onboard → /aep-scaffold → [ /aep-design → /aep-launch → /aep-build → /aep-wrap ]
                                              ▲ you are here
```

**Session:** Workspace session, autonomous
**Input:** OpenSpec artifacts on disk (committed to main by `/aep-design`)
**Output:** Merged PR

---

## Phase 0: Initialize Tracking

Before any work begins, set up the tracking infrastructure and environment. The branch is already created by `/aep-launch` (`feat/<name>`); you do not pre-create commits — implement linearly in Phase 4.

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
      .dev-workflow/progress-$(git rev-parse --short HEAD).md
   ```

   Fill in feature name, base commit SHA, date, and OpenSpec change name.
   **Mark design phases as pre-completed** (they were done on main via `/aep-design`).

6. **Read tasks.md** to understand the task list:

   ```bash
   cat openspec/changes/<change-name>/tasks.md
   ```

   `tasks.md` _is_ the skeleton. You will implement tasks linearly in Phase 4, committing once per task with conventional-commit messages — the resulting commit history will mirror the task list 1:1.

7. **Run project setup** (if a setup hook exists):

   ```bash
   SETUP_HOOK=.claude/hooks/workspace-setup.sh
   if [ -f "$SETUP_HOOK" ]; then
     bash "$SETUP_HOOK"
   else
     echo "No workspace setup hook found at $SETUP_HOOK"
     echo "Read the project README or ask the user for setup instructions."
   fi
   ```

   The project's setup hook handles all project-specific concerns:
   - Package installation (bun/npm/pnpm/cargo/poetry/etc.)
   - Dev server startup
   - Port assignment → writes `.dev-workflow/ports.env`
   - Database migrations, seeding
   - Docker/container management
   - `.env` file validation

   **Contract:** The hook MUST write `.dev-workflow/ports.env` with at minimum:

   ```
   WEB_PORT=<port>
   SERVER_PORT=<port>
   BASE_URL=http://localhost:<web-port>
   SERVER_URL=http://localhost:<server-port>
   ```

   If no hook exists and no README provides instructions, ask the user how to set up the project.

8. **Generate sprint contracts:**

   Read `specs/*.md`, `design.md`, and `tasks.md`. For each task, generate a contract entry in `.dev-workflow/contracts.md` using the template at `references/contract-template.md`:

   ```markdown
   ## Task: <task-description>

   **Source spec:** <matching spec file>

   ### What will be built

   - [specific files/components]

   ### Success criteria

   - [extracted from matching spec]

   ### Verification steps

   1. [concrete, executable step]
   2. [what to check]
   ```

9. **Generate feature verification list:**

   Extract the verification steps from contracts into `.dev-workflow/feature-verification.json`:

   ```json
   [
     {
       "task": "<task description>",
       "commit_sha": null,
       "verification_steps": ["step 1", "step 2", "step 3"],
       "passes": false,
       "evaluated_by": null,
       "round": null
     }
   ]
   ```

   `commit_sha` starts as `null` and is filled in (8-char prefix) after each task is committed in Phase 4.

   **Rules:**
   - JSON format is intentional — models tamper with JSON less than Markdown
   - The generator agent **MUST NOT** modify `verification_steps` or `passes` — only the evaluator (or human) does

10. **Generate session recovery script:**

    Create `.dev-workflow/init.sh` for resuming after context resets:

    ```bash
    #!/bin/bash
    # Session recovery script — run this to resume after context reset
    set -e
    cd "$(dirname "$0")/.."

    # Project setup (deps, dev server, ports)
    SETUP_HOOK=.claude/hooks/workspace-setup.sh
    if [ -f "$SETUP_HOOK" ]; then
      bash "$SETUP_HOOK"
    else
      echo "No workspace setup hook found. Check project README for setup instructions."
    fi

    # Source ports (written by setup hook)
    source .dev-workflow/ports.env 2>/dev/null

    # Current state
    echo "=== Branch & Commits ==="
    echo "Branch: $(git branch --show-current)"
    git log --oneline "$(git config --get aep.integration-branch 2>/dev/null || (git show-ref --verify --quiet refs/remotes/origin/develop && echo develop || echo main))"..HEAD 2>/dev/null || git log --oneline -10

    echo "=== Progress ==="
    grep '\[x\]' .dev-workflow/progress-*.md 2>/dev/null | tail -10

    echo "=== Next Phase ==="
    grep '\[ \]' .dev-workflow/progress-*.md 2>/dev/null | head -3
    ```

    Make executable: `chmod +x .dev-workflow/init.sh`

11. **Initialize inter-agent signals:**

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
      "last_updated": "<ISO 8601 timestamp>",
      "story_status": "in_progress",
      "pr_url": null,
      "cost_usd": null,
      "completed_at": null,
      "failure_log": null,
      "blocked_on": null
    }
    ```

    Check for feedback from main session:

    ```bash
    cat .dev-workflow/signals/feedback.md 2>/dev/null
    ```

    See `skills/agentic-development-workflow/launch/references/signals-spec.md` for the full signal file specification.

12. **Create the lessons file:**

    ```bash
    cat > .dev-workflow/lessons.md <<'TEMPLATE'
    # Lessons: <change-name>

    Module: <module>
    Activity: <activity>
    Date: <date>
    Story: <story-id>

    ## Solutions

    ## Errors

    ## Missing

    ## Summary for Next Agent
    TEMPLATE
    ```

    This file captures what the agent learns during builds — solutions discovered, errors encountered, missing docs, patterns that worked. Fill in the header fields from the OpenSpec change. Sections are populated during Phase 4 (opt-in) and Phase 9 (summary).

Update the Phase 0 checkbox in the progress file when done.

> **Signal update:** Update `.dev-workflow/signals/status.json` with `"phase": 0, "phase_name": "initialized", "completion_pct": 10`.

---

## Story Status Tracking (Product-Cycle Mode Only)

> **Standalone mode:** If `product-context.yaml` doesn't exist, skip this entire section. Signal updates (status.json) still work — just omit `story_status` fields.

If this feature corresponds to a story in `product-context.yaml` (check if the OpenSpec change name matches a story's `openspec_change` field):

### Build-Time Dependency Re-Verification (Phase 0)

Before starting implementation, re-verify that all dependencies are still completed:

```
Read product-context.yaml (READ-ONLY — workspace agents never write to this file)
Find this story by openspec_change match
For each dependency in story.dependencies:
  If dependency.status != completed:
    ABORT build
    Signal via .dev-workflow/signals/status.json:
      { "phase": 0, "phase_name": "dependency_check_failed",
        "blockers": ["<dep_id> is not completed"] }
    Stop and wait for main session to investigate
```

Also check `dispatched_at_epoch` vs current `dispatch_epoch`. If the epoch advanced since dispatch, re-read the YAML to check for architecture amendments.

**Why:** A dependency could be rolled back after dispatch (e.g., PR reverted). This defense-in-depth catches issues that dispatch-time checks missed.

### Status Updates via Signals

> **CONCURRENCY PROTOCOL:** Only the main session writes to `product-context.yaml`. Workspace agents report status through `.dev-workflow/signals/status.json`. The main session (via `/aep-wrap`, `/aep-dispatch`) reads signals and updates the YAML.

All story status updates flow through the signal file, NOT direct YAML writes:

- **Phase 0 start:** Confirm story status is `in_progress` in the YAML (read-only check)
- **Phase 10 (PR created):** Update `status.json` with `story_status: "in_review"` and `pr_url`
- **Phase 12 merge:** Update `status.json` with `story_status: "completed"`, `completed_at`, `pr_url`, `cost_usd`
- **On failure (escalation):** Update `status.json` with `story_status: "failed"` and `failure_log` (structured record: error_class, approach_summary, unexplored_alternatives)

`/aep-wrap` (running on main) reads these signal fields and writes the final status to `product-context.yaml`.

If `product-context.yaml` doesn't exist, skip this tracking (standalone feature mode).

---

## Phase 4: OpenSpec Apply

Implement each task in `tasks.md` linearly. After each task, commit with a conventional-commit message and record the resulting commit SHA in `feature-verification.json`:

```bash
# For each task in tasks.md, in order:
# ... implement the task — edit files directly ...
git status                                  # Inspect changes
git diff                                    # Verify the change matches the task
# ... run targeted tests for this task ...
git add -A
git commit -m "feat(<scope>): <task description>"

# Record the commit SHA in feature-verification.json:
SHA=$(git rev-parse --short HEAD)
# Update the matching task entry's commit_sha field with $SHA
# (use jq or a small inline script — never modify verification_steps or passes)
```

Invoke the apply skill for guidance on implementing each task:

```
/opsx:apply
```

The agent works **one task at a time**. Each task = one commit. The branch's commit history mirrors `tasks.md` 1:1, which makes per-task code review on the PR straightforward.

Update the progress file checkbox for each completed task, and mark the Phase 4 checkbox when all tasks are done.

> **If you need to amend a just-committed task** (e.g., you forgot a file), use `git commit --amend --no-edit` _before_ moving on. Once you've committed the next task, leave prior commits alone — fixes belong as a follow-up commit, not a rewrite.

**Lesson capture (opt-in per task):** After completing each task, if you encountered something noteworthy — a non-obvious solution, an error that took multiple attempts, missing documentation — append to `.dev-workflow/lessons.md` under the appropriate section (`## Solutions`, `## Errors`, or `## Missing`). Use a `### <title>` sub-heading with a brief explanation. This is not mandatory for every task — only write when something is genuinely worth passing on to the next agent.

> **Signal update:** Update `.dev-workflow/signals/status.json` with `"phase": 4` at start, update `task_current`, `task_index`, `task_total` as tasks progress, and `completion_pct` proportionally.

---

## Phase 5: Code Review & Verification

After implementation, verify the code before moving to testing. This phase uses the **generator/evaluator pattern** — read the shared utility for full details:

- **Scoring framework:** `.claude/skills/aep-gen-eval/references/scoring-framework.md` (dimensions, thresholds, presets)
- **Agent contracts:** `.claude/skills/aep-gen-eval/references/agent-contracts.md` (role separation, prompt templates)
- **Eval protocol:** `.claude/skills/aep-gen-eval/references/eval-protocol.md` (request/response format, verification JSON, convergence rules)

### Completeness check (always done by generator)

1. Re-read the proposal (including any design review adjustments)
2. Walk through each task's commit, reviewing with `git show <commit-sha>` against its task description
3. Check `.dev-workflow/contracts.md` — verify each task's success criteria are met
4. If any task is incomplete, add a follow-up commit (`feat(<scope>): complete <task>` or `fix(<scope>): <issue>`) and loop back to Phase 4. Do not rewrite prior commits.

### Quality review

**With separate evaluator (full mode):**

If `.dev-workflow/evaluator-criteria.md` exists (written during `/aep-launch`), spawn an evaluator via `executor.spawn_evaluator()`. The generator orchestrates the entire evaluation loop — no manual intervention needed. The spawn mechanism tracks the active executor mode (read `.claude/skills/aep-executor/references/backends.md`):

| Generator mode                  | Evaluator spawn (`executor.spawn_evaluator`)                                                                                        | eval-protocol mechanism                                                           |
| ------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| **claude-team / claude-bg**     | **foreground Task subagent** in the generator's own session — inherits the worktree cwd; the evaluator prompt _is_ the spawn prompt | **Context B mechanism**, worktree-bound — _not_ its read-only `/aep-validate` use |
| **codex-subagent / codex-exec** | `codex exec --cd <abs worktree>` with the `aep-evaluator` role — enforced cwd, bounded one-shot                                     | **Context C mechanism**, in-host headless — _not_ API/SDK CI                      |
| **legacy** (tmux)               | `tmux split-window` — evaluator in a bottom pane                                                                                    | **Context A** (tmux split)                                                        |
| **workflow**                    | the workflow's `verify` stage (worktree-isolated)                                                                                   | **Context C mechanism**, in-host — _not_ API/SDK CI                               |

> **Always worktree-bound.** Whatever the mode, the evaluator runs against this
> workspace's worktree (files + git state), per `executor.spawn_evaluator()`.
> The Context labels name the spawn _mechanism_, not the read-only/CI _use
> cases_ those contexts describe in eval-protocol — a Task-subagent evaluator
> is **not** a main-session read-only reviewer, and a `codex exec` evaluator is
> **not** an API/SDK CI job. It is worktree-bound review.

For the native modes the evaluator prompt is delivered **at spawn time** — there
is no readiness wait, no separate send step, and no pane to kill; the agent/exec
returns when `eval-response-<N>.md` is written. The legacy tmux recipe is shown
below for pinned-tmux workspaces; the request/response signal files and
convergence rules are identical across modes.

> **Why tmux splits, not cmux splits (legacy):** The generator runs inside tmux but was not spawned by cmux,
> so it cannot use cmux socket commands. Use `tmux split-window` instead — under cmux the surface attached
> to the tmux session will display both panes automatically.

#### Evaluation round

For each round N (starting at 1, max 5):

1. **Write eval-request** — create `.dev-workflow/signals/eval-request.md` per the format in `eval-protocol.md` (Signal Files section).

2. **Compose the evaluator prompt** from `agent-contracts.md` (Evaluator Prompt — Code Quality template). Customize with the workspace paths:

   ```
   EVAL_PROMPT = <evaluator prompt from agent-contracts.md, customized with:
     criteria_file=.dev-workflow/evaluator-criteria.md
     eval_request_file=.dev-workflow/signals/eval-request.md
     spec_directory=openspec/changes/<change-name>/
     contracts_file=.dev-workflow/contracts.md
     verification_file=.dev-workflow/feature-verification.json
     eval_response_file=.dev-workflow/signals/eval-response-<N>.md
   >
   ```

3. **Spawn the evaluator (mode-dispatched, prompt = spawn prompt):**
   - **claude-team / claude-bg:** spawn a **foreground Task subagent** with
     `EVAL_PROMPT`. It inherits the worktree cwd and returns when done — no
     sleep, no send step, no teardown.
   - **codex-subagent / codex-exec:**

     ```bash
     codex exec --cd "$(pwd)" --dangerously-bypass-approvals-and-sandbox \
       "$EVAL_PROMPT" < /dev/null
     ```

     (With the `aep-evaluator` role committed in `.codex/agents/`, prefer
     `spawn_agent(agent_type: "aep-evaluator", ...)` from a living thread.)

   - **legacy (pinned tmux):**

     ```bash
     # Split current tmux window vertically (top=generator, bottom=evaluator). The evaluator
     # needs to read files and write eval-response, so it runs the INTERACTIVE executor.
     tmux split-window -v -c "$(pwd)" "${EXECUTOR:-claude --dangerously-skip-permissions}"
     tmux select-pane -t :.0       # return focus to the generator pane

     sleep 10
     # Multi-line prompt: send literally with -l, then a single Enter.
     tmux send-keys -t :.1 -l -- "$EVAL_PROMPT"
     tmux send-keys -t :.1 Enter
     ```

4. **Confirm the response exists** (the native spawns return on completion; the
   legacy pane needs a poll):

   ```bash
   while [ ! -f .dev-workflow/signals/eval-response-<N>.md ]; do sleep 15; done
   ```

5. **Read the response.** Legacy only: close the evaluator pane
   (`tmux kill-pane -t :.1`). Native evaluators have already exited.

6. **Fix FAIL items** — add follow-up commits addressing each FAIL item, then loop back to step 1 with round N+1. Do not rewrite history; the PR review should see the fix as new commits on top.

7. **Max 5 rounds** — if not converging, escalate to human via the **human-gate
   protocol** (see below) and the convergence rules in `eval-protocol.md`.

The evaluator also updates `.dev-workflow/feature-verification.json` with pass/fail results per the field ownership rules in `eval-protocol.md`.

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

### The Human-Gate Protocol (any phase)

When you hit a decision **only the human can make** — design ambiguity the spec
doesn't resolve, eval non-convergence after max rounds, a product judgment call —
do not guess and do not silently stall. Raise a gate:

1. **Record it (always):** append to `.dev-workflow/signals/needs-human.md`:

   ```markdown
   ## <ISO8601> — Phase <N>

   **Question:** <the decision needed, with the options you considered>
   **Context:** <why you can't decide autonomously>
   ```

   and set `"blocked_on": "human"` in `status.json`.

2. **Raise it on your mode's transport** (how you were launched tells you the
   mode — see the Human-Gate Protocol in `aep-executor/references/backends.md`).
   The answer always comes back through the **main agent** (hub-and-spoke) —
   you never need the human to visit your surface:
   - **claude-team:** `SendMessage` the team lead, message prefixed `HUMAN_GATE:`
   - **codex-subagent:** ask the parent thread (approvals surface natively)
   - **legacy:** the file is the transport — the orchestrator detects it and
     relays the answer via nudge
   - **claude-bg / codex-exec / workflow / headless:** **gate-and-park** — no
     push channel reaches you, so after recording the gate: commit WIP (or
     leave the tree clean), update `status.json`, and **end your run cleanly**
     (workflow agents: return a structured `gated` result carrying the
     question). The orchestrator will resume a worker into this same worktree
     with the human's answer — your state is in the worktree +
     `.dev-workflow/`, so nothing is lost.

3. **Block-in-place modes only:** continue what you can that doesn't depend on
   the answer; otherwise wait, re-checking `feedback.md`.

4. **On answer** (delivered by push, or in your resume prompt after a park):
   append `resolved: <summary>` under your entry, clear `blocked_on`, and
   proceed.

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

> **Note:** Do NOT run `/opsx:archive` here. Archive runs on the integration branch after merge (via `/aep-wrap`).

### 0. Write lesson summary

Before publishing, write a final `## Summary for Next Agent` section in `.dev-workflow/lessons.md` (1-3 sentences): what would you tell the next agent building in this module? If the lessons file has no entries beyond the template header, write the summary anyway — even "straightforward implementation, no surprises" is useful signal.

### 1. Review the commit history

```bash
git log --oneline "$(git config --get aep.integration-branch 2>/dev/null || (git show-ref --verify --quiet refs/remotes/origin/develop && echo develop || echo main))"..HEAD
```

The history should be a clean linear sequence: one commit per `tasks.md` task, optionally followed by review-fix commits. The PR will be squash-merged on merge, so per-commit hygiene matters for review readability, not the integration branch's history.

### 2. Rebase onto the latest integration branch

```bash
# Resolve $BASE (integration branch) — see git-ref "Integration Branch" (override → develop → main)
BASE=$(git config --get aep.integration-branch 2>/dev/null || true)
[ -z "$BASE" ] && { git show-ref --verify --quiet refs/heads/develop \
  || git show-ref --verify --quiet refs/remotes/origin/develop; } && BASE=develop
BASE=${BASE:-main}

git fetch origin
git rebase origin/"$BASE"
```

If conflicts arise, resolve them in the working tree, then `git add <files>` and `git rebase --continue`. Abort with `git rebase --abort` if conflicts are too tangled — surface to the orchestrator via the signal file.

### 3. Push the feature branch

```bash
git push -u origin feat/<name>
```

The `-u` (`--set-upstream`) flag is needed only on the first push; subsequent pushes can drop it.

---

## Phase 10: Create PR / MR

**Auto-detect the platform and target the integration branch:**

```bash
# Resolve $BASE (integration branch) — see git-ref "Integration Branch" (override → develop → main)
BASE=$(git config --get aep.integration-branch 2>/dev/null || true)
[ -z "$BASE" ] && { git show-ref --verify --quiet refs/heads/develop \
  || git show-ref --verify --quiet refs/remotes/origin/develop; } && BASE=develop
BASE=${BASE:-main}

REMOTE_URL=$(git remote get-url origin)
case "$REMOTE_URL" in
  *github.com*) gh pr create --title "<title>" --body "<body>" --base "$BASE" ;;
  *gitlab*)     glab mr create --title "<title>" --description "<body>" --target-branch "$BASE" ;;
  *)
    # Self-hosted GitHub Enterprise / Bitbucket / other host — pattern not matched.
    # Do NOT silently no-op: open the PR/MR manually with the correct tool, always
    # passing the base explicitly (--base "$BASE" / --target-branch "$BASE").
    echo "Unrecognized remote host: $REMOTE_URL — create the PR/MR manually with base \"$BASE\"." >&2
    exit 1
    ;;
esac
```

> **CRITICAL — always specify `--base "$BASE"` (GitHub) or `--target-branch "$BASE"` (GitLab)**, resolving `$BASE` in the same block as above. Workspace sessions run from a worktree whose checked-out branch is `feat/<name>`, not the integration branch. Without an explicit base, `gh pr create` may infer the wrong base from the most recent merged branch — causing the PR to target a stale base and never land on the integration branch even after merge. In two-branch mode an inferred base could be production `main`, which AEP must never merge feature work into directly.

Include in the PR/MR body:

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
4. **Add follow-up commits** for each fix:
   ```bash
   # ... make the fix ...
   git add -A
   git commit -m "fix(<scope>): address review feedback on <topic>"
   ```
   Squash-merge at PR-merge time keeps the integration branch's history clean, so per-commit hygiene on the feature branch only matters for reviewer readability.
5. Re-run tests
6. Re-push:
   ```bash
   git push origin feat/<name>
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

2. **Add a follow-up commit per fix:**

   ```bash
   # ... make the fix ...
   git add -A
   git commit -m "fix(<scope>): <human-eval finding>"
   ```

3. **Align OpenSpec change** — Update `openspec/changes/<name>/` artifacts:
   - Add completed tasks to `tasks.md` for the work just done
   - Update `specs/` if behavior changed
   - Update `design.md` only if approach details shifted
   - Keep `proposal.md` scope as-is (direction unchanged)

4. **Re-test** — Re-run Phase 5 (code review) and Phase 6 (dogfood) on the changed areas.

5. **Push** — Update the PR:

   ```bash
   git push origin feat/<name>
   ```

6. **Repeat** — If the human tester finds more issues, start a new round.

> **Signal update:** Create `.dev-workflow/signals/ready-for-review.flag` when ready for human evaluation. Update `status.json` with `"phase": 11.5, "phase_name": "human-evaluation"`. This is a human gate — also follow the Human-Gate Protocol (needs-human.md + `blocked_on: "human"` + your mode's transport) so the orchestrator surfaces it instead of counting you as stuck.

---

## Phase 12: Pre-merge Checks & Merge

1. Up-to-date with the integration branch:
   ```bash
   # Resolve $BASE — see git-ref "Integration Branch" (override → develop → main)
   BASE=$(git config --get aep.integration-branch 2>/dev/null || true)
   [ -z "$BASE" ] && { git show-ref --verify --quiet refs/heads/develop \
     || git show-ref --verify --quiet refs/remotes/origin/develop; } && BASE=develop
   BASE=${BASE:-main}
   git fetch origin && git rebase origin/"$BASE" && git push --force-with-lease origin feat/<name>
   ```
2. CI checks green
3. No unresolved review comments
4. E2E tests passed (if applicable)
5. Present final status summary
6. **Merge decision:**
   - **Interactive mode** (user present in session): Ask user for confirmation before merging
   - **Autopilot mode** (launched via `/aep-launch` into `.feature-workspaces/`): Merge immediately when all pre-merge checks pass — do not wait for user confirmation. The autopilot orchestrator monitors via signals, not interactive prompts.
   - **Detection:** If your working directory is inside `.feature-workspaces/`, you are in autopilot mode.

Merge:

```bash
REMOTE_URL=$(git remote get-url origin)
```

- GitHub: `gh pr merge <number> --squash --delete-branch`
- GitLab: `glab mr merge <number> --squash --remove-source-branch`

> **Why `--force-with-lease`:** rebasing rewrites the feature branch's commit SHAs. `--force-with-lease` forces the push only if the remote hasn't advanced since you last fetched — protecting concurrent collaborators while still letting you push the rebased history.

---

## Guardrails

- **Never skip the tracking initialization** (Phase 0). Every workflow needs a progress file and contracts.
- **Never run `/opsx:archive` from a workspace** — it writes to `openspec/specs/` and causes conflicts. Archive always runs on the integration branch via `/aep-wrap`.
- **Never write to `product-context.yaml` from a workspace** — only the main session writes to the YAML. Report all status through `.dev-workflow/signals/status.json`. This is the concurrency protocol.
- **Always confirm with the user** before creating PRs, merging, or pushing to shared branches.
- **The `.dev-workflow/` folder is ephemeral** — gitignored, local to each workspace.
- **Resume support**: If returning to an in-progress workflow, run `.dev-workflow/init.sh` if it exists, then read the progress file.
- **Phase skipping**: Users may ask to skip phases. Update progress file accordingly.
- **One commit per task** in Phase 4 — the PR review reads cleanly when the commit list mirrors `tasks.md`. Don't bundle multiple tasks into one commit; don't split one task across multiple commits.
- **Don't rewrite history mid-stream** — once you've moved past a committed task, fixes go in as new commits, not amends or rebases. Squash-merge at PR-merge handles the cleanup.
- **Use `git push --force-with-lease`, never `--force`** — the `lease` variant fails safely if someone else pushed to the same branch since your last fetch.
- **Signal updates are required** — update `.dev-workflow/signals/status.json` at the start and end of every phase. Check `.dev-workflow/signals/feedback.md` for main session feedback at phase boundaries.
- **Generator must not modify verification data** — never modify `verification_steps` or `passes` in `feature-verification.json`. Only `commit_sha` is generator-writable. The evaluator or human updates `passes` / `evaluated_by` / `round`.
- **Evaluator loop max 5 rounds** — if the generator-evaluator loop hasn't converged after 5 rounds, escalate to human.
- **Raise human gates, don't guess** — decisions only the human can make go through the Human-Gate Protocol (`needs-human.md` + `blocked_on: "human"` + your mode's transport). Silent stalls read as stuck; unrecorded guesses read as scope drift.

---

## Next Step

After merge, signal the main session to run:

```
/aep-wrap
```
