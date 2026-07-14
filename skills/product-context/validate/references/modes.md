# Validation Modes B–D and Customization

`/aep-validate` runs in one of four modes based on the artifact type. Mode A
(Product Context) is the primary branch and stays in SKILL.md with its two-pass
detail. Modes B–D below are simpler generator/evaluator runs — load this file when
validating a design artifact, code, or a document, and when adjusting the agent set.

Read the agent prompt templates for these modes from `/aep-gen-eval`
`agent-contracts.md` (Generator — Artifact Validation; Evaluator — Codebase
Verification). Score against the dimensions in
[validation-dimensions.md](validation-dimensions.md).

---

## Mode B: Design Validation

**When:** After `/aep-design` — validating OpenSpec artifacts (proposal, design, specs, tasks)
**Agents:** Generator + Evaluator

| Agent     | Role                                 | What it checks                                                                  |
| --------- | ------------------------------------ | ------------------------------------------------------------------------------- |
| Generator | Walk through implementation mentally | Are tasks implementable? Missing technical details, unclear acceptance criteria |
| Evaluator | Check specs against codebase         | Do referenced files exist? Are API assumptions correct? Do types match?         |

## Mode C: Code Validation

**When:** After implementation — validating code changes
**Agents:** Generator + Evaluator (same as `/aep-build` Phase 5)

| Agent     | Role                         | What it checks                                                             |
| --------- | ---------------------------- | -------------------------------------------------------------------------- |
| Generator | Review code against spec     | Does the code match what was specified? Missing features, incomplete flows |
| Evaluator | Test the running application | Functional testing, edge cases, security, performance                      |

For code validation in a workspace, prefer `/aep-build` Phase 5 — it has the full
evaluator loop (spawned worktree-bound via `executor.spawn_evaluator`, verification
JSON, scoring framework). Use this skill for code review on the integration branch or
for lighter validation.

## Mode D: Document Validation

**When:** Validating any structured document (architecture doc, RFC, migration plan)
**Agents:** Generator + Evaluator

| Agent     | Role                               | What it checks                                                           |
| --------- | ---------------------------------- | ------------------------------------------------------------------------ |
| Generator | Follow the document's instructions | Can someone execute this document? Missing steps, ambiguous instructions |
| Evaluator | Check claims against reality       | Do referenced tools/files/APIs exist? Are version numbers correct?       |

---

## Customization

### Adding domain-specific checks

Create a `validation-criteria.md` file in your project's `.dev-workflow/` directory to
add project-specific validation checks. The agents read this file if it exists.

```markdown
# Project Validation Criteria

## Additional checks for Mode A (Product Context)

- All stories must have `business_value` field (required by our dispatch)
- Complexity must use S/M/L format (not small/medium/large)
- All file paths must be verified against the actual filesystem

## Additional checks for Mode B (Design)

- All API endpoints must include Zod validation schemas
- Database schema changes must include migration plan
```

### Adjusting agent count

By default, Mode A spawns 3 agents, Modes B–D spawn 2. Adjust for risk:

- **Lighter validation** (1 agent): the artifact is small or low-risk. The single agent combines generator + evaluator roles.
- **Heavier validation** (4+ agents): add domain-specific agents for complex artifacts — a "security reviewer" for auth-related designs, a "performance reviewer" for data-pipeline architectures.
