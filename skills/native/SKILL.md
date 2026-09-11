---
name: aep
description: Use the AEP CLI to build project context, manage scoped work, and verify actual results in projects that use AEP. Read this when operating AEP or following a project's AEP entrypoint.
---

# AEP

AEP keeps project context, work records and verification evidence in the repository. The agent chooses the relevant procedure and work order from the user's request. The CLI maintains records and checks declared constraints; it does not decide product intent or run a model.

## Start with the project

Read `AGENTS.md`, the project README and the indexed project rules. Honor the selected workflow and the user's scope. `.aep/config.toml` identifies the native stores, executable checks and review policy. Use `aep doctor` to inspect setup and `aep --help` for commands; use `<command> --help` for exact arguments.

```bash
aep status
aep context STORY-ID
aep --skill project
```

Build enough context to identify intent, current behavior, applicable constraints, prior evidence and observable success. Carry the requested delivery endpoint and existing authorization through implementation, verification and handoffs. For end-to-end work, continue through `aep --skill deliver` and its closure guidance once validation is ready; explicit analysis, review or candidate-only requests keep their stated scope. `aep context` follows explicit links and accepted decisions referring back to that context; inspect relevant code and runtime behavior as the task requires. Use `--json` for complete structured records when a human summary omits details needed for a decision.

For product refocusing, priority or benchmark decisions, use `aep --skill roadmap` to research the purpose, discuss the recommendation and maintain product context from the user's direction; continue through design when a selected change needs a contract.

## Read guidance as needed

`aep --skill NAME` prints a complete procedure. `aep --skill NAME --ref REFERENCE` prints its supporting resource. REFERENCE accepts a catalog name such as `status` or its Markdown link path `references/status.md`. The list below includes the bundled procedures and any procedures discovered in this project. These commands serve Markdown directly from the executable or project; reading guidance needs no model key or installed AEP plugin.

```bash
aep --skill design
aep --skill design --ref prototype
aep --skill validate
```

## Verify and retain evidence

Use the project's own verification procedure and configured checks. Bind results to the actual candidate and environment; retain failed checks and missing evidence. Follow explicit project independent-review requirements. Imported completion is historical context until reconciled and verified. Read the validate procedure before claiming acceptance or delivery.

Native work writes the configured native stores. Legacy files remain migration sources. Read the migrate procedure for a workflow cutover and keep unresolved context visible. The user's authorization determines whether to implement, review, integrate or publish.
