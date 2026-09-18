# Corrections from the live downstream delivery trial

Date: 2026-09-11
Status: Accepted implementation of the user's request to fix the new 5.0 findings
Evidence: [MITS delivery continuation](../lessons/2026-09-11-mits-delivery-continuity-observation.md)

## Follow-up decisions and immutable context

The downstream agent corrected its own candidate-only interpretation with a new accepted decision referencing the immutable story. Inspection and verification followed outgoing references only, so that decision was invisible from the story and absent from its verification fingerprint. A separate delivery-context file happened to invalidate evidence in MITS; the decision itself was not recognized. An independent reviewer reproduced the missing edge with the previous executable in a disposable project.

Include incoming accepted decisions when traversing task context and the linked context used by verification. Follow later decisions that reference earlier decisions, retaining superseded decision anchors and their accepted successors. Native `decision supersede` must preserve discoverability and keep older evidence stale, including when the successor refers only to the earlier decision. Preserve both original and correcting records; the agent reconciles their meaning with the user's current intent and actual authority. Acceptance does not authorize an action by itself, and this is not an automatic latest-record-wins resolver.

Reverse accepted decision references and superseded decision history. Draft proposals, unrelated decisions and incoming operational receipts/lessons do not become verification inputs merely by naming the story. Explicit outgoing links retain their existing meaning. The same verification context determines dispatch's committed-base requirements. A relevant later accepted decision invalidates old candidate evidence even when code HEAD remains unchanged.

Keep accepted-record immutability and story scope checks. Improve the update error and record-authoring guidance to explain a linked decision for a context clarification. Changed acceptance or implementation scope requires a new change/story, not a decision that bypasses the existing contract.

## Shared ledger delivery

Keep code in the candidate worktree and later attempt/check/review/delivery records in the shared store. Accepted dispatch context is committed in the base. Additional operational records copied into the candidate remain subject to ordinary scope checks.

Document the remaining transport responsibility: retain the shared store on its context/integration branch, reconcile it with actual product integration, resolve old-state conflicts using provider/attempt/delivery evidence, then verify and synchronize the combined state before resource cleanup. This preserves the guard that exposed the problem rather than exempting all managed-store paths or implementing an automatic Git conflict resolver.

## Project skill provenance and reflection

Retain the user-endorsed single `aep --skill` discovery/read interface. Label built-in and project procedures separately; show project source paths. Structured source/digest metadata and exact named SKILL.md/reference output remain unchanged. Local procedures remain project-owned and can evolve through reflect, without making AEP depend on a downstream product. Guidance covers when reusable knowledge belongs in a skill and how to verify discovery and a representative use.

## Specification formatting

The publisher joined every requirement with two newlines, including the final requirement when no suffix followed. Render blank separators between requirement blocks and a single final newline in that case. Preserve document prefix/suffix and existing no-op publication bytes. This does not rewrite already published downstream specifications or alter acceptance semantics.

## Validation boundaries

Real subprocess fixtures cover reverse-decision discovery, stale verification, immutable history, committed dispatch context, separate-store scope enforcement, catalog provenance and published-spec whitespace. Existing suites cover isolated provider mocks, lifecycle completion, and exact embedded guidance. A copied release executable must pass the standalone preview procedure and be installed with source/digest provenance. This correction does not rerun or mutate MITS delivery, and local fixtures do not establish cross-platform or new live-agent behavior.
