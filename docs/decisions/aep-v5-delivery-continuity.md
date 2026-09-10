# Native delivery continuity

Date: 2026-09-11
Status: Accepted for the 5.0 preview correction requested by the user

## Observed problem

The MITS agent received “這次 reflect 產生的故事是哪一個？自主完成”, implemented MITS-109, repaired failed verification, obtained review and reached delivery eligibility. It ended at a ready candidate. No PR, integration, specification publication, change closure or resource reconciliation followed. See [the observation](../lessons/2026-09-10-mits-reflection-to-implementation.md).

Candidate readiness did not establish that the user's overall outcome was satisfied. The native implement/validate guidance lacked an explicit continuation responsibility; deliver guidance described commands without first recovering the requested endpoint. The human CLI displayed `Eligible: true` without explaining what remained.

## Decision

Preserve the requested outcome and existing authority across implementation, validation, worker handoffs and recovery. For end-to-end work, validation hands control to delivery and closure. A worker finishing its bounded implementation assignment returns responsibility to its coordinator. The agent selects the work order; this adds no deterministic orchestration or fixed generator/evaluator topology.

The canonical completion and authority rules live in `skills/native/deliver/references/closure.md`. Root, implement and validate provide concise continuation pointers. Explicit analysis, review, candidate-only and PR-only requests retain their bounded scope. Existing authorization carries forward. General autonomy does not invent a merge target or public-release authority. When authority or the target is missing, finish authorized preparation into a concrete reviewable result and identify the remaining decision; report partial completion rather than silently redefining the goal.

After confirmed integration, carry out applicable combined checks, spec publication, change closure, durable evidence/lessons and owned-resource reconciliation. Preserve unintegrated or unrelated work. A PR-only endpoint retains its pending integration resources with an explanation. Release/deployment remains a distinct requested outcome.

`deliver plan` and delivery dry runs report readiness and a pointer to delivery guidance. Their JSON retains the existing eligibility/evidence fields and adds guidance. These read/dry-run paths continue to perform no delivery or cleanup. The CLI does not infer user intent or authorize external actions.

## Verification

Exercise real CLI readiness and delivery dry runs in a disposable Git lifecycle. Check that story state, delivery records, Git head/status and worktree inventory remain unchanged before explicit local integration. Keep full lifecycle assertions for integrated story, spec publication and change closure.

Evaluate candidate instructions independently for end-to-end work with existing merge authority, PR-only, review-only, candidate-only and missing merge authority. These are simulated guidance observations, separate from executed CLI fixtures and a future live MITS continuation. Ship the embedded guidance and executable together; retain copied-binary evidence.

## Scope

This correction restores responsibility previously split between 4.x build and wrap within native implement/validate/deliver. It leaves native stores, v4 guidance, context fingerprinting and downstream product changes intact. Further live observation is needed to establish whether downstream agents consistently perform the complete requested lifecycle.
