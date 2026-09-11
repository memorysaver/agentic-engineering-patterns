---
name: validate
description: Verify implementation or other artifacts against intent using observable results, current evidence, and project-required checks or reviews.
---

# Validate

For a review-only request, inspect the supplied candidate and report findings. Use read-only `aep context`, `aep verify plan`, and `aep query` as needed. Do not start another review, repair code, or merge unless requested. Respond to an existing review using its supplied shape and carried findings.

For completion validation, read [Self verification](references/self-verification.md) (`aep --skill validate --ref self-verification`). Inspect the actual candidate against acceptance, use the project's operation procedure, and identify untested claims. Run `aep verify plan --story <id>` to inspect required checks and policy, then `aep verify run --story <id>` to execute them. Record failures and repair defects against the accepted behavior.

Independent review is required when project policy says so and available as an additional chosen method. Use `aep review request --story <id>` and pass the contract, revisions, and evidence to an independent reviewer under host authority. Record the structured response with `aep review record --file <file>`. CLI validation checks attribution structure and freshness; reviewer independence also depends on actual host execution.

Resolve concrete blocking/material findings with current evidence, including findings from optional reviews. A follow-up review is useful when an unresolved issue or changed candidate warrants it. Let evidence and remaining problems determine further investigation. Report limits or needed decisions when progress depends on missing access, intent or authority.

Evaluate declared gates with `aep gate evaluate <id>` when their scope is ready. Distinguish pass, fail, blocked and skipped results; local evidence supports local claims. Changes to relevant inputs require current evidence before delivery.

After completion validation, continue with `aep --skill deliver` toward the carried delivery endpoint. Passing checks and a clean review establish candidate readiness. Finish a validation-only request with findings; for end-to-end work, carry this evidence into authorized delivery and closure.
