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

A task context packet joins these sources through recorded links and explicit query scope. It is a derived view with citations and revisions, not another authoritative `project-context.yaml` or a mandatory vector database. The agent decides which task, skills, and additional sources are relevant; the CLI retrieves requested records and diagnoses missing links without claiming complete semantic context.

## 4. Proposed file structure

Create only the records a project needs. Retain existing application and package directories.

```text
AGENTS.md
project-rules/
  README.md                      # task/path -> applicable maintenance rules
  context.md                     # this project's source/authority routing
  aep.md                         # project workflow; links to CLI configuration
  skills/                        # optional project-owned procedures, e.g. monet-*
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
  imports/<migration-id>.yaml     # Git base/path references and current ID mappings
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
  archive/                       # selected prior plans useful for current work
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

These directories hold project content and are created as needed. The default tooling install is the Rust binary plus the AGENTS.md entrypoint to `aep skills`; built-in skill content is bundled in that binary. `.aep/config.toml` records the CLI release and project settings as defined by the [5.0 architecture](aep-v5-rust-cli-architecture.md#3-storage-and-versioning).

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

Import existing main specs as the current baseline and carry forward active changes needed for ongoing work. Check whether those changes were already synced before applying their deltas. Archived bundles remain available through their Git commit/path; import a selected bundle when its reasoning or evidence is useful. Do not replay historical deltas to reconstruct current truth or batch-run archive as a migration step.

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

Git owns the old project's history and recovery. Migration carries useful current context into the new workflow and updates the consumers that use it. Full historical conversion, duplicate source snapshots, and a dedicated reverse-migration engine are outside the 5.0 requirement. Reuse the scoped instruction migration from the [earlier proposal](project-rules-and-story-workflow-refactor.md).

### Source-to-target mapping

| Existing source | Current context to carry forward |
| --- | --- |
| `product-context.yaml` stories | Active work and completed dependencies/relevant history, with IDs, criteria, dependencies, ownership, and evidence-backed status; older tickets can remain in Git |
| Layers, waves, gates, execution slices | Containers and prerequisites relevant to migrated work; preserve label aliases, grouped changes, and active gate scope |
| `product/index.yaml`, maps, old inline product | Useful current backbone/journeys/goals; omit invented story-map links and retrieve older plans from Git |
| Architecture and ADR arrays | Current architecture and durable decisions, with Git provenance and code references; keep superseded reasoning linked where it explains current decisions |
| Operational-truth/governance registries | Applicable claims, limits, and checks; keep existing canonical paths if moving them adds no value |
| Topology, routing, model baseline | Applicable project configuration/rules; runtime handles stay with active attempts |
| Changelog, cost, historical events | Selected events/accounting facts needed now; older logs remain accessible by Git base/path |
| OpenSpec active/archive bundles | Current changes and their drafts, BDD deltas, checklists, and aliases; archived bundles are retrieved on demand |
| OpenSpec main specs | Current roadmap specs baseline, with unknown implementation evidence kept explicit |
| Lessons, convergence records, eval addenda | Useful lessons and supporting evidence links; do not require reformatting every old evaluation |
| Existing memory/plugin stores | Keep unique useful context and an explicit capture route; register existing paths where appropriate |

When importing or retrieving history, retain what it actually establishes. Historical evaluator rounds do not become compliant by truncating them to the current cap. Empty templates do not establish learning. Slug mismatches need supported aliases or an unresolved reference. These are interpretation rules for selected records, not a requirement to convert all history.

A small migration receipt records the source repository/base commit and relevant paths, selected scope, ID aliases where needed, and unresolved current mappings. A historical lookup can use `git show <base>:<path>` and retain that citation. No per-field disposition ledger or copied source blobs are required. An unknown field that governs current work must be resolved or explicitly block that work; an unused historical field can remain in Git without delaying adoption.

### Stages

1. **Record the Git base.** Use a migration branch and record the source commit. Identify relevant uncommitted work and active attempts; preserve unrelated edits. Required current content must be included deliberately, not mixed into the input by accident.
2. **Plan current context.** The agent selects useful records and identifies current rules and consumers. `aep migrate plan` computes the conversion for that explicit scope and reports unresolved active references or constraints. Include completed dependencies and cited decisions needed to understand ongoing work. Leave bulk history at its Git source.
3. **Apply and switch consumers.** `aep migrate apply` writes the candidate structure through normal CLI transactions. Switch the affected entrypoints, scripts, and app consumers together. Finish old-contract attempts or restart them explicitly; a scope with continuing old writers stays on its old store. Keep read-only compatibility only where a consumer needs it.
4. **Verify usability.** `aep migrate verify` checks parsing, current IDs/dependencies/container links, declared gate constraints, indexed rule links, and the AGENTS-to-CLI entrypoint. The agent reviews whether current intent and applicable rules carried forward. Retrieve a current story's design, ADRs, and evidence through the new context path and retrieve an old file from its Git reference. Require no new diff on rerun. Check dispatch eligibility under unchanged policy for migrated work; historical coverage is not an acceptance gate.
5. **Commit the cutover.** Commit the coherent CLI pin, entrypoint, data/rule changes, and consumer updates. Remove obsolete writable paths and unused AEP skill copies once their consumers switch. Record any remaining current-work issue in the PR; do not claim adoption complete while the active workflow is broken.

Recover old files with Git, or revert the migration commits in a reviewed branch. Resolve later edits with normal Git conflict handling and preserve unrelated work. AEP does not need a reverse schema translator or a second backup store for this operation.

### Consumer changes that must ship together

The shared [file resolver](../../skills/product-context/_shared/references/file-resolution.md), [product schema](../../packages/api/src/lib/product-context-schema.ts), [loader/watchers](../../packages/api/src/lib/product-context-loader.ts), [API router](../../packages/api/src/routers/product-context.ts), and [story-map UI](../../apps/web/src/routes/story-map.tsx) all consume the old contract. The current loader can return unvalidated data after schema failure; successful loading is therefore insufficient migration validation. Add explicit migration validation rather than relying on that fallback.

Update envision/map/model to own roadmap intent; design to own change/draft/BDD relations; dispatch/launch/build/autopilot to consume ledger readiness and attempts; wrap to own evidence, contract promotion, and integration; reflect/watch/workflow-feedback to own scoped lessons and proposals. Also migrate scaffold/onboard audits, signal schemas, generated copies, workflow aliases, local operational preflights, BDD renderers, memory hooks, dashboard caches/watchers, and release documentation. Inventory matches before editing; do not global-replace historical prose or vendor installations.

## 9. Acceptance and rollout decision

Use representative legacy single-YAML and split-layout fixtures. Cover current dependency/container relationships, active custom governance, grouped changes, already-synced changes, complete checkboxes without implementation evidence, duplicate IDs, and writer conflicts. Missing journey links or old fractional labels must not force invented structure. Historical variants need dedicated conversion only when a project actually imports them.

Migration acceptance requires usable current context, preserved IDs/dependency edges for that scope, equivalent readiness under the existing policy, no stronger gate/completion claim without new evidence, working rule/instruction links, Git source references, and a no-op rerun. A static schema check cannot certify deployed behavior. Validate the native design → implementation → evidence → integration → lesson → next-task recall lifecycle in the 5.0 pilot; exhaustive old-history parity is not its purpose.

Future context capture and retrieval need their own quality test, separate from migration completeness. Build a small real-history-derived private question set with expected source references, then compare the current full-context workflow with the indexed proposal at a fixed repository snapshot and model/effort. Permit Git-backed historical lookup and measure its cost. Questions must cover:

- Current product journey versus a maintenance ticket with no journey link.
- Which dependencies and gate prevent a story from starting.
- Which BDD scenarios are accepted, implemented, or still unverified.
- Why an alternative was rejected, and which later ADR superseded it.
- The difference between a completed experiment, a merge, and a release.
- Which lesson applies here, and which stale or out-of-scope lesson must not apply.
- What happened first when archive time and merge time differ, including honest unknowns.

Measure answer correctness, citation accuracy, missed/conflicting evidence, temporal/scope mistakes, unsupported completion/authority claims, retrieval time, and loaded context. Keep reference answers and held-out cases out of candidate optimization. Publish only sanitized fixtures if the underlying history is private. Do not claim a memory improvement merely because files are smaller.

**Recommended order:** prove the short AGENTS entrypoint, CLI skill catalog, and agent selection of guidance; establish native context ownership, schemas, and checks; add practical current-context conversion; pilot the lifecycle with unchanged gates and no external OpenSpec CLI; then evaluate learning-policy changes. OpenSpec format/interoperability support remains part of AEP 5.0. Keep aliases or legacy readers only where an active consumer needs them. Breaking required store/skill contracts needs the release and migration discipline in [release conventions](../../project-convention/release.md).

Research stopped after the material questions had pinned source evidence or explicit uncertainty: downstream structure, OpenSpec completion/path semantics, memory scope/promotion, and AEP consumer coupling. The folder contract, importer, downstream migration, and quality experiments remain proposed implementation work.
