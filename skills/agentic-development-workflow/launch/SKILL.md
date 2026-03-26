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
**Input:** OpenSpec change name (from `/design`)
**Output:** Running workspace session with bootstrapped agent + optional evaluator

---

## Guardrail: Verify Main is Clean

```bash
jj st
```

**If any files are modified — ABORT.** Describe and create a new change first (`jj describe -m "..." && jj new`).

---

## Launch Workspace

```bash
# 1. Create the jj workspace
jj workspace add .claude/workspaces/<name>

# 2. Start Claude Code in a tmux session
tmux new-session -d -s <name> \
  -c .claude/workspaces/<name> \
  "claude --dangerously-skip-permissions --rc"

# 3. Create a cmux tab and attach
SURFACE_REF=$(cmux new-surface --type terminal | grep -o 'surface:[0-9]*')
cmux send --surface "$SURFACE_REF" "tmux attach -t <name>\n"
cmux rename-tab --surface "$SURFACE_REF" "<name>"
```

Replace `<name>` with a short feature name (e.g., `add-auth`).

---

## Send Bootstrap Prompt

After the cmux tab is attached and Claude Code is ready, send the bootstrap instruction:

```bash
cmux send --surface "$SURFACE_REF" "/build execute implementation for openspec change <change-name>. Read the worktree-onboarding reference at skills/agentic-development-workflow/build/references/worktree-onboarding.md for full setup instructions. Design phases are pre-completed on main.
"
```

---

## Optional: Set Up Evaluator Agent (Full Mode)

For complex features, set up a separate evaluator agent that independently reviews the generator's work. This is the single most impactful improvement for agent output quality.

### Why Separate Evaluation

When asked to evaluate their own work, agents consistently rate it positively — even when quality is mediocre. Anthropic's research found that separating generation from evaluation (inspired by GANs) dramatically improves output quality:

- The **generator** focuses on building features
- The **evaluator** focuses on finding problems
- Separation makes it easy to calibrate the evaluator toward **skepticism**

> "When asked to evaluate work they've produced, agents tend to respond by confidently praising the work — even when, to a human observer, the quality is obviously mediocre."
>
> **Source:** [Harness Design for Long-Running Application Development](https://www.anthropic.com/engineering/harness-design-long-running-apps) — Anthropic Engineering

### When to Use

| Use evaluator | Skip evaluator |
|--------------|---------------|
| Complex features with 3+ tasks | Single-file config changes |
| UI-heavy work (forms, dashboards, layouts) | Simple CRUD endpoints |
| Auth, payments, or security-sensitive work | Documentation updates |
| Features at the edge of model capability | Bug fixes with clear repro steps |
| Multi-component integrations | Dependency upgrades |

**Rule of thumb:** If the feature has 3+ tasks in `tasks.md` or touches UI, use an evaluator.

### Step 0: Brainstorm Evaluation Criteria

Before spawning the evaluator, brainstorm **project-specific** scoring criteria with the user. Generic criteria miss what matters; task-specific criteria catch the right problems.

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

### Step 1: Spawn the Evaluator

Create a second cmux tab in the same tmux session:

```bash
# Create a new cmux tab for the evaluator
EVAL_SURFACE=$(cmux new-surface --type terminal | grep -o 'surface:[0-9]*')
cmux send --surface "$EVAL_SURFACE" "tmux new-window -t <session-name> -n evaluator -c .claude/workspaces/<name>\n"

# Start Claude Code in the evaluator tab
cmux send --surface "$EVAL_SURFACE" "claude --dangerously-skip-permissions --rc\n"

# Rename the tab
cmux rename-tab --surface "$EVAL_SURFACE" "evaluator-<name>"
```

### Step 2: Send Evaluator Bootstrap Prompt

```bash
cmux send --surface "$EVAL_SURFACE" "You are an EVALUATOR agent. Your job is to critically evaluate the generator's work, not to write code.

Start by reading these files:
1. .dev-workflow/evaluator-criteria.md (your scoring calibration — brainstormed for this feature)
   If this file does not exist, fall back to: skills/agentic-development-workflow/launch/references/evaluator-criteria.md
2. All files in openspec/changes/<change-name>/ (proposal, design, specs, tasks)
3. .dev-workflow/contracts.md (success criteria per task)
4. .dev-workflow/feature-verification.json (verification checklist)

After reading, check .dev-workflow/signals/eval-request.md — if it exists, begin your evaluation. If it does not exist yet, the generator is still implementing. Check again at phase boundaries or when prompted.

Follow the evaluation protocol in your criteria file. Write your response to .dev-workflow/signals/eval-response-1.md.

CRITICAL: Score honestly. Do not rationalize problems away. Apply hard failure thresholds strictly. Never modify verification_steps in feature-verification.json.
"
```

### Step 3: Notify the Generator

```bash
cmux send --surface "$GEN_SURFACE" "An evaluator agent is now running in a separate tab. After you complete Phase 4 implementation:
1. Write .dev-workflow/signals/eval-request.md describing what to evaluate
2. Wait for .dev-workflow/signals/eval-response-1.md
3. Read the eval response and fix any FAIL items
4. Write a new eval-request for round 2
5. Repeat until the evaluator passes all thresholds
"
```

### Evaluator Cleanup

After the evaluation loop completes and all phases pass:

```bash
tmux kill-window -t <session-name>:evaluator
```

---

## Monitoring Workspace Progress

The main session can check workspace progress without interrupting the agent:

```bash
# Check current phase and progress
cat .claude/workspaces/<name>/.dev-workflow/signals/status.json

# Check if ready for human review
ls .claude/workspaces/<name>/.dev-workflow/signals/ready-for-review.flag 2>/dev/null

# Send mid-flight feedback
cat >> .claude/workspaces/<name>/.dev-workflow/signals/feedback.md << 'EOF'

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
