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

### 5. Remove the workspace

```bash
jj workspace forget <name>
```

---

## Guardrails

- **Never run `/opsx:archive` from a workspace** — it writes to `openspec/specs/` and causes conflicts when parallel workspaces are active. Archive always runs on `main`.
- **Phase 13 clean-workspace check** — after `jj git fetch`, run `jj st` to verify no unexpected files are modified.
- **Always use `jj`** for local changes — never use raw `git commit` or `git add` in a colocated repo.

---

## Reflect on What Was Learned

After archiving, consider running `/reflect` to classify observations from this feature and update the product context:

```
/reflect
```

This closes the feedback loop — bugs, refinements, and discoveries get routed back to the right phase of the product context layer.

**Layer gate check:** If this was the last story in a layer (check `product-context/story-graph.md`), run the layer gate integration test before starting the next layer.

---

## Next Step

Start the next feature with:

```
/design
```
