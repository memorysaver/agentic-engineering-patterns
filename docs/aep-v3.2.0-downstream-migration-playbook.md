# AEP v3.2.0 Downstream Migration Playbook

**Status:** proven end-to-end on SIBYL, 2026-07-16 (PRs memorysaver/SIBYL#10 + #11, both merged) ·
**Applies to:** any consumer repo re-pinning v2.5.0 → v3.2.0 ·
**Companions:** [`aep-v3.0.0-migration-guide.md`](aep-v3.0.0-migration-guide.md) (the v2.x→v3.0.0
lean-skills cutover mechanics this jump crosses),
[`audits/2026-07-15-looplia-v2.5.0-to-v3.0.0-compatibility.md`](audits/2026-07-15-looplia-v2.5.0-to-v3.0.0-compatibility.md),
and SIBYL's instantiations: `docs/reviews/2026-07-16-aep-v3.2.0-migration-audit.md` (audit shape) +
PR #11 (artifact-plane execution shape).

The migration is **four phases in two PRs plus loop work**. Phases 1–2 are one PR (mechanical re-pin +
audit), Phase 3 is a second PR (artifact-plane execution), Phase 4 runs through the repo's own story
loop. Do not fold phases together: the re-pin must stay byte-verifiable, and the artifact plane needs
operator decisions the re-pin must not smuggle in.

---

## Invariants (all phases, every repo)

1. **Vendored `aep-*` is pristine** — cite it, never edit it. All repo-specific adaptation happens in
   the repo's own files (policy.md, its own skills, its stories).
2. **Forward-only** — historical layer gates are never retro-audited for the new evidence rules; new
   gates use them from day one. Historical records get at most a one-line "home migrated" annotation.
3. **In-flight work keeps its dispatch-time contract** — never re-tier, re-scope, or cost-optimize a
   story mid-build. New economics apply from the next dispatch.
4. **Naming discipline** — "verification tier" = light/standard/deep; "e2e tier" = the 1/2/3 pyramid.
   Repos that predate v3.1 use "tier" loosely; fix wording during the audit or it will bite the recipe.
5. **Calibration activation gate** — the tier-calibration loop stays record-only in a consumer until
   that consumer has ≥2 layers of verification-accounting data. The first migrated layer gate records
   actuals only (budget-box cold start).
6. **Human stop-loss until the tier cap lands** — if an eval loop repeats same-class findings for two
   consecutive rounds, stop and surface evidence (rounds, tokens, USD) to the human. The measured
   SIBYL/looplia failure mode is unbounded repeated verification; the cap is Phase-4 work, so until it
   exists the human IS the cap.

## Phase 1 — Mechanical re-pin (PR 1, ~30 min)

1. Preconditions: upstream tag + Release exist; consumer working tree clean; note the repo's current
   pin (skills-lock.json has no ref field — the pin IS the installed content hash).
2. Branch `chore/aep-pin-v3.2.0`.
3. **Remove pre-existing `aep-*` dirs from BOTH `.claude/skills/` and `.agents/skills/` first**
   (`git rm -r`). Known CLI gotcha: with entries pre-existing, `skills add` copies full duplicate
   trees into both dirs instead of laying the canonical shape.
4. Install: `npx skills add memorysaver/agentic-engineering-patterns@v3.2.0 --skill '*' -a claude-code -y`
   (22 skills — `aep-design-lens` is new since v2.6.0 and should be included).
5. **Normalize the layout**: the CLI may still emit real dirs under `.claude/skills/`. Move them to
   `.agents/skills/` and re-create `.claude/skills/aep-X → ../../.agents/skills/aep-X` relative
   symlinks, matching the repo's non-AEP skills.
6. **Verify — never trust the install output:**
   - Byte-equality: `git worktree add /tmp/aep-tag v3.2.0` in the AEP repo, then `diff -rq` every
     installed skill dir against `dirname(skillPath)` from the lock. Must be N/N identical.
   - Every `.claude` symlink resolves to a readable SKILL.md; `npx skills ls` counts 22.
   - Blast radius: `git status` touches only `aep-*` + `skills-lock.json`. The project-owned
     `skills/e2e-test/` and non-AEP skills must be untouched.
7. **Cutover safety (per repo):**
   - Live worktrees/workspaces: does any in-flight worker read the vendored tree mid-story? (SIBYL's
     kernel worktrees don't — verify per repo. An AEP-launched `.feature-workspaces/` worker DOES:
     quiesce it first per the v3.0.0 guide.)
   - AEP autopilot state: paused / zero workspaces, or quiesce.
   - Code coupling: grep the repo's own code for the changed signal fields
     (`verification_tier|tier_escalated|failure_classes|self_review|failure_class|live_policy`).
     v3.x signal changes are additive, so hits are usually zero — but a hit means Phase-2 planning.
8. **CI honesty:** if checks fail, compare against the base branch's latest run job-by-job. Pre-existing
   red is recorded in the PR body as pre-existing; never drive-by fix unrelated code in the pin PR.

## Phase 2 — Migration audit (same PR, before merge)

Three parallel sweeps; the output is a dated audit doc in the consumer repo
(`docs/reviews/<date>-aep-v3.2.0-migration-audit.md` or the repo's convention) — it becomes the
continuation contract every later session executes against.

- **Plane A — AEP-artifact surface** (the 10-row checklist):
  1. `live_policy` home (product-context.yaml → policy.md; value unchanged)
  2. `sensitive_paths` deep-tier hard floor (propose candidates from the repo's trust boundary)
  3. environment preflight probes (deploy-independent vs target-bound; CLI/TUI vocabulary if applicable)
  4. deterministic security gates (`secret_scan`/`sast`; `none` only by explicit, reasoned decline)
  5. `scripts/derive-verification-recipe.sh` + `scripts/preflight.sh` presence
  6. layer-gate evidence template: tamper-evident-class line + verification budget box
  7. dogfood reports: per-finding `Failure-Class:` line (forward-only)
  8. execution-record `verification:` accounting block (usually lands via the repo's ledger story)
  9. status.json additive signals (usually lands via the repo's sync story)
  10. journeys/results/fixtures — usually no change
- **Plane B — the repo's own skills/code** (only for self-implementing repos): where does the repo's
  runtime load its own skill prose, and which files duplicate AEP semantics (PASS rules, eval loops,
  recovery routing, embedded scaffold templates)? Each finding maps to "sync story" or "reconcile,
  don't replace" (a repo's pre-existing tamper-evidence design usually just needs to be _named_ as an
  evidence class).
- **Plane C — verification economics**: quantify the repo's actual verification spend (rounds, tokens,
  USD, suite seconds, lossy records) and map each pain to the v3.2.0 mechanism that answers it. This
  is what turns the migration from compliance into a fix for the repo's real cost problem.

Close the audit with: an ordered execution plan (§5-style), **one backlog story** for the self-owned
sync (SIBYL-202-pattern: single-source PASS rule, `failure_class` typed findings, additive signals,
template parity, retire dead skills), and named follow-ups (F1 tier-cap = the actual un-sticker,
F2 budget-box activation, F3 evidence-class reconciliation, F4 env preflight in the repo's loop).

Merge PR 1 only after the audit is in it — the PR body carries the item-by-item continuation table.

## Phase 3 — Artifact-plane execution (PR 2)

Run in a fresh session with the audit as the contract (a per-repo prompt: scope = Plane A + doc
hygiene; explicitly NOT the self-loop plane; in-flight stories untouched).

1. `/aep-e2e-skill-scaffolding` upgrade-in-place re-run, confirming each policy decision with the
   operator: live_policy (+ named milestone gates), sensitive_paths globs, probe table, security-gate
   commands.
2. Re-point old `live_policy` cross-refs to policy.md; historical docs get the one-line annotation.
3. Re-ground the repo's verification-adjacent backlog stories on the vendored canon
   (`aep-gen-eval/references/verification-economics.md` sections) so builders implement the upstream
   schema instead of re-deriving it.
4. **Functionally verify, never just emit** (the step that caught a real bug on SIBYL: a probe whose
   CLI invocation always exited 0/2 would have refused valid providers):
   - `bash -n` both scripts, then RUN them: derive-recipe against a real diff (expect the referee
     floor / a synthetic sensitive-path touch → deep), preflight all three paths (required-met exit 0,
     bogus requirement → named `REFUSING` exit 1, undeclared → SKIP).
   - Check every sensitive_paths glob matches at least one real path.
   - Repo YAML/type checks + the repo's skill-layout tests.
5. PR body = audit §1 item-by-item status table; note pre-existing CI red explicitly.

## Phase 4 — Loop plane (through the repo's own story loop, not a migration session)

1. Land in-flight stories under their original contracts; close the current layer.
2. **The first post-migration layer gate** uses the new evidence template: named tamper-evident class
   (the repo's existing digest/CI machinery usually qualifies — name it), budget box with actuals only.
3. Dispatch the sync story (SIBYL-202-pattern), then F1 (tier-derived round cap + recipe consumption in
   the repo's loop) — F1 is what actually changes the cost curve; everything before it is plumbing.
4. Calibration stays record-only until ≥2 layers of accounting data exist in THIS repo.

## Per-repo variance checklist

Before starting, answer these five questions — they are where repos differ, and every SIBYL-specific
choice above came from one of them:

1. **Layout**: where do real skill files live vs symlinks? (Match the repo's existing non-AEP pattern.)
2. **Self-implementing?** If the repo's product runs its own agent loop (SIBYL), Plane B is the bulk of
   the work; if it's a plain consumer (looplia), Plane B collapses to AGENTS.md wording checks.
3. **In-flight work**: what is mid-build right now, and does its worker read the vendored tree?
4. **Policy reality**: dogfood target / live_policy / milestone gates / trust-boundary paths are
   per-project facts, not defaults to copy.
5. **CI conventions**: what gates commits (hooks), what's already red, and does CI qualify as a
   tamper-evident class (run bound to the exact SHA, workflow defs outside story diff scope)?
