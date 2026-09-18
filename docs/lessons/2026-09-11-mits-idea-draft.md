# MITS idea capture and draft discovery

Date: 2026-09-11, Asia/Taipei
Status: completed downstream turn reconstructed from terminal output and retained files; agent idle at inspection
Previous: [product context follow-up](2026-09-11-mits-product-focus-followup.md)

The user asked to continue monitoring and identified a use case: new ideas initially written under `docs/design` should belong to AEP's research, draft and ADR structure. The observer inspected Herdr pane `w4:p2`, source files, native records and configuration without sending a prompt or changing MITS.

## Observed behavior

The downstream agent completed a reported 6m48s turn. Its accepted native decision `v2-first-longitudinal-evaluation` attributes the user's choice to test LongMemEval V2 first, then collect evaluation evidence through MITS during long-running real tasks. The decision retains the source statement, selected order, unresolved contracts and execution limits. Creation and acceptance events are retained in the evidence bundle.

The agent wrote `docs/design/2026-09-11-longitudinal-evaluation-collection.md`, explicitly labeled as a collection/evaluation design draft, and updated the earlier product-focus design. Both directions are linked from the decision's `data.design_artifacts`; the new draft links back to that decision. Acceptance applies to execution order, not to every proposed recorder interface or schema. No V2 run or collection hooks were reported as executed.

Visible commands include a native decision operation ending in `decision accept — Saved`, formatting, `aep check`, and `bun run check`. The observer independently confirmed `aep check --json` passed with 422 records and no diagnostics. Project-check success is transcript evidence; the observer did not rerun it or independently research the draft's external benchmark claims.

## Classification and completeness gap

MITS explicitly configures `stores.designs = "docs/design"`. This file therefore already occupies the configured AEP design store. Current AEP guidance assigns research/drafts there and ADRs to the roadmap store's `decisions/`; the directory itself is not a routing failure.

However, the draft is ordinary Markdown with prose status, not a native research/draft record. The current `Kind` enum has no research or draft kind. `aep context v2-first-longitudinal-evaluation --json` returns 166 linked records, no missing record references, and `sources: []`. The two artifact paths in arbitrary `data` do not automatically include their contents or verify their existence. Explicit `--source <path>` is the existing way to include file content and a digest; `context` source code confirms this behavior.

Thus the current implementation supports a storage convention and manually selected context, but does not provide a native research/draft identity and automatic discovery through the recorded relationship. A passing structural check is not a verdict on draft content completeness. This repeats the earlier artifact-discovery gap with a concrete new-idea case.

## Retention and next design

MITS remained on main at `7af2d5ba96ef042c005ee3139dac863bf8a01719`. Design files, decisions, roadmap and events remained untracked. The downstream final response disclosed the uncommitted state; Git retention is still incomplete. Herdr reported idle at the inspection checkpoint.

The proposed AEP follow-up is recorded separately in [research and draft context](../decisions/aep-v5-research-draft-context.md). No runtime fix, downstream migration, installation or publication occurred in this observation.

Evidence: `~/.local/share/aep/trials/2026-09-11-mits-idea-draft/` contains terminal output, agent state, configuration, check/context output, Git state, design/decision/event copies and a SHA-256 manifest.
