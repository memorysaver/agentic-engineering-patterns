---
name: aep-dispatch
description: >-
  Selects and routes one ready story by syncing signals, scoring the queue, and
  handing off to /aep-design or /aep-launch. Use for "what's next", "dispatch",
  or "pick a story"; use /aep-autopilot for continuous execution.
---

# Dispatch

Bridge between the product context (control plane) and the feature lifecycle (execution plane). Syncs workspace state, scores stories, picks what to build next, assembles context, and routes into `/aep-design` or `/aep-launch`.

**Where this fits:**

```
/aep-envision → /aep-map → /aep-model (UI-facing) → /aep-scaffold
  → [ /aep-dispatch → /aep-design → /aep-launch → /aep-build → /aep-wrap ]
       ▲ you are here
  → /aep-reflect → loop
```

**Session:** Main, interactive
**Input:** Product definition from `product/index.yaml` (split mode) or `product-context.yaml` (v1 mode); operational state from `product-context.yaml`
**Output:** OpenSpec change with pre-assembled context, story status updated, handoff to `/aep-design` or `/aep-launch`

> **For autonomous orchestration:** Use `/aep-autopilot` instead. Autopilot runs the full dispatch-launch-monitor-review-wrap-dispatch cycle as a tick-based state machine via `/loop`. Dispatch remains a single-pass interactive tool.
>
> **For hands-free batch under Claude Code:** dispatch a wave **"with workflow"** to build the whole wave as one dynamic workflow (Step 5 → _Dynamic Workflow_).

---

## Before Starting

Probe which mode the product context is in:

```bash
ls product/index.yaml 2>/dev/null && echo "SPLIT MODE" || echo "V1 MODE"
cat product-context.yaml
```

Mode semantics are canonical in `references/file-resolution.md`. Dispatch reads the product definition from `product/index.yaml` (split) or `product-context.yaml` (v1); it reads/writes operational state (stories, topology, architecture, cost, changelog) always in `product-context.yaml`.

If `product-context.yaml` doesn't exist, run `/aep-envision` then `/aep-map` first. If the `stories` section is empty, run `/aep-map` to decompose the product.

---

## The Dispatch Protocol

Every dispatch run follows the same 7-step protocol. Each step is idempotent — running `/aep-dispatch` twice with no state changes produces the same result. Story states (pending / ready / blocked / in_progress / in_review / completed / failed / deferred) follow the product-context schema.

```
① SYNC signals    → bring YAML up to date with workspace reality
② CASCADE states  → compute all pending→ready, pending→blocked transitions
③ SCORE stories   → rank the ready queue by dispatch_score
④ PRESENT queue   → show user the scored dispatch queue
⑤ DISPATCH        → lock stories (status→in_progress), create OpenSpec changes
⑥ MONITOR         → agents work, main session watches signals
⑦ COMPLETE        → /aep-wrap updates YAML, atomic cascade, re-invoke dispatch
```

---

## Step 1: Signal Sync

Sync workspace signals into the YAML **before** computing anything — stale YAML shows `in_progress` for stories already done and keeps downstream stories `pending` when they are actually ready.

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

**Postcondition:** every `in_progress`/`in_review` story's YAML status matches its signal file.

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

This is the dependency gate: a story only becomes `ready` when every dependency is `completed`, so no story with unmet dependencies is ever dispatchable.

**Recovery transitions** (user-initiated, handle if requested):

- `failed → pending` — user resets after fixing the spec
- `deferred → pending` — user un-defers a story

**Validate** the YAML after all updates:

```bash
npx js-yaml product-context.yaml > /dev/null && echo "YAML OK"
```

If this fails, fix the YAML before proceeding (see `references/yaml-guardrails.md` for the common fixes).

**Commit** the synced + cascaded state to YAML before computing scores, and increment `dispatch_epoch`.

**Postcondition:** `npx js-yaml` exits 0 and the cascaded state is committed.

---

## Step 3: Score Stories

### Determine Active Layer

```
For each layer (0, 1, 2, ...):
  If any story in this layer has status not in [completed, deferred]:
    This is the active layer. Stop.
```

**Layer gate check:** If active layer > 0, verify `layer_gates[active_layer - 1].status == passed`. If not, block and suggest running the layer gate. A gate at `scripted_passed` (Tier-1 machinery green, but the journey dogfood / coverage half isn't done) does **not** unblock the next layer — report it as "machinery green, dogfood pending" so the human knows the remaining half. Only `/aep-wrap`'s two-phase flip advances a gate to `passed`.

### Filter Ready Queue

Dispatch only `ready` stories in the active layer that have no file-level conflict with in-progress work:

```
For each ready story:
  For each in_progress story:
    If files_affected intersection is non-empty:
      Mark as conflicted — cannot dispatch until the in_progress story completes
```

### Compute Scores

For each remaining ready story, compute `readiness_score` and `dispatch_score` per `references/scoring.md`, and write both into the story YAML:

- **`readiness_score`** (0–1) = spec completeness; drives Step 7 routing.
- **`dispatch_score`** = (business value + unblock potential + critical-path urgency + reuse leverage) / (complexity cost + ambiguity penalty + interface risk); ranks the queue.

Grouped changes (`compile_mode: grouped_change` sharing a `change_group`) score and dispatch as one unit — see `references/scoring.md` → Grouped Change Dispatch.

**Postcondition:** every ready story has `readiness_score` and `dispatch_score` written to its YAML.

---

## Step 4: Present Dispatch Queue

Show the sorted queue with context. Highlight the top story and explain why (highest score = critical path + high value + unblocks the most). One row per story, plus the conflicted / blocked / in-progress sections:

```
Dispatch Queue (Layer 0 — 4 ready, 2 in_progress, WIP 3/5)

  1. ★ PROJ-003 "Setup auth middleware"           score: 14.5
     [high] S | Module: auth | Wave 1 | Critical path | Shared enabler
     Unblocks: PROJ-005, PROJ-007, PROJ-008
     → Readiness: 0.9 — skip to /aep-launch

  Conflicted (waiting):  • PROJ-006 — files overlap with in_progress PROJ-002
  Blocked (deps unmet):  • PROJ-008 — waiting on PROJ-003 (auth middleware)
  In progress:           • PROJ-001 (tab: feat-api-scaffold) — Phase 4, 60%
```

---

## Step 5: Dispatch

### Dispatch Modes

- **Interactive (default)** — user picks stories one at a time. Best for early layers or learning the system.
- **Wave Batch (`--batch wave`)** — dispatch all ready stories in the current wave at once (up to the WIP limit), creating N workspaces via `/aep-launch`.
- **Dynamic Workflow (`--batch wave` + "…with workflow")** — when the host is Claude Code with the Workflow tool **and** the user asked for a workflow, route the batch through workflow mode (one dynamic workflow, one agent per locked story) instead of N `/aep-launch` workers. Read `references/workflow-mode.md`: it carries the machine-assembled STEP-0 brief requirement (the stale-base hazard) and the mode announcement dispatch must make when it bypasses `/aep-launch`.

### WIP Limits

```
max_wip = topology.routing.concurrency_limit  (default: 5)
current_wip = count of stories with status: in_progress
available_slots = max_wip - current_wip
```

Dispatch at most `available_slots` stories — this cap holds in every mode, including Dynamic Workflow. The integration/merge bottleneck (usually human PR review, not agent speed) is what `available_slots` protects: by Little's Law, `Lead Time = WIP / Throughput`, so dispatching beyond capacity creates traffic jams, not speed.

**Gate:** WIP limit respected — do not dispatch when `available_slots <= 0`.

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

The commit happens BEFORE the workspace is created. Two consecutive `/aep-dispatch` runs: Run 1 writes `in_progress` and commits. Run 2 reads `in_progress`, skips. No double dispatch.

> **Mechanical ordering invariant [lock-before-workspace].** This step is
> deterministic WHEN+SHAPE — a precondition re-read, a state flip, a commit,
> in that order. It is exactly the class of step that drifts when left to
> recall and holds when owned by mechanism; a project scaling dispatch should
> put it behind a typed gate (a verb that refuses `[story-not-ready]` /
> `[dependencies-completed]` by name). See `aep-autopilot`
> `references/deterministic-orchestration.md`.

---

## Step 6: Create OpenSpec Change with Context Package

For each dispatched story, create the OpenSpec change `openspec/changes/<story-id>/` — `proposal.md`, `design.md`, `specs/<module>.md`, `tasks.md`, and a `.context/` package (`stable-prefix.md`, `dependencies.md`, `retrieval.md`). Always create the OpenSpec change, even for a well-specified story — the `.context/` directory is what the agent reads.

Assemble the handoff package per `references/context-assembly.md`, pruned to the role's token budget from topology.

**Dispatch-blocking gates in assembly:** calibrated stories (`calibration_type` set, or `.5` alignment layers — concept canonical in `/aep-map` `references/alignment-layers.md`) require `calibration/<type>.yaml` to exist — else route to `/aep-calibrate`. UI-facing stories (`object_model_refs`, `calibration_type` in {visual-design, ux-flow}, or a `ui`-kind module) require an `approved` Object Map that covers the story — else route to `/aep-model`. Both gates are detailed in `references/context-assembly.md`; if either fails, **do not dispatch**.

**Postcondition:** `openspec/changes/<story-id>/.context/` exists for each dispatched story.

---

## Commit and Push Before Handoff

> **CRITICAL:** Commit and push ALL dispatch artifacts (YAML updates, OpenSpec changes, changelog) to remote BEFORE handing off to `/aep-launch`. If the dispatch commit stays local, it is lost when workspace PRs merge to the integration branch and you rebase. The push ensures OpenSpec changes survive on the remote.

Append to the `changelog` section:

```yaml
- date: <today>
  type: dispatch
  author: human
  summary: "Dispatched PROJ-003 (auth middleware), PROJ-004 (user model) — Layer 0, Wave 1"
  sections_changed: [stories]
```

Then commit and push per `/aep-git-ref` "Control-Plane Commits": `git add product-context.yaml openspec/changes/`, commit (`feat: dispatch <ids> — Layer N Wave M`), and push to `$BASE` (resolve `$BASE` per `/aep-git-ref` "Resolving `$BASE`").

**Verify the push succeeded** before proceeding to handoff. If push fails (e.g., remote conflict), resolve before launching workspaces.

**Postcondition:** the dispatch commit is present on `origin/$BASE`.

---

## Step 7: Hand Off

> **Launch mode is normally resolved at `/aep-launch`, not here.** For the default
> path dispatch stays executor-agnostic — it hands a well-specified change to
> `/aep-launch`, which detects the host and selects a mode via `aep-executor`.
> **The one exception is the _Dynamic Workflow_ opt-in (Step 5):** that path runs the
> **workflow** mode _from dispatch_, bypassing `/aep-launch`, so dispatch itself owns
> mode selection and the announcement for that case.

Route on the `readiness_score` band computed in Step 3. Band definitions are canonical in `references/scoring.md` § Routing Thresholds; this section owns only the interactive actions:

- **dispatch-ready** → skip to `/aep-launch`
- **borderline** → present the `/aep-launch` versus `/aep-design` decision to the user
- **under-specified** → route to `/aep-design`

### Full-auto / auto-design routing (medium/low readiness)

Routing of a borderline or under-specified story depends on two `topology.routing`
flags — the `full_auto` master switch (default **false**) and the finer-grained
`auto_design` (default **false**). `full_auto` sits **above** `auto_design`:
`full_auto: true` implies `auto_design: true`.

- **`full_auto: true` OR `auto_design: true`** → resolve the story with a
  **non-interactive gen/eval design resolver** (a design agent that refines the spec
  without a human, then routes to `/aep-launch`) instead of escalating to interactive
  `/aep-design`. No strategic pause.
- **`full_auto: false` AND `auto_design: false` (default)** → route to interactive
  `/aep-design` for human design refinement before launch. The strategic "what to
  build" gate stays with the human.

Example single-story handoff:

```
Story PROJ-003 dispatched (score: 14.5, critical path, shared enabler).
OpenSpec change: openspec/changes/PROJ-003/   Context: .../.context/
Recommendation: Readiness 0.9 — well-specified → /aep-launch
  /aep-launch    ← start building immediately
  /aep-design    ← refine the spec first
```

### Batch Handoff

For batch dispatch, create all workspaces via `/aep-launch`:

```
Batch dispatched: PROJ-003 (score 23.0), PROJ-004 (score 12.0)
  /aep-launch PROJ-003  → tab: auth-middleware
  /aep-launch PROJ-004  → tab: user-model
```

---

## Edge Cases

- **No stories ready:** All pending stories have unmet dependencies. Check if any `in_progress` stories are stuck (high attempt_count, old started_at). Suggest checking workspace progress or running `/aep-reflect`.
- **All stories completed in active layer:** Trigger the layer gate test. If passed, advance to the next layer and re-run dispatch.
- **All stories completed in all layers:** Product is done. Suggest `/aep-reflect` for final review.
- **Layer gate failed:** Do not advance. Create fix stories based on gate failure, add to current layer, re-dispatch.
- **WIP limit reached:** No available slots. Show what's in progress and suggest waiting or reviewing PRs to unblock slots.
