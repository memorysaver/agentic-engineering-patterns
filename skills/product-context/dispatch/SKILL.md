---
name: dispatch
description: Pick the next story to work on and bridge it into the feature lifecycle. Use when ready to start building, or when the user says "what's next", "dispatch", "pick a story", "start next feature", "what should I work on". Reads product-context.yaml, syncs workspace signals, scores stories by critical path + business value + unblock potential, assembles context, and hands off to /design or /launch. Supports parallel batch dispatch with WIP limits.
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

**Session:** Main, interactive with user
**Input:** `product-context.yaml` with stories
**Output:** OpenSpec change with pre-assembled context, story status updated, handoff to `/design` or `/launch`

---

## Before Starting

```bash
cat product-context.yaml
```

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
  signal_path = .claude/workspaces/<workspace>/.dev-workflow/signals/status.json

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

### Compute Dispatch Score

Each remaining ready story gets a score:

```
dispatch_score = (critical_path_urgency + business_value + unblock_potential) / complexity_cost
```

#### Critical Path Urgency (0-10)

Compute the critical path through the dependency DAG (longest chain from any root to any leaf within the active layer). Stories on the critical path get maximum urgency:

```
If story is on critical path:
  critical_path_urgency = 10
Else:
  slack = latest_possible_start - earliest_possible_start
  critical_path_urgency = max(0, 10 - slack)
```

A critical-path story delayed by 1 hour delays the entire layer by 1 hour.

#### Business Value (1-10)

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

#### Complexity Cost (divisor)

```
S = 1    (fast feedback)
M = 2
L = 4    (slow, expensive)
```

#### Example Scores

| Story | CP | Value | Unblock | Complexity | Score |
|-------|-----|-------|---------|------------|-------|
| Auth middleware (critical path, high priority, unblocks 3) | 10 | 7 | 6 | S=1 | **23.0** |
| User model (not critical, medium priority, unblocks 2) | 4 | 4 | 4 | S=1 | **12.0** |
| Dashboard layout (not critical, low priority, leaf) | 2 | 1 | 0 | L=4 | **0.75** |

---

## Step 4: Present Dispatch Queue

Show the sorted queue with context:

```
Dispatch Queue (Layer 0 — 4 ready, 2 in_progress, WIP 3/5)

  1. ★ PROJ-003 "Setup auth middleware"           score: 23.0
     [critical] S | Module: auth | Slice 1 | Critical path
     Unblocks: PROJ-005, PROJ-007, PROJ-008
     → Well-specified (3 acceptance criteria, contracts defined)
     → Recommend: skip to /launch

  2.   PROJ-004 "Create user model"                score: 12.0
     [medium] S | Module: db | Slice 1 | 4h slack
     Unblocks: PROJ-006, PROJ-009
     → Well-specified
     → Recommend: skip to /launch

  3.   PROJ-010 "Add settings page"                score: 0.75
     [low] L | Module: web | Slice 3 | Leaf
     → Ambiguous (1 vague criterion, no interface contracts)
     → Recommend: go through /design

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

#### Slice Batch (`--batch slice`)
Dispatch all ready stories in the current execution slice at once:
```
Dispatches all ready stories in Slice N (up to WIP limit)
Creates N workspaces via /launch
```

#### Wave (`--batch wave`)
Dispatch the highest-scored ready stories regardless of slice:
```
N = min(ready_count, wip_limit - in_progress_count)
Dispatches top N stories by dispatch_score
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

Extracted from `product-context.yaml`:
- `product.problem` — what we're solving
- `product.constraints` — tech stack, infrastructure
- `product.layers[active_layer]` — what the user can do at this layer
- `architecture.overview` — high-level structure
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

### Assembly Rules

1. **Prune aggressively** — irrelevant context degrades agent performance
2. **Dependency outputs = public API only** — types, exports, endpoint signatures, never internals
3. **Measure the package** — if it exceeds the role's token budget from topology, prune harder or split the story
4. **Stable prefix is cacheable** — when dispatching multiple stories in the same layer, write it once

---

## Step 7: Hand Off

Determine the handoff based on story completeness:

### Well-specified → skip to /launch
- 3+ specific, testable acceptance criteria
- Interface obligations defined
- Verification strategy complete
- Files affected identified
- Complexity S or M

### Ambiguous → go through /design
- Vague or fewer than 3 acceptance criteria
- Missing interface details
- Complexity L
- Open questions relevant to this story

```
Story PROJ-003 dispatched (score: 23.0, critical path).

OpenSpec change: openspec/changes/PROJ-003/
Context package: openspec/changes/PROJ-003/.context/

Recommendation: Well-specified (3 criteria, contracts defined, S complexity)
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

## Append Changelog

After dispatching, append to the `changelog` section:

```yaml
- date: <today>
  type: dispatch
  author: human
  summary: "Dispatched PROJ-003 (auth middleware), PROJ-004 (user model) — Layer 0, Slice 1"
  sections_changed: [stories]
```

Commit all changes:

```bash
jj describe -m "feat: dispatch PROJ-003, PROJ-004 — Layer 0 Slice 1"
jj new
jj git push --change @-
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
