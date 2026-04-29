#!/usr/bin/env bash
# Smoke test for the post-migration /launch worktree mechanics.
#
# Validates the parts of /launch and /wrap that don't need tmux/cmux/Claude:
#   - git worktree add succeeds and registers correctly
#   - the worktree's git state is sane (branch, HEAD, log)
#   - workspace-setup.sh's IS_WORKSPACE detection block correctly distinguishes
#     the worktree from the main repo (mirrors testing-guide/SKILL.md)
#   - .dev-workflow/signals/ paths resolve from both inside and outside the worktree
#   - autopilot's `git -C <worktree> diff --stat` liveness check works
#   - both launch-blocking regression scenarios (orphan branch, orphan registration)
#     are detectable and recoverable with the documented primitives
#
# Run from the repo root. Re-runnable. Cleans up after itself even on failure.

set -euo pipefail

NAME="smoke-test-$$"
WORKTREE_PATH=".feature-workspaces/$NAME"
BRANCH="feat/$NAME"

cleanup() {
  git worktree remove --force "$WORKTREE_PATH" 2>/dev/null || true
  rm -rf "$WORKTREE_PATH" 2>/dev/null || true
  git branch -D "$BRANCH" 2>/dev/null || true
  git worktree prune 2>/dev/null || true
}
trap cleanup EXIT

fail() {
  echo "FAIL: $1" >&2
  exit 1
}

echo "=== 1. Pre-flight: clean state ==="
git worktree list
[ ! -d "$WORKTREE_PATH" ] || fail "$WORKTREE_PATH already exists"

echo "=== 2. git worktree add ==="
mkdir -p .feature-workspaces
git worktree add -b "$BRANCH" "$WORKTREE_PATH" main
[ -d "$WORKTREE_PATH" ] || fail "worktree dir missing after add"
git worktree list | grep -q "$WORKTREE_PATH" || fail "worktree not registered in git worktree list"

echo "=== 3. Worktree git state ==="
(
  cd "$WORKTREE_PATH"
  current_branch="$(git branch --show-current)"
  [ "$current_branch" = "$BRANCH" ] || fail "wrong branch in worktree: $current_branch"
  git rev-parse --short HEAD >/dev/null || fail "git rev-parse --short HEAD failed"
  git log --oneline -1 >/dev/null || fail "git log failed"
  echo "  branch=$current_branch HEAD=$(git rev-parse --short HEAD)"
)

echo "=== 4. workspace-setup.sh IS_WORKSPACE detection ==="
# Mirror the IS_WORKSPACE detection block from testing-guide/SKILL.md
(
  cd "$WORKTREE_PATH"
  REPO_ROOT="$(git rev-parse --show-toplevel)"
  MAIN_REPO="$(git worktree list --porcelain | head -1 | sed 's/^worktree //')"
  IS_WORKSPACE=false
  [ "$REPO_ROOT" != "$MAIN_REPO" ] && IS_WORKSPACE=true
  [ "$IS_WORKSPACE" = "true" ] || fail "IS_WORKSPACE detection wrong: REPO_ROOT=$REPO_ROOT MAIN_REPO=$MAIN_REPO"
  echo "  IS_WORKSPACE=true (REPO_ROOT=$REPO_ROOT, MAIN_REPO=$MAIN_REPO)"
)

echo "=== 5. .dev-workflow/signals/ path resolution ==="
mkdir -p "$WORKTREE_PATH/.dev-workflow/signals"
echo '{"phase":0,"phase_name":"smoke-test"}' > "$WORKTREE_PATH/.dev-workflow/signals/status.json"
[ -f "$WORKTREE_PATH/.dev-workflow/signals/status.json" ] || fail "signal file path"
# Autopilot's liveness check from main session:
git -C "$WORKTREE_PATH" diff --stat >/dev/null || fail "git -C <worktree> diff --stat (autopilot liveness)"
echo "  signal write + git -C diff --stat: OK"

echo "=== 6. Regression: orphan branch detection logic ==="
# Set up the orphan-branch scenario, then verify the new launch Step 4 logic
git worktree remove --force "$WORKTREE_PATH"
[ ! -d "$WORKTREE_PATH" ] || fail "worktree not removed"
# Branch still exists; ahead count should be 0 since we never committed
ahead="$(git rev-list --count "main..$BRANCH" 2>/dev/null || echo 0)"
[ "$ahead" = "0" ] || fail "ahead-count logic: expected 0, got $ahead"
echo "  orphan branch ahead-count: $ahead (safe to delete)"
git branch -D "$BRANCH"

echo "=== 7. Regression: orphan worktree registration prune ==="
# Re-create the worktree, then simulate manual deletion (rm -rf without git worktree remove)
git worktree add -b "$BRANCH" "$WORKTREE_PATH" main >/dev/null
rm -rf "$WORKTREE_PATH"
[ -d ".git/worktrees/$NAME" ] || fail "registration not present after rm -rf"
git worktree prune
[ ! -d ".git/worktrees/$NAME" ] || fail "git worktree prune did not clean up registration"
echo "  orphan registration pruned: OK"
git branch -D "$BRANCH"

echo "=== 8. Final cleanup verification ==="
if git worktree list | grep -q "$NAME"; then
  fail "leftover worktree in git worktree list"
fi
if git branch --list "$BRANCH" | grep -q .; then
  fail "leftover branch $BRANCH"
fi
echo "  no leftover worktree or branch: OK"

echo ""
echo "PASS: smoke-test-worktree.sh"
