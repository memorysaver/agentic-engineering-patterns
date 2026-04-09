---
name: dispatch
description: Pick the next story to work on and bridge it into the feature lifecycle. Use when ready to start building, or when the user says "what's next", "dispatch", "pick a story", "start next feature", "what should I work on". Reads product-context.yaml, syncs workspace signals, computes readiness score for routing, then scores stories by business value + unblock potential + critical path urgency + reuse leverage (penalized by ambiguity + interface risk), assembles context, and hands off to /design or /launch. Supports batch dispatch with WIP limits. For autonomous orchestration, use /autopilot instead.
---

# Dispatch

Bridge between the product context (control plane) and the feature lifecycle (execution plane). Syncs workspace state, scores stories, picks what to build next, assembles context, and routes into `/design` or `/launch`.

**Where this fits:**

```
/envision → /map → /scaffold
  → [ /dispatch → /design → /launch → /build → /wrap ]
       ▲ you are here
  → /reflect → loop
```

**Session:** Main, interactive
**Input:** Product definition from `product/index.yaml` (split mode) or `product-context.yaml` (v1 mode); operational state from `product-context.yaml`
**Output:** OpenSpec change with pre-assembled context, story status updated, handoff to `/design` or `/launch`

> **For autonomous orchestration:** Use `/autopilot` instead. Autopilot runs the full dispatch-launch-monitor-review-wrap-dispatch cycle as a tick-based state machine via `/loop`. Dispatch remains a single-pass interactive tool.

---

## Before Starting

**File Resolution:**

```bash
ls product/index.yaml 2>/dev/null && echo "SPLIT MODE" || echo "V1 MODE"
cat product-context.yaml
```

- **Split mode** (`product/index.yaml` exists): Read product definition from `product/index.yaml` for context assembly. Read stories, topology, architecture, cost from `product-context.yaml`.
- **V1 mode**: Read everything from `product-context.yaml`.

If `product-context.yaml` doesn't exist, run `/envision` then `/map` first.
If the `stories` section is empty, run `/map` to decompose the product.

---

## The Dispatch Protocol

Every dispatch run follows the same 7-step protocol. Each step is idempotent — running `/dispatch` twice with no state changes produces the same result.

```
① SYNC signals    → bring YAML up to date with workspace reality
② CASCADE states  → compute all pending→ready, pending→blocked transitions
③ SCORE stories   → rank the ready queue by dispatch_score
④ PRESENT queue   → show user the scored dispatch queue
⑤ DISPATCH        → lock stories (status→in_progress), create OpenSpec changes
⑥ MONITOR         → agents work, main session watches signals
⑦ COMPLETE        → /wrap updates YAML, atomic cascade, re-invoke dispatch
```

---

## Step 1: Signal Sync

Before calculating anything, sync workspace signals into the YAML to reflect reality:

```
For each story with status: in_progress or in_review:
  workspace = story.assigned_to
  signal_path = .feature-workspaces/<workspace>/.dev-workflow/signals/status.json

  Read signal file (if exists):
    If signal.story_status == "completed":
      Update YAML: story.status → completed
      Update: story.completed_at = signal.completed_at
      Update: story.pr_url = signal.pr_url
      Update: story.cost_usd = signal.cost_usd
    If signal.story_status == "in_review":
      Update YAML: story.status → in_review
      Update: story.pr_url = signal.pr_url
    If signal.story_status == "failed":
      Update YAML: story.status → failed
      Append signal.failure_log to story.failure_logs
```

**Why:** Without signal sync, the YAML shows `in_progress` for stories already done. Downstream stories stay `pending` even though they're actually ready.

---

## Step 2: Cascade State Transitions

After syncing all signals, compute all state transitions in one pass:

```
For each story with status: pending:
  If any story in dependencies[] has status: failed:
    Transition → blocked
  Elif any story in dependencies[] has status: deferred:
    Transition → blocked (dependency deferred)
  Elif all stories in dependencies[] have status: completed:
    Transition → ready

For each story with status: blocked:
  If the blocking dependency is now completed:
    Transition → pending (will be re-evaluated in this same pass)
```

**Recovery transitions** (user-initiated, handle if requested):

- `failed → pending` — user resets after fixing the spec
- `deferred → pending` — user un-defers a story

**Validate** the YAML after all updates (see `references/yaml-guardrails.md`):

```bash
npx js-yaml product-context.yaml > /dev/null && echo "YAML OK"
```

If this fails, fix the YAML before proceeding. Common fixes: quote list items containing colons, flatten nested sub-lists, escape embedded double quotes.

**Commit** the synced + cascaded state to YAML before computing scores. Increment `dispatch_epoch`.

---

## Step 3: Score Stories

### Determine Active Layer

```
For each layer (0, 1, 2, ...):
  If any story in this layer has status not in [completed, deferred]:
    This is the active layer. Stop.
```

**Layer gate check:** If active layer > 0, verify `layer_gates[active_layer - 1].status == passed`. If not, block and suggest running the layer gate test.

### Filter Ready Queue

From `ready` stories in the active layer, remove stories with file-level conflicts:

```
For each ready story:
  For each in_progress story:
    If files_affected intersection is non-empty:
      Mark as conflicted — cannot dispatch until the in_progress story completes
```

### Compute Readiness Score

Before scoring, compute each story's readiness (spec completeness):

```
readiness_score = (
  min(3, acceptance_criteria_count)      # 0-3
  + (interface_obligations_defined ? 2 : 0)  # 0 or 2
  + (files_affected_identified ? 1 : 0)      # 0 or 1
  + (verification_defined ? 2 : 0)           # 0 or 2
  + (no_relevant_open_questions ? 2 : 0)     # 0 or 2
) / 10
```

Write `readiness_score` to the story in YAML. This is used for routing in Step 7.

### Compute Dispatch Score

Each remaining ready story gets a score:

```
dispatch_score = (business_value + unblock_potential + critical_path_urgency + reuse_leverage) / (complexity_cost + ambiguity_penalty + interface_risk)
```

#### Business Value (1-10)

Use `story.business_value` if explicitly set. Otherwise derive from priority:

```
critical = 10
high     = 7
medium   = 4
low      = 1
```

#### Unblock Potential (0-10)

```
unblock_potential = min(10, count of stories that directly depend on this one * 2)
```

A story that unblocks 5 others scores 10. A leaf story scores 0.

#### Critical Path Urgency (0-10)

Compute the critical path through the dependency DAG (longest chain from any root to any leaf within the active layer). Stories on the critical path get maximum urgency:

```
If story is on critical path:
  critical_path_urgency = 10
Else:
  slack = latest_possible_start - earliest_possible_start
  critical_path_urgency = max(0, 10 - slack)
```

#### Reuse Leverage (0-10)

Stories that produce shared enablers (auth middleware, base components, shared utilities) score higher:

```
reuse_leverage = min(10, count_of_modules_depending_on_output * 3)
```

Only applies to stories with `compile_mode: shared_enabler` or whose module appears in 2+ other modules' `depends_on`.

#### Complexity Cost (denominator term)

```
S = 1    (fast feedback)
M = 2
L = 4    (slow, expensive)
```

#### Ambiguity Penalty (0-5, denominator term)

```
ambiguity_penalty = 0
If acceptance_criteria count < 3:     +2
If interface_obligations empty:       +1
If relevant open_questions exist:     +1
If files_affected empty:              +1
```

Stories with high ambiguity get lower scores, biasing dispatch toward well-specified work.

#### Interface Risk (0-3, denominator term)

```
interface_risk = min(3, count of interface contracts this story creates or modifies)
```

Cross-module interface changes carry integration risk in parallel execution.

#### Example Scores

| Story                                                      | Value | Unblock | CP  | Reuse | Cost | Ambig | IFace | Score    |
| ---------------------------------------------------------- | ----- | ------- | --- | ----- | ---- | ----- | ----- | -------- |
| Auth middleware (critical path, high, unblocks 3, enabler) | 7     | 6       | 10  | 6     | S=1  | 0     | 1     | **14.5** |
| User model (not critical, medium, unblocks 2)              | 4     | 4       | 4   | 0     | S=1  | 0     | 0     | **12.0** |
| Dashboard layout (not critical, low, leaf, ambiguous)      | 1     | 0       | 2   | 0     | L=4  | 3     | 0     | **0.43** |

### Grouped Change Dispatch

For stories with `compile_mode: grouped_change` sharing the same `change_group`:

- **Readiness gate:** Use **min readiness_score** of any story in the group — if any story is under-specified, the group isn't ready
- **Dispatch score:** Sum `business_value` and `unblock_potential` across the group; use max `critical_path_urgency` and max `reuse_leverage`; divide by sum of `complexity_cost` + max `ambiguity_penalty` + max `interface_risk`
- Dispatch the entire group as one unit — one OpenSpec change, one workspace, one PR
- Max 3 stories per group. Failure of any story fails the group.

---

## Step 4: Present Dispatch Queue

Show the sorted queue with context:

```
Dispatch Queue (Layer 0 — 4 ready, 2 in_progress, WIP 3/5)

  1. ★ PROJ-003 "Setup auth middleware"           score: 14.5
     [high] S | Module: auth | Wave 1 | Critical path | Shared enabler
     Unblocks: PROJ-005, PROJ-007, PROJ-008
     → Readiness: 0.9 — skip to /launch

  2.   PROJ-004 "Create user model"                score: 12.0
     [medium] S | Module: db | Wave 1 | 4h slack
     Unblocks: PROJ-006, PROJ-009
     → Readiness: 0.8 — skip to /launch

  3.   PROJ-010 "Add settings page"                score: 0.43
     [low] L | Module: web | Wave 3 | Leaf
     → Readiness: 0.3 — go through /design (ambiguous)

  Conflicted (waiting):
  • PROJ-006 — files overlap with in_progress PROJ-002

  Blocked (dependencies not met):
  • PROJ-008 — waiting on PROJ-003 (auth middleware)

  In progress:
  • PROJ-001 (tab: feat-api-scaffold) — Phase 4, 60% complete
  • PROJ-002 (tab: feat-db-schema) — Phase 5, code review
```

**Recommendation:** Always highlight the top story and explain why (highest score = critical path + high value + unblocks the most).

---

## Step 5: Dispatch

### Dispatch Modes

#### Interactive (default)

User picks stories one at a time. Best for early layers or learning the system.

#### Wave Batch (`--batch wave`)

Dispatch all ready stories in the current wave (execution slice) at once:

```
Dispatches all ready stories in Wave N (up to WIP limit)
Creates N workspaces via /launch
```

### WIP Limits

```
max_wip = topology.routing.concurrency_limit  (default: 5)
current_wip = count of stories with status: in_progress
available_slots = max_wip - current_wip

Never dispatch more than available_slots stories.
```

**Why (Little's Law):** `Lead Time = WIP / Throughput`. If you merge 3 stories/day, WIP 3 = 1 day lead time. WIP 15 = 5 days. The bottleneck is usually human PR review, not agent speed.

### The Dispatch Lock

For each selected story, dispatch atomically:

```
1. Re-read story.status from YAML (not from cache)
2. If status != ready → SKIP (already dispatched by another run)
3. Write to YAML:
     status: in_progress
     assigned_to: <workspace-name>
     openspec_change: <story-id>
     started_at: <ISO 8601 now>
     dispatched_at_epoch: <current dispatch_epoch>
4. Commit YAML immediately (this IS the lock)
5. THEN create OpenSpec change and workspace
```

The commit happens BEFORE the workspace is created. Two consecutive `/dispatch` runs: Run 1 writes `in_progress` and commits. Run 2 reads `in_progress`, skips. No double dispatch.

---

## Step 6: Create OpenSpec Change with Context Package

For each dispatched story, create the OpenSpec change with pre-assembled context:

```
openspec/changes/<story-id>/
├── proposal.md          ← story description + why + business value
├── design.md            ← module definition + interface contracts + dependency APIs
├── specs/<module>.md    ← acceptance criteria + interface obligations + verification
├── tasks.md             ← story decomposed into implementation tasks
└── .context/            ← pre-assembled context package
    ├── stable-prefix.md ← shared product/architecture context (cacheable)
    ├── dependencies.md  ← public APIs from completed dependency stories
    └── retrieval.md     ← what to explore at runtime
```

### Context Assembly

#### Part 1: Stable Prefix (~10K tokens, shared across agents in same layer)

Extracted from product definition (`product/index.yaml` in split mode, `product-context.yaml` in v1 mode):

- `product.problem` — what we're solving
- `product.constraints` — tech stack, infrastructure
- `product.layers[active_layer]` — what the user can do at this layer
- `architecture.overview` — high-level structure
- `architecture.technical_spec` — if set, include the technical specification document (or relevant sections for the story's module). This provides Symphony-style precision for protocol-heavy systems.
- Coding conventions (conventional commits, jj for local, trunk-based)

#### Part 2: Story-Specific Payload (~20K tokens, unique per agent)

- **Full story spec** from the `stories` section
- **Module definition** from `architecture.modules` matching `story.module`
- **Adjacent interfaces** from `architecture.interfaces` where `from` or `to` = story module
- **Dependency outputs** — for each completed dependency: public API surface (types, exports, endpoints). NOT internal implementation.

#### Part 3: Retrieval Instructions (~500 tokens)

```markdown
## Files to read first

- <files_affected from story spec>

## Patterns to explore

- Check existing patterns in <module> directory
- Read interface contract tests for consumed interfaces

## Do not read

- Other module internals — use dependency_outputs above
```

#### Calibration Context (for `.5` alignment layers and calibrated stories)

For stories with `calibration_type` set, or stories in `.5` alignment layers:

**Heavy calibrations** (visual-design, ux-flow, copy-tone):

1. **Include the calibration artifact** — `calibration/<type>.yaml` (e.g., `calibration/visual-design.yaml`)
2. **For visual-design:** Also include reference design files from `docs/design-references/` matching the story's page (by story activity or title)
3. **Include calibration constraint directive:**

   ```markdown
   This story has calibrated <dimension> decisions.
   Follow the calibration artifact in calibration/<type>.yaml strictly.
   Do not introduce new [visual tokens / flow patterns / voice patterns]
   not defined in the calibration artifact.
   ```

If the required `calibration/<type>.yaml` does not exist, **do not dispatch** — instruct the user to run `/calibrate <type>` first.

**Light calibrations** (api-surface, data-model, scope-direction, performance-quality):

No additional context needed — decisions are already in the architecture section of `product-context.yaml` and the product section of `product/index.yaml` (split mode), which flow through the stable prefix (Part 1) and story-specific payload (Part 2).

**Backward compatibility:** For `.5` layer stories without `calibration_type` set, default to visual-design. Check both `calibration/visual-design.yaml` and `design-context.yaml` (legacy path).

### Assembly Rules

1. **Prune aggressively** — irrelevant context degrades agent performance
2. **Dependency outputs = public API only** — types, exports, endpoint signatures, never internals
3. **Measure the package** — if it exceeds the role's token budget from topology, prune harder or split the story
4. **Stable prefix is cacheable** — when dispatching multiple stories in the same layer, write it once

---

## Commit and Push Before Handoff

> **CRITICAL:** Commit and push ALL dispatch artifacts (YAML updates, OpenSpec changes, changelog) to remote BEFORE handing off to `/launch`. If the dispatch commit stays local, it will be lost when workspace PRs merge to main and you rebase. The push ensures OpenSpec changes survive on the remote.

Append to the `changelog` section:

```yaml
- date: <today>
  type: dispatch
  author: human
  summary: "Dispatched PROJ-003 (auth middleware), PROJ-004 (user model) — Layer 0, Wave 1"
  sections_changed: [stories]
```

Commit and push:

```bash
jj describe -m "feat: dispatch PROJ-003, PROJ-004 — Layer 0 Wave 1"
jj new
jj git push --change @-
```

**Verify the push succeeded** before proceeding to handoff. If push fails (e.g., remote conflict), resolve before launching workspaces.

---

## Step 7: Hand Off

Determine the handoff based on story completeness:

### Readiness-based routing

Use the `readiness_score` computed in Step 3:

- **readiness_score >= 0.7** → skip to `/launch` (spec is dispatch-ready)
- **readiness_score 0.5–0.7** → present to user for decision (`/launch` or `/design`)
- **readiness_score < 0.5** → route to `/design` (spec needs refinement)

### Well-specified (readiness >= 0.7) → skip to /launch

- 3+ specific, testable acceptance criteria
- Interface obligations defined
- Verification strategy complete
- Files affected identified

### Ambiguous (readiness < 0.5) → go through /design

- Vague or fewer than 3 acceptance criteria
- Missing interface details
- Open questions relevant to this story

```
Story PROJ-003 dispatched (score: 14.5, critical path, shared enabler).

OpenSpec change: openspec/changes/PROJ-003/
Context package: openspec/changes/PROJ-003/.context/

Recommendation: Readiness 0.9 — well-specified
  → Skip to /launch

  /launch    ← start building immediately
  /design    ← refine the spec first
```

### Batch Handoff

For batch dispatch, create all workspaces via `/launch`:

```
Batch dispatched: PROJ-003 (score 23.0), PROJ-004 (score 12.0)

  /launch PROJ-003  → tab: auth-middleware
  /launch PROJ-004  → tab: user-model
```

---

## Edge Cases

- **No stories ready:** All pending stories have unmet dependencies. Check if any `in_progress` stories are stuck (high attempt_count, old started_at). Suggest checking workspace progress or running `/reflect`.
- **All stories completed in active layer:** Trigger layer gate test. If passed, advance to next layer and re-run dispatch.
- **All stories completed in all layers:** Product is done. Suggest `/reflect` for final review.
- **Layer gate failed:** Do not advance. Create fix stories based on gate failure, add to current layer, re-dispatch.
- **WIP limit reached:** No available slots. Show what's in progress and suggest waiting or reviewing PRs to unblock slots.

---

## Guardrails

- **Never dispatch a story with unmet dependencies** — even if the user insists.
- **Never dispatch conflicting stories in parallel** — file-level conflicts cause merge chaos.
- **Always sync signals before computing** — stale YAML produces wrong dispatch decisions.
- **Always commit YAML before creating workspaces** — the commit IS the dispatch lock.
- **Always create the OpenSpec change** — even for well-specified stories. The `.context/` directory is what the agent reads.
- **Respect WIP limits** — dispatching beyond integration capacity creates traffic jams, not speed.
