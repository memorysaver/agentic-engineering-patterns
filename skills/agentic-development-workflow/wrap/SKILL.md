---
name: aep-wrap
description: >-
  Archives OpenSpec artifacts and cleans the workspace after a PR merges. Use
  for "wrap up", "archive", or "post-merge cleanup"; this is the final feature
  step before /aep-dispatch or /aep-reflect.
---

# Wrap

Post-merge archive and workspace cleanup. Run on the **integration branch** (`$BASE`) after the PR merges — resolve `$BASE` per /aep-git-ref "Integration Branch". This archives the OpenSpec change, converges the build's runtime signal, and removes the workspace.

**Where this fits:**

```
/aep-onboard → /aep-scaffold → [ /aep-design → /aep-launch → /aep-build → /aep-wrap ]
                                                        ▲ you are here
```

**Session:** Main session, post-merge
**Input:** Merged PR notification
**Output:** Archived OpenSpec change, converged execution records, cleaned-up workspace

---

## Phase 13: Archive & Cleanup on the Integration Branch

> **Hard guardrail:** `/opsx:archive` runs from the **main checkout on the integration branch (`$BASE`)** — never from a workspace, where it writes `openspec/specs/` and collides with parallel worktrees.

### 1. Fetch merged state and fast-forward the integration branch

Resolve `$BASE` per /aep-git-ref "Integration Branch" (override → auto-detect `develop` → `main`), then update the local integration branch to include the merged PR:

```bash
git fetch origin
git checkout "$BASE"
git pull --ff-only origin "$BASE"
git status
```

`--ff-only` is intentional — if it fails because `$BASE` has unpushed local commits, push or rebase those first. After this checkout you are on the integration branch; later steps recover its name with `BASE=$(git branch --show-current)`.

**Postcondition:** HEAD is on `$BASE` and `git status` is clean — only `openspec/` and `product-context.yaml` may differ during wrap. If code under `apps/`/`packages/` is modified, investigate first. If `openspec/changes/<name>/` is missing (a dispatch commit lost before launch), recover per /aep-git-ref "Recovery" → "OpenSpec files missing after rebase" (`git restore --source=<dispatch-sha> -- openspec/`) before proceeding.

### 2. Stop the dev server (from the workspace, if still running)

```bash
source .feature-workspaces/<name>/.dev-workflow/ports.env 2>/dev/null
lsof -ti :$SERVER_PORT | xargs kill 2>/dev/null
lsof -ti :$WEB_PORT | xargs kill 2>/dev/null
```

### 2.5. Convergence Gather — gather execution records (before archive)

Converge the workspace's build-time runtime signal into the **pre-archive change dir** (`openspec/changes/<change-name>/convergence/`) so the archive `mv` in step 3 carries it in one commit. Run the gather commands and write `execution-record.yaml` per the producer contract — copy list, manifest field list, and schema — in [references/convergence.md](references/convergence.md) §1. The gather is **best-effort**: a missing source becomes an explicit `null` or an absent copy, never a failed wrap.

> **Gate (ordering invariant — gather before archive):** placing files in `openspec/changes/<change-name>/` _before_ the archive lets them ride the archive `mv` in one commit; gathering after step 3 needs a separate commit and races teardown — a silent-loss window.

**Postcondition:** `convergence/execution-record.yaml` exists in the pre-archive change dir.

### 3. Run archive

```
/opsx:archive <change-name>
```

### 4. Commit and push the archive

Use the **control-plane commit** — the fast-forward commit pattern shared by steps 4, 5, and 5.5 (the integration branch was checked out in step 1):

```bash
BASE=$(git branch --show-current)   # integration branch, from step 1
git add openspec/                   # stages the convergence/ records from step 2.5 — the archive commit carries them, no extra commit
git commit -m "chore: archive <change-name>"
git pull --ff-only origin "$BASE"
git push origin "$BASE"
```

### 5. Sync story status from workspace signals (Product-Cycle Mode Only)

> **Standalone mode:** If `product-context.yaml` doesn't exist, skip to step 6.

If this feature was a dispatched story, read the workspace signals and cross-check against actual PR state — signals can be stale (a workspace may still show `in_review` after merge):

```bash
cat .feature-workspaces/<name>/.dev-workflow/signals/status.json
gh pr view <pr-number> --json state,mergedAt
```

If the PR is merged but the signal says `in_review`, treat the story as `completed`. From the (PR-corrected) signal, update the matching story in `product-context.yaml`:

```yaml
status: completed # from signal story_status
completed_at: <timestamp> # from signal completed_at
pr_url: <url> # from signal pr_url
cost_usd: <cost> # from signal cost_usd
```

If `story_status` is `failed`, set `status: failed` and record the structured `failure_log` under `failure_logs:` instead. Then transition any `pending` story whose dependencies are now all `completed` to `ready`. Validate YAML (`npx js-yaml product-context.yaml > /dev/null`; see product-context references/yaml-guardrails.md), then commit all transitions atomically via the **control-plane commit** (step 4) — `git add product-context.yaml`, message `chore: update story <id> status to completed`.

> **Concurrency protocol:** this is the only place story completion status enters `product-context.yaml` — workspace agents write signals; `/aep-wrap` (on the integration branch) reads signals and writes YAML.

### 5.5. Archive lessons learned

Before worktree removal — the last chance, since `git worktree remove` deletes `.dev-workflow/lessons.md` — copy any real lessons:

```bash
LESSONS=".feature-workspaces/<name>/.dev-workflow/lessons.md"
if [ -f "$LESSONS" ] && [ "$(wc -l < "$LESSONS")" -gt 12 ]; then   # >12 = content beyond the template header
  mkdir -p lessons-learned
  cp "$LESSONS" "lessons-learned/<change-name>.md"
fi
```

If a file was copied, commit it via the **control-plane commit** (step 4): `git add lessons-learned/<change-name>.md`, message `docs: archive lessons from <change-name>`. This copy is **additive** alongside the step 2.5 convergence record — the archived `convergence/` dir is the full per-change record; `lessons-learned/` stays the fast-path index `/aep-reflect` Step 1 reads.

### 6. Tear down the worker + worktree (`executor.teardown()`)

Stop the workspace's worker **before** removing the worktree — an OS-bound worker left running against a deleted directory orphans and accumulates across an autopilot run. The stop is per launch mode (recorded as `backend`/`agent_id` in autopilot state, or evident from how you launched):

```bash
# Mode-specific worker stop (each is a no-op for the other modes):
#   native-bg-subagent → TaskStop(<bare-hex bg-subagent id>) (session-bound, no team)
#   claude-bg      → claude stop <agent_id>; claude rm <agent_id>
#   codex-subagent → close_agent(<agent_id>) if still running
#   codex-exec     → nothing to kill (the exec process exited with the build)
#   legacy         → tmux kill-session -t <name> 2>/dev/null || true
```

Then remove the worktree and delete the merged feature branch per /aep-git-ref "Worktree Lifecycle" → "Remove (`/aep-wrap` step 6)" (`git worktree remove` + `git branch -d feat/<name>`; force-delete only after `gh pr view <number> --json state` confirms `MERGED`).

---

## Guardrails

> **Ordering invariant (world-derived postconditions).** The wrap step chain is
> mechanical — each step's completion is observable from the world, so an
> interrupted wrap resumes by checking, not re-doing: **gathered** =
> `convergence/execution-record.yaml` exists (pre- or post-archive location) →
> **archived** = `openspec/changes/<name>/` gone AND an `archive/*<name>*` dir
> exists → **committed** = `git status --porcelain` clean over `openspec/` →
> **status-flipped** = story shows `completed` with its completion fields set →
> **lessons-copied** = `lessons-learned/<change-name>.md` exists → **torn down** =
> worktree path gone (cross-check `git worktree list`). Steps whose postcondition
> already holds are skipped, never repeated. This is the pattern
> `aep-autopilot/references/deterministic-orchestration.md` generalizes.

---

## Reflect and Advance (Product-Cycle Mode)

> **Standalone mode:** If `product-context.yaml` doesn't exist, skip the layer gate; you may still run `/aep-reflect` to classify observations.

**At layer completion** — when `product-context.yaml` exists and every story in the active layer is `completed` — read [references/layer-advance.md](references/layer-advance.md) for the two-phase **Layer Gate Check** (run the gate, record evidence, flip `scripted_passed → passed` on covered) and **Layer Distillation** (the isolated, proposal-only synthesis). Advancing to the next layer's design is a **human-confirmed** step, recorded before the next `/aep-dispatch`.

### Feedback Loop

Run `/aep-reflect` to classify observations from this feature — bugs, refinements, and discoveries route back to the right phase, closing the loop.

---

## Next Step

Pick the next story from the dispatch queue (`/aep-dispatch`), or classify feedback from what you just shipped (`/aep-reflect`).
