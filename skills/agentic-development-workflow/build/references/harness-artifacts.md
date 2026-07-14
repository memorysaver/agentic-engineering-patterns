# Phase 0 Harness Artifact Bodies

Load this when generating the `.dev-workflow/` tracking artifacts in `/aep-build`
Phase 0. Each artifact below has exactly one template. The **sprint-contract**
format lives in [`contract-template.md`](contract-template.md); the **progress
file** format lives in [`progress-template.md`](progress-template.md). This file
holds the other four: `feature-verification.json`, `init.sh`, `status.json`, and
`lessons.md`.

---

## `feature-verification.json` (Phase 0 step: feature verification list)

Extract each task's verification steps from `contracts.md` into
`.dev-workflow/feature-verification.json`:

```json
[
  {
    "task": "<task description>",
    "commit_sha": null,
    "verification_steps": ["step 1", "step 2", "step 3"],
    "passes": false,
    "evaluated_by": null,
    "round": null
  }
]
```

- `commit_sha` starts `null`; the generator fills it (8-char prefix) after each
  task commit in Phase 4.
- JSON is intentional — models tamper with JSON less than Markdown.
- **Generator ownership rule:** the generator writes only `commit_sha`. It MUST
  NOT modify `verification_steps` or `passes` — only the evaluator (or a human)
  writes `passes` / `evaluated_by` / `round`.

---

## `init.sh` (Phase 0 step: session recovery script)

Create `.dev-workflow/init.sh` for resuming after a context reset, then
`chmod +x .dev-workflow/init.sh`:

```bash
#!/bin/bash
# Session recovery script — run this to resume after context reset
set -e
cd "$(dirname "$0")/.."

# Project setup (deps, dev server, ports)
SETUP_HOOK=.claude/hooks/workspace-setup.sh
if [ -f "$SETUP_HOOK" ]; then
  bash "$SETUP_HOOK"
else
  echo "No workspace setup hook found. Check project README for setup instructions."
fi

# Source ports (written by setup hook)
source .dev-workflow/ports.env 2>/dev/null

# Current state
echo "=== Branch & Commits ==="
echo "Branch: $(git branch --show-current)"
git log --oneline "$(git config --get aep.integration-branch 2>/dev/null || (git show-ref --verify --quiet refs/remotes/origin/develop && echo develop || echo main))"..HEAD 2>/dev/null || git log --oneline -10

echo "=== Progress ==="
grep '\[x\]' .dev-workflow/progress-*.md 2>/dev/null | tail -10

echo "=== Next Phase ==="
grep '\[ \]' .dev-workflow/progress-*.md 2>/dev/null | head -3
```

---

## `status.json` (Phase 0 step: initialize inter-agent signals)

Create `.dev-workflow/signals/status.json`:

```json
{
  "phase": 0,
  "phase_name": "initializing",
  "task_current": null,
  "task_index": 0,
  "task_total": 0,
  "started_at": "<ISO 8601 timestamp>",
  "blockers": [],
  "completion_pct": 0,
  "last_updated": "<ISO 8601 timestamp>",
  "story_status": "in_progress",
  "pr_url": null,
  "cost_usd": null,
  "completed_at": null,
  "failure_log": null,
  "blocked_on": null
}
```

Full signal-file specification: `/aep-launch` references/signals-spec.md.

---

## `lessons.md` (Phase 0 step: create the lessons file)

```bash
cat > .dev-workflow/lessons.md <<'TEMPLATE'
# Lessons: <change-name>

Module: <module>
Activity: <activity>
Date: <date>
Story: <story-id>

## Solutions

## Errors

## Missing

## Summary for Next Agent
TEMPLATE
```

Fill the header fields from the OpenSpec change. `## Solutions` / `## Errors` /
`## Missing` are populated opt-in during Phase 4; `## Summary for Next Agent` is
written in Phase 9.
