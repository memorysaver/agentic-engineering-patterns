---
name: launch
description: Spawn an autonomous workspace session for feature implementation. Use after /design is complete, or when the user says "launch workspace", "start building", "spawn agent", "send it to build". Creates a jj workspace, starts a Claude Code session in tmux/cmux, and optionally sets up a separate evaluator agent. Followed by /build (which runs autonomously in the workspace).
---

# Launch

Spawn an autonomous workspace session to implement a feature. Creates a jj workspace, bootstraps a Claude Code agent via tmux/cmux, and optionally sets up a separate evaluator agent for quality assurance.

**Where this fits:**

```
/onboard → /scaffold → [ /design → /launch → /build → /wrap ]
                                    ▲ you are here
```

**Session:** Main session, automated
**Input:** OpenSpec change name (from `/design` or `/dispatch` for well-specified stories)
**Output:** Running workspace session with bootstrapped agent + optional evaluator

---

## Guardrail: Verify Main is Clean

```bash
jj st
```

**If any files are modified — ABORT.** Describe and create a new change first (`jj describe -m "..." && jj new`).

---

## Launch Workspace

> **Important:** Workspaces must live **outside** `.claude/` — Claude Code treats everything under
> `.claude/` as sensitive and blocks file writes with permission prompts, even with `--dangerously-skip-permissions`.

```bash
# 1. Create the jj workspace (outside .claude/ to avoid sensitive path protection)
mkdir -p .feature-workspaces
jj workspace add .feature-workspaces/<name>

# 2. Start Claude Code in a tmux session
tmux new-session -d -s <name> \
  -c .feature-workspaces/<name> \
  "claude --dangerously-skip-permissions --rc"

# 3. Create a cmux tab and attach to the generator window
GEN_SURFACE=$(cmux new-surface --type terminal | grep -o 'surface:[0-9]*')
cmux send --surface "$GEN_SURFACE" "tmux attach -t <name>\n"
cmux rename-tab --surface "$GEN_SURFACE" "<name>"
```

Replace `<name>` with a short feature name (e.g., `add-auth`).

> **Note:** Add `.feature-workspaces/` to your project's `.gitignore` — workspace directories are ephemeral and should not be committed.

---

## Send Bootstrap Prompt

Wait for Claude Code to fully initialize (look for the `❯` prompt in the tmux pane), then send the bootstrap instruction:

```bash
# Verify Claude Code is ready before sending (look for the prompt indicator)
sleep 5
tmux capture-pane -t <name>:0 -p -S -5 | grep -q '❯' && echo "ready"

# Send bootstrap prompt via cmux
cmux send --surface "$GEN_SURFACE" "/build execute implementation for openspec change <change-name>. Read the worktree-onboarding reference in the build skill's references/worktree-onboarding.md for full setup instructions. Design phases are pre-completed on main.
"
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

| Use evaluator | Skip evaluator |
|--------------|---------------|
| Complex features with 3+ tasks | Single-file config changes |
| UI-heavy work (forms, dashboards, layouts) | Simple CRUD endpoints |
| Auth, payments, or security-sensitive work | Documentation updates |
| Features at the edge of model capability | Bug fixes with clear repro steps |
| Multi-component integrations | Dependency upgrades |

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

| Feature type | Signals |
|-------------|---------|
| **UI-heavy** | Forms, dashboards, layouts, user-facing pages |
| **API-only** | Endpoints, services, integrations, no frontend |
| **Security-sensitive** | Auth, payments, data handling, permissions |
| **Data pipeline** | ETL, migrations, batch processing, data transforms |
| **Mixed** | Full-stack features spanning multiple categories |

#### c. Propose dimensions

Read the dimension presets in `references/evaluator-criteria.md` (bottom section). Based on the feature type, propose:

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
- Few-shot examples adapted from the defaults in `references/evaluator-criteria.md`

> **Skip brainstorming?** If the user wants to move fast, fall back to the default criteria at `references/evaluator-criteria.md`. But note that task-specific calibration significantly improves evaluator judgment.

### How the Evaluator Loop Works (Phase 5)

The generator self-orchestrates the evaluation loop at Phase 5. **You do not need to spawn the evaluator manually.** The generator uses `tmux split-window` to create a bottom pane with a new evaluator Claude Code instance (the cmux surface attached to the session displays both panes automatically):

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

The full loop is documented in `/build` Phase 5. In summary:

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
1. Review code changes via jj diff
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

- Launch multiple workspace sessions (one tab per feature)
- See all sessions as named cmux tabs
- Switch between sessions by clicking tabs
- Handle `/wrap` after each PR merges

Each workspace shares the underlying jj store — no extra disk space, no branch naming conflicts between agents.

---

## Next Step

The workspace agent is now running autonomously. It follows `/build` to implement, test, and merge the feature.

When the PR merges, run:

```
/wrap
```
