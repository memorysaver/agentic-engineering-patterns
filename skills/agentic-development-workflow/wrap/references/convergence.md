# Convergence: execution records + layer distillation

The producer contract for the build-convergence pipeline (decision doc:
`docs/decisions/build-convergence-pipeline.md` in the AEP repo — the canonical
schema definition; this file is the operative contract `/aep-wrap` executes).

Three phases, one determinism boundary: **mechanism decides WHEN and validates
SHAPE; an isolated agent owns the JUDGMENT; skill amendments are proposal-only.**

> **Authoring note:** this file is authored in the wrap skill, not generated
> from `_shared/` — `build-skills.sh` materializes shared resources only into
> `product-context/*` skills, so a wrap-side reference cannot live there. The
> consumer-side mapping (the `distillation` adapter) lives in
> `aep-reflect` `references/telemetry-ingestion.md`.

---

## 1. Convergence Gather (deterministic, per story — wrap step 2.5)

Runs on the integration branch, **before** `/opsx:archive`, writing into the
pre-archive change dir so the archive move carries everything in one commit:

```
openspec/changes/<change-name>/convergence/
  execution-record.yaml     # the manifest (schema below)
  lessons.md                # raw artifact copies the manifest indexes
  eval-response-*.md
  code-review-*.md
  dogfood-*.md
```

### Gather commands (wrap step 2.5)

```bash
CONV="openspec/changes/<change-name>/convergence"
mkdir -p "$CONV"
WS=".feature-workspaces/<name>/.dev-workflow"

# Best-effort copies — skip whatever is missing (never fail the wrap):
cp "$WS/lessons.md"                  "$CONV/" 2>/dev/null
cp "$WS"/signals/eval-response-*.md  "$CONV/" 2>/dev/null
cp "$WS"/code-review-*.md            "$CONV/" 2>/dev/null
cp "$WS"/dogfood-*.md                "$CONV/" 2>/dev/null
```

Then write `"$CONV"/execution-record.yaml` per the schema below — identity fields
(`story_id`, `merge_commit`, `pr_url`, `cost_usd`, timestamps) come from
`signals/status.json`; every field degrades to `null`.

### The gather manifest (parameterized)

**Minimum set** (all present in AEP's standard `.dev-workflow/` workspace layout):

| Signal             | Source in `.feature-workspaces/<name>/.dev-workflow/`    |
| ------------------ | -------------------------------------------------------- |
| Lessons            | `lessons.md`                                             |
| Gen-eval rounds    | `signals/eval-response-*.md`                             |
| Review findings    | `code-review-*.md`                                       |
| Dogfood reports    | `dogfood-*.md`                                           |
| Cost / PR identity | `signals/status.json` (`cost_usd`, `pr_url`, timestamps) |

A project with richer telemetry (mutation testing, coverage probes, custom
beacons) extends the copy list and `gathered_files` with the same rule.

**The best-effort rule (absolute):** every copy and every field degrades on a
missing source — an explicit `null` in the manifest, or simply no copy. The
gather **never fails the wrap, never blocks, never throws.** A project without
gen-eval still wraps cleanly with `gen_eval: []`.

### `execution-record.yaml` schema

```yaml
# openspec/changes/<change-name>/convergence/execution-record.yaml
story_id: <id> # REQUIRED — the only required field
generated_at: <ISO 8601> # the only timestamp in the record
merge_commit: <sha> | null # from git / signals; best-effort
pr_url: <url> | null # from signals/status.json; best-effort
cost_usd: <n> | null # from signals/status.json; best-effort
lessons_present: true | false
gathered_files: # sorted relative paths under convergence/
  - code-review-1.md
  - lessons.md
gen_eval: # per-round summaries; [] if none
  - round: 1
    result: PASS | FAIL
    scores: { completeness: <n>, correctness: <n>, security: <n>, code_quality: <n> }
review_findings: [] # one-line summaries from code-review artifacts; [] if none
```

Standalone mode (no `product-context.yaml`): `story_id` takes the change name.

---

## 2. Layer Distillation (judgment, per layer, isolated)

### Trigger rule (world-derived, idempotent)

Fire for layer N iff **all four** hold:

1. the layer has ≥1 story;
2. every story in the layer is `completed` or `deferred`;
3. at least one is `completed`;
4. `lessons-learned/retrospectives/layer-<N>.md` does **not** exist.

Condition 4 is the dedupe — the retrospective file's existence, not a state
file. Both firing sites (`/aep-wrap` → Reflect and Advance, and
`aep-autopilot/references/tick-protocol.md` → Layer Completion) apply this rule
verbatim, so they cannot double-fire and an interrupted run heals by re-running.

### Distiller subagent protocol

Spawn one **isolated** subagent (fresh context — not the wrap session):

- **Inputs (committed files only, never a live worktree):** the layer's archived
  convergence dirs (`openspec/changes/archive/*/convergence/` for the layer's
  stories) + `lessons-learned/*.md` for those changes + the layer's entry in
  `product-context.yaml` (acceptance criteria, outcome contract).
- **Task:** synthesize what the layer taught — patterns across stories, not
  per-story recaps. Where did builds drag or retry? What did gen-eval flag
  repeatedly? What would have made the layer cheaper or safer?
- **Outputs (both files, nothing else):**
  1. `lessons-learned/retrospectives/layer-<N>.md` — prose retrospective.
  2. `lessons-learned/distillations/layer-<N>.yaml` — schema below.
- **Hard rule:** the distiller **never edits skill files, product-context, or
  code**. `skill_amendments` are proposals a human reviews and applies.

### `distillation.yaml` schema

```yaml
# lessons-learned/distillations/layer-<N>.yaml
layer: <N>
generated_at: <ISO 8601>
refinements: # candidate product refinements
  - description: <what to change>
    target_layer: <layer to slot it into>
skill_amendments: # PROPOSAL-ONLY — never applied by the distiller
  - skill: <skill name>
    change: <what to add/modify>
    rationale: <why>
weak_areas: # recurring friction with no crisp fix yet
  - <string>
```

### Shape-validation checklist (the invoking session runs this before committing)

- [ ] YAML parses (`npx js-yaml <file> > /dev/null`)
- [ ] `layer` is a number matching the distilled layer; `generated_at` present
- [ ] `refinements[*]` each have non-empty `description` + `target_layer`
- [ ] `skill_amendments[*]` each have non-empty `skill` + `change` + `rationale`
- [ ] `weak_areas` is a list of strings (may be empty)
- [ ] The retrospective `.md` exists and is non-empty
- [ ] No other file was created or modified by the distiller

On any failure: fix or re-run the distiller; do not commit a malformed shape.

---

## 3. Reflect ingestion (consumer side — for reference)

The next `/aep-reflect` (or `/aep-watch`) ingests
`lessons-learned/distillations/*.yaml` via the **`distillation` adapter**
(`aep-reflect` `references/telemetry-ingestion.md` → Distillation
adapter): dedupe-only (no high-water mark), per-item `suggested_class` hints
(`refinements` → refinement, `skill_amendments` → process — never auto-filed,
`weak_areas` → discovery/refinement), human confirms every classification.
