# Merge & Worktree Decision Cases (regression fixture)

Canonical worked cases that pin the Phase 0 worktree guard and the Phase 12 merge
decision. This is the regression artifact for the "PR-ready stop" + "feat branch in
the main checkout" failure mode (autopilot, Codex backend). Each case states the
inputs, the **required** behavior, and the **bug** behavior it forbids.

**Canonical source of the stop-condition taxonomy:** the 6-item list in
`build/SKILL.md` Phase 12 ("PR ready is NOT a stop condition"). It is mirrored here.
The orchestrator merge **nudges** (`autopilot/SKILL.md`, `tick-protocol.md`) must
enumerate all 6 (the two safety stops — human-approval gate, policy pause — are the
ones most often dropped). Files that only describe the **mode** decision
(`worktree-onboarding.md`, `signals-spec.md`) need the autopilot-vs-interactive rule,
not the full stop list, but must not contradict it. When you change Phase 0 / Phase
12 / the launch `mode` marker, re-verify every case below still holds and that no
file contradicts the canonical list — half-applying it is this repo's #1 bug class.

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

---

## The only legitimate stop-at-ready conditions (autopilot)

Stop short of merge **only** when one holds; otherwise MUST merge:

1. a **required** review is missing,
2. **required** checks are pending or failing,
3. a **merge conflict** / dirty branch (rebase failed),
4. an **unresolved review thread** blocks the merge,
5. an explicit **human-approval gate** applies (raise via Human-Gate Protocol),
6. project **policy** (`full_auto`/strategic) says pause.
