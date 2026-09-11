# Publication after integration context changes

Date: 2026-09-11
Status: Accepted implementation of the user's request to fix this trial's findings and reinstall
Evidence: [MITS-108 observation](../lessons/2026-09-11-mits108-development-observation.md)

## Problem and reproduced boundary

MITS-108 integrated its implementation, then merged additional change-directory context into the shared store. Publication required the integration receipt's fingerprint to equal the current verification fingerprint even after new checks and review passed. The agent reopened the story and merged a second records-only PR to obtain a new receipt. `deliver plan` and merge dry run also claimed readiness while the actual merge required an active attempt that the integrated story no longer had.

A disposable native lifecycle reproduces the publication failure with one fixed executable: integrate a checked/reviewed candidate, add a retained change-directory artifact to shared context, rerun checks and review at the same candidate, then publish. The previous publisher rejects it. This establishes the shared-context failure independently of the mixed-build limitation in the live observation.

## Decision

Keep integration receipts immutable. They describe the verified candidate, observed merge revision, evidence and context at integration time. Publication separately records the current context fingerprint and passing check/review IDs, alongside the original integration fingerprint and receipt ID. Recheck that the recorded merge tree exactly matches the candidate Git tree. Current-context validation remains bound to that candidate; changed/dirty verification worktrees, failed/stale checks, unresolved findings, required review and unavailable/different integration trees remain blockers.

Context-only changes can therefore use `verify run`, current required review and `spec publish` on the integrated story without another attempt or merge. This does not authorize a new product scope or reinterpret accepted contracts: changed intent/scope still uses the appropriate new change/story. It also does not certify the current moving main branch or a deployment; the publication names its exact historical integration and currently revalidated implementation.

Delivery plans expose verification readiness separately from whether another integration action applies. An integrated/released story has `candidate_ready` according to current evidence, `integrated=true`, and `eligible=false` for another PR/merge. Both actual PR/merge and their dry runs direct it toward closure. Local checks for an existing/missing PR, integration checkout cleanliness and candidate ancestry apply to both real actions and dry runs. Provider state is checked on actual execution; plan/dry-run output states that boundary.

## Producer provenance and Git transport

Verification plans and check receipts include the executing AEP version and binary SHA-256. Reviews retain request and response producer identities; delivery retains intent and integration producer identities; publication records its producer. These fields identify builds sharing one preview version, without turning every executable rebuild into an automatic context change. They are attribution, not authenticity/signature guarantees, and existing records are not backfilled with invented identities. Controlled trials should use immutable executable paths or explicitly retain cutovers.

Shared-store transport remains an explicit Git operation. Reconcile actual product integration into the shared checkout before publication/closure. For conflicting story snapshots, inspect both parent versions and referenced attempt/delivery records, preserve historical receipts and unrelated edits, and choose the current confirmed pointer. Guidance explicitly avoids whole-file ours/theirs replacement. It does not add an automatic semantic conflict resolver. Retrospective probe artifacts normally belong in lessons/evidence; putting them under a linked change intentionally changes verification context.

## Verification

Native lifecycle fixtures cover same-build reproduction/recovery, retained original delivery/story records, stale review rejection, moved worktree rejection, publication provenance and matching Git trees. Existing provider tree-drift and failed-evidence fixtures remain applicable. Run workspace tests, format/clippy, skill checks, then exercise a copied release executable before installation. Live MITS records remain untouched; future live-agent behavior and macOS execution remain separate observations.
