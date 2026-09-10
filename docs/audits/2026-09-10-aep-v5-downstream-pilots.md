# AEP v5 preview: three downstream experiments

The owner selected Looplia, then requested parallel experiments on MITS and Rewarc. Each pilot uses an independent local clone with its remote removed. Live projects, uncommitted work, host settings, production resources and provider jobs remain unchanged. These are migration and context/verification observations, not production deployments or feature-completion claims.

## Observed outcomes

| Project | Migration response | Actual project verification | Cutover status |
| --- | --- | --- | --- |
| MITS (Rust) | Found overlapping story/OpenSpec IDs and deferred-state loss; repaired centrally. A reviewed plan explicitly maps topology and historical references into native context. | Offline Rust smoke tests: 10 passed; portable evidence verifier/self-test passed; synthetic vault init, checkpoint, rebuild and BM25-only recall exercised. | Isolated native context applied; current verification reports seven legacy scenarios missing WHEN. Production dispatch remains unproven. |
| Looplia | Binary screenshot evidence initially crashed plan creation. Repaired retention lets the plan bind 11 assets. Two selected stories expand to 362 stories through dependencies and layer barriers. | Source-byte comparisons and CLI migration/context checks; application services and provider journeys are outside this pilot. | Reviewed apply, migrate verify, check and context pass (710 records). Product tests were not run; 28 custom artifacts on 13 historical draft changes still need mapping before acceptance. |
| Rewarc | A custom Purpose section needs documented clone-only normalization. The normalized plan proposes 125 writes but reports 63 unresolved items: an active attempt, topology and 61 archived references. | Operational-truth check passed (25 capabilities, 34 gates); 100 governance tests passed. | Apply correctly blocked, v4 remains the owner. Empty native check/status success is not migration success. |

## CLI corrections from the experiments

- Read legacy trees as bytes. Keep binary assets outside native UTF-8 stores at their original paths, with source commit/path/digest in the receipt. Rebase recognized explicit references; require mapping for unfamiliar syntax. Source changes still invalidate plan/verification.
- Reserve all legacy story IDs when importing OpenSpec, including later scopes. Colliding changes receive stable native IDs, preserving source identity and artifact paths. Source aliases are scoped to the actual bundle so an unrelated import cannot hijack a story link. Standalone reimport recognizes the prior mapping.
- Preserve non-completed blocked/deferred/cancelled/unknown states. Migration cannot silently turn a project hold into dispatchable pending work.
- Validate a reviewed plan's prospective configuration before writing. Invalid fields, deleted configuration, incompatible versions and changed store roots are rejected; candidate records are checked under the proposed policy.

Regression fixtures cover binary drift and links, unsupported reference syntax, scoped ID stability, foreign-source aliases, retained holds, and invalid prospective configuration. The final workspace suite has **62 passing tests**; formatting and Clippy pass. macOS CI then exposed a fixture assertion comparing `/var` with its canonical `/private/var` spelling. The assertion now compares canonical paths, matching the importer's existing behavior; no production code changed for this platform correction.

## Remaining adoption work

Historical archived change references currently need attributable reviewed-plan mappings. Preserve the original plan and an item-by-item resolution record, bind exact historical files/commits, and keep imported completions unverified. Do not invent a live change merely to satisfy a reference. Project topology must become active policy, configuration and checks; copying prose or deleting diagnostics is insufficient.

Rewarc's mandatory operational-truth and preflight scripts still read legacy product-context directly. A real v5 writer cutover requires an executable reader transition, plus an explicit checkpoint/restart of OBS-001. Source normalization does not resolve those constraints.

MITS retains WIP=1, independent review, local-only and protected-resource boundaries. Its successful synthetic vault check does not establish hybrid/vector readiness, production-vault restoration, or approval to launch deferred cloud/research work. The BDD failures stay visible.

Looplia's 161 plan diagnostics were resolved individually (160 historical references and topology), with the generated/reviewed plans and resolution ledger retained. It preserves 2,176 source files, keeps L41-006b blocked and L41-006a2 imported/unverified, and enforces independent review. Repeat apply is a no-op. Looplia retains owner decisions and production journey gates. Its broad dependency closure is itself a usability finding: a small requested migration scope need not produce a small import. Historical completed labels remain unverified, and missing source-origin decisions remain blockers. The YAML-based input reader rejected JSON surrogate escapes during plan authoring; literal UTF-8 serialization worked and is recorded as a remaining input-compatibility limitation.

## Evidence locations

Full local command transcripts and before/after manifests are retained in the pilot evidence directories. Durable reports: [MITS](evidence/2026-09-10-mits-pilot.md), [Rewarc](evidence/2026-09-10-rewarc-pilot.md), [Looplia](evidence/2026-09-10-looplia-pilot.md). Earlier candidate hashes identify the exact binary used at each step; final rechecks are recorded separately rather than retroactively attributing all observations to one build.

- MITS: `/tmp/aep-mits-pilot-rly6kn3y/evidence`, isolated repository `/tmp/aep-mits-pilot-rly6kn3y/MITS`.
- Looplia: `/tmp/aep-looplia-preview-20260910`, initial failure `/tmp/aep-looplia-migrate-plan-result.json`, retained-asset retry `/tmp/aep-looplia-assets-retry.json`.
- Rewarc: `/tmp/aep-rewarc-pilot-ga18j4vs/evidence`, isolated repository `/tmp/aep-rewarc-pilot-ga18j4vs/repo`.

No public tag or release was created by these experiments. The prepared prerelease and actual project adoption remain separate decisions.
