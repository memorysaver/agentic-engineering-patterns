# Host-aware Dogfood Validation — `dogfood_method()` & `target_url()`

Dogfood/validation picks the right **native** method per host, both locally
(pre-merge, `/aep-build` Phase 6) and on staging/production (post-deploy). This
closes gap **G4b**: until now Phase 6 ran only against localhost and only if
`agent-browser` happened to be installed (else the whole phase was skipped), and
there was no post-deploy validation at all.

Detection reuses `executor.detect()` for HOST + mode — read
[`backends.md`](backends.md) first. The two functions here add a **method**
layer on top of that: which validation tool to drive (`dogfood_method()`) and
which URL to point it at (`target_url()`). All methods emit one unified report
format so the downstream classifier is host-agnostic.

---

## Table of Contents

1. [`dogfood_method()` — host × mode selection](#dogfood_method--host--mode-selection)
2. [`target_url(env)` — URL resolution](#target_urlenv--url-resolution)
3. [Unified report format](#unified-report-format)
4. [Config block](#config-block)
5. [Post-deploy worker & boundary (v1.8.0)](#post-deploy-worker--boundary-v180)
6. [Cross-references](#cross-references)

---

## `dogfood_method()` — host × mode selection

`executor.detect()` resolves HOST + mode; this adds a method probe on top. Each
host uses its **native** capability first and degrades only when that is
unavailable. `agent_browser_healthy()` (`agent-browser navigate about:blank`)
lives in `testing-guide`; `playwright_available()` is the analogous
write-and-run probe for Codex headless.

```
dogfood_method():
  resolve HOST + mode via executor.detect()

  if HOST == claude:                          # any mode
    if agent_browser_healthy():  return "agent-browser"      # /agent-browser:dogfood
    else:                        return "degrade"            # non-UI → API/curl; UI → human-eval

  if HOST == codex:
    if mode == codex-subagent and computer_use_enabled:      # desktop app
                                 return "codex-native"       # in-app browser + computer-use
    else:                                                    # codex-exec / headless
      if playwright_available():   return "playwright-script"  # GPT-5.4 writes + runs it
      elif agent_browser_healthy(): return "agent-browser"     # CLI fallback
      else:                        return "degrade"            # API checks
```

| Host / mode                        | Native method (default)                                   | Detection                      | Fallback                             |
| ---------------------------------- | --------------------------------------------------------- | ------------------------------ | ------------------------------------ |
| **Claude Code** (any mode)         | `/agent-browser:dogfood`                                  | `agent_browser_healthy()`      | non-UI → API/curl; UI → human-eval   |
| **Codex desktop** (codex-subagent) | native in-app browser + computer-use (GPT-5.4 multimodal) | desktop + computer-use enabled | Playwright skill → agent-browser CLI |
| **Codex headless** (codex-exec)    | write + run a Playwright script                           | `playwright_available()`       | agent-browser CLI → API checks       |

> **Why Codex splits two ways.** Computer-use and the in-app (Atlas) browser are
> **desktop-only**. `codex exec` (headless) has neither, so it writes and runs a
> Playwright script (GPT-5.4 does this natively) and falls back to the
> agent-browser CLI, then to API/curl checks.

---

## `target_url(env)` — URL resolution

```
target_url(env):                 # env ∈ {local, staging, production}
  if env == local:               # unchanged from current Phase 6
    source .dev-workflow/ports.env → return $BASE_URL
  else:
    u = topology.routing.deploy_targets.<env>_url   # product-context.yaml
    if u: return u                                  # config first
    else: return <CI/deploy step output URL>        # fallback CI (e.g. preview URL)
```

- **`env=local`** — source `.dev-workflow/ports.env`, return `$BASE_URL` (the
  Phase 6 status quo; `ports.env` is written by the workspace-setup hook).
- **`env=staging|production`** — read
  `topology.routing.deploy_targets.<env>_url` first; if unset, read the URL the
  CI/deploy step printed (e.g. a Vercel/Netlify preview URL or deploy output).

---

## Unified report format

Every method — `/agent-browser:dogfood`, `codex-native`, `playwright-script`,
the degrade paths — emits the **same** severity / category / repro structure as
`/agent-browser:dogfood`, so the downstream classifier never branches on host.
Reports are written to `.dev-workflow/dogfood-<feature>.md` (local) or the
post-deploy report path (staging/prod), one entry per finding:

```markdown
## <finding title>

**Severity:** blocker | major | minor
**Category:** UX | logic | visual | edge-case | accessibility | performance
**Repro:** <ordered steps to reproduce against the target URL>
**Observed:** <what happened> **Expected:** <what should happen>
**Evidence:** <screenshot path / log excerpt>
```

**On issue** → route per `topology.routing.dogfood.on_issue` (default
`create_story`): the report is ingested by the **`dogfood_report` adapter**
(`product-context/_shared/references/telemetry-ingestion.md` → Dogfood-report
adapter), which parses each `##` finding into a normalized record → the
`/aep-reflect` Step 2 classifier → a bug/refinement story in
`product-context.yaml` → dispatch (the G6 self-feeding loop). Set `escalate`
instead to surface to the human rather than auto-filing.

> **The report path is the contract.** Whatever the trigger — local Phase 6, the
> post-deploy post-merge guard, or a **standalone / ad-hoc** dogfood — write the
> unified report to `.dev-workflow/dogfood-*.md`. That is what makes the finding
> ingestible: `/aep-watch`'s `dogfood_report` source (or the guard's Path 1) picks
> it up on its next pass and runs it through the adapter above. A dogfood that
> only prints findings to chat (never writing the report file) is a **dead end** —
> nothing can auto-file it. Auto-creation still obeys the confirmation policy
> (`full_auto` / `watch.auto_create`); only **bug / refinement** auto-file, while
> calibration / discovery / opportunity-shift / process surface to a human.

> Hard service regressions (health signals) are a **separate** path — they go
> through the autopilot post-merge guard's revert policy, not this story-filing
> path. Dogfood finds UX/functional issues and files stories; the guard finds
> service regressions and decides rollback.

---

## Config block

Added under `topology.routing` in `product-context.yaml`:

```yaml
topology:
  routing:
    deploy_targets:
      staging_url: "https://staging.example.com" # optional; missing → fallback CI
      production_url: "https://example.com"
    dogfood:
      method: auto # auto | agent-browser | codex-native | playwright
      post_deploy_env: staging # staging | production | none
      on_issue: create_story # create_story | escalate
```

- **`method`** — `auto` (default) defers to `dogfood_method()`; the explicit
  values pin a method (parallels the `aep.executor-backend` pin).
- **`post_deploy_env`** — which environment the post-deploy step validates;
  `none` disables post-deploy dogfood.
- **`on_issue`** — `create_story` (default) or `escalate`.

---

## Post-deploy worker & boundary (v1.8.0)

When the post-deploy step needs a worker to run validation (e.g. a Codex
headless Playwright run, or a Claude `/agent-browser:dogfood` pass), it is
spawned as **`native-bg-subagent`** and confirmed live by the mandatory
**post-spawn liveness probe** before being treated as running — never trust a
flag or roster (see [`backends.md`](backends.md) → Post-Spawn Liveness Probe).

Screenshots captured by any method feed **each host's multimodal evaluator**:
Claude evaluates natively; Codex is confirmed multimodal (GPT-5.4). This keeps
the visual judgment in-host rather than crossing back to the orchestrator.

The **orchestrator boundary holds.** The post-deploy step reads reports and
signals and runs CLIs (`gh`, deploy tooling, `target_url` resolution) — it never
reads workspace code. The validation worker is bound to its worktree (or runs
against the deployed URL); the main session stays at arm's length, consistent
with the autopilot orchestrator boundary.

---

## Cross-references

- [`backends.md`](backends.md) — `executor.detect()` (HOST + mode), the
  `native-bg-subagent` default, and the Post-Spawn Liveness Probe.
- `agentic-development-workflow/build/SKILL.md` **Phase 6** — local (pre-merge)
  dogfood; calls `dogfood_method()` with `env=local` instead of skipping when
  agent-browser is absent.
- `patterns/autopilot/references/post-merge-guard.md` — the post-deploy step
  invokes `target_url(staging|production)` + `dogfood_method()` after merge +
  deploy; hard regressions go through the guard's revert policy.
- `product-context/reflect` (`/aep-reflect`) — the host-agnostic classifier that
  turns a unified dogfood report into a bug/refinement story.
