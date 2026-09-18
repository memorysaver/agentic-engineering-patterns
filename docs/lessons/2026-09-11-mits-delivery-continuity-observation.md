# MITS delivery continuity after the AEP correction

Date: 2026-09-11 (Asia/Taipei)
Completed observation: 15m 52s of downstream execution after the prompt. This records a live downstream agent, separate from the simulated guidance review in [the correction audit](../audits/2026-09-11-aep-v5-delivery-continuity.md).

## Request and observer boundary

The user asked: “引導他更新 aep 然後收尾 我們觀察看看”. The observer used Herdr to prompt the existing MITS agent at `w4:p2`; the observer does not implement or integrate MITS on its behalf.

Herdr initially rejected the prompt with `agent_blocked`. Inspecting the queued question showed the old MITS-108 versus reflection selection, which the user had already resolved in favor of the reflection story and the agent had implemented as MITS-109. The observer skipped that superseded question with the UI's `ctrl+]` action; Herdr then reported `done`. No alternative answer was selected. The subsequent prompt was delivered once; its short wait timed out, and a read confirmed the agent was working, so it was not resubmitted.

Exact submitted prompt:

> AEP 5.0 的本機 preview 已更新。請確認你目前使用的是這次新版 AEP，重新讀取需要的內建指引，然後接續把 MITS-109 依原本任務與專案流程收尾。新版 executable 位於 /home/memorysaver/.local/share/aep/builds/98689e4053158a96/bin/aep，SHA-256 是 98689e4053158a9619df4199bbcd8de1aa6c63de4b2a86aacb52cdb365f0d572；版本仍為 5.0.0-preview.1。請自行判斷剩餘工作並執行，最後回報實際完成的交付、收尾結果及仍待決定的事項。

This is an explicitly prompted continuation after a guidance update. It cannot establish that an entirely unprompted implementation session would now continue correctly.

## Starting state

- Main checkout: `design/status-evidence-scope`, head `21347f7562d29a14ce25b0dce124631b88ebab12`, clean.
- MITS-109 candidate: sibling worktree `MITS-status-evidence`, head `8d275417989befc25e0762a0ee2da86272cd571a`, clean, existing checks/review eligible.
- MITS-108: sibling worktree `MITS-adoption-fix`, head `c2e390ac0ab4afba5325d2d299c672e0e46c17cc`, preserved unintegrated.
- MITS-109 story description included the agent-authored restriction “no merge or live-vault checkpoint backfill”. The original user request had been “這次 reflect 產生的故事是哪一個？自主完成”. This demonstrates an additional failure mode: an agent's narrowed interpretation can become durable task context.

## Observed continuation

At approximately 00:34, the agent independently checked `command -v aep`, the resolved executable path, SHA-256 and version. PATH already resolved to the supplied updated binary; no additional installation was needed. It reread AGENTS/project rules, the root skill, deliver, closure, validate, implement and the project-owned verify-mits procedure.

It read the current story and `deliver plan`, then Git remotes/branches/diff, delivery command help/status and GitHub repository/PR state. It stated that the prior “no merge” restriction was self-authored and would reassess the endpoint using the current request while preserving MITS-108. No observer supplied a command sequence, integration base or solution to downstream branch state.

The agent selected the repository's fetched default branch `main` and announced PR/integration/spec/change closure, preserving MITS-108's product changes and installed executable. It attempted to update the accepted story to correct its narrowed scope, but `story update` rejected it with “Accepted records are immutable”. It then authored `project-ledger/changes/status-evidence-scope/delivery-context.md` to retain the newer user request, target, constraints, binary provenance and intended outcome. This is an observed recovery; whether later consumers consistently reconcile the immutable old wording with newer context remains an open context-maintenance issue. The observer supplied no fix.

The agent also created and accepted a linked native decision, `project-roadmap/decisions/mits109-delivery-endpoint.md`, so the continuation authority is more than an unlinked note. It then merged the shared context branch into the candidate, which caused `verify plan` to reject an attempt record as outside story scope. Recovery preserved that merge on the shared context branch, restored the clean product worktree to its original `8d27541` head, and resumed verification with separate code/shared-store roots. It explicitly retained the authored story scope rather than broadening it to bypass the check. Ledger transport at delivery is a real workflow friction point exposed here.

## Skill provenance question during observation

The user noticed `aep --skill verify-mits` and asked whether AEP depends on MITS. The project catalog reports `source: project:project-rules/skills/verify-mits/SKILL.md`. It is MITS-owned verification guidance introduced during downstream migration, discovered from the configured rules store's skills directory; `.aep/config.toml` has an empty additional `skill_paths` list. The AEP source checkout's catalog contains only its eight bundled procedures. Searching native guidance and runtime source found no MITS/verify-mits reference. Runtime dependency remains generic local-skill discovery, not a dependency on MITS.

The human catalog currently mixes bundled and project entries without an obvious per-entry origin label, while JSON retains origin. Record this as a presentation gap; no CLI presentation change was made during live observation.

The user subsequently said “等等 這個idea 其實不錯” about project-owned verification skills. Preserve the architectural separation: AEP supplies generic context/verification responsibilities, each project supplies product-specific operation and verification knowledge, and the agent discovers both through one entrypoint. The presentation issue concerns visible provenance, not removal of local skill discovery.

With code and shared ledger separated again, the agent ran all four configured checks against the new context fingerprint, obtained a fresh independent review, and recorded the result. It ran the PR dry run, then `aep deliver pr --story MITS-109 --base main`, which opened [PR #101](https://github.com/memorysaver/MITS/pull/101) at candidate `8d27541`. It retained/pushed the context branch, updated the PR description, inspected provider mergeability and then used `aep deliver merge --story MITS-109`. The fetched remote main advanced from `67d9db9` to `36a7f03`. It proceeded to specification publication planning and integration-state inspection rather than ending after PR creation or merge.

Post-merge, the agent committed the GitHub integration receipt and merged remote main into the shared context branch. It resolved the one add/add story conflict by retaining the confirmed integrated record over the old pending snapshot, then fast-forwarded local main to the reconciled context/code commit `258bf9f01ef90d0b732e49d165c386ba3c8384ba`. A direct product-tree comparison against remote main passed. On this combined checkout it executed preflight, `bun run check`, full Rust tests (523 passed, 0 failed, 1 ignored) and `aep check`, retaining command/head/output digests in its integration-check artifact. These were explicit post-integration subprocess checks, separate from pre-merge native check receipts.

The agent then executed `aep spec publish --change status-evidence-scope` and `aep change close status-evidence-scope`; both succeeded. Native `aep check` passed with 349 records at this point. Thus the observed continuation reached specification publication and change closure as well as remote integration.


## Final outcome and independent read-only checks

The agent removed the integrated MITS-109 worktree and local/remote implementation/context branches, while preserving MITS-108's unintegrated worktree. A local context-branch deletion initially failed because its upstream was behind; after confirming ancestry in main, it unset that branch's upstream and used normal `branch -d`. Remote deletion used exact expected-head leases. Source/receipts/specs/lessons were retained before final handoff.

It wrote `lesson-learned/observations/mits109-delivery-closure.md` through the native lesson command, including integrated check identities/digests, cleanup results, remaining decisions and the observed workflow issues. It committed/pushed the lesson and recorded/synced a MITS progress memory (`20260910T164916Z112469374`). This was a progress entry, not a passing checkpoint or a live-vault acceptance claim.

The observer independently checked native read commands, Git state, worktree existence and GitHub PR state after the work:

| Surface | Confirmed result |
| --- | --- |
| PR #101 | MERGED into main at `36a7f0318ceca3526a4de8a92ac72b0e2f3de485` |
| MITS-109 | `integrated`, delivery `delivery-18d40342bfef3e52-128a98-0` |
| Attempt | `done` |
| status-evidence-scope change | `closed`, publication evidence retained |
| Final main and origin/main | Both `0cce911181b0482ee69e68919450c54cef9d2819`; clean checkout |
| MITS-status-evidence | Directory absent and no longer in worktree inventory |
| MITS-adoption-fix | Still present at `c2e390a`; not integrated by this task |
| Installed MITS | Agent verified unchanged SHA-256 `80cbf44ed2bd4adc60f511604eba1f7ec85ee0539c038ec44ab3329fbc64b87d` |
| Agent | Herdr `done`; final report after 15m 52s |

MITS-108 delivery and a future installed MITS update remain separate decisions. No AEP code, MITS product changes, or additional prompts were supplied by the observer during execution. The AEP observer only retained this report and the user-confirmed project-skill direction.

## Lessons and limits

The continuation now crossed the previous candidate-ready stopping point and completed remote integration, post-integration checks, spec/change closure, lessons and resource cleanup. This supports the corrected guidance for a prompted continuation in this session. A fresh implementation session's spontaneous continuation still needs separate observation.

Observed friction to retain for later work:

- Agent-invented scope restrictions can become immutable story context; correction required a newer linked decision and delivery context.
- Shared verification/attempt records merged into an implementation candidate trigger story-scope rejection. Separate-root guidance and delivery transport deserve clarification; recovery here was agent-led.
- Integrating the shared ledger conflicted with the older pending story snapshot in the merged candidate; actual delivery evidence resolved it.
- Native spec publication emitted a trailing EOF blank line flagged by `git diff --cached --check`. The agent retained canonical publisher output and explicitly recorded the warning; this is a publisher formatting issue, not a passing whitespace check.
- Human skill discovery needs clearer project-versus-bundled provenance. Preserve the user-endorsed local-skill interface and reflect-to-skill path.

Raw Herdr snapshots, observed final state, and copied command artifacts are retained locally under `~/.local/share/aep/trials/2026-09-11-mits-delivery-continuity/`, with file digests in `sha256.json`. They are supporting observations rather than a deterministic acceptance suite. Post-integration command output copied here is preserved separately from the downstream lesson's summarized hashes and its full pre-merge native receipts.

Follow-up: the user subsequently authorized fixing these findings in AEP 5.0. The implemented corrections, additional incoming-decision defect, independent review and installed artifact are recorded in [the downstream findings verification](../audits/2026-09-11-aep-v5-downstream-findings.md). This observation remains the historical account of the original run.
