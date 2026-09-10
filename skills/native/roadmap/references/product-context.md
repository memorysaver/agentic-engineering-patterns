# Product context and useful slices

Start with the problem, who experiences it, their current workaround, and the observable improvement sought. State the evidence for the opportunity and the assumptions still needing observation. Define an initial boundary small enough to learn whether to proceed, defer, or stop; make the reason visible in the decision rather than assigning a ceremonial score.

Map the user journey and the system components it crosses. Identify external actors, data owners, interfaces, and failure boundaries that affect acceptance. Keep domain terms consistent with existing contracts; a glossary can resolve missing premises without importing another explanation workflow.

Choose the smallest end-to-end slice that demonstrates an outcome, even if depth comes later. Describe acceptance and the public surface where it can be verified. Infrastructure slices can be legitimate prerequisites; link the feature they enable and give their own observable completion condition.

Use explicit dependency edges for required interfaces, data, or integrations. Separate readiness from business priority: a ready story may still provide less value or learning than another. Compare value, dependency unlocks, uncertainty, and real integration cost with the evidence available. Numeric layer labels are organization, not implicit barriers.

## From feedback to maintained context

After presenting a proposed direction and obtaining enough user input to settle it, complete the authorized context work in the same task. Writing and linking the affected records is part of that deliverable; per-file permission questions add no decision. Carry forward the user's accepted choices and existing authority, while keeping unresolved alternatives visibly draft. An explicit discussion, analysis or read-only request retains that boundary.

Inspect the configured stores and affected existing records before writing. Product context is the linked canonical material below; update the relevant parts rather than creating a parallel monolithic context file or every possible document.

| Content | Canonical home and responsibility |
| --- | --- |
| Purpose, users, outcomes, priorities, journeys and system relationships | Roadmap records and relevant context under the configured roadmap store (default `project-roadmap/`) |
| Choice, rationale, alternatives, source/attribution and replaced decisions | Decisions under the roadmap store's `decisions/`; accept or supersede under current authority |
| Research support, design options, interfaces, unknowns and prototype findings | Relevant research artifacts and the configured design store (default `docs/design/`), linked from their decision/change |
| Behavior, scenarios, verification contract and concrete work | Changes and stories under the configured ledger (default `project-ledger/`), developed through `aep --skill design` |
| Reusable execution lessons | Configured lessons store (default `lesson-learned/`), using reflect when useful |

Preserve accepted history. A correcting decision references affected records; a changed behavior or implementation scope needs its own contract. Read `aep --skill design --ref records` for native inputs, revisions and supersession. Link the selected outcome, decision, design and applicable change/story so a later agent can find both the rationale and what to verify. File links aid discovery; arbitrary referenced artifact bytes are not automatically bound into every verification fingerprint.

Check that related documents agree about the selected direction, superseded choices and remaining questions. Run `aep check` and, for a concrete change, `aep spec check --change <id>`; structural success still needs semantic review against the user's feedback. Retain the work using the project's Git procedure and report actual saved/committed state and canonical paths. Research conclusions and document checks are separate from executed product evidence.

Continue through design when acceptance or interfaces need resolution. If implementation is authorized, carry the selected contract through implement, validate and deliver. Otherwise finish with the requested context/design deliverable and its remaining decisions. Link the verification procedure's Feature Map to the applicable scenarios, and revisit outcome assumptions after delivery.
