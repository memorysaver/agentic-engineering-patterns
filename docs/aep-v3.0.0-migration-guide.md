# AEP v3.0.0 Downstream Migration Guide

This guide upgrades a downstream repository from an AEP v2.x pin to the exact
`v3.0.0` release. It is written for repositories that commit installed skills for
Claude Code and Codex, keep AEP workflow state under `.dev-workflow/`, and may use
split product context.

The evidence-backed path is v2.5.0 → v3.0.0. Repositories on an earlier v2 release
must also review the cumulative release notes between their pin and v2.5.0; do not
assume the Looplia audit covers an older local schema or custom workflow fork.

The migration is a **skill re-pin plus a controlled runtime cutover**. It is not a
product-context data migration. Do not rewrite stories, layers, calibration files,
or autopilot state merely to adopt v3.0.0.

For the concrete evidence behind these instructions, see the
[Looplia v2.5.0 → v3.0.0 compatibility audit](audits/2026-07-15-looplia-v2.5.0-to-v3.0.0-compatibility.md).

## Migration Contract

The safe upgrade sequence is:

1. Reach a terminal lifecycle boundary on the old pin.
2. Re-pin both agent runtimes in an isolated PR.
3. Converge the shared skills layout.
4. Verify that product and runtime data were not changed.
5. Merge only after deterministic checks pass.
6. Start fresh agent sessions and run a bounded behavior canary.

The rollback unit is the re-pin PR. Keeping product data out of that PR makes
rollback mechanical and prevents workflow state from being coupled to skill bytes.

## What Changes in v3.0.0

### Front-tier footprint

Compared directly with v2.5.0, v3.0.0 reduces the front-tier surface while moving
detail into references:

| Metric            | v2.5.0 | v3.0.0 | Change |
| ----------------- | -----: | -----: | -----: |
| Front-tier lines  |  7,816 |  4,873 | -37.7% |
| Description words |  1,522 |    652 | -57.2% |

This is the primary token-efficiency improvement. The intended behavior is
progressive disclosure: route on concise skill descriptions, then load only the
references needed by the selected workflow.

### Intentional behavior changes

v3.0.0 is not byte-for-byte behavioral parity with v2.x. The main changes are:

- `aep-design-lens` is a new skill, bringing the release to 22 AEP skills.
- Design and calibration use an explicit gather/convergence flow.
- Heavy/light alignment policy has one canonical owner.
- Optional distillation can supplement the native lessons loop.
- Isolated agents receive machine-assembled briefs with stale-base guardrails.
- Autopilot and executor orchestration are more deterministic.
- `aep-scaffold` can audit and converge cross-runtime layout drift.

### What does not require migration

- Existing product-context schemas remain readable.
- The autopilot state version remains compatible.
- Signal formats do not require a field rewrite.
- Existing story IDs, dependency graphs, and layer numbers remain authoritative.

Treat unexpected changes under product context or `.dev-workflow/` as migration
errors, not as expected v3 churn.

## Stop Conditions

Do not start or continue the re-pin when any of these conditions is true:

- The repository contains unrelated uncommitted changes.
- A story or workspace is still active.
- An autopilot tick lock exists.
- Product context says a story is pending while runtime signals say it is active.
- Claude and Codex contain divergent real copies of the same `aep-*` skill.
- A stale lock entry cannot be distinguished from an intentionally retained skill.
- Project CI is already failing for an unexplained reason.

Resolve the condition on the old pin first. Do not use the version upgrade as a
vehicle for repairing live workflow state or product data.

## Phase 0 — Preflight and Quiesce

### 0.1 Record the old boundary

Run these commands from the downstream repository root and save the output in the
migration PR description:

```bash
git rev-parse HEAD
git status --short --branch
git worktree list --porcelain
npx skills list
```

Also record:

- The AEP version stated in `AGENTS.md`.
- Whether `.agents/skills/aep-*` are real directories.
- Whether `.claude/skills/aep-*` are real directories or symlinks.
- The current story, workspace, autopilot phase, and tick lock.

Do not infer quiescence from an empty terminal window. Use the repository's
persisted product state, workspace map, and runtime signal together.

### 0.2 Finish the old-pin lifecycle

Before installing v3.0.0:

1. Stop new dispatches.
2. Let the current story finish, merge, and wrap on the old pin.
3. Confirm the story is terminal in canonical product context.
4. Confirm the active workspace map is empty.
5. Confirm no tick lock remains.
6. Reconcile any mismatch between product state and runtime signals.

Registered worktrees are not automatically disposable. Inspect them individually;
an old registration can still contain user work.

After the old-pin lifecycle is terminal, snapshot ignored runtime files so the
later no-migration check covers more than Git-tracked product context:

```bash
AEP_RUNTIME_SNAPSHOT="${TMPDIR:-/tmp}/aep-v3-runtime-before-$(basename "$PWD").sha256"
if [ -d .dev-workflow ]; then
  find .dev-workflow -type f -exec shasum -a 256 {} + | LC_ALL=C sort
fi > "$AEP_RUNTIME_SNAPSHOT"
echo "Runtime snapshot: $AEP_RUNTIME_SNAPSHOT"
```

### 0.3 Require a clean base

```bash
test -z "$(git status --porcelain)" || {
  echo "ERROR: worktree is not clean"
  exit 1
}
```

Create a dedicated branch only after the old lifecycle is terminal:

```bash
git switch -c chore/aep-v3-repin
```

## Phase 1 — Re-pin Both Runtimes

Run the installer once per runtime. A repeated `-a` does not install to multiple
runtimes, so keep these as two commands:

```bash
npx skills add memorysaver/agentic-engineering-patterns@v3.0.0 \
  -a claude-code --skill '*' -y

npx skills add memorysaver/agentic-engineering-patterns@v3.0.0 \
  -a codex --skill '*' -y
```

The expected result is:

- 22 physical `aep-*` skill packages.
- A new `aep-design-lens` package.
- Updated AEP objects and hashes in `skills-lock.json`.
- No change to non-AEP skills from other sources.

`skills-lock.json` stores content hashes, not the release tag. The durable pin is
the combination of committed installed bytes, the lockfile, and the human-readable
pin note in `AGENTS.md`.

## Phase 2 — Converge the Cross-runtime Layout

If the repository uses real AEP directories under `.agents/skills/` and Claude
symlinks into them, the Claude installer temporarily replaces those symlinks with
real copies. Normalize the layout only after both installs have completed.

Resolve the installed scaffold path and run its read-only audit:

```bash
AEP_SCAFFOLD=.agents/skills/aep-scaffold
if [ ! -f "$AEP_SCAFFOLD/scripts/audit.sh" ]; then
  AEP_SCAFFOLD=.claude/skills/aep-scaffold
fi

bash "$AEP_SCAFFOLD/scripts/audit.sh"
```

The first audit may exit `1` because installation created layout drift. Review the
category A findings, then converge only that mechanical category:

```bash
bash "$AEP_SCAFFOLD/scripts/converge.sh" --category A
bash "$AEP_SCAFFOLD/scripts/audit.sh"
```

The second audit must exit `0`. The converge script:

- Promotes a Claude-only copy to the canonical Codex location.
- Collapses copies only when content and file modes match.
- Refuses to delete either side when copies diverge.
- Restores `.claude/skills/aep-*` as canonical symlinks.

If it fails closed on divergent copies, compare the two directories and resolve the
cause manually. Do not choose one side based only on modification time.

Repositories that intentionally keep independent real copies for every runtime must
review that choice separately; category A converges to the shared canonical layout.

## Phase 3 — Clean the Lock and Pin Metadata

### 3.1 Remove lock-only orphans

The skills CLI updates installed packages but does not necessarily remove a stale
object that has no physical skill directory. In the audited v2.5.0 downstream,
`aep-testing-guide` was such an orphan: it was absent from the release and absent on
disk, but remained in `skills-lock.json`.

Check it explicitly:

```bash
if [ -e .agents/skills/aep-testing-guide ] || \
   [ -e .claude/skills/aep-testing-guide ]; then
  echo "ERROR: inspect the physical aep-testing-guide package before continuing"
  exit 1
fi

node - <<'NODE'
const lock = require("./skills-lock.json");
if (lock.skills?.["aep-testing-guide"]) {
  console.error("ERROR: remove the lock-only aep-testing-guide object manually");
  process.exit(1);
}
NODE
```

If the object is lock-only, remove that object from `skills-lock.json` in the
migration PR. Do not remove similarly named project-owned skills without tracing
their source.

### 3.2 Update the prose pin

Change the AEP pin note in `AGENTS.md` from the previous version to `v3.0.0`. The
installer does not update this hand-written note.

Keep supplemental skills such as `project-memory` or `memory-forge` at their
existing source and pin unless they are being upgraded in a separate change.

### 3.3 Protect installed bytes

Exclude these paths from Markdown/JSON formatters:

```text
.agents/skills/**
.claude/skills/**
skills-lock.json
```

Formatting installed bytes after the installer computes their hashes breaks the
lock. Commit the re-pin with hooks bypassed only after running the checks in this
guide explicitly.

## Phase 4 — Prove Data Compatibility

### 4.1 Do not rewrite product context

The migration should leave these paths unchanged when they exist:

```text
product-context.yaml
design-context.yaml
product/
calibration/
.dev-workflow/
```

Check the tracked product tree directly:

```bash
git status --short --untracked-files=all -- \
  product-context.yaml design-context.yaml product calibration
git diff --exit-code -- \
  product-context.yaml design-context.yaml product calibration .dev-workflow

AEP_RUNTIME_SNAPSHOT="${TMPDIR:-/tmp}/aep-v3-runtime-before-$(basename "$PWD").sha256"
test -f "$AEP_RUNTIME_SNAPSHOT"
if [ -d .dev-workflow ]; then
  find .dev-workflow -type f -exec shasum -a 256 {} + | LC_ALL=C sort
fi | diff -u "$AEP_RUNTIME_SNAPSHOT" -
```

An empty diff is the expected result. If a formatter or tool changed these files,
remove those changes from the migration branch and investigate separately.

### 4.2 Parse all YAML

Use the same parser family as AEP's validation checks:

```bash
bash <<'BASH'
set -euo pipefail

yaml_files=()
for file in product-context.yaml design-context.yaml; do
  [ -f "$file" ] && yaml_files+=("$file")
done
for directory in product calibration; do
  [ -d "$directory" ] || continue
  while IFS= read -r -d '' file; do
    yaml_files+=("$file")
  done < <(find "$directory" -type f -name '*.yaml' -print0)
done

for file in "${yaml_files[@]}"; do
  npx --yes js-yaml "$file" >/dev/null
done

printf 'Validated %s YAML files\n' "${#yaml_files[@]}"
BASH
```

This is a syntax check, not a reason to normalize existing values. Preserve
canonical story and layer semantics.

### 4.3 Preserve runtime state

Do not hand-edit autopilot state to make it look idle. If the state, product story,
workspace map, and signal disagree, stop and reconcile the old run before cutover.

The v3.0.0 state schema is compatible with the audited v2.5.0 state, but compatibility
does not make a mid-story model/session handoff safe.

### 4.4 Account for known protocol-document drift

The v3.0.0 `aep-validate` protocol reference still contains legacy prose for some
canonical product fields. It can describe:

- `description` as a string instead of the canonical object form.
- `business_value` as text instead of numeric or `null`.
- `done` instead of the canonical terminal statuses `completed` or `deferred`.
- Older layer-gate evidence keys.

This drift predates v3.0.0. Do not rewrite valid canonical product data to satisfy
the stale prose. If a strict checker flags only these known differences, record the
finding and track the upstream documentation fix separately.

## Phase 5 — Review the Migration Diff

Expected changes are limited to:

- `.agents/skills/aep-*/**`
- `.claude/skills/aep-*` layout entries
- `skills-lock.json`
- The AEP version note in `AGENTS.md`

Expected release-level additions include `aep-design-lens`. Some obsolete reference
files may disappear because v3 consolidates canonical ownership.

The migration diff must not include:

- Product stories, layers, activities, modules, or calibration output.
- `.dev-workflow/` state or signals.
- Application source or tests.
- Unrelated project-owned or supplemental skills.
- Formatter-only churn inside installed skill files.

Review both names and modes:

```bash
git status --short
git diff --stat
git diff --summary
git diff --check
```

## Phase 6 — Deterministic Verification

### 6.1 Verify inventory and layout

```bash
npx skills list

test "$(find .agents/skills -mindepth 1 -maxdepth 1 \
  -type d -name 'aep-*' | wc -l | tr -d ' ')" = "22"

test -f .agents/skills/aep-design-lens/SKILL.md

for link in .claude/skills/aep-*; do
  test -L "$link"
  test -e "$link"
done

AEP_SCAFFOLD=.agents/skills/aep-scaffold
if [ ! -f "$AEP_SCAFFOLD/scripts/audit.sh" ]; then
  AEP_SCAFFOLD=.claude/skills/aep-scaffold
fi
bash "$AEP_SCAFFOLD/scripts/audit.sh"
```

If the repository uses a different intentional layout, replace the symlink
assertions with documented equivalent checks; do not silently skip layout
verification.

### 6.2 Verify lock completeness

Every physical AEP skill should have one lock object, every AEP lock object should
have a physical package, and `aep-design-lens` should exist in both sets. A lock-only
`aep-testing-guide` is a failure.

```bash
node - <<'NODE'
const fs = require("node:fs");
const lock = require("./skills-lock.json");

const physical = fs
  .readdirSync(".agents/skills", { withFileTypes: true })
  .filter((entry) => entry.isDirectory() && entry.name.startsWith("aep-"))
  .map((entry) => entry.name)
  .sort();

const locked = Object.entries(lock.skills ?? {})
  .filter(
    ([name, metadata]) =>
      name.startsWith("aep-") &&
      metadata?.source === "memorysaver/agentic-engineering-patterns",
  )
  .map(([name]) => name)
  .sort();

const missingFromLock = physical.filter((name) => !locked.includes(name));
const missingFromDisk = locked.filter((name) => !physical.includes(name));

if (
  physical.length !== 22 ||
  missingFromLock.length ||
  missingFromDisk.length ||
  !physical.includes("aep-design-lens")
) {
  console.error({ physical: physical.length, missingFromLock, missingFromDisk });
  process.exit(1);
}

console.log("AEP inventory: 22 physical packages matched by 22 lock objects");
NODE
```

The installed skill bytes are the authoritative pin. Do not run a formatter or an
unversioned restore command between installation and commit.

### 6.3 Run project checks

Run the downstream repository's normal CI-equivalent checks, then:

```bash
git diff --check
```

All pre-existing failures must be explained in the PR. Do not classify a newly
introduced failure as unrelated without reproducing it on the old pin.

## Phase 7 — Commit and Merge

Stage only the migration paths:

```bash
git add .agents/skills .claude/skills skills-lock.json AGENTS.md
git diff --cached --check
git commit --no-verify -m "chore: re-pin AEP skills to v3.0.0"
```

The `--no-verify` flag protects pinned bytes from formatter hooks; it is not a
substitute for the explicit checks above.

The PR description should include:

- Old commit and AEP pin.
- Confirmation that the old story/workspace lifecycle was terminal.
- Before/after skill counts.
- Scaffold audit result.
- Product-tree no-diff result.
- YAML validation count.
- CI results.
- The planned canary and rollback owner.

Merge only when deterministic checks are green and no workflow process is still
using the old pin.

## Phase 8 — Post-merge Canary

### 8.1 Start fresh sessions

Close or discard agent sessions that loaded v2.x skill metadata. Start new Claude
Code and Codex sessions from the merged commit so routing descriptions, references,
and installed bytes all come from v3.0.0.

### 8.2 Run routing canaries

Use representative prompts and confirm the front-tier router selects the intended
skill before allowing autonomous execution:

| Canary intent                                    | Expected route                      |
| ------------------------------------------------ | ----------------------------------- |
| Install or converge an existing project          | `aep-onboard` / `aep-scaffold`      |
| Choose and launch the next product story         | `aep-dispatch` / `aep-autopilot`    |
| Evaluate a design decision or alignment          | `aep-design-lens` / `aep-calibrate` |
| Validate context or generate evaluation evidence | `aep-validate` / `aep-gen-eval`     |

The release contains direct/boundary routing observations and dry-run behavior
scenarios, but not a same-model v2.5.0-versus-v3.0.0 A/B benchmark. Treat downstream
canary evidence as required, not optional reassurance.

### 8.3 Bound the rollout

For the first one or two v3 stories:

1. Rehearse the next dispatch read-only.
2. Confirm the prior layer gate is satisfied.
3. Confirm story, activity, and module references resolve.
4. Observe workspace creation and the first executor handoff.
5. Confirm signals and canonical product state stay consistent.
6. Inspect the first v3 wrap/convergence result before widening autonomy.

Do not combine the first v3 run with a schema rewrite or a large product-context
cleanup. Separate changes make routing or orchestration regressions diagnosable.

## Rollback

Rollback is appropriate when deterministic verification passed but the canary shows
a routing, orchestration, or stability regression attributable to v3.0.0.

1. Stop new dispatches.
2. Let any already-started write settle, or preserve its signals and workspace for
   investigation.
3. Revert the merged re-pin commit or PR with `git revert <merge-sha>`.
4. Run the old-pin deterministic checks.
5. Start fresh agent sessions from the reverted commit.
6. Record the failing prompt, selected route, loaded references, runtime state, and
   expected behavior before retrying the upgrade.

Do not use `git reset --hard` as a migration rollback. A revert preserves the audit
trail and avoids destroying canary work. Because the migration PR excludes product
data, rollback should affect only installed skills, layout entries, the lockfile,
and the prose pin.

## Looplia-specific Cutover Gate

The July 15 compatibility audit found Looplia structurally compatible with v3.0.0,
but not yet at a safe runtime cutover boundary. Before applying this guide there:

1. Finish, merge, and wrap `FIX-L31-DOGFOOD-SECRETS-001` on v2.5.0.
2. Reconcile its pending canonical story state with the active runtime signal.
3. Resolve or explicitly map its missing `e2e-verification` module reference.
4. Confirm the active workspace map and tick lock are empty.
5. Inspect all 29 registered worktrees; do not delete them in bulk.
6. Remove the lock-only `aep-testing-guide` object during the re-pin.
7. Keep all product-context YAML byte-for-byte unchanged.
8. After merge, prove the Layer 31 gate and rehearse Layer 32 dispatch read-only
   before resuming autopilot.

The audit also found stale module/activity references concentrated in terminal
history. Those are cleanup findings, not reasons to mutate history inside the v3
migration PR.

## Final Checklist

- [ ] Old-pin story is terminal and wrapped.
- [ ] Active workspace map is empty.
- [ ] Tick lock is absent.
- [ ] Migration started from a clean branch.
- [ ] Claude Code and Codex were each installed at exact tag `v3.0.0`.
- [ ] Category A layout convergence completed without ambiguity.
- [ ] Exactly 22 physical AEP skills exist, including `aep-design-lens`.
- [ ] No lock-only `aep-testing-guide` object remains.
- [ ] `AGENTS.md` states `v3.0.0`.
- [ ] Product context and `.dev-workflow/` have no migration diff.
- [ ] All product/calibration YAML parses.
- [ ] Scaffold audit exits `0`.
- [ ] Project CI and `git diff --check` pass.
- [ ] Migration PR contains only pin/layout metadata.
- [ ] Fresh sessions are used after merge.
- [ ] Routing and first-story canaries pass before full autonomy resumes.
- [ ] Rollback owner and merge SHA are recorded.

## Related Documentation

- [README: upgrading to a new release](../README.md#upgrading-to-a-new-release)
- [Skill authoring standard](decisions/skill-authoring-standard.md)
- [Build convergence pipeline](decisions/build-convergence-pipeline.md)
- [Deterministic orchestration](decisions/deterministic-orchestration.md)
- [Lean skills refactor plan](plans/2026-07-14-lean-skills-refactor.md)
- [Looplia compatibility audit](audits/2026-07-15-looplia-v2.5.0-to-v3.0.0-compatibility.md)
