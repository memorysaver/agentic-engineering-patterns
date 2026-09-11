# Product and design criteria

For domain-heavy work, identify the meaningful objects, relationships, user actions, and attributes. Compare those concepts with screens, API resources or commands: can users reach and manipulate each relevant object, and are ownership and state transitions coherent? OOUX/ORCA can help organize this investigation; use the parts that resolve a real ambiguity.

When quality depends on the user's preferences, capture the applicable criterion, scope, source and examples/counterexamples. Visual design, interaction, API surface, data model, scope, wording and performance are useful dimensions to consider selectively. Show concrete alternatives when a subjective choice changes the result; use established preferences and authority for routine decisions.

Select evaluation lenses from the surface and task. For UI, inspect navigation, feedback, error recovery, keyboard access, semantics and contrast where relevant. For CLI/API, inspect discoverability, input/result consistency, error information and retry behavior. Tie a concern to an observed task or applicable standard, and state its practical impact rather than requiring a universal scorecard.

Retain the chosen criteria and rejected alternatives in a decision/change with source version or observation time. Existing checks and manual observations should reference that same acceptance. A record field pointing at a design document does not automatically bind the document's bytes into every gate; preserve artifact identity and explain any machine-enforcement gap.

If evidence disagrees with a preference or contract, clarify which source governs before rewriting acceptance. Ask for missing human intent when it changes the decision; dimensional classification by itself is not a reason to interrupt work.
