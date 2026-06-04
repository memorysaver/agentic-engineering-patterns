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

[Unreleased]: https://github.com/memorysaver/agentic-engineering-patterns/compare/v1.3.0...HEAD
[1.3.0]: https://github.com/memorysaver/agentic-engineering-patterns/compare/v1.2.0...v1.3.0
[1.2.0]: https://github.com/memorysaver/agentic-engineering-patterns/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/memorysaver/agentic-engineering-patterns/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/memorysaver/agentic-engineering-patterns/releases/tag/v1.0.0
