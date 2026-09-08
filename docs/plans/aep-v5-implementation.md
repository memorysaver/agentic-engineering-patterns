# AEP 5.0 implementation

Implement the merged [5.0 design](../decisions/aep-v5-rust-cli-architecture.md) on `feat/aep-5-native-cli`. Agent reasoning selects skills and work order. Rust serves content, maintains records, evaluates declared constraints, and executes requested operations.

- [x] Native workspace, JSON/exit contract, versioned records, graph checks.
- [x] File transactions, conflict detection, interrupted-write recovery.
- [x] Embedded skill catalog, references, local procedures, minimal project setup.
- [x] Ledger, roadmap/ADR, change/BDD/spec, rule and lesson maintenance.
- [x] Worktree attempts, verification/review receipts, gates and delivery.
- [x] Current-context migration and OpenSpec import/export/reference checks.
- [x] Dashboard consumer, installation/release documentation and CI.
- [x] Lifecycle and failure fixtures; independent subagent code review and fixes.

Validation distinguishes deterministic CLI fixtures from agent behavior observations. Platform support and external provider operations are reported only where exercised. This branch prepares 5.0; publication and downstream production migration are separate operations.

See the [implementation audit](../audits/2026-09-09-aep-v5-implementation.md) for executed checks, review closure, and release limits.
