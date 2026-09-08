---
name: validate
description: Verify an implementation against its change contract and collect test, independent review, and gate evidence.
---

# Validate

For a review-only request, inspect the supplied candidate and report findings to the caller. Use read-only `aep context`, `aep verify plan`, and `aep query` as needed. Do not start another review, repair code, or merge unless the request includes that work. When responding to an existing review request, use its supplied response shape and preserve carried blocking findings.

For implementation validation, inspect the diff and observable behavior against accepted scenarios. Run `aep verify plan --story <id>` to inspect configured checks and the derived verification floor, then `aep verify run --story <id>` to execute them. Record failures and revise the implementation rather than changing acceptance to certify a defect.

For an independent review, run `aep review request --story <id>` and give the returned contract, revisions, and evidence to a fresh reviewer under the host's capabilities. Record its structured response using `aep review record --file <file>`. The request defines the response shape, attribution, and revision. CLI validation checks evidence structure and freshness; it does not establish a reviewer's real identity.

Keep the existing two-round review limit for standard and deep work. A second round confirms a blocking correction; material findings are fixed with evidence and polish remains within scope. An unresolved blocking finding after the limit requires a revised plan or the project's escalation policy. Light review applies only to eligible documentation changes without contract obligations.

Run `aep gate evaluate <id>` when its declared scope is ready. Report pass, fail, blocked, and skipped checks accurately. A passing local test does not establish deployment, and edits after a check require fresh evidence for the affected inputs.
