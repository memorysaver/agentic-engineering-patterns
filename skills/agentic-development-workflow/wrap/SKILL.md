---
name: wrap
description: Post-merge archive and cleanup. Use after a PR has been merged, or when the user says "wrap up", "archive", "cleanup after merge", "post-merge". Runs OpenSpec archive, commits the archive, and removes the workspace. The final step in the feature lifecycle.
---

# Wrap

Post-merge archive and workspace cleanup. Run this on `main` after the PR merges to archive the OpenSpec change and clean up the workspace.

**Where this fits:**

```
/onboard → /scaffold → [ /design → /launch → /build → /wrap ]
                                                        ▲ you are here
```

**Session:** Main session, post-merge
**Input:** Merged PR notification
**Output:** Archived OpenSpec change, cleaned up workspace

---

## Phase 13: Archive & Cleanup on Main

> Run on `main` after the PR merges. Do not run from the workspace.

### 1. Fetch merged state and verify clean workspace

```bash
jj git fetch
jj st
```

> **Verify the workspace is clean** before archiving. If `jj st` shows unexpected modified files (code in `apps/`, `packages/`, etc.), investigate before proceeding. Only `openspec/` files should change during the archive step.

### 2. Stop the dev server (from the workspace, if still running)

```bash
source .claude/workspaces/<name>/.dev-workflow/ports.env 2>/dev/null
lsof -ti :$SERVER_PORT | xargs kill 2>/dev/null
lsof -ti :$WEB_PORT | xargs kill 2>/dev/null
```

### 3. Run archive

```
/opsx:archive <change-name>
```

### 4. Commit and push the archive

```bash
jj describe -m "chore: archive <change-name>"
jj new
jj git push --change @-
```

### 5. Sync story status from workspace signals

If `product-context.yaml` exists and this feature was a dispatched story, read the workspace signals and update the YAML:

```bash
# Read completion data from workspace signals
cat .claude/workspaces/<name>/.dev-workflow/signals/status.json
```

From the signal file, extract `story_status`, `pr_url`, `cost_usd`, `completed_at`, and `failure_log` (if present). Update the story in `product-context.yaml`:

```yaml
# Update the matching story:
status: completed          # from signal story_status
completed_at: <timestamp>  # from signal completed_at
pr_url: <url>              # from signal pr_url
cost_usd: <cost>           # from signal cost_usd
```

If `story_status` is `failed`, update with failure data instead:
```yaml
status: failed
failure_logs:
  - <structured failure_log from signal>
```

After updating the story, check if any `pending` stories should transition to `ready` (all dependencies now completed). Commit all transitions atomically:

```bash
jj describe -m "chore: update story <id> status to completed"
jj new
jj git push --change @-
```

> **Concurrency protocol:** This is the only place where story completion status enters `product-context.yaml`. Workspace agents write to signals; `/wrap` (running on main) reads signals and writes to YAML.

### 6. Remove the workspace

```bash
jj workspace forget <name>
```

---

## Guardrails

- **Never run `/opsx:archive` from a workspace** — it writes to `openspec/specs/` and causes conflicts when parallel workspaces are active. Archive always runs on `main`.
- **Phase 13 clean-workspace check** — after `jj git fetch`, run `jj st` to verify no unexpected files are modified.
- **Always use `jj`** for local changes — never use raw `git commit` or `git add` in a colocated repo.

---

## Reflect and Advance

After archiving, check the product context:

### Layer Gate Check

If `product-context.yaml` exists and this feature was a dispatched story:

```bash
# Check: was this the last story in the current layer?
# Read product-context.yaml and check if all stories in the active layer are completed
```

If all stories in the current layer are completed:
- Suggest running the **layer gate integration test** (defined in `layer_gates` section of the YAML)
- If the gate passes, update `layer_gates[layer].status: passed` and `completed_at`
- The next `/dispatch` will advance to the next layer

### Feedback Loop

Consider running `/reflect` to classify observations from this feature and update the product context:

```
/reflect
```

This closes the feedback loop — bugs, refinements, and discoveries get routed back to the right phase.

---

## Next Step

Pick the next story from the dispatch queue:

```
/dispatch
```

Or classify feedback from the feature you just shipped:

```
/reflect
```
