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

## When to skip a tier

Not every project needs all tiers. Tier 1 is always the project framework's job; this table covers what
the **e2e-test skill** should include:

| Project type               | Tier 2 journeys        | Tier 3 API drivers |
| -------------------------- | ---------------------- | ------------------ |
| Full-stack web app         | Yes                    | Yes                |
| API-only service           | Skip (or thin)         | Yes                |
| CLI tool                   | Skip                   | Skip               |
| Static site / landing page | Yes (UI only)          | Skip               |
| Library / package          | Skip                   | Skip               |
| Mobile app (API backend)   | Yes (`target: mobile`) | Yes                |

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
