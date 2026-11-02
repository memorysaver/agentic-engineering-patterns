# Native specifications and the web scaffold

AEP 5.0 retires the external specification compatibility surface completely: commands, profile configuration, import/export, automatic migration of external specification bundles, and reference CLI execution are removed. Native BDD parsing, baseline conflict checks, change acceptance and publication remain part of AEP. Historical documentation and the separately versioned v4 skill corpus retain their history.

Existing external specification files are preserved as ordinary files. They are not active AEP contracts. Before resuming work, translate relevant requirements and constraints into native change records, rules and specifications with source attribution. Preview configurations require an explicit edit removing the retired profile key and selecting 5.0.0; strict unknown-field checks remain intact.

The web scaffold is a bundled `stack` procedure, not a runtime dependency of the Rust CLI. It selects Better-T-Stack with Vite+ for workspace tasks, lint/format, tests and Git hooks. The selected application stack is TanStack Start, Hono, oRPC, Drizzle/D1, Better Auth and evlog, with alchemy for local runtime and separately authorized deployment. Generator and dependency overrides are explicit; generated projects retain their lockfiles.

The files reference supplies local environment handling, project-local state, guarded deploy/destroy, tests and CI. A smoke test reads the bundled procedure from the actual executable and exercises a disposable scaffold. It verifies local behavior and guards without deployment credentials. It does not establish a production deployment or provider configuration.

Validation covers native lifecycle regression, absence of retired commands/configuration in new projects, bundled guidance, and the scaffold smoke test. New zero-negation steering baselines cover the stack, runbook and principle procedures.
