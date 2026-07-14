# Orphan Worktree / Branch Recovery

When a previous `/aep-launch` died mid-flight, git worktree (unlike jj's
`jj workspace forget`) does **not** auto-clean. Two silent, confusing-on-first-encounter
failure modes can block re-launch. Run these idempotent checks before creating the
worktree (Step 2). Resolve `$BASE` per /aep-git-ref "Resolving `$BASE`" first.

```bash
# Check 1: orphan branch with no unmerged work → safe to delete
if git show-ref --verify --quiet refs/heads/feat/<name>; then
  ahead=$(git rev-list --count "$BASE"..feat/<name> 2>/dev/null || echo 0)
  if [ "$ahead" = "0" ]; then
    echo "Removing orphan branch feat/<name> (no commits ahead of $BASE)"
    git branch -D feat/<name>
  else
    echo "ABORT: feat/<name> has $ahead unmerged commit(s). Investigate before re-launching."
    echo "  - If the work is salvageable: git checkout feat/<name> && finish manually"
    echo "  - If the work is abandoned:   git branch -D feat/<name> && retry /aep-launch"
    exit 1
  fi
fi

# Check 2: orphan worktree registration → prune
if [ -d ".git/worktrees/<name>" ] && [ ! -d ".feature-workspaces/<name>" ]; then
  echo "Pruning orphan worktree registration for <name>"
  git worktree prune
fi
```

The branch deletion is gated on `ahead == 0` so live workspaces are never affected —
if the orphan branch has unmerged commits, abort and let the user investigate. The
worktree prune is gated on the working directory being missing, so it only fires after
a manual `rm -rf` of the worktree dir.

## Orphan with a live worktree

If `.feature-workspaces/<name>` exists with committed/in-progress work but no live agent
(state says active, agent list says gone), do **not** delete anything — follow the
**orphan re-adoption** protocol in /aep-executor `backends.md`: re-spawn a worker into
the existing worktree with a recovery bootstrap.
