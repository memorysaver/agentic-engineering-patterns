# MITS story planning and auto research loop design

Date: 2026-09-11, Asia/Taipei
Status: live-observed downstream turn, 8 minutes 25 seconds; agent `done` at inspection
Previous: [idea capture and draft discovery](2026-09-11-mits-idea-draft.md)

The user asked the MITS agent to organize the current stories, propose an execution order, and design a continuous-evaluation auto research loop. The observer watched Herdr pane `w4:p2` from before the prompt to completion, snapshotted the transcript every minute, and compared native records, files and Git before and after. No prompt was sent to MITS and no MITS file was changed.

## Guidance and context consumption

Visible commands before any write: `aep --skill`, `aep --skill roadmap` with the `product-context` and `purpose-research` references, `aep --skill design` with the `records` and `bdd` references, `aep --skill validate`, and `--help` for `story new`, `story update`, `change new`, `wave new`, `roadmap accept`, `decision new`. The agent read `.aep/config.toml`, `project-rules/README.md`, `project-rules/v5-preview-migration.md`, the two accepted decisions, the existing design drafts, story YAML for MITS-063/068/070–073/078, `aep context MITS-109` and `MITS-107`, the runner and memorybench READMEs, and its own MITS memory (`mits orient`, `mits recall project-state`). It also searched the web for the karpathy autoresearch program and the LongMemEval-V2 leaderboard before designing the loop.

The design draft was opened and read as a file. Existing story context came from `aep status` and `aep context`, not from path presence alone.

## Native writes and classification

One Python orchestration cell created, through native commands, roadmap `evaluation-research-roadmap`, nine changes `eval-*` each with a `specs/<name>/spec.md`, and stories MITS-110 to MITS-118. Nineteen retained events show every record moving `null → pending`; a twentieth event records one `pending → pending` revision of the roadmap. No record was accepted. The changes describe themselves as draft acceptance proposals; the roadmap says planning only, story contracts remain draft, no research or spend schedule is activated.

| Artifact | State and purpose |
| --- | --- |
| `docs/design/2026-09-11-evaluation-roadmap-and-autoresearch.md` | Design draft: disposition of existing stories, story table with observable outcomes and hard dependencies, auto research loop, spend/authority gap, user participation, delivery boundary. Links to both accepted decisions, both earlier drafts and the runner README; all local links resolve |
| `project-roadmap/plans/evaluation-research-roadmap.md` | Pending roadmap: recommended order 110→111→112→113→114→116→117→118, MITS-115 on a data-driven longitudinal track, WIP 1 and independent review policy |
| `project-ledger/stories/MITS-110..118.yaml` | Pending stories, each bound to one draft change; `depends_on` chain 110→111→112→113→114→115 and 112→116→117→118 matches the roadmap; `data` carries draft contract status, design artifact path, evidence boundary, recommended order |
| `project-ledger/changes/eval-*/` | Nine pending changes with requirement/scenario specs |

Draft, research, accepted direction and executed results stay distinguishable: the accepted records are still only the two prior decisions and the one prior roadmap; every new record is pending and says so in prose.

## Verification

The agent ran `aep spec check` for all nine changes, `aep check`, `aep context MITS-110`, `aep status`, `bun run check` and `git diff --check`, then wrote a MITS progress memory and synced. The observer independently reran `aep check --json` (pass, 461 records, no diagnostics) and all nine `aep spec check` commands (pass). `bun run check` output in the transcript reported the formatting gate passed with 86 retained pre-migration debts; the observer did not rerun it.

`aep status` reports 0 stories ready to start. MITS-110 has no dependencies but is not ready because its change `eval-v2-evaluation-contract` is not accepted. This matches the story's own `contract_status: draft; complete design and acceptance before dispatch`. The final response told the user to start designing MITS-110 and that stories are unimplemented; it did not state that acceptance of the change is the gate before dispatch.

MITS-110 lists `benchmarks/longmemeval_v2/contracts` in `paths`; that directory does not exist yet. It is a planned location inside a draft, not a broken reference, but it will look like a missing path to a reader who treats `paths` as existing files.

## Context loading

`aep context MITS-110 --json` returns 185 linked records, no missing references, and `sources: []`. The design artifact path appears in `data`; its content is not included. The same gap recorded in the previous two observations recurs, now on a story whose actionable contract lives mainly in the design draft and in the change spec.

## Handoff and retained state

The final response summarized the order, the loop, the fixed rules, the spend limit gap in the current runner permit contract, and said explicitly: saved, uncommitted, stories unimplemented, loop not enabled. MITS remains on `main` at `7af2d5ba96ef042c005ee3139dac863bf8a01719`; the earlier design drafts, decisions, roadmap and events are still untracked, and this turn added the new files to that untracked set. Git retention has now been deferred across three consecutive turns.

Scope question for the user: the request was to organize and order stories; the agent created nine new stories and nine changes as pending drafts. Native state preserves the draft status, so nothing was accepted on the user's behalf, but the volume exceeds a reorder of existing stories.

Evidence: `~/.local/share/aep/trials/2026-09-11-mits-story-planning/`, including the user prompt, baseline status/query/events/Git before the turn, minute-level transcript snapshots and the final transcript, copies of the new design, roadmap, stories, changes and events, check output, agent state and a SHA-256 manifest.
