# Telemetry Ingestion & Outcome Auto-Evaluation

How `/aep-reflect` (and `/aep-watch`) pull real-world signals automatically, and
how a layer's **quantitative** outcome contract is evaluated without a human.
This augments the interactive reflect flow — it never replaces human review by
default. (Gap G5.)

> **Authoring note:** this file is canonical in
> `skills/product-context/_shared/references/`; `scripts/build-skills.sh`
> materializes it into each consuming skill's `references/`.

---

## 1. Automated source ingestion

Pull from read-only sources with `bash`/`curl`/`jq` and reduce each to the
**normalized observation record** the reflect Step 2 classifier consumes:

```json
{
  "source": "error_stream | analytics | monitoring | bug_tracker | dogfood | distillation",
  "signal": "one-line description of what was observed",
  "evidence": "url | query | sample (no secrets)",
  "story_ref": "<story-id if attributable, else null>",
  "suggested_class": "bug | refinement | discovery | opportunity_shift | process | null"
}
```

`suggested_class` is a hint only — the reflect Step 2 classifier (and the human,
unless `full_auto`) makes the final call. Ingested records are **merged** with
interactive input before classification; automation augments, never replaces.

### Source config

Endpoints live under `topology.routing.telemetry_sources` (a list). Each entry:

```yaml
telemetry_sources:
  - kind: error_stream # error_stream | analytics | monitoring | bug_tracker
    endpoint: "https://…/api/…?since={since}" # {since} = last-ingest high-water mark
    token_env: SENTRY_TOKEN # NAME of an env var / secret — never the secret itself
    metric_map: # for analytics/monitoring: outcome-metric name → query
      activation_rate: "SELECT … "
```

**Safety:** access is **read-only**; reference credentials by env-var / secret-store
name only — **never embed secrets in the repo or in `product-context.yaml`**.

### The `/aep-watch` finding record

`/aep-watch` Step 1 reduces every source item to this **finding record** — the
operative shape Step 3 dedupes and Step 4 turns into a story (distinct from the
5-field observation record above, which is the classifier's conceptual input):

```yaml
- source: "sentry" # which configured source produced it
  external_id: "ISSUE-4821" # stable id — the dedupe key (Step 3)
  title: "TypeError in checkout flow"
  detail: "..." # stack / message / metric summary (no secrets)
  signal: error_stream # bug_tracker | error_stream | telemetry | dogfood
  count: 142 # occurrences / affected users (priority input)
  first_seen: "<ISO8601>"
  last_seen: "<ISO8601>"
```

`external_id` is the dedupe key for every source. The two file-glob adapters
(`dogfood_report`, `distillation`) carry no occurrence count or timestamp, so
they leave `count`/`first_seen`/`last_seen` unset and supply the deterministic
`external_id` defined in their adapter sections below.

### Dogfood-report adapter (`dogfood_report` source)

Dogfood runs — local (`/aep-build` Phase 6), post-deploy (autopilot post-merge
guard), **or a standalone / ad-hoc live exercise** — emit the **unified markdown
report** (`## <title>` / `**Severity:**` / `**Category:**` / `**Repro:**` /
`**Observed:**` / `**Expected:**` / `**Evidence:**`) to `.dev-workflow/dogfood-*.md`
(see `/aep-executor` → `references/dogfood-validation.md`, Unified report format).
This adapter parses each `##` finding into the **`/aep-watch` Step 1 finding
record** (the operative shape Step 3 dedupes and Step 4 turns into a story — _not_
the 5-field telemetry record above, which is the classifier's conceptual input)
so the **same** Step 2 classifier consumes it — closing the G6 self-feeding loop
for **every** dogfood trigger, not just the guard path. It is a **file glob**, not
a network source: self-describing, so `coverage_check` (§1.5) does not gate it.

Source config (the discriminator key matches the container: `type:` under
`watch.sources[]`, `kind:` under `telemetry_sources[]`):

```yaml
watch:
  sources:
    - type: dogfood_report
      glob: ".dev-workflow/dogfood-*.md" # default; add post-deploy report paths as needed
```

Per-finding mapping (markdown field → finding field):

| Dogfood field                                | Finding field                                                                                                                                                                                                          |
| -------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `## <title>`                                 | `title` (the story title `/aep-watch` Step 4 reads)                                                                                                                                                                    |
| `**Repro / Observed / Expected / Evidence**` | `detail` (repro steps + observed-vs-expected; **no secrets**) — also the classifier's `evidence`                                                                                                                       |
| `**Severity:**`                              | priority — `blocker`/`major` → `high` (`critical` if it blocks a core flow); `minor` → normal. Dogfood findings have **no `count`**, so priority comes from Severity, not the count-based escalation other sources use |
| `**Category:**`                              | `suggested_class` hint — `UX`/`logic`/`edge-case`/`accessibility` → `bug`; `visual`/`performance` → `bug` when Severity ∈ {blocker,major}, else `refinement`                                                           |
| `**Failure-Class:**`                         | routing gate (below) — only `product-defect` findings continue into the Step 2 classifier / story path; absent line → `product-defect` (the tamper-resistance default)                                                 |
| —                                            | `signal: dogfood`, `story_ref: null`, `external_id:` (below); `count`/`first_seen`/`last_seen` **unset** (the report carries no occurrence count or timestamp)                                                         |

**Failure-class routing (before classification).** The adapter **never
auto-files** non-product classes (`/aep-gen-eval` →
`references/verification-economics.md` → Failure Taxonomy):

- `environment` → surfaced to the human/orchestrator as an **ops checklist**
  (named refusal tags + implied repairs); no story, no classifier pass.
- `harness-flake` → surfaced for **quarantine ratification** (wrap/reflect
  confirms the reproduction evidence); the ratified quarantine files a _harness_
  story through the human, never automatically.
- `scope` → surfaced to the human gate for `/aep-reflect` re-slicing.
- `product-defect` (or no `Failure-Class:` line — the default) → the normal
  path below.

`suggested_class` is a **hint only** — the Step 2 classifier (and the human, unless
`full_auto`) makes the final call, exactly as for every other source. In
particular a finding that reads as **calibration / discovery / opportunity-shift /
process** is **not** auto-filed; it surfaces to a human (see `/aep-watch` Step 2).

**Escape-rate ingestion (reflect side).** When `/aep-reflect` classifies a
post-merge bug, it traces the bug to the story that introduced it (blame the
diff, not the reporter) and appends an entry to that story's
`escaped_defects` in the archived execution record
(`openspec/changes/archive/<change>/convergence/execution-record.yaml` →
`verification:` block). Escape rate is defined **per story per tier**; when
attribution is ambiguous (multi-story interaction bugs) the escape attributes
to the **layer**, not to no one. This is the feedback signal the dampened
calibration loop reads (`aep-wrap` `references/convergence.md`).

**No high-water mark — dedupe-only.** The unified report has no per-finding
timestamp, so a `dogfood_report` source does **not** advance `watch.since` (that
cursor applies only to time-ordered sources); re-scanning the glob each tick is
harmless because idempotency rests entirely on the **stable dedupe key**. Each
finding gets a deterministic
`external_id = "dogfood:" + slug(report-basename) + ":" + shorthash(slug(title) + "|" + category)`,
so `/aep-watch` Step 3 dedupes on `watch_origin.{source,external_id}`: already-filed
findings no-op, and a genuinely new finding (new title/category) yields a new id
and a new story. The autopilot post-merge guard Path 1 stamps the **same**
`external_id` on the story it files, so whichever path ingests a given report first
wins and the other no-ops — no double-filing.

### Distillation adapter (`distillation` source)

Layer Distillation — the isolated, proposal-only synthesis `/aep-wrap` (Reflect
and Advance → Layer Distillation) or `aep-autopilot` (tick-protocol → Layer
Completion) runs when a layer completes — writes
`lessons-learned/distillations/layer-<N>.yaml` (producer contract:
`aep-wrap` `references/convergence.md`). This adapter normalizes each item into
the same observation record Step 2 classifies. Like `dogfood_report`, it is a
**file glob**, not a network source: self-describing, so `coverage_check` (§1.5)
does not gate it.

Source config:

```yaml
watch:
  sources:
    - type: distillation
      glob: "lessons-learned/distillations/*.yaml" # default
```

Per-item mapping (distillation field → observation record):

| Distillation item    | Mapping                                                                                                                                                                                                                                                                        |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `refinements[]`      | `suggested_class: refinement`; `signal` = the `description`; the item's `target_layer` rides along so the refinement slots into the right layer                                                                                                                                |
| `skill_amendments[]` | `suggested_class: process`; `signal` = `"<skill>: <change>"`; `rationale` → evidence. **Never auto-filed as a story and never auto-applied** — routed to the changelog as a proposed amendment for human review (same rule as `/aep-reflect`'s "do not auto-edit skill files") |
| `weak_areas[]`       | `suggested_class: refinement` (or `discovery` when it invalidates a stated assumption); `signal` = the string                                                                                                                                                                  |
| —                    | every record: `source: distillation`, `story_ref: null`, evidence anchored to `retrospectives/layer-<N>.md` + the `.yaml`; `count`/timestamps **unset**                                                                                                                        |

`suggested_class` is a **hint only** — the human confirms every classification,
exactly as for `dogfood_report`.

**No high-water mark — dedupe-only.** Distillation items carry no per-item
timestamp, so this source does **not** advance `watch.since`; idempotency rests
on the stable dedupe key
`external_id = "distillation:" + layer + ":" + shorthash(slug(item))`
(dedupe on `(layer, item)`): re-scanning the glob is harmless, already-ingested
items no-op, and a re-distilled layer would need a changed item to produce a new
id.

---

## 1.5 Deciding which sources to wire (the coverage rule)

You don't list telemetry for its own sake — **a source is needed _iff_ some
declared signal requires it.** The decision is **hybrid**:

1. **Metric-driven (what signals do we need?)** — enumerate every **quantitative**
   `success_metric` across `product.layers[].outcome_contract` plus every
   `topology.routing.post_merge_guard.health_signals` entry. That set _is_ the
   demand for telemetry.
2. **Inventory (which tool provides each?)** — `/aep-scaffold`'s audit detects the
   project's observability stack (Sentry, Datadog, PostHog, OpenTelemetry, log
   drains, `/healthz`-style endpoints) and records **candidate** `telemetry_sources`
   (kind + endpoint + `token_env`, no `metric_map` yet); you can also add candidates
   by hand.
3. **Bind (`/aep-map`)** — for each needed signal, attach it to a candidate source
   by adding a `metric_map: { <metric-or-signal>: "<query>" }` entry. A needed
   signal with no measurable source is **flagged**, not ignored: make the metric
   qualitative, or record it `unmeasured` — never leave a quantitative metric
   silently un-sourced.

### `coverage_check(needed)` — the guard helper

Consumers that rely on telemetry (`/aep-watch`, `/aep-reflect` Step 2.75,
`/aep-autopilot`) call this **before** trusting auto behavior. It is pure
config inspection — no network:

```
coverage_check(needed_signals):
  missing = []
  for sig in needed_signals:           # quantitative success_metric names + health_signals
    if no telemetry_sources[*].metric_map has key == sig
       (and, for a health_signal, no source/endpoint provides it):
      missing.append(sig)
  return { covered: missing == [], missing }
```

**On `covered == false`:** surface
`"telemetry binding incomplete for <missing> — run /aep-map (observability step)"`
and **block the auto path** (watch refuses to claim auto-coverage; reflect falls
back to the human pause; autopilot pauses). Missing wiring must **block auto,
never silently no-op** — that's the v2 human-in-the-loop default.

---

## 2. Outcome-contract auto-evaluation

A layer's `outcome_contract` carries a `success_metric` (`type` + `target`) and a
`decision_rule` (`keep_if` / `otherwise`). **Precondition:** run
`coverage_check([success_metric])` (§1.5) first — if the metric isn't bound to a
source, take the human-pause path (the binding is incomplete; do not auto-eval).
When covered, evaluate per `topology.routing.auto_outcome_eval`:

| Metric `type`                                        | `auto_outcome_eval: quantitative`                                                                                                     | default (`none`)               |
| ---------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------ |
| **quantitative** (numeric, measurable from a source) | fetch actual value via the matching `telemetry_sources` query, apply `keep_if`/`otherwise` mechanically, record result — **no pause** | human pause (current behavior) |
| **qualitative**                                      | human pause — **unless** `full_auto: true` (then agent-judgment auto-eval)                                                            | human pause                    |

On a **fetch failure or ambiguity**, fall back to the human pause (fail safe, not
fail open). Record every auto-evaluation in the `changelog`:

```yaml
- date: YYYY-MM-DD
  type: outcome_evaluation
  summary: "Layer N: <metric> = <actual> vs target <target> → passed|failed (auto)"
```

---

## 3. `full_auto` interaction (A1)

`topology.routing.full_auto` (default **false**) is the master switch. It only
changes the **qualitative** path:

| `full_auto`     | `auto_outcome_eval`    | quantitative outcome | qualitative outcome          |
| --------------- | ---------------------- | -------------------- | ---------------------------- |
| false (default) | none                   | human pause          | human pause                  |
| false           | quantitative           | auto-eval            | human pause                  |
| true            | (implied quantitative) | auto-eval            | **agent-judgment auto-eval** |

Default keeps humans in the loop; only an explicit `full_auto: true` removes the
qualitative pause.

---

## Cross-references

- `/aep-reflect` Step 1 (Gather Feedback) and Step 2.75 (Evaluate Outcome Contracts)
- `/aep-watch` (reuses the normalized observation record for its ingest step)
- `aep-autopilot` `references/tick-protocol.md` — Step ⑥ Layer Completion (what the
  auto-eval lets advance without a pause; also the autopilot firing site for the
  Layer Distillation the `distillation` adapter ingests)
- `aep-wrap` `references/convergence.md` — the producer contract for the
  `distillation` source (execution records + distiller protocol + schemas)
