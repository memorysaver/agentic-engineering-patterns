# MITS user-feedback-to-context continuation

Date: 2026-09-11, Asia/Taipei
Status: observed turn complete, 3 minutes 48 seconds
Previous observation: [product focus research](2026-09-11-mits-product-focus-observation.md)
Guidance: [purpose-research implementation](../audits/2026-09-11-aep-v5-purpose-research.md)

## User direction and guidance consumption

The observer was asked to continue monitoring, then specifically to establish whether the MITS agent used AEP to record the direction and ensure format/content completeness. The observer sent no prompt, answered no downstream question, and changed no MITS file.

The downstream user chose “跨專案學習：把過去的教訓與偏好用到新工作”. This differs from the agent's prior recommendation of same-project handoff as the primary outcome. The agent adopted the user's selection, revisited cross-project research and existing specifications, and revised the design proposal accordingly.

The visible transcript confirms calls to `aep --skill roadmap`, `aep --skill roadmap --ref product-context`, `aep --skill roadmap --ref purpose-research`, and `aep --skill design --ref records`. The installed executable was build `9f32299fdc45ca53`; no executable switch occurred during this observation. This is a live continuation with the new guidance, not merely a format-only fixture.

## Actual AEP writes and classification

The agent used a Python orchestration cell invoking native record commands. Terminal output showed `decision accept — Saved` and saved roadmap paths. The retained native event chain independently confirms creation and acceptance for both records, including before/after revisions:

| Artifact | Native state and purpose |
| --- | --- |
| `project-roadmap/decisions/product-focus-cross-project-learning.md` | Accepted decision: exact user statement, source of authority, accepted scope, considered alternatives, boundaries and unresolved choices |
| `project-roadmap/plans/cross-project-learning-focus.md` | Accepted roadmap: primary user and selected outcome; design pointer; LongMemEval V2 explicitly tentative |
| `docs/design/2026-09-11-product-focus-and-longmemeval-v2.md` | Revised prose design: chosen product priority, proposed first journey, eight positive/negative cases, existing research, benchmark mapping and limits, comparison/cost design, unresolved next decisions |
| `project-ledger/events/event-18d42832*.yaml` | Four native events: each new record moves from absent to pending, then pending to accepted |

The accepted scope is product priority only. The records explicitly exclude story activation/implementation acceptance, benchmark execution, baseline changes and spending decisions. Detailed first scenario, metrics/thresholds and benchmark contract remain under design. No new implementation change/story was created for unresolved scope; existing MITS-068/MITS-072 were linked as context and retained their prior states.

The design was edited as ordinary Markdown. AEP serves its authoring guidance and maintains linked native records; it has no separate command that semantically authors or approves the prose file. Both records use normal frontmatter and prose, with additional intent in structured `data`; an empty `description` alone does not mean their full file content is empty.

## Verification and completeness boundary

The agent ran formatting, `git diff --check`, `aep check` and `bun run check`. Visible outputs passed. Parent read-only checks confirmed `aep check --json` passed with 419 records and no diagnostics; all nine local Markdown links across the new records/design resolve.

Semantic inspection confirmed that the chosen priority is cross-project learning, same-project handoff is supporting capability, and a tentative benchmark was not promoted into an accepted execution contract. The draft includes applicability, negative transfer, source attribution, uncertain preferences and current-user-instruction boundaries. These are proposed scenarios, not executed product evidence. External benchmark claims were not independently re-researched by the observer in this turn.

`aep context cross-project-learning-focus --json` has no missing record references, but expands to 165 linked records with `sources: []`. The design is named in `data.design_artifact`; its full prose is not automatically included. Thus graph validity and context traversal do not establish that a subsequent reader received every relevant design claim. The observer opened the actual files separately. This also exposes a context-selection cost: a focused direction query pulls substantial historical work through existing links. Record this for a later context retrieval/completeness design, without claiming the currently documented explicit-source behavior is an automatic artifact loader.

## Handoff and retained state

The agent wrote MITS progress memory `20260911T035943Z641121167`, inspected it, then wrote a user-origin decision memory `20260911T035952Z004988765` and synchronized successfully according to the visible commands. The observer did not independently inspect the memory service.

The final response reported the updated direction, asked for a concrete cross-project example to resolve the first journey, and explicitly said the files were saved but uncommitted. Herdr reported `done`. Main remained `7af2d5ba96ef042c005ee3139dac863bf8a01719`; the new design, two canonical records and four events were untracked. Native persistence occurred, but Git retention/synchronization remains incomplete at this checkpoint. No observer commit or repair was made in MITS.

The user-feedback-to-canonical-context behavior was observed: the agent acted on feedback without another request to write files. This does not prove all research triggers, full design acceptance or subsequent implementation are complete. Follow-up concerns are explicit artifact discovery/completeness, historical context expansion, and the uncommitted handoff.

Evidence: `~/.local/share/aep/trials/2026-09-11-mits-product-focus-followup/`, including transcript, native records, event timeline, context response, source-file copies, checks, final Git/agent state and SHA-256 manifest.
