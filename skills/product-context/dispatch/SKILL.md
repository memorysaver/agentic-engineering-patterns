---
name: dispatch
description: Pick the next story to work on and bridge it into the feature lifecycle. Use when ready to start building, or when the user says "what's next", "dispatch", "pick a story", "start next feature", "what should I work on". Reads product-context.yaml, presents the dispatch queue, creates an OpenSpec change, and hands off to /design or /launch.
---

# Dispatch

Bridge between the product context (control plane) and the feature lifecycle (execution plane). Reads the story graph from `product-context.yaml`, determines what's ready to build, and routes the selected story into `/design` or `/launch`.

**Where this fits:**

```
/envision → /map → /scaffold
  → [ /dispatch → /design → /launch → /build → /wrap ]
       ▲ you are here
  → /reflect → loop
```

**Session:** Main, interactive with user
**Input:** `product-context.yaml` with stories
**Output:** OpenSpec change created, story status updated, handoff to `/design` or `/launch`

---

## Before Starting

Read the product context:

```bash
cat product-context.yaml
```

If `product-context.yaml` doesn't exist, run `/envision` then `/map` first.

If the `stories` section is empty, run `/map` to decompose the product into stories.

---

## Step 1: Determine Active Layer

Find the current active layer — the lowest layer number with incomplete stories:

```
For each layer (0, 1, 2, ...):
  If any story in this layer has status != completed and status != deferred:
    This is the active layer. Stop.
```

### Layer Gate Check

If the active layer is > 0, verify the previous layer's gate has passed:

```
If layer_gates[active_layer - 1].status != passed:
  BLOCK — cannot start Layer N until Layer N-1 gate passes.
  Suggest: run the layer gate integration test first.
```

Present the current state to the user:

```
Layer 0: 5/8 completed, 2 in_progress, 1 ready
Layer 1: 0/6 completed (blocked — waiting for Layer 0 gate)
```

---

## Step 2: Compute Ready Stories

Transition stories based on dependency status:

```
For each story with status: pending:
  If any story in dependencies[] has status: failed:
    Transition → blocked
  Elif any story in dependencies[] has status: deferred:
    Transition → blocked (dependency deferred — will unblock if un-deferred)
  Elif all stories in dependencies[] have status: completed:
    Transition → ready

For each story with status: blocked:
  If the blocking dependency is now completed:
    Transition → pending (will be re-evaluated next cycle)
```

Also handle recovery transitions (user-initiated):
- `failed → pending` — user resets after fixing the spec
- `deferred → pending` — user un-defers a story

Update the YAML with all new statuses.

---

## Step 3: Filter and Sort

From the `ready` stories in the active layer:

### Conflict Detection

Remove stories whose `files_affected` overlap with any `in_progress` story. These would cause merge conflicts if built in parallel.

```
For each ready story:
  For each in_progress story:
    If files_affected intersection is non-empty:
      Mark as conflicted — cannot dispatch until the in_progress story completes.
```

### Priority Sort

Sort remaining stories by:
1. **Priority:** critical > high > medium > low
2. **Slice:** lower slice number first (earlier in layer execution order)
3. **Complexity:** S > M > L (simpler stories first — faster feedback)

---

## Step 4: Present Dispatch Queue

Show the sorted queue to the user with context for each story:

```
Dispatch Queue (Layer 0, 3 ready):

  1. [critical] PROJ-003 "Setup auth middleware" (S)
     Module: auth | Slice: 1 | Dependencies: none
     → Ready to build

  2. [high] PROJ-005 "Create user registration endpoint" (M)
     Module: api | Slice: 1 | Dependencies: none
     → Ready to build

  3. [medium] PROJ-007 "Add session persistence" (S)
     Module: auth | Slice: 2 | Dependencies: PROJ-003
     → Ready (PROJ-003 completed)

  Conflicted (waiting):
  - PROJ-004 "Add login form" — files overlap with in_progress PROJ-002

  Blocked (dependencies not met):
  - PROJ-008 "Dashboard layout" — waiting on PROJ-005, PROJ-006
```

**Recommendation:** Highlight the top story and explain why it's recommended (highest priority, no conflicts, unblocks the most downstream work).

---

## Step 5: User Picks a Story

The user selects a story from the queue (or accepts the recommendation).

For **batch dispatch** (multiple parallel workspaces), the user can pick multiple non-conflicting stories from the same slice.

---

## Step 6: Create OpenSpec Change

Generate an OpenSpec change from the selected story spec:

```bash
# Create the OpenSpec change directory
mkdir -p openspec/changes/<story-id>/specs
```

Map story fields to OpenSpec artifacts:

| Story field | OpenSpec artifact |
|---|---|
| `description.what_changes` + `description.why` | `proposal.md` |
| Architecture module + interface contracts | `design.md` |
| `acceptance_criteria` + `interface_obligations` | `specs/<module>.md` |
| Story as implementation task | `tasks.md` |

### proposal.md

```markdown
# <story title>

## What
<description.what_changes>

## Why
<description.why>

## Story Reference
- ID: <story id>
- Layer: <layer>
- Module: <module>
- Priority: <priority>
- Complexity: <complexity>
```

### design.md

Extract from `product-context.yaml`:
- The story's module definition (from `architecture.modules`)
- Adjacent interface contracts (from `architecture.interfaces` where `from` or `to` matches the module)
- Relevant data flow (from `architecture.data_flows`)

### specs/<module>.md

```markdown
# Acceptance Criteria

<acceptance_criteria as numbered list>

# Interface Obligations

Implements: <interface_obligations.implements>
Consumes: <interface_obligations.consumes>
Contract tests required: <yes/no>

# Verification Strategy

Unit: <verification.unit>
Integration: <verification.integration>
Contract: <verification.contract>
```

### tasks.md

```markdown
# Tasks

- [ ] <story title>
  - Acceptance: <criteria summary>
  - Files: <files_affected>
```

For complex stories (L), decompose into sub-tasks based on acceptance criteria.

---

## Step 7: Update YAML

Update the story in `product-context.yaml`:

```yaml
status: in_progress
assigned_to: <workspace-name or "main">
openspec_change: <story-id>
started_at: <ISO 8601 now>
```

Append to changelog:

```yaml
- date: <today>
  type: dispatch
  author: human
  summary: "Dispatched <story-id>: <story-title>"
  sections_changed: [stories]
```

Commit:

```bash
git add product-context.yaml openspec/changes/<story-id>/
git commit -m "feat: dispatch <story-id> — <story-title>"
```

---

## Step 8: Hand Off

Determine the handoff based on story completeness:

### Well-specified story (skip to /launch)

The story has:
- Detailed acceptance criteria (3+ specific, testable items)
- Interface obligations defined
- Verification strategy complete
- Files affected identified

→ Suggest: **skip `/design` and go straight to `/launch`**. The OpenSpec change already has everything the agent needs.

### Ambiguous or complex story (go to /design)

The story has:
- Vague acceptance criteria
- Missing interface details
- Complexity: L
- Open questions relevant to this story

→ Suggest: **run `/design` first** to refine the spec through explore/propose conversation.

```
Story PROJ-003 dispatched.

OpenSpec change created at openspec/changes/PROJ-003/

Recommendation: This story is well-specified (3 acceptance criteria,
interface contracts defined, files identified). Skip to /launch.

  /launch    ← start building immediately
  /design    ← refine the spec first
```

---

## Batch Dispatch

For parallel execution (multiple workspace sessions), dispatch multiple stories at once:

1. Select non-conflicting stories from the same slice
2. Create OpenSpec changes for each
3. Update all statuses in YAML
4. Launch each in its own workspace via `/launch`

```
Batch dispatch: PROJ-003, PROJ-005 (Slice 1, no file conflicts)

  /launch PROJ-003  → tab: auth-middleware
  /launch PROJ-005  → tab: user-registration
```

---

## Edge Cases

- **No stories ready:** All pending stories have unmet dependencies. Check if any `in_progress` stories are stuck (high attempt_count, old started_at). Suggest checking workspace progress or running `/reflect`.
- **All stories completed in active layer:** Trigger layer gate test. If passed, advance to next layer and re-run dispatch.
- **All stories completed in all layers:** Product is done. Suggest `/reflect` for final review and celebration.
- **Layer gate failed:** Do not advance. Create fix stories based on gate failure, add to current layer, re-dispatch.

---

## Guardrails

- **Never dispatch a story with unmet dependencies** — even if the user insists. Dependencies exist because module boundaries require ordered integration.
- **Never dispatch conflicting stories in parallel** — file-level conflicts cause merge chaos across workspace sessions.
- **Always update the YAML before handing off** — the YAML is the source of truth. If it says `pending` but the agent is building, the system is inconsistent.
- **Always create the OpenSpec change** — even for well-specified stories. The change directory is what `/build` reads.
