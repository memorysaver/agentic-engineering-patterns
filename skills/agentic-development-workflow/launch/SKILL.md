---
name: aep-launch
description: Spawn an autonomous workspace agent for feature implementation. Use after /aep-design is complete, or when the user says "launch workspace", "start building", "spawn agent", "send it to build". Creates a git worktree on a feature branch, starts the selected executor mode (Claude Code native background subagents or background sessions; Codex native subagents or exec workers; tmux only when pinned), and optionally sets up a separate evaluator agent. Followed by /aep-build (which runs autonomously in the workspace).
---

# Launch

Spawn an autonomous workspace agent to implement a feature. Creates a git
worktree on a fresh feature branch, bootstraps an implementation agent through
the **executor abstraction** (`aep-executor`) — which picks the right mode for
the current host — and optionally sets up a separate evaluator agent for
quality assurance.

> **Native-first:** This skill does not hardwire `claude` + tmux + cmux. It
> delegates spawning and presentation to `aep-executor`. Read
> `.claude/skills/aep-executor/references/backends.md` before spawning. On
> Claude Code the launch mode is **native-bg-subagent** (default — Agent tool
> `run_in_background`, no team) or **claude-bg** (native background sessions,
> where `claude --bg` exists); on Codex it is **codex-subagent** or
> **codex-exec**; **legacy** (tmux + optional cmux) runs only when explicitly
> pinned or on generic hosts. Every mode uses the same AEP-created worktree.
> (`claude-team` was removed — silent agent-teams spawn failure; see
> `aep-executor/references/backends.md`.)

**Where this fits:**

```
/aep-onboard → /aep-scaffold → [ /aep-design → /aep-launch → /aep-build → /aep-wrap ]
                                    ▲ you are here
```

**Session:** Main session, automated
**Input:** OpenSpec change name (from `/aep-design` or `/aep-dispatch` for well-specified stories)
**Output:** Running workspace agent with bootstrapped build + optional evaluator

---

## Guardrails Before Launch

### 1. Verify working copy is clean

```bash
git status --porcelain
```

**If any files are modified or staged — ABORT.** Commit them first (`git add <files> && git commit -m "..."`) or stash them with `git stash`.

### 2. Verify dispatch commit is pushed to remote

```bash
# Resolve $BASE (integration branch) — see git-ref "Integration Branch" (override → develop → main)
BASE=$(git config --get aep.integration-branch 2>/dev/null || true)
[ -z "$BASE" ] && { git show-ref --verify --quiet refs/heads/develop \
  || git show-ref --verify --quiet refs/remotes/origin/develop; } && BASE=develop
BASE=${BASE:-main}

git fetch origin
git log --oneline origin/"$BASE".."$BASE"
```

**If any unpushed commits appear — ABORT.** The dispatch commit (YAML updates + OpenSpec changes) must be on the remote before launching workspaces. Without this, workspace branches base off a `$BASE` that doesn't include the dispatch commit, and the OpenSpec files won't be visible inside the worktree.

Push if needed: `git push origin "$BASE"`

### 3. Verify calibration context for `.5` layer stories

If the story belongs to a `.5` layer (0.5, 1.5, 2.5) or has `calibration_type` set:

```bash
# Check for calibration artifact (new path first, then legacy)
type="${calibration_type:-visual-design}"
[ -f "calibration/${type}.yaml" ] || [ -f design-context.yaml ] && echo "calibration context exists" || echo "MISSING"
```

**If the calibration artifact does not exist — ABORT.** The user must run `/aep-calibrate <type>` first. Agents dispatched without calibration context will reproduce the same generic output that created the need for alignment in the first place.

### 4. Clean up orphan worktree/branch from prior failed launches

Git worktree, unlike jj's `jj workspace forget`, does not auto-clean if a previous `/aep-launch` died mid-flight. Two failure modes can block re-launch — both are silent and confusing on first encounter. Run these idempotent checks before `git worktree add`:

```bash
# Resolve $BASE (integration branch) — see git-ref "Integration Branch" (override → develop → main)
BASE=$(git config --get aep.integration-branch 2>/dev/null || true)
[ -z "$BASE" ] && { git show-ref --verify --quiet refs/heads/develop \
  || git show-ref --verify --quiet refs/remotes/origin/develop; } && BASE=develop
BASE=${BASE:-main}

# Check 1: orphan branch with no unmerged work → safe to delete
if git show-ref --verify --quiet refs/heads/feat/<name>; then
  ahead=$(git rev-list --count "$BASE"..feat/<name> 2>/dev/null || echo 0)
  if [ "$ahead" = "0" ]; then
    echo "Removing orphan branch feat/<name> (no commits ahead of $BASE)"
    git branch -D feat/<name>
  else
    echo "ABORT: feat/<name> has $ahead unmerged commit(s). Investigate before re-launching."
    echo "  - If the work is salvageable: git checkout feat/<name> && finish manually"
    echo "  - If the work is abandoned:   git branch -D feat/<name> && retry /aep-launch"
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

> **Orphan with a live worktree?** If `.feature-workspaces/<name>` exists with
> committed/in-progress work but no live agent (state says active, agent list
> says gone), do **not** delete anything — follow the **orphan re-adoption**
> protocol in `aep-executor/references/backends.md`: re-spawn a worker into the
> existing worktree with a recovery bootstrap.

---

## Step 1: Detect the Launch Mode

Run the detection recipe from `.claude/skills/aep-executor/references/backends.md`
and **announce the selection**, e.g.:

- "Claude Code → **native-bg-subagent**: in-process background subagent (Agent
  tool, `run_in_background`, no team); steer with `SendMessage(to: agentId)` /
  `feedback.md`; watch with `TaskOutput`."
- "Claude Code + `claude --bg` present, OS-bound need → **claude-bg**: native
  background session; watch with `claude attach <id>`."
- "Codex main thread → **codex-subagent**: native worker (aep-builder role);
  steer with send_input; threads visible in the app / `/agent`."
- "Pinned tmux → **legacy**: tmux session (+ cmux tab if available)."

If the user said "…with workflow" and the host is Claude Code, select
**workflow** and follow the dynamic-workflow recipe in the executor reference
instead of the steps below.

## Step 2: Create the Worktree (common to all modes)

> **Invariant — one launch = one worktree = one subagent = one story.** Each
> `/aep-launch` creates exactly one worktree and spawns exactly one worker (one
> `native-bg-subagent` on Claude Code) to build exactly one story. Do not put
> multiple stories into a single launch and do not spawn a second worker into a
> worktree that already has a live one. (The autopilot enforces the upstream half
> of this: **max ONE launch per tick** — see `aep-autopilot` tick protocol Step
> ⑥.) The one exception is a `compile_mode: grouped_change` story group, which is
> deliberately compiled into a single change/worktree/worker — see Step 4.

> **Important:** Workspaces must live **outside** `.claude/` — Claude Code
> treats everything under `.claude/` as sensitive and blocks file writes with
> permission prompts, even with `--dangerously-skip-permissions`.

```bash
# Resolve $BASE (integration branch) — see git-ref "Integration Branch" (override → develop → main)
BASE=$(git config --get aep.integration-branch 2>/dev/null || true)
[ -z "$BASE" ] && { git show-ref --verify --quiet refs/heads/develop \
  || git show-ref --verify --quiet refs/remotes/origin/develop; } && BASE=develop
BASE=${BASE:-main}

# Create the git worktree on a fresh feature branch (outside .claude/)
mkdir -p .feature-workspaces
git worktree add -b feat/<name> .feature-workspaces/<name> "$BASE"
```

Replace `<name>` with a short feature name (e.g., `add-auth`). The branch will be `feat/add-auth`.

> **Note:** Add `.feature-workspaces/` to your project's `.gitignore` — worktree directories are ephemeral and should not be tracked.

> **Disk note:** Each worktree shares `.git/objects` with the main repo (no history duplication), but the working tree itself is duplicated. Budget ~working-tree-size per active workspace.

## Step 3: Compose the Bootstrap Prompt

The bootstrap is the same text in every mode; only the delivery differs.

> **Skill naming:** AEP skills are canonically registered with the `aep-` prefix (`/aep-build`, `/aep-wrap`, …). If your project installed them under different names, check `.claude/skills/` and adjust the command in the bootstrap accordingly.

### Inject Prior Lessons (if available)

Before composing the bootstrap, check for relevant lessons from previous builds:

```bash
ls lessons-learned/*.md 2>/dev/null
ls lessons-learned/process/*.md 2>/dev/null
```

If relevant lessons exist (matching the story's `module` or `activity`), append a `## Prior Lessons` section to the bootstrap prompt with a summary of relevant entries. Cap at 2000 tokens to avoid context bloat. Also include any relevant process lessons from `lessons-learned/process/*.md` (these apply to all builds, not just module-specific ones).

```bash
# NOTE: /aep-build is the canonical name; adjust if your project registered the build skill differently
PROMPT="/aep-build execute implementation for openspec change <change-name>. Read the worktree-onboarding reference in the build skill's references/worktree-onboarding.md for full setup instructions. Design phases are pre-completed on the integration branch.

## Prior Lessons
<relevant lessons summary, if any — omit this section if no lessons exist>
"
```

## Step 4: Spawn — per mode

The bootstrap **is the spawn prompt** for every native mode; only `legacy`
has a separate send step. Full recipes live in the executor reference files —
this is the dispatch:

> **Post-spawn verification is mandatory — do NOT return "running" from a spawn
> until the liveness probe passes.** After spawning, run the
> [Post-Spawn Liveness Probe](../../patterns/executor/references/backends.md#post-spawn-liveness-probe)
> (`bash .claude/skills/aep-executor/scripts/spawn-liveness-probe.sh <name> <agent_id>`):
> the worker process/agent must exist **and** the worktree must show activity
> (status.json written or non-empty `git diff`) within N seconds. On failure,
> tear down the dead spawn (and `TeamDelete` any team that got created) and
> **auto-fall-back to `native-bg-subagent`** into the same worktree — never leave
> a silently-dead worktree for the autopilot to flag 30+ minutes later. "Roster
> says active" is not liveness.

### Mode: native-bg-subagent (`aep-executor/references/claude-native.md`) — Claude Code default

Pre-flight: if any agent-teams team is active, `TeamDelete` it first (a live team
re-routes teamless background spawns through the broken agent-teams backend). Then
spawn with the Agent tool: `run_in_background: true`, **no `team_name`**, prompt =
worktree contract (absolute path + "operate exclusively there") + `$PROMPT` + the
human-gate instructions. A working spawn returns a **bare-hex `agentId`** with a
JSONL `output_file` (not an `@<team>` id) — record it as `agent_id`. Then run the
liveness probe above.

### Mode: claude-bg (`aep-executor/references/claude-native.md`) — only if `claude --bg` exists

```bash
cd .feature-workspaces/<name> && claude --bg --dangerously-skip-permissions "$PROMPT"
cd - >/dev/null   # record the printed session id as agent_id; then run the liveness probe
```

> On Claude Code ≥ 2.1.x the `claude --bg` flag is absent (`BG_AVAILABLE=no`) —
> detection won't select this mode; native-bg-subagent is used instead.

### Mode: codex-subagent (`aep-executor/references/codex-native.md`)

`spawn_agent(agent_type: "aep-builder", message: "<abs worktree path> (branch feat/<name>). $PROMPT")`.
Record the agent id. Ensure `.codex/agents/aep-builder.toml` is committed in
the repo (see the role TOML in the reference; `/aep-onboard` installs it).

### Mode: codex-exec (`aep-executor/references/codex-native.md`)

Background `codex exec --cd .feature-workspaces/<name> ... "$PROMPT"`; record
the exec session id as `agent_id` (steerable later via `codex exec resume`).

### Mode: legacy (`aep-executor/references/tmux-session.md`)

`tmux new-session` in the worktree → readiness wait → `send-keys -l "$PROMPT"`

- `Enter` → then (cmux hosts only) attach the review tab as a sibling pane —
  the full recipe, ordering caveats included, is in the reference.

## Step 5: Present

Tell the user where to watch/steer, per mode:

| Mode               | Watch                                     | Steer                                                |
| ------------------ | ----------------------------------------- | ---------------------------------------------------- |
| native-bg-subagent | `TaskOutput <agentId>` / signals          | `SendMessage(to: agentId)` or `feedback.md`          |
| claude-bg          | `claude attach <id>` / `claude logs <id>` | attach; or `feedback.md`                             |
| codex-subagent     | app thread list / `/agent`                | open the thread; or `send_input` via the main thread |
| codex-exec         | signals + PR (headless)                   | `codex exec resume <id> "<msg>"`                     |
| legacy             | cmux tab / `tmux attach -t <name>`        | `tmux send-keys` or `feedback.md`                    |

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
cat openspec/changes/<change-name>/aep-design.md
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

The generator self-orchestrates the evaluation loop at Phase 5. **You do not
need to spawn the evaluator manually.** `/aep-build` Phase 5 picks the matching
spawn via `executor.spawn_evaluator()`:

| Generator mode                 | Evaluator spawn                                                                                                           |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------------- |
| native-bg-subagent / claude-bg | foreground Task subagent in the generator's context (inherits the worktree cwd; the evaluator prompt is the spawn prompt) |
| codex-subagent / codex-exec    | `codex exec --cd <abs worktree>` with the `aep-evaluator` role — enforced cwd, bounded one-shot                           |
| legacy                         | `tmux split-window` bottom pane (under cmux the surface shows both panes)                                                 |

The full loop is documented in the build skill's Phase 5. In summary:

1. Generator writes `eval-request.md` → spawns evaluator (worktree-bound)
2. Evaluator evaluates → writes `eval-response-<N>.md`
3. Generator reads response → fixes issues
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
1. Review code changes via `git diff "$(git config --get aep.integration-branch 2>/dev/null || (git show-ref --verify --quiet refs/remotes/origin/develop && echo develop || echo main))"...HEAD`
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

# Check if ready for human review / blocked on a human decision
ls .feature-workspaces/<name>/.dev-workflow/signals/ready-for-review.flag 2>/dev/null
ls .feature-workspaces/<name>/.dev-workflow/signals/needs-human.md 2>/dev/null

# Send mid-flight feedback (every mode reads this at phase boundaries)
cat >> .feature-workspaces/<name>/.dev-workflow/signals/feedback.md << 'EOF'

## <date> <time>
Priority: high
<feedback here>
EOF
```

If `needs-human.md` appears (or `status.json` shows `"blocked_on": "human"`),
the worker is waiting on a decision — answer it through the mode's transport
(see the Human-Gate Protocol in `aep-executor/references/backends.md`).

---

## Managing Parallel Sessions

The main session stays on the integration branch (`$BASE`) and can:

- Launch multiple workspace workers (one per feature)
- See them all: `TaskList` (**native-bg-subagent**), `claude agents --json`
  (**claude-bg**), the thread list / `list_agents` (**codex-subagent**),
  `tmux ls` (**legacy**)
- Switch into any worker: `TaskOutput <agentId>` / `claude attach <id>` / open
  its thread / `tmux attach -t <name>`
- Handle `/aep-wrap` after each PR merges

> Under **codex-exec**, **workflow**, and **headless** there is no live surface —
> progress is read from signals (+ the `/workflows` view for workflow mode).
> Human decisions are still covered: workers **gate-and-park** (record
> `needs-human.md`, end cleanly), you answer here in the main session, and the
> story resumes in its worktree with the answer.

Each worktree gets its own working tree on its own `feat/<name>` branch. They share `.git/objects` so history isn't duplicated, but each working tree adds its own checkout-size to disk.

---

## Next Step

The workspace agent is now running autonomously. It follows the build skill to implement, test, and merge the feature.

When the PR merges, run the wrap skill:

```
/aep-wrap
```
