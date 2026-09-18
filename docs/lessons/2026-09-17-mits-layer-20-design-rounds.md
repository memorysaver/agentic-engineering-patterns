# MITS Layer 20 design rounds under container guidance

Date: 2026-09-14 to 2026-09-17, Asia/Taipei
Status: four completed downstream turns reconstructed from terminal output, retained files and Git; one delivery turn observed live on 2026-09-17 (outcome recorded at the end)
Previous: [layer/wave adoption](2026-09-11-mits-layer-wave-adoption.md)
Guidance: [layer and wave encapsulation](../decisions/aep-v5-layer-wave-encapsulation.md)

After Layer 20 was created, the user drove MITS-110 through four design rounds in the MITS pane while the observer only read terminal output, files, native records and Git. The observer sent no prompt and changed no MITS file. The user twice relayed the observer's review points into the MITS conversation; the agent's replies to those points are recorded here as downstream behavior, including one correction of the observer.

## Rounds and retained state

| Commit | Turn | Recorded outcome |
| --- | --- | --- |
| `b31c27b` 2026-09-14 | 12m58s | MITS-110 concrete proposal: design draft, machine-readable run contract with `executable: false`, source audit with upstream file digests and 42 local test results, BDD spec; story and change `data` point at all three artifacts |
| `30642db` 2026-09-14 | after a user choice | Decision `v2-three-arm-agent-comparison` accepted with the user's statement; story and change `refs` updated |
| `ba82156` 2026-09-15 | 6m35s | Decision `v2-local-subscription-luna-max` accepted; the earlier decision marked `superseded` with `superseded_by`; layer-20, MITS-110, change and spec updated; Qwen/A100/cost sections removed from the draft and noted as retained in Git history |
| `c874f2c` 2026-09-16 | 4m46s | Decision `task-trajectory-evidence-design` accepted after the user's "我同意你的設計"; design document written; layer-20, MITS-110/113/114 and three changes linked; three specs extended |
| `5c17686` 2026-09-16 | 21m14s | Decision `codex-trajectory-v1-adapter-design` accepted as agent technical design; change `eval-v2-evaluation-contract` moved `pending → accepted`; Codex capability audit, trajectory v1 schema, synthetic example, verifier and role prompts retained |

All four commits were made by the agent; none was pushed. At the start of the 2026-09-17 turn `main` was three commits ahead of `origin/main` with a clean tree, and the agent reported the unpushed state in every summary. `aep check --json` passed at each observer inspection (483, 488, 495, 505 and 518 records). `aep spec check` passed for every affected change. The observer reran the trajectory verifier and obtained the same result as the transcript: 6 synthetic events accepted, 21 invalid cases rejected, no model turn.

## Container behavior confirmed

Every new decision lists `layer-20` in `refs` together with the affected member stories and changes, and the layer, stories and changes gained the reverse link in the same turn. Supersession used the native lifecycle: the replaced decision keeps its content and points at its successor. Readiness stayed coherent throughout: MITS-110 was blocked only by its unaccepted change until that change was accepted on 2026-09-16, after which `aep status --json` reported MITS-110 as the only ready story and MITS-111 blocked on MITS-110 delivery and its own contract. This is the structural evidence the 2026-09-11 guidance change aimed for: one concept's decisions, design artifacts, contract and members remain reachable from the container.

## Boundaries and self-correction

The agent kept the requested boundaries: no V2 run, no dataset or model download, no capture activation, no import of existing user sessions. Its Codex protocol probe ran under `bwrap` with home, root, run and tmp masked and the network unshared, produced zero model turns, and pinned the actual binary digest rather than the `mise` wrapper. The observer searched the retained evidence directory for other projects' paths and session identifiers and found none.

Three corrections are notable. The agent told the user that the observer's "USD 10 ceiling blocks the pilot" reading was too broad because the existing guard only forecasts Modal CPU and memory cost; the observer confirmed this in `benchmarks/runner/providers/modal/cost.py`. In the following turn it stated that its own earlier wording had presented the A100 candidate as if it were settled. When the user wrote `~/.MITS`, the agent verified that the existing root is lowercase `~/.mits`, kept it, and listed the discrepancy as unresolved rather than creating a second vault. It also asked one clarifying question with three options before recording the Luna role selection.

## Behaviors worth changing

These are structural observations on the records, not model-specific reactions.

**Concept-level status is copied into every member.** After four rounds, `layer-20.data` has seven keys, `MITS-110.data` sixteen and the change's `data` seventeen. The same `design_status` paragraph is stored verbatim in the story and the change; `trajectory_design_status` is copied into six records. Each round added a new `<topic>_decision` and `<topic>_design_artifact` pair to the layer, the story and the change. The links are correct, but the status prose now has several canonical homes and each new decision requires touching every member. The guidance says a layer's `data` names the design artifact; it does not say that concept-level status and decision lists belong to the container and decision records while members carry links only.

**A contract was accepted under inferred authority.** The agent accepted `eval-v2-evaluation-contract` after its own independent review, attributing authority to the user's "好我們照著流程往下" and to the plan it had announced in the previous reply. The attribution fields exist and were filled, and the commit is traceable, so nothing is hidden. But this acceptance was the transition that made MITS-110 ready for dispatch, and the user had not named the contract. The four preceding decisions each cite a specific user statement; this one cites a general go-ahead. Guidance currently says "accept the change under the task's authority" without distinguishing a readiness-changing acceptance from a contextual decision.

**Update events carry no note.** All 33 events recorded between 2026-09-14 and 2026-09-16 have the title "Records updated" and an empty description. Their revisions and the commit messages are the only way to learn what changed. The `update` commands accept no note, so the agent had nowhere to put one.

**Context expansion remains wide.** `aep context layer-20 --json` produced about 12,800 lines in the agent's terminal, and design artifact contents are still reached only by reading the paths in `data`. This repeats the finding in [research and draft context](../decisions/aep-v5-research-draft-context.md).

**Design-only story enters the implementation flow.** On 2026-09-17 the agent began MITS-110 delivery. `aep deliver plan --story MITS-110` failed with "Expected one active attempt for MITS-110; found 0", so the agent ran `aep dispatch start`, which created a worktree under `.aep/worktrees/`, recorded the attempt as running and requested a review, all for a story whose deliverable is design documents already on `main`. The flow is consistent with the runtime, and the outcome is recorded below.

## Delivery turn outcome

The 2026-09-17 turn started from the user's "好繼續工作" and ran 19 minutes 56 seconds. Within it the agent delivered MITS-110 and moved on to MITS-111 without a stop, then ended by asking the user to choose the reader/judge tool boundary (allow pure computation with proof of no external access, or keep strict zero tools) before changing any contract. At the end `main` was at `09316b5`, seven commits ahead of `origin/main`, with two checkpoint commits retaining MITS-111 attempt and check records; the attempt branch was at `f573cbd` with an independent review checkpoint; both trees clean; MITS-111 `in_progress`:

| Step | Observer-verified state |
| --- | --- |
| Attempt for MITS-110 | `dispatch start` created a worktree from `5c17686`; one commit `5f8a5fa` added a twenty-line acceptance map from the spec to the retained design evidence |
| Checks and review | Three check evidence records and one review record; `deliver merge --local` produced receipt `delivery-18d60745f7ef7828-1ca582-0` with head and integration head `5f8a5fa`, `tree_verified: true`, producer binary `3fae0e42…`, CLI 5.0.0-preview.1 |
| Closure | `spec publish` wrote `evidence.yaml` and a 140-line published spec; `change close` moved `eval-v2-evaluation-contract` to `closed`; MITS-110 is `integrated`; commit `ef56963` (24 files) on `main`; `aep check` 544 records, no diagnostics |
| Next contract | The agent accepted `eval-v2-mits-adapter` in the same turn with `accepted_by: codex-user-authorized-implementation`, then started attempt `attempt-18d6074cd7a7b763-1cc19d-0` for MITS-111 in a second worktree; MITS-111 is `in_progress` and `main` is five commits ahead of `origin/main` |
| MITS-111 work so far | Codex App Server tool-inventory probes in a `bwrap` fixture against a loopback fake Responses endpoint that returns HTTP 400, so no external model request and no subscription use; eight retained receipts, two unit test modules, committed to the attempt branch as `7fddf8e`; `aep verify run` reported local-preflight, repo-check and secret-scan passing; the adapter README states that the accepted zero-tool reader/judge requirement is not met by the tested configuration because `exec` and `wait` remain advertised, and that this does not prove no configuration can meet it |

Two points for the guidance discussion. First, a story whose deliverable was design documents already on `main` went through attempt, review, local merge, publication and closure; the receipt and closed contract now give MITS-110 the same completion evidence as an implementation story, which is what readiness for MITS-111 required. Second, the adapter contract was accepted under the same general go-ahead as the evaluation contract, so the readiness-changing acceptance pattern recurred immediately after it was first observed.

## Follow-up: `--note` accepted, implemented and installed

The user accepted the event-note candidate the same day. Commit `8e8a79c` adds a global `--note <TEXT>` whose text becomes the description of the event a write produces, prints notes in `aep timeline`, and adds one sentence each to the records guidance about notes and about keeping concept-level status in the layer and decision records. Checks: fmt, clippy, workspace tests (lifecycle 24 including the new note test), steering ceilings, generated-skill sync and the preview proof. Build `11186c26c439cec0` was verified read-only against MITS (548 records), looplia (981) and Rewarc-AutoResearch (426) with `check`, `doctor` and `timeline` before the shared `~/.local/bin/aep` link was switched from build `71a66621…`; the manifest and proofs are under `~/.local/share/aep/trials/2026-09-17-note-install/`. Downstream stores needed no migration: version and schema are unchanged, and the new guidance is embedded in the binary. A one-paragraph notice describing the option and asking each agent to read the records reference was prepared for the three downstream agents; Herdr refused delivery to all three because each was waiting at a question dialog for the user, so the notice text is retained in the trial bundle for delivery once they are free. Their next writes will show whether notes appear.

Evidence: `~/.local/share/aep/trials/2026-09-17-mits-layer-20-design-rounds/` contains the commit list, Git status, the four decision files, the layer/story/change records, check and status output, the verifier result, the 2026-09-17 terminal capture and a SHA-256 manifest.
