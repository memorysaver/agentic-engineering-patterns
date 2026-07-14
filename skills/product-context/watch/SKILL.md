---
name: aep-watch
description: Continuously monitor bug trackers, error streams, and telemetry; classify findings with the /aep-reflect classifier, dedupe against the backlog, and auto-create bug/refinement stories in product-context.yaml — the self-feeding reflect→dispatch loop. Use for "watch", "monitor for new work", "ingest errors", "auto-create stories from telemetry", "/aep-watch". Main workspace only.
---

# Watch

Self-feeding work discovery. `/aep-watch` is a continuous/scheduled monitor: it
pulls from configured sources (bug trackers, error streams, telemetry),
classifies each finding with the **same classifier as `/aep-reflect`**, dedupes
against the backlog, and writes new bug/refinement stories into
`product-context.yaml` — which then flow into `/aep-dispatch` (or autopilot picks
them up), closing the loop so the system keeps finding work without a human
running `/aep-envision` or `/aep-reflect` by hand.

```
sources → [ /aep-watch: pull → classify → dedupe → write stories ] → product-context.yaml
                                                                          │
                                                                          ▼
                                                          /aep-dispatch  (or /aep-autopilot)
```

`/aep-reflect` is the **human-in-the-loop** classifier you run after shipping;
`/aep-watch` is its **always-on** sibling — same logic, no human prompting each
finding — the thing that makes the loop _continuous_. It feeds the **same
`stories` section `/aep-dispatch` reads**, so discovered work re-enters the
`/aep-envision → /aep-map → /aep-dispatch → … → /aep-wrap → /aep-reflect` cycle.

**Session:** Main workspace only (like `/aep-autopilot`) — respects the orchestrator boundary.
**Driver:** `/loop <interval>` (Claude Code) or `codex exec` cron/launchd (Codex).
**Input:** Sources configured in `topology.routing.watch`.
**Output:** New `bug` / `refinement` stories appended to the `stories` section of `product-context.yaml` (or surfaced as proposals for confirmation — see Config).

---

## STOP — Orchestrator Boundary

`/aep-watch` runs from the **main workspace only** and is an **orchestrator**, not
an executor. Like `/aep-autopilot`, it never reads, reviews, edits, or evaluates
**workspace code**. It reads only:

- the configured sources (via their APIs/feeds — see Step 1),
- `product-context.yaml` (to dedupe and to write stories).

If a finding needs code investigation, that happens inside a **workspace agent**
after the story is dispatched — never in the watch session.

```bash
# Main workspace guard
pwd | grep -q '.feature-workspaces' && echo "ABORT: Run /aep-watch from main workspace only" && exit 1
[ -f product-context.yaml ] || echo "ABORT: Run /aep-envision and /aep-map first"
```

Any worker `/aep-watch` spawns (e.g. a cheap CHECK delegate to fetch + classify a
batch) is a **`native-bg-subagent`** on Claude Code, gated by the standard
**post-spawn liveness probe** (`scripts/spawn-liveness-probe.sh`): confirm the
agent exists AND shows activity before counting it; on failure, tear down and
fall back to `native-bg-subagent`. The watch session itself does **not** read
workspace code.

---

## Config

Watch is driven entirely by `topology.routing.watch` in `product-context.yaml`.
Each `sources[]` entry names a source `type` (its adapter and finding shape live
in `references/telemetry-ingestion.md` — do not invent new source types here):

```yaml
topology:
  routing:
    full_auto: false # A1 master switch (see below)
    watch:
      sources: # source types + adapters: references/telemetry-ingestion.md
        - type: bug_tracker # github_issues | linear | jira | sentry | datadog | log_stream
          query: "is:open label:bug"
        - type: error_stream
          dsn: "<sentry/rollbar/...>"
        - type: telemetry
          metric: "error_rate"
          threshold: 0.02
        - type: dogfood_report # dogfood findings (local / post-deploy / standalone)
          glob: ".dev-workflow/dogfood-*.md" # default; see telemetry-ingestion.md adapter
        - type: distillation # layer distillations (proposal-only synthesis from /aep-wrap)
          glob: "lessons-learned/distillations/*.yaml" # default; see telemetry-ingestion.md adapter
      interval: 30m # poll cadence for the /loop or cron driver
      auto_create: false # write stories directly vs. surface proposals
      since: null # high-water mark — last ingested timestamp (watch maintains this)
```

**Confirmation policy (conservative by default):** auto-create a story only when
`full_auto: true` (A1 master switch) **OR** `watch.auto_create: true` (per-watch
opt-in, narrower than the master switch). Otherwise **surface a proposal**: write
the story object to a `watch_proposals` block under `topology.routing.watch` and
print it; nothing enters the `stories` section until a human approves (via
`/aep-reflect` or inline). Under `full_auto: true`, watch writes straight into
`stories` and `/aep-dispatch` / `/aep-autopilot` pick them up on the next tick.
**When in doubt, surface** — recreating noise as stories is worse than a prompt.

---

## The Watch Loop

Each tick runs the same four-step body. **Idempotent** — re-running with no new
source data produces no new stories (the dedupe + `since` high-water mark guarantee it).

```
⓪ PRECHECK  → verify the /aep-map telemetry binding is complete (coverage_check)
① PULL      → fetch new findings from each configured source (since high-water mark)
② CLASSIFY  → run each finding through the /aep-reflect Step 2 classifier
③ DEDUPE    → drop findings that already map to an existing story
④ WRITE     → create bug/refinement stories (or surface proposals)
```

### Step 0: Precondition — verify the map binding

`/aep-watch` consumes telemetry sources, so first confirm `/aep-map` actually
**bound** them — don't silently watch nothing. Run `coverage_check()` (the helper
in `references/telemetry-ingestion.md` §1.5) over the signals this watch needs:
each `topology.routing.watch.sources[]` entry (and any `metric`/`error_stream` it
relies on) must resolve to a wired `topology.routing.telemetry_sources` entry with
a `metric_map`.

- **Covered** → proceed to Step 1.
- **Not covered** (sources empty, or a referenced metric has no `metric_map`) →
  **do not claim auto-coverage.** Surface:
  `"telemetry binding incomplete for <missing> — run /aep-map (Telemetry Binding step) before /aep-watch can ingest it"`, skip the uncovered sources, and (if nothing is covered) stop the tick with that message. A missing binding **blocks**; it never silently no-ops.

**Postcondition:** every covered source resolves to a wired `telemetry_sources`
entry with a `metric_map` (file-glob sources `dogfood_report`/`distillation` are
self-describing and exempt), or the tick surfaced the incomplete-binding message.

### Step 1: Pull from Sources

For each entry in `watch.sources`, pull findings created/updated since
`watch.since`, reducing each to the **finding record** and using the per-source
adapters in `references/telemetry-ingestion.md` (→ The `/aep-watch` finding
record; Dogfood-report adapter; Distillation adapter) — do not invent a new
finding shape. Advance `watch.since` to the newest `last_seen` **only after** the
tick completes successfully (a failed tick re-pulls rather than dropping findings).

- **File-glob sources (`dogfood_report`, `distillation`)** carry no per-item
  timestamp, so `watch.since` does **not** advance for them; re-scanning the glob
  each tick is harmless because Step 3 dedupes on each adapter's stable
  `external_id` (`dogfood:<report>:<hash>` / `distillation:<layer>:<hash>`).
  Being self-describing, neither is gated by Step 0's `coverage_check`.
- **`distillation` extra rule:** items mapped to `process` (`skill_amendments`)
  are **never** auto-created as stories — they surface to a human as proposed
  amendments regardless of `full_auto`/`auto_create`.

**Postcondition:** each configured source yielded zero or more finding records;
`watch.since` is unchanged until the tick completes successfully.

### Step 2: Classify Each Finding

Classify every finding with the **exact same classifier as `/aep-reflect` Step 2**
(bug / refinement / discovery / opportunity shift / process) — apply
`/aep-reflect` "Classify Each Observation" (the canonical 5-category logic; if it
changes, it changes there).

Watch acts autonomously on only the **two** categories it can safely turn into
work: **bug** → a bug story (Step 4), **refinement** → a refinement story in the
next layer (Step 4). **Discovery, opportunity shift, calibration, and process**
findings are **never** auto-created regardless of `full_auto` — they change
product intent or workflow, so they always surface to a human (via `/aep-reflect`;
opportunity shifts escalate because they change the bet).

### Step 3: Dedupe Against Existing Stories

Before creating anything, check the finding against the current `stories` section
of `product-context.yaml` (and existing `watch_proposals`). Skip a finding when:

- a story already records this `source` + `external_id` (watch stamps
  `watch_origin: { source, external_id }` on every story it creates), **or**
- an open story's `title`/description clearly covers the same issue
  (same error signature, same endpoint, same metric).

If a matching story is `completed`/`closed` and the issue has **recurred** (new
occurrences after `completed_at`), do not silently recreate — add a note and
surface as a regression for human attention. Never recreate work.

**Postcondition:** every surviving finding has no matching open story and no prior
`watch_origin.{source, external_id}`.

### Step 4: Write Stories (or Surface Proposals)

For each surviving **bug** / **refinement** finding, build a story:

```yaml
- id: "watch-<source>-<external_id>"
  title: "<finding title>"
  description: "<finding detail> (auto-discovered by /aep-watch from <source>)"
  type: bug # or refinement
  status: pending
  priority: high # bugs: high; tune by count/severity (see below)
  layer: <active_layer> # bug → current layer; refinement → next layer
  module: <best-effort or unset> # leave unset if the source doesn't localize it
  watch_origin:
    source: "<source>"
    external_id: "<external_id>"
    discovered_at: "<ISO8601>"
```

**Priority / layer rules (mirror `/aep-reflect`):**

- **Bug** → `priority: high`, `status: pending`, in the **current/active layer**
  (escalate to `critical` when `count` or severity is high, e.g. crash affecting
  many users / error_rate over threshold).
- **Refinement** → `status: pending` in the **next layer**.
- Leave `module` / `files_affected` unset when the source can't localize them;
  dispatch's readiness score routes these through `/aep-design` first.

**Then, per the confirmation policy (Config):**

- **Auto-create** (`full_auto: true` OR `watch.auto_create: true`): append the
  story to the `stories` section — a normal pending story `/aep-dispatch` scores
  and `/aep-autopilot` picks up on the next tick.
- **Surface** (default): append the story object to
  `topology.routing.watch.watch_proposals` and print it; a human runs
  `/aep-reflect` (or confirms inline) to promote proposals into `stories`.

**Validate + commit** (same guardrails as reflect/dispatch — run the validation
command in `references/yaml-guardrails.md`):

```bash
npx js-yaml product-context.yaml > /dev/null && echo "YAML OK"
# Resolve $BASE (integration branch) per /aep-git-ref "Integration Branch".
git pull --ff-only origin "$BASE"
git add product-context.yaml
git commit -m "chore: watch — auto-discovered N stories from <sources>"
git push origin "$BASE"
```

Append a `changelog` entry (`type: watch`) summarizing findings ingested,
classified, deduped, and created vs. proposed. **Postcondition:**
`npx js-yaml product-context.yaml` exits 0 and the commit is pushed to `$BASE`.

---

## Driver

`/aep-watch` is a continuous/scheduled monitor — the same driver matrix as
`/aep-autopilot` (executor `detect()` + the driver × backend matrix in the
`/aep-executor` backends reference):

- **Claude Code — `/loop <interval>`** (long-lived, in-session):

  ```
  /loop 30m /aep-watch tick
  ```

  Use `watch.interval` for `<interval>`. The session stays alive, so any spawned
  CHECK delegate is a session-bound **native-bg-subagent**.

- **Codex — `codex exec` cron/launchd** (ephemeral, OS-scheduled): schedule
  `/aep-watch tick` externally (e.g. `launchd` `StartInterval`, cron, or a
  `while … sleep` loop), one cheap one-shot per tick. Workers must be OS-bound
  (codex-exec). AEP prints the snippet; it does not install the scheduler.

`/aep-watch tick` runs one pass of the four-step loop and exits. `/aep-watch stop`
cancels the driver (`/loop` cancel, or remove the cron/launchd job).

---

## Cross-References

- `/aep-reflect` — **Step 2 classifier** (bug / refinement / discovery / …),
  reused here; the human-in-the-loop counterpart to watch.
- `references/telemetry-ingestion.md` — the finding record + per-source adapters
  used by Step 1 (shared with `/aep-reflect` Step 1).
- `/aep-dispatch` — consumes the stories watch creates (scoring, readiness, WIP).
- `/aep-autopilot` — the orchestrator pattern, driver matrix, liveness probe, and
  main-workspace boundary watch mirrors; picks up watch-created stories next tick.
