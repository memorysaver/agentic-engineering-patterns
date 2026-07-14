# Autopilot Drivers — Full Recipes

The **driver** is what keeps `/aep-autopilot` ticking and decides when it stops.
The SKILL's "Driver selector" picks one and states the compatibility rules; this
file holds the full per-host recipe. Both drivers keep the orchestrator
long-lived (so any steerable mode works) and run the **same** 7-step CHECK→ACT
tick body (`references/tick-protocol.md`) — only how the next tick is triggered
differs. The **goal driver** additionally self-terminates when the layer is done.
Rationale: `docs/decisions/goal-driven-autopilot.md`.

Under either driver, **keep delegating each tick's CHECK** to a cheap
context-isolated agent (Haiku subagent / `codex exec` one-shot) — the driver
session is long-lived, so the token-isolation reason from the SKILL's "Execution
model — CHECK → ACT" still holds.

---

## Goal driver (default)

Build the **goal condition** for the current layer and hand it to the host's
native `/goal` primitive. The condition is the success predicate the goal
evaluator judges each turn — against the **status line the tick surfaces** (step
⑦; signals only — never workspace code) — plus a one-line per-turn directive:

```
Layer <N> of this product is COMPLETE: every story in layer <N> is
status=completed AND its worktree has been wrapped (none remain under
.feature-workspaces/), as shown by the AUTOPILOT status line surfaced this
turn — OR autopilot has entered status=paused requiring human input (design
ambiguity, layer-gate failure, outcome contract, or repeated failure), as
shown by the same status line. Judge ONLY from the surfaced status line,
never from memory. Each turn, run exactly ONE `/aep-autopilot tick`, then end
the turn; never run more than one tick per turn. Stop after <max-turns> turns
if the layer has not completed.
```

### Claude Code — `/goal` (requires v2.1.139+)

```
/goal <the condition above>
```

`/goal` starts a turn immediately and, after each turn, a small fast model
(Haiku) checks the condition against the conversation: "no" auto-starts the next
turn with the evaluator's reason as guidance, "yes" clears the goal and stops.
Pair with auto mode so each turn runs unattended. The per-tick wait floor (tick
step ⑦) is what prevents hot-looping — without it the evaluator re-fires the
instant a turn ends. The evaluator reads only the surfaced status line, so the
orchestrator boundary holds (it never sees workspace code).

### Codex — `goals` feature (experimental — enable with `--enable goals`)

```
/goal <the condition above>
```

Set a `token_budget` as the hard runaway wall (on exhaustion Codex soft-stops to
`budget_limited` with a wrap-up steer rather than dying). Codex continues the goal
only when the thread is **idle**, the goal is active, and it is within budget;
`/goal pause` · `/goal resume` · `/goal check` · `/goal clear` manage it.

---

## Loop driver (fallback — `--loop <interval>`)

The fixed-interval driver, unchanged from earlier versions. Use it for hosts
without `/goal`, for fully-unattended **OS-scheduled** runs (cron/launchd — `/goal`
is in-session-only), or when you simply want a fixed cadence. The loop driver does
**not** self-terminate; stop it with `/aep-autopilot stop`.

### Claude Code — `/loop` (GA, in-session, long-lived)

```
/loop <interval> /aep-autopilot tick
```

`<interval>` comes from the `--loop` flag (default `5m`). `/loop` invokes
`/aep-autopilot tick` each interval; the tick keeps the main session cheap by
delegating its CHECK to a Haiku subagent. Like the goal driver it keeps the
session alive, so session-bound modes (**native-bg-subagent**) work under it.

### Codex — two loop options

- **In-thread (long-lived):** the orchestrator is a living main thread (desktop
  app or interactive CLI) that runs `/aep-autopilot tick` on a cadence (self-paced
  or user-prompted). Supports **codex-subagent** — workers are this thread's
  subagents, steerable via `send_input`, visible as threads.
- **Scheduled (ephemeral):** no native `/loop`; schedule `codex exec` externally
  — each tick is a fresh cheap one-shot (already context-isolated, so no nested
  CHECK needed). Workers must then be **codex-exec** (OS-bound; a fresh tick
  session steers them via `codex exec resume`). Recommended: a macOS `launchd`
  agent with `StartInterval=300` (or cron / a `while … sleep 300` loop) running:

  ```bash
  codex exec -m gpt-5.4-mini -c model_reasoning_effort=low \
    -C "$PWD" --skip-git-repo-check --dangerously-bypass-approvals-and-sandbox \
    "/aep-autopilot tick" < /dev/null    # < /dev/null: exec hangs on stdin otherwise
  ```

Tell the user the exact scheduler snippet for their platform; AEP does not install
it for them.
