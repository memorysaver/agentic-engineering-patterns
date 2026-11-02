# AEP 5.0 runbooks and principles

Add runbooks and engineering principles to the standalone AEP CLI. The existing rules, ledger and lessons remain unchanged. The implementation adds a loader, progress checker and regression fixtures.

Procedures retain their existing format. A runbook declares ordered steps with IDs, actions, evidence kinds and optional admitted principle citations. The catalog identifies runbooks and rejects malformed definitions. The first bundled runbook covers bug reproduction, root cause, a failing test, a minimal fix, verification and lessons.

Four principles are served with triggers, rules and honest enforcement limits. Project-specific occurrences remain in each project’s own lesson store; the catalog distinguishes guidance from observed evidence.

`--skill ... --attempt ID` records successful reads under the Git common directory, outside tracked content. Ordinary reads remain free of writes; dry runs and refused reads produce no receipt. `runbook check` validates progress for an existing attempt, evidence declarations, configured check and record references, and principle reads before step completion. It executes no steps and does not substitute for verification or delivery gates. Text evidence remains an assertion for review, not proof of command execution.

The schema stays at 1. Stable release pins remain explicit; preview projects use config update and rerun verification with the new binary.
