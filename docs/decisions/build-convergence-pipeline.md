# Build Convergence Pipeline: Gather, Distill, Reflect

Why build-time runtime signal dies at `git worktree remove` while the methodology's whole purpose is retrospective and distillation-into-skills — and how to carry that signal into the archive, synthesize it per layer, and feed it back through `/aep-reflect`. Proven downstream in SIBYL across 21 layers; this document upstreams the project-agnostic shape. Complements [deterministic-orchestration.md](deterministic-orchestration.md) — the gather-before-archive ordering invariant proposed here is exactly the class of mechanical step that document argues must not live as recallable prose.

> **Status:** Accepted — implemented in **v2.7.0** (PR #22). This document records the design precisely; the implementation was reviewed against it. It changes how AEP works, so it lives in `decisions/` per the [docs routing guide](../README.md).

> **Sourcing note:** The pipeline below was designed, implemented, and dogfooded in SIBYL (`SIBYL/docs/AEP-improvement-suggestion/2026-07-06-build-convergence-pipeline.md`, Status: implemented in SIBYL as SIBYL-owned determinism). SIBYL's harness-specific machinery (its `wrap-gather` / `distill` CLIs and YAML splice transactions) stays downstream; what upstreams here is the abstract three-phase shape and its determinism boundary.

---

## The Asymmetry

A story's build produces two kinds of output, and AEP currently preserves only one:

| Output                                                                                                                    | What happens at `/aep-wrap`                                                                                           | Outcome                            |
| ------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- | ---------------------------------- |
| **The change itself** (code, OpenSpec change dir)                                                                         | Merged, then archived via `/opsx:archive` and committed (`wrap/SKILL.md:65-79`)                                       | Durable, versioned                 |
| **The runtime signal** (lessons, gen-eval rounds, review findings, dogfood reports, cost/PR identity, progress narrative) | One file rescued (`lessons.md` → `lessons-learned/`, `wrap/SKILL.md:130-150`); the rest dies at teardown (`:152-173`) | Discarded at `git worktree remove` |

For a methodology whose feedback loop (`/aep-reflect`) exists to turn execution experience into product-context and skill refinements, discarding the majority of execution experience at teardown is the wrong default.

---

## Diagnosis — where build signal dies today

Three compounding gaps, all in the wrap/reflect seam:

### A. Only one artifact survives, and it survives by a fragile ordering

`/aep-wrap` step 5.5 (`wrap/SKILL.md:130-150`) copies `.feature-workspaces/<name>/.dev-workflow/lessons.md` to `lessons-learned/<change-name>.md` — guarded only by prose ("Why before worktree removal", `:148`) and a guardrail bullet ("Read signals AND lessons before removing worktrees", `:183`). Everything else in the workspace's `.dev-workflow/` — gen-eval round responses (`signals/eval-response-*.md`), code-review findings (`code-review-*.md`), dogfood reports (`dogfood-*.md`), progress narrative (`progress-*.md`), the final `signals/status.json` with cost and PR identity — is deleted at step 6 (`:152-173`).

### B. The rescue runs after the archive, so the archive can't carry it

The OpenSpec archive move and its commit (steps 3–4, `wrap/SKILL.md:65-79`) run **before** the lessons copy (step 5.5). Anything gathered after the archive needs its own separate commit and lives outside the change's archived record; anything gathered after teardown is silently lost. The natural durable carrier already exists: files placed in `openspec/changes/<change-name>/` **before** step 3 ride the archive `mv` and its commit in one shot. Nothing uses it today.

### C. Reflect has per-story crumbs but no layer-level synthesis

`/aep-reflect` Step 1 (`reflect/SKILL.md:39-52`) reads dogfood reports (`:47`, via the `dogfood_report` adapter) and `lessons-learned/*.md` (`:48`) — raw, per-story signal. No step in the lifecycle asks "what did this **layer** teach us?" even though layer completion is explicitly detected in two places: `/aep-wrap`'s Layer Gate Check (`wrap/SKILL.md:193-242`) and autopilot's tick-protocol Layer Completion (`patterns/autopilot/references/tick-protocol.md:458-470`). Patterns emerge across a layer, not within a single story; today they must be re-derived by a human reading N lessons files.

---

## The Model: GATHER → DISTILL → REFLECT

Three phases, split along the same determinism boundary that [deterministic-orchestration.md](deterministic-orchestration.md) names: **mechanism decides WHEN and validates SHAPE; an isolated agent owns the JUDGMENT; skill amendments are proposal-only end to end.**

### Phase 1 — Convergence Gather (deterministic, per story, at `/aep-wrap`)

A new wrap step **2.5**, immediately before `/opsx:archive` (between `wrap/SKILL.md:63` and `:65`): converge the workspace's runtime signal into the **pre-archive change dir** so steps 3–4 carry it with zero extra machinery.

```
openspec/changes/<change-name>/convergence/
  execution-record.yaml     # the deterministic manifest (below)
  lessons.md                # copies of the raw artifacts the manifest indexes
  eval-response-*.md
  code-review-*.md
  ...
```

```yaml
# openspec/changes/<change-name>/convergence/execution-record.yaml
story_id: <id> # required; the only required field
generated_at: <ISO 8601> # the only timestamp
merge_commit: <sha> | null # every field below is best-effort:
pr_url: <url> | null #   a missing source degrades to explicit null —
cost_usd: <n> | null #   the gather NEVER fails or blocks the wrap
lessons_present: true | false
gathered_files: [] # sorted relative paths under convergence/
gen_eval: [] # per-round summaries: { round, result, scores }
review_findings: [] # one-line summaries from code-review artifacts
```

**The gather manifest is parameterized.** The minimum set every project gathers: lessons + gen-eval/review findings + cost/PR identity (all present in AEP's standard `.dev-workflow/` layout — `lessons.md`, `signals/eval-response-*.md`, `code-review-*.md`, `signals/status.json`). A project with richer telemetry (mutation testing, coverage probes) extends `gathered_files` with the same rule: **best-effort, explicit `null`/absent on missing, never throw.**

The existing step 5.5 `lessons-learned/` copy **stays, unchanged and additive** — `/aep-reflect`'s existing lessons source (`reflect/SKILL.md:48`) is never orphaned. The converged record is a superset carried in the archive; the lessons copy is the fast-path index reflect already reads.

### Phase 2 — Layer Distillation (judgment, per layer, isolated)

When a layer completes, an **isolated subagent** (independent context; reads only committed archive files — never a live worktree) reads the layer's set of converged records plus its `lessons-learned/` files and writes two artifacts:

- `lessons-learned/retrospectives/layer-<N>.md` — the prose retrospective (what worked, what dragged, what surprised).
- `lessons-learned/distillations/layer-<N>.yaml` — the structured, **proposal-only** distillation:

```yaml
# lessons-learned/distillations/layer-<N>.yaml
layer: <N>
generated_at: <ISO 8601>
refinements: # candidate product refinements
  - description: <what to change>
    target_layer: <layer to slot it into>
skill_amendments: # PROPOSAL-ONLY — the distiller never edits a skill file
  - skill: <skill name>
    change: <what to add/modify>
    rationale: <why>
weak_areas: # recurring friction with no crisp fix yet
  - <string>
```

**Trigger rule (world-derived, idempotent):** distillation for layer N fires iff the layer has ≥1 story, every story is `completed`/`deferred`, at least one is `completed`, **and `lessons-learned/retrospectives/layer-<N>.md` does not exist**. The existence check — not a state file — is the dedupe, so both firing sites below can share it without coordination, and an interrupted distillation heals by re-running.

**Two firing sites, same rule:** the wrap path (a new "Layer Distillation" subsection in `/aep-wrap` → Reflect and Advance, after the Layer Gate Check `wrap/SKILL.md:193-242`) and the autopilot path (tick-protocol Layer Completion, `tick-protocol.md:458-470`). Autopilot detects layer completion independently of a manual wrap; wiring only wrap would leave the autonomous path skipping distillation — exactly the silent prose-step drift [deterministic-orchestration.md](deterministic-orchestration.md) documents.

The invoking skill validates the output **shape** (fields present, lists well-formed) before committing; the subagent owns only the judgment. `skill_amendments` mirrors the exact shape and rule `/aep-reflect` already enforces for process feedback (`reflect/SKILL.md:108`, `:275`: "**do not auto-edit skill files**" — a human reviews and applies).

### Phase 3 — Reflect ingestion (routing, human-confirmed)

`/aep-reflect` Step 1 gains a **Layer distillations** source, and the shared telemetry reference gains a `distillation` adapter mirroring the existing `dogfood_report` adapter (`_shared/references/telemetry-ingestion.md:49-98`):

- **Source shape:** a file glob (`lessons-learned/distillations/*.yaml`), self-describing — like `dogfood_report`, it is not a network source and `coverage_check` (§1.5) does not gate it.
- **Per-item mapping:** `refinements[]` → `suggested_class: refinement` (evidence = description, `layer_ref` = `target_layer`); `skill_amendments[]` → `suggested_class: process`, routed to the changelog as a proposed amendment — **never auto-filed, never auto-applied**; `weak_areas[]` → `refinement` (or `discovery` when it invalidates a stated assumption).
- **No high-water mark — dedupe-only**, same rationale as `dogfood_report` (`telemetry-ingestion.md:88-98`): distillation files carry no per-item timestamp; idempotency rests on a stable key `external_id = "distillation:" + layer + ":" + shorthash(slug(item))`.
- `suggested_class` remains a **hint**: the human confirms every classification, exactly as for every other source.

Reflect thus consumes **one converged, distilled source** per layer instead of raw per-signal telemetry — while the raw records remain in the archive for anyone who needs to drill down.

---

## Granularity: layer-only, deliberately

Distillation fires at the **layer** gate, not per story. Patterns emerge across a layer; per-story distillation would multiply cost and dilute signal (SIBYL evaluated both and chose layer-only). Gather, by contrast, is per-story — it must be, since the worktree dies per-story. **Open question recorded for a future revision:** whether to offer an optional per-story distillation mode hanging off the same execution records; nothing in this design precludes it.

---

## Forcing functions

No new commands — every change enhances an existing skill:

| Skill            | New responsibility                                                                                                                                                         |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/aep-wrap`      | Step 2.5 Convergence Gather (before archive); "Layer Distillation" subsection in Reflect and Advance; guardrails "gather before archive" + "distill is idempotent"         |
| `/aep-autopilot` | tick-protocol Layer Completion runs the same Layer Distillation step (autonomous-path parity), feeding the Orchestration Learning Checkpoint (`tick-protocol.md:472-474`)  |
| `/aep-reflect`   | Step 1 gains the **Layer distillations** source via the `distillation` adapter                                                                                             |
| `/aep-watch`     | `distillation` recognized as a source type; joins `dogfood_report` in the no-high-water-mark exception; `skill_amendments` surface to a human, never auto-filed as stories |

---

## Migration (additive-first)

Each phase is independently shippable; **nothing is removed or renumbered** — existing wrap steps keep their numbers (2 → **2.5** → 3 …), and step 5.5 stays.

| Phase                       | Action                                                                                                             | Breaking? |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------ | --------- |
| **P0 — this decision doc**  | Record the design; implementation PR is reviewed against it                                                        | No        |
| **P1 — Convergence Gather** | Insert wrap step 2.5 + `wrap/references/convergence.md` (schemas, manifest, distiller protocol)                    | No        |
| **P2 — Layer Distillation** | Wire the trigger into wrap Reflect-and-Advance **and** autopilot tick-protocol Layer Completion                    | No        |
| **P3 — Reflect ingestion**  | `distillation` adapter in `_shared/references/telemetry-ingestion.md`; reflect Step 1 bullet; watch source listing | No        |

**Exact change sites (the implementation PR's review contract):**

- `skills/agentic-development-workflow/wrap/SKILL.md` — new step 2.5 between `:63` and `:65`; one clarifying sentence in step 4 (`git add openspec/` already stages the record); a note at step 5.5 that the lessons copy stays additive; new "Layer Distillation" subsection between Layer Gate Check and Feedback Loop; two new guardrail bullets.
- **NEW** `skills/agentic-development-workflow/wrap/references/convergence.md` — the operative producer contract: gather manifest, `execution-record.yaml` schema, distiller subagent protocol, `distillation.yaml` schema, shape-validation checklist, idempotence rule. This file is **authored, not generated**: `scripts/build-skills.sh` materializes `_shared/` only into `skills/product-context/*/`, so a wrap-side reference cannot live in `_shared/` — do not "move it to `_shared/` for consistency"; wrap would never receive a copy.
- `skills/product-context/_shared/references/telemetry-ingestion.md` — extend the source enum (`:21`) with `distillation`; new "Distillation adapter" section after the dogfood adapter; update Cross-references (`:186-192`). Then run `bash scripts/build-skills.sh` — the generated copies in `envision`, `map`, `reflect`, `watch`, `dispatch` update from the canonical source; **never hand-edit a `.aep-generated` copy**.
- `skills/product-context/reflect/SKILL.md` — Step 1 gains a "Layer distillations" bullet after `:48`.
- `skills/product-context/watch/SKILL.md` — source-type listing + no-high-water-mark exception + never-auto-file rule.
- `skills/patterns/autopilot/references/tick-protocol.md` — Layer Completion (`:458-470`) gains the distillation step; Orchestration Learning Checkpoint note.
- `skills/product-context/_shared/templates/product-context-schema.yaml` — comment-level mention of `distillation` under `watch.sources` (propagates via rebuild).
- Docs: `docs/glossary.md` (add **Convergence Pipeline**, **Execution Record**, **Layer Distillation** — qualified names; "gather" and "distill" already carry other senses in reflect Step 1 and memory-forge), `docs/skills-quick-reference.md` (`/aep-wrap`, `/aep-reflect` rows), `CHANGELOG.md` `[2.7.0]`, `.claude-plugin/marketplace.json` + `package.json` → `2.7.0`.

**Propagation discipline:** `distillation` is a new source-type enum value — it must land at **every** listing site (the `_shared` enum line + adapter section, the five build-generated copies via rebuild only, `watch/SKILL.md`'s source list **and** its high-water-mark exception, `reflect/SKILL.md` Step 1, the schema comment) in the same PR; audit with `grep -rn "distillation" skills/ docs/` against this list, using the `dogfood_report` footprint as the shape of a fully-propagated source type. The schemas are restated in three places by necessity (this doc = canonical; `wrap/references/convergence.md` = producer contract; the adapter = consumer mapping) — future edits fan out to all three. Downstream consumers go live only after the v2.7.0 tag is cut and each consumer re-pins via the skills CLI.

---

## Worked example: one SIBYL layer, end to end

- Eight stories build in worktrees; each `/aep-wrap` step 2.5 writes `openspec/changes/<id>/convergence/execution-record.yaml` (lessons + three gen-eval rounds + review findings + `cost_usd` from `signals/status.json`); the archive commit carries each record into `openspec/changes/archive/`.
- The eighth story completes the layer. The wrap's Layer Distillation check finds all stories completed and no `retrospectives/layer-21.md` → spawns the isolated distiller, which reads eight archived records and writes the retrospective plus a distillation: two `refinements` targeting layer 22, one `skill_amendment` proposing a build-skill guardrail, one `weak_area` (flaky seeding).
- The next `/aep-reflect` ingests the distillation via the adapter: the human confirms both refinements (slotted into layer 22), routes the skill amendment to the changelog as a proposed change (applied by hand after review), and reclassifies the weak area as a discovery.
- Cost of the whole pipeline beyond wrap's existing work: one subagent run per **layer**, not per story.

(SIBYL ran exactly this loop for its layer 21 — the distillation artifacts are `SIBYL/docs/retrospectives/layer-21.md` + `layer-21.distillation.yaml`.)

---

## Anti-patterns this prevents

- **Gathering after the archive (or after teardown).** The ordering is the invariant: convergence dir written → archive `mv` carries it. Post-archive gathering needs a second commit and races teardown — the current step-5.5 fragility, generalized away.
- **The distiller editing skill files.** Amendments are proposals with rationale; a human applies them (`reflect/SKILL.md:275` already states the rule — this design extends it, never weakens it).
- **Re-distilling a completed layer.** The retrospective file's existence is the idempotence check at both firing sites; a state-file dedupe would drift.
- **Blocking a wrap on a missing signal source.** Every gather field is best-effort → explicit `null`. A project without gen-eval still wraps cleanly.
- **Treating `suggested_class` as a decision.** Adapter output is a hint; the human confirms, exactly as for `dogfood_report`.

---

## References

- `SIBYL/docs/AEP-improvement-suggestion/2026-07-06-build-convergence-pipeline.md` — the originating downstream proposal; SIBYL's `execution-record.yaml` / `distillation.yaml` and `wrap-gather` / `distill` CLIs are the reference implementation (harness-specific; not upstreamed as-is).
- [deterministic-orchestration.md](deterministic-orchestration.md) — the companion proposal: gather-before-archive is an ordering invariant of exactly the class it argues belongs behind mechanism, and the distillation trigger's file-existence idempotence is an instance of its world-derived resumability.
- `skills/product-context/_shared/references/telemetry-ingestion.md` — the `dogfood_report` adapter (`:49-98`) this design mirrors: file-glob source, hint-only classes, dedupe-only idempotency.
- `skills/agentic-development-workflow/wrap/SKILL.md` — the lifecycle this slots into (steps `:57-173`, Reflect and Advance `:187-252`).
- `skills/patterns/autopilot/references/tick-protocol.md` — Layer Completion (`:458-470`), the second firing site.
- Affected skills: `/aep-wrap`, `/aep-autopilot`, `/aep-reflect`, `/aep-watch`.
