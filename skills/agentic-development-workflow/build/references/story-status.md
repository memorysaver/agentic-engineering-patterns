# Story-Status Tracking (Product-Cycle Mode Only)

Load this when the OpenSpec change maps to a story in `product-context.yaml`
(the change name matches a story's `openspec_change` field). If
`product-context.yaml` does not exist, this is standalone feature mode — skip
this file entirely (signal updates to `status.json` still work; just omit the
`story_status` fields).

---

## Build-Time Dependency Re-Verification (Phase 0)

Before implementation, re-verify that every dependency is still completed:

```
Read product-context.yaml (READ-ONLY — workspace agents never write to this file)
Find this story by openspec_change match
For each dependency in story.dependencies:
  If dependency.status != completed:
    ABORT build
    Signal via .dev-workflow/signals/status.json:
      { "phase": 0, "phase_name": "dependency_check_failed",
        "blockers": ["<dep_id> is not completed"] }
    Stop and wait for the main session to investigate
```

Also compare `dispatched_at_epoch` vs the current `dispatch_epoch`. If the epoch
advanced since dispatch, re-read the YAML for architecture amendments.

**Why:** a dependency could be rolled back after dispatch (e.g. a PR reverted).
This defense-in-depth catches issues that dispatch-time checks missed.

---

## Status Updates via Signals

**Concurrency protocol:** only the main session writes to `product-context.yaml`.
Workspace agents report status through `.dev-workflow/signals/status.json`; the
main session (via `/aep-wrap`, `/aep-dispatch`) reads signals and writes the YAML.

All story-status updates flow through the signal file, never direct YAML writes:

- **Phase 0 start:** confirm story status is `in_progress` in the YAML (read-only).
- **Phase 10 (PR created):** set `status.json` `story_status: "in_review"` + `pr_url`.
- **Phase 12 (merge):** set `story_status: "completed"`, `completed_at`, `pr_url`,
  `cost_usd`.
- **On failure (escalation):** set `story_status: "failed"` + `failure_log`
  (structured: error_class, approach_summary, unexplored_alternatives).

`/aep-wrap` (on the integration branch) reads these signal fields and writes the
final status to `product-context.yaml`.
