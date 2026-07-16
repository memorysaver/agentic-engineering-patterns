# Layer advance: two-phase gate + distillation (wrap → Reflect and Advance)

Loaded at **layer completion** in product-cycle mode: `product-context.yaml` exists
and every story in the active layer is `completed`. Runs after the archive/cleanup
steps of `/aep-wrap` Phase 13. Both phases below are **execution** — the tier model,
applicable-tier rules, dogfood target, and journey/timing _theory_ are canonical in
the project's e2e-test skill (`skills/e2e-test/policy.md`) and the
`/aep-e2e-skill-scaffolding` references; consult them, do not restate them here.

---

## Contents

- [Layer Gate Check (two-phase, coverage-checked)](#layer-gate-check-two-phase-coverage-checked)
- [Layer Distillation](#layer-distillation)

---

## Layer Gate Check (two-phase, coverage-checked)

A gate is green only when the layer is **covered**, not when one journey passes.
**Read `skills/e2e-test/policy.md` first** (if the project has the e2e-test skill)
for the layer's applicable tiers, dogfood target (`none` / `cli` / `local` /
`deployed:<url>`), and timing — with `journey_timing: post-deploy` the journey runs
against the `deployed:<url>` target _here_, after merge/deploy, which is what flips
`scripted_passed → passed`.

1. **Tier-1 (machinery).** Run the project's scripted suite for this layer. If green,
   set `layer_gates[layer].status: scripted_passed` and record the test file under
   `evidence.scripted`.
2. **Tier-2/3 (product) + regression.** First run the **target-bound environment
   preflight** declared in `policy.md` (probes derived from `live_policy` — only
   the preconditions of criteria this gate _requires_; the optional live half of
   a non-milestone gate stays SKIP). An unmet required precondition is a named
   refusal (`REFUSING [...]`) — the gate pauses as `environment` with an ops
   checklist, spending zero scenarios; **REFUSED is not FAIL** (`/aep-gen-eval`
   → `references/verification-economics.md`). Then run only the tiers `policy.md`
   marks applicable. When Tier-2 applies, locate the layer's journey in
   `skills/e2e-test/journeys/` (`layer: N`), run it via its `tool-selection.md`
   (a `cli` journey runs the built binary via **bash** — no URL) plus any applicable
   API drivers, and **replay prior-layer journeys in full** — the full replay
   belongs here (per-story replay is impacted-only + canary, `/aep-build` Phase 8)
   — seeding the policy's target
   first (`deployed:<url>` needs `SERVER_URL=<url> bash skills/e2e-test/scripts/seed.sh`;
   `cli`/`local` seed locally). For large layers, run a **mid-layer full-replay
   checkpoint every k stories** so gate-time bisects stay tractable — k is
   derived, not improvised: default `k = min(5, ⌈N/3⌉)` for a layer of N stories,
   human-overridable. Record evidence — screenshots or CLI output
   (exit / stdout / fs), API JSON, PASS/FAIL per Then, and the two coverage matrices —
   in `docs/layer-gates/<layer>.md`.
   - **Missing-journey backstop (execution, not authoring):** the journey is a
     pre-merge build deliverable (`/aep-build` Phase 6 Step A authors it from the
     layer's acceptance criteria). If any journey planned in `layer_gates[N].journeys`
     is missing from disk (or none covers the layer), it is a **COVERAGE FAILURE** —
     do **not** flip to `passed`; leave the gate at `scripted_passed`, record the gap
     in `coverage.uncovered`, and route it back to `/aep-build` to author. Do not
     author a missing journey at the gate. (Fixing selector/route drift in an
     _existing_ journey to match the deployed target is fine.)
3. **Check coverage.** Confirm `coverage.criteria_covered == criteria_total` — every
   layer acceptance criterion maps to ≥1 proving test. A deliberate deferral carries a
   `WAIVER: <reason>` line; never flip to `passed` while criteria are silently uncovered.
4. **Flip to `passed`** only when all applicable tiers are green AND coverage is
   complete-or-waived AND the regression replay passed AND the evidence includes
   **at least one tamper-evident class** — a class the generator cannot modify
   (`/aep-gen-eval` → `references/verification-economics.md` → Tamper-Evident
   Evidence Classes): a CI run bound to the merged SHA (with workflow definitions
   outside the stories' diff scope), a wrap-executed journey, read-only golden
   fixtures with a ledger-equality oracle, or `/aep-watch` telemetry. A gate with
   no named evidence class records a **warning, not a refusal, for one release**
   (migration grace). Set `completed_at`. If only Tier-1 passed, leave it
   `scripted_passed`.
5. **Record the layer budget box.** In `docs/layer-gates/<layer>.md`, record the
   layer's **actual** verification spend — eval rounds across stories, suite runs
   - `suite_seconds`, journey scenarios, `cost_usd` where known — next to the
     **expected** values. Expected values are **human-owned** (set at layer
     planning or accepted from a distillation proposal — never authored by the loop
     graded against them). **Cold start:** the first instrumented layer has no box
     — record actuals only; those actuals plus a human-chosen margin become the
     next layer's expected values.
6. **Human-confirmed advance (observable).** Surface the coverage summary
   (`criteria_covered / criteria_total`, per-tier status, waivers) and get explicit
   user approval to begin the next layer's design. **Record that approval** (in the
   `layer_gates[layer]` entry or `docs/layer-gates/<layer>.md`) before the next
   `/aep-dispatch` proceeds — the gate flip is automatic-on-evidence; the _advance_ is
   a human decision.

---

## Layer Distillation

Distill what the completed layer taught before moving on. The **trigger rule,
distiller subagent protocol, and both output schemas are canonical in
[convergence.md](convergence.md) §2** — applied verbatim by `aep-autopilot`'s
tick-protocol Layer Completion too, so the two firing sites cannot double-fire. This
section is the wrap-side execution:

1. **Check the trigger** (world-derived, idempotent — convergence.md §2): fire iff the
   layer has ≥1 story, every story is `completed` / `deferred`, at least one is
   `completed`, **and `lessons-learned/retrospectives/layer-<N>.md` does not exist**.
   The retrospective file's existence is the dedupe — an interrupted distillation heals
   by re-running; skip if it already exists.
2. **Spawn an isolated subagent** (fresh context; reads only committed files — the
   layer's archived `openspec/changes/archive/*/convergence/` records plus
   `lessons-learned/*.md`, never a live worktree) per the distiller protocol in
   convergence.md §2. It writes exactly two artifacts:
   - `lessons-learned/retrospectives/layer-<N>.md` — the prose retrospective.
   - `lessons-learned/distillations/layer-<N>.yaml` — the structured, **proposal-only**
     distillation (`refinements` / `skill_amendments` / `weak_areas`). The distiller
     **never edits a skill file** — `skill_amendments` are proposals a human reviews
     and applies (same rule as `/aep-reflect`'s process feedback).
3. **Shape-validate then commit both files.** Run the shape-validation checklist
   (convergence.md §2) against the YAML before committing; on any failure, fix or
   re-run the distiller — never commit a malformed shape. The next `/aep-reflect`
   ingests the distillation via the `distillation` adapter (`aep-reflect`
   `references/telemetry-ingestion.md` → Distillation adapter).
