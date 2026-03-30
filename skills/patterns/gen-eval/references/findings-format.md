# Findings Format

How to consolidate, categorize, and present findings from generator/evaluator agents. Used when multiple agents produce findings that must be merged into a single action list.

---

## Severity Categories

Every finding falls into one of three categories:

| Category | Definition | Action |
|----------|-----------|--------|
| **Blocking** | Would stop the downstream consumer from working. Implementation cannot proceed without fixing this. | Fix immediately before proceeding |
| **Important** | Would cause friction, confusion, or rework. Consumer can work around it but shouldn't have to. | Fix before proceeding if possible |
| **Minor** | Cosmetic, missing optional fields, style inconsistencies. No functional impact. | Fix if time permits, or defer |

### Classification heuristics

| Signal | Likely category |
|--------|----------------|
| Missing required field → downstream skill errors | Blocking |
| Security vulnerability (auth bypass, data leak) | Blocking |
| Ambiguous acceptance criteria (implementer must guess) | Important |
| Wrong file path (exists but different name) | Important |
| Missing optional field (has reasonable default) | Minor |
| Naming convention mismatch (functional but inconsistent) | Minor |

---

## Deduplication

Multiple agents often find the same issue from different angles. Merge duplicates:

### How to detect duplicates

Two findings are duplicates if they:
1. Reference the same item (story, field, file, endpoint)
2. Describe the same root cause (even if symptoms differ)
3. Would be fixed by the same change

### How to merge

When merging duplicate findings:
- Keep the **higher severity** classification
- Combine evidence from both agents
- Note which agents found it: "found by Generator + Evaluator"
- Keep the most actionable fix suggestion

### Example

**Generator found:** "Story `api-storage-router` accepts `creatorId` from client — implementer needs clarification on ownership model"

**Evaluator found:** "Story `api-storage-router` has a security issue — `creatorId` should come from session, not client input, to prevent cross-user access"

**Merged finding:**
```
BLOCKING: creatorId must come from session, not client input
- Found by: Generator (missing detail) + Evaluator (security issue)
- Impact: Any user could upload to another user's creator profile
- Fix: Derive creatorId from session.user.id in all storage procedures
```

---

## Presentation Format

### Summary line

```
Validation complete: {N} blocking, {M} important, {K} minor issues found.
```

### Findings list (grouped by severity)

```markdown
## Blocking ({N} issues)

1. **[Short title]** — found by [agent(s)]
   - Impact: [what breaks if not fixed]
   - Fix: [specific, actionable change]

2. **[Short title]** — found by [agent(s)]
   - Impact: [what breaks]
   - Fix: [what to do]

## Important ({M} issues)

3. **[Short title]** — found by [agent(s)]
   - Impact: [what friction this causes]
   - Fix: [suggestion]

## Minor ({K} issues)

4. **[Short title]** — found by [agent(s)]
   - Impact: [cosmetic/optional]
   - Fix: [suggestion]
```

### Table format (for large finding sets)

When there are 10+ findings, a table is more scannable:

```markdown
| # | Severity | Issue | Found by | Fix |
|---|----------|-------|----------|-----|
| 1 | Blocking | creatorId from client = security hole | Gen + Eval | Derive from session |
| 2 | Blocking | Missing dispatch_epoch field | Protocol | Add top-level field |
| 3 | Important | Zod v4 incompatibility | Evaluator | Hand-write schemas |
| ... | | | | |
```

---

## Changelog Entry Format

After applying fixes, append a changelog entry to the artifact:

```yaml
- date: <ISO 8601 date>
  author: aep-gen-eval
  summary: >
    Generator/evaluator validation ({mode}). Found {N} blocking, {M} important,
    {K} minor issues. Fixed: [brief list of key fixes applied].
```

### Mode labels for changelog

| Mode | Label |
|------|-------|
| Product context validation | `product-context` |
| Design artifact validation | `design` |
| Code review | `code-review` |
| Document validation | `document` |
| Custom | Use the artifact name |

---

## Rules for Applying Fixes

1. **Only modify the artifact being validated** — never create new files or modify other artifacts as a side effect
2. **Preserve the artifact's existing structure** — don't reorganize sections unless the fix requires it
3. **Add, don't replace** — when adding missing fields, don't remove existing fields
4. **Mark decisions as open questions** — if a fix requires a judgment call the agent can't make, add it as an `open_question` with a default assumption and revisit trigger
5. **Present findings before fixing** — always show the user the findings and let them approve before applying changes. They may disagree with some findings or want to prioritize differently.
6. **Fix blocking issues first** — if time is limited, blocking issues take priority over important, which take priority over minor

---

## Open Questions Format

When a finding requires a decision the agent can't make:

```yaml
open_questions:
  - id: question-id
    question: "What is the correct approach for X?"
    default_assumption: "Do Y (most common pattern)"
    revisit_trigger: "If Z happens, reconsider"
    raised_by: gen-eval validation
    date: <ISO 8601>
```

This pattern ensures the artifact is usable with the default assumption, but the decision point is visible for human review.
