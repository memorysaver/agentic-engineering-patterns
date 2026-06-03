---
name: aep-launch
description: Spawn an autonomous workspace session for feature implementation. Use after /design is complete, or when the user says "launch workspace", "start building", "spawn agent", "send it to build". Creates a git worktree on a feature branch, starts a Claude Code session in tmux/cmux, and optionally sets up a separate evaluator agent. Followed by /build (which runs autonomously in the workspace).
---

# Launch

Spawn an autonomous workspace session to implement a feature. Creates a git worktree on a fresh feature branch, bootstraps an implementation agent through the **executor abstraction** (`aep-executor`) — which picks the right backend for the current host (Claude Code or Codex; tmux/cmux, native subagent, or dynamic workflow) — and optionally sets up a separate evaluator agent for quality assurance.

> **Host-agnostic:** This skill no longer hardwires `claude` + tmux + cmux. It delegates spawning and presentation to `aep-executor`, which detects the host and selects a backend (B1–B4). Read `.claude/skills/aep-executor/references/backends.md` before spawning. The recipes shown below are the **session backend (B1/B2)** path — the common case; the executor reference covers the native-subagent (B3) and dynamic-workflow (B4) paths.

**Where this fits:**

```
/onboard → /scaffold → [ /design → /launch → /build → /wrap ]
                                    ▲ you are here
```

**Session:** Main session, automated
**Input:** OpenSpec change name (from `/design` or `/dispatch` for well-specified stories)
**Output:** Running workspace session with bootstrapped agent + optional evaluator

---

## Guardrails Before Launch

### 1. Verify working copy is clean

```bash
git status --porcelain
```

**If any files are modified or staged — ABORT.** Commit them first (`git add <files> && git commit -m "..."`) or stash them with `git stash`.

### 2. Verify dispatch commit is pushed to remote

```bash
git fetch origin
git log --oneline origin/main..main
```

**If any unpushed commits appear — ABORT.** The dispatch commit (YAML updates + OpenSpec changes) must be on the remote before launching workspaces. Without this, workspace branches base off a `main` that doesn't include the dispatch commit, and the OpenSpec files won't be visible inside the worktree.

Push if needed: `git push origin main`

### 3. Verify calibration context for `.5` layer stories

If the story belongs to a `.5` layer (0.5, 1.5, 2.5) or has `calibration_type` set:

```bash
# Check for calibration artifact (new path first, then legacy)
type="${calibration_type:-visual-design}"
[ -f "calibration/${type}.yaml" ] || [ -f design-context.yaml ] && echo "calibration context exists" || echo "MISSING"
```

**If the calibration artifact does not exist — ABORT.** The user must run `/calibrate <type>` first. Agents dispatched without calibration context will reproduce the same generic output that created the need for alignment in the first place.

### 4. Clean up orphan worktree/branch from prior failed launches

Git worktree, unlike jj's `jj workspace forget`, does not auto-clean if a previous `/launch` died mid-flight. Two failure modes can block re-launch — both are silent and confusing on first encounter. Run these idempotent checks before `git worktree add`:

```bash
# Check 1: orphan branch with no unmerged work → safe to delete
if git show-ref --verify --quiet refs/heads/feat/<name>; then
  ahead=$(git rev-list --count main..feat/<name> 2>/dev/null || echo 0)
  if [ "$ahead" = "0" ]; then
    echo "Removing orphan branch feat/<name> (no commits ahead of main)"
    git branch -D feat/<name>
  else
    echo "ABORT: feat/<name> has $ahead unmerged commit(s). Investigate before re-launching."
    echo "  - If the work is salvageable: git checkout feat/<name> && finish manually"
    echo "  - If the work is abandoned:   git branch -D feat/<name> && retry /launch"
    exit 1
  fi
fi

# Check 2: orphan worktree registration → prune
if [ -d ".git/worktrees/<name>" ] && [ ! -d ".feature-workspaces/<name>" ]; then
  echo "Pruning orphan worktree registration for <name>"
  git worktree prune
fi
```

The branch deletion is gated on `ahead == 0` so live workspaces are never affected — if the orphan branch has unmerged commits, abort and let the user investigate. The worktree prune is gated on the working directory being missing, so it only fires after a manual `rm -rf` of the worktree dir.

---

## Launch Workspace — `executor.spawn()`

> **Important:** Workspaces must live **outside** `.claude/` — Claude Code treats everything under
> `.claude/` as sensitive and blocks file writes with permission prompts, even with `--dangerously-skip-permissions`.

### 1. Detect the backend

Run the detection recipe from `.claude/skills/aep-executor/references/backends.md`
and **announce the selection** (e.g. "tmux + cmux present → session backend B1").
If the user said "…with workflow" and the host is Claude Code, select B4 and
follow the dynamic-workflow recipe in the executor reference instead of the steps
below.

### 2. Spawn (session backend B1/B2 recipe)

```bash
# Create the git worktree on a fresh feature branch (outside .claude/ to avoid sensitive path protection)
mkdir -p .feature-workspaces
git worktree add -b feat/<name> .feature-workspaces/<name> main

# Start the implementation agent in a tmux session. $EXECUTOR is the INTERACTIVE
# session command from detect():
#   claude → "claude --dangerously-skip-permissions"            (interactive; NO -p, NO --rc)
#   codex  → "codex --dangerously-bypass-approvals-and-sandbox" (interactive TUI; NOT `codex exec`)
[ -z "$EXECUTOR" ] && { echo "run detect() first — \$EXECUTOR unset (would launch a bare shell)"; exit 1; }
tmux new-session -d -s <name> -c .feature-workspaces/<name> "$EXECUTOR"

# cmux is OPTIONAL — only create a tab if we are actually INSIDE a cmux surface.
# (Merely having cmux installed does not let us spawn a sibling tab → would fail.)
if [ -n "$CMUX_SOCKET" ]; then
  GEN_SURFACE=$(cmux new-surface --type terminal | grep -o 'surface:[0-9]*')   # B1
  cmux send --surface "$GEN_SURFACE" "tmux attach -t <name>\n"
  cmux rename-tab --surface "$GEN_SURFACE" "<name>"
else
  echo "No cmux surface — workspace runs in tmux session '<name>'. Watch it: tmux attach -t <name>"  # B2
fi
```

> **No tmux at all (Desktop → B3) / dynamic workflow (B4):** do **not** use the
> block above. Follow the B3/B4 `spawn()` recipe in the executor reference. For
> B3, tell the user up front: the build runs to completion with no live monitor
> or mid-flight feedback in this host.

Replace `<name>` with a short feature name (e.g., `add-auth`). The branch will be `feat/add-auth`.

> **Note:** Add `.feature-workspaces/` to your project's `.gitignore` — worktree directories are ephemeral and should not be tracked.

> **Disk note:** Each worktree shares `.git/objects` with the main repo (no history duplication), but the working tree itself is duplicated. Budget ~working-tree-size per active workspace.

---

## Send Bootstrap Prompt

**Session backends (B1/B2):** wait for the agent to fully initialize, then send
the bootstrap instruction. The ready signal is executor-specific: Claude Code
shows a `❯` prompt; the Codex TUI has no `❯`, so fall back to a short timed wait.
**For B3 (native subagent):** the bootstrap text _is_ the subagent's initial
prompt — pass it at `spawn()` time, there is no separate send step. **For B4
(workflow):** the bootstrap text goes into the build agent's prompt in the
workflow script.

> **Skill prefix:** If your project syncs skills with a prefix (e.g., `aep-`), replace `/build` with the prefixed name (e.g., `/aep-build`). Check how the build skill is registered in your project's `.claude/skills/` directory.

```bash
# Wait for readiness. $READY_GREP comes from detect() ('❯' for claude, empty for codex).
if [ -n "$READY_GREP" ]; then
  for _ in $(seq 1 12); do
    tmux capture-pane -t <name>:0 -p -S -5 | grep -q "$READY_GREP" && { echo "ready"; break; }; sleep 2
  done
else
  sleep 8   # codex TUI has no ❯ glyph — give the composer time to come up
fi
```

### Inject Prior Lessons (if available)

Before sending the bootstrap prompt, check for relevant lessons from previous builds:

```bash
# Read lessons matching this story's module or activity
ls lessons-learned/*.md 2>/dev/null
ls lessons-learned/process/*.md 2>/dev/null
```

If relevant lessons exist (matching the story's `module` or `activity`), append a `## Prior Lessons` section to the bootstrap prompt with a summary of relevant entries. Cap at 2000 tokens to avoid context bloat. Also include any relevant process lessons from `lessons-learned/process/*.md` (these apply to all builds, not just module-specific ones).

```bash
# Send the bootstrap prompt — same text, backend-aware delivery (executor.spawn finishes here).
# NOTE: Replace /build with your project's build skill name (e.g., /aep-build)
PROMPT="/build execute implementation for openspec change <change-name>. Read the worktree-onboarding reference in the build skill's references/worktree-onboarding.md for full setup instructions. Design phases are pre-completed on main.

## Prior Lessons
<relevant lessons summary, if any — omit this section if no lessons exist>
"

if [ -n "$GEN_SURFACE" ]; then
  cmux send --surface "$GEN_SURFACE" "$PROMPT"        # B1 (cmux sends the whole string)
else
  # B2: -l sends the literal multi-line text; a separate Enter submits it ONCE.
  # (A bare `send-keys "$PROMPT" Enter` would let embedded newlines submit it line-by-line.)
  tmux send-keys -t <name>:0.0 -l -- "$PROMPT"
  tmux send-keys -t <name>:0.0 Enter
fi
# B3/B4: $PROMPT was passed as the agent's initial prompt at spawn() — nothing to send here.
```

---

## Optional: Evaluator Mode (Full Mode)

For complex features, a separate evaluator agent independently reviews the generator's work. This is the single most impactful improvement for agent output quality.

> **Key design decision:** The evaluator is **not spawned at launch time**. It is spawned by the
> generator at Phase 5, after implementation is complete. Anthropic's research shows evaluation
> should be sequential — build first, then evaluate — not concurrent.
>
> **Source:** [Harness Design for Long-Running Application Development](https://www.anthropic.com/engineering/harness-design-long-running-apps) — Anthropic Engineering

### Why Separate Evaluation

When asked to evaluate their own work, agents consistently rate it positively — even when quality is mediocre. Separating generation from evaluation (inspired by GANs) dramatically improves output quality:

- The **generator** focuses on building features
- The **evaluator** focuses on finding problems
- Separation makes it easy to calibrate the evaluator toward **skepticism**

### When to Use

| Use evaluator                              | Skip evaluator                   |
| ------------------------------------------ | -------------------------------- |
| Complex features with 3+ tasks             | Single-file config changes       |
| UI-heavy work (forms, dashboards, layouts) | Simple CRUD endpoints            |
| Auth, payments, or security-sensitive work | Documentation updates            |
| Features at the edge of model capability   | Bug fixes with clear repro steps |
| Multi-component integrations               | Dependency upgrades              |

**Rule of thumb:** If the feature has 3+ tasks in `tasks.md` or touches UI, use an evaluator.

### Brainstorm Evaluation Criteria (at launch time)

Before the generator starts, brainstorm **project-specific** scoring criteria with the user. The criteria are written to `.dev-workflow/evaluator-criteria.md` so they're ready when the generator reaches Phase 5.

#### a. Read the OpenSpec change

```bash
cat openspec/changes/<change-name>/proposal.md
cat openspec/changes/<change-name>/design.md
ls openspec/changes/<change-name>/specs/
cat openspec/changes/<change-name>/tasks.md
```

#### b. Identify the feature type

| Feature type           | Signals                                            |
| ---------------------- | -------------------------------------------------- |
| **UI-heavy**           | Forms, dashboards, layouts, user-facing pages      |
| **API-only**           | Endpoints, services, integrations, no frontend     |
| **Security-sensitive** | Auth, payments, data handling, permissions         |
| **Data pipeline**      | ETL, migrations, batch processing, data transforms |
| **Mixed**              | Full-stack features spanning multiple categories   |

#### c. Propose dimensions

Read the dimension presets in the gen-eval utility skill at `.claude/skills/aep-gen-eval/references/scoring-framework.md` (Dimension Presets section). Based on the feature type, propose:

- Which default dimensions to **keep** (Completeness, Correctness, UX, Security, Code Quality)
- Which to **drop** or de-weight
- Which to **add** (Originality, Accessibility, API Design, Performance, Data Integrity, etc.)
- Which to **weight heavily** — these are where the model tends to fall short

#### d. Ask the user

Present the proposed dimensions and ask:

1. **Which dimensions matter most** for this specific feature?
2. **What does "good" look like** — any concrete quality bars?
3. **Where have you seen mediocre output** from the model before on similar work?
4. **Any hard failure conditions** beyond the defaults?

#### e. Generate project-specific criteria

Write `.dev-workflow/evaluator-criteria.md` (per-workspace, not the default reference) with:

- The agreed-upon dimensions with weights
- Scale definitions tailored to this feature
- Hard failure thresholds reflecting what the user cares about
- Few-shot examples adapted from the defaults in `.claude/skills/aep-gen-eval/references/scoring-framework.md`

> **Skip brainstorming?** If the user wants to move fast, fall back to the default criteria at `.claude/skills/aep-gen-eval/references/scoring-framework.md`. But note that task-specific calibration significantly improves evaluator judgment.

### How the Evaluator Loop Works (Phase 5)

The generator self-orchestrates the evaluation loop at Phase 5. **You do not need to spawn the evaluator manually.** On a session backend (B1/B2) the generator uses `tmux split-window` to create a bottom pane with a new evaluator instance (under B1 the cmux surface displays both panes automatically). Under B3/B4 the evaluator is a sibling subagent / verify stage instead — `/build` Phase 5 picks the matching eval-protocol execution context via `executor.spawn_evaluator()`:

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│   Generator (building/fixing)                       │
│   Phase 0-4: full tab / Phase 5+: top half          │
│                                                     │
├─────────────────────────────────────────────────────┤
│                                                     │
│   Evaluator (spawned at Phase 5, bottom half)       │
│                                                     │
└─────────────────────────────────────────────────────┘
```

The full loop is documented in the build skill's Phase 5. In summary:

1. Generator writes `eval-request.md` → spawns evaluator in bottom pane
2. Evaluator evaluates → writes `eval-response-<N>.md`
3. Generator reads response → fixes issues → closes evaluator pane
4. Repeat until pass (max 5 rounds)

### Evaluator Bootstrap Prompt Template

The generator sends this when spawning the evaluator (for reference — the build skill handles this automatically):

```
You are an EVALUATOR agent. Begin evaluation immediately.

Read these files:
1. .dev-workflow/evaluator-criteria.md (scoring calibration)
2. .dev-workflow/signals/eval-request.md (what to evaluate)
3. All files in openspec/changes/<change-name>/
4. .dev-workflow/contracts.md (if exists)
5. .dev-workflow/feature-verification.json (if exists)

Then:
1. Review code changes via `git diff main...HEAD`
2. Test the running application if possible
3. Score each dimension per your criteria
4. Write structured feedback to .dev-workflow/signals/eval-response-<N>.md

CRITICAL: Score honestly. Do not rationalize problems away.
Apply hard failure thresholds strictly.
Never modify verification_steps in feature-verification.json.
```

---

## Monitoring Workspace Progress

The main session can check workspace progress without interrupting the agent:

```bash
# Check current phase and progress
cat .feature-workspaces/<name>/.dev-workflow/signals/status.json

# Check if ready for human review
ls .feature-workspaces/<name>/.dev-workflow/signals/ready-for-review.flag 2>/dev/null

# Send mid-flight feedback
cat >> .feature-workspaces/<name>/.dev-workflow/signals/feedback.md << 'EOF'

## <date> <time>
Priority: high
<feedback here>
EOF
```

---

## Managing Parallel Sessions

The main workspace stays on `main` and can:

- Launch multiple workspace sessions (one per feature)
- See all sessions as named cmux tabs **(B1)**, or list them with `tmux ls` and attach by name **(B2)**
- Switch between sessions by clicking tabs (B1) or `tmux attach -t <name>` (B2)
- Handle `/wrap` after each PR merges

> Under B3/B4 there are no separate sessions to switch between — progress is read
> from signals (B3) or the `/workflows` view (B4).

Each worktree gets its own working tree on its own `feat/<name>` branch. They share `.git/objects` so history isn't duplicated, but each working tree adds its own checkout-size to disk.

---

## Next Step

The workspace agent is now running autonomously. It follows the build skill to implement, test, and merge the feature.

When the PR merges, run the wrap skill:

```
/wrap
```

> If using a prefix (e.g., `aep-`), run `/aep-wrap` instead.
