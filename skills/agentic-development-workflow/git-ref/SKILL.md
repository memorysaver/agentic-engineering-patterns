---
name: git-ref
description: AEP-specific reference for git + git worktree workflows. Use when the user asks "how do I create a worktree?", "what's the AEP branch convention?", "how do I clean up a worktree?", "how does AEP use git?", "remind me of git commands for parallel agents", or needs to recover from a worktree mishap. Documents worktree lifecycle, branch naming, the one-commit-per-task pattern, recovery procedures, and PR conventions used by `/launch`, `/build`, and `/wrap`.
---

# Git + Worktree Reference (AEP)

`/launch`, `/build`, and `/wrap` operate on a plain git repository plus `git worktree` for parallel agent isolation. There is no separate VCS, no colocated mode, no special wrapper — when these skills say "commit", they mean `git commit`. This skill documents the AEP-specific conventions on top of standard git.

If you've used git for ten minutes, you already know 90% of this. The remaining 10% is the conventions AEP layers on top.

---

## Why Git, Not jj

AEP previously used Jujutsu (jj) in colocated mode for change-mutability and zero-disk workspaces. We migrated to pure git because:

- **Agent training data.** Every LLM has orders of magnitude more git in its training set. Agents reach for `git status` reflexively, and on a colocated jj+git repo that returned confusing detached-HEAD output.
- **No colocated rulebook.** The "use jj for local, jj git for remote, never raw git commit" rule produced repeated agent violations and wasted prompt tokens reinforcing it.
- **Universal tooling.** `gh`, IDE git panes, every CI provider, husky hooks — all assume git. jj needed adapter skills.
- **No async-snapshot footgun.** jj's working-copy auto-snapshot has no daemon; agents could lose work between commands. Manual `git commit` per task is now enforced upfront.

What we lost: jj's auto-rebase, conflict-as-data, and `op log` recovery. The replacements (linear commits, eager conflict resolution, `git reflog`) are documented below.

See [docs/decisions/migrate-from-jj-to-git.md](../../../docs/decisions/migrate-from-jj-to-git.md) for the full rationale.

---

## Worktree Lifecycle

### Create

```bash
mkdir -p .feature-workspaces
git worktree add -b feat/<name> .feature-workspaces/<name> main
```

- Path is **always** under `.feature-workspaces/<name>` (kept gitignored).
- Branch is **always** `feat/<name>` — corresponding to the OpenSpec change name or story id.
- Base is **always** `main` — never another feature branch.

The worktree shares `.git/objects` with the main repo, so creating it is fast and history is not duplicated. Only the working tree files are duplicated on disk.

### Inspect

```bash
git worktree list                            # all worktrees, branches, paths
git -C .feature-workspaces/<name> status     # status inside a specific worktree
git -C .feature-workspaces/<name> log --oneline main..HEAD   # commits unique to the feature branch
```

### Remove (`/wrap` step 6)

```bash
git worktree remove .feature-workspaces/<name>
git branch -d feat/<name>
```

If `git branch -d` warns the branch isn't fully merged (likely because the PR was squash-merged so commit SHAs differ), force with `git branch -D feat/<name>` _after_ confirming via `gh pr view <number> --json state` that the PR is `MERGED`.

### Recover from a corrupt worktree directory

If `.feature-workspaces/<name>/` was deleted manually:

```bash
git worktree prune                           # forget the orphaned registration
git branch -D feat/<name>                    # delete the orphan branch (if needed)
```

If `.git/worktrees/<name>/` still references a missing path (e.g. you moved the parent directory):

```bash
git worktree repair .feature-workspaces/<name>
```

---

## Branch Naming

| Branch            | Pattern              | Created by                |
| ----------------- | -------------------- | ------------------------- |
| Feature work      | `feat/<short-name>`  | `/launch` (one per story) |
| Migration / chore | `chore/<short-name>` | manually, when applicable |
| Hotfix off main   | `fix/<short-name>`   | manually                  |
| Migration project | `migration/<topic>`  | manually                  |

**Rules:**

- Use kebab-case after the slash. No spaces, no underscores.
- Keep names short (≤ 30 chars) — they appear in tab labels, signal files, and PR titles.
- Don't reuse a branch name after deletion until the corresponding worktree is fully removed (`git worktree prune` cleans up dangling refs).

---

## The One-Commit-per-Task Pattern (Phase 4 of `/build`)

This is the largest AEP-specific convention.

### What

`tasks.md` lists N tasks. The feature branch ends up with N commits — one per task — in the same order. Conventional-commit format. Workspace agents implement linearly, committing after each task, never bundling and never splitting:

```bash
# Implement task 1
# ... edit files ...
git add -A
git commit -m "feat(auth): extract auth service"

# Implement task 2
# ... edit files ...
git add -A
git commit -m "feat(auth): add token refresh flow"

# ... etc.
```

After each commit, record the short SHA in `.dev-workflow/feature-verification.json` against the matching task entry's `commit_sha` field.

### Why this shape

- The PR's commit list reads like a table of contents matching `tasks.md`.
- Each commit is a self-contained unit that the evaluator (in Phase 5) and reviewers (in Phase 11) can `git show <sha>` independently.
- We squash-merge at PR-merge time, so per-commit hygiene only matters for review readability — main history stays clean automatically.
- It removes any need to rewrite history mid-stream, which is where agents most often go off the rails.

### What to do when something goes wrong

| Situation                                                 | Action                                                                                                                                                            |
| --------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Forgot a file in the just-committed task                  | `git add <file> && git commit --amend --no-edit` (only safe **before** the next task's commit)                                                                    |
| Realized the previous task is broken, several commits ago | Add a new commit: `fix(<scope>): correct <issue from task N>` — do not rebase                                                                                     |
| Review feedback or eval-loop FAIL                         | Add a follow-up commit: `fix(<scope>): address review on <topic>`                                                                                                 |
| Need to update against new origin/main                    | `git fetch origin && git rebase origin/main && git push --force-with-lease origin feat/<name>`                                                                    |
| Conflicts during rebase                                   | Resolve in working tree, `git add <files> && git rebase --continue`. If hopelessly tangled, `git rebase --abort` and surface to the orchestrator via signal file. |

**Never** use `git push --force` (without `--force-with-lease`). The lease variant fails safely when someone else has pushed since your last fetch.

---

## Publishing & PR Conventions

### Push the feature branch

```bash
# First push (after Phase 9 cleanup):
git push -u origin feat/<name>

# Subsequent pushes (review fixes, rebases):
git push origin feat/<name>
# or after a rebase:
git push --force-with-lease origin feat/<name>
```

### Open the PR — always set base explicitly

```bash
gh pr create --base main --title "<title>" --body "<body>"
```

The `--base main` flag is **mandatory**. Without it, `gh` infers the base from local branch state and can target the wrong branch (especially when a dispatch commit looked like a recent base). PRs targeting the wrong base merge into a non-main branch, and the code never lands on `main` even after a successful merge.

### Merge the PR

```bash
gh pr merge <number> --squash --delete-branch
```

We always squash-merge. The feature branch's per-task commits collapse into one commit on `main`, which keeps `main`'s log readable while preserving the per-task review trail in the PR's "Commits" tab.

---

## Control-Plane Commits (on `main`)

`/dispatch`, `/envision`, `/map`, `/calibrate`, `/validate`, `/reflect`, and the `/wrap` archive step all commit directly to `main`. The pattern is identical:

```bash
git pull --ff-only origin main          # fail-fast if main has diverged
git add <specific-files>                # never -A on main; be explicit
git commit -m "<conventional message>"
git push origin main
```

The `--ff-only` is intentional — it refuses a non-fast-forward pull, which means concurrent pushes get caught instead of silently merged. If the pull fails because someone else pushed first, fetch, rebase your work, and try again.

---

## Recovery

### Lost a commit (committed to wrong branch, or amend'd away)

```bash
git reflog                              # find the orphaned commit's SHA
git cherry-pick <sha>                   # bring it onto the current branch
# or
git reset --hard <sha>                  # if you want to rewind to that point
```

### Lost an entire branch (deleted with -D)

```bash
git reflog                              # the deleted branch's tip is in here
git branch <name> <sha>                 # recreate the branch at that tip
```

### Lost work in a worktree directory you removed

If you ran `rm -rf .feature-workspaces/<name>` without committing first, the work is gone — `git worktree` only tracks committed history. `.dev-workflow/` files (signals, lessons, progress) are also gone since they're gitignored. Use `git worktree remove` instead of `rm -rf` next time; it refuses if the worktree has uncommitted changes.

### OpenSpec files missing after rebase

If the dispatch commit's OpenSpec files disappeared during a rebase or merge:

```bash
git log --oneline -n 30                 # find the dispatch commit SHA
git restore --source=<sha> -- openspec/
```

This restores `openspec/` to the state of the dispatch commit without touching anything else.

---

## Disk Budget

Each worktree adds approximately one full working-tree copy on disk. `.git/objects` is shared across all worktrees, so commit history isn't duplicated. For a typical AEP repo (~100 MB working tree, multi-GB `.git/objects` after years of history), three concurrent agents adds ~300 MB on top of one shared `.git/`.

Monitor with:

```bash
du -sh .feature-workspaces/             # working-tree footprint
du -sh .git/objects                     # shared history
```

If disk pressure hits before agents finish, prefer pausing new `/launch` invocations over forcibly removing in-flight worktrees.

---

## Cheat Sheet

| You want to…                 | Run                                                                     |
| ---------------------------- | ----------------------------------------------------------------------- |
| Create a feature worktree    | `git worktree add -b feat/<n> .feature-workspaces/<n> main`             |
| List all worktrees           | `git worktree list`                                                     |
| See feature-branch commits   | `git log --oneline main..HEAD`                                          |
| Sync against latest main     | `git fetch origin && git rebase origin/main`                            |
| Push after rebase            | `git push --force-with-lease origin feat/<n>`                           |
| Open PR                      | `gh pr create --base main`                                              |
| Merge PR                     | `gh pr merge <#> --squash --delete-branch`                              |
| Remove worktree post-merge   | `git worktree remove .feature-workspaces/<n> && git branch -d feat/<n>` |
| Recover deleted commit       | `git reflog` then `git cherry-pick <sha>`                               |
| Restore lost openspec/ files | `git restore --source=<sha> -- openspec/`                               |
