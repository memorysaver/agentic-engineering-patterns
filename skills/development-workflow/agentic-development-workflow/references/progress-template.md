# Development Progress

| Field | Value |
|-------|-------|
| **Feature** | <!-- feature name --> |
| **Change ID** | <!-- jj change ID --> |
| **Started** | <!-- YYYY-MM-DD --> |
| **OpenSpec Change** | <!-- change name --> |
| **Mode** | <!-- full / light --> |
| **Evaluator** | <!-- yes / no --> |

---

## Part A — Scaffold

- [ ] Project scaffolded via Better-T-Stack
- [ ] OpenSpec initialized
- [ ] Build verified

## Part B — Design (on main)

- [ ] Phase 1: OpenSpec Explore
- [ ] Phase 2: OpenSpec Propose
- [ ] Phase 3: Design Review
- [ ] Artifacts committed to main

## Part C — Launch Workspace

- [ ] Workspace created (`jj workspace add`)
- [ ] tmux/cmux session started
- [ ] Bootstrap prompt sent
- [ ] Evaluator agent launched (full mode only)

## Part D — Implementation (in workspace)

- [ ] Phase 0: Tracking initialized
  - [ ] Progress file created
  - [ ] jj change stack created (skeleton-first)
  - [ ] Dependencies installed
  - [ ] Dev server running
  - [ ] Port config written
  - [ ] Sprint contracts generated (`.dev-workflow/contracts.md`)
  - [ ] Feature verification list generated (`.dev-workflow/feature-verification.json`)
  - [ ] Session recovery script generated (`.dev-workflow/init.sh`)
  - [ ] Inter-agent signals initialized (`.dev-workflow/signals/`)
- [ ] Phase 4: OpenSpec Apply (jj edit each change)
  - [ ] Task 1: <!-- task description -->
  - [ ] Task 2: <!-- task description -->
  - [ ] Task 3: <!-- task description -->
- [ ] Phase 5: Code Review & Verification
  - [ ] Completeness check (per-change review)
  - [ ] Contracts verified (`.dev-workflow/contracts.md`)
  - [ ] Quality review (evaluator or self-review)
  - [ ] Evaluator round 1: <!-- PASS/FAIL + summary -->
  - [ ] Evaluator round 2: <!-- if needed -->
  - [ ] `feature-verification.json` updated
  - [ ] Issues fixed
- [ ] Phase 6: Browser Testing (Dogfood)
  - [ ] Dogfood report created
  - [ ] Issues fixed
- [ ] Phase 7: E2E Test Scripts
  - [ ] Test scripts generated
  - [ ] Tests passing
- [ ] Phase 8: Review Results
  - [ ] All results reviewed
  - [ ] No blocking issues
- [ ] Phase 9: Cleanup & Publish
  - [ ] Changes split/squashed as needed
  - [ ] Rebased onto latest main
  - [ ] Bookmark created and pushed
- [ ] Phase 10: Create PR
  - [ ] PR created
  - [ ] PR URL: <!-- url -->
- [ ] Phase 11: PR Review Loop
  - [ ] Round 1: <!-- status -->
- [ ] Phase 11.5: Human Evaluation & Iteration
  - [ ] Iteration round 1: _[findings summary]_
    - [ ] Findings documented
    - [ ] Code fixed (jj edit + squash)
    - [ ] OpenSpec change aligned
    - [ ] Re-tested
    - [ ] Pushed
  - [ ] Iteration round 2: _[findings summary]_
- [ ] Phase 12: Merge
  - [ ] CI green
  - [ ] Reviews resolved
  - [ ] User confirmed
  - [ ] Merged

## Part E — Post-Merge (on main)

- [ ] Phase 13: Archive & Cleanup
  - [ ] Fetched merged state (`jj git fetch`)
  - [ ] Dev server stopped
  - [ ] `/opsx:archive` run
  - [ ] Archive committed + pushed
  - [ ] Workspace forgotten (`jj workspace forget`)
