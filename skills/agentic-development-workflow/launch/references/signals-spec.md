# Inter-Agent Signals Specification

The `.dev-workflow/signals/` directory provides file-based communication between the workspace agent and the main session. This follows Anthropic's pattern of agents communicating through files rather than chat.

Anthropic's harness design research uses file-based communication between agents — one agent writes a file, another reads and responds by creating a new file or updating in place. This avoids the ambiguity of chat-based coordination and maintains structured context.

**Source:** [Harness Design for Long-Running Application Development](https://www.anthropic.com/engineering/harness-design-long-running-apps)

---

## Directory Structure

```
.dev-workflow/signals/
├── status.json              # Workspace agent writes — current phase and progress
├── feedback.md              # Main session writes — mid-flight feedback
├── ready-for-review.flag    # Workspace agent creates — signals human eval needed
├── eval-request.md          # Generator writes — requests evaluator review
└── eval-response-<N>.md     # Evaluator writes — evaluation results per round
```

---

## Signal Files

### `status.json` — Progress Signal

**Written by:** Workspace agent (generator)
**Read by:** Main session
**Updated:** At the start of each phase and on blockers

```json
{
  "phase": 4,
  "phase_name": "implementing",
  "task_current": "Add user login form",
  "task_index": 2,
  "task_total": 5,
  "started_at": "2026-03-25T10:00:00Z",
  "blockers": [],
  "completion_pct": 40,
  "last_updated": "2026-03-25T11:30:00Z"
}
```

**Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `phase` | number | Current phase number (0–13) |
| `phase_name` | string | Human-readable phase name |
| `task_current` | string | Current task being worked on (Phase 4 only) |
| `task_index` | number | 1-based index of current task |
| `task_total` | number | Total number of tasks |
| `started_at` | string | ISO 8601 timestamp of phase start |
| `blockers` | string[] | List of blockers preventing progress |
| `completion_pct` | number | Estimated completion percentage (0–100) |
| `last_updated` | string | ISO 8601 timestamp of last update |

**Update points:**
- Phase 0: After initialization completes
- Phase 4: At start of each task and on completion
- Phase 5–8: At start and completion of each phase
- Phase 9–12: At each sub-step (push, PR create, merge)
- On blockers: Add to `blockers` array immediately

### `feedback.md` — Main Session Feedback

**Written by:** Main session (user or main agent)
**Read by:** Workspace agent
**Format:** Append-only markdown

```markdown
# Feedback

## 2026-03-25 11:45
Priority: high
Focus on the auth flow first — the settings page can wait until a follow-up PR.

## 2026-03-25 14:20
Priority: low
The button colors look off on dark mode. Not blocking.
```

**Rules:**
- Main session appends new entries with timestamp and priority
- Workspace agent checks for new feedback at the start of each phase
- High-priority feedback should be addressed before continuing
- Low-priority feedback can be deferred to Phase 11.5

### `ready-for-review.flag` — Review Signal

**Created by:** Workspace agent
**Read by:** Main session
**Format:** Empty file or single-line description

The workspace agent creates this file when it reaches Phase 11.5 (human evaluation) to signal that the feature is ready for human testing.

```bash
# Workspace agent creates:
echo "Feature ready for testing at http://localhost:3000" > .dev-workflow/signals/ready-for-review.flag
```

The main session can watch for this file:
```bash
# Main session polls (or uses filesystem watcher):
cat .claude/workspaces/<name>/.dev-workflow/signals/ready-for-review.flag
```

### `eval-request.md` — Evaluation Request

**Written by:** Generator agent
**Read by:** Evaluator agent
**Created:** When generator completes Phase 4 and is ready for evaluation

```markdown
# Evaluation Request

**Round:** 1
**Date:** 2026-03-25
**Requested by:** generator

## What to evaluate
- All tasks in the jj change stack are implemented
- Dev server running on port 3000 (web) / 3001 (server)

## Changes since last round
- [First evaluation — all changes are new]

## Known issues
- [Any issues the generator is already aware of]

## Files changed
[Output of jj diff --stat]
```

### `eval-response-<N>.md` — Evaluation Response

**Written by:** Evaluator agent
**Read by:** Generator agent
**Format:** Follows the structure defined in `evaluator-criteria.md`

See the "Evaluation Protocol" section in `evaluator-criteria.md` for the response format.

---

## Reading Signals from Main Session

The main session can check workspace progress without interrupting the agent:

```bash
# Check current phase and progress
cat .claude/workspaces/<name>/.dev-workflow/signals/status.json | jq .

# Check if ready for review
ls .claude/workspaces/<name>/.dev-workflow/signals/ready-for-review.flag 2>/dev/null

# Send feedback
cat >> .claude/workspaces/<name>/.dev-workflow/signals/feedback.md << 'EOF'

## 2026-03-25 14:00
Priority: high
The API response format changed — check the latest main for updates.
EOF
```

---

## Signal Polling

Agents should check for signal files **at phase boundaries**, not continuously:

- **Generator** checks `feedback.md` at the start of each new phase
- **Evaluator** checks `eval-request.md` after completing its bootstrap (initial read of specs/contracts) and after writing each eval-response
- **Main session** checks `status.json` and `ready-for-review.flag` when the user wants a progress update

There is no filesystem watcher or continuous polling. Agents read signal files at natural transition points in the workflow.

If a signal file doesn't exist yet, skip it and continue — it will be checked again at the next phase boundary.

---

## Lifecycle

1. **Phase 0:** Create `.dev-workflow/signals/` directory, initialize `status.json`
2. **Each phase:** Update `status.json` at start and end
3. **Phase 4:** Update `task_current`, `task_index` as tasks progress
4. **Phase 5:** Generator creates `eval-request.md` if evaluator is running
5. **Phase 5:** Evaluator writes `eval-response-<N>.md`
6. **Phase 11.5:** Create `ready-for-review.flag`
7. **On blockers:** Update `status.json` blockers immediately
8. **On feedback:** Check `feedback.md` at start of each phase
