# Complete AEP 5 CLI before dashboard work

**Status:** Accepted from the user's 2026-09-09 scope correction.

The current deliverable is the standalone Rust AEP 5 CLI and its embedded
guidance. Dashboard implementation and packaging are deferred. The native release
does not include a web server, JavaScript runtime, web assets, or a dashboard
command. Read-only project inspection uses `status`, `query`, `context`, and
`timeline`, including their JSON output.

The existing Web App is retained at its pre-v5 baseline (`b47d143`) outside this
deliverable. Reverting the v5 integration does not establish a backward-compatible
consumer for migrated projects. A later web redesign is separate work.

Keep migration as an explicit conversion into v5 project stores. After cutover,
the CLI operates on native records; it does not maintain an old live data model
or require a backward-compatible web consumer.

Complete and validate native guidance discovery, project setup/configuration,
records and contracts, isolated worktrees/attempts, checks/reviews/gates,
integration/publication receipts, migration, and learning. Native installation,
archive verification, and Linux/macOS CI remain the release boundary. A future
dashboard needs a separate design and request; its work is not a prerequisite for
finishing the CLI.

## Acceptance

- The installed `aep` executable serves embedded guidance outside this checkout
  without a JavaScript runtime or web files.
- With only Git on PATH, it can initialize a project, create a story, and inspect
  its native records/readiness through ordinary CLI commands without mutating
  those records during reads.
- `aep --help` exposes the native command families; dashboard implementation and
  dashboard jobs are absent from the v5 native deliverable.
- Lifecycle/failure fixtures verify native behavior. Report production migration,
  provider integration, platform CI, and release publication only where actually
  performed; missing external evidence is not replaced by a completion label.
