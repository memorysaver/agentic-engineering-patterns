# BDD Journeys — the natural-language test plan

A **journey** is a declarative, BDD-style user-story scenario set. It describes **intent** (Given / When
/ Then / **Verify**), **not** a click script — the executing agent translates each step into the right
tool calls (resolved by `tool-selection.md` in the generated skill). This keeps the plan
tool-agnostic and close to product requirements.

## Why intent, not a recipe

- **Tool-agnostic** — the same journey runs under agent-browser, Playwright, webwright, Codex's browser,
  or agent-device. Only the executor's command syntax differs.
- **Durable** — UI selectors churn; intent ("the user signs in and lands on the dashboard") doesn't.
- **Reviewable as requirements** — a journey reads like an acceptance criterion, so product and QA can
  read it without knowing the test harness.

## Journey doc template

The canonical, copy-paste journey template ships **inside the generated skill** at
`skills/e2e-test/journeys/README.md` (rendered from
[`templates/journeys-README.md.tmpl`](../templates/journeys-README.md.tmpl)). Edit that one template
rather than restating the block here — that keeps this reference and the shipped artifact from drifting.
Keep it **light Gherkin**: prose Given/When/Then/**Verify**, no step-definition binding (the executor is
an LLM agent, not Cucumber), front-matter `target:` + `layer:` + `covers:`.

## Conventions (the contract)

- **Verify-before-claim.** Every `Then` needs a **Verify** line — a concrete API response, DB/inspector
  check, or reload-and-re-snapshot. "Looks done" is **not** a pass; a `Verify` that can't be confirmed is
  a **FAIL**, not a pass-by-assumption.
- **Auth is browser-simulated** where the product has a real sign-in — drive the UI form, not an API
  auto-login. Backend-only fixture state may use `scripts/seed.sh`.
- **Report PASS/FAIL per `Then`**, with evidence (screenshot path + API JSON).
- **Cost-aware.** Scenarios that spend real model/compute budget on a live environment stay minimal;
  disable any loop/cron in cleanup.
- **One journey per capability area**, numbered. `00-walking-skeleton.md` proves the Layer-0 path.
- **A journey maps to a layer gate** — record evidence in `docs/layer-gates/<layer>.md`, then flip
  `layer_gates[layer=N]` in `product-context.yaml` (see [`layer-gate-loop.md`](layer-gate-loop.md)).
- **Traceability is the coverage hook.** Each scenario's **Verify** is what proves an acceptance
  criterion; the journey's `covers:` front-matter (and the `**Covers:**` body line) name which criteria.
  `/aep-build` builds the layer's **coverage matrix** from these — every layer acceptance criterion must
  map to ≥1 scenario `Verify` (or a Tier-1 scripted case / Tier-3 API check), or the gate can't reach
  `passed`. An uncovered criterion is auto-closed (a scenario gets authored) rather than ignored.

## Front-matter fields

| Field    | Values                       | Use                                                                                                                        |
| -------- | ---------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `target` | `web` / `mobile` / `desktop` | Tells `tool-selection.md` which automation track to resolve                                                                |
| `layer`  | integer (`0`, `1`, …)        | The layer gate this journey proves                                                                                         |
| `covers` | list of ids                  | Acceptance-criterion / capability ids this journey proves — feeds the gate's coverage matrix (`coverage.criteria_covered`) |

## Keeping journeys current

When a feature ships or changes behavior, update the matching journey in the **same PR**. A stale journey
silently shrinks regression coverage. New capability → new journey (copy the template above).
