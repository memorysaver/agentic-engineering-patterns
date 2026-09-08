---
name: reflect
description: Capture execution lessons and propose a tested correction to project rules, local procedures, or AEP behavior.
---

# Reflect

Read relevant prior lessons, rules, and rejected proposals before proposing a correction. Distinguish the observation, its evidence, and an uncertain diagnosis. Use `aep lesson record --file <file>` for durable observations; `aep lesson find <text>` performs literal text search.

Choose the smallest useful destination: code/configuration, a scoped project rule, a repeatable local procedure, or an upstream AEP change. Use `aep reflect propose --file <file>` to record a rule candidate with its scope, evidence references, hypothesis, counterexample, and validation plan.

Validate the correction on relevant and held-out cases. Adopt through `aep rule adopt <id> --by <actor> --evidence <check-id>` under project authority, and update the indexed rule or project-owned `monet-*` procedure through the same reviewed project change. A candidate record alone does not change an executable policy or built-in skill.

Keep rejected proposals and superseded lessons discoverable. Apply a new rule to new attempts, or explicitly restart affected work when immediate correction is required. Built-in guidance changes ship through AEP source and a CLI release.

Adoption requires a checked and independently reviewed integration containing the exact committed proposal. Pass the resulting check receipt with `--evidence`; an unrelated passing test does not validate a rule.
