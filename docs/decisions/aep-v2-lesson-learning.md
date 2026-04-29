# AEP v2: Lesson Learning & Self-Improvement

> Decision record from Layer 0 post-mortem (2026-04-03).
> 17/17 stories shipped, 0 failures, 42 ticks, 0 escalations.
> Three structural improvements identified.

---

## 1. UI Polish Layer Pattern

### What

After every implementation layer (0, 1, 2...), insert a `.5` polish layer (0.5, 1.5, 2.5) dedicated to human-driven UI/UX iteration.

### Why

Layer 0 shipped 17 stories autonomously — all functional, all merged. But when the human looked at the result, the UI was the gap: no landing page, generic auth pages, dashboard lacking detail. Agents build **functional** UI but cannot judge **visual design quality**, layout harmony, or brand consistency.

This isn't a bug or a missed requirement. It's a structural property of agent-driven development: implementation correctness ≠ design quality. The polish layer makes this explicit rather than discovering it after every layer.

### How

| Skill                           | Change                                                                                                                                                                                                                                                                                              |
| ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/map` (Step 3: Layer Planning) | After defining each implementation layer, add guidance: "Consider whether a .5 polish layer is needed for UI-heavy activities in this layer. If the layer contains `signup`, `register-daemon`, `configure-guardrails`, or `review-audit` activities with web module stories, plan a polish layer." |
| `/autopilot` (Step ⑥: Dispatch) | After completing all stories in a layer, check if a polish layer exists. If yes, dispatch polish stories before advancing to the next integer layer.                                                                                                                                                |
| `/reflect` (Step 2: Classify)   | Add "Polish" as a refinement sub-type. When UI observations surface, default to creating stories in the next `.5` layer rather than the next integer layer.                                                                                                                                         |

### Pattern

```
Layer 0 (walking skeleton)
  → Layer 0.5 (UI polish: landing page, auth pages, dashboard detail)
Layer 1 (MCP, heartbeat, audit)
  → Layer 1.5 (UI polish: audit viewer, guardrails page, web terminal)
Layer 2 (web terminal, queue audit)
  → Layer 2.5 (UI polish: terminal UX, session management)
```

### Decision

Polish layers are **opt-in per layer**, not automatic. The `/reflect` step after each layer explicitly asks: "Does this layer need a UI polish pass?" The human decides. But the workflow makes the question unavoidable.

---

## 2. Build-Time Lesson Capture

### What

Workspace agents capture what they learn during builds — solutions discovered, errors encountered, missing docs, patterns that worked — and this knowledge persists in the repo as institutional memory.

### Why

Layer 0 workspace agents solved 17 stories independently. Each one discovered things: workarounds for library quirks, patterns for oRPC routes, Rust async PTY patterns, worktree gotchas. All of that knowledge **died when the workspace was removed**. The next agent building a similar story starts from zero.

### Data Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│  /build (workspace agent)                                       │
│                                                                 │
│  Phase 0: Create .dev-workflow/lessons.md (empty template)      │
│  Phase 4: Append notes after noteworthy tasks                   │
│           "## Solution: <title>"                                │
│           "## Error: <title>"                                   │
│           "## Missing: <title>"                                 │
│  Phase 9: Final summary — what would help the next agent?       │
│                                                                 │
└──────────────────────┬──────────────────────────────────────────┘
                       │ .dev-workflow/lessons.md
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│  /wrap (main session, Phase 13)                                 │
│                                                                 │
│  BEFORE git worktree remove:                                    │
│    1. Read .feature-workspaces/<name>/.dev-workflow/lessons.md   │
│    2. If non-empty, commit to lessons-learned/<change-name>.md  │
│    3. Then proceed with workspace cleanup                       │
│                                                                 │
└──────────────────────┬──────────────────────────────────────────┘
                       │ lessons-learned/<change-name>.md
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│  /launch (main session, bootstrap)                              │
│                                                                 │
│  Before sending bootstrap prompt:                               │
│    1. Read lessons-learned/*.md                                  │
│    2. Filter by module or activity matching the new story       │
│    3. Append relevant lessons to bootstrap prompt context       │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Skill Changes

| Skill        | Phase                    | Change                                                                                                                                                                                                                                           |
| ------------ | ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `aep-build`  | Phase 0 (Initialize)     | Create `.dev-workflow/lessons.md` with template header. Add to Phase 0 step list.                                                                                                                                                                |
| `aep-build`  | Phase 4 (Implementation) | After completing each task, if the agent encountered something noteworthy (a non-obvious solution, an error that took multiple attempts, missing documentation), append to `lessons.md`. This is **opt-in per task**, not mandatory.             |
| `aep-build`  | Phase 9 (Cleanup)        | Before publishing, write a final "## Summary for Next Agent" section in `lessons.md`: what would you tell the next agent building in this module?                                                                                                |
| `aep-wrap`   | Phase 13 (Archive)       | **New step 5.5** (after archive, before workspace forget): Read `.feature-workspaces/<name>/.dev-workflow/lessons.md`. If non-empty and substantive, copy to `lessons-learned/<change-name>.md` and commit.                                      |
| `aep-launch` | Bootstrap prompt         | **New step** before sending bootstrap: Read `lessons-learned/*.md`, filter for entries matching the story's `module` or `activity`, and append a "## Prior Lessons" section to the bootstrap context. Cap at 2000 tokens to avoid context bloat. |

### Lesson Template

```markdown
# Lessons: <change-name>

Module: <module>
Activity: <activity>
Date: <date>
Story: <story-id>

## Solutions

### <title>

<what worked, why it was non-obvious>

## Errors

### <title>

<what went wrong, root cause, how it was resolved>

## Missing

### <title>

<what documentation, types, or patterns were missing and had to be discovered>

## Summary for Next Agent

<1-3 sentences: what would you tell the next agent building in this module?>
```

### Key Decisions

- **`lessons-learned/` lives at repo root** — not inside `.dev-workflow/` (ephemeral) or `.claude/` (tooling). These are project knowledge, committed to git.
- **Lesson capture is optional** — agents write only when something is noteworthy. No empty ceremony.
- **Lesson injection is filtered** — only lessons from the same module/activity enter the bootstrap prompt. Context is precious.
- **Markdown, not JSON** — low barrier to write, easy to read, git-friendly diffs.

---

## 3. Reflect → Workflow Self-Improvement

### What

The `/reflect` skill should improve not just `product-context.yaml` but also the workflow itself. Lessons learned feed back into the launch phase, and recurring patterns trigger workflow amendments.

### Why

The Layer 0 autopilot revealed two process issues that only became visible after the full run:

1. Workspace agents stalled on permission prompts (needed pre-configured settings)
2. Workspace signals could be stale (needed cross-check against actual PR state)

Both were fixed ad-hoc during the run. But without a structured feedback path, they would have been forgotten by the next session. The reflect skill needs to capture **workflow observations**, not just product observations.

### How

| Skill         | Step                                | Change                                                                                                                                                                                                                                                                                                                                                                                             |
| ------------- | ----------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `aep-reflect` | Step 1 (Gather)                     | Add `lessons-learned/*.md` as a data source alongside user testing, error logs, and cost data. Read and summarize lessons from the completed layer.                                                                                                                                                                                                                                                |
| `aep-reflect` | Step 2 (Classify)                   | Add a fifth classification type: **Process** — observations about the workflow itself (not the product). Examples: permission stalls, signal staleness, missing tooling.                                                                                                                                                                                                                           |
| `aep-reflect` | New Step 5.5 (Workflow Improvement) | After updating product context, review Process-classified observations. For each: (a) document the pattern in `lessons-learned/process/<observation>.md`, (b) add a `process_learnings` entry to the `topology.routing` section of `product-context.yaml`, (c) if the pattern warrants a skill file change, record it as a proposed amendment in the changelog — **do not auto-edit skill files**. |
| `aep-launch`  | Bootstrap prompt                    | Read `lessons-learned/process/*.md` and inject relevant process guidance (e.g., "pre-configure permissions", "cross-check PR state after signal sync").                                                                                                                                                                                                                                            |

### Feedback Loop Diagram

```
                    ┌──────────────────┐
                    │                  │
                    │   product-       │
                    │   context.yaml   │◄──────────────────┐
                    │                  │                    │
                    └───────┬──────────┘                    │
                            │                              │
                   /dispatch + /launch                  /reflect
                            │                              │
                            ▼                              │
                    ┌──────────────────┐                    │
                    │                  │                    │
                    │   /build         │                    │
                    │   (workspace)    │                    │
                    │                  │                    │
                    │   writes:        │                    │
                    │   lessons.md     │                    │
                    │                  │                    │
                    └───────┬──────────┘                    │
                            │                              │
                          /wrap                            │
                            │                              │
                            ▼                              │
                    ┌──────────────────┐                    │
                    │                  │                    │
                    │  lessons-learned/│────────────────────┘
                    │  <change>.md     │
                    │  process/*.md    │
                    │                  │
                    └──────────────────┘
```

### Key Decisions

- **Workflow changes are proposed, not auto-applied** — skill files are sensitive. The reflect step records proposed amendments; a human reviews and applies them.
- **Process lessons are separate from feature lessons** — stored in `lessons-learned/process/` subdirectory. They apply to all future builds, not just builds in a specific module.
- **The loop is explicit** — the reflect skill's Step 5.5 asks: "Did we learn anything about how we work, not just what we built?" This makes workflow improvement a first-class activity.

---

## Implementation Priority

1. **Lesson capture in `/build`** (write side) — immediate value, low effort
2. **Lesson archival in `/wrap`** (persist side) — completes the write path
3. **Lesson injection in `/launch`** (read side) — closes the feature learning loop
4. **Workflow improvement in `/reflect`** (process learning) — closes the process loop
5. **Polish layer in `/map`** (layer planning) — independent, can be done anytime

Items 1-2 are the **minimum viable lesson system**. Items 3-4 close the feedback loop. Item 5 is a planning pattern change.

---

## Appendix: Evidence from Layer 0

| Observation                                        | Classification | Evidence                                                                |
| -------------------------------------------------- | -------------- | ----------------------------------------------------------------------- |
| LR-003 needed bootstrap resent                     | Process        | Workspace agent sat at idle prompt — permission or initialization stall |
| LR-007 signal showed `in_review` but PR was merged | Process        | Signal staleness — cross-check via `gh pr view` caught it               |
| No landing page after Layer 0                      | Polish         | Functional walking skeleton but no public face                          |
| Auth pages generic                                 | Polish         | Better Auth forms work but look default                                 |
| Dashboard lacks daemon detail                      | Polish         | List view only, no connection status or actions                         |
| All 17 stories completed first attempt             | Cost           | S stories: 1-2 ticks, M stories: 2-4 ticks, 0 retries                   |
| Concurrency rarely hit limit of 5                  | Cost           | Dependency serialization kept typical concurrency at 1-3                |
