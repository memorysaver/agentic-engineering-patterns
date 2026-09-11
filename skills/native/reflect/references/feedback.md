# Feedback destinations

Capture what was observed, source identity, event/observation time when known, affected version/environment and evidence. Monitoring adapters own fetching, deduplication and cursors; AEP can retain the resulting observation. A source timestamp and the time AEP recorded it are distinct facts.

Compare the observation with accepted intent. A defect against that intent becomes repair work. A refinement or newly discovered opportunity changes roadmap/change/decision context after its assumptions and tradeoffs are examined. Process failures can become code, a scoped rule or a local procedure. State uncertainty when the evidence does not yet distinguish these cases.

For outcome learning, compare actual user behavior with the original hypothesis and its measurement scope. Update the next slice when evidence changes its value or dependencies. Preserve useful counterevidence and rejected approaches rather than retaining only success stories.

If a verification map is stale, rerun the affected operation and repair navigation/tool instructions. If the product violates acceptance, retain the defect and repair the implementation; changing map expectations would erase the signal. Keep existing canonical journeys and their links intact.

For a reusable upstream candidate, save the smallest reproducer, expected/observed behavior, project context needed to interpret it, attempted correction and its results. Combine similar observations only when they support the same mechanism. Local capture is separate from sending an issue or message; use explicit task authority for external publication.

When an observation yields a reusable project operation, create or improve a project skill at `<rules-store>/skills/<name>/SKILL.md` (normally `project-rules/skills/`). Give its frontmatter a distinct `name` and a `description` that explains when to use it; keep the procedure focused on project-specific knowledge and link supporting references. Reuse an existing suitable skill before adding another. Exercise a representative use and verify that `aep --skill` lists it under Project procedures and `aep --skill <name>` reads the intended content. The shared interface discovers project-owned knowledge; built-in guidance still ships with AEP. One-off observations remain lessons. Read `aep --skill project --ref verification-setup` for a verification procedure and its operation map.
