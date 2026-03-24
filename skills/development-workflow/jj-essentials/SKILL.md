---
name: jj-essentials
description: Reference guide for jj (Jujutsu), the change-oriented VCS used for local development alongside Git. Use when the user asks about jj, jujutsu, "change vs commit", "how does jj work", or needs help with jj commands. Maps jj concepts to Git equivalents and explains the skeleton-first pattern for agent workflows.
---

# jj Essentials

jj (Jujutsu) handles local change management. Git handles remote collaboration. Together they give you the best of both: mutable, change-oriented local work with Git-compatible publishing to GitHub.

This skill is a reference — read it to understand concepts and commands. The agentic-development-workflow skill uses jj throughout its phases.

---

## The Mental Model Shift

**Git thinks in branches and commits:**

```
create branch → stage files → commit → push branch
```

Every step requires explicit state management. Agents waste tokens on `git add`, `git stash`, `git checkout`, resolving detached HEAD states.

**jj thinks in changes:**

```
edit files → describe the change → move on
```

The working copy *is* a change. No staging area. No branch management during development. Changes are mutable until published. Clean up history after generation, not during.

The key insight from the agent era: **the important ability is low-cost reorganization of existing output, not perfect first-pass commits.**

---

## Concept Mapping: Git to jj

| Git concept | jj equivalent | Why it matters for agents |
|---|---|---|
| `commit` | **change** | Mutable until published. No staging needed — edits are automatically part of the current change. |
| `branch` | **bookmark** | Only needed when publishing to Git remotes. Not used during local development. |
| staging area (`git add`) | **(none)** | All file modifications automatically belong to the current change. Agents never think about staging. |
| `git stash` | `jj new <base>` | Interruptions become real changes with full history, not hidden stash state that gets lost. |
| `git rebase -i` | `jj split` / `jj squash` | Post-hoc cleanup without fragile interactive mode. Split a mixed change into pieces. Squash a fix into its parent. |
| `git worktree` | `jj workspace` | Shared underlying store — no extra disk space, no branch naming conflicts. Parallel agents each get their own workspace. |
| `git checkout` | `jj edit` / `jj new` | `edit` = modify an existing change (auto-rebases dependents). `new` = create a fresh change from any point. |
| `git log` | `jj log` | Shows the change graph with immutable Change IDs that survive rebases. |
| `git reset --hard` | `jj undo` | Safely reverses any operation. `jj op log` shows full operation history. No reflog spelunking. |
| `git merge` | `jj new A B` | Create a multi-parent change. No merge ceremony or merge commits. |
| detached HEAD | **(impossible)** | The working copy always belongs to a change. No confusing detached state. |
| `git commit --amend` | `jj describe` / `jj squash` | `describe` edits the message. `squash` folds the current change into its parent. |
| `git cherry-pick` | `jj new <change>` | Fork a new change from any point in history. |
| `git reflog` | `jj op log` | Complete operation history with `jj op restore <id>` to go back to any point. |

---

## Why jj is Better for Agent Workflows

### 1. No staging = fewer tokens on VCS mechanics

Git requires: `git add -A` → `git commit -m "..."` → worry about what's staged vs unstaged.

jj: just edit files. They're part of the current change. Done.

### 2. Changes are mutable = generate rough, clean later

Agents naturally produce mixed diffs. With git, you need perfect commits or painful `rebase -i`. With jj, generate freely then use `split`/`squash` to clean up — a deliberate post-generation step.

### 3. Auto-rebase = edit any change in the stack

`jj edit <change>` lets you modify any historical change. All dependent changes automatically rebase. No manual rebase conflicts for the common case. Git can't do this without interactive rebase.

### 4. split/squash = first-class post-hoc cleanup

```bash
jj split    # One change has mixed concerns → split into multiple
jj squash   # A fix belongs in the parent change → fold it in
```

These are everyday operations, not emergency tools.

### 5. undo/op log = safe recovery

Agents make mistakes. `jj undo` reverses the last operation — any operation. `jj op log` shows the full history. `jj op restore <id>` goes back to any point. No git reflog archaeology.

### 6. Workspaces = parallel agents with shared store

`jj workspace add` creates a new working copy that shares the underlying store. No extra disk space. No branch naming conflicts. Multiple agents work in parallel, each in their own workspace.

### 7. Immutable protection = can't edit published changes

`jj edit` on a change that's already been pushed to a remote will warn and prevent accidental modification. Safety built into the model.

---

## The Skeleton-First Pattern

The most powerful integration with OpenSpec: create empty changes with descriptions **before** implementing.

### How it works

1. Read `tasks.md` from the OpenSpec change
2. Create one jj change per task:

```bash
jj new main
jj describe -m "refactor(auth): extract auth service"
jj new
jj describe -m "feat(auth): add token refresh flow"
jj new
jj describe -m "test(auth): add integration coverage for refresh"
jj new
jj describe -m "docs(auth): update API examples"
```

3. Implement each change in order:

```bash
jj edit <change-id-for-task-1>
# ... implement task 1 ...
jj diff                          # verify the change
# ... run targeted tests ...

jj edit <change-id-for-task-2>
# ... implement task 2 ...
# auto-rebase keeps task 2 on top of task 1's changes
```

4. Clean up after implementation:

```bash
jj split    # if any change mixed concerns
jj squash   # if a fix belongs in a parent change
```

### Why this matters

- **Spec and history are structurally aligned** — each OpenSpec task has its own change
- **Agent focuses on one task at a time** — `jj edit` scopes the work
- **Review is per-task** — each change is a reviewable unit matching a spec item
- **Auto-rebase handles dependencies** — edit task 1, task 2+ automatically update

---

## Essential Commands

### Daily toolkit

| Command | What it does |
|---|---|
| `jj log` | Show the change graph |
| `jj st` | Show working copy status (what files changed) |
| `jj diff` | Show diff of current change |
| `jj diff -r <change>` | Show diff of a specific change |
| `jj new` | Create a new empty change on top of current |
| `jj new <base>` | Create a new change from a specific point (e.g., `jj new main`) |
| `jj new A B` | Create a change with multiple parents (merge) |
| `jj describe -m "..."` | Set or update the current change's description |
| `jj edit <change>` | Switch to and modify an existing change |
| `jj commit -m "..."` | Describe the current change and create a new empty one on top |
| `jj split` | Split current change into multiple (interactive file/hunk selection) |
| `jj squash` | Fold current change into its parent |
| `jj rebase -s <source> -d <dest>` | Move a change (and descendants) to a new base |
| `jj undo` | Reverse the last operation |
| `jj op log` | Show operation history |

### Publishing to Git

```bash
# Fetch latest remote state
jj git fetch

# Rebase onto latest main before publishing
jj rebase -d main@origin

# Create a bookmark (maps to a Git branch) and push
jj bookmark create feat-<name> -r @-
jj git push --bookmark feat-<name>
```

After pushing, use `gh pr create` as normal — jj's bookmark becomes a Git branch.

### Updating after push

```bash
# After making fixes, re-push (bookmark auto-tracks)
jj git push --bookmark feat-<name>
```

### Quick publish (without named bookmark)

```bash
# Push the current change's parent — jj auto-creates a bookmark
jj git push --change @-
```

---

## Recovery

Recovery is a first-class operation, not an emergency procedure.

```bash
# Undo the last operation (any operation)
jj undo

# See full operation history
jj op log

# Restore to a specific point in history
jj op restore <operation-id>
```

Use when:
- A rebase went wrong
- The wrong change was edited
- A split/squash damaged the stack
- The agent over-applied a refactor

---

## Colocated Mode

jj runs alongside git in **colocated mode** (`jj git init --colocate`). Both `.jj/` and `.git/` exist in the same repo.

- **jj** manages local changes, history, workspaces
- **git** handles remote push/fetch, GitHub PRs, CI/CD
- All git tooling (GitHub Actions, `gh` CLI, IDE git panels) sees normal git state
- `jj git fetch` and `jj git push` bridge the two

Rule: **use `jj` commands for all local work. Use `jj git` subcommands for remote operations. Never use raw `git commit` or `git add` in a colocated repo.**
