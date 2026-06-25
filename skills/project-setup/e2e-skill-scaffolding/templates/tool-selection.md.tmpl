# Test-tool selection — `e2e_tool(target_type)`

The journeys in [`journeys/`](./journeys/) are **tool-agnostic** — they say *what* to verify, not *how*.
This file is the *how*: it resolves which browser/device automation tool drives the UI for a given
journey, by **target type** (web / mobile / desktop), the **host** running the agent, and any **pinned
preference** — with health probes and graceful degrade.

> **Kept in sync with `e2e_tool()` in AEP `patterns/executor/references/dogfood-validation.md`.** That
> upstream selector is the canonical matrix (used by `/aep-build` and `/aep-autopilot`); this file is its
> self-contained, project-local projection so the e2e-test skill works without AEP's pattern skills present.

## Resolution

```
e2e_tool(target_type):              # target_type ∈ {web, mobile, desktop}; from the journey's `target:` front-matter
  detect HOST (claude | codex | generic) and whether computer-use / desktop is available
  pref = topology.routing.e2e.tool.<target_type>   # product-context.yaml; optional pin
  if pref: return pref if healthy(pref) else degrade

  web:
    HOST == claude →  agent-browser  → webwright           → degrade
    HOST == codex  →  (desktop + computer-use) codex-native → playwright → agent-browser → degrade
    generic        →  playwright     → agent-browser        → degrade

  mobile:
    agent-device (iOS/Android)        → degrade (API/contract checks only)

  desktop:
    HOST == codex + computer-use → codex-native             → agent-browser (Electron/CDP) → degrade
    else                          → agent-browser (Electron/CDP) → degrade
```

## Tools

| Tool             | Track   | Covers                                   | Health probe                         | Source |
| ---------------- | ------- | ---------------------------------------- | ------------------------------------ | ------ |
| **agent-browser**| web/desktop | SPA nav, forms, multi-tab, Electron (CDP) | `agent-browser navigate about:blank` | [vercel-labs/agent-browser](https://github.com/vercel-labs/agent-browser) |
| **Playwright**   | web     | cross-browser scripts (Chromium/FF/WebKit) | `npx playwright --version`         | playwright.dev |
| **webwright**    | web     | web automation + accessibility checks    | `webwright --version`                | [microsoft/webwright](https://github.com/microsoft/webwright) |
| **codex-native** | web/desktop | in-app browser + computer-use (multimodal) | host = Codex desktop + computer-use | Codex built-in |
| **agent-device** | mobile  | iOS / Android native app automation      | `agent-device doctor` (or device list) | [callstack/agent-device](https://github.com/callstack/agent-device) |

## Health probes

Each tool gets a smoke test; a failed probe drops to the next fallback (not a hard FAIL):

```bash
agent_browser_healthy() { command -v agent-browser >/dev/null 2>&1 && agent-browser navigate about:blank >/tmp/ab-smoke.log 2>&1; }
playwright_available()  { command -v npx >/dev/null 2>&1 && npx --no-install playwright --version >/dev/null 2>&1; }
webwright_available()   { command -v webwright >/dev/null 2>&1 && webwright --version >/dev/null 2>&1; }
agent_device_healthy()  { command -v agent-device >/dev/null 2>&1 && agent-device doctor >/tmp/ad-smoke.log 2>&1; }
# codex-native: not a CLI probe — available only when HOST is Codex desktop with computer-use enabled.
```

## Target environment (from `policy.md`)

This file picks the *tool*; *which environment* the journey runs against is set by the project's
[`policy.md`](./policy.md) `dogfood_target` — don't assume local:

- `local` → `$BASE_URL` from `.dev-workflow/ports.env` (pre-merge).
- `deployed:<url>` → that URL (e.g. a Cloudflare prod/preview), typically post-deploy.
- `none` → no UI dogfood at all (this file isn't emitted for such projects).

So `target_type` (below) resolves the **tool**; `dogfood_target` (policy.md) resolves the **environment**.

## Target-type detection

The journey's `target:` front-matter is authoritative. When absent, infer from the stack:

| Stack signal                          | target_type |
| ------------------------------------- | ----------- |
| `native-uniwind` / React Native / Expo | `mobile`   |
| `tauri` / `electrobun` / Electron     | `desktop`   |
| anything else (web frontend)          | `web`       |

## Config pin

Pin a tool per target type in `product-context.yaml` (parallels `aep.executor-backend`):

```yaml
topology:
  routing:
    e2e:
      tool:
        web: auto       # auto | agent-browser | playwright | webwright | codex-native
        mobile: auto    # auto | agent-device
        desktop: auto   # auto | codex-native | agent-browser
```

`auto` (default) defers to `e2e_tool()`; an explicit value pins it (still subject to the health probe —
a pinned-but-unhealthy tool degrades).

## Degrade paths

When no tool in the track is healthy, **degrade — don't FAIL**:

- **web/desktop:** skip the UI step, mark `SKIP`, and verify via the API driver (Tier 3) where possible;
  for purely-visual checks, route to human eval.
- **mobile:** fall back to API/contract checks; mark UI steps `SKIP` with the reason.

A `SKIP` with a recorded reason is honest coverage; a silent pass is not.

## Unified report

Whatever tool ran, findings use the **same** structure so the downstream classifier never branches on
tool. Write to `.dev-workflow/dogfood-<feature>.md`, one entry per finding:

```markdown
## <finding title>

**Severity:** blocker | major | minor
**Category:** UX | logic | visual | edge-case | accessibility | performance
**Repro:** <ordered steps against the target>
**Observed:** <what happened> **Expected:** <what should happen>
**Evidence:** <screenshot path / log excerpt / API JSON>
```

Writing the report file is what makes a finding ingestible by `/aep-reflect` (bug → story; "feels wrong"
→ calibration). A dogfood that only prints to chat is a dead end.
