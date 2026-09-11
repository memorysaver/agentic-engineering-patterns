# Rewarc AEP v5 isolated migration pilot — 2026-09-10

## Outcome

Full cutover is blocked honestly; no live checkout was modified. AEP 5.0.0-preview.1 initialization works and preserves the default v4 owner. Apply rejects unresolved context rather than converting this active research project incompletely.

- Original: `/home/memorysaver/Work/github/Rewarc-AutoResearch`, clean HEAD `e551a2ab368cb7f3b439e1a40a6596d4a4323987`.
- Candidate source: `808fa3e29530311ddc3fae942cde9ad1a1a98d0f`; binary `/tmp/aep-preview-release-uqtkfv0t/extracted/aep`.
- Isolated independent clone: `/tmp/aep-rewarc-pilot-ga18j4vs/repo`; origin removed. No original `.venv`, `.dev-workflow`, credentials or runtime jobs copied; existing original Python interpreter used read-only with isolated cwd, sanitized environment and bytecode disabled.
- Evidence: `/tmp/aep-rewarc-pilot-ga18j4vs/evidence`. Exact command argv, exit codes, stdout/stderr are retained in per-command JSON.

## Actual compatibility response

1. Initial migration plan fails exit 4 on `## Purpose` in the active `local-research-operations` delta. The common native OpenSpec profile only accepts delta sections. In the clone, preserve the exact purpose in design and preserve the entire ADDED section byte-for-byte; commit the documented transformation as `7cb517686aca95e97b9dad50bb8754044c487a91`. The original remains accessible in Git history. This preprocessing is required for this source; migration was not automatic.
2. Normalized plan produces 125 writes and 63 unresolved diagnostics. OBS-001 has an active legacy attempt. Project-specific topology is not yet mapped. 61 story change references are reported absent; 61 have one matching archived directory. The archive analysis lists exact matches and missing ones.
3. Apply exits 3, creates no native ledger, and keeps `AEP default: v4`. `doctor`, `check` and `status` succeed on the initialized empty native store; `check` reports zero records and does not prove migration. No postmigration product success is claimed.

## Preserved project policies and consumer gap

`project-convention/dispatch-truth.md` explicitly survives re-pins: run the operational-truth gate before queue mutation and per-story/group preflight before launch. Neither completed labels nor a workflow change may waive the checks. These scripts read `product-context.yaml` and product/index.yaml directly. A live native-writer cutover therefore requires an explicit native context adapter or reviewed reader transition; keeping a legacy snapshot while ignoring native changes would produce stale acceptance evidence.

`project-convention/local-owner-approval.md` preserves exact candidate/version/target human approval. No approval, provider job, trading action, network service, or process termination was executed. Existing memory conventions and calibration gates remain contextual constraints.

## Checks actually run

- Provider-free `scripts/check_operational_truth.py --repo-root .`: passed, 25 capabilities and 34 gates.
- Provider-free `pytest -q tests/python/governance/test_operational_truth_registry.py -p no:cacheprovider`: **100 passed in 36.40s**. Existing excluded provider markers preserved; no provider credentials passed to subprocesses.
- Native catalog has eight bundled skills; project and migrate guidance read; doctor succeeds.
- Default v4 preserved after init and rejected apply, and no native ledger created.
- Original before/after HEAD, complete Git status and hashes of all 2574 tracked files match exactly.

## Remaining work for a real cutover

Resolve archived change identities with attributable source paths; checkpoint/restart OBS-001 explicitly; map topology/calibration and operational-truth consumers into an executable native contract. Repeat apply/verify/context and real checks on the resulting native state. Preserve current provider, research-library approval and operational maturity boundaries.

## Final candidate recheck

The final extracted binary at `/tmp/aep-preview-final-7x626jh8/extracted/aep` (SHA-256 `11195ffddc9010c4cae91fa7062afd5c60defa01532a7c0736ef177862345175`) was rerun against the isolated repository. Doctor/status/check still pass on the empty native store; a fresh plan retains the same 63 diagnostics. No apply or project mutation was performed. Evidence: `/tmp/aep-rewarc-pilot-ga18j4vs/evidence/final-candidate-recheck.json`.
