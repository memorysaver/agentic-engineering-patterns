# Autopilot State Schema

All autopilot state files live in `.dev-workflow/` on the main workspace (repo root). This reuses the existing `.dev-workflow/` pattern already established for workspace agents.

---

## `.dev-workflow/autopilot-state.json`

Machine-readable state file. Read and written by the autopilot tick.

```json
{
  "version": 1,
  "status": "running",
  "started_at": "2026-04-01T10:00:00Z",
  "last_tick_at": "2026-04-01T10:25:00Z",
  "tick_count": 5,
  "tick_in_progress": null,

  "workspaces": {
    "auth-middleware": {
      "story_id": "PROJ-003",
      "phase": 5,
      "phase_name": "code-review",
      "story_status": "in_progress",
      "completion_pct": 60,
      "pr_url": null,
      "cost_usd": null,
      "completed_at": null,
      "failure_log": null,
      "last_action": "review_triggered",
      "last_action_at": "2026-04-01T10:20:00Z",
      "code_review_triggered": true,
      "code_review_triggered_at": "2026-04-01T10:20:00Z",

      "eval_rounds_completed": 0,
      "consecutive_stuck_ticks": 0,
      "last_tmux_hash": null,
      "blockers": []
    }
  },

  "escalations": [
    {
      "type": "design_needed",
      "story_id": "PROJ-010",
      "workspace": null,
      "reason": "Complexity L with 1 acceptance criterion, UI-heavy activity",
      "details": "Story 'Add settings page' needs UI/UX decisions: layout structure, form grouping, navigation pattern. The current spec has only 'settings page exists' as acceptance criteria.",
      "expected_human_action": "Run /design PROJ-010 to refine the spec with concrete acceptance criteria, or add criteria directly to product-context.yaml. Then run /autopilot start to resume.",
      "created_at": "2026-04-01T10:15:00Z",
      "acknowledged": false
    }
  ],

  "stats": {
    "stories_completed": 3,
    "stories_failed": 0,
    "total_ticks": 5,
    "total_cost_usd": 12.5
  }
}
```

### Field Reference

#### Top-level

| Field              | Type         | Description                                                                 |
| ------------------ | ------------ | --------------------------------------------------------------------------- |
| `version`          | number       | Schema version (currently 1)                                                |
| `status`           | enum         | `"running"`, `"paused"`, `"stopped"`                                        |
| `started_at`       | string       | ISO8601 timestamp of `/autopilot start`                                     |
| `last_tick_at`     | string\|null | ISO8601 timestamp of last completed tick                                    |
| `tick_count`       | number       | Total ticks completed                                                       |
| `tick_in_progress` | string\|null | ISO8601 of currently running tick (tick lock). Null when no tick is active. |

#### Workspace Entry

| Field                      | Type         | Description                                               |
| -------------------------- | ------------ | --------------------------------------------------------- |
| `story_id`                 | string       | Story ID from product-context.yaml                        |
| `phase`                    | number       | Current build phase (0-12) from signal                    |
| `phase_name`               | string       | Human-readable phase name from signal                     |
| `story_status`             | string       | `"in_progress"`, `"in_review"`, `"completed"`, `"failed"` |
| `completion_pct`           | number       | 0-100 from signal                                         |
| `pr_url`                   | string\|null | PR URL once created                                       |
| `cost_usd`                 | number\|null | Accumulated cost from signal                              |
| `completed_at`             | string\|null | ISO8601 completion timestamp                              |
| `failure_log`              | object\|null | Structured failure from signal                            |
| `last_action`              | string       | Last autopilot action for this workspace                  |
| `last_action_at`           | string       | ISO8601 of last action                                    |
| `code_review_triggered`    | boolean      | Whether autopilot has triggered gen/eval                  |
| `code_review_triggered_at` | string\|null | When gen/eval was triggered                               |

| `eval_rounds_completed` | number | How many eval rounds the workspace has completed |
| `consecutive_stuck_ticks` | number | Ticks with no progress change |
| `last_tmux_hash` | string\|null | Hash of tmux pane content at last tick. Used for liveness comparison. Null on first tick or after restart. |
| `blockers` | string[] | Current blockers from signal |

#### `last_action` Values

| Value                   | Meaning                                  |
| ----------------------- | ---------------------------------------- |
| `"launched"`            | Workspace just launched via /launch      |
| `"review_triggered"`    | Gen/eval triggered via tmux              |
| `"review_re_triggered"` | Gen/eval re-triggered after stuck        |
| `"detected_merged"`     | PR detected as merged by workspace agent |
| `"detected_closed"`     | PR detected as closed without merge      |
| `"wrapping"`            | /wrap in progress                        |
| `"nudged"`              | Sent stuck nudge via tmux                |
| `"escalated_stuck"`     | Escalated due to prolonged stuck         |

#### Escalation Entry

| Field                   | Type         | Description                                                                              |
| ----------------------- | ------------ | ---------------------------------------------------------------------------------------- |
| `type`                  | enum         | `"design_needed"`, `"stuck"`, `"failed"`, `"layer_gate_failed"`, `"eval_not_converging"` |
| `story_id`              | string       | Related story ID                                                                         |
| `workspace`             | string\|null | Workspace name (null if not yet launched)                                                |
| `reason`                | string       | One-line reason                                                                          |
| `details`               | string       | Detailed explanation of why escalation triggered                                         |
| `expected_human_action` | string       | What the human should do                                                                 |
| `created_at`            | string       | ISO8601 timestamp                                                                        |
| `acknowledged`          | boolean      | Whether human has seen this                                                              |

---

## `.dev-workflow/autopilot-history.jsonl`

Append-only audit trail. One JSON line per tick.

```jsonl
{"tick":1,"at":"2026-04-01T10:00:00Z","status":"running","actions":["initialized","dispatched PROJ-003"],"workspaces_active":1,"stories_completed_total":0}
{"tick":2,"at":"2026-04-01T10:05:00Z","status":"running","actions":["synced 1 workspace","dispatched PROJ-004"],"workspaces_active":2,"stories_completed_total":0}
{"tick":3,"at":"2026-04-01T10:10:00Z","status":"running","actions":["synced 2 workspaces","triggered review for PROJ-003"],"workspaces_active":2,"stories_completed_total":0}
{"tick":4,"at":"2026-04-01T10:15:00Z","status":"paused","actions":["design escalation for PROJ-010"],"workspaces_active":2,"stories_completed_total":0}
```

---

## `.dev-workflow/autopilot-status.md`

Human-readable status file. Updated at the end of every tick.

```markdown
# Autopilot Status

**Status:** Running
**Started:** 2026-04-01 10:00
**Last tick:** 2026-04-01 10:25 (tick #5)

## Active Workspaces

| Workspace       | Story    | Phase           | Progress | Last Action      |
| --------------- | -------- | --------------- | -------- | ---------------- |
| auth-middleware | PROJ-003 | 5 (code-review) | 60%      | review triggered |
| user-model      | PROJ-004 | 10 (pr-created) | 90%      | detected_merged  |

## Escalations

### PROJ-010: Design Needed (UNRESOLVED)

**Why:** Complexity L with 1 acceptance criterion, UI-heavy activity 'Settings'
**What needs attention:** Story 'Add settings page' needs UI/UX decisions — layout structure, form grouping, navigation pattern
**Expected action:** Run `/design PROJ-010` to refine the spec, then `/autopilot start`

## Stats

- Stories completed: 3
- Stories failed: 0
- Total cost: $12.50
- Total ticks: 5
```

When **paused**, the status file includes additional sections:

```markdown
## PAUSED — Human Attention Required

**Paused at:** 2026-04-01 10:15
**Reason:** Design escalation for PROJ-010

### Why autopilot paused

Story PROJ-010 'Add settings page' was next in the dispatch queue (score: 8.5) but
does not meet the criteria for autonomous implementation:

- Complexity: L (large scope)
- Acceptance criteria: 1 (minimum 3 required for autonomous dispatch)
- Activity: 'Settings' (UI-heavy — requires visual design decisions)

### What decisions need human input

1. **Page layout:** Single page vs tabbed sections vs sidebar navigation
2. **Form grouping:** How to organize settings (profile, notifications, privacy, etc.)
3. **Interaction patterns:** Inline editing vs modal dialogs vs save-all-at-once

### How to resume

1. Run `/design PROJ-010` to work through the design interactively
2. Or add at least 3 specific acceptance criteria to product-context.yaml
3. Then run `/autopilot start` to resume orchestration

### Current state while paused

- 2 workspaces still running (PROJ-003, PROJ-004)
- 3 stories completed so far
- Paused workspaces will continue autonomously
```

---

## Tick Lock Mechanism

The `tick_in_progress` field prevents overlapping ticks:

1. **Before tick:** Read `tick_in_progress`. If set and less than 4 minutes old, skip this tick.
2. **Start of tick:** Set `tick_in_progress` to current timestamp. Write state immediately.
3. **End of tick:** Set `tick_in_progress` to null. Write state.

If a tick crashes (lock never released), the next tick after 4 minutes will clear the stale lock and proceed.

---

## Atomic Write Protocol

To prevent state corruption from mid-write crashes:

```bash
# Write to temp file
cat > .dev-workflow/autopilot-state.json.tmp << 'EOF'
{ ... }
EOF

# Atomic rename (POSIX guarantees this is atomic)
mv .dev-workflow/autopilot-state.json.tmp .dev-workflow/autopilot-state.json
```

The autopilot skill instructs the agent to use this pattern. In practice, the agent writes the file and the filesystem handles atomicity.
