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
  "source": "error_stream | analytics | monitoring | bug_tracker | dogfood",
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

### Dogfood-report adapter (`kind: dogfood_report`)

Dogfood runs — local (`/aep-build` Phase 6), post-deploy (autopilot post-merge
guard), **or a standalone / ad-hoc live exercise** — emit the **unified markdown
report** (`## <title>` / `**Severity:**` / `**Category:**` / `**Repro:**` /
`**Observed:**` / `**Expected:**` / `**Evidence:**`) to `.dev-workflow/dogfood-*.md`
(see `patterns/executor/references/dogfood-validation.md` → Unified report format).
This adapter parses each `##` finding into the normalized record above so the
**same** Step 2 classifier consumes it — closing the G6 self-feeding loop for
**every** dogfood trigger, not just the guard path. It is a **file glob**, not a
network source: self-describing, so `coverage_check` (§1.5) does not gate it.

Source config:

```yaml
sources: # watch.sources[] (or telemetry_sources[])
  - kind: dogfood_report
    glob: ".dev-workflow/dogfood-*.md" # default; add post-deploy report paths as needed
```

Per-finding mapping (markdown field → normalized record):

| Dogfood field                                | Normalized record                                                                                                                                            |
| -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `## <title>`                                 | `signal` (one-line) + basis for the dedupe key                                                                                                               |
| `**Repro / Observed / Expected / Evidence**` | `evidence` (repro steps + observed-vs-expected; **no secrets**)                                                                                              |
| `**Severity:**`                              | priority input — `blocker`/`major` → `high`; `minor` → normal                                                                                                |
| `**Category:**`                              | `suggested_class` hint — `UX`/`logic`/`edge-case`/`accessibility` → `bug`; `visual`/`performance` → `bug` when Severity ∈ {blocker,major}, else `refinement` |
| —                                            | `source: "dogfood"`, `story_ref: null`                                                                                                                       |

`suggested_class` is a **hint only** — the Step 2 classifier (and the human, unless
`full_auto`) makes the final call, exactly as for every other source. In
particular a finding that reads as **calibration / discovery / opportunity-shift /
process** is **not** auto-filed; it surfaces to a human (see `/aep-watch` Step 2).

**Stable dedupe key.** Each finding gets a deterministic
`external_id = "dogfood:" + slug(report-basename) + ":" + shorthash(slug(title) + "|" + category)`,
so re-running the **same** dogfood (same findings) never creates duplicate stories:
`/aep-watch` Step 3 dedupes on `watch_origin.{source,external_id}`, and the
post-merge guard Path 1 uses the **same** key, so whichever fires first wins and
the other no-ops. A genuinely new finding (new title/category) yields a new id and
a new story.

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
  auto-eval lets advance without a pause)
