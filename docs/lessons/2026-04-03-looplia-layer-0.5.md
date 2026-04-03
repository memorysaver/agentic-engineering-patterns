# Lessons Learned: Looplia Run Layer 0.5 (UI Polish)

**Date:** 2026-04-03
**Project:** Looplia Run (remote shell execution platform)
**Layer:** 0.5 (UI Polish Pass)
**Stories:** 4 (landing page, auth polish, dashboard detail, E2E visual test)

## Process Lessons

### 1. /calibrate Phase 2 should modify real components, not produce reference HTML

**Problem:** `/calibrate capture` produced standalone HTML reference files that the user couldn't preview from the terminal. They had no way to see what the design system looked like until it was applied to real components.

**Fix:** After creating `design-context.yaml` and updating `globals.css`, apply the design system directly to the project's React components and use `agent-browser` to screenshot each page for visual verification. Present screenshots to the user for approval before committing.

**Impact:** Saves a full round-trip of "here are reference files" → "I can't see them" → "let me apply them to real code."

### 2. cmux send needs trailing newline for auto-submit

**Problem:** Bootstrap prompts sent to workspace agents via `cmux send` didn't get submitted — the message appeared in the Claude Code prompt but required the user to manually press Enter.

**Fix:** Always end `cmux send` messages with `\n` to simulate pressing Enter.

### 3. Workspace agents skip Phase 5 — autopilot quality gate catches this

**Observation:** Workspace agents sometimes jump from Phase 4 (implementation) directly to Phase 10 (PR creation), skipping Phase 5 (code review/gen-eval). The autopilot's Step ④b quality gate detects this (`phase >= 5 AND no eval-response with PASS`) and sends a tmux nudge to run Phase 5.

**Assessment:** This is working as designed. The safety net is valuable. No skill change needed — this is emergent behavior that the monitoring system handles correctly.

### 4. Stories implemented in main session bypass all quality gates

**Observation:** LR-P01 and LR-P02 were implemented directly during `/calibrate`, bypassing the workspace pipeline entirely — no gen/eval, no Phase 5, no evaluator, no PR review. The only validation was visual screenshots.

**Trade-off:** Speed vs quality assurance. For simple UI styling work where the user is actively reviewing screenshots, this is acceptable. For complex logic, it would be risky.

**Recommendation:** The `/calibrate` skill documentation should note this trade-off explicitly. If quality verification is needed for calibrate-session work, the user should request it.

### 5. Dogfood testing should be part of the layer gate

**Problem:** The Layer 0.5 gate test was purely visual: "Landing page renders, auth polished, dashboard shows detail." Screenshots pass this gate easily. But actual functional testing (sign up → register daemon → see it in dashboard) revealed two real bugs:

1. `keyring` crate missing `apple-native` feature — daemon couldn't store/read API keys on macOS
2. Daemon connection code used a non-existent `/ws/daemon` endpoint instead of the ticket-based flow

**Fix:** Layer gate tests should include functional verification, not just visual checks. At minimum: can a user complete the core flow end-to-end?

### 6. Rust `keyring` crate needs platform features

**Problem:** `keyring = "3.6"` without platform features (e.g., `apple-native` on macOS, `linux-native` on Linux) has no credential backend. The crate silently returns `NoEntry` for all reads, making it appear like no key is stored.

**Fix:** Always specify platform features: `keyring = { version = "3.6", features = ["apple-native"] }` on macOS.

**Recommendation:** Rust project scaffolding should include platform-specific dependency notes for common crates with required features.

## Workflow Observations

- **4 stories, 11 autopilot ticks** — from calibrate to layer gate passed
- **2 stories via main session** (calibrate), **2 via workspace agents** (autopilot)
- **1 gen/eval cycle** on LR-P03 (score: 4.0/5.0 PASS), **0 on LR-P04** (light mode)
- **0 failures, 0 retries** across all stories
- **Biggest time cost:** LR-P04 workspace agent took ~50 minutes for a test-only story (E2E visual test with agent-browser screenshots)
