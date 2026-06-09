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
