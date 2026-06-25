---
name: aep-wrap
description: Post-merge archive and cleanup. Use after a PR has been merged, or when the user says "wrap up", "archive", "cleanup after merge", "post-merge". Runs OpenSpec archive, commits the archive, and removes the workspace. The final step in the feature lifecycle.
---

# Wrap

Post-merge archive and workspace cleanup. Run this on the **integration branch** (`$BASE` — `main` in single-branch mode, `develop` in two-branch mode; see [git-ref](../git-ref/SKILL.md) → "Integration Branch") after the PR merges to archive the OpenSpec change and clean up the workspace.

**Where this fits:**

```
/aep-onboard → /aep-scaffold → [ /aep-design → /aep-launch → /aep-build → /aep-wrap ]
                                                        ▲ you are here
```

**Session:** Main session, post-merge
**Input:** Merged PR notification
**Output:** Archived OpenSpec change, cleaned up workspace

---

## Phase 13: Archive & Cleanup on the Integration Branch

> Run on the integration branch (`$BASE`) after the PR merges. Do not run from the workspace.

### 1. Fetch merged state and fast-forward the integration branch

```bash
# Resolve $BASE — see git-ref "Integration Branch" (override → develop → main)
BASE=$(git config --get aep.integration-branch 2>/dev/null || true)
[ -z "$BASE" ] && { git show-ref --verify --quiet refs/heads/develop \
  || git show-ref --verify --quiet refs/remotes/origin/develop; } && BASE=develop
BASE=${BASE:-main}

git fetch origin
git checkout "$BASE"
git pull --ff-only origin "$BASE"
git status
```

> **Update the local integration branch** to include the merged workspace PR. `--ff-only` is intentional — if it fails because `$BASE` has unpushed local commits, push or rebase those first.
>
> After this checkout you are on the integration branch; later steps recover its name with `BASE=$(git branch --show-current)` (HEAD persists across shells, so this is safe even in a fresh command).
>
> **Check for lost OpenSpec changes:** If the dispatch commit included OpenSpec changes that are now missing (common if the dispatch commit wasn't pushed before launching), recover them from the original dispatch commit:
>
> ```bash
> # Find the original dispatch commit
> git log --oneline -n 20
> # Restore OpenSpec files from the dispatch commit
> git restore --source=<dispatch-commit-sha> -- openspec/
> ```
>
> **Verify the workspace is clean** before archiving. If `git status` shows unexpected modified files (code in `apps/`, `packages/`, etc.), investigate before proceeding. Only `openspec/` and `product-context.yaml` files should change during the wrap step.

### 2. Stop the dev server (from the workspace, if still running)

```bash
source .feature-workspaces/<name>/.dev-workflow/ports.env 2>/dev/null
lsof -ti :$SERVER_PORT | xargs kill 2>/dev/null
lsof -ti :$WEB_PORT | xargs kill 2>/dev/null
```

### 3. Run archive

```
/opsx:archive <change-name>
```

### 4. Commit and push the archive

```bash
BASE=$(git branch --show-current)   # integration branch, checked out in step 1
git add openspec/
git commit -m "chore: archive <change-name>"
git pull --ff-only origin "$BASE"
git push origin "$BASE"
```

### 5. Sync story status from workspace signals (Product-Cycle Mode Only)

> **Standalone mode:** If `product-context.yaml` doesn't exist, skip this step and proceed to step 6.

If `product-context.yaml` exists and this feature was a dispatched story, read the workspace signals and cross-check against the actual PR state:

```bash
# Read completion data from workspace signals
cat .feature-workspaces/<name>/.dev-workflow/signals/status.json

# Cross-check: verify actual PR state (signals can be stale)
gh pr view <pr-number> --json state,mergedAt
```

> **Signal validation:** Workspace agents don't always update their signal files after merge (they may show `in_review` even though the PR is already merged). Always cross-check the signal's `story_status` against the actual PR state via `gh pr view`. If the PR is merged but the signal says `in_review`, treat the story as `completed`.

From the signal file (corrected by PR state if needed), extract `story_status`, `pr_url`, `cost_usd`, `completed_at`, and `failure_log` (if present). Update the story in `product-context.yaml`:

```yaml
# Update the matching story:
status: completed # from signal story_status
completed_at: <timestamp> # from signal completed_at
pr_url: <url> # from signal pr_url
cost_usd: <cost> # from signal cost_usd
```

If `story_status` is `failed`, update with failure data instead:

```yaml
status: failed
failure_logs:
  - <structured failure_log from signal>
```

After updating the story, check if any `pending` stories should transition to `ready` (all dependencies now completed). Validate and commit all transitions atomically:

```bash
# Validate YAML before committing (see product-context references/yaml-guardrails.md)
npx js-yaml product-context.yaml > /dev/null && echo "YAML OK"

BASE=$(git branch --show-current)   # integration branch, checked out in step 1
git add product-context.yaml
git commit -m "chore: update story <id> status to completed"
git pull --ff-only origin "$BASE"
git push origin "$BASE"
```

> **Concurrency protocol:** This is the only place where story completion status enters `product-context.yaml`. Workspace agents write to signals; `/aep-wrap` (running on the integration branch) reads signals and writes to YAML.

### 5.5. Archive lessons learned

Before forgetting the workspace, check for lessons captured during the build:

```bash
LESSONS=".feature-workspaces/<name>/.dev-workflow/lessons.md"
if [ -f "$LESSONS" ] && [ "$(wc -l < "$LESSONS")" -gt 12 ]; then
  # File has content beyond the template header
  mkdir -p lessons-learned
  cp "$LESSONS" "lessons-learned/<change-name>.md"
  BASE=$(git branch --show-current)   # integration branch, checked out in step 1
  git add lessons-learned/<change-name>.md
  git commit -m "docs: archive lessons from <change-name>"
  git pull --ff-only origin "$BASE"
  git push origin "$BASE"
fi
```

> **Why before worktree removal:** Once `git worktree remove` runs, the worktree directory and its `.dev-workflow/lessons.md` are gone. This is the only chance to extract them.

If the lessons file contains only the template header (no Solutions, Errors, Missing, or Summary entries), skip — don't archive empty ceremony.

### 6. Tear down the worker + worktree (`executor.teardown()`)

Stop the workspace's worker **before** removing the worktree — otherwise an
OS-bound worker keeps running against a deleted directory, and these orphans
accumulate across an autopilot run. The stop step is per launch mode (recorded
as `backend`/`agent_id` in autopilot state, or evident from how you launched):

```bash
# Mode-specific worker stop (each is a no-op for the other modes):
#   native-bg-subagent → TaskStop(<bare-hex bg-subagent id>) (session-bound, no team)
#   claude-bg      → claude stop <agent_id>; claude rm <agent_id>
#   codex-subagent → close_agent(<agent_id>) if still running
#   codex-exec     → nothing to kill (the exec process exited with the build)
#   legacy         → tmux kill-session -t <name> 2>/dev/null || true

git worktree remove .feature-workspaces/<name> \
  || git worktree remove --force .feature-workspaces/<name>   # --force only if leftover files block removal
git worktree prune
git branch -d feat/<name>   # PR was merged → branch is reachable from the integration branch, safe to delete
```

If `git branch -d` warns the branch isn't fully merged (e.g., the PR was squash-merged so commit SHAs differ), force with `git branch -D feat/<name>` after confirming via `gh pr view <number> --json state` that the PR is `MERGED`.

---

## Guardrails

- **Never run `/opsx:archive` from a workspace** — it writes to `openspec/specs/` and causes conflicts when parallel workspaces are active. Archive always runs on the integration branch (`$BASE`).
- **Phase 13 clean-workspace check** — after `git fetch && git pull --ff-only`, run `git status` to verify no unexpected files are modified.
- **Verify OpenSpec changes exist before archiving** — if `openspec/changes/<name>/` is missing, the dispatch commit may have been lost. Recover from the original dispatch commit using `git restore --source=<sha>` before running archive.
- **Cross-check signals against PR state** — workspace signals can be stale (e.g., showing `in_review` after PR is merged). Always verify via `gh pr view` before updating `product-context.yaml`.
- **Read signals AND lessons before removing worktrees** — once `git worktree remove` runs, the worktree directory, signal files, and `lessons.md` are gone. Extract all needed data (status, lessons) first.

---

## Reflect and Advance (Product-Cycle Mode)

> **Standalone mode:** If `product-context.yaml` doesn't exist, skip the layer gate check. You can still run `/aep-reflect` if you want to classify observations.

After archiving, check the product context:

### Layer Gate Check

If `product-context.yaml` exists and this feature was a dispatched story:

```bash
# Check: was this the last story in the current layer?
# Read product-context.yaml and check if all stories in the active layer are completed
```

If all stories in the current layer are completed, run the **two-phase layer gate** — a gate is green
only when the layer is _covered_, not when one journey passes.

**Read `skills/e2e-test/policy.md` first** (if the project has the e2e-test skill): it declares the
**applicable tiers** (run only these — a `none`-target / CLI project has no Tier-2), the **dogfood
target** (`none` / `local` / `deployed:<url>`), and the **timing**. With `journey_timing: post-deploy`,
run the journey against the `deployed:<url>` target _here_ (after merge/deploy) — that is what flips
`scripted_passed → passed`.

1. **Tier-1 (machinery).** Run the project's scripted suite for this layer. If green, set
   `layer_gates[layer].status: scripted_passed` and record the test file under `evidence.scripted`.
2. **Tier-2/3 (product) + regression.** _Skip this step entirely if `dogfood_target == none`_ (CLI/library
   — Tier-2 N/A; prove criteria via Tier-1/Tier-3). Otherwise run the matching BDD journey/journeys in
   `skills/e2e-test/journeys/` (`layer: N`) via their `tool-selection.md`, plus any applicable API
   drivers, and **replay prior-layer journeys** — **seeding the policy's target** first (a `deployed:<url>`
   target needs `SERVER_URL=<url> bash skills/e2e-test/scripts/seed.sh`, not local). Record evidence —
   screenshots, API JSON, PASS/FAIL per Then, and the two coverage matrices — in `docs/layer-gates/<layer>.md`.
3. **Check coverage.** Confirm every layer acceptance criterion maps to ≥1 proving test
   (`coverage.criteria_covered == criteria_total`). Genuine gaps were already auto-closed during
   `/aep-build` Phase 6; a _deliberate_ deferral must carry a `WAIVER: <reason>` line. Never flip to
   `passed` while criteria are silently uncovered.
4. **Flip to `passed`** only when all applicable tiers are green AND coverage is complete-or-waived AND
   the regression replay passed; set `completed_at`. If only Tier-1 passed, leave it `scripted_passed`.
5. **Ask the human before advancing.** Surface the coverage summary (`criteria_covered / criteria_total`,
   per-tier status, any waivers) and **confirm with the user** that the next layer's design should begin.
   The gate flip is automatic-on-evidence; the _advance_ is a human decision — the next `/aep-dispatch`
   proceeds only after that confirmation.

### Feedback Loop

Consider running `/aep-reflect` to classify observations from this feature and update the product context:

```
/aep-reflect
```

This closes the feedback loop — bugs, refinements, and discoveries get routed back to the right phase.

---

## Next Step

Pick the next story from the dispatch queue:

```
/aep-dispatch
```

Or classify feedback from the feature you just shipped:

```
/aep-reflect
```
