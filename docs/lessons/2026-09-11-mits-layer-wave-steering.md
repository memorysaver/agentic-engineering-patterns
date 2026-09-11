# MITS layer and wave grouping needed user steering

Date: 2026-09-11, Asia/Taipei
Status: live-observed downstream turn, 2 minutes 37 seconds; agent `done`; no records written
Previous: [story planning](2026-09-11-mits-story-planning.md)

After the story-planning turn, the user asked the MITS agent whether the nine new stories were wrapped in a wave or layer. The observer watched pane `w4:p2` without sending any prompt or changing MITS.

## Observed behavior

The agent inspected MITS-110 to MITS-118 through native `show` calls and confirmed that `layer` and `wave` were null on every story, while all nine shared `refs` to `evaluation-research-roadmap` and `v2-first-longitudinal-evaluation`. Its context was compacted during the turn.

It answered that no formal grouping existed and proposed one layer, "build the evaluation and research loop for cross-project learning", with three waves: V2 baseline (110–112), real longitudinal evaluation data (113–115), and continuous evaluation with auto research (116–118). It stated a layer completion criterion: MITS can decide from reproducible experiments whether a memory change improves later work. It said this remained a suggestion and wrote no layer or wave record.

Git status and the event directory were unchanged before and after the turn.

## Why this needed steering

The previous turn's planning had already expressed the grouping idea without the native containers: a roadmap `data.recommended_execution_order`, per-story `data.recommended_order`, a `depends_on` chain, and a shared `data.design_artifact`. The nine stories, nine changes and one design draft therefore share a concept but are related only through a roadmap record, ad hoc data fields and ordering. None of the native mechanisms that read containers, such as inherited container dependencies and gates in readiness, gate scope over container members, or `status`/`context` traversal by container, can see this group.

Native guidance currently mentions containers in one sentence in the roadmap skill and in the records reference's note that container gates govern dispatch. It does not say when a multi-story breakdown from one design should be wrapped, what a container should carry, or how to link the container to its design and decision. The agent produced the grouping immediately when asked, so the concept was available; the trigger and the record shape were not in the guidance it read.

The proposed AEP follow-up is recorded in [layer and wave encapsulation](../decisions/aep-v5-layer-wave-encapsulation.md).

Evidence: `~/.local/share/aep/trials/2026-09-11-mits-layer-wave/`, including the user prompt, baseline Git status and event list, transcript snapshots and the final transcript.
