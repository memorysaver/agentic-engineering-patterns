# MITS product focus and benchmark direction observation

Date: 2026-09-11, Asia/Taipei
Scope: read-only Herdr observation of MITS `w4:p2`; no observer prompt or downstream file changes
State at this checkpoint: research proposal written; product-priority clarification still pending

## Observed request and behavior

The user asked to observe MITS while testing product focus and direction changes. The visible preceding discussion showed the agent recommending MITS-072, the memory topology observatory. When asked why and what research supported it, the agent revisited the proposal and later runtime research, acknowledged insufficient evidence for priority, and revised its recommendation. Those earlier events were recovered from visible transcript, not observed from the beginning.

The downstream user then requested product refocusing before choosing a suitable benchmark, expressing a preference for LongMemEval V2. The agent asked which product outcome should lead: cross-session/agent continuation, long-term factual question answering, or cross-project learning. While that clarification remained pending, it continued relevant independent research instead of treating the unanswered question as an accepted choice.

It read native roadmap product-context guidance, design/design-criteria/prototype guidance, project-owned verify-mits, current product direction, benchmark matrix, research and execution policy. Its transcript shows searches of primary upstream sources, including the paper, repository, leaderboard/data documentation and actual adapter/harness source. It retained upstream commit `2cc8c540bdb87fe6761629b585e727e1c4704520` as a research anchor rather than an accepted run pin.

The agent explicitly revised an earlier research inference: comparing scores from different systems does not isolate the effect of iterative versus one-shot retrieval. This observation records its source-checking behavior; the observer did not independently reproduce the benchmark or validate every external research claim.

## Artifact and boundaries

The proposal is `MITS/docs/design/2026-09-11-product-focus-and-longmemeval-v2.md`. It labels product priority and benchmark cutover as unaccepted, ties a provisional goal to existing product context, and compares external benchmark coverage with direct MITS handoff outcomes. It includes comparison controls, capability gaps, evidence/cost boundaries, and a proposed staged experiment. It leaves old benchmark results/gates intact.

At this checkpoint Git shows only the new untracked design directory; main remains `7af2d5ba96ef042c005ee3139dac863bf8a01719`. No accepted roadmap/decision, adapter implementation, benchmark execution or gate replacement has been observed. Draft placement fits the unresolved intent. A later accepted direction still needs canonical roadmap/decision links and a grounded implementation contract; a design file alone does not establish that transition.

The agent ran formatting, `git diff --check`, `aep check` and `bun run check`; the visible outputs passed. Observer read-only `aep check --json` independently reported pass with 413 records. These are document/repository structure checks, not evidence of benchmark or product success. The transcript also shows a progress memory write (`20260910T175230Z792290719`) and successful sync; memory-service persistence was not independently inspected.

Herdr reports `blocked` while an asynchronous product question is queued, even during visible research activity. That classifier alone does not show execution is stalled. The observer did not answer or dismiss the question. The draft is locally present but uncommitted at this checkpoint. The visible turn ended after 4 minutes 55 seconds with the clarification still pending; this does not establish whether the agent will autonomously update canonical context after receiving the user's direction.

## AEP lesson

The useful behavior was assembling evidence around a product decision, separating observation from inference, correcting an unsupported causal claim, and distinguishing proposed intent from accepted execution. The initial recommendation needed the user's evidence challenge to trigger that deeper investigation. This is one concrete motivation for purpose-driven autonomous research, not proof that every recommendation requires a large research exercise.

The user identified this as a missing capability in 4.x and requested the new concept “auto deep research for purpose.” The resulting [AEP design proposal](../decisions/aep-v5-purpose-driven-research.md) is separate from this downstream observation. No new live behavior has yet established that the proposed guidance triggers at the right time without prompting.

Local raw transcript/state snapshots and the draft copy are retained under `~/.local/share/aep/trials/2026-09-11-mits-product-focus/`. The available installed AEP build was `aec8e775495a49ff`; this observation did not switch binaries or intervene in MITS.
