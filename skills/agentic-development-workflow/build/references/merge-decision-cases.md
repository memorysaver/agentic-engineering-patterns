# Merge & Worktree Decision Cases (regression fixture)

Canonical worked cases that pin the Phase 0 worktree guard and the Phase 12 merge
decision. This is the regression artifact for the "PR-ready stop" + "feat branch in
the main checkout" failure mode (autopilot, Codex backend). Each case states the
inputs, the **required** behavior, and the **bug** behavior it forbids. When you
change Phase 0 / Phase 12 / the launch `mode` marker, re-verify every case still
holds — and that the wording matches across `build/SKILL.md`, this file,
`worktree-onboarding.md`, `autopilot/SKILL.md`, `tick-protocol.md`, and
`signals-spec.md` (the merge stop-condition list is a taxonomy — keep it identical).

---

## Case A — clean PR, no required checks, autopilot ⇒ MERGE + WRAP

**Inputs:** worker launched via `/aep-launch` (`.dev-workflow/signals/mode` = `autopilot`);
PR open, non-draft; `mergeStateStatus=CLEAN`; **zero** required checks configured;
no unresolved review threads; eval PASSED.

**Required:** Phase 12 detects autopilot mode (reads `mode`), treats "no required
checks" as mergeable (not "wait"), runs `gh pr merge --squash --delete-branch`,
sets `story_status=completed`, then `/aep-wrap` runs.

**Forbidden (the bug):** stopping at "PR ready" / asking for confirmation /
reporting `mergeStateStatus=CLEAN` as a terminal state. "PR ready" is not done —
**merged + wrapped** is done.

---

## Case B — build invoked outside a worktree ⇒ guard self-heals, never branches in main

**Inputs:** `/aep-build` starts with cwd = the main checkout (e.g. invoked without
`/aep-launch`, or a Codex `codex-subagent` that didn't honor the cd contract —
`spawn_agent` has no cwd parameter). No `.feature-workspaces/<name>` entered.

**Required:** the Phase 0 worktree guard trips (`git rev-parse --show-toplevel`
not under `.feature-workspaces/`, or branch ∉ `feat/*`), self-heals — `cd` into the
existing worktree, else `git worktree add` and enter it — then proceeds. If it
cannot establish a clean `feat/<name>` worktree, it STOPS and escalates via the
Human-Gate Protocol.

**Forbidden (the bug):** creating `feat/<name>` in the main checkout and building
there. That also poisons Case A: cwd never lands under `.feature-workspaces/`, so
the cwd fallback for autopilot detection fails too.

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
