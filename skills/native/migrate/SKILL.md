---
name: migrate
description: Convert selected current context from an existing product-context or OpenSpec layout while keeping older history recoverable through Git.
---

# Migrate an existing project

Record the Git base and inspect current work, rules, consumers, and active attempts. Select useful current stories and their dependencies; preserve ongoing constraints, accepted decisions, and relevant lessons. Git retains the older files.

Use `aep migrate plan --story <id> --output <plan>` to preview the selected conversion. Repeat `--story` for additional subjects, or inspect the command's default scope. Review unresolved references, custom fields, consumer changes, and the intended file diff. Classify semantic content yourself; CLI conversion follows the selected records and known fields.

Apply with `aep migrate apply --plan <plan>` and run `aep migrate verify`. Switch affected readers and writers together; finish or explicitly restart old-contract attempts. Update instruction discovery and project-specific rules using the project skill. Retrieve old context by the recorded Git commit/path when needed.

Commit the coherent migration after current context and checks work. Git revert or selective restore provides recovery. Use `aep recover` only for an interrupted CLI file transaction.

Commit the exact source files before cutover so each imported source can be retrieved by commit and path. Plans preserve prior wave barriers, dependency closure, and prior-layer gate constraints as explicit edges. Rule files move from `project-convention/` into the configured rules store; inspect the resulting index and imported project instructions.

An imported completion remains unverified. After inspecting the code and Git evidence, `aep story reconcile <id> --commit <sha> --by <actor>` records an attributed integration claim and a worktree for current validation. It creates no passing check or review. Configure and run current checks, then record independent review before evaluating a gate that covers it.

Imported custom OpenSpec context is retained on import/change records. After mapping its semantics into active rules/configuration/decisions, record each explicit mapping with `aep openspec map --file <mapping.json>`. Root-only context still constrains later native changes.
