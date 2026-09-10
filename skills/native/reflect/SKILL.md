---
name: reflect
description: Turn observed product or execution feedback into context updates, lessons, project rules, local procedures, or grounded upstream proposals.
---

# Reflect

Read prior lessons, rules, and rejected proposals relevant to the observation. Separate the evidence from an uncertain diagnosis. Read [Feedback destinations](references/feedback.md) when deciding whether the result changes product intent, implementation, verification or process.

Use `aep lesson record --file <file>` for durable observations; `aep lesson find <text>` provides literal text search. Choose the smallest useful destination: a product story/decision, code/configuration, a scoped rule, a project-owned procedure, or an upstream AEP proposal.

For rule changes, `aep reflect propose --file <file>` records scope, evidence references, hypothesis, counterexample and validation plan. Evaluate the correction on relevant and contrasting cases. Implement the exact committed proposal under a story, then adopt with `aep rule adopt <id> --by <actor> --evidence <check-id>` under project authority. Adoption checks integrated evidence and the project's review requirements.

Update the indexed rule or local procedure in that same project change. A candidate record alone leaves executable policy and bundled skills unchanged. Keep rejected proposals and superseded lessons discoverable; apply a new rule to new attempts or explicitly restart affected work when immediate correction is required.

Built-in guidance changes ship through AEP source and release. Save an upstream candidate with reproducible evidence; external submission follows the task's authorization.
