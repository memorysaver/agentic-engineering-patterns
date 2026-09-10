---
name: migrate
description: Convert useful legacy context into native stores, preserve source provenance, and explicitly switch the project's workflow owner.
---

# Migrate an existing project

Record the Git base and inspect current work, instructions, rules, consumers and active attempts. Select useful stories and dependencies, preserving ongoing constraints, accepted decisions and relevant lessons. Keep installed legacy skills and original data as source snapshots; native stores become the live owner after cutover.

Use `aep migrate plan --story <id> --output <plan>` to preview conversion. Repeat `--story` for more subjects or inspect the default scope. Review writes, source identity, store overlap, target collisions, unresolved references, custom fields and consumer changes. Classify semantic content yourself; CLI conversion handles known structures.

Commit exact source files before cutover so imported context remains retrievable by commit/path/digest. Apply with `aep migrate apply --plan <plan>` and run `aep migrate verify`. Resolve source drift with a fresh plan and preserve edited native targets. Inspect copied rules/lessons and their relative links alongside retained originals.

Switch affected readers/writers together and finish or explicitly restart old-contract attempts. Use `aep skills show project` to reconcile instruction discovery, project constraints and the selected workflow. Legacy workflow commands remain historical context; subsequent work updates native stores. Reading native guidance alone leaves migration unapplied.

Commit the coherent migration after context and checks work. Git revert or selective restore supplies recovery; returning to a legacy workflow after native work requires an explicit handoff of new facts. Use `aep recover` for interrupted CLI file transactions.

An imported completion remains unverified. Inspect code/Git evidence, then use `aep story reconcile <id> --commit <sha> --by <actor>` to record an attributed integration claim and worktree for current validation. Configure and execute current checks and any required review before evaluating covering gates.

Imported custom OpenSpec context stays on import/change records. Map its semantics into active rules/configuration/decisions, then record each mapping through `aep openspec map --file <mapping.json>`. Root-only context also constrains later native changes. Report unmapped constraints and capability gaps explicitly.
