---
name: aep-build
description: >-
  Implements one feature autonomously in an isolated workspace, including task
  commits, review, tests, PR, feedback, and merge. Use when a workspace agent
  should build or implement; this workflow does not interact with the user.
---

# Build

Autonomous feature implementation inside an isolated git worktree on a fresh `feat/<name>` branch. Initialize the harness, implement tasks linearly (one commit per `tasks.md` row), review, test, create a PR, handle feedback, and merge — all without user interaction.

> **Phase numbering:** Phases 1–3 (explore, propose, review) were completed on the integration branch via `/aep-design`. This skill begins at Phase 0 (workspace init) and continues from Phase 4 (implementation).

> **Loop hygiene (G7):** Each phase runs under a unified `--max-turns` runaway budget. Hitting the cap is **not** completion — treat it as "possibly unsolvable → escalate" (Human-Gate Protocol), distinct from a genuine clean finish. This keeps a stuck phase (e.g. a non-converging Phase 5 loop) from silently burning turns and reading as done.

**Where this fits:**

```
/aep-onboard → /aep-scaffold → [ /aep-design → /aep-launch → /aep-build → /aep-wrap ]
                                              ▲ you are here
```

**Session:** Workspace session, autonomous
**Input:** OpenSpec artifacts on disk (committed to the integration branch by `/aep-design`)
**Output:** Merged PR

---

## Phase 0: Initialize Tracking

Set up the tracking infrastructure and environment. The branch already exists (`/aep-launch` created `feat/<name>`); do not pre-create commits — implement linearly in Phase 4. Run these steps in order; each ends in a checkable postcondition.

**Step 0 — Worktree guard (FIRST, before anything else).** You MUST run inside your own feature worktree, never the main checkout. `/aep-build` does not assume `/aep-launch` succeeded — it **verifies**. On Codex `codex-subagent` the worktree binding is a soft prompt-contract (`spawn_agent` has no cwd parameter), so a worker can silently start in the main checkout and create `feat/<name>` there — which breaks Phase 12 autopilot detection. This guard makes that impossible:

```bash
WS="<name>"   # your story/change name from the bootstrap prompt — your worktree is
#             .feature-workspaces/<name> on branch feat/<name>, matching the active
#             OpenSpec change (ls openspec/changes/). If WS is still the literal
#             "<name>", resolve it from the prompt/change BEFORE continuing.
TOP=$(git rev-parse --show-toplevel)
BRANCH=$(git branch --show-current)

if [[ "$TOP" != *"/.feature-workspaces/$WS" || "$BRANCH" != "feat/$WS" ]]; then
  echo "GUARD: not in worktree .feature-workspaces/$WS on feat/$WS (top=$TOP branch=$BRANCH)"
  # /aep-launch OWNS worktree creation. The guard's only job is to ENTER the
  # worktree launch already made — NEVER create a branch/worktree here and NEVER
  # touch the main checkout (no `git switch`, no `git worktree add -b`).
  ROOT=$(git worktree list --porcelain | sed -n '1s/^worktree //p')   # main worktree root
  if [ -n "$ROOT" ] && [ -d "$ROOT/.feature-workspaces/$WS" ]; then
    cd "$ROOT/.feature-workspaces/$WS"          # launch made it; just enter it
  else
    # No worktree for $WS. Do NOT build here, do NOT improvise one. STOP and
    # escalate (Human-Gate Protocol / needs-human.md): "no worktree for $WS —
    # run /aep-launch first, or the launch misfired."
    echo "ESCALATE: no .feature-workspaces/$WS — run /aep-launch first"; exit 1
  fi
fi
# Re-verify before doing ANY work:
TOP=$(git rev-parse --show-toplevel); BRANCH=$(git branch --show-current)
[[ "$TOP" == *"/.feature-workspaces/$WS" && "$BRANCH" == "feat/$WS" ]] \
  || { echo "STILL NOT IN feat/$WS WORKTREE — STOP and escalate"; exit 1; }
```

Run Phases 4–12 only inside `.feature-workspaces/<name>` on `feat/<name>`. Worktree creation belongs to `/aep-launch`; if yours is missing, **escalate** — do not improvise one in the main checkout or mutate any other worktree's branch. **Postcondition:** guard exits 0 (`show-toplevel` ends in `.feature-workspaces/<name>`, branch is `feat/<name>`).

1. **Discover the OpenSpec change.** List `openspec/changes/` to find the active change; read `proposal.md`, `design.md`, `specs/**/*.md`, `tasks.md`. **Post:** change dir identified, artifacts read.
2. **Create the tracking folder:** `mkdir -p .dev-workflow`. **Post:** dir exists.
3. **Gitignore it:** `grep -q '.dev-workflow' .gitignore || echo '\n# Development workflow tracking (per-workspace)\n.dev-workflow/' >> .gitignore`. **Post:** `.dev-workflow/` is gitignored (ephemeral — never committed). **Post:** `grep -q` exits 0.
4. **Create the progress file** from this installed skill's template. Resolve the package from the worktree root, preferring the canonical Codex install: `ROOT=$(git rev-parse --show-toplevel); AEP_BUILD_DIR="$ROOT/.agents/skills/aep-build"; [ -f "$AEP_BUILD_DIR/references/progress-template.md" ] || AEP_BUILD_DIR="$ROOT/.claude/skills/aep-build"; [ -f "$AEP_BUILD_DIR/references/progress-template.md" ] || { echo "aep-build progress template not installed" >&2; exit 1; }; cp "$AEP_BUILD_DIR/references/progress-template.md" ".dev-workflow/progress-$(git rev-parse --short HEAD).md"`. Fill in feature name, base SHA, date, change name, mode; mark design Phases 1–3 pre-completed. **Post:** file exists.
5. **Read `tasks.md`.** It _is_ the skeleton — you implement its rows linearly in Phase 4, one commit per row, so the commit history mirrors it 1:1. **Post:** task list known.
6. **Run project setup.** If `.claude/hooks/workspace-setup.sh` exists, `bash` it (handles deps, dev server, ports, DB/seed, `.env`); otherwise read the README or ask the user how to set up. The hook MUST write `.dev-workflow/ports.env` with at minimum:

   ```
   WEB_PORT=<port>
   SERVER_PORT=<port>
   BASE_URL=http://localhost:<web-port>
   SERVER_URL=http://localhost:<server-port>
   ```

   **Post:** `.dev-workflow/ports.env` exists with those four keys.

7. **Generate sprint contracts** to `.dev-workflow/contracts.md`, one section per task, from `specs/*.md` + `design.md` + `tasks.md`. Format: `references/contract-template.md`. **Post:** file exists with one section per task.
8. **Generate the feature verification list** `.dev-workflow/feature-verification.json` (skeleton + generator-ownership rule in `references/harness-artifacts.md`). The generator writes only `commit_sha`; it MUST NOT modify `verification_steps` or `passes`. **Post:** file exists.
9. **Generate the session recovery script** `.dev-workflow/init.sh` (body in `references/harness-artifacts.md`); `chmod +x` it. **Post:** file exists and is executable.
10. **Initialize signals:** `mkdir -p .dev-workflow/signals`, create `status.json` (skeleton in `references/harness-artifacts.md`), and read `.dev-workflow/signals/feedback.md` for main-session input. Full spec: `/aep-launch` references/signals-spec.md. **Post:** `status.json` exists.
11. **Create the lessons file** `.dev-workflow/lessons.md` (template in `references/harness-artifacts.md`); fill the header from the change. Sections populate in Phase 4 (opt-in) and Phase 9 (summary). **Post:** file exists.

Update the Phase 0 checkbox in the progress file. **Signal:** set `status.json` `"phase": 0, "phase_name": "initialized", "completion_pct": 10`.

> **Product-cycle mode:** if this change maps to a story in `product-context.yaml` (change name matches a story's `openspec_change`), read `references/story-status.md` for build-time dependency re-verification and the signal-based status protocol. Standalone (no `product-context.yaml`): skip it.

---

## Phase 4: OpenSpec Apply

Implement each `tasks.md` task **linearly, one task at a time = one commit**, then record the commit SHA. For each task: implement (edit files directly), inspect (`git status` / `git diff`), run targeted tests, then commit `feat(<scope>): <task description>` and write the 8-char short SHA into the matching `feature-verification.json` entry's `commit_sha` (never touch `verification_steps` or `passes`). The commit shape, amend rules, and recovery are the canonical **One-Commit-per-Task Pattern** — see `/aep-git-ref` "The One-Commit-per-Task Pattern (Phase 4 of `/aep-build`)".

Invoke the apply skill for per-task guidance:

```
/opsx:apply
```

> **UI-facing stories — obey the Object Map.** If the dispatched context package includes an Object Map slice (objects, attributes, relationships, CTAs, screens), treat it as binding structure: build the listed screens object-first (noun→verb — object collection/detail, the given CTA placements), use the specified core attributes, and do **not** introduce objects/screens outside the slice or collapse a flow into a step-by-step wizard unless the slice marks it `task_oriented`. Visual look (`calibration/visual-design.yaml`), copy voice (`copy-tone`), and journey/transition (`ux-flow`) still govern taste — the Object Map governs object structure and CTA grammar.

**Lesson capture (opt-in per task):** after a task, if you hit something noteworthy — a non-obvious solution, an error that took multiple attempts, missing docs — append it to `.dev-workflow/lessons.md` under `## Solutions` / `## Errors` / `## Missing` with a `### <title>` sub-heading. Only when it is genuinely worth passing on.

Update the progress checkbox per task; mark Phase 4 done when all tasks are committed. **Signal:** set `status.json` `"phase": 4` at start; keep `task_current`, `task_index`, `task_total`, `completion_pct` current as tasks progress.

---

## Phase 5: Code Review & Verification

Verify the code before testing. This phase uses the **generator/evaluator pattern** — full mechanics in `/aep-gen-eval` references: scoring-framework.md (dimensions, thresholds, presets), agent-contracts.md (role separation, prompt templates), eval-protocol.md (request/response format, verification JSON, convergence rules).

**Phase 5 entry — binding tier derivation (BEFORE any evaluation round spends the cap).** Re-run the verification-tier derivation from the **actual diff** (`git diff --name-only "$BASE"...HEAD`) against `policy.md`'s `sensitive_paths` + the referee-asset rule, and emit `.dev-workflow/verification-recipe.json` (tier + dimension preset + hard floors + `scope_drift`/`tier_escalated` flags — schema and derivation function in `/aep-gen-eval` references/verification-economics.md; the generated e2e skill ships a reference script). Rules: the tier may only go **up** from the dispatch brief's provisional tier; a diff outside declared `files_affected` recomputes from the diff and sets `scope_drift: true`; referee-asset diffs floor at `standard`; a negative assertion delta forces at least one evaluation round. **Phase 5 refuses to start without the recipe file** — in product-cycle mode with a dispatch brief, no recipe means derive it now, not skip it; a standalone build with no brief runs `deep` defaults (fail-open) and records a recipe saying so. Publish `verification_tier` and `tier_escalated` to `status.json` (signals-spec is the schema owner).

**Completeness check (always, generator-side):**

1. Re-read the proposal (including any design-review adjustments).
2. Walk each task's commit — `git show <commit-sha>` against its task description.
3. Check `.dev-workflow/contracts.md` — verify each task's success criteria are met.
4. If a task is incomplete, add a follow-up commit (`feat(<scope>): complete <task>` / `fix(<scope>): <issue>`) and loop back to Phase 4. Do not rewrite prior commits.

**Quality review — with separate evaluator (full mode):** whether an evaluator runs is decided by the **binding tier in `verification-recipe.json`** (`standard`/`deep` ⇒ evaluator; `light` ⇒ structured self-review) — **never by whether a criteria file happens to exist**. `/aep-launch` intentionally writes no `.dev-workflow/evaluator-criteria.md` for a provisional-`light` story, so when the binding derivation upgrades the tier (sensitive paths, referee assets, scope drift), assemble the criteria **now** from the binding recipe (`dimension_preset` + `hard_floors`; scoring-framework.md → How to use presets) before entering the eval loop — falling through to light self-review because the file is missing would re-open the under-declaration escape the two-point derivation exists to close. Under a binding `light` tier, run the structured self-review instead and publish its durable completion signal to `status.json` (`self_review: {result, sha, report}` — signals-spec is the schema owner; this is what the autopilot quality gate reads). With the criteria in place, the evaluator runs via `executor.spawn_evaluator()`. **Spawn authority leaves the generator where an orchestrating layer exists** (autopilot nudges the spawn; the main session owns it in interactive runs) — the orchestrator assembles the evaluator's context (criteria file, contracts, diff range) so the generator cannot curate what its judge sees; only in genuinely standalone builds does the generator invoke the spawn recipe itself. In every mode the evaluator prompt marks `eval-request.md` as the **generator's untrusted claim** — data to verify, never framing to adopt (/aep-gen-eval references/agent-contracts.md). The spawn is **mode-dispatched** and — whatever the mode — the evaluator runs **worktree-bound** against this workspace's files + git state. Spawn recipes per mode (native-bg-subagent/claude-bg foreground Task subagent, codex-subagent/codex-exec `codex exec --cd`, legacy tmux split, workflow verify stage) live in `/aep-executor` references/backends.md — the Context labels there name the spawn _mechanism_, not the read-only/CI _use_ (a Task-subagent evaluator is not a main-session read-only reviewer; a `codex exec` evaluator is not an API/SDK CI job).

**Evaluation loop** — for each round N (start 1, **max = the recipe's tier cap**: `light` 0 / `standard` 2 / `deep` 5; 5 when no recipe exists):

1. **Write eval-request** `.dev-workflow/signals/eval-request.md` per eval-protocol.md (Signal Files).
2. **Compose the evaluator prompt** from agent-contracts.md (Evaluator Prompt — Code Quality), customized with the workspace paths (`criteria_file`, `eval_request_file`, `spec_directory`, `contracts_file`, `verification_file`, `eval_response_file=.dev-workflow/signals/eval-response-<N>.md`).
3. **Spawn the evaluator** (mode-dispatched; the prompt _is_ the spawn prompt — recipes in backends.md).
4. **Confirm the response exists** — native spawns return on completion; legacy polls: `while [ ! -f .dev-workflow/signals/eval-response-<N>.md ]; do sleep 15; done`.
5. **Read the response** (legacy only: `tmux kill-pane -t :.1`).
6. **On FAIL, run the taxonomy step first** — every FAIL finding carries an evaluator-authored `Failure-Class` (/aep-gen-eval references/verification-economics.md → Failure Taxonomy): `environment` → ops checklist via the Human-Gate Protocol, zero further rounds; `harness-flake` → quarantine + harness story (evidence-gated, ratified by wrap/reflect); `scope` → human gate → `/aep-reflect` re-slicing; unbuilt in-repo dependency → `/aep-dispatch` re-ordering. Only `product-defect` findings continue: **fix per the recovery rung**, add follow-up commits, then loop to step 1 with round N+1. Do not rewrite history — the PR sees fixes as new commits.

The evaluator (only) updates `.dev-workflow/feature-verification.json` pass/fail per eval-protocol.md field ownership. Track `eval_round` + `recovery_rung` in `status.json`, and on every FAIL transcribe the response's per-finding class counts verbatim into `status.json.failure_classes` (evaluator-authored values — the generator transcribes, never reclassifies; signals-spec owns the schema and the mixed-class routing rule).

**Cap exhaustion is defined, not silent:** when `standard` exhausts its 2 rounds on a genuine `product-defect`, **auto-escalate once to `deep`** — update `verification-recipe.json` and `status.json` (`tier_escalated: true`) and continue the ladder from where it left off. Only after `deep`'s ladder exhausts does the story reach the human gate.

**Recovery ladder (generator-side escalation on FAIL)** — don't retry the same way every round. Rungs key to position past the tier's cap (rendered here for `deep`/cap-5):

- Rounds **1–2**: same generator fixes the FAIL items in place.
- Round **3**: **re-ground** — re-read the full spec + design + contracts from scratch, then re-attempt.
- Round **4**: spawn a **fresh `native-bg-subagent` generator** told "the previous approach failed on X; take a different design path" (inherits the existing worktree).
- Round **5**: **decompose** — split the story, attempt the smallest viable slice, surface the split.
- **After 5**: ladder exhausted → escalate via the Human-Gate Protocol with type `eval_not_converging`, recording the ladder history.

Generator≠evaluator separation holds — the evaluator only scores; re-ground / fresh generator / decompose are all generator-side. **The ladder is `product-defect` machinery only** — the taxonomy step (eval-loop step 6) routes every other class off the ladder before any rung is chosen. **Skip the ladder and escalate immediately** on a hard-failure / security FAIL (auth-model gap, data-exposure risk — `product-defect`, escalation preserved), a spec contradiction (`scope`), or a missing external dependency (`environment` — needs a named refusal tag) — these need human judgment. Full rung rationale + the rung-4 fresh-generator spawn contract: `/aep-gen-eval` references/recovery-ladder.md.

**Quality review — without evaluator (light mode):** self-review for correctness (logic errors, off-by-one, edge cases), security (input validation, auth, SQL parameterization), performance (N+1, missing indexes, unbounded loops), and conventions (naming, structure, error handling, imports). Self-review is lenient — walk `feature-verification.json` steps manually. Document findings in `.dev-workflow/code-review-<feature>.md` and fix issues found.

Update the Phase 5 checkbox. **Signal:** set `status.json` `"phase": 5, "phase_name": "code-review"`.

### The Human-Gate Protocol (any phase)

When you hit a decision **only the human can make** — design ambiguity the spec doesn't resolve, eval non-convergence after max rounds, a product judgment call — do not guess and do not silently stall. Raise a gate:

1. **Record it (always):** append to `.dev-workflow/signals/needs-human.md`:

   ```markdown
   ## <ISO8601> — Phase <N>

   **Question:** <the decision needed, with the options you considered>
   **Context:** <why you can't decide autonomously>
   ```

   and set `"blocked_on": "human"` in `status.json`.

2. **Raise it on your launch mode's transport** (per `/aep-executor` references/backends.md, "The Human-Gate Protocol"). The answer always comes back through the **main agent** (hub-and-spoke) — you never need the human to visit your surface:
   - **push-channel modes** (codex-subagent, legacy): the parent thread / orchestrator delivers the answer live (ask the parent; or the file is the transport and a nudge relays it).
   - **no-push modes** (native-bg-subagent, claude-bg, codex-exec, workflow, headless): **gate-and-park** — commit WIP (or leave the tree clean), update `status.json`, and **end your run cleanly** (workflow agents return a structured `gated` result carrying the question). The orchestrator resumes a worker into this same worktree with the answer — your state is in the worktree + `.dev-workflow/`, so nothing is lost.

3. **Block-in-place modes only:** continue anything not blocked on the answer; otherwise wait, re-checking `feedback.md`.
4. **On answer** (by push, or in your resume prompt after a park): append `resolved: <summary>` under your entry, clear `blocked_on`, and proceed.

---

## Phase 6: Browser Testing (Dogfood)

This phase is **journey-first**: **Step A** authors the journey from the layer's acceptance criteria; **Step B** executes it. Step A is **unconditional** (independent of build mode, `dogfood_target`, `journey_timing`). Only _execution_ (Step B) varies: skipped in Light mode or when `dogfood_target == none`, and deferred to the `/aep-wrap` gate under `journey_timing: post-deploy`.

### Step A — Author the layer's journey (always, pre-merge)

If the project has an `e2e-test` skill with a `journeys/` dir (`dogfood_target != none`) and no journey covers this layer — or new acceptance criteria lack scenarios — author/extend the journey now in `skills/e2e-test/journeys/<NN-slug>.md` (path planned in `layer_gates[N].journeys`; copy the template in `journeys/README.md`, set front-matter `target:`, `layer:`, `covers:` — the acceptance-criterion ids this journey proves; see `/aep-e2e-skill-scaffolding` references/bdd-journeys.md):

- **One scenario per acceptance criterion** (from `stories[].acceptance_criteria`), each `Then` → a concrete **`Verify`** (API response / DB row / inspector check), intent-level and tool-agnostic.
- The journey **must map every acceptance criterion to a scenario `Verify`** (or a Tier-1 scripted case for deterministic behavior, or a Tier-3 API check for backend/async state) **before execution**. A deliberately deferred criterion carries a `WAIVER: <reason>` line, not silence. This is what makes the layer _covered_, not just _touched_ — `/aep-wrap` refuses to flip the gate to `passed` while criteria are silently uncovered.
- **Commit the journey file with the feature now (pre-merge)** — even under `journey_timing: post-deploy`, where only the journey's _execution_ is deferred (Step B). The authored, committed journey is the deliverable; the post-deploy gate executes it, never authors it.

> `dogfood_target == cli` (CLI/library): author a `target: cli` journey — bash drives the built binary, each `Then` → a `Verify` on exit code / stdout / stderr / filesystem. Same authoring rules; only the tool track differs.
> `dogfood_target == none` (no runnable surface — config/schema/docs, no `journeys/` dir): no journey to author — instead map every acceptance criterion to a Tier-1 scripted case / Tier-3 API check, then jump to Step B's coverage note.

Tier applicability is per project type (a CLI/library layer is `[1,2]`; only a genuine `none`-target layer is all Tier-1, no journey). Three-tier model: `/aep-e2e-skill-scaffolding` references/three-tier-model.md.

### Step B — Execute the authored journey (dogfood)

> **Light mode:** skip Step B (Step A still ran). Outside Light mode, do **not** skip Step B just because `agent-browser` is absent — pick a host-aware method and degrade.

**Environment preflight (before any journey spend).** Run the probes declared in `skills/e2e-test/policy.md` (/aep-gen-eval references/verification-economics.md → Environment Preflight Gate):

- **Deploy-independent probes** run here **on every story, regardless of `journey_timing`** — required secret _names_ present in CI config, expected account fingerprint vs. actual auth identity, env-var names wired in deploy config. They need no live target.
- **Target-bound probes** (target reachable, bindings present, fixtures seedable) run here only under `journey_timing: pre-merge`; under `post-deploy` they run at the `/aep-wrap` gate.

An unmet **required** precondition is a named refusal (`REFUSING [dogfood-secret-absent:<NAME>]`) — surface it via the Human-Gate Protocol as an ops checklist and stop the journey: zero scenarios run, zero rounds spent, `environment` by construction. Never file a code story for it (except the ownership check's paired `product-defect` finding when the wiring lives in this repo). An absent **optional** capability (the live half of a non-milestone gate under `live_policy: milestone_gates_only`) is **SKIP**, not REFUSED — it blocks nothing.

**Run the layer's journey** (intent, not a click script). The journey's `target:` (web/mobile/desktop/cli) plus `skills/e2e-test/tool-selection.md` — the project-local projection of `e2e_tool(target_type)` — resolve which automation tool to use. Verify state per each `Verify` line.

**Pick the tool, host-aware** via `e2e_tool(target_type)` (web wrapper `dogfood_method()`) from `/aep-executor` references/dogfood-validation.md — the native tool for this target/host/mode (Claude Code web → `/agent-browser:dogfood` if healthy else webwright, then degrade; Codex web → in-app browser / Playwright / CLI / API; mobile/desktop → agent-device / computer-use / agent-browser-Electron; cli → bash asserting exit code / stdout / stderr / filesystem).

**Resolve the target from `skills/e2e-test/policy.md`** (`dogfood_target`) — don't assume local:

- `cli` → run the built binary **locally via bash** (no URL); seed local fixtures with `bash skills/e2e-test/scripts/seed.sh` if needed.
- `none` → **skip the journey dogfood** (Tier-2 N/A); prove criteria via Tier-1 / Tier-3.
- `local` → `source .dev-workflow/ports.env` and use `$BASE_URL` (`target_url(local)`).
- `deployed:<url>` → use that URL (e.g. a Cloudflare preview/prod) and seed that same target (`SERVER_URL=<url> bash skills/e2e-test/scripts/seed.sh`), not local. If `journey_timing` is `post-deploy`, execution defers to the `/aep-wrap` gate (target isn't up pre-merge) — Phase 6 here runs only Tier-1/Tier-3, but the Step-A journey file is still committed now.

```bash
source .dev-workflow/ports.env   # local target → $BASE_URL  (deployed target: use the URL from policy.md)
```

Whatever the method, emit the unified severity/category/repro report format (dogfood-validation.md → Unified report format) so the classifier stays host-agnostic. Document results in `.dev-workflow/dogfood-<feature>.md` — **write the report file, not just chat output**: that path is the ingestion contract (dogfood-validation.md → On issue), so `/aep-watch`'s `dogfood_report` source can auto-file findings. Chat-only findings are a dead end.

**Confirm the coverage matrix.** After executing, recompute the layer's coverage matrix: every acceptance criterion maps to the scenario `Verify` / scripted case / API check that proves it. Because Step A authored a scenario per criterion, this is a confirmation — any uncovered criterion is a Step-A miss: author it now and re-run until `coverage.criteria_covered == coverage.criteria_total` (deferrals carry a `WAIVER: <reason>` line).

**Signal:** set `status.json` `"phase": 6, "phase_name": "dogfood-testing"`.

---

## Phase 7: Finalize the Journey + Record the Layer Gate

> Skip if the project has no `e2e-test` skill. **Light mode:** skip this phase.

The journey was **already authored in Phase 6 Step A**. This phase **finalizes** that durable BDD regression artifact with reality discovered during execution. In `skills/e2e-test/journeys/`:

- **Refine the existing journey** for this layer — fold in selector/route drift, extra `Verify` lines, corrected preconditions (you are refining, not creating). Under `journey_timing: post-deploy` the journey executes later at the `/aep-wrap` gate, so this pre-merge finalize folds in only what local / Tier-3 checks revealed.
- Each `Then` keeps a concrete **Verify** line (API response / state check) — "looks done" is not a pass. Keep it tool-agnostic; API-level assertions use `$BASE_URL` / `$SERVER_URL` from `.dev-workflow/ports.env` (never hardcoded ports).

**Record the layer gate.** Write evidence to `docs/layer-gates/<layer>.md` (the generated `e2e-test` skill ships a `layer-gate-evidence` template): the two coverage matrices — **acceptance traceability** (criterion → proving test → Verify → PASS/FAIL) and **scripted-coverage** (case → asserts → PASS/FAIL) — plus the manual checklist, screenshots / API JSON, and any `WAIVER:` lines. Update `layer_gates[N].coverage` and `evidence.{scripted,journeys,matrix}` in `product-context.yaml`. The two-phase flip (`scripted_passed` → `passed`) happens at `/aep-wrap` once all applicable tiers are green and coverage is complete — that is what lets `/aep-dispatch` advance to the next layer.

---

## Phase 8: Review Results

> **Light mode:** skip this phase.

1. Source `.dev-workflow/ports.env` for correct ports.
2. Run the project's framework tests (Tier 1), any applicable Tier-3 API drivers, and **replay the impacted prior-layer journey set** (regression) — selected against the **merged diff**, never the declared `files_affected`: a journey is impacted when its `covers:` criteria belong to this story, when its optional `paths:` front-matter globs intersect the diff, or when it declares no `paths:` at all (fail-open — undeclared journeys stay in the replay set). **Plus, always, the walking-skeleton journey as a canary** — one execution that catches the "the app no longer starts" class immediately. Full prior-layer replay belongs to the `/aep-wrap` layer gate and its mid-layer checkpoints, not to every story — with one exception: a **`deep`-tier story replays the full prior-layer set here**. Replay **this** layer's Tier-2 journey here only when `journey_timing: pre-merge` — under `post-deploy` its execution defers to the `/aep-wrap` gate (the `deployed:<url>` target isn't up pre-merge). Verify everything applicable passes and `coverage.criteria_covered == criteria_total` (or gaps carry a `WAIVER:`).
3. Present (or note in the progress file): the Phase 5 code review, the Phase 6 dogfood report (if run), and the Phase 7 journey result + layer-gate evidence + coverage summary (`criteria_covered / criteria_total`, if run).
4. If tests fail, loop back to the appropriate phase.

**Signal:** set `status.json` `"phase": 8, "phase_name": "review-results"`.

---

## Phase 9: Cleanup & Publish

> Do NOT run `/opsx:archive` here — archive runs on the integration branch after merge (via `/aep-wrap`).

1. **Write the lesson summary.** Before publishing, write a final `## Summary for Next Agent` (1–3 sentences) in `.dev-workflow/lessons.md`: what would you tell the next agent building in this module? Write it even if there are no other entries ("straightforward implementation, no surprises" is useful signal).
2. **Review the commit history:** `git log --oneline "$BASE"..HEAD` (resolve `$BASE` per `/aep-git-ref` "Resolving `$BASE`"). It should be a clean linear sequence — one commit per `tasks.md` task, optionally followed by review-fix commits. Squash-merge at PR time keeps the integration branch clean, so per-commit hygiene is for reviewer readability.
3. **Rebase onto the latest integration branch and push** (`-u` on first push): `git fetch origin && git rebase origin/"$BASE"`, resolve conflicts in the tree then `git add <files> && git rebase --continue` (or `git rebase --abort` if too tangled — surface via the signal file), then `git push -u origin feat/<name>`. Full recipe: `/aep-git-ref` "Publishing & PR Conventions".

---

## Phase 10: Create PR / MR

Open the PR/MR **targeting the integration branch** — auto-detect the host and **always pass the base explicitly**: `gh pr create --base "$BASE"` (GitHub) / `glab mr create --target-branch "$BASE"` (GitLab); resolve `$BASE` per `/aep-git-ref` "Resolving `$BASE`". Host auto-detect + the recipe: `/aep-git-ref` "Publishing & PR Conventions" → "Open the PR". For an unrecognized host, do not silently no-op — open it manually with the base passed explicitly.

> **CRITICAL — always specify `--base "$BASE"` / `--target-branch "$BASE"`.** Workspace sessions run from a `feat/<name>` worktree, not the integration branch. Without an explicit base, `gh pr create` may infer the wrong base from the most recent merged branch — targeting a stale base, or (in two-branch mode) production `main`, which AEP must never merge feature work into directly.

Include in the PR/MR body: summary of changes (from the proposal), test-coverage notes, and a link to the manual test plan (if created).

---

## Phase 11: PR Review & CI Feedback Loop

Monitor for CI and review feedback.

**Triage each comment:** **Fix** — correctness, CI failures, convention violations, security. **Acknowledge but skip** — style preferences, over-engineering, cosmetic. **Discuss** — architectural suggestions that expand scope, conflicting comments.

**Fix loop:** (1) triage all comments; (2) write a fix plan at `.dev-workflow/pr-fix-plan-<round>.md`; (3) reply to skipped/discussed comments; (4) add a follow-up commit per fix (`fix(<scope>): address review feedback on <topic>`); (5) re-run tests; (6) re-push (`git push origin feat/<name>`); (7) repeat until CI is green and reviews are resolved. Commit/push conventions: `/aep-git-ref` "Publishing & PR Conventions".

---

## Phase 11.5: Human Evaluation & Iteration

After PR-review fixes resolve, the human tester evaluates the feature (typically by running the app from the workspace). If they find minor issues (UX tweaks, missing edge cases, behavior mismatches), this phase handles the iteration loop.

> **If no issues found:** skip to Phase 12.

**Iteration round:**

1. **Document findings** in `.dev-workflow/human-eval-round-<N>.md`: what was found (description + repro), severity (minor/moderate), category (UX/logic/edge case/visual).
2. **Add a follow-up commit per fix** (`fix(<scope>): <human-eval finding>`).
3. **Align the OpenSpec change** in `openspec/changes/<name>/`: add completed tasks to `tasks.md`, update `specs/` if behavior changed, update `design.md` only if approach shifted; keep `proposal.md` scope as-is.
4. **Re-test** — re-run Phase 5 (code review) and Phase 6 (dogfood) on the changed areas.
5. **Push** (`git push origin feat/<name>`).
6. **Repeat** if the tester finds more issues.

**Signal:** create `.dev-workflow/signals/ready-for-review.flag` when ready for human evaluation; set `status.json` `"phase": 11.5, "phase_name": "human-evaluation"`. This is a human gate — also follow the Human-Gate Protocol so the orchestrator surfaces it instead of counting you as stuck.

---

## Phase 12: Pre-merge Checks & Merge

1. **Up-to-date with the integration branch** (resolve `$BASE` per `/aep-git-ref` "Resolving `$BASE`"): `git fetch origin && git rebase origin/"$BASE" && git push --force-with-lease origin feat/<name>`.

   > **Why `--force-with-lease` (never `--force`):** rebasing rewrites the feature branch's SHAs; the lease variant pushes only if the remote hasn't advanced since your last fetch, protecting concurrent collaborators.

2. **Read GitHub's own readiness:**

   ```bash
   gh pr view <number> --json mergeStateStatus,mergeable --jq '{state: .mergeStateStatus, mergeable}'
   ```

   - `CLEAN` → mergeable now (required checks satisfied or none; no required review missing) → **proceed** (a CLEAN PR with zero required checks is mergeable — absence of checks is not a reason to wait).
   - **Anything else** (`UNKNOWN`, `UNSTABLE`, `BLOCKED`, `DIRTY`, `BEHIND`) → load `references/merge-decision-cases.md` for the per-state handling before acting.

3. No unresolved review comments.
4. E2E tests passed (if applicable).
5. **Tier re-check on post-eval commits:** if commits landed after the last eval PASS (Phase 11 review fixes, Phase 11.5 human-eval fixes), re-run the tier derivation on the updated diff. A drift into `sensitive_paths` upgrades the tier and requires **one fresh evaluation round at the upgraded tier** before merging — this rides the existing stale-eval rule ("code has changed since your last evaluation"), it is not a new mechanism.
6. Present the final status summary.
7. **Merge decision.** Detection — the `mode` marker is the **SOLE authority; do NOT infer from cwd** (the Phase 0 guard relocates every build, interactive included, into a worktree, so cwd no longer distinguishes autonomous from interactive):

   ```bash
   MODE=$(cat "$(git rev-parse --show-toplevel)/.dev-workflow/signals/mode" 2>/dev/null)  # anchored — a build phase may have cd'd into a subdir
   ```

   - `mode` reads exactly `autopilot` → **autopilot mode**: **merge immediately** when the pre-merge conditions pass — do not wait for confirmation (the orchestrator monitors via signals).
   - anything else (absent, empty, other) → **interactive mode**: ask for confirmation before merging. When the signal is ambiguous, default to interactive — never auto-merge when unsure.

   **"PR ready" is NOT a stop condition.** In autopilot you may stop short of merge **only** on the 6 legitimate conditions; otherwise you MUST merge. Those conditions, the full per-state handling, and the worked cases are canonical in `references/merge-decision-cases.md`. Reporting `mergeStateStatus=CLEAN` and stopping is a bug. **After merging, set `status.json` `story_status: "completed"` — your job ends there. Do NOT run `/aep-wrap` yourself** (wrap runs `/opsx:archive` on the integration branch; running it from this worktree corrupts the concurrency protocol). The story is done only when **merged + wrapped**, but the worker only merges.

Merge:

- GitHub: `gh pr merge <number> --squash --delete-branch`
- GitLab: `glab mr merge <number> --squash --remove-source-branch`

---

## Guardrails

Cross-cutting rules with no single step home:

- **Archive runs on the integration branch via `/aep-wrap`, never from a workspace** — it writes `openspec/specs/` and causes conflicts.
- **Only the main session writes `product-context.yaml`** — workspaces report status through `.dev-workflow/signals/status.json` (the concurrency protocol).
- **Resume:** if returning to an in-progress workflow, run `.dev-workflow/init.sh` if it exists, then read the progress file.
- **Phase skipping:** users may ask to skip phases — update the progress file accordingly.
- **Signals:** update `status.json` at the start and end of every phase, and check `.dev-workflow/signals/feedback.md` for main-session feedback at phase boundaries.

---

## Next Step

After merge, signal the main session to run:

```
/aep-wrap
```
