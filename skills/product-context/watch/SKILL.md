---
name: aep-watch
description: Continuously ingest bug trackers, error streams, and telemetry; classify each finding with the /aep-reflect classifier; dedupe against existing stories; and auto-create bug/refinement stories into product-context.yaml so reflect→dispatch becomes a self-feeding loop. Use when the user says "watch", "monitor for new work", "ingest errors", "auto-create stories from telemetry", "keep an eye on the bug tracker", "/aep-watch", or wants new work to enter the backlog without manually running /aep-envision or /aep-reflect. Runs from the MAIN workspace only.
---

# Watch

Self-feeding work discovery. `/aep-watch` is a continuous/scheduled monitor that
**discovers** new work: it pulls from configured sources (bug trackers, error
streams, telemetry), classifies each finding with the **same classifier as
`/aep-reflect`**, dedupes against the existing backlog, and writes new
bug/refinement stories into `product-context.yaml`. Those stories then flow into
`/aep-dispatch` (or autopilot picks them up) — closing the loop so the system
keeps finding work to do without a human running `/aep-envision` or `/aep-reflect`
by hand.

```
sources → [ /aep-watch: pull → classify → dedupe → write stories ] → product-context.yaml
                                                                          │
                                                                          ▼
                                                          /aep-dispatch  (or /aep-autopilot)
```

`/aep-reflect` is the **human-in-the-loop** feedback classifier you run after
shipping. `/aep-watch` is its **always-on** sibling: same classification logic,
no human prompting each finding — it is what makes the loop _continuous_.

**Where this fits:**

```
/aep-envision → /aep-map → /aep-validate
  → /aep-watch  (continuous monitor — discovers + ingests new work)
  → /aep-dispatch → … → /aep-wrap → /aep-reflect → loop
       ▲ /aep-watch feeds the same stories section /aep-dispatch reads
```

**Session:** Main workspace only (like `/aep-autopilot`) — respects the orchestrator boundary.
**Driver:** `/loop <interval>` (Claude Code) or `codex exec` cron/launchd (Codex).
**Input:** Sources configured in `topology.routing.watch`.
**Output:** New `bug` / `refinement` stories appended to the `stories` section of `product-context.yaml` (or surfaced as proposals for confirmation — see Config).

---

## STOP — Orchestrator Boundary

`/aep-watch` runs from the **main workspace only** and is an **orchestrator**, not
an executor. Like `/aep-autopilot`, it never reads, reviews, edits, or evaluates
**workspace code**. It only reads:

- the configured sources (via their APIs/feeds — see Step 1),
- `product-context.yaml` (to dedupe and to write stories).

If a finding needs investigation that requires reading code, that happens inside
a **workspace agent** after the story is dispatched — never in the watch session.

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

Watch is driven entirely by `topology.routing.watch` in `product-context.yaml`:

```yaml
topology:
  routing:
    full_auto: false # A1 master switch (see below)
    watch:
      sources: # what to pull from — see references/telemetry-ingestion.md
        - type: bug_tracker # e.g. github_issues, linear, jira, sentry, datadog, log_stream
          query: "is:open label:bug"
        - type: error_stream
          dsn: "<sentry/rollbar/...>"
        - type: telemetry
          metric: "error_rate"
          threshold: 0.02
      interval: 30m # poll cadence for the /loop or cron driver
      auto_create: false # write stories directly vs. surface proposals
      since: null # high-water mark — last ingested timestamp (watch maintains this)
```

**Confirmation policy (default conservative):**

- **`full_auto: false` (default)** — watch **surfaces proposed stories** for human
  confirmation. It writes them to a `watch_proposals` block (under
  `topology.routing.watch`) and prints them; nothing enters the `stories` section
  until the human approves. `auto_create: true` lets watch write stories directly
  even when `full_auto` is off (a per-watch opt-in, narrower than the master switch).
- **`topology.routing.full_auto: true` (A1 master switch)** — watch **auto-creates
  AND lets dispatch run** without confirmation: it writes new stories straight into
  the `stories` section, and `/aep-dispatch` / `/aep-autopilot` pick them up on the
  next tick. No human gate per finding.

> **Resolution:** auto-create when `full_auto: true` **OR** `watch.auto_create: true`;
> otherwise surface proposals. When in doubt, surface — recreating noise as stories
> is worse than a confirmation prompt.

---

## The Watch Loop

Each tick runs the same four-step body. **Idempotent** — re-running with no new
source data produces no new stories (the dedupe + `since` high-water mark guarantee it).

```
① PULL      → fetch new findings from each configured source (since high-water mark)
② CLASSIFY  → run each finding through the /aep-reflect Step 2 classifier
③ DEDUPE    → drop findings that already map to an existing story
④ WRITE     → create bug/refinement stories (or surface proposals)
```

### Step 1: Pull from Sources

For each entry in `watch.sources`, pull findings created/updated since
`watch.since`. **Reuse the ingestion format and per-source adapters defined in
`references/telemetry-ingestion.md`** (the same source contract `/aep-reflect`
Step 1 draws on) — do not invent a new finding shape here. Each finding normalizes to:

```yaml
- source: "sentry"
  external_id: "ISSUE-4821" # stable id used for dedupe
  title: "TypeError in checkout flow"
  detail: "..." # stack/message/metric summary
  signal: error_stream # bug_tracker | error_stream | telemetry
  count: 142 # occurrences / affected users (priority input)
  first_seen: "<ISO8601>"
  last_seen: "<ISO8601>"
```

Advance `watch.since` to the newest `last_seen` only **after** the tick completes
successfully (so a failed tick re-pulls rather than dropping findings).

### Step 2: Classify Each Finding

Classify every finding using the **exact same classifier as `/aep-reflect`
Step 2** — bug / refinement / discovery / opportunity shift / process. **Do not
duplicate that logic here**; apply `/aep-reflect`'s "Classify Each Observation"
rules (see `../reflect/SKILL.md` → Step 2). Watch only acts autonomously on the
two categories it can safely turn into work:

| Classification            | Watch action                                                                 |
| ------------------------- | ---------------------------------------------------------------------------- |
| **Bug**                   | Create a bug story (Step 4).                                                 |
| **Refinement**            | Create a refinement story in the next layer (Step 4).                        |
| **Discovery**             | Do NOT auto-create. Surface for `/aep-reflect` → `/aep-envision`/`/aep-map`. |
| **Opportunity shift**     | Do NOT auto-create. Always escalate to a human — this changes the bet.       |
| **Process / Calibration** | Do NOT auto-create. Surface for `/aep-reflect`.                              |

Discoveries, opportunity shifts, calibrations, and process findings **always**
go to a human regardless of `full_auto` — they change product intent or workflow,
which watch must never decide autonomously.

### Step 3: Dedupe Against Existing Stories

Before creating anything, check the finding against the current `stories` section
of `product-context.yaml` (and existing `watch_proposals`). Skip a finding when:

- a story already records this `source` + `external_id` (watch stamps
  `watch_origin: { source, external_id }` on every story it creates), **or**
- an open story's `title`/description clearly covers the same issue
  (same error signature, same endpoint, same metric).

If a matching story exists but is `completed`/`closed` and the issue has
**recurred** (new occurrences after `completed_at`), do not silently recreate —
add a note and surface as a regression for human attention. Never recreate work.

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
  dispatch's readiness score will route these through `/aep-design` first.

**Then, per the confirmation policy:**

- **Auto-create** (`full_auto: true` OR `watch.auto_create: true`): append the
  story to the `stories` section. It is now a normal pending story —
  `/aep-dispatch` scores it and `/aep-autopilot` picks it up on the next tick.
- **Surface** (default): append the story object to `topology.routing.watch.watch_proposals`
  instead, and print it. The human runs `/aep-reflect` (or confirms inline) to
  promote proposals into `stories`.

**Validate + commit** (same guardrails as reflect/dispatch — see
`../reflect/references/yaml-guardrails.md`):

```bash
npx js-yaml product-context.yaml > /dev/null && echo "YAML OK"
# Resolve $BASE (integration branch): override → develop → main
BASE=$(git config --get aep.integration-branch 2>/dev/null || true)
[ -z "$BASE" ] && { git show-ref --verify --quiet refs/heads/develop \
  || git show-ref --verify --quiet refs/remotes/origin/develop; } && BASE=develop
BASE=${BASE:-main}
git pull --ff-only origin "$BASE"
git add product-context.yaml
git commit -m "chore: watch — auto-discovered N stories from <sources>"
git push origin "$BASE"
```

Append a `changelog` entry (`type: watch`) summarizing findings ingested,
classified, deduped, and created vs. proposed.

---

## Driver

`/aep-watch` is a continuous/scheduled monitor — the same driver matrix as
`/aep-autopilot` (executor `detect()` + the driver × backend matrix in
`.claude/skills/aep-executor/references/backends.md`):

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

## Guardrails

- **Main workspace only** — refuse to run if `pwd` contains `.feature-workspaces`.
- **Never read workspace code** — watch reads sources + `product-context.yaml` only;
  any code investigation happens inside a dispatched workspace agent.
- **Reuse, don't duplicate, the reflect classifier** — Step 2 applies
  `/aep-reflect` Step 2; if classification logic changes, it changes there.
- **Conservative by default** — surface proposals unless `full_auto: true` (A1)
  or `watch.auto_create: true`. When in doubt, surface.
- **Only bugs and refinements are auto-creatable** — discoveries, opportunity
  shifts, calibrations, and process findings always go to a human.
- **Always dedupe** — never recreate work that already has a story; stamp
  `watch_origin` so future ticks recognize it.
- **Spawned workers are native-bg-subagent + liveness probe** — never trust
  "state says active"; confirm via the probe, fall back on failure.
- **Advance the high-water mark only on success** — a failed tick re-pulls.

---

## Cross-References

- `../reflect/SKILL.md` — **Step 2 classifier** (bug / refinement / discovery / …),
  reused here verbatim; the human-in-the-loop counterpart to watch.
- `references/telemetry-ingestion.md` — source adapters + normalized finding format
  used by Step 1 (shared with `/aep-reflect` Step 1).
- `../dispatch/SKILL.md` — consumes the stories watch creates (scoring, readiness, WIP).
- `../../patterns/autopilot/SKILL.md` — the orchestrator pattern, driver matrix,
  liveness probe, and main-workspace boundary watch mirrors; autopilot picks up
  watch-created stories on its next tick.
