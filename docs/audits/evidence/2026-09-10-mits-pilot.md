# MITS isolated AEP v5 preview pilot

Result: native migration applied and post-migration orientation completed in an independent local clone. Native validation remains blocked by seven inherited BDD syntax failures. This is a successful observation experiment, not a successful release/development-readiness gate or autonomous dispatch.

## Snapshot and binaries

- Original: `/home/memorysaver/Work/github/MITS`, clean `main` at `1de339116c11c9a7a8f6de12c638cdb677b98d46` before and after; original unchanged: `True`.
- Independent clone: `/tmp/aep-mits-pilot-rly6kn3y/MITS`, no Git remotes. Pilot checkpoint `7e09f476b2b654220f3b446b0ff32e2ccfc5273b`, clean. Ignored runtime state, machine-local settings, dependency outputs and production memory vault were not copied. Tracked project hooks were preserved.
- Original release candidate AEP 5.0.0-preview.1 source `808fa3e`, binary SHA-256 `d0b8a6ff43014e5e12cda86ec3ea6dad71f55dd78c0a4c2db4a782d0a6a948ef`.
- First central collision repair used for apply: `/tmp/aep-pilot-id-fix`, SHA-256 `11b980c3db187bdec9c520f1f6dbd176b795081c0b7bf5d45953cdd0777b771e`.
- Final packaged candidate used for release read recheck: `/tmp/aep-preview-final-7x626jh8/extracted/aep`, SHA-256 `11195ffddc9010c4cae91fa7062afd5c60defa01532a7c0736ef177862345175`. The preceding packaged candidate also passed its recorded read checks; its evidence is retained.
- Structured evidence: `/tmp/aep-mits-pilot-rly6kn3y/evidence`. Full command outputs, original/reviewed plans, source hashes, config and snapshots are retained there. The final candidate includes the central prospective-config validation repair. This report names the exact final binary actually tested.

## Actual migration and context result

Initial default planning failed exit 5 because story and OpenSpec change IDs overlap (`MITS-069/070/071/072/073/089`). Central repair introduced source-bound `openspec-change-<id>` aliases. Planning then produced 265 native records and 93 review diagnostics.

Every diagnostic has an individual resolution in `project-rules/migration-mapping-resolutions.json`, with the original list retained. Topology became an active rule plus WIP 1, independent review and required repo/preflight/security checks. Ninety exact dated archive references became source-bound historical change provenance on imported stories. MITS-057's deleted proposal was recovered from Git with a digest and its evidence-only failed-gate closure retained. MITS-078 has no authored change and remains explicitly deferred with a design gap. No active change was fabricated. Imported completed stories and gates remain unverified.

The applied snapshot contains {'decision': 32, 'story': 107, 'gate': 19, 'roadmap': 1, 'layer': 20, 'import': 2, 'change': 9, 'wave': 75}. Legacy data and v4 skills remain preserved; the root route selects v5 and future writes use native stores. Local verification procedure `skills/e2e-test` is registered in native skill discovery. The project-specific independent review, deep sensitive-path floor, benchmark restrictions and work holds remain active.

`aep context MITS-072` succeeds with 162 related records, no missing record references, the original six acceptance criteria, dependencies MITS-059/MITS-066 and the explicit mapped OpenSpec change. The agent used the rules index and wrote `docs/decisions/aep-v5-preview-orientation.md` without starting implementation. `aep context` returned no source content when no --source was requested; agents must still read AGENTS and rules. MITS-072 remains a candidate; Layer 19 is paused.

## Checks actually observed

| Check | Result |
|---|---|
| Local Rust preflight | PASS |
| Rust 1.95.0 formatter | PASS |
| Offline locked `help_smoke` integration test | PASS, 10 tests |
| Retained Layer 18 archive verifier and self-test | PASS, 31 members |
| Synthetic vault init/checkpoint | PASS |
| Initial synthetic recall | Correctly failed missing derived current view, exit 60 |
| Synthetic state rebuild + BM25-only index rebuild + recall | PASS; local_only/vector_unavailable retained |
| Packaged `aep doctor`, status, context, skills | PASS |
| Packaged `aep check`, `aep migrate verify` | FAIL, seven inherited missing-WHEN scenarios |
| Native verify plan for MITS-072 | BLOCKED as expected: no active attempt |
| Git diff check | PASS |
| Original Git state before/after | Identical and clean |

The seven BDD issues occur in imported capture-adapter-kit, fixture-harness and packet-builder specs. They reflect source scenarios with Given/Then and no explicit When. No diagnostics or acceptance criteria were removed to force a green result. Source hashes and migration receipt checks completed; the overall verifier remains failure because the spec check fails.

## Concrete product and workflow findings

1. Same-ID legacy story/change import collision: fixed centrally and exercised on real MITS data.
2. Legacy deferred statuses initially became pending: corrected in reviewed conversion and fixed centrally. Held work must not become dispatchable.
3. Archived change references require substantial agent semantic mapping. The reviewed plan is deliberately manual and auditable; this was not an automatic migration success.
4. Invalid proposed config was accepted by apply: a pilot authoring typo produced `required_protected_paths`. Subsequent reads rejected it. The typo was corrected, evidence retained (`malformed-reviewed-config.toml`, `migration-verify-before-config-repair.json`), and central prospective-config validation is fixed and independently rechecked. A second config authoring error used parent `skills` instead of exact `skills/e2e-test`; corrected through `aep config update`, after which skill discovery passes.
5. Native check truthfully exposes inherited BDD debt. Followup is semantic repair of the seven native trigger clauses, preserving original sources, then rerun validation.

## Limits and remaining work

No production vault read/write, external messages, provider call, deployment, ignored benchmark, dataset/model download or process termination occurred. The synthetic memory fixture is separate and intentionally BM25-only. No feature was selected or implemented. Full `bun run check`, full Rust suite, vector/hybrid development readiness and independent implementation review were not claimed; they remain project gates when applicable. The clone lacks installed JS dependencies; tool availability is recorded separately. The pilot checkpoint was committed locally with hooks disabled and known validation failures explicitly retained, not promoted as a passed release.

Next: retain this experiment as downstream evidence, fix the seven native BDD trigger gaps with semantic review, rerun native verification and applicable project gates, and separately reconcile historical completions/gates before selecting new product work. The live MITS project remains on its original v4 state.

## Final release artifact recheck

Final binary SHA-256 `11195ffddc9010c4cae91fa7062afd5c60defa01532a7c0736ef177862345175`: doctor, status and MITS-072 context PASS; check and migrate verify retain exactly the same seven BDD failures (exit 1). No spec repair, reapply or state change was performed. Both the original repository and isolated pilot checkpoint remain clean and unchanged. Exact outputs: `evidence/release-*.json`; aggregate `evidence/release-native-checks.json`.
