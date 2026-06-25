# The three-tier E2E model

The generated `e2e-test` skill organizes testing into three tiers. Pick the tier that matches the
question being asked — they prove different things and must not be mixed.

| Tier                   | Proves                                 | Where                                                 |
| ---------------------- | -------------------------------------- | ----------------------------------------------------- |
| **1. Scripted gates**  | the MACHINERY (deterministic, CI-able) | the project's test framework (vitest/pytest/…)        |
| **2. Journey dogfood** | the PRODUCT (real model/providers/env) | `skills/e2e-test/journeys/` (BDD user-story journeys) |
| **3. API drivers**     | backend state, bulk setup, async flows | throwaway scripts in `.dev-workflow/` (gitignored)    |

> A green scripted gate is **not** proof the live product works — every layer gate has a manual journey
> half. Conversely, never debug machinery against a live environment — write a scripted test.

## Tier 1 — Scripted gates (project framework)

Owned by the project's own test runner (vitest, jest, pytest, cargo test, go test). Deterministic,
co-located with source, run during `/aep-build` Phase 4. The plugin doesn't teach unit/integration
testing — your framework's docs do. The e2e-test skill doesn't manage these; it only points at them.

## Tier 2 — Journey dogfood (declarative BDD, agent-executed)

The dogfood plan is the **journey library** ([`bdd-journeys.md`](bdd-journeys.md)) — natural-language
Given/When/Then/Verify scenarios, one per capability area, covering the shipped surface layer by layer.
The executing agent reads intent and drives the UI with the tool resolved by `tool-selection.md`, then
verifies state via the API. This is the manual half of each **layer gate**.

## Tier 3 — API drivers

For backend verification and things browsers are slow at (async/streamed turns, bulk fixture creation,
state-tree diffs). Write `node .dev-workflow/dogfood-<feature>.mjs` (or the project-language equivalent):
sign in → call the API → assert. These are **gitignored throwaways** — reusable patterns belong in the
generated `SKILL.md` / this guide, not in committed scripts.

## How the tiers map to `/aep-build` phases

Features start with zero tests and accrue coverage through the build phases:

```
Phase 4 (implement) → Tier 1: run the project's unit/integration tests (framework-level)
Phase 5 (review)    → Tier 3: API contract checks via an API driver
Phase 6 (dogfood)   → Tier 2: run/extend the layer's journey with the resolved tool; find gaps
Phase 7 (e2e)       → Tier 2: codify findings as journey scenarios + Verify lines
Phase 8 (review)    → Tiers 1-3: run framework tests + replay the journey before merge
```

Each tier catches a different failure mode:

- **Scripted gates** — logic errors, edge cases, data transforms, contract breaks.
- **Journey dogfood** — integration failures, UX regressions, flow breaks, "works but feels wrong".
- **API drivers** — backend state divergence, async/eventual-consistency bugs.

## Which tiers apply — and which gate a layer

Not every project needs all tiers. **The tiers that _apply_ to a project are exactly the tiers that must
be green for its layer gate to reach `passed`** — so "applicable" and "gating" are the same set. Tier 1
is always the project framework's job; this table covers what the **e2e-test skill** should include and
what a gate's `passed` requires:

| Project type               | Tier 1 scripted | Tier 2 journeys        | Tier 3 API drivers | Gate `passed` needs        |
| -------------------------- | --------------- | ---------------------- | ------------------ | -------------------------- |
| Full-stack web app         | Yes             | Yes                    | Yes                | all three green + coverage |
| API-only service           | Yes             | Skip (or thin)         | Yes                | T1 + T3 green + coverage   |
| CLI tool                   | Yes             | Skip                   | Skip               | T1 green + coverage        |
| Static site / landing page | Yes             | Yes (UI only)          | Skip               | T1 + T2 green + coverage   |
| Library / package          | Yes             | Skip                   | Skip               | T1 green + coverage        |
| Mobile app (API backend)   | Yes             | Yes (`target: mobile`) | Yes                | all three green + coverage |

"+ coverage" means: every acceptance criterion in the layer maps to ≥1 proving test across the applicable
tiers (`coverage.criteria_covered == criteria_total`, deliberate gaps recorded as `WAIVER:`). A
CLI/library layer with no journey still gets a meaningful gate — its `passed` is Tier-1 green **plus**
every criterion proven by a scripted case.

**The per-project choice is recorded in the generated skill's `policy.md`** (`applicable_tiers`,
`dogfood_target` = `none`/`local`/`deployed:<url>`, `journey_timing`) — confirmed with the user at
scaffold time, then read by `/aep-build` and `/aep-wrap`. That's the single source of truth for "which
tiers gate _this_ project", so a CLI tool is never asked for a Cloudflare/UI check it doesn't need, and a
pre-release web app can dogfood post-deploy against prod. No copy lives in `AGENTS.md` — the skill is
canonical cross-tool, so every runtime reads the same `policy.md`.

## The two-phase gate (coverage, not one green test)

A layer gate flips through two states, never on a single passing journey:

- **`scripted_passed`** — the layer's Tier-1 suite is green. Machinery proven; live product not yet. This
  does **not** unblock the next layer.
- **`passed`** — `scripted_passed` **+** every applicable higher tier green **+** coverage complete **+**
  prior-layer journeys replay green.

When `/aep-build` Phase 6 finds an **uncovered** acceptance criterion it **auto-authors the missing test**
to close the gap (a Tier-2 scenario by default; a Tier-1 case where deterministic; a Tier-3 API check for
backend/async state), then re-runs — looping until covered or a `WAIVER:` is recorded. `/aep-wrap`
performs the two-phase flip and then asks the human before advancing. The full state machine is in
[`layer-gate-loop.md`](layer-gate-loop.md).

## Graceful degradation

When the resolved automation tool is unavailable on a machine, journey steps **degrade** (skip the UI
step, mark SKIP, fall back to API checks) rather than FAIL. Tool resolution and degrade paths are owned
by `tool-selection.md` in the generated skill. Keep API/framework tiers running regardless.

## CI integration

Tier-3 driver scripts and any committed gate scripts are CI-ready: auto-source `.dev-workflow/ports.env`
with fallback defaults, handle missing tools gracefully (SKIP not FAIL), exit 1 on any FAIL. Journeys
(Tier 2) are agent-executed and run in the dogfood phase, **not blocking CI** unless wired explicitly.

> **Migration note (BDD journeys ≠ CI gate).** This skill no longer generates per-feature
> `<feature>-e2e.sh` bash scripts. A repo whose CI globbed `.claude/skills/e2e-test/scripts/*-e2e.sh`
> will now loop over nothing and pass vacuously. If you need a **CI-blocking** E2E check, keep it in
> **Tier 1** (the project's framework tests) or write a **Tier 3** API-driver script with an exit code —
> the agent-executed journey is the manual layer-gate half, not a pipeline gate.

## Evaluator integration

In full mode (`/aep-launch` with an evaluator), the evaluator reads
`.dev-workflow/feature-verification.json`; each verification step should map to a journey `Verify` line
or an API-driver assertion, so the same checks that gate the build also gate the layer.
