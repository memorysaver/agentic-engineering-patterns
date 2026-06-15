# Goal-Driven Autopilot Driver

> **Superseded in part (2026-06):** `claude-team` was removed (silent agent-teams spawn failure on Claude Code ≥ 2.1.x) and replaced by `native-bg-subagent` as the Claude Code default, with a mandatory post-spawn liveness probe. See [`remove-claude-team.md`](./remove-claude-team.md).

> Decision record (2026-06-11). Adds a **goal-driven driver** as the default way
> `/aep-autopilot` keeps itself ticking, alongside the existing fixed-interval
> `/loop` driver (retained as a fallback). This changes only the **driver** — how
> the next tick is triggered and when autopilot stops. The 7-step CHECK→ACT tick,
> the delegated cheap CHECK, the signals protocol, the executor abstraction, and
> the orchestrator boundary are all unchanged.

---

## 1. The Problem

Autopilot's real objective is goal-shaped — _"complete the current layer"_ — but
it was expressed as a **fixed-interval poll loop** (`/loop 5m /aep-autopilot
tick`). The 5-minute interval is not intrinsic to the design; it is just a
throttle on polling the workspace `signals/` files, and termination was ad-hoc
("all layers complete → stop"). Two consequences:

- **Idle ticks.** A layer with nothing to do still wakes every 5 minutes.
- **No native stop.** The loop runs forever until a human stops it; there is no
  first-class "this layer is done, hand back" signal.

Both hosts now ship a native primitive for exactly this shape.

## 2. Key Research Findings (2026-06, verified against live runtimes)

1. **Both Claude Code and Codex ship a `/goal` primitive**, with near-identical
   UX and semantics:
   - **Claude Code `/goal`** (v2.1.139+): a session-scoped completion condition.
     After each turn a small fast model (Haiku) judges the condition **against
     the conversation transcript**; "no" auto-starts the next turn, "yes" clears
     the goal. It is a wrapper around a session-scoped prompt-based Stop hook. The
     evaluator **cannot run tools** — it judges only what the turn surfaced.
     Headless via `claude -p "/goal …"`.
   - **Codex `goals`** (experimental flag `goals=true`, confirmed in
     `codex features list` on CLI 0.130): a **persisted thread-level goal**
     (SQLite state machine with statuses `active / paused / budget_limited /
complete`, events `thread/goal/updated|cleared`, re-check `Goal unmet (…)`).
     First-class **`token_budget`** → on exhaustion a soft stop to
     `budget_limited` with a wrap-up steer. Continues only when the thread is
     **idle**, the goal is active, and within budget. Controls: `/goal`,
     `/goal check|pause|resume|clear`.
2. **Both are turn/idle-driven, not time-interval-driven** — the next turn fires
   the instant the previous one ends (CC) or the thread goes idle (Codex). This
   is the opposite of `/loop`'s fixed cadence.
3. **A raw foreground `sleep` is blocked inside a Claude Code turn**; the
   sanctioned bounded wait is the `Monitor` tool with a hard timeout. Codex has
   no such restriction.
4. **Background-completion push notifications are unreliable cross-platform**
   (Claude Code issue #21048), so an event-driven "wake on worker completion"
   design cannot be the foundation.

## 3. The Decision

Add `executor`-level concept of a **goal driver** and make it the autopilot
default; keep the **loop driver** as a fallback selected by `--loop <interval>`.

### 3.1 Driver selection

- No `--loop` flag → **goal driver** (default).
- `--loop <interval>` → **loop driver** (the prior behavior, unchanged).
- `--floor <dur>` (goal driver only) → per-tick wait floor, default `5m`.
- `--max-turns <n>` (goal driver only) → runaway backstop, default `200`.

### 3.2 Scope — one layer per goal

A single `/goal` run drives the **current layer** to completion, then stops and
hands back to the human to run the layer gate / `/aep-reflect` and re-invoke
`/aep-autopilot` for the next layer. This maps 1:1 onto AEP's existing
pause-at-gate boundaries (layer gates, `.5` calibration layers, outcome
contracts) but replaces the infinite loop with crisp termination.

### 3.3 Goal condition

The skill builds the condition for layer `N`:

> Layer `N` is COMPLETE — every story is `status=completed` and its worktree is
> wrapped (none remain under `.feature-workspaces/`), per the surfaced AUTOPILOT
> status line — **OR** autopilot has entered `status=paused` requiring human
> input. Judge **only** from the surfaced status line. Each turn run exactly one
> `/aep-autopilot tick` then end the turn. Stop after `max-turns` turns.

The two stop conditions (`layer_complete` / `paused`) are exactly the conditions
that today end or pause the loop, so pause/escalation semantics are unchanged:
hard-pause escalations (design ambiguity, layer-gate failure, outcome contract,
repeated failure) flip `paused=true` and the goal stops; a single relayed
human-gate does **not** halt the layer (other workspaces keep progressing).

### 3.4 The three adaptations to the tick

The tick body is unchanged; the goal driver adds a tail to step ⑦ (skipped under
the loop driver):

1. **Surface a signals-only status line** into the transcript so the evaluator
   can judge `layer_complete` / `paused`. It contains no workspace code or file
   contents, so the **orchestrator boundary holds** — the evaluator never reads
   code.
2. **Wait the per-tick floor** before ending the turn (CC `Monitor`-with-timeout;
   Codex `sleep`). This is mandatory: without it `/goal` re-fires instantly and
   hot-loops. Early-wake on a signal change is an allowed optimization, not a
   correctness dependency — the timeout alone guarantees a stable cadence.
3. **Keep the delegated cheap CHECK** — the goal session is long-lived, so the
   token-isolation reason for running the read-heavy CHECK in a Haiku subagent /
   `codex exec` one-shot still holds.

### 3.5 Driver × backend compatibility

The goal driver is a **long-lived in-session** driver, so it is compatible with
every steerable mode (claude-team / claude-bg / codex-subagent / codex-exec /
legacy) — the same row as `/loop`. It **cannot** drive a fresh-session-per-tick
cron/launchd scheduler; fully-unattended OS-scheduled runs therefore stay on the
`/loop` / `codex exec` path. This is why the loop driver is retained, not removed.

## 4. Why reliability over cleverness

The chosen wait mechanism is a **bounded wait with a hard timeout floor**, not
push notifications or event-watch APIs. Rationale: fewest moving parts, no
silent cross-platform failure modes (#21048), and a cadence **behaviorally
identical to the `/loop 5m` users already trust** — the only change is
self-termination. "Slow is fine; stable matters more."

## 5. Alternatives Considered

- **Push-when-available + watch fallback.** Pure event-driven, zero idle turns,
  but two code paths and an unreliable push channel (#21048). Rejected for
  reliability.
- **Replace `/loop` entirely.** Rejected: `/goal` is in-session-only, so the
  cron/launchd unattended path needs `/loop`/`codex exec`. Keeping both also de-
  risks hosts on older CLI versions without `/goal`.
- **Whole-backlog goal scope.** More autonomy, but it must pause at every
  outcome-contract / gate boundary anyway; per-layer scope gives cleaner
  termination and matches existing pause points.

## 6. Consequences

- Autopilot stops on its own at the layer boundary; no more idle 5-minute ticks
  within a quiet layer (it still waits the floor, but never loops past `done`).
- Identical command surface (`/aep-autopilot`); the default driver changes
  underneath. `--loop` restores the exact prior behavior.
- New invariant: the per-tick status line must stay signals-only. Enforced by
  guardrail in the autopilot skill.
- Codex requires the `goals` feature enabled (`--enable goals`); Claude Code
  requires v2.1.139+. Hosts lacking `/goal` fall back to `--loop`.
