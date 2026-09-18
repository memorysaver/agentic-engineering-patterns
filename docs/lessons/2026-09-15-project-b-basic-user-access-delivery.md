# project-b basic user access: two native deliveries in one turn

Date: 2026-09-15, Asia/Taipei
Status: live-observed downstream turn, 36 minutes 48 seconds; agent turn complete, Herdr status `blocked` throughout
Previous: [Project A story planning and auto research loop design](2026-09-11-project-a-story-planning.md)

The user asked the project-b agent (Codex, Herdr pane `w5:p1`, cwd `~/Work/github/project-b`, `develop`, aep `5.0.0-preview.1`) to close the current bugs first and then make basic functions usable, including account signup and login, so the product can be operated and explored as a user. The observer watched from the AEP repo pane from before the prompt to the end of the turn, snapshotted the transcript at each milestone, and compared native records, Git, GitHub and the CLI before and after. No prompt was sent, no question was answered, no project-b file was changed by the observer. Evidence bundle: `~/.local/share/aep/trials/2026-09-15-project-b-v5-progress/` (39 files, `SHA256SUMS`).

## Guidance and context consumption

Before the prompt, in an earlier turn, the agent had run `aep --skill`, `aep --skill roadmap`, `aep --skill roadmap --ref status`, `aep status`, `aep context FIX-L41-SUPERVISOR-RPC-503-002`, `aep timeline`, `aep check`, and read the candidate story YAML and two handoff docs. Its recommendation (503 first, then L41-006b, then CI baseline) matched the CLI: 786 records, 3 ready, 0 ready to start, check pass.

After the prompt, visible commands before any write: `aep --skill implement`, `aep --skill design`, `aep --skill validate`, and `--help` for `config`, `verify run`, `story record`, `attempt record`. During the turn it also loaded `aep --skill deliver`, `--ref closure`, `aep --skill validate --ref self-verification`, and `aep --skill reflect`. It asked one environment question (local first, production site, or separate staging) with three options, said it would continue with locally reproducible work meanwhile, and did so. The question was never answered; a second queued question was not recoverable from the alternate screen.

## Native writes and classification

Records created through native commands, in order: layer `basic-user-access` with a design artifact `docs/design/2026-09-15-basic-user-access.md`; decision `prioritize-basic-user-access-20260915`, accepted with `--by owner-direction-20260915`; change `basic-test-baseline` with spec, accepted the same way; story `BASIC-001`. Later: change `basic-account-journey` with spec, accepted the same way; story `BASIC-002`; story `FIX-DEPLOY-STARTUP-CPU-001` (pending, with listed unknowns); lesson `basic-test-interface-drift` (pending).

Acceptance was attributed to the user's chat direction, not to a user action on the records. The design doc and decision text restate the direction accurately. Whether one prompt should carry `attributed_decision` acceptance for a change is a policy question for the user; structurally the records are consistent and `aep check` accepted them at every snapshot (786 → 820 → 823 → 824 → 846 → 855 records, all pass).

| Story     | Attempt                                                                            | Checks                       | Review                                               | Delivery                            | Result                                      |
| --------- | ---------------------------------------------------------------------------------- | ---------------------------- | ---------------------------------------------------- | ----------------------------------- | ------------------------------------------- |
| BASIC-001 | worktree `attempt-18d57d0491cb10ea-35c41d-0`, head d0e467bf                        | 3 pass                       | round 1 pass, 0 findings                             | PR #363 merged 12:11:07Z → 21f799ce | `integrated`, change closed, spec published |
| BASIC-002 | worktree `attempt-18d57d67ef53255d-35fc11-0`, heads bbd6d167 → f6329b6e → 37e69e31 | 3 × 3 pass, one set per head | round 1 superseded, round 2 superseded, round 3 pass | PR #364 merged 12:36:04Z → de86eb41 | `integrated`, change closed, spec published |

Both attempts were dispatched with `aep dispatch start`, moved through `attempt record --status review`, `review request`, `verify run`, `deliver plan`, `deliver pr`, `deliver merge`, `spec publish`, `change close`. Delivery records carry the attempt head, merge commit, fingerprint, producer binary hash and provider PR state; GitHub confirms both merges.

## Review loop with real findings

The reviewer was a Codex subagent (`/root/basic_baseline_review`) started by the builder. For BASIC-001 it returned no findings. For BASIC-002 it found a fragment lost on post-login return (round 1) and a router-supplied fragment without a leading `#` (round 2); each produced a follow-up commit and a new review request against the new head. Independence is not verifiable from the records; the findings were real and were acted on.

The CLI rejected three record attempts, each visible in the transcript:

```
Error: unknown field `id`, expected one of `severity`, `description`, `resolved`, `evidence`
Error: Review attribution or current revision/fingerprint does not match
prerequisite (exit 3): Resolved findings require current passing evidence
```

After each rejection the agent corrected its input or retained the raw reviewer response as an evidence file (`project-ledger/evidence/basic-user-journey/review-round{1,2}-finding.txt`) and did not record a pass. The observations file states this explicitly. This is the intended split: Rust refused stale fingerprints and unevidenced resolutions; the model supplied content and chose how to retain what was refused.

## Verification

The agent ran the affected E2E suites, workspace typecheck, format, lint, `git diff --check`, gitleaks on staged content, `aep spec check`, `aep check` and native `verify run` at each candidate. It drove the real local API and web app through agent-browser for signup, wrong password, retry, protected return, external-redirect rejection, logout, work item creation and restart persistence, and recorded negative HTTP probes separately. It loaded the production login page once and reports no account creation or data change there.

The observer independently confirmed: PR states and merge commits via `gh pr view`; remote and local `develop` heads at each checkpoint; `aep check` pass at each checkpoint; review, check, delivery, story and change statuses in the YAML; both attempt worktrees removed at turn end; working tree clean apart from four untracked files that predate the session. The observer did not rerun tests or the browser journey; the 40-test and 12-task typecheck claims come from retained logs (`integrated-tests.txt`, `integrated-types.txt`) and were not reproduced.

## Direct pushes and secret scanning

Three commits went to `origin/develop` without a PR: `23b437ff` and `d93e30c3` (records, design doc, `.gitleaks.toml`), `00753227` (records after BASIC-001), `9548f3a0` (60 files of records and evidence after BASIC-002). Code changes went through PRs; ledger and docs commits did not. The project rules the agent read (`project-rules/commits.md`) were not checked by the observer for a rule on this.

gitleaks flagged v5 record digest fields as generic API keys. The agent added `.gitleaks.toml` allowlists per exact digest and path (commit `d93e30c3`), then ran staged scans before every commit. Any downstream project with gitleaks in its commit path will hit the same false positives on `fingerprint`, `binary_sha256`, `guidance_digest` and migration source digests.

## Production issues carried, not closed

The agent's diagnosis, not verified by the observer: the deploy failure on `eebcd25d` is Cloudflare 10021, Worker startup CPU limit, distinct from the intermittent Supervisor RPC 503. It profiled a dry-run bundle locally with `wrangler check startup`, recorded that local profiling does not establish a fix, and filed `FIX-DEPLOY-STARTUP-CPU-001`. The 503 investigation is parked on missing Cloudflare log access; `wrangler whoami` reports unauthenticated and the agent asked the user to log in. Its final message says the bugs are not all closed and nothing was deployed. That matches the records.

## Against the open v5 items

Read after the turn against the branch state at `3abffb1` (installed binary `3fae0e42…`, built 2026-09-11 14:31, the same digest the project-b records carry as `producer.binary_sha256`).

- **Layer encapsulation (accepted 2026-09-11).** Without steering, the agent created layer `basic-user-access` with a design artifact before its decision, changes and stories, and set `layer:` on all three stories. That is the second downstream project to follow the new guidance unprompted. It did not create waves, which the decision says is the expected default for a small layer. It left the layer's `refs` empty. Observer check: `aep context basic-user-access --json` returns 200 records, including 43 imported layers, 38 waves and 41 gates reached through the decision's refs, but none of the three member stories and neither change. `aep context BASIC-002` does reach the layer, the decision, the change and BASIC-001. The container-to-member edge is therefore only discoverable from the member side. `aep check --json` passed with no diagnostics. This is the exact case behind the pending candidate "`aep check` diagnostic for a layer/wave without members" in the layer/wave decision, now observed in a second project.
- **Container context over-expansion (open observation from Project A).** Recurs: the layer's context expands through one decision into 74 stories and the whole imported layer/wave/gate history.
- **`aep context` source loading (recurring gap).** `sources: []` for both the layer and the story; the design artifact path sits in `data` and its content is not loaded. Same as every prior observation.
- **Delivery continuity and closure (accepted 2026-09-11).** Confirmed structurally in a project other than Project A: implement → validate → deliver → close ran in one turn for two stories, with `deliver plan`, `deliver pr`, `deliver merge`, `spec publish` and `change close` each leaving records that bind heads, fingerprints and the executable digest.
- **Review evidence enforcement (accepted 2026-09-11 corrections).** The three CLI rejections above are the enforcement working on real reviewer output.
- **gitleaks on digest fields (lesson 2026-09-10 §7).** Already recorded for migration receipts. Today it recurred on fresh native records (`fingerprint`, `binary_sha256`, `guidance_digest`) with no migration involved, so the exposure is ongoing, not a one-time migration cost.
- **Research/draft context (proposal).** Not exercised; the agent went straight from direction to layer, decision and change.

## Second turn: production promotion (observed 21:16 to 21:41)

The user told the agent there are no real customers yet and it could deploy to `main` and verify the earlier scenarios. The user also answered the earlier environment question in favor of the production site. The observer tracked two things in particular at the user's request: whether the layer's `refs` get filled, and what happens to `FIX-DEPLOY-STARTUP-CPU-001`.

Observed sequence, verified against GitHub and the ledger: `aep --skill deliver`, the project's `verify-project-b` skill and e2e-test policy read; a pre-promotion independent review retained as evidence; PR #365 `develop → main` opened and merged at 13:16Z by `gh pr merge --match-head-commit`; `aep release new` created `basic-access-production-20260915` (pending, `authorized_by: owner-request-20260915`, candidate 9548f3a0, refs BASIC-001, BASIC-002, FIX-DEPLOY-STARTUP-CPU-001, FIX-L41-SUPERVISOR-RPC-503-002). CI Deploy run 34973936817 succeeded; the agent recorded 11 canary cases passing and that neither the startup CPU rejection nor the RPC 503 reproduced, adding that one run does not establish a cause. It ran the existing production preflight workflow in plan-only mode, downloaded and digest-checked the artifacts, then drove project-b.run with the fixed test account: 14 scenarios recorded PASS in `journey.md`, one PENDING. The pending one is fresh signup, because the fixed account exists and rebuilding it deletes data; the agent asked the user for authorization and cited the e2e-test rule requiring separate authorization for destructive Cloudflare operations. That question was still open at turn end. An independent reviewer subagent re-ran login and logout on production. Evidence was committed as `bb2f11b0` (21 files) and `d0c21cb9`, both pushed directly to `develop`; the production worktree was removed.

Focus results:

- **Layer `refs` never changed.** `basic-user-access` still has `refs: []` and its original `updated_at`. The release record has `layer: null`. The layer's observable outcome now has production evidence and no record on the layer side points to it. Second confirmation in this project of the empty-container case.
- **`FIX-DEPLOY-STARTUP-CPU-001` never changed.** Still `pending`, no change contract, `updated_at` from 12:27Z, and its `data.unknowns` still lists "CI deployment verification" although that deployment has since succeeded. The contrary evidence lives only in the release record's `data` (`deploy_status: success`, `canary_status: pass`) and in `journey.md`. `aep dispatch plan` for the story reports one readiness reason: an accepted change contract is required. The agent did not touch the story, did not close it, and did not create a change for it. Structurally correct, since one success is not a fix, but a reader of the story alone cannot see that its main unknown was exercised.
- **Release record behavior.** `release new` then one `release update` moved the record pending → pending with `data` gaining deploy run, deployed SHA, canary and preflight results and per-journey outcomes. No `release promote`. The release links the two fix stories and the two delivered stories; it is the only record that ties the deployment evidence to them.
- **gitleaks, third recurrence.** The two event records carrying the release's revision digests were flagged; two more exact allowlist entries were added to `.gitleaks.toml`. Every native write that produces an event with a revision digest now costs one allowlist entry in this project.
- **Direct pushes.** Four commits to `origin/develop` without a PR during the session, all records or evidence. Code and the promotion went through PRs #363, #364, #365.

## Third and fourth turns: signup check and authorized reset (2026-09-16, 01:12 to 01:31)

After about three hours idle, the user asked whether the interface currently allows signup. The agent loaded `aep --skill validate`, opened the production login page in a browser, clicked Sign Up, confirmed the name/email/password form, submitted nothing, and answered yes. It also clarified that its pending question was about rebuilding the test account, not about signup being closed. Turn length under a minute.

The user then confirmed the rebuild and noted Cloudflare is not set up on this machine. The agent loaded `aep --skill deliver`, read the reset workflow, and used the project's existing `production-test-tenant-reset` GitHub Actions workflow instead of local credentials: one plan run (35000668258) to re-verify the deletion scope against the plan the user had seen, then one execute run (35000829325). Both succeeded on GitHub. It recorded zero remaining rows for the account and untouched R2. It then completed fresh signup on production through the real form, verified default company and agent creation, task creation, logout returning 401 on a protected API, and re-login with data retained. An independent reviewer subagent re-checked the new account on production. Evidence (14 files) was committed as `585f2543` and pushed directly to `develop`; the production worktree was removed. `.gitleaks.toml` gained more entries (fourth recurrence). `aep check` passes on 859 records.

The release record was updated again, still `pending`: `fresh_signup: pass`, `fresh_signup_reset_authorized_by: owner-confirmation-20260916`, both run IDs, the new user ID and the evidence path. No `release promote` at any point across the session.

Focus results after four turns: `basic-user-access` still has `refs: []` and its creation timestamp; `FIX-DEPLOY-STARTUP-CPU-001` is still `pending` with no change and the stale "CI deployment verification" unknown. Every piece of production outcome evidence for this layer now lives in the release record's `data` and in evidence files, and nothing on the layer or the fix story points at it.

Destructive-operation boundary: the agent waited roughly four hours for explicit user confirmation before deleting production test data, cited the e2e-test rule requiring separate authorization, and executed through the CI path the project already had. The record of that authorization is the release's `data` field, attributed to the user's chat confirmation.

## Fifth turn: first production model execution (2026-09-16, 11:40 to 11:56)

The user asked the agent to register through agent-browser and set a company goal following the earlier E2E scenarios. The agent read the e2e-test journeys and the project's `verify-project-b` skill, asked which goal to try, offered the E2E "company status summary" as the default, and proceeded with it without waiting for an answer. It scoped the experiment itself: reuse the account created in the previous turn, one attended execution, wiki write only, no email, no hiring, autonomous loop left disabled.

It ran the reset workflow in plan-only mode (run 35053093500) to obtain validated preflight receipts, then drove production through the browser: set the Mission by chat, confirmed the northStar API and the Mission page showed the same statement, sent one attended instruction to inventory agents and tasks and write `/company/wiki/company-status.md`, saw the Write and Read tool cards complete, and read the actual file back through the company knowledge export endpoint. It recorded a verification-route correction: the agent workspace file inspector returned not found because the wiki is a company mount, so it treated that negative as a route error rather than a missing artifact. Final loop state `enabled=false`, `ticksUsed=0`. An independent reviewer subagent checked the persisted conversation and exported file.

Records: `aep lesson record` created lesson `company-wiki-verification-surface` (pending, paths to the journey evidence, `refs: []`). No release update, no story, no change. Evidence (18 files) committed as `f3734688` and pushed directly to `develop` (sixth direct push). `aep check` passes on 861 records.

This is the first turn in the session that invoked the product's model-backed execution on production. The agent did not stop for spend authorization; it bounded the run to one execution and recorded the loop as disabled. Whether one attended execution needed separate authorization is a project policy question; the ledger holds no record of the authorization boundary for it beyond the journey prose.

Focus results: layer `refs` and `FIX-DEPLOY-STARTUP-CPU-001` unchanged after five turns. The new lesson also has empty `refs`, so it is reachable only through its path, not from the layer, the release, or a story.

## Sixth turn: planning after production evidence (2026-09-16, 18:47)

The user asked what to do next. The agent ran `aep status`, `aep --skill roadmap`, `aep --skill roadmap --ref status`, and read the imported product direction and the autonomous-loop E2E journey. It recommended, in order: the complete create-task → execute → view-results user flow from the interface, then single-employee delegation, then bounded autonomous execution with pause, budget and duplicate checks. It said it would start on the first item using the current company and fix blocking bugs as found, and repeated that the 503 root cause is unconfirmed.

Against the status guidance: the handoff was organized by user journeys, not by container. It did not name `basic-user-access`, did not state the layer's member status or outcome, and did not mention `FIX-DEPLOY-STARTUP-CPU-001` at all, although that story sits in the layer and its main unknown was exercised by the successful deploy. `aep status` still reports 0 ready to start. Neither focus record changed. No file was written this turn.

Reading: with the layer's `refs` empty and `aep status` not grouped by layer, nothing in the CLI output pushed the agent toward reporting by container, and the guidance sentence alone did not produce it. This is the third project-level observation supporting the two pending CLI candidates in the layer/wave decision.

## Seventh turn: story list on request (2026-09-16, 19:20)

The user clarified they wanted the remaining stories, not operating advice. The agent queried stories by pending, ready and blocked status and answered with a table: BASIC-001 and BASIC-002 done; then FIX-L41-SUPERVISOR-RPC-503-002 (root cause unconfirmed, recommended first), FIX-DEPLOY-STARTUP-CPU-001 ("pending; latest deploy succeeded but the root cause is not proven fixed"), FIX-L41-DEPLOY-REGISTRY-AUTH-001, L41-005 (awaiting Browser Run eligibility decision), L41-006b (awaiting the user's source decision), L41-006c (after 006b). No file was written.

The FIX-DEPLOY-STARTUP-CPU-001 line is accurate and reflects the deploy evidence the agent holds in the release record, so the agent knows the story's state even though the story record itself does not. The list is flat by story ID with no layer grouping; `basic-user-access` is not named. `aep status` still says 0 ready to start, and the answer did not explain why the ready-labelled stories cannot be dispatched.

## Eighth turn: scoped 503 diagnosis without the story lifecycle (2026-09-16, 19:42 to 19:53)

The user narrowed scope twice: "only L41", then "only the 503 fix". Baseline recorded by the observer before the turn: `FIX-L41-SUPERVISOR-RPC-503-002` is labelled `ready` but `aep dispatch plan` lists 84 reasons it cannot start, beginning with its direct dependency `FIX-L41-SUPERVISOR-RPC-503-001` (status `imported`, no completion evidence) and the whole imported gate chain from `gate-layer-39-41` down to `gate-layer-26-29`, all pending.

The agent loaded `aep context` for the story, the design and implement skills and their records/git references, and the existing 503 lesson. It did not run `dispatch start`, create an attempt, change or worktree, or touch the story record. It started `wrangler login` in the background and asked the user to complete the browser authorization; the login timed out unanswered. It established from Cloudflare documentation that the original 2026-08-06 event is past the 7-day Workers Logs retention, ran the 61 driver and canary tests (all passing, none reproducing the rejection), and wrote a diagnosis: the failure is an awaited `session.exec` rejection on the capnweb `ContainerControlClient` path of `@cloudflare/sandbox@0.12.4`; `operationMs=21116` includes post-rejection cleanup with a 15 s destroy bound, so it cannot be read as supervisor wait time; `start_not_started` and `cleanupFailed=false` are weaker than their labels; no retry, backoff, timeout or status-remap change is justified; the next evidence is the SDK's `sandbox.exec` event and typed error code. An independent reviewer subagent contributed findings and was interrupted twice; the retained findings file states it is not a completed review and approves nothing.

Two commits, `ffa53aab` and `e2616223`, went directly to `develop` (seventh and eighth direct pushes) carrying only evidence files. `aep check` passes on 861 records. The story is unchanged: `ready`, no change, no attempt, `updated_at: null`. The final message asks the user to run `bunx --no-install wrangler login` in the project terminal.

Reading for v5: real investigative work on a story blocked by 84 imported dependency reasons happened entirely outside the story lifecycle. Nothing in the ledger links the story to the diagnosis directory except the directory name. Whether the agent avoided `dispatch start` because of the readiness wall is not stated in the transcript; the outcome is that the imported gate chain, which the 2026-09-10 pilot audit already flagged as a usability finding, now coincides with a story being worked without records.

## Ninth turn: sibling story around the readiness wall (2026-09-16, 20:16 onward)

The user said "continue". `wrangler whoami` still reported not authenticated. The agent restarted the login in the background and, rather than waiting, changed approach: it loaded `aep --skill design --ref bdd` and `--ref records`, read BASIC-002 and its change as templates, read `--help` for `change`, `story`, `config`, `dispatch start` and `decision`, started a reviewer subagent on the proposed contract, and then created:

- change `supervisor-rpc-diagnostic-channel` with a spec (two requirements: a fixed-schema private rejection event with separated RPC and cleanup timings and a closed SDK error category; a bounded read-only CI workflow that queries Workers Observability with existing credentials). Accepted with `--by owner-scoped-503-continuation-20260916`, `acceptance_class: attributed_decision`. Refs the original story.
- story `FIX-L41-SUPERVISOR-RPC-503-DIAGNOSTICS` with `depends_on: []`, `layer: null`, `risk: deep`, `refs: [FIX-L41-SUPERVISOR-RPC-503-002]`, eight paths including a new driver diagnostics module, a diagnostics script, and a new workflow file. Its description says it does not replace or reset the original failure lineage and that the original fix and imported gates remain unresolved.
- two required checks added to `.aep/config.toml`: `supervisor-diagnostic-tests` (four test files, 300 s) and `supervisor-diagnostic-types` (`bun run check-types`, 600 s), both `required = true` and path-scoped.

Commit `74f96b8b` holds these records and the config change (not yet pushed at inspection). `aep dispatch start` then created attempt `attempt-18d5cc6f80610f38-be902-0` in a worktree on branch `aep/FIX-L41-SUPERVISOR-RPC-503-DIAGNOSTICS-attempt-…`; the story is `in_progress`; `aep check` passes on 869 records.

Reading for v5: the original story stays untouched at `ready` with 84 readiness failures, and the agent routed real work through a new sibling story that has no dependencies and no layer, so it passed readiness immediately. The lineage is preserved only through `refs`. The CLI enforced nothing about this: a story with an empty `depends_on` and no container is dispatchable regardless of what it references. Whether this is the intended escape hatch for imported dependency walls, or a gap the empty-container and dependency-inheritance rules should cover, is a decision for the AEP owner. The new story also sits outside any layer, so the `basic-user-access` observation now has a second instance in the L41 area: work without a container.

### Ninth turn, continued: delivery, unrequested promotion, and two failures (20:27 to 20:40)

In the attempt worktree the agent committed `9d0dced5` (8 files, 907 insertions: a 52-line driver change emitting a fixed-schema rejection event, a diagnostics module and tests, a bounded log-query script and tests, a new `supervisor-rpc-diagnostics.yml` workflow, 19 lines in `deploy.yml`). It ran the tests (75 pass), formatted, installed and ran actionlint on both workflows, moved the attempt to review, requested review, ran `verify run` (three checks pass: secret-scan and the two checks it had just added to config), recorded the review as pass, `deliver pr` → PR #366 (base develop), `deliver merge` → merged 12:33Z as `e2904926`. Story `integrated`, attempt `done`, change `closed`; 885 records, check pass. Records committed as `29627721` and pushed to `develop` (tenth direct push).

It then opened PR #367 `develop → main` at 12:34:30Z and merged it at 12:35:43Z with `--match-head-commit`, without asking the user and without an `aep release` record. The authorization it relied on was the previous day's "no real customers, deploy to main" for the account journey. The promoted content included the driver change and `deploy.yml`.

Outcomes, verified on GitHub:

- Deploy run 35096748362 on `bf0103a8` **failed** at "Verify professional admission manifest": `unreviewed sensitive callsite packages/do-agent/src/execution/sandbox-driver.ts:1403:13:start:writeFile`. The new diagnostics code shifted line numbers of an existing sensitive call and the project's reviewed-callsite manifest was not updated. Production was not updated by this deploy. None of the agent's checks (its two new config checks, secret scan, actionlint, review) covered the project's own admission-manifest check, which only runs in CI.
- Diagnostics workflow run 35096765224 (dispatched by the agent with a 168-hour lookback) **failed**: Cloudflare returned permission-denied; the existing CI token lacks the Workers Observability permission the query API needs. The agent retained the denial as evidence and asked the user to choose between local Wrangler login (its recommendation) and widening the CI token.

The agent's own diagnosis of the deploy failure matched the log: line movement, manifest not synced, no change to the guarded logic. It said it would update the manifest, re-verify, and push a fix. Four questions were queued at this point.

Reading for v5: the sibling-story route produced a complete, well-evidenced native delivery, and the promotion to production then failed on a project check that lives outside `.aep/config.toml`. The agent added two checks to config for its own tests but did not add the admission-manifest check, which the project's CI treats as a deploy gate. `aep verify run` therefore reported a clean candidate that CI rejected. Separately, the second production promotion in this session has no release record, unlike the first.

### Tenth stretch: the CI gate folded into config (20:37 to 20:41)

Correction went through the same native route, with one structural improvement. The agent created story `FIX-L41-SUPERVISOR-RPC-MANIFEST-SYNC` (`depends_on: []`, `layer: null`, `risk: deep`, refs the diagnostics story) and change `supervisor-rpc-manifest-sync` (accepted `--by owner-authorized-503-delivery-correction`). Using `aep config update --expect <revision>` it added a third required, path-scoped check, `supervisor-admission-manifest`, running the project's `check:professional-admission-manifest` script. The new story requires that check and the diagnostics tests. Records commit `e3cc4a4e`.

In the attempt worktree it regenerated the callsite manifest: one line changed (1390 → 1403), 218 sensitive callsites verified unchanged. Attempt → review → review request → review pass → `deliver pr` (#368) → `deliver merge` (merged 12:40:28Z as `91fd20c2`) → spec publish → change close. Check evidence for `supervisor-admission-manifest` is `pass`. Records commit `7f3c13e9` pushed to `develop`. `aep check` passes on 911 records. The agent stated it will re-run the production deployment with this version and keep the failed-deployment record; log access still needs a Cloudflare permission decision from the user.

Reading for v5: the reflect loop closed inside one turn. A CI gate outside `.aep/config.toml` rejected a native-verified candidate; the agent registered that gate as a required config check before the corrective story, so `aep verify run` now covers it. This is the behavior the validate and reflect guidance describes, produced without steering. It does not change the container finding: three L41 stories were created today with no layer and no dependencies.

### Ninth turn close: verified production deployment (20:41 to 20:58)

PR #369 `develop → main` opened 12:41:09Z, merged 12:42:06Z as `6746a92b`, again without asking. Deploy run 35097364438 passed the admission-manifest step, deployed, and the full Sandbox canary succeeded. The agent waited with `gh run watch`, downloaded the artifacts, verified the canary receipt SHA is `6746a92b`, 11 scenarios passed, 7 unique cleanup receipts, and had the reviewer subagent confirm receipt, deployment proof and SBOM hashes. It confirmed the log-query workflow and the deploy use the same CI credentials with no environment override, so the permission denial is a token-scope gap, not a configuration mistake. It removed both attempt worktrees and deleted the attempt branches locally and on origin. A new lesson `sensitive-callsite-coordinate-check` was recorded through reflect. Evidence committed as `befefe3f` and pushed to `develop` (twelfth direct push). `aep check` passes on 913 records; `aep status` reports 421 stories, 0 ready to start.

Final message: the 503 diagnostic reinforcement and the deployment check correction are live on `main` at `6746a92b`; the original 503 is not closed because the Cloudflare log query returned permission-denied; the next step needs either a local Wrangler login or a CI token with log-query permission. Four questions remain queued.

End-of-session ledger state for the stories touched today:

| Story                                  | Status                        | Layer             | Note                                                                              |
| -------------------------------------- | ----------------------------- | ----------------- | --------------------------------------------------------------------------------- |
| BASIC-001, BASIC-002                   | integrated                    | basic-user-access | delivered 09-15; production-verified 09-15/16                                     |
| FIX-DEPLOY-STARTUP-CPU-001             | pending                       | basic-user-access | never updated; deploy succeeded three times since                                 |
| FIX-L41-SUPERVISOR-RPC-503-002         | ready (84 readiness failures) | layer-41          | never updated; worked around                                                      |
| FIX-L41-SUPERVISOR-RPC-503-DIAGNOSTICS | integrated                    | none              | sibling story, PR #366, promoted by #367 (deploy failed) and #369 (deploy passed) |
| FIX-L41-SUPERVISOR-RPC-MANIFEST-SYNC   | integrated                    | none              | correction story, PR #368                                                         |

`project-ledger/releases/` still holds only `basic-access-production-20260915`. The two promotions on 09-16 (`bf0103a8` failed, `6746a92b` succeeded) have no release record; their evidence lives under `project-ledger/evidence/supervisor-rpc-diagnostic-channel-20260916/`. `basic-user-access` still has `refs: []`.

Session totals from the observer's bundle: five PRs merged (#365 to #369), twelve direct pushes to `develop` carrying records and evidence, three sibling or correction stories created without layer or dependencies, four gitleaks allowlist rounds, one CLI-rejected review record set, one CI gate folded into config after it rejected a native-verified candidate.

## Rule audit: ledger and lesson recording against the guidance (2026-09-16, 21:10)

Checked after the user asked whether the agent recorded the ledger and lessons according to the rules. Sources compared: `aep --skill reflect` and `--ref feedback`, `aep --skill deliver --ref closure`, `aep --skill design --ref records`, project-b `project-rules/memory.md`, `commits.md`, `preview-operating-policy.md`, and the records on disk.

**Compliant.**

- Every delivered story (BASIC-001, BASIC-002, DIAGNOSTICS, MANIFEST-SYNC) has the full native chain: change with spec, story, attempt, review request and recorded review, `verify run` check evidence (18 check records in total), delivery with heads and fingerprints, spec publication, change close. All written through CLI commands, none by hand.
- Four lessons exist as native records created with `aep lesson record`, each in the configured `lesson-learned` store, each with `paths` to retained evidence, each describing one specific mechanism rather than a ritual entry. Three of the four carry `refs` to the story they came from, which is what `memory.md` asks for ("linking evidence and the actual task"). `aep lesson find` locates them.
- Every loose evidence directory under `project-ledger/evidence/` is referenced from at least one native record (a release, a story's `data`, or a lesson's `paths`). Nothing is orphaned.
- Code integrated into `develop` through squash PRs and production through explicit develop-to-main merge-commit PRs, as `commits.md` requires. Records commits went to `develop` directly; `commits.md` does not say records need a PR.
- Reflect was run at the end of each delivery, and the deploy-gate lesson turned into a config check in the same turn.
- Deployed-SHA evidence and canary receipts were retained for both production deployments, as `preview-operating-policy.md` requires.

**Not compliant or omitted.**

- Layer `basic-user-access` has `refs: []`. The records reference's minimal layer input shows `refs: [S-42, S-43, ADR-42]`, and Project A filled it. Consequence measured here: `aep context basic-user-access` returns 200 records and none of its three members or two changes. The guidance sentence "`aep context <layer>` expands to members" is only true when `refs` is filled.
- The 503 work became three stories around one concept (502, DIAGNOSTICS, MANIFEST-SYNC) with no layer for the two new ones. The records reference says to create the layer when a design breaks into several stories. The agent created sibling stories with `layer: null` and `depends_on: []` instead.
- Stories `FIX-DEPLOY-STARTUP-CPU-001` and `FIX-L41-SUPERVISOR-RPC-503-002` were never updated although contrary evidence accumulated. The records reference gives the mechanism for this: a decision with `refs` to the affected story, accepted under the user's authority. No such decision was created; the evidence sits in release `data`, `diagnosis.md`, and lesson prose.
- Only the first production promotion has a release record. The two promotions on 09-16 (`bf0103a8` failed, `6746a92b` succeeded) have none, so no native record binds the deployed SHA to the delivered stories; the SHA lives in loose files. The records reference says release records "can" select stories, so this is an inconsistency rather than a rule breach, but the project policy's production-evidence expectations are met only by files.
- Lesson `company-wiki-verification-surface` has `refs: []`; it links evidence but not the task.

**CLI behavior observed while auditing, not agent behavior.**

- `aep context <story>` does not include lessons whose `refs` name that story, and `aep context <layer>` does not include stories whose `layer` field names that layer. Reverse edges are not followed. Lessons are therefore reachable only through `aep query --kind lesson`, `aep lesson find`, or the path, which weakens `memory.md`'s "recall applicable lessons during context assembly" unless the agent queries lessons separately. The agent did read a legacy lesson file by path in the 503 turn.
- There is no native record kind for free-form evidence beyond `check-*`; the agent's loose evidence directories are the only available shape. `aep check` accepts them silently.
- `sources: []` on every context query; design artifacts and evidence paths in `data` and `paths` are never loaded.

## Tenth turn: discussion with no landing record (2026-09-17, 11:35)

The user opened a cross-project question: the main projects manage Cloudflare resources with Alchemy and need per-project profiles for different accounts plus shared-resource management, "let's discuss how to manage this first". The agent said it would discuss without changing settings, read the project's Alchemy config, searched the Alchemy and Cloudflare documentation, and answered in under two minutes with two project-specific findings (project-b pins Alchemy 0.93.9 whose profile commands differ from the current release; CI uses remote state while local uses local state, so account management must define state ownership) and a four-layer principle (CF account, profile, project+environment, resource owner), a "one owner, many consumers" model for shared infrastructure, and the platform limit that Worker service bindings cannot cross accounts. It ended with two questions back to the user (which projects on which accounts; whether shared resources mean DNS only or also R2, D1, Queues, shared Workers).

No file, record, decision, draft or lesson was written; the working tree stayed clean. That is correct for an undecided discussion under current guidance, and it is the exact gap the research/draft context proposal describes: real research and a reusable cross-project principle now exist only in this pane's scrollback. `aep --skill roadmap` was loaded at the start of the turn but no purpose-driven research record or design draft followed.

## CLI change implemented and installed from these observations (2026-09-17, 14:10 to 14:23)

At the user's request the reverse-edge and empty-container findings were turned into [a decision](../decisions/aep-v5-context-reverse-edges.md), implemented on `feat/aep-5-native-cli` as commit `9bc3686`, and installed as the shared `aep`.

- Build `71a66621…` (`~/.local/share/aep/builds/71a66621575e9709/bin/aep`, manifest beside it) replaced `3fae0e42…`; the previous manifest records the replacement. Version string unchanged, `5.0.0-preview.1`.
- Checks before install: fmt, clippy, 66 workspace tests including a new lifecycle fixture, skills steering and generated-copy checks, `verify-native-preview.py` passed.
- A first build followed incoming edges transitively and expanded every project-b context to 690 of 913 records; the installed build follows them for the subject only.
- After install, read-only on the three downstream projects: project-b `aep check` PASS 913 records, 0 warnings; `aep context basic-user-access` 205 records now including BASIC-001, BASIC-002, FIX-DEPLOY-STARTUP-CPU-001 and both changes; `aep context BASIC-001` includes its lesson and the release. Project A PASS 540 records, 0 warnings, `layer-20` context lists all nine members. Project C PASS 382 records, 0 warnings. No migrated container produced an `empty_container` warning, so the advisory is quiet on the current projects.
- Outputs are retained in the trial bundle as `preinstall-0475eae7-verification.txt`, `preinstall-71a66621-verification.txt`, `postinstall-71a66621-project-b.txt` and `postinstall-71a66621-project-a-project-c.txt`.

Project A had an active attempt at the switch; the running agent was not prompted. Whether project-b's next planning turn reports by container with the members now visible is the next observation.

## First delivery on the new build (2026-09-17, 14:24 to 14:52)

The user asked the agent to set up project-b's Cloudflare login per the discussed design and to do this project first. The turn ran entirely on build `71a66621…` and loaded `aep --skill design --ref records`, `implement`, `validate`, `deliver`, `reflect` and their references.

Native chain, verified: change `project-b-cloudflare-context` with spec (accepted), single story `OPS-CLOUDFLARE-CONTEXT-001` (`layer: null`, which is consistent with the guidance since it is one story), config checks added, attempt in a worktree, review round 1 with two real findings (a release path acquiring the same concurrency lock twice; a log window outside the backend's supported 1 or 24 hours) fixed and superseded, round 2 pass, PR #370 merged to develop 06:37Z, spec published, change closed, post-integration `verify run` and a round-3 review request on the integrated tree. Promotion PR #371 opened 06:38:55Z and merged 06:39:15Z without asking; Deploy run 35190784164: account guard pass, Alchemy deploy success, 11 canary cases pass, 7 cleanup receipts. `aep check` PASS on 955 records, 0 warnings.

Difference from the previous day: the agent created release record `cloudflare-context-production-20260917` before the deployment finished (`canary_status: in_progress`, `local_oauth: pending-human-authorization`) and updated it afterwards with the deployed SHA, run id, canary and audit results and the evidence path. The two promotions on 09-16 had no release record; this one matches the sentence added to the deliver closure reference on the same day. One instance, so attribution to the guidance is plausible rather than established.

The agent also verified profile isolation by running real Wrangler from a directory containing another project's `.env` and confirming only the project-b profile was selected, and it printed the OAuth authorization link for the user with an explicit note that the granted scope includes `workers:write` even though the profile is named for observation. Five questions were queued at the end.

## Permissions follow-up on the new build (2026-09-17, 15:00 to 16:20)

Three short turns and one delivery. The user completed the Wrangler OAuth over an SSH port forward (the agent had explained the forward and the Tailscale variant when asked). Verification: profile `project-b-production-observe` logged in, account and deploy info readable, historical logs still permission-denied. The user then said to open all permissions for this workstation. The agent re-ran Wrangler OAuth with every optional scope (28 granted, user-approved through a fresh link), confirmed logs are still denied, and identified the cause: Wrangler's optional scopes do not include `workers_observability_telemetry:write`, which the pinned Alchemy version does support. It started an Alchemy named profile (`project-b-production-admin`, via `configure` since it is a first setup) and asked the user for a second forward on port 9976, stopping before submitting scopes.

Native chain for the login-entry change: change `project-b-full-oauth` (accepted `--by owner-full-permissions-request-20260917`), story `OPS-CLOUDFLARE-OAUTH-002` (`layer: null`, `depends_on: []`), attempt, tests, review pass, PR #372 merged to develop 08:12Z, spec published, change closed. `aep check` PASS on 978 records, 0 warnings.

Two observations for v5:

- The Cloudflare concept now has two stories (`OPS-CLOUDFLARE-CONTEXT-001`, `OPS-CLOUDFLARE-OAUTH-002`) and no layer. The `empty_container` warning installed today cannot fire because no container exists; this is the first instance of the second candidate in the layer/wave decision (several stories under one concept with no container), the one deferred there for false-positive risk.
- Commit `d513084f` added five more per-value gitleaks allowlist entries for event revision digests. The verification-setup sentence added today suggests path-and-field scoping, but project-b's own migration rule (2026-09-10 lesson §7) requires exact value plus exact path exceptions with no generic hash exemption, so the project policy wins and the guidance sentence cannot change this here. The friction is therefore a project-policy cost of the record format, not something guidance resolves.

## Build superseded and re-baselined by the agent (2026-09-17, 17:44 to 17:50)

Another session built commit `8e8a79c` ("record the reason for a write with `--note`") on top of `9bc3686` and installed it as `11186c26…` at 17:44 using the same staging and symlink procedure; the `71a66621…` manifest records the replacement. The reverse-edge behavior, the `empty_container` advisory and the three guidance sentences are all present in the installed build (verified from project-b: layer context 205 records with all members, `aep check` PASS on 982 records, 0 warnings).

The user then told the project-b agent that the tool had been updated and to re-read every skill. The agent re-read `aep --skill`, all eight native skills and the project's `verify-project-b`, ran `aep doctor` and `aep check`, and reported the version string unchanged but the executable hash changed. Its summary did not name any specific guidance difference and it took no action on the two OPS stories without a layer or on the release record whose `retained_logs` field still says permission-denied after Alchemy log access succeeded. The Alchemy result (71 scopes, three successful log queries) is recorded in lesson `cloudflare-oauth-client-scope-differences` and the verification evidence, uncommitted at inspection.

## v4 removed from project-b by another session (2026-09-18, 23:05 to 23:07)

While the pane `w5:p1` agent stayed idle for about 29 hours after the skill re-baseline, a different session landed three AEP pull requests on the branch (#37 minimal `AGENTS.md` on fresh init, #38 migration-cleanliness review and proposals, #39 post-migration cleanup of the v4 route and legacy skills), built `dc7aaf75…` from `abfd7b3` and installed it at 23:05 as the shared `aep`, then ran the cleanup on project-b under the user's instruction "直接幫我移除 v4 簡化我們的 AGENTS.md". project-b commit `77cb2f8b` (amended from `6aa912ed`) removes 48 legacy `aep-*` skill directories under `.agents/skills` and `.claude/skills` plus `skills-lock.json`, 449 files and 65,470 lines, and reduces `AGENTS.md` to one sentence pointing at `aep --skill` and `project-rules/README.md`. It records decision `aep-v4-removal` (accepted, `attributed_decision`, `accepted_by: memorysaver`, `authority` quoting the instruction, `cli_build: dc7aaf75e0028de9`, `recovery: git history`) and states that legacy sources named in the migration receipt and host hook settings remain untouched until a cleanup receipt exists. Not pushed at inspection; the three uncommitted Cloudflare files from 09-17 are still present.

Verified with the installed `dc7aaf75…` on project-b: `aep check` PASS on 986 records, 0 warnings; `aep context basic-user-access` still lists its three members; `aep context BASIC-001` still includes its lesson and release; the deliver closure sentence is present. All six commits from this session are ancestors of the AEP HEAD. Project procedures visible through `aep --skill` after the removal: e2e-test, memory-forge, project-behavior, project-memory, verify-project-b.

One linking observation in the same shape as before: `aep-v4-removal` has `refs: []` and `paths: []`, so nothing connects it to the migration import record or the receipt whose retained sources it mentions; it is reachable only by ID.

### Follow-up commits (2026-09-18, 23:23)

Two more Project B commits from the same session, both pushed to `develop`: `17a4f3e0` retained the 09-17 session's uncommitted work (the Cloudflare setup doc and lesson edits, the Alchemy auth verification JSON) together with the four untracked files that had predated this observation since 09-15 (host settings, the CI workflow draft, two setup handoff docs). `548177f6` added decision `aep-v4-removal-context` (accepted) whose `refs` name `aep-v4-removal` and the migration import record `migration-c4782ca44fee809a`, describing itself as a context correction so the removal stays tied to the receipt whose sources it left in place. `aep context aep-v4-removal` now reaches the import record through the incoming accepted decision. This is the clarification mechanism the records reference describes, used correctly, and it closes the linking gap noted above. `aep check` PASS on 989 records, 0 warnings; the working tree is clean for the first time since the observation began.

## Herdr observation note

`herdr agent get` reported `blocked` from the first queued Codex question until after the turn ended, while revision advanced from 43 to over 1300 and two PRs merged. `agent wait --until idle|done|working` timed out twice. Switching to `pane wait-output --regex` on CLI milestones (`attempt record --status review`, `deliver pr|merge`, `change close`, `git commit`, `worktree remove`) gave reliable triggers. Codex reported one context compaction mid-turn; the records before and after are consistent.

## Handoff and retained state

Final agent message: basic account flow usable locally and merged to `develop`; start with `bun run dev:local` and open `http://localhost:3001/login`; production 503 and startup CPU remain open. Local account data was copied from the attempt worktree to the root checkout's ignored `.wrangler/basic-local/` with `.dev.vars` at mode 600. Four `aep/*` attempt branches remain locally and on origin. Two queued questions remain unanswered in the pane.

Nothing in this turn showed a structural failure. Items for the user's judgment: attributed acceptance from a single prompt, direct ledger pushes to `develop`, and the gitleaks friction on digest fields.
