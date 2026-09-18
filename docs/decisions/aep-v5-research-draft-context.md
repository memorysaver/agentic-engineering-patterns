# Research and draft context before implementation

Status: proposal; user requested AEP-managed idea/research/draft classification, concrete schema and CLI design remain open
Date: 2026-09-11
Evidence: [MITS idea capture observation](../lessons/2026-09-11-mits-idea-draft.md)
Related: [purpose-driven research](aep-v5-purpose-driven-research.md)

## Problem

A user may ask only to save an idea. The agent needs to retain its origin, uncertainty and supporting research so a future task can retrieve it, continue exploration, or make a decision. Creating an implementation contract or accepting the idea at capture time would misrepresent the request.

The current design store defaults to `docs/design`, and MITS used that configured path correctly. Native decisions can carry arbitrary artifact paths, but those paths do not give the draft an identity in AEP's record graph or automatically supply its contents to `aep context`. Directory placement, record validity and semantic completeness are separate responsibilities.

## Proposed behavior

- Capture a new idea with its source, intended outcome, current maturity and unresolved questions in the configured design store. Saving an idea requires no commitment to research or implement it.
- Keep research evidence, alternatives and findings associated with that draft. Use research when uncertainty justifies it; do not impose a mandatory sequence of research documents on every idea.
- When the user settles a choice, record the rationale, alternatives and authority in an ADR under the configured roadmap store's `decisions/`, linked to the supporting draft and research. Retain the earlier material as provenance rather than rewriting it into an accepted decision.
- Create a change contract and story when behavior and work scope become concrete. Continue implementation when the user's request authorizes it. Deferred or rejected ideas remain discoverable without entering the work queue.
- Give AEP a declared relationship through which it can discover the relevant draft/research and expose selected source content, revision and omissions. Detect missing targets; keep context bounded instead of indiscriminately loading the design directory.

This extends the current roadmap/design responsibilities. It does not propose an additional skill or make every idea an ADR. The agent chooses exploration depth and checks meaning against the request; the CLI can enforce required structure and relationship integrity, but cannot certify that the reasoning is complete.

## Open implementation decisions

Decide whether drafts/research need a dedicated native record kind or a typed artifact registration linked from existing records. Define minimal metadata, maturity updates, discovery and source-loading behavior before selecting commands. Avoid overloading an accepted decision merely to obtain a searchable record.

Keep honoring `stores.designs` and existing files. A different default directory or subdirectory layout is not decided here; changing the folder name alone does not resolve discovery. Existing freeform drafts need an adoption path that preserves content and Git history, without automatically inferring acceptance.

Verification should demonstrate: saving an idea with no story; retrieving its research and unresolved state; detecting a broken declared artifact link; recording an authorized ADR with retained draft provenance; and excluding deferred ideas from implementation readiness. Report structural checks separately from semantic review and actual Git retention.
