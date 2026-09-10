# Verification with a prototype

Use a prototype to answer a question that matters to a design decision: whether a flow is understandable, an integration works, performance meets a constraint, or a context shape lets an agent locate necessary evidence. State the question, success observation, and what outcome would change the choice.

Build the smallest experiment that can produce that observation in an isolated scratch directory or worktree. Use the project's verification procedure to operate the relevant surface; read `aep --skill validate --ref self-verification` for evidence quality. One candidate or sequential experiments may be sufficient. Delegation and competing candidates are optional methods under host authority.

Record the tested version, environment, inputs, method, result and limitations. Compare alternatives on the decision's criterion. For example, evaluate a context index by whether a cold reader can find the actual acceptance and run its check, rather than whether its summary looks complete.

Save the durable finding and decision in the change/ADR; retain useful evidence before deleting scratch resources. Distinguish observed facts from the inference that a production approach will work.

`aep dispatch start` creates a formal implementation attempt after contract acceptance. Use ordinary scratch isolation for exploration while the production contract is unresolved. A prototype's completion means the question is answered; production code still needs the relevant implementation, integration and verification work. If prototype code is reused, validate the resulting formal candidate at its actual revision.
