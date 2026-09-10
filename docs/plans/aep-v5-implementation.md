# AEP 5.0 implementation

Implement the merged [5.0 design](../decisions/aep-v5-rust-cli-architecture.md) on `feat/aep-5-native-cli`. Agent reasoning selects skills and work order. Rust serves content, maintains records, evaluates declared constraints, and executes requested operations.

- [x] Native workspace, JSON/exit contract, versioned records, graph checks.
- [x] File transactions, conflict detection, interrupted-write recovery.
- [x] Embedded skill catalog, references, local procedures, minimal project setup.
- [x] Ledger, roadmap/ADR, change/BDD/spec, rule and lesson maintenance.
- [x] Worktree attempts, verification/review receipts, gates and delivery.
- [x] Current-context migration and OpenSpec import/export/reference checks.
- [x] Native installation/release documentation and CI.
- [x] Lifecycle and failure fixtures; independent subagent code review and fixes.

The [CLI-first scope correction](../decisions/aep-v5-cli-first.md) defers dashboard
implementation and packaging. Existing web source is retained at its pre-v5 state;
the native deliverable has no dashboard command or consumer.

The [24-skill capability inventory](../audits/2026-09-10-aep-v5-skill-migration-inventory.md)
records the initial gaps. The user's subsequent direction centers AEP on context
completeness and self verification, leaving generic patterns to model autonomy.
The [accepted classification](../decisions/aep-v5-context-and-self-verification.md)
maps every legacy capability to that direction and adds verification with
prototypes. The [preview adoption decision](../decisions/aep-v5-preview-adoption.md)
adds explicit v4/v5 routing and source-preserving migration. Final behavior and
validation evidence are tracked separately from acceptance of those decisions.

- [x] Build the first context/self-verification slice in project and validate, with an executable project-owned procedure and Feature Map.
- [x] Align review policy and evidence contracts with autonomy, retaining explicit project requirements and current revision/environment checks.
- [x] Add grounded design criteria, prototype exploration, and task context handoff through roadmap/design/implement.
- [x] Complete delivery/learning/migration guidance and record bounded cold-selection, recovery and evidence-retention observations.
- [ ] Observe actual downstream Feature Map maintenance and a real prototype-driven implementation.
- [x] Finish preview source-preserving migration and version routing, including consumer diagnostics and collision/drift protection.
- [x] Complete independent cross-review and package `5.0.0-preview.1` for downstream trials.

The completed embedded-catalog item above establishes packaging of the current
eight procedures, not full capability coverage of the 24 legacy skills. Evaluate
the proposed destinations and explicit exclusions rather than restoring every
legacy pattern. Generic scheduling, host backends, and source-ingestion engines
are outside the proposed core; selected adapters need their own evidence.

Remaining release/adoption evidence is tracked separately from implemented code:

- [ ] Representative production downstream pilot with the target project's actual checks and constraints.
- [ ] Linux/macOS CI for the final release candidate.
- [ ] Authorized version tag and native artifact publication.

Validation distinguishes deterministic CLI fixtures from agent behavior observations. Platform support and external provider operations are reported only where exercised. This branch prepares 5.0; publication and downstream production migration are separate operations.

See the [implementation audit](../audits/2026-09-09-aep-v5-implementation.md) for executed checks, review closure, and release limits.

The [preview audit](../audits/2026-09-10-aep-v5-preview.md) tracks the subsequent accepted decisions, implementation, review findings, and candidate validation.
