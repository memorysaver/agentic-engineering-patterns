# BDD contracts

Describe observable outcomes and relevant failure cases. Requirements have stable names and scenarios state the condition, action, and expected result. A scenario is a specification until a recorded check exercises it at an identified revision and environment.

Use the supported OpenSpec headings: `## ADDED Requirements`, `## MODIFIED Requirements`, `## REMOVED Requirements`, and `## RENAMED Requirements`. Each added or modified requirement starts with `### Requirement: <name>` and has at least one `#### Scenario: <name>`. Modified requirements provide their complete intended text. Rename entries use `- FROM: <name>` and `- TO: <name>`.

Declare each delta file and its baseline digest in the change's `data.specs` list. For a new capability the baseline is null. `aep spec diff --change <id>` exposes the candidate, conflicts, and current baseline. Acceptance, specification publication, integration, and release are separate claims.

```markdown
### Requirement: Safe retry
The operation SHALL preserve a completed result when retried.
#### Scenario: retry-after-completion
- **GIVEN** an operation has completed
- **WHEN** the same operation is retried
- **THEN** the original result is returned
```

Requirement bodies state a SHALL or MUST obligation. AEP checks unfenced scenario headings and explicit WHEN/THEN steps. GIVEN describes relevant setup. Requirement and scenario names are stable identities. Existing scenarios must remain in a MODIFIED block; record deliberate behavior removal in the change design. Unsupported sections and custom schemas require an explicit mapping before acceptance.
