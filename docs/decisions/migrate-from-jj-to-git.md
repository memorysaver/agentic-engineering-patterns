# Decision: Migrate from jj to git + git worktree

**Date:** 2026-04-29
**Status:** Accepted (single-shot migration on branch `migration/git-only`)
**Supersedes:** Section 6 of `docs/aep-v2-improvement-guideline.md` (the "VCS Abstraction" dual-backend proposal)

## Context

AEP's execution plane previously ran on Jujutsu (jj) in colocated mode (`.git/` + `.jj/` in the same repo). jj was chosen for genuine technical advantages:

- Mutable changes until published (no `git rebase -i` ceremony)
- Auto-rebase when editing earlier changes in a stack
- Conflict-as-data (commits with unresolved conflicts are first-class)
- `jj workspace add` for parallel agents sharing one object store
- `jj op log` / `jj undo` for safe operation-level recovery

These properties enabled the **skeleton-first pattern** in `/build` Phase 0: create empty changes for every `tasks.md` row, then implement them in any order with auto-rebase keeping dependents consistent. Elegant in theory.

## What Drove the Migration

In practice, jj caused enough friction with Claude Code and Codex that the friction outweighed the wins. Over many workspace sessions we observed:

1. **Agents reach for git reflexively.** LLMs have orders of magnitude more git in their training set. On a colocated repo, agents would run `git status` and see confusing detached-HEAD output, or interleave `git commit` with jj's snapshotting and create bookmark conflicts.

2. **The colocated rulebook was a continuous tax.** Skills repeated "use jj for local, jj git for remote, never raw git commit" multiple times each, in `/onboard`, `/wrap`, `/jj-ref`, and several reference docs. Agents still violated it. The rule consumed prompt tokens that should have been spent on the task.

3. **No daemon, no async snapshot.** jj only snapshots the working copy when a jj command runs. Agents that crashed before issuing a jj command lost work between snapshots. We had to add Claude Code hooks to force `jj st` at session boundaries — defense-in-depth that wouldn't be needed under git's explicit-commit model.

4. **Universal tooling assumes git.** `gh`, IDE git panes, every CI provider, husky hooks, the GitHub Actions ecosystem — all expect a plain git repo. jj was a translation layer with edge cases.

5. **Existing third-party Claude Code skills exist _because_ the problem is real.** Multiple "jj for Claude Code" skills (danverbraganza, mtaran, RealAdarsh, HotThoughts) exist precisely because Claude Code defaults to git and "getting Claude to work with jj proved tricky." That's a market signal we shouldn't ignore.

The original v2 improvement guideline (Section 6) proposed a softer fix: a dual-backend abstraction with `jj-backend.md` + `git-backend.md` so projects could pick. We rejected this because it doubles the documentation surface for a transition we believe will land on git anyway, and because every additional code path agents must reason about increases the chance of confusion.

## Decision

**Drop jj entirely. Adopt pure git + `git worktree`.** Single-shot migration on branch `migration/git-only`, single PR, no dual-mode period.

### What Changes

| Concern               | Before (jj)                                                             | After (git)                                                             |
| --------------------- | ----------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| Workspace creation    | `jj workspace add .feature-workspaces/<n>`                              | `git worktree add -b feat/<n> .feature-workspaces/<n> main`             |
| Implementation        | `jj new && jj describe -m "..."` skeleton, then `jj edit <id>` per task | Read `tasks.md`, implement linearly, one `git commit` per task          |
| Pre-publish sync      | `jj git fetch && jj rebase -d main@origin`                              | `git fetch origin && git rebase origin/main`                            |
| Publishing            | `jj bookmark create … && jj git push --bookmark …`                      | `git push -u origin feat/<n>`                                           |
| Workspace teardown    | `jj workspace forget <n>`                                               | `git worktree remove .feature-workspaces/<n> && git branch -d feat/<n>` |
| Control-plane commits | `jj describe → jj new → jj git push --change @-`                        | `git pull --ff-only && git add … && git commit && git push origin main` |
| Verification field    | `change_id` (jj 8-char change ID)                                       | `commit_sha` (git 8-char short SHA)                                     |

The skeleton-first pattern is **dropped** in favor of linear commits. `tasks.md` already serves as the skeleton; the resulting feature branch ends up with N commits matching N tasks 1:1, which makes per-task PR review just as natural as per-change jj review was — without auto-rebase machinery.

### Path Rationale

- **`/jj-ref` skill removed**, replaced by `/git-ref` covering AEP's worktree conventions (path, branch naming, the one-commit-per-task pattern, recovery procedures, PR conventions). Far thinner than `/jj-ref` because git is universal — only AEP-specific conventions need documenting.
- **`.feature-workspaces/<name>` retained** as the worktree path. Familiar from the jj era, minimizes ripple changes to autopilot signal-file paths and lessons archival.
- **Squash-merge at PR merge** preserves clean main history while letting the PR review trail show the per-task commits.
- **Concurrency hooks rewritten** to match `git commit|add` of `product-context.yaml` from workspace sessions (the previous jj branch of the regex is removed). Same protection, simpler match.

## What We Lost (and how we cope)

| Lost jj feature                           | Replacement                                                                                                                                                       |
| ----------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Working-copy auto-snapshot                | Manual `git commit` per task in Phase 4 (already enforced upfront by the linear pattern)                                                                          |
| `jj edit` + auto-rebase mid-stack         | Implement linearly; if cleanup is genuinely needed, agents can use `git rebase -i` (rare under linear pattern)                                                    |
| `jj split` / `jj squash` post-hoc cleanup | Squash-merge at PR-merge keeps main clean. Per-commit hygiene is enforced upfront by conventional-commit-per-task.                                                |
| Conflict-as-data                          | None — agents resolve `<<<<<<<` markers like humans. Counterweight: linear-on-feature-branch makes conflicts rare (only on `git rebase origin/main` before push). |
| `jj op log` / `jj undo`                   | `git reflog` + `git reset` / `git restore --source`, documented in `/git-ref`                                                                                     |
| Zero-disk workspaces                      | Worktrees share `.git/objects`. Only the working tree is duplicated — small for typical AEP repos.                                                                |

## Out of Scope for This Migration

- Removing `.jj/` from any user's local repo (we ignore it; users can `rm -rf .jj/` themselves if desired).
- Rewriting any in-flight jj-created feature branches. Those merge or are abandoned naturally.
- A dual-mode `jj-backend.md` / `git-backend.md` abstraction (the original v2 Section 6 proposal — explicitly rejected here).
- Changes to autopilot's main-session boundary rules. Those are governed by separate memory/feedback and are unaffected by this migration.

## Recovery Plan If We Reverse

If a future lesson tells us git was the wrong call (e.g., agents start losing work in ways jj would have caught), the reversal path is:

1. Re-introduce `/jj-ref` skill from git history.
2. Re-add `.jj/` initialization to `/onboard` Phase 2.5.
3. Replace `git worktree add -b feat/<n>` with `jj workspace add` in `/launch`.
4. Replace linear-commit Phase 4 with skeleton-first jj change stacks.
5. Replace the control-plane `git commit` pattern with `jj describe + jj new + jj git push --change @-`.

This list exists so we don't have to reconstruct the change set from scratch if the migration turns out to be wrong. As of this commit, we believe it is right and we will not reopen it absent a concrete failure case.

## References

- [skills/agentic-development-workflow/git-ref/SKILL.md](../../skills/agentic-development-workflow/git-ref/SKILL.md) — the new AEP git + worktree reference
- [docs/aep-v2-improvement-guideline.md § 6](../aep-v2-improvement-guideline.md) — the closed-out proposal
- [Avoid Losing Work with Jujutsu (jj) for AI Coding Agents — Anthony Panozzo, 2025-11](https://www.panozzaj.com/blog/2025/11/22/avoid-losing-work-with-jujutsu-jj-for-ai-coding-agents/) — exemplifies the async-snapshot footgun
- [Parallel Agentic Development With Git Worktrees — MindStudio](https://www.mindstudio.ai/blog/parallel-agentic-development-git-worktrees) — established patterns for the model we're adopting
