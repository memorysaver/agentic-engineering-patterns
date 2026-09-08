# Project ledger, roadmap, specifications, and memory

**AEP 5.0 entrypoint:** [Rust CLI, project context, and verification](aep-v5-rust-cli-architecture.md) defines the release target, native command/check system, and OpenSpec integration. It takes precedence where this supporting review describes an earlier implementation boundary.

AEP should preserve a project's direction, work history, accepted behavior, decisions, and lessons without forcing every maintenance task into a user story map.

**Status:** Proposed; research and migration design only. **Research dates:** 2026-09-08–09. **AEP source baseline:** v4.1.0, `acf03fc41fe25faf67905c9004451702b7986dd8`. **Audience:** AEP maintainers and downstream project owners.

This extends [Project rules and a simpler story workflow](project-rules-and-story-workflow-refactor.md). It supersedes that proposal's use of OpenSpec as a required canonical store and `lessons-learned/` as the new-layout spelling. Existing stores and skills remain unchanged until implementation and consumer migration. The [Astra review](astra6-v4.1-design-guidance.md) still governs measurement and evidence limits.

## 1. What the existing runs tell us

The review inspected two existing private downstream repositories: their committed operational YAML, split product definitions/maps, OpenSpec changes/specs/archive, gate evidence, and lesson/memory rules. Detailed paths, snapshots, counts, and counterexamples are retained in the local research record. This public proposal uses generalized findings and synthetic examples; it does not reproduce private product content.

- The operational files have grown into substantial work histories. They contain maintenance and infrastructure work, dependencies, containers, outcomes, costs, and project-specific governance. A monolithic file requires unrelated collaborators to edit the same surface.
- Product maps still contain useful intent and some explicit work references. Later entries have fewer literal map links in the inspected array-order samples. Range references and implicit semantic links also exist. This supports separating the roles; it does not prove the backbone is useless or that array order is execution chronology.
- Cross-layer dependencies and gates remain meaningful. Some legacy stories have no explicit wave membership. Numeric layer labels are not a reliable clock, and a gate status is not necessarily proof of a deployed outcome.
- A recorded handoff failure shows detailed story criteria existed while the change artifacts expected by launch did not. The design must expose one executable change contract with pointers, rather than require parallel descriptions to be maintained independently.
- Some consumers already maintain separate operational-truth and memory surfaces. Those rules can constrain dispatch beyond a story's `completed` label. A generic migration must preserve that authority and its evidence.

There is also a direct AEP source mismatch: [buildStoryMap](../../packages/api/src/lib/product-context-loader.ts) builds its `backbone` from architecture modules, while product activities are a separate collection. [Autopilot](../../skills/patterns/autopilot/references/tick-protocol.md) selects the first incomplete layer and assumes prior-layer gate ordering. These are implementation choices, not inherent properties of a product journey or a collaboration theme.

**Design conclusion:** separate the product map, technical architecture, delivery organization, and historical record. Link them where the relationship is known. Do not fabricate a persona or activity to satisfy an execution schema.

## 2. OpenSpec: preserve the semantics, make the tool optional

Reviewed official OpenSpec v1.12.0 source at `e062b9572be933564ba3899d059377dfa1393e32`, with the same locally installed CLI version. This was source inspection; no sync/archive operation was executed.

| Question | Finding and implication |
| --- | --- |
| What does a change preserve? | Proposal, design, tasks, and delta specifications form a coherent change bundle. Current specifications and proposed changes are separate. Keep that distinction. [Concepts](https://github.com/Fission-AI/OpenSpec/blob/e062b9572be933564ba3899d059377dfa1393e32/docs/concepts.md) |
| Does a scenario prove behavior? | The base schema requires scenario text; it does not establish an executable test, successful run, or implementation commit. Preserve BDD scenarios and add explicit evidence links. [Schema](https://github.com/Fission-AI/OpenSpec/blob/e062b9572be933564ba3899d059377dfa1393e32/src/core/schemas/base.schema.ts) |
| Does completion mean implemented? | `isComplete` aliases planning completion. Task progress counts checkboxes. Import these as distinct historical claims. [Agent contract](https://github.com/Fission-AI/OpenSpec/blob/e062b9572be933564ba3899d059377dfa1393e32/docs/agent-contract.md), [task progress](https://github.com/Fission-AI/OpenSpec/blob/e062b9572be933564ba3899d059377dfa1393e32/src/utils/task-progress.ts) |
| Does archive prove delivery? | Archive can proceed with incomplete tasks under `--yes` and can skip spec updates. Archived does not imply implemented, merged, or released. [Archive source](https://github.com/Fission-AI/OpenSpec/blob/e062b9572be933564ba3899d059377dfa1393e32/src/core/archive.ts) |
| Can custom schemas implement the requested directory split? | They customize artifacts and dependencies, but stock root resolution still uses `openspec/changes`, `openspec/specs`, and its archive directory. Renaming the root tree is insufficient; the audited planning-home implementation retains that layout too. [Customization](https://github.com/Fission-AI/OpenSpec/blob/e062b9572be933564ba3899d059377dfa1393e32/docs/customization.md), [root resolution](https://github.com/Fission-AI/OpenSpec/blob/e062b9572be933564ba3899d059377dfa1393e32/src/core/root-selection.ts) |

Sync merges proposed deltas into current specs while the change can remain active. Archive and sync therefore need separate import facts. The generated sync workflow uses agent judgment, while CLI archive has its own merge implementation; a compatibility adapter must specify which behavior it implements. [Sync workflow](https://github.com/Fission-AI/OpenSpec/blob/e062b9572be933564ba3899d059377dfa1393e32/src/core/templates/workflows/sync-specs.ts).

| Option | Tradeoff | Decision |
| --- | --- | --- |
| Keep OpenSpec mandatory with a smaller custom schema | Least tool change; retains its physical store and integration dependencies | Valid transitional mode |
| AEP owns the context contract; OpenSpec is an optional adapter | Fits the proposed stores; AEP must provide reference validation, readiness, promotion, and conflict handling | Recommended target |
| Remove OpenSpec immediately | Drops existing parser, readiness, sync/archive behavior and command integrations before replacements are proven | Defer removal until replacement fixtures and migration pass |

This is a change of responsibility, not a claim that OpenSpec reduces model quality. AEP keeps change identity, BDD acceptance, decisions, and history. It stops requiring one particular CLI to represent them.

## 3. Sources of truth by question

Code, executable configuration, and observed runs define the implemented system at a stated revision/environment. They cannot recover the human intent or rejected alternatives that were never written down. An accepted requirement can also expose a bug in the current code. Preserve that discrepancy rather than editing the requirement to certify the bug.

| Question | Canonical source | Evidence boundary |
| --- | --- | --- |
| What does this revision actually do? | Code/configuration plus tests and runtime evidence | A passing local test does not establish production behavior |
| What should the product enable, and for whom? | `project-roadmap/` backbone, journeys, goals | Desired behavior can be unimplemented |
| What behavior did we accept? | Versioned contracts in `project-roadmap/specs/` | Acceptance and verified implementation are separate facts |
| Why was this tradeoff chosen? | `project-roadmap/decisions/` | Decision status and supersession must be explicit |
| What work happened, by whom, in which containers? | `project-ledger/` | Historical assertions retain their source and uncertainty |
| What is currently being explored? | `docs/` design drafts and research | A draft is not a binding decision |
| What went wrong or transferred to later work? | `lesson-learned/` | An observation is not automatically a rule |
| What maintenance rules currently apply? | `project-rules/` and executable policy | Recalled memories cannot override these rules |

A task context packet joins these sources. It is a derived view with citations and revisions, not another authoritative `project-context.yaml` or a mandatory vector database.

## 4. Proposed file structure

Create only the records a project needs. Retain existing application and package directories.

```text
AGENTS.md
project-rules/
  README.md                      # task/path -> applicable maintenance rules
  context.md                     # this project's source/authority routing
  aep.md                         # installed version and workflow configuration
project-ledger/
  index.yaml                     # schema version and store locations
  stories/<story-id>.yaml         # one work item; retains existing ID
  layers/<layer-id>.yaml          # theme, owner, waves, explicit prerequisites
  releases/<release-id>.yaml      # frozen scope and gate references
  gates/<gate-id>.yaml            # requirements, verdict, evidence scope
  changes/<change-id>/
    change.yaml                  # story/design/decision/spec/evidence links
    specs/<capability>/spec.md    # proposed BDD/delta behavior
    tasks.md                     # when useful; preserves imported checklists
  events/<record-id>/<event-id>.yaml
  evidence/<change-id>/           # small durable receipts and manifests
  imports/<migration-id>/         # source map, digests, exceptions, prior snapshot
project-roadmap/
  README.md                      # purpose and task-relevant navigation
  backbone.yaml                  # product activities, not architecture modules
  journeys/<journey-id>.md
  architecture/                  # explanatory model, points to code
  decisions/<decision-id>.md      # ADRs and other accepted product tradeoffs
  specs/<capability>/spec.md      # current published behavior contract
  specs/<capability>/evidence.yaml
  models/                        # existing object models/calibration when needed
  state/                         # project-owned operational claims, if present
  views/                         # optional generated map/progress views
  archive/                       # prior product plans retained with provenance
  index.yaml                     # stable IDs and paths, not copied contents
  # Keep indexes small; omit machine index until a consumer requires it.
docs/
  design/<change-id>.md           # design draft and its retained revision history
  research/
  audits/                        # migration and validation reports
lesson-learned/
  observations/<lesson-id>.md
  retrospectives/<container-id>.md
  proposals/<proposal-id>.md      # proposed rule/skill edits and evaluation
```

The new preferred spelling is the user's `lesson-learned/`. Existing `lessons-learned/` is a legacy input, not a second writable store. Move it only after its readers/writers have migrated. Large logs, screenshots, and sensitive traces can stay in a project-controlled artifact store; keep their location, digest, retention/access status, and meaningful evidence locally. Do not make an unavailable artifact look verified.

`docs/` remains a general documentation area, with design drafts as one category. This does not force setup guides, public documentation, or research into the ledger. Accepted design reasoning is linked from an ADR/contract; the original draft remains auditable rather than being copied into several active files.

## 5. Ledger model and collaboration

### Stories, layers, waves, and releases

Retain existing story IDs and permit work kinds such as feature, defect, maintenance, research, and migration. `roadmap_refs` may be empty with an honest reason. Link product-facing work to a journey when useful; technical dependencies point to actual work or contract IDs.

A layer is a named delivery theme with an owner and explicit prerequisites. Different collaborators can own different layers concurrently. A wave is a scheduling group scoped to a layer. A release selects exact story/contract revisions from one or more layers and owns its release acceptance. Layers may contribute to multiple releases; freeze release membership when evaluating a gate so later additions cannot silently change what passed.

Preserve old numeric/fractional labels as aliases. Use stable opaque IDs for new records. Numbering conveys display order only; dependency/gate edges determine readiness. Legacy wave barriers remain explicit during migration. Changing their scheduling semantics is a later policy change, even if dependency-driven refill is the target.

Example only; this is not an installed schema:

```yaml
schema: aep-ledger/v1
id: APP-042
kind: maintenance
title: Repair retry behavior
layer_ref: layer-reliability
wave_ref: wave-2
roadmap_refs: []
roadmap_note: Internal reliability work; no new user journey.
depends_on: [APP-039]
change_refs: [retry-repair]
owner: team-runtime
status: in_review
revision: 3
created_at: null                 # unknown legacy time stays unknown
legacy:
  layer: '4.5'
  source_path: product-context.yaml
```

For new work, the story stores the objective, scheduling fields, and acceptance references; the change/BDD contract owns detailed acceptance text. Do not require the same design prose in both places. Imported inline criteria remain preserved with provenance, and migration chooses their canonical destination explicitly.

The change ID is distinct from the story ID. One change can implement several grouped stories; one story can require multiple changes or attempts. This avoids breaking existing grouped-change relationships. A layer file owns its wave definitions; the story owns its current membership. Reverse story lists and progress summaries are derived, not hand-maintained duplicates.

### Durable state without a second workflow engine

Use per-story current records for operational state and append-only event files for material transitions. Update a record and its event in the same reviewed commit. The current record is authoritative for dispatch; events provide audit history. Initial implementation does not require replaying every event to run the project.

Record `occurred_at` when known, `recorded_at`, actor/source, referenced revision, and a unique event ID. A source may only establish the archive date or first observed date. Preserve those event types. Do not infer merge time from an archive name or assign fabricated timestamps to achieve a total order. Timeline views distinguish known order, dependency order, and unknown/concurrent order.

The coordinator owns shared container/gate state and integration. A worker owns its attempt/worktree and proposes a result receipt. Use an expected record revision and unique attempt/event IDs to reject stale updates and duplicate dispatch. Separate files reduce textual conflicts; they do not eliminate semantic races. Cross-record reference/cycle checks and serialized integration remain necessary.

A legacy `completed` label is preserved as a legacy assertion. New completion transitions require their corresponding evidence. Implementation, integration, and release observations should be fields/receipts with explicit scope, not four competing copies of story status. A reopened issue appends a transition and retains the earlier outcome.

### Gates

Several evaluations can belong to the same legacy layer. Give each imported gate occurrence a distinct provenance-based identity; do not deduplicate by layer number. A gate has a stable ID, subject (layer/release/operational claim), criteria revision, verdict, applicable story/contract revisions, environment, evidence links, and any required human decision. Missing evidence is unknown or blocked, not pass. Preserve narrower outcomes such as local validation, experiment completion, scripted-only pass, and not-deployed. A passed gate is invalidated or re-evaluated when its relevant scope changes; migration itself never upgrades the verdict.

Dispatch consults explicit dependency and gate edges plus project operational policy. It must not assume every `layer N` waits for numeric `N-1`. Preserve that old relation as an explicit edge where the current workflow actually requires it, then review any proposed relaxation separately.

## 6. Change, BDD, and promotion to a formal contract

A change manifest is the join point for story IDs, design draft, proposed spec deltas, decisions, tests/probes, implementation commits, integration receipt, release receipt, and lessons. Each relationship points to a stable ID/revision or repository path plus commit. A draft's location never determines whether its decision was accepted.

Keep BDD as plain Given/When/Then scenarios, with stable scenario IDs. Retain existing OpenSpec Markdown where present. Projects with executable Gherkin can link existing `.feature` tests; do not require a new test runner or duplicate every scenario into both formats.

Separate two facts on a contract:

- **Intent:** proposed, accepted, superseded, or withdrawn, with a decision reference when known.
- **Implementation evidence:** unknown, partial, verified at a specific revision/environment, or contradicted by later evidence.

Those are descriptions of different questions. They need not become two large workflow engines. A generated current-spec view can show accepted-but-unimplemented behavior and the last verified revision together. Imported current OpenSpec files retain their published-baseline role and unknown acceptance provenance where absent; they do not acquire a newly invented human approval.

Promotion procedure:

1. Resolve the contract/scenario ID and baseline revision; review the proposed delta and any human tradeoff. A concurrent contract edit requires reconciliation, not last-writer-wins replacement.
2. Record the actual implementation and scenario-level evidence. A checkbox or scenario description is insufficient. For a manual check, record method, result, scope, and actor instead of pretending it was automated.
3. Verify the integration commit contains the intended implementation. Publish the contract revision and its evidence through the integration owner. Accepted intent may exist earlier; `verified` requires matching evidence.
4. Record release/deployment separately. A merge receipt cannot certify an environment it did not exercise.
5. On rollback, changed assumptions, or later failure, append contrary/superseding evidence. Do not erase earlier results or automatically rewrite the contract to match a defect.

Import existing main specs as the baseline snapshot. Preserve all active and archived deltas, including already-synced active changes and archived changes that skipped sync. Do not replay every historical delta to reconstruct current truth or batch-run archive as a migration step.

## 7. Memory and self-learning

The selected `~/idea` research distinguishes project execution authority from long-term recall, and raw diagnostic evidence from distilled reusable guidance. This proposal adopts those distinctions; it does not claim that the older local research snapshots establish current downstream behavior.

Primary-source checks support a restrained approach:

| Source | Relevant mechanism | AEP use and limit |
| --- | --- | --- |
| [LangMem concepts](https://langchain-ai.github.io/langmem/concepts/conceptual_guide/) | Semantic, episodic, and procedural memory have different roles | Separate contracts/decisions, execution observations, and active rules; no LangMem dependency is required |
| [SkillOpt v2](https://arxiv.org/abs/2605.23904v2) | Bounded skill text edits selected through held-out validation | Evaluate rule/skill candidates against separate cases; do not adopt every reflection |
| [SkillRL](https://arxiv.org/abs/2602.08234) | Experience distillation and hierarchical skills coupled with policy training | Borrow scoped retrieval/distillation; AEP is not implementing its reinforcement-learning method |
| [Meta-Harness v1](https://arxiv.org/html/2603.28052v1) | Optimizer access to code, scores, and execution traces | Preserve access to diagnostic evidence while keeping routine recall compact |

These studies do not validate this file layout, Astra performance in these projects, or autonomous changes to authorization policy. No benchmark was reproduced for this proposal.

### Capture and recall

Capture at meaningful boundaries: design decision, failed acceptance, review correction, integration, release/gate outcome, and post-release incident. Store the concise observation once, with evidence links and applicable/excluded scope. Keep an uncertain cause explicitly uncertain. An ADR records a chosen tradeoff; a lesson records what execution taught us. Cross-link them without converting one into the other.

At task start, read the small root/rule indexes, target story/change, applicable roadmap/contracts/ADRs, dependency/gate receipts, and relevant lessons. Load additional raw evidence only to resolve a question. A context packet should identify its repository revision, sources, unknowns, superseded records, and any stale/unavailable dependency. Rebuild caches/indexes from canonical files; optional memory tools consume cited boundaries and return advisory recall. They cannot certify a gate, merge, or authorization.

Prior `project-memory` or other memory-plugin stores need an inventory of original versus copied content. Keep unique observations, backlinks, and plugin configuration. During transition choose one capture owner; disable duplicate capture only when the replacement is verified. Do not bulk copy the whole ledger into a new memory database.

### Improvement loop

Route an observation to a code/config fix, a scoped project rule, a project skill, an AEP upstream proposal, or a product decision. Check existing rules and rejected candidates first. A candidate includes its evidence, hypothesis, proposed diff, applicability, counterexample, validation cases, adoption authority, and rollback trigger. Record rejected candidates so later agents do not repeatedly propose the same failed fix.

Keep the production rule revision fixed during a story attempt. A newly adopted rule applies to future attempts unless an explicit correction requires stopping/restarting affected work. Memory of past approval is evidence to examine in its original scope, not standing authority for unrelated actions. Retire stale rules and superseded lessons from default recall while retaining their history.

## 8. Migration plan

This is a content and consumer migration, not a directory rename. Reuse the instruction migration from the [earlier proposal](project-rules-and-story-workflow-refactor.md) and extend its inventory.

### Source-to-target mapping

| Existing source | Target and preservation rule |
| --- | --- |
| `product-context.yaml` stories | Per-story ledger records; preserve IDs, descriptions, criteria, dependencies, ownership, attempts, costs, raw statuses, and unknown fields |
| Layers, waves, gates, execution slices | Explicit container/gate records and edges; preserve fractional labels, grouped-change membership, missing-wave state, and legacy ordering |
| `product/index.yaml`, maps, old inline product | Roadmap backbone/journeys/goals plus archived prior plans; preserve duplicate/range references for reconciliation |
| Architecture and ADR arrays | Roadmap architecture/decisions, with source provenance; link to actual code |
| Operational-truth/governance registries | Roadmap state or preserved existing canonical paths initially; update executable checks and project-rules before moving |
| Topology, routing, model baseline | Project AEP configuration/rules where normative; runtime handles stay with attempts; retain unknown/custom sections |
| Changelog, cost, historical events | Typed ledger events/accounting references, preserving original order and unknown dates; retain source snapshot |
| OpenSpec active/archive bundles | Changes plus linked design drafts, delta specs, checklists, and legacy metadata; preserve slug/path aliases |
| OpenSpec main specs | Roadmap specs baseline plus unverified/import provenance where evidence is absent |
| Lessons, convergence records, eval addenda | Singular lesson store and ledger evidence; keep round history, closure evidence, and raw-source links |
| Existing memory/plugin stores | Deduplicate only proven copies; preserve unique records and a single capture route |

Preserve historical evaluator rounds and historical policy violations as recorded; never truncate them to the current round cap. Empty lesson templates stay historical artifacts and do not count as established learning. Slug mismatches require evidence-backed aliases or an unresolved reference, not silent fuzzy matching.

A mapping manifest records source repo/full commit/path/section or record ID, source digest, target ID/path, transformation, and disposition. Every source field/content block must be mapped or explicitly retained in the import snapshot with an unresolved exception. Unknown data must not disappear through a narrow schema or a lossy merge function.

### Stages

1. **Freeze an auditable input.** Identify the exact commit and separately inventory dirty/untracked files and active worktrees. Use a detached snapshot or `git show` for analysis if the live repository moves. Never silently mix commits or absorb another agent's unfinished change.
2. **Add readers and migration validation.** Implement new-layout detection, reference checks, and read-only legacy adapters before changing consumers. The legacy snapshot remains authoritative during shadow comparison; generated new output has no active writer.
3. **Dry-run semantic conversion.** Generate the manifest and candidate tree in a disposable checkout. Preserve YAML extensions/comments in the source snapshot; parse with duplicate-key diagnostics. Report missing links, cycles, ambiguous map ranges, conflicting contracts, untracked changes, and undocumented status values. Do not repair unrelated product facts during conversion.
4. **Compare normalized views.** Match story/ID sets, dependency edges, container memberships, gate semantics, costs, change/spec content, decisions, and lesson provenance. Compare scheduler eligibility under the unchanged policy. Differences must be explained and approved as explicit design changes, not hidden in migration.
5. **Cut over one writer set.** Checkpoint/finish active old-contract work, or explicitly restart it on the new revision. Switch all active skill/script/app writers for the migrated scope together. Retain read-only compatibility views only where old readers need them; no bidirectional YAML synchronization. A scope that cannot stop legacy writers stays on the legacy store.
6. **Verify a real lifecycle.** In a fixture first, then a selected downstream pilot, run design → dispatch → isolated implementation → BDD evidence → integration → contract promotion → lesson → next-task recall. Exercise cancellation, partial implementation, simultaneous changes, stale evidence, and rollback. Re-run migration and require zero new changes for unchanged input.
7. **Retire compatibility deliberately.** Update the install pin, instruction marker, rendered recipes, migration ledger, CI/scripts, and command wrappers. Remove old writable paths only after consumer scans and lifecycle checks pass. Retain source snapshots and ID aliases for historical references.

Rollback uses the preserved base/import snapshot and a reviewed revert of migration commits. If new work occurred after cutover, first translate or preserve those new records; reverting the directory move alone would lose them. Unsupported reverse mappings stop rollback with an explicit recovery plan. Preserve user changes and never hard-reset a shared checkout.

### Consumer changes that must ship together

The shared [file resolver](../../skills/product-context/_shared/references/file-resolution.md), [product schema](../../packages/api/src/lib/product-context-schema.ts), [loader/watchers](../../packages/api/src/lib/product-context-loader.ts), [API router](../../packages/api/src/routers/product-context.ts), and [story-map UI](../../apps/web/src/routes/story-map.tsx) all consume the old contract. The current loader can return unvalidated data after schema failure; successful loading is therefore insufficient migration validation. Add explicit migration validation rather than relying on that fallback.

Update envision/map/model to own roadmap intent; design to own change/draft/BDD relations; dispatch/launch/build/autopilot to consume ledger readiness and attempts; wrap to own evidence, contract promotion, and integration; reflect/watch/workflow-feedback to own scoped lessons and proposals. Also migrate scaffold/onboard audits, signal schemas, generated copies, workflow aliases, local operational preflights, BDD renderers, memory hooks, dashboard caches/watchers, and release documentation. Inventory matches before editing; do not global-replace historical prose or vendor installations.

## 9. Acceptance and rollout decision

Required fixture cases include both inspected split layouts, a legacy single YAML, missing map links, range aliases, fractional/out-of-order layers, unassigned waves, grouped changes, custom governance fields, active and archived-but-unsynced changes, active-but-synced changes, complete checkboxes with no implementation evidence, duplicate IDs, and concurrent writers. Preserve known input anomalies as diagnosed facts; do not create new ones.

Migration acceptance requires complete source disposition, preserved IDs/dependency edges, equivalent scheduler eligibility under legacy policy, no stronger gate/completion claim without new evidence, restored relative links, no-op rerun, and a tested rollback. A static schema check cannot certify deployed behavior.

Context quality needs its own test. Build a small real-history-derived private question set with expected source references, then compare the current full-context workflow with the indexed proposal at a fixed repository snapshot and model/effort. Questions must cover:

- Current product journey versus a maintenance ticket with no journey link.
- Which dependencies and gate prevent a story from starting.
- Which BDD scenarios are accepted, implemented, or still unverified.
- Why an alternative was rejected, and which later ADR superseded it.
- The difference between a completed experiment, a merge, and a release.
- Which lesson applies here, and which stale or out-of-scope lesson must not apply.
- What happened first when archive time and merge time differ, including honest unknowns.

Measure answer correctness, citation accuracy, missed/conflicting evidence, temporal/scope mistakes, unsupported completion/authority claims, retrieval time, and loaded context. Keep reference answers and held-out cases out of candidate optimization. Publish only sanitized fixtures if the underlying history is private. Do not claim a memory improvement merely because files are smaller.

**Recommended order:** agree on source ownership and record IDs; implement the resolver/import validator; migrate instruction/rule routing; pilot ledger/roadmap/spec/lesson stores with unchanged gates; simplify the workflow; then validate independent operation without the external OpenSpec CLI and evaluate learning-policy changes. OpenSpec format/interoperability support remains part of AEP 5.0. Preserve public command aliases and legacy readers during transition. Breaking required store/skill contracts needs the release and migration discipline in [release conventions](../../project-convention/release.md).

Research stopped after the material questions had pinned source evidence or explicit uncertainty: downstream structure, OpenSpec completion/path semantics, memory scope/promotion, and AEP consumer coupling. The folder contract, importer, downstream migration, and quality experiments remain proposed implementation work.
