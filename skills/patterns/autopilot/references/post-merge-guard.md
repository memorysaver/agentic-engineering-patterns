# Post-Merge Guard Protocol

The post-merge monitoring window that runs **after** a story is merged and wrapped. Today autopilot wraps a merged story and forgets it; this guard keeps watching the deployed result for a bounded window, runs the host-aware dogfood against the live environment, and — only when explicitly enabled — can revert a hard service regression. It is the safety net that makes unattended autonomy survivable: the difference between "merged and walked away" and "merged, verified the deploy is healthy, and rolled back if it wasn't".

> **BOUNDARY REMINDER:** This step is an **orchestrator** action, identical in posture to the rest of the tick. It reads CI/health signals, reads dogfood reports, and runs `gh` / deploy / CLI commands — it **NEVER** reads workspace source code, **NEVER** spawns reviewers or evaluators from main, and **NEVER** forms code-quality opinions. The dogfood itself runs via `dogfood_method()` (see `dogfood-validation.md`) using the host's native browser tooling, producing a signals-only report the orchestrator consumes. See SKILL.md "STOP — Orchestrator Boundaries".

---

## Where this runs

The guard is a **post-deploy step that runs after Step ③ wrap** in the [tick protocol](./tick-protocol.md#step--wrap-completed-workspaces). When a story is merged (④a detects `MERGED`) and wrapped (③ removes its worktree), the story is **not** forgotten: its `guard_state` is opened and subsequent ticks drive it through the monitoring window below until the window closes (healthy) or fires (regression / dogfood issue).

```
③ wrap completed  →  open guard_state for the merged story
                      │
                      ▼
  ┌─ POST-MERGE GUARD (per merged story, across ticks) ───────────────┐
  │  1. trigger/await deploy        (deploy_status: pending→deploying  │
  │                                   →deployed | failed)              │
  │  2. open monitoring window      (window_min, default 15)          │
  │  3. each tick within window:                                      │
  │       • read health_signals (CI / error-rate / health endpoint)   │
  │       • run host-aware dogfood against target_url(staging|prod)    │
  │  4. classify findings → ONE of two issue paths (below)            │
  │  5. window elapsed, all green → close guard_state (healthy)       │
  └───────────────────────────────────────────────────────────────────┘
```

The guard never blocks dispatch — Steps ④/⑤/⑥ continue normally for in-flight workspaces while a merged story's window is open. The guard is signals-only and adds no per-tick workspace-code reads, so the orchestrator boundary and the `<60s` tick budget hold.

---

## Step PG.1: Trigger / Await Deploy

After wrap, advance the merged story's deploy lifecycle. The host-native deploy trigger is project-specific; the guard treats it as a CLI/CI signal, never as code:

- **CI-driven deploy** (most projects): the merge to the integration branch already triggered the pipeline. Poll status:
  ```bash
  gh run list --branch "$BASE" --limit 1 --json status,conclusion,databaseId
  gh run view <id> --json status,conclusion,jobs --jq '.status,.conclusion'
  ```
- **Explicit deploy**: if the project declares a deploy command/workflow, dispatch it once and record the run id, then poll as above.

Set `guard_state.deploy_status` accordingly: `pending` → `deploying` → `deployed` (CI success + deploy URL resolvable) or `failed`. A **failed deploy** is itself a hard regression — go straight to the [auto-revert / escalate path](#path-2-hard-service-regression--auto_revert-policy).

The monitoring window (PG.2/PG.3) opens only once `deploy_status == "deployed"`. Until then the guard waits across ticks (idempotent — see [state](#state--idempotency)).

---

## Step PG.2: Open the Monitoring Window

Once deployed, open a window of `topology.routing.post_merge_guard.window_min` minutes (default **15**). Record `window_opened_at`. Each subsequent tick that falls inside the window runs PG.3; once `now > window_opened_at + window_min` with no firing condition met, the window closes and the guard records the story **healthy** and clears its `guard_state`.

---

## Step PG.3: Watch Health Signals + Run Host-Aware Dogfood

Within the open window, each tick performs two independent reads:

### (a) Health signals

Read every signal named in `topology.routing.post_merge_guard.health_signals`. These are service-level, signals-only probes — no workspace code.

> **Coverage precondition.** Run `coverage_check(health_signals)` (`../../../product-context/reflect/references/telemetry-ingestion.md` §1.5) first: a signal like `error_rate` / `latency_p95` that needs a metrics source must be **bound** (the `/aep-map` Telemetry Binding step wired a `telemetry_sources` entry / `health_url`). An **unbound** signal is reported as "telemetry binding incomplete — run /aep-map", **not** treated as green — never infer health from a signal you can't actually read. (`ci_status` / `health_endpoint` / `smoke_check` are self-describing and need no binding.)

| Signal kind       | How the orchestrator reads it (examples)                                             |
| ----------------- | ------------------------------------------------------------------------------------ |
| `ci_status`       | `gh run view <id> --json status,conclusion` for the post-merge pipeline              |
| `health_endpoint` | `curl -fsS --max-time 5 <health_url>` (e.g. `/healthz`, `/readyz`) → expect 2xx      |
| `error_rate`      | query the project's metrics/log source for error-rate over the window vs. a baseline |
| `latency_p95`     | same source — p95 latency vs. baseline threshold                                     |
| `smoke_check`     | a declared CLI/API smoke command exiting 0                                           |

A signal is **red** when it fails its declared threshold (non-2xx health, CI `failure`, error-rate above baseline + margin, etc.). One transient red is not a regression — require the red to persist across **2 consecutive ticks** (or match a declared confirm rule) before treating it as confirmed, to avoid reverting on a deploy-warmup blip.

### (b) Host-aware dogfood

Run the dogfood validation against the deployed environment:

```
method = dogfood_method()                       # host × mode detection (see dogfood-validation.md)
url    = target_url(post_deploy_env)             # staging | production, from deploy_targets / CI
run dogfood(method, url) → report (severity/category/repro, signals-only)
```

`post_deploy_env` comes from `topology.routing.dogfood.post_deploy_env` (`staging` | `production` | `none`). `target_url()` resolves config-first then CI fallback (see `dogfood-validation.md`). The dogfood report uses the unified `/agent-browser:dogfood` severity/category/repro template, so the downstream classifier is host-agnostic.

---

## Step PG.4: Two Issue Paths — Kept Strictly Separate

The design fixes two **distinct** failure shapes (`g4-dogfood-validation-design.md` → "發現問題時的行為"). Do not conflate them: a dogfood UX finding is **never** a revert, and a service regression is **never** a new backlog story.

### Path 1: Dogfood-found UX / functional issues → create story (NOT a revert)

The deploy is healthy at the service level, but the dogfood surfaced a UX or functional defect (broken flow, visual regression, wrong copy, dead link). This is feedback, not an outage.

- Feed the dogfood report to the **`/aep-reflect` classifier** via the **`dogfood_report` adapter** (`../../../product-context/_shared/references/telemetry-ingestion.md` → Dogfood-report adapter), which classifies severity/category and **auto-creates a bug/refinement story** in `product-context.yaml` (links the G6 self-feeding loop).
- **Stamp `watch_origin: {source: dogfood, external_id: <adapter key>}`** on each story you file, using the adapter's deterministic `external_id`. This is the **same** dedupe key `/aep-watch`'s `dogfood_report` source uses, so if watch also ingests the report neither path double-files — whichever runs first wins and the other no-ops (see the adapter's "No high-water mark — dedupe-only").
- The new story enters the normal dispatch queue — Step ⑥ picks it up on a later tick by `readiness_score`.
- **Never revert** for a Path-1 finding. The merged change stays; the fix ships as its own story.
- Record `guard_state.dogfood = {report_path, issues_created:[story_ids]}`.

### Path 2: Hard service regression → `auto_revert` policy

A health signal is **confirmed red** (or the deploy failed). The deployed service is degraded — users are affected now. Behavior is governed by `topology.routing.post_merge_guard.auto_revert`:

> **DEFAULT IS CONSERVATIVE — `auto_revert: false`.** With auto-revert off (the default), the guard **warns and escalates only**: it adds a `post_merge_regression` escalation, pauses if the story is on the critical path, and waits for a human to confirm the revert. Automatic reverting is **opt-in** and presumes the architectural back-pressure below is in place.

- **`auto_revert: false` (default):** add escalation, do not touch the merge.
  ```json
  {
    "type": "post_merge_regression",
    "story_id": "<id>",
    "reason": "Health signal '<signal>' red for 2 consecutive ticks after merge of <pr>",
    "details": "<signal readings vs. baseline; deploy status>",
    "expected_human_action": "Investigate the deployed regression. If confirmed, revert with `gh pr revert <number>` (or revert the merge commit) and redeploy; then run /aep-reflect to log the incident.",
    "created_at": "<ISO8601>",
    "acknowledged": false
  }
  ```
- **`auto_revert: true` (opt-in) and regression confirmed:**
  1. **Revert** — `gh pr revert <number>` (opens/auto-merges a revert PR per repo policy) or revert the merge commit on `$BASE` and push. This is the **one** sanctioned exception to "never act on the merge" — it is a _recovery_ action, gated behind explicit opt-in, not a normal merge.
  2. **Record an incident** — write `.dev-workflow/incidents/<story_id>-<ISO8601>.md` (or append to `autopilot-history.jsonl` with `type: incident`): the red signals, readings, the reverted PR, and the deploy outcome.
  3. **Feed `/aep-reflect`** — hand the incident to the reflect classifier so the regression becomes a learning + a follow-up story (root-cause / guard hardening), closing the loop the same way Path 1 does for UX issues.
  4. Set `guard_state.reverted = true` so no later tick reverts the same story twice (see [state](#state--idempotency)).

---

## Architectural Back-Pressure (prerequisites for safe auto-revert)

`auto_revert: true` is only as safe as the scaffolding that makes a revert clean and a regression detectable. Document these as **prerequisites** the project should have before enabling auto-revert; they are scaffold-level recommendations, not steps the guard performs:

- **Pre-commit hooks** — lint/typecheck/format/secret-scan at commit time, so obviously-broken changes never reach the merge that the guard would have to revert.
- **Property-based tests** — broaden coverage beyond example-based tests so regressions are caught by signals (and by Phase 5 eval) rather than only in production.
- **Feature-flag / canary gating** — ship merged code dark or to a canary slice; a regression then degrades a fraction of traffic and a "revert" can be a flag flip, far safer and faster than a code revert.
- **Audit log** — append-only record of every guard action (deploy triggered, signals read, revert performed, incident filed) so auto-revert decisions are reconstructable and reviewable.

Without these, prefer the default `auto_revert: false` (warn + escalate). The guard should note in its escalation when prerequisites appear absent.

---

## Config

```yaml
topology:
  routing:
    post_merge_guard:
      window_min: 15 # monitoring window length, minutes (default 15)
      auto_revert: false # OPT-IN. false = warn + escalate only (conservative default)
      health_signals: # service-level, signals-only probes watched during the window
        - ci_status # post-merge pipeline conclusion
        - health_endpoint # 2xx from /healthz (URL from deploy_targets / CI)
        - error_rate # error-rate over window vs. baseline
        # - latency_p95
        # - smoke_check
```

Reuses `topology.routing.deploy_targets.{staging_url,production_url}` and `topology.routing.dogfood.{post_deploy_env,on_issue}` from the G4 dogfood design — the guard does not duplicate URL/method config.

---

## State & Idempotency

The guard records its progress **per merged story** so a re-fired tick never double-acts (double-deploys, double-reverts, double-files an incident). Add a `guard_state` entry keyed by `story_id` (alongside `workspaces` in `autopilot-state.json`; see `state-schema.md`):

```json
{
  "story_id": "PROJ-003",
  "pr_number": 412,
  "merged_at": "<ISO8601>",
  "deploy_status": "deployed", // pending | deploying | deployed | failed
  "window_opened_at": "<ISO8601>",
  "health": { "ci_status": "green", "health_endpoint": "green", "error_rate": "green" },
  "red_streak": { "error_rate": 0 }, // consecutive red ticks per signal (confirm rule)
  "dogfood": { "report_path": null, "issues_created": [] },
  "reverted": false,
  "incident_path": null,
  "last_action": "watching", // watching | dogfood_ran | story_created | escalated | reverted | closed
  "closed_at": null
}
```

Idempotency rules:

- **Deploy once** — only trigger a deploy if `deploy_status == "pending"`; otherwise poll.
- **Revert once** — never revert if `reverted == true`; the confirmed-red check is short-circuited once reverted.
- **One escalation per regression** — guard against duplicate `post_merge_regression` escalations for the same `story_id` while unacknowledged.
- **Close cleanly** — when the window elapses with all-green (or after Path-1 story creation / Path-2 revert + incident), set `last_action` accordingly, set `closed_at`, and drop the `guard_state` entry on the next tick.

---

## Cross-References

- [tick-protocol.md](./tick-protocol.md) — Step ③ wrap (the guard opens immediately after wrap); this guard is the new post-deploy step that runs across subsequent ticks.
- `dogfood-validation.md` — `dogfood_method()` host × mode detection, `target_url(env)` resolution, and the unified report format the guard consumes.
- `/aep-reflect` — the classifier both issue paths feed: Path 1 (UX/functional → new story) and Path 2 (incident → learning + follow-up story).
- [state-schema.md](./state-schema.md) — where `guard_state` lives in `autopilot-state.json`.
