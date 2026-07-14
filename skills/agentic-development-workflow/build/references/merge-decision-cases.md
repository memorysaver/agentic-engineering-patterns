# Merge & Worktree Decision Cases (canonical)

Canonical home for the Phase 12 merge decision: the `mergeStateStatus` state
handling and the 6-item stop-condition taxonomy both live here. `/aep-build`
Phase 12 keeps only the happy path (compute `mergeStateStatus`; `CLEAN` →
proceed) and loads this file for every other state. This is also the regression
fixture for the "PR-ready stop" + "feat branch in the main checkout" failure mode
(autopilot, Codex backend). Each case states the inputs, the **required**
behavior, and the **bug** behavior it forbids.

**Canonical ownership:** the 6-item stop-condition list and the per-state
handling below are the source of truth. The orchestrator merge **nudges**
(`autopilot/SKILL.md`, `tick-protocol.md`) must enumerate all 6 (the two safety
stops — human-approval gate, policy pause — are the ones most often dropped).
The launch `mode` marker's semantics (`/aep-launch` `references/signals-spec.md`)
carry the autopilot-vs-interactive rule, not the full stop list, but must not
contradict it. When you change Phase 0 / Phase 12 / the launch
`mode` marker, re-verify every case below still holds and that no file
contradicts this list — half-applying it is this repo's #1 bug class.

---

## `mergeStateStatus` — per-state handling (Phase 12 step 2)

Read GitHub's own readiness rather than counting raw checks:
`gh pr view <number> --json mergeStateStatus,mergeable --jq '{state: .mergeStateStatus, mergeable}'`.
`mergeStateStatus` already accounts for _required_ checks and _required_ reviews;
raw check counts can't tell required from optional, nor "none configured" from
"not yet reported".

- `CLEAN` → mergeable now (required checks satisfied or none; no required review
  missing) → **proceed**. A CLEAN PR with **zero** required checks is mergeable
  now — absence of checks is not a reason to wait.
- `UNKNOWN` → GitHub is still computing mergeability (normal right after the
  step-1 push) → **wait briefly and re-read**; do not park, do not merge yet.
- `UNSTABLE` → mergeable per branch protection, but a non-required check is
  pending/failing. Do **not** blanket-merge: if any check is **failing**,
  **stop** — don't land red code, even if the repo configured no _required_
  checks (`gh pr checks <number>` to see); if checks are only **pending**, wait
  and re-read.
- `BLOCKED` → a **required** review/check is missing/pending/failing, **or** a
  branch-protection rule blocks (conversation-resolution, signed commits, linear
  history, admin-only) → **stop**: maps to conditions 1–2/4, or surface the
  protection rule as a human-gate (condition 5) — never force past it.
- `DIRTY` (conflict) → **stop** (condition 3). `BEHIND` → rebase (step 1) and
  re-read.

---

## The only legitimate stop-at-ready conditions (autopilot)

"PR ready" is a _precondition to merge_, not a place to stop. Stop short of merge
**only** when one of these holds; otherwise you MUST merge:

1. a **required** review is missing,
2. **required** checks are pending or failing,
3. a **merge conflict** / dirty branch (rebase failed),
4. an **unresolved review thread** blocks the merge,
5. an explicit **human-approval gate** applies (raise via Human-Gate Protocol),
6. project **policy** (`full_auto`/strategic) says pause.

None of the above ⇒ merge. Reporting `mergeStateStatus=CLEAN` and stopping is a
bug, not a safe state. After merging, set `status.json`
`story_status: "completed"` — the worker's job ends there; it does **not** run
`/aep-wrap` (wrap runs `/opsx:archive` on the integration branch; running it from
the worktree corrupts the concurrency protocol). The story is "done" only when
**merged + wrapped**, but the worker only merges.

---

## Case A — clean PR, no required checks, autopilot ⇒ worker MERGES (orchestrator wraps)

**Inputs:** worker launched via `/aep-launch` (`.dev-workflow/signals/mode` = `autopilot`);
PR open, non-draft; `mergeStateStatus=CLEAN`; **zero** required checks configured;
no unresolved review threads; eval PASSED.

**Required:** Phase 12 detects autopilot mode (reads the `mode` marker), treats a
CLEAN PR with no required checks as mergeable (not "wait"), runs
`gh pr merge --squash --delete-branch`, sets `status.json` `story_status=completed`
— and **stops there**. Wrap (`/aep-wrap`) runs on the integration branch (the
orchestrator's next tick), **not** from the worktree.

**Forbidden (the bug):** stopping at "PR ready" / asking for confirmation / reporting
`mergeStateStatus=CLEAN` as a terminal state; **or** the worker running `/aep-wrap`
itself. "PR ready" is not done — **merged + wrapped** is done, but the worker only merges.

---

## Case B — worker not in its worktree ⇒ guard ENTERS the launch-made worktree, else escalates

**Inputs:** `/aep-build` starts outside `.feature-workspaces/<name>` (e.g. a Codex
`codex-subagent` that didn't honor the cd contract — `spawn_agent` has no cwd
parameter — so it is in the main checkout or a sibling worktree).

**Required:** the Phase 0 guard trips (`show-toplevel` ≠ `.feature-workspaces/<name>`
or branch ≠ `feat/<name>`) and **enters the worktree `/aep-launch` already created**
(`cd "$ROOT/.feature-workspaces/<name>"`). Worktree creation belongs to `/aep-launch`;
if no worktree for `<name>` exists, the guard **STOPS and escalates** (Human-Gate
Protocol) — it does **not** create a branch/worktree or touch the main checkout.

**Forbidden (the bug):** building/branching in the main checkout; mutating another
worktree's branch (e.g. `git switch` in main, which can strand uncommitted work);
or building in a _sibling_ feature worktree (the guard is `<name>`-specific, so this
trips). Autopilot mode is decided by the `mode` marker only — there is **no** cwd
fallback.

---

## Case C — interactive build ⇒ confirm before merge (unchanged)

**Inputs:** a human runs `/aep-build` directly in their own session; no `mode`
marker; cwd is not a launched worktree.

**Required:** interactive mode — ask for confirmation before merging. (This is the
_only_ case where "ask before merge" is correct.)
