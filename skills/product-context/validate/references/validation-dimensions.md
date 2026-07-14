# Validation Dimensions by Mode

Dimensions the agents weigh when evaluating (adapted from the evaluator criteria in
`/aep-build`). Not all dimensions apply to every mode. Load this when assembling the
evaluator's verification checklist (Step 2) or when scoring consolidated findings.

## For Product Context (Mode A)

| Dimension                    | What to check                                                                                 |
| ---------------------------- | --------------------------------------------------------------------------------------------- |
| **Completeness**             | Are all required sections present? Are enums listed explicitly? Are defaults specified?       |
| **Consistency**              | Do field names match across sections? Do stories reference valid module IDs?                  |
| **Implementability**         | Can each story be implemented with the information given? Missing technical details?          |
| **Security**                 | Are there security implications in the design that aren't addressed? (auth, data access, PII) |
| **Downstream compatibility** | Does the artifact work with its consumers? (dispatch, design, build)                          |

## For Design Artifacts (Mode B)

| Dimension         | What to check                                                                      |
| ----------------- | ---------------------------------------------------------------------------------- |
| **Completeness**  | Do specs cover all capabilities in the proposal? Are acceptance criteria testable? |
| **Feasibility**   | Can the tasks be implemented with the stated approach? Are file paths correct?     |
| **Scope control** | Are tasks properly bounded? Any scope creep beyond the proposal?                   |

## For Code (Mode C)

Score against the full 5-dimension framework (Completeness, Correctness, UX Quality,
Security, Code Quality) defined in `/aep-gen-eval` `scoring-framework.md`.

## For Documents (Mode D)

| Dimension         | What to check                                                        |
| ----------------- | -------------------------------------------------------------------- |
| **Accuracy**      | Are all factual claims correct? Do referenced resources exist?       |
| **Executability** | Can someone follow this document step by step? Are commands correct? |
| **Completeness**  | Are there missing steps or assumptions?                              |
