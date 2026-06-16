# Changelog

All notable changes to this skills plugin are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) on the
`metadata.version` field in [`.claude-plugin/marketplace.json`](.claude-plugin/marketplace.json).

**Maintenance convention:** every change that bumps the `marketplace.json`
version ships with a matching entry here, in the same PR. Add work in progress
under `[Unreleased]` as you go; when cutting a release, rename `[Unreleased]` to
`[X.Y.Z] - YYYY-MM-DD` and tag `vX.Y.Z` on merge to `main`. SemVer guide for this
repo: new skills / new backends / additive capability → **minor**; recipe and
bug fixes → **patch**; removing or breaking a skill contract → **major**.

> This is **release history for the plugin**. It is unrelated to the per-project
> `changelog:` array inside a downstream `product-context.yaml` (written by
> `/envision`, `/dispatch`, `/reflect`, …), which records product-state history
> for that project. See [`docs/glossary.md`](docs/glossary.md).

## [Unreleased]

_Nothing yet._

## [2.1.0] - 2026-06-16

Add an **object-first design stage** — `/aep-model` — that turns the verb-first
story map into a noun-first **Object Map** (OOUX/ORCA) before UI is built, so build
agents stop inventing one-step-one-screen task-wizard UIs. The structural UI plan
(objects, attributes, relationships, CTAs, screens) is auto-drafted from artifacts
AEP already produces, human-approved at a short gate, then governs build — leaving
only taste (look/voice/journey) to `/aep-calibrate`. Background and the verb-first
vs noun-first analysis: [`docs/research/ooux-object-modeling.md`](docs/research/ooux-object-modeling.md).

### Added

- **`/aep-model` skill** (`product-context`): runs ORCA (Objects → Relationships →
  Calls-to-action → Attributes → screens) to draft an Object Map, takes a short
  human review gate (object boundaries, primary anchor, task-flow exceptions), and
  writes the approved noun-first blueprint. Sits between `/aep-map` and
  `/aep-dispatch` for UI-facing products. Registered in
  [`marketplace.json`](.claude-plugin/marketplace.json) `product-context` plugin.
- **Object Map artifacts + schemas**: `product/object-model.yaml` (cross-capability
  object ontology) and `product/maps/<capability>/object-map.yaml` (capability-scoped
  ORCA/IA projection), via `_shared/templates/object-model-schema.yaml` and
  `object-map-schema.yaml`. Object-first is the default; task-oriented flows are an
  opt-in escape hatch recorded with a reason.
- **ORCA reference** (`product-context/model/references/orca-process.md`):
  round-by-round derivation from AEP inputs + the object-first/task-oriented decision
  framework and the completeness checks.
- **Glossary terms**: Object Model, Object Map, ORCA, Call-to-Action (CTA), Nested
  Object Matrix, Object-First vs Task-Oriented.
- **Research note** `docs/research/ooux-object-modeling.md` and a `Research` category
  in [`docs/README.md`](docs/README.md).

### Changed

- **Schema** (`product-context-schema.yaml`): adds `stories[].object_model_refs`,
  `stories[].capability`, `architecture.modules[].kind`, and the `object-model`
  quality dimension. Object-map approvals are tracked as thin `calibration.history`
  references — the artifact bodies stay under `product/`, not inlined into
  `product-context.yaml`.
- **`/aep-envision`**: declares the `object-model` structural gate by default for
  UI-facing products.
- **`/aep-map`**: auto-drafts Object Maps after decomposition; sets `module.kind` +
  `story.capability`; flips an approved map to `stale` on re-decompose; routes Next
  Step to `/aep-model`.
- **`/aep-dispatch`**: injects the minimal Object Map slice into a story's context
  package and refuses UI-facing stories without an approved (non-stale) map.
- **`/aep-launch`**: aborts a UI-facing story when no approved Object Map covers it.
- **`/aep-build`**: UI implementation obeys the injected Object Map slice (object structure and CTA grammar; taste still from calibration).
- **`/aep-validate`**: Mode A gains Object Map completeness checks (coverage, object
  homes, anchors, task-flow justification, ref resolution).

All additions are backward-compatible — the object-model path only engages for
UI-facing products that opt in.

## [2.0.1] - 2026-06-16

Operationalize the **dogfood → reflect classifier → story** link so the G6
self-feeding loop fires for **every** dogfood trigger — not just the autopilot
post-merge guard. Previously "feed the report to the `/aep-reflect` classifier"
was prose-only: no adapter parsed the unified markdown report into the classifier's
normalized record, so a standalone / ad-hoc dogfood (and even the guard path) left
findings on disk with no auto-filing. A dogfood that surfaced real bugs would stop
and wait for a human to hand-author stories.

### Added

- **`dogfood_report` source adapter** (`product-context/_shared/references/telemetry-ingestion.md`):
  parses each `##` finding in `.dev-workflow/dogfood-*.md` (unified
  severity/category/repro format) into the normalized observation record the
  `/aep-reflect` Step 2 classifier consumes. Maps Severity → priority, Category →
  `suggested_class` hint, and assigns a deterministic
  `external_id = dogfood:<report>:<hash>` so re-running the same dogfood never
  duplicates stories. Self-describing file glob — not gated by `coverage_check`.
- **`dogfood_report` watch source** (`/aep-watch`): a standalone, local, or
  post-deploy dogfood report is now ingested headlessly — classified, deduped, and
  (under `full_auto` / `watch.auto_create`) auto-filed as a bug/refinement story
  that autopilot dispatches on its next tick. Calibration / discovery /
  opportunity-shift / process findings still surface to a human.

### Changed

- **`dogfood-validation.md`** "On issue" — replaced the prose assertion with the
  concrete adapter + watch ingestion path, and made the report-file path an
  explicit contract: findings left only in chat are a dead end.
- **`/aep-reflect` Step 1** — adds `.dev-workflow/dogfood-*.md` to the gathered
  feedback sources (was omitted), normalized via the same adapter `/aep-watch` uses.
- **`/aep-build` Phase 6** — write the dogfood report file even when findings are
  clean, since that path is the ingestion contract.

## [2.0.0] - 2026-06-16

The **autonomy loop** release. Closes the loop-engineering gaps identified in
`docs/research/loop-engineering-autonomy-gap.md` (G2–G7) and adds a `full_auto`
master switch. Every new capability defaults to **human-in-the-loop** — autonomy
is opt-in via `topology.routing` flags.

### Added

- **`/aep-watch` skill** (G6) — continuously ingests telemetry / error streams /
  bug trackers, classifies findings with the `/aep-reflect` classifier, and
  auto-files bug/refinement stories so reflect→dispatch becomes self-feeding.
- **Change-strategy recovery ladder** (G2) — `gen-eval/references/recovery-ladder.md`;
  on repeated eval FAIL the build climbs same-fix → re-ground → fresh
  `native-bg-subagent` generator → decompose **before** the `eval_not_converging`
  human gate.
- **Host-aware post-deploy dogfood** (G4b) — `executor/references/dogfood-validation.md`:
  `dogfood_method()` (Claude → agent-browser; Codex → native in-app browser /
  computer-use, or Playwright headless) + `target_url()` (config-first, CI fallback).
- **Post-merge guard** (G4a) — `autopilot/references/post-merge-guard.md` + tick
  Step ③.5: monitors merged stories' deploy health; dogfood issues → reflect story;
  hard regression → conservative `auto_revert` (default off, warn + escalate).
- **Telemetry-driven reflect** (G5) — `reflect/references/telemetry-ingestion.md`:
  automated source ingestion + quantitative outcome-contract auto-evaluation.
- **Telemetry source determination** — projects decide sources via a hybrid
  metric-driven rule: `/aep-scaffold`/`/aep-onboard` detect the observability stack
  (candidate sources); `/aep-map` binds each quantitative `success_metric` +
  `health_signal` to a source (`metric_map`); a shared `coverage_check()` lets
  `/aep-watch`, `/aep-reflect`, and the post-merge guard **block auto when the
  binding is incomplete** instead of silently no-op'ing.
- **Visual Design evaluator dimension** (G3) — vision-model scoring of screenshots
  against the design system, for both Claude and Codex (multimodal).
- **`full_auto` master switch** (A1) — `topology.routing.full_auto` (default false)
  gates the strategic human pauses (design escalation, qualitative outcome eval);
  implies `auto_design` + `auto_outcome_eval` + `watch.auto_create`. New config keys
  added to the product-context schema.

### Changed

- `/aep-build` Phase 5 climbs the recovery ladder; Phase 6 dogfood is host-aware
  (degrades instead of skipping when agent-browser is absent).
- `/aep-reflect` Step 1 supports automated ingestion; Step 2.75 auto-evaluates
  quantitative outcome contracts (qualitative still pauses unless `full_auto`).
- `/aep-autopilot` gains the post-merge guard step and `full_auto`-aware routing;
  loop hygiene unified on `--max-turns` (G7).

### Fixed

- Carries forward the v1.8.0 executor fix (claude-team removed; `native-bg-subagent`
  default + post-spawn liveness probe). Every new spawn path uses it.

## [1.8.0] - 2026-06-15

### Changed

- **Executor: removed the `claude-team` backend.** Its agent-teams spawn path
  fails silently on Claude Code ≥ 2.1.x — the launch command is truncated in a
  detached `claude-swarm` tmux pane and never submitted, so no worker starts,
  yet the team roster still reports the member "active". `native-bg-subagent`
  replaces it as the Claude Code default. The agent-teams env flag is no longer
  consulted and there is no "…with agent team" opt-in. See
  [`docs/decisions/remove-claude-team.md`](docs/decisions/remove-claude-team.md).
- Orphan/stuck detection now decides liveness by the **real-liveness probe**
  (process/agent exists AND worktree shows activity), never by roster/state
  membership.

### Added

- **`native-bg-subagent` executor backend** (Claude Code default): Agent tool
  with `run_in_background: true`, no team. Success signature is a bare-hex
  `agentId` + JSONL `output_file`; steered via `SendMessage(to: agentId)` +
  `feedback.md`; human gate is gate-and-park.
- **Mandatory post-spawn liveness probe** with auto-fall-back to
  `native-bg-subagent` on failure
  (`skills/patterns/executor/scripts/spawn-liveness-probe.sh`).
- Explicit **one launch = one worktree = one subagent = one story** invariant in
  `/aep-launch` (the `compile_mode: grouped_change` story group is the one
  documented exception).

### Fixed

- `claude-bg` is now gated on `BG_AVAILABLE`; documented that the `claude --bg`
  one-shot spawn flag was removed on Claude Code ≥ 2.1.x (so claude-bg is skipped
  there and native-bg-subagent is used).

## [1.7.0] - 2026-06-11

**Goal-driven autopilot driver**: `/aep-autopilot` now keeps itself ticking with
a **goal driver** by default — the host-native `/goal` primitive (Claude Code
v2.1.139+ and Codex's experimental `goals` feature) — which re-fires a tick when
there is work and **self-terminates** when the current layer is complete or a
human-judgment gate is hit. The fixed-interval `/loop` driver is retained as a
fallback (`--loop`). Only the _driver_ changes; the 7-step CHECK→ACT tick, the
delegated cheap CHECK, the signals protocol, and the orchestrator boundary are
unchanged. Decision record:
[`docs/decisions/goal-driven-autopilot.md`](docs/decisions/goal-driven-autopilot.md).

### Added

- **Goal driver (default)** — `/aep-autopilot` with no `--loop` flag builds a
  one-layer goal condition and drives it via `/goal`: "layer N complete (all
  stories merged + wrapped) OR autopilot paused". Scoped to **one layer per run**
  — it stops at the layer boundary so the human runs the layer gate / `/aep-reflect`
  and re-invokes for the next layer. Native and near-symmetric on both hosts
  (Claude Code Haiku-evaluator Stop hook; Codex persisted thread goal with
  `token_budget`).
- **Per-tick surface + wait tail (step ⑦, goal driver only)** — each tick
  surfaces a **signals-only** `AUTOPILOT …` status line for the goal evaluator to
  judge (boundary-safe: never workspace code), then waits a bounded **floor**
  (default `5m`, `--floor`) before ending the turn. The floor is the anti-hot-loop
  mechanism — CC uses `Monitor` with a hard timeout (a raw foreground `sleep` is
  blocked in a turn); Codex uses shell `sleep`.
- **Goal-driver flags** — `--floor <dur>` (per-tick wait floor) and
  `--max-turns <n>` (runaway backstop, default `200`); on Codex a `token_budget`
  is set as the hard wall (soft-stops to `budget_limited`).

### Changed

- **`/aep-autopilot` default behavior** — the default driver is now goal-driven
  and self-terminating per layer. The command surface is unchanged; `--loop
<interval>` selects the prior fixed-interval behavior exactly.
- **`/aep-autopilot stop`** — cancels whichever driver is active (`/goal clear`
  for the goal driver; `/loop` cancel or cron/launchd removal for the loop
  driver).
- **Driver × backend compatibility** (executor `backends.md`) — the long-lived
  session class now names two in-session variants, `/goal` (default) and `/loop`;
  the goal driver is in-session-only, so the cron/launchd row stays the
  `/loop` / `codex exec` path.

## [1.6.0] - 2026-06-10

**Native-first executor backends**: launch/dispatch/build/autopilot/wrap now
target each host's native parallel-agent machinery instead of tmux+cmux. The
B1–B4 ladder is replaced by named launch modes; every mode still runs its
worker in the AEP-created worktree at `.feature-workspaces/<ws>` with the
file-based signals protocol as the source of truth. Decision record:
[`docs/decisions/native-first-executor.md`](docs/decisions/native-first-executor.md).

### Added

- **`claude-team` mode** — Claude Code agent teams: one teammate per story in a
  **standing team** (TeamCreate once; spawn/shutdown teammates per tick), push
  steering via `SendMessage`, native display via `teammateMode`. Requires the
  experimental `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` flag (project
  `.claude/settings.json` `env` block; `/aep-onboard` documents it).
- **`claude-bg` mode** — Claude Code native background sessions
  (`claude --bg` / `attach` / `logs` / `stop` / `respawn`): the GA fallback;
  OS-bound, cron-driver compatible, worktree-enforced by process cwd.
- **`codex-subagent` mode** — Codex multi_agent workers (desktop app + CLI)
  with shipped `aep-builder` / `aep-evaluator` roles (`.codex/agents/*.toml`),
  `send_input` steering, and the native approval overlay as a human gate.
- **`codex-exec` mode** — headless `codex exec --cd <worktree>` workers,
  steerable across sessions via `codex exec resume <id>`; the Codex mode for
  cron-driven autopilot and hard cwd isolation.
- **Human-gate protocol (hub-and-spoke)** — `.dev-workflow/signals/needs-human.md`
  - `status.json` `blocked_on: "human"`: host-agnostic record of decisions only
    the human can make. The **main agent is the canonical human console**: the
    question flows back to the orchestrator, the human answers there, and the
    answer is relayed per mode — by push on steerable modes
    (**block-in-place**), or by resuming a parked worker into its worktree on
    batch/pull modes (**gate-and-park**: the worker commits WIP, records the
    gate, and ends its run cleanly). Direct worker surfaces (teammate pane,
    `claude attach`, Codex thread, `tmux attach`) are optional conveniences.
    Gated workspaces count as waiting, not stuck.
- **Workflow mode is now a complete backend** — gate-and-park gives the
  dynamic-workflow fan-out a human-gate path: build agents return a structured
  `gated` result instead of guessing or stalling; the main agent collects
  gated stories after the run, asks the human, and resumes them into their
  existing worktrees (continuation via `resumeFromRunId` or re-launch). New
  "Mode: workflow" recipe in `backends.md`; the `/aep-dispatch … with
workflow` path now creates the `.feature-workspaces/<ws>` worktrees and
  announces gate behavior instead of "no mid-flight feedback".
- **Orphan re-adoption** — lead restarts no longer strand session-bound
  workers: autopilot re-spawns into the existing worktree with the recovery
  bootstrap instead of failing the story.
- New executor references: `claude-native.md`, `codex-native.md`,
  `tmux-session.md`; new `gate()` operation; driver × backend compatibility
  matrix (session-bound vs OS-bound worker lifetimes).
- Autopilot state: `backend`, `agent_id`, `last_liveness_hash` (generalizes
  `last_tmux_hash`), `human_gate` escalation type, `readopted` action.

### Changed

- **Behavior change:** Claude Code with tmux installed no longer auto-selects
  tmux — native modes win. Pin the old workflow with
  `git config aep.executor-backend tmux` (or "…with tmux"). tmux+cmux recipes
  are preserved verbatim as the **`legacy`** mode (generic-host fallback).
- Autopilot accepts any steerable, driver-compatible mode (was: B1/B2 only);
  Codex autopilot is now supported (in-thread ticks → codex-subagent; scheduled
  `codex exec` ticks → codex-exec).
- `/aep-build` Phase 5 evaluator spawn is mode-dispatched: foreground Task
  subagent (Claude native modes) or `codex exec --cd` with the aep-evaluator
  role (Codex modes); the evaluator prompt is delivered at spawn time — no
  sleep/poll/kill-pane on native modes.
- `/aep-wrap` teardown is mode-dispatched (teammate shutdown / `claude stop` /
  `close_agent` / no-op / `tmux kill-session`).
- **Command naming normalized to `/aep-*`** across all skills, README, and
  docs: every AEP skill is invoked by its canonical registered name
  (`/aep-autopilot`, `/aep-dispatch`, `/aep-launch`, …) — the unprefixed forms
  (`/autopilot`, `/launch`) no longer appear, eliminating the prefix confusion.
  Non-AEP commands (`/loop`, `/opsx:*`, Codex `/agent`, `/workflows`) are
  unchanged. (Changelog entries v1.5.0 and older keep their original wording
  as historical record.)

## [1.5.0] - 2026-06-09

Configurable **integration branch**: AEP no longer hardcodes `main`. The branch
that feature work is based off, rebased onto, PR'd into, and where control-plane
commits land is now resolved per-repo, with `main` as the default — so GitFlow
repos (`develop` = integration/staging, `main` = production) work out of the box.

### Added

- **Integration-branch resolution (`$BASE`).** Every git-touching skill resolves
  the integration branch in this order: explicit override
  `git config aep.integration-branch <name>` → auto-detect `develop` if it exists
  → `main`. `git config` is repo-local and shared across worktrees, so `$BASE`
  resolves identically in the main session and inside `.feature-workspaces/`.
- **Two modes, auto-detected.** _single-branch_ (no `develop`): integration =
  production = `main` (unchanged default behavior). _two-branch_ (`develop`
  exists): integration = `develop`; production `main` is promote-only and AEP
  never touches it — promotion `develop` → `main` stays the user's CI/CD or PR
  step, exactly like deployment.
- `aep-git-ref` gains an "Integration Branch" section documenting `$BASE`, the
  modes, the resolver, and the `aep.integration-branch` config key.
- `/onboard` Phase 5 detects the mode and persists `aep.integration-branch`;
  `/scaffold` sets it to `main` for fresh repos.

### Changed

- `/design`, `/launch`, `/build`, `/wrap`, `/dispatch`, `/reflect`, `/calibrate`,
  `/autopilot` (+ tick-protocol), and the executor backend recipe now substitute
  `$BASE`/`origin/$BASE` for the previously hardcoded `main`/`origin/main` in
  worktree creation, rebases, PR base (`--base "$BASE"` / `--target-branch`),
  and control-plane commits. Default-mode behavior is unchanged (`$BASE` = `main`).
- `.claude-plugin/marketplace.json` version `1.4.0` → `1.5.0`.

## [1.4.0] - 2026-06-05

Codex `/launch` now uses Codex-native, worktree-bound subagents for coding
launches by default, while Claude/generic executors keep the tmux session path.

### Changed

- `aep-executor` backend selection now chooses B3 for Codex hosts before checking
  cmux/tmux, so Codex coding launches no longer create tmux sessions just because
  tmux is installed.
- The B3 spawn recipe still creates the standard AEP worktree first
  (`.feature-workspaces/<name>` on `feat/<name>`), then starts the Codex
  worker/subagent with an explicit "operate only in this worktree" contract.
- `/launch`, `/dispatch`, `/build`, and `/autopilot` docs now distinguish Codex
  subagent launches from steerable tmux sessions. Autopilot remains B1/B2-only
  because it requires live `nudge()`/`liveness()`.
- `.claude-plugin/marketplace.json` version `1.3.2` → `1.4.0`.

## [1.3.2] - 2026-06-05

Fixes how `/launch` and the executor attach a cmux review surface (reported and
verified from a downstream session).

### Fixed

- **cmux detection no longer requires `$CMUX_SOCKET`.** A host where the `cmux` CLI
  is reachable but `$CMUX_SOCKET` is unset (Claude Code inside a cmux-managed tmux
  session) wrongly fell back to headless B2. Detection now keys on "cmux CLI
  reachable **and** a target pane resolves" (`cmux tree` `◀ here` / `$CMUX_PANE_ID`).
- **The review tab opens as a sibling of the orchestrator's own tab.** The old bare
  `cmux new-surface` defaulted to an unset `$CMUX_WORKSPACE_ID`; `/launch` now
  resolves the orchestrator's pane from `cmux tree` and opens the tab with
  `new-surface --workspace <ws> --pane <pane> --focus true`. `cmux new-workspace`
  (a separate top-level workspace) is kept out of the launch path.
- **Bootstrap is sent before the cmux surface attaches.** An attached cmux surface
  focuses the tmux composer and blocks external `send-keys`, so the bootstrap
  couldn't land. Reordered to: spawn tmux → wait ready → send bootstrap via
  `tmux send-keys` → then attach the cmux tab.
- `.claude-plugin/marketplace.json` version `1.3.1` → `1.3.2`.

## [1.3.1] - 2026-06-05

Onboarding refinements: OpenSpec is now a required install step, and the optional
memory supplement is wired as a thin layer over AEP's native lessons loop.

### Changed

- `/onboard` and the README "Agent prompt" now install the **OpenSpec CLI** as a
  REQUIRED step (`npm install -g @fission-ai/openspec@latest`, Node >= 20.19). AEP's
  skills shell out to `openspec`, but the delegated install never ensured it.
- The optional **memory supplement** (`project-memory` + `memory-forge`) now layers onto
  AEP's existing lessons loop instead of a parallel one. AGENTS.md gets a concise
  `## Memory & Learning Loop` section mapping recall → `/dispatch`, persist → `/wrap`, and
  distill → `/reflect` / pre-PR. AEP still captures via `/build` → `.dev-workflow/lessons.md`
  → `/wrap` → `lessons-learned/`; the supplement adds qmd semantic recall plus
  distillation-to-skills on top.
- `.claude-plugin/marketplace.json` version `1.3.0` → `1.3.1`.

### Fixed

- Corrected the OpenSpec install command in the `/onboard` and `/scaffold` tool tables:
  both listed `bun add -g openspec`, which installs the wrong npm package (OpenSpec's CLI
  is published as `@fission-ai/openspec`).

## [1.3.0] - 2026-06-04

Adds an optional supplement and modernizes onboarding to the `npx skills` installer.

### Added

- Optional **supplement** pointer to [`memorysaver/skills`](https://github.com/memorysaver/skills)
  in the README and `/onboard`: `project-behavior` (scaffold an `AGENTS.md` behavior pack),
  `project-memory` (git-committable memory system), and `memory-forge` (distill lessons into
  skills). The `/onboard` and README "Agent prompt" flows now **always ask** whether the user
  wants the `AGENTS.md` behavior config and the memory system before installing either.

### Changed

- `/onboard` (aep-onboard) Phase 1 install modernized from the legacy
  `/plugin marketplace add` and `/plugin install` commands to project-level
  `npx skills add memorysaver/agentic-engineering-patterns@<latest-tag>` (pin to the latest
  release, commit the installed files), matching the README install guide.
- `.claude-plugin/marketplace.json` version `1.2.0` → `1.3.0`.

## [1.2.0] - 2026-06-04

Host-agnostic executor: the feature lifecycle now runs under Claude Code **or**
Codex, with or without tmux/cmux, and (on explicit opt-in) as a Claude Code
dynamic workflow. `B1` (tmux + cmux) reproduces the prior behavior byte-for-byte,
so existing installs are unaffected. Decision record:
[`docs/decisions/host-agnostic-executor.md`](docs/decisions/host-agnostic-executor.md).

### Added

- New `aep-executor` utility skill (in the `patterns` group) with a uniform
  operation contract (`detect` / `spawn` / `spawn_evaluator` / `nudge` /
  `liveness` / `check` / `monitor` / `present` / `teardown`) and a recipe per
  backend.
- `executor.check(prompt, schema)` — run a read-only analysis prompt in a
  **cheap, context-isolated** agent (Claude Code Haiku subagent / Codex
  `codex exec` cheap one-shot) and return only its compact JSON result.
- Four execution backends, selected automatically from env markers
  (`$CLAUDECODE`, `$CODEX_*`, `$CMUX_SOCKET`, `$TMUX`) plus `command -v`:
  **B1** session in tmux + cmux tab, **B2** session in tmux (no cmux),
  **B3** native subagent (Desktop / no-tmux fallback), **B4** dynamic-workflow
  fan-out (explicit "…with workflow" opt-in, Claude Code only).
- Codex as a first-class executor alongside Claude Code.

### Changed

- `/launch`, `/build` (Phase 5 evaluator), `/autopilot`, `/dispatch`, `/onboard`,
  and `/wrap` now delegate spawning/steering/teardown to the executor abstraction.
- `/autopilot` ticks now split into **CHECK → ACT**: the read-heavy analysis
  (state, workspace signals, PR status) and the state write run in a cheap delegate
  via `executor.check()`, so the long-lived orchestrator session's context/token
  cost stays low; the orchestrator only executes the few emitted actions (nudge,
  wrap, launch, escalate). The default tick interval stays **5m**.
- Periodic driver is now documented per host: Claude Code `/loop` (in-session);
  Codex has no `/loop`, so schedule `codex exec "/autopilot tick"` via launchd / cron
  / a sleep-loop (each tick already runs isolated + cheap).
- `cmux` is now optional (auto-detected) rather than a hard dependency; `tmux` is
  recommended but no longer an abort-on-missing requirement in `/onboard`.
- `.claude-plugin/marketplace.json` version `1.1.0` → `1.2.0`.

### Fixed

- Removed the bogus `--rc` flag (not a real Claude Code parameter).
- Corrected the per-host CLI invocations, verified against Claude Code 2.1.161 and
  Codex 0.130.0: session backends use the interactive command
  (`claude --dangerously-skip-permissions` / `codex --dangerously-bypass-approvals-and-sandbox`),
  headless paths use `claude -p …` / `codex exec …`. Codex sessions no longer
  (incorrectly) used the non-interactive `codex exec`.
- Readiness probe is executor-aware (`❯` for claude, timed wait for codex).
- Multi-line prompts/nudges sent via `tmux send-keys -l` + a separate `Enter`
  (was submitting line-by-line).
- The gen/eval evaluator is always worktree-bound (eval-protocol Context labels
  name the spawn mechanism, not a read-only or CI use case).
- `/wrap` now kills the tmux session before removing the worktree, fixing an
  orphaned-session leak across autopilot runs.

## [1.1.0] - 2026-06-03

### Added

- Installable via the open [Agent Skills](https://agentskills.io/) format —
  `npx skills add memorysaver/agentic-engineering-patterns -a claude-code --skill '*'`.
- `aep-` prefix on all skill names for namespace isolation.
- Shared-resource materialization (`skills/product-context/_shared/` +
  `scripts/build-skills.sh`) keeping each skill self-contained, with a
  `skills:check` drift guard wired into the pre-commit hook.

### Changed

- README install docs refocused on `npx skills add` (maintainer `sync.sh` /
  `sync-downstream.sh` retained for the downstream-projects workflow).
- `.claude-plugin/marketplace.json` version `1.0.0` → `1.1.0`.

## [1.0.0] - 2026-04-29

First stable baseline after the Jujutsu → git migration. Decision record:
[`docs/decisions/migrate-from-jj-to-git.md`](docs/decisions/migrate-from-jj-to-git.md).

### Added

- Pure git + `git worktree` execution model (`.feature-workspaces/<name>` on a
  `feat/<name>` branch per parallel agent).
- Idempotent `/launch` pre-launch orphan-cleanup (orphan branch + worktree
  registration).
- `scripts/smoke-test-worktree.sh` for reproducible worktree-mechanics checks.
- `/git-ref` skill documenting the AEP git + worktree conventions and recovery.

### Changed

- Linear commit-per-task implementation: `tasks.md` rows map 1:1 to commits,
  squash-merged at PR time.

### Removed

- Jujutsu (one-shot migration, no dual-mode period) and the `/jj-ref` skill.

[Unreleased]: https://github.com/memorysaver/agentic-engineering-patterns/compare/v1.5.0...HEAD
[1.5.0]: https://github.com/memorysaver/agentic-engineering-patterns/compare/v1.4.0...v1.5.0
[1.4.0]: https://github.com/memorysaver/agentic-engineering-patterns/compare/v1.3.2...v1.4.0
[1.3.2]: https://github.com/memorysaver/agentic-engineering-patterns/compare/v1.3.1...v1.3.2
[1.3.1]: https://github.com/memorysaver/agentic-engineering-patterns/compare/v1.3.0...v1.3.1
[1.3.0]: https://github.com/memorysaver/agentic-engineering-patterns/compare/v1.2.0...v1.3.0
[1.2.0]: https://github.com/memorysaver/agentic-engineering-patterns/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/memorysaver/agentic-engineering-patterns/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/memorysaver/agentic-engineering-patterns/releases/tag/v1.0.0
