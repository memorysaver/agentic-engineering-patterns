# Looplia isolated v5 preview migration pilot

Date: 2026-09-10. Result: **isolated migration applied; source preservation, structural verification, status, context and project-skill discovery pass. Original Looplia remains v4 and unchanged.** This is an agent-reviewed migration/context pilot, not an unattended importer or product implementation/production verification claim.

## Candidate and isolation

- Final inspected binary: `/tmp/aep-preview-final-7x626jh8/extracted/aep`, version `5.0.0-preview.1`, SHA-256 `11195ffddc9010c4cae91fa7062afd5c60defa01532a7c0736ef177862345175`.
- Original checkout: `/home/memorysaver/Work/github/looplia`, develop, HEAD `eebcd25d1f7477b93da912d67d43bf5e4e8b7011`.
- Independent local clone: `/tmp/aep-looplia-preview-20260910`, branch `aep-v5-preview-pilot`, source snapshot commit `7fa9542f4ce010064844dadf89106cf1f9f9dcf5`. Cloned without hardlinks, no push remote. Original Git/worktree metadata was not changed.
- Snapshot includes only the four current non-host dirty inputs (commits convention, CI draft, two setup notes) and a new tracked consumer review. Local concurrency-hook settings were kept outside version control. No ignored workstation settings, credentials, `.env`, dependencies, runtime state, services, external provider calls or production operations were copied/run.
- Native migration changes are left uncommitted in the isolated clone for inspection. Original checkout still has its initial five dirty paths and unchanged HEAD.

## What the real downstream data exposed

The first packaged candidate failed planning on ten PNG lesson screenshots because it assumed every source asset was UTF-8. That failure was reported, fixed upstream, and re-tested through the real downstream data. The source-safe implementation retains eleven assets at original source paths with commit/path/digest provenance (ten PNGs plus the retained noncanonical l41-006a2-per-item-watermark.md lesson asset); it does not discard evidence or invent text encodings. First-attempt report: `/tmp/aep-looplia-pilot-first-attempt.md`.

The corrected automatic plan still honestly reported **161 semantic diagnostics**. Requesting `L41-006a2` and `L41-006b` expanded to **362 stories / 708 proposed records**, because the selected work references earlier-layer gates and their dependency closure. This is a material scope/ergonomics observation: a two-story request is not a two-story migration.

The pilot agent reviewed and transformed a separate plan, retaining both the original and a per-diagnostic resolution ledger. It did not erase diagnostics to force success:

- 160 referenced changes were historical archives rather than active OpenSpec changes. 159 resolved through unique exact/date-prefixed paths. LR-033 required reading its proposal/design because the archived slug says `daemon-agent-skeleton` while the legacy reference says `looplia-agent-skeleton`; the proposal and design explicitly identify LR-033 and its matching layer/contract. Each story now has exact archive commit/path/file digests, preserving status and recording that this is source evidence, not an active Change or fresh completion.
- The topology diagnostic was resolved into `project-rules/preview-operating-policy.md`, the indexed native entrypoint and actual configuration. The mapping preserves independent contract review, context/handoff requirements, three global workers, one implementation per module, conflict checks, bounded recovery, owner gates, and all existing sensitive-path/production verification obligations. Historical v4 scheduler, permission and terminal recipes remain identified as historical mechanisms. The CLI does not newly implement their external-host capabilities.
- `independent_review=true` remains required for Looplia; `max_parallel=3`. The existing `skills/e2e-test` procedure is registered and discovered by the native catalog. The existing gitleaks and Worker SQLite commands are configured, **not executed**. One explicit imported deep verification tier is preserved instead of defaulting to standard.
- Imported root OpenSpec config was reviewed field by field and mapped with the supported `aep openspec map` command to `project-rules/openspec-root-context.md`. Current AI-employee product intent and develop-to-main release routing are distinguished from the older remote-shell/main-only prose; valid stack/protocol constraints are preserved.

## Executed checks

| Check | Result and boundary |
| --- | --- |
| Reviewed-plan apply dry run | Pass, zero side effects |
| Apply in independent clone | Pass; only clone receives native stores |
| Explicit root OpenSpec mapping | Pass through supported CLI command |
| Final-candidate `migrate verify` | Pass, exit 0, zero diagnostics; source commit/digests checked |
| Final-candidate `check` | Pass, exit 0, zero structural diagnostics; 710 stored records |
| Final-candidate `status` | Pass; native records readable |
| Final-candidate `context L41-006b` with active rule sources | Pass; 188 linked records, zero missing references; L41-006b remains blocked, L41-006a2 remains imported/unverified |
| Final-candidate `skills` | Pass; eight native skills plus project e2e-test discovered |
| Final-candidate `config show` | Pass; independent review and actual check commands preserved |
| Repeat reviewed apply | Pass; `already_migrated=true`, zero files |
| Source comparison | 2,176 source files checked, zero changes in original, zero preserved-source differences in clone; original AGENTS bytes preserved in project-rules/legacy-entrypoint.md |

Final inspection command outputs: `/tmp/aep-looplia-final-{verify,check,status,context,skills,config}.json`. Repeat-apply result: `/tmp/aep-looplia-repeat-apply.json`. Preservation result and manifest: `/tmp/aep-looplia-preservation-result.json`, `/tmp/aep-looplia-source-hashes.json`.

Plan artifacts: `/tmp/aep-looplia-final-original-plan.json`, `/tmp/aep-looplia-final-reviewed-plan.json`, `/tmp/aep-looplia-resolution-ledger.json`, `/tmp/aep-looplia-resolve-plan.py`. Native clone also contains the complete resolution ledger under project-rules.

## Remaining work and practical limits

- No Looplia test suite, gitleaks, dependency installation, product service, provider, production journey or deployed receipt was exercised. A historical 31-test SQLite pass is not current evidence. This pilot proves native migration/context orientation, not full self-verification or a shipped product feature.
- Thirteen imported historical changes retain **28 unmapped custom artifacts**. None was accepted or published; accepting affected changes still requires their own semantic review. A clean global structural check does not mean every imported draft is acceptance-ready. No BDD/spec-repair work was performed to manufacture a clean result.
- L41-006b remains blocked on the owner's exact source-origin policy; its retired watermark-cap blocker was not revived. Other owner's entitlement/design gates and deferred/failed statuses remain applicable. Imported completions and archived changes confer no automatic current pass.
- Module concurrency, sensitive-diff derivation, external watch ingestion and other guidance-level obligations remain agent/host responsibilities where no corresponding CLI enforcement exists. Configuration does not waive the production evidence policy.
- Next useful downstream work is a freshly authorized, dependency-provisioned local verification run with real SQLite evidence, then one bounded native task. Production work still requires the existing Looplia exact-SHA/attended preflight and CI-only deployment constraints.
- Original Looplia has not cut over. A real in-place migration needs review of the isolated diff, source snapshot handling and remaining constraints; the pilot does not authorize publishing or pushing its changes.
