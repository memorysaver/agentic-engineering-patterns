# Context reverse edges and container completeness

Status: proposal implemented at the user's request on 2026-09-17; acceptance and release remain the user's decision
Date: 2026-09-17
Evidence: [looplia basic user access delivery](../lessons/2026-09-15-looplia-basic-user-access-delivery.md) (sections "Against the open v5 items", "Rule audit", turns six through ten)
Related: [layer/wave encapsulation](aep-v5-layer-wave-encapsulation.md) (CLI candidates), [research and draft context](aep-v5-research-draft-context.md)

## Observed structural failures

Measured on looplia at `aep 5.0.0-preview.1` build `3fae0e42…` after two days of native deliveries:

1. `aep context basic-user-access` returned 200 records and none of the layer's three member stories or two changes, because the layer's `refs` was empty and the CLI only followed forward links. Members had `layer: basic-user-access` set, as the design guidance instructs. `aep check` passed with no diagnostic.
2. `aep context BASIC-001` did not include lesson `basic-test-interface-drift`, whose `refs` names BASIC-001. Lessons were reachable only through `aep query --kind lesson`, `aep lesson find` or their path, so the project rule "recall applicable lessons during context assembly" could not be satisfied by the context command.
3. Release `basic-access-production-20260915` selected BASIC-001 and BASIC-002 in `refs` and carried the deployed SHA, canary and journey results in `data`; neither story's context included it.
4. A planning turn run against this state organized the handoff by user journeys, named no container, and omitted a member story whose main unknown had been exercised by three successful deployments. The status guidance asks for reporting by container; nothing in the CLI output pointed at one.
5. Two later production promotions had no release record at all; the deployed revision lived only in evidence files.
6. A native-verified candidate failed CI on the project's admission-manifest gate, which was not a configured check. The agent then registered it. Secret scanning flagged record digests four times, one allowlist entry per event.

These are structural: they hold for any model, they are visible in CLI output, and they were reproduced by the observer with the CLI alone.

## Decision

Rust changes, all read-side or advisory:

- `aep context` follows incoming edges in addition to forward links and incoming accepted decisions: records whose `layer` or `wave` field names the subject, and lessons or releases whose `refs` select the subject. Incoming edges are followed for the subject only; each reached record then expands through its own forward links and accepted decisions. A first build that followed incoming edges transitively expanded every looplia context to 690 of 913 records, because any path reaching an imported layer pulled in all of that layer's members; restricting incoming edges to the subject keeps the same subjects near their previous size while adding the members, lessons and releases that were missing. Verification fingerprints and `linked_context` for dispatch are unchanged; lessons and releases still do not become verification inputs.
- `aep_core::advisories` reports a layer or wave with no members (no forward `refs` to a record and no record naming it) as an `empty_container` diagnostic with `severity: warning`. Superseded and cancelled containers are skipped.
- `aep check` appends advisories, reports `warnings` as a count, and computes `pass` and the exit code from error-severity diagnostics only. `validate` is untouched, so record writes and config updates are never blocked by a warning.
- Human output prefixes warning diagnostics and the context footer names the new edges.

Guidance changes, one to three sentences each, no new imperatives:

- design records reference: state that container expansion works through `refs` or members' `layer`/`wave` fields and that `check` warns on an empty container.
- deliver closure reference: record each environment promotion with `aep release new`/`aep release update`, selecting promoted stories in `refs` and retaining the deployed revision and results in `data`.
- project verification-setup reference: register every CI or deployment gate as a configured check; scope secret-scanner allowlists for record digests by path and field.

## Not changed

- No new record kind, no reverse edge for story-to-story `refs` (that expansion was already observed to pull most of imported history), no policy switch. If the empty-container warning proves noisy on migrated projects, a `policy` flag to silence it is the next candidate, not a change to the edge rules.
- Sibling stories that bypass an imported dependency wall, direct pushes of records to the integration branch, and promotion without renewed authorization are project policy and agent behavior, outside this change.
- The research/draft landing gap remains with its own proposal.

## Verification

- Rust fixture: a layer with an empty `refs` and one member story that sets `layer`; a lesson and a release with `refs` to the story. `aep context <layer>` includes the story and its change; `aep context <story>` includes the layer, the lesson and the release and not the memberless layer; `aep check` passes with one `empty_container` warning for the memberless layer and exit code 0, and the warning clears once a story sets that layer. The existing decision-context fixture continues to prove that a lesson with `refs` to the story leaves the `verify plan` fingerprint unchanged.
- Existing workspace checks: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `bun run skills:check-steering`, `bun run skills:check`.
- Downstream: after installing the rebuilt binary as the shared `aep`, rerun on looplia `aep context basic-user-access --json` (members present), `aep context BASIC-001 --json` (lesson and release present), and `aep check` (PASS with warnings listed, exit 0). Record the before/after digests and outputs in the trial bundle.
