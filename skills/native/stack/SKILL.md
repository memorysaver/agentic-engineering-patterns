---
name: stack
description: Scaffold a new web project with Better-T-Stack and Vite+, native AEP specifications, local development, starter tests and configured checks.
---

# Web project scaffold

Use this procedure for a new web repository. Preserve existing files and project decisions. The default is TanStack Start, Hono on Workers, oRPC, Drizzle with D1, Better Auth, evlog, Vite+ and alchemy. Bun 1.4.2, Git and registry access are required for scaffolding; the AEP CLI itself remains independent of Node and Bun.

1. Select the product name (`<name>`, lowercase letters, digits and hyphens) and purpose. Scaffold in a temporary directory:

   ```bash
   bunx create-better-t-stack@3.44.2 <name> --frontend tanstack-start --backend hono --runtime workers --database sqlite --orm drizzle --api orpc --auth better-auth --payments none --addons evlog vite-plus --examples none --db-setup d1 --web-deploy cloudflare --server-deploy cloudflare --no-git --package-manager bun --no-install --disable-analytics --no-render-title
   ```

2. Merge the generated files into the target repository, retaining its license and existing content. Merge ignore patterns and retain `bts.jsonc`. Run `aep init` there. Record project-specific CI, environments and release choices in the existing rules index.
3. Read `aep --skill stack --ref files`. Apply its Write, Merge into and Append to blocks in order, substituting `__PROJECT__` and the product purpose. These supply Vite+ workspace checks, task caching, Git hooks, tests, CI, local environment handling and guarded deployment. Follow the pinned alchemy and Effect overrides together.
4. Run `bun install`, `bun run format` and `bun run db:generate`; retain `bun.lock` and generated database migrations. Read `aep --skill stack --ref checks` and configure the five checks through `aep config update`.
5. Run `bun install --frozen-lockfile`, `aep check`, `bun run check`, `bun run test` and `bun run build`. Repeat check/build to inspect cache hits. Run `bun run dev`, exercise the landing page, sign-up, sign-in and dashboard, then stop its process group. Confirm plain deploy/destroy commands refuse and the explicit project profile plus stage passes `--guard-only`. This procedure performs local verification; deployment requires its own authorized scope.
6. Inspect tracked files for local secrets and state, then commit the verified bootstrap. Configure CI and branch protection for the user's repository within the requested scope. Use worktrees outside the repository for subsequent stories so JavaScript tools resolve the worktree's dependencies.
7. Record requirements using `aep --skill design --ref bdd`, change records and `aep spec check`. Publish through the existing native delivery and specification lifecycle. Extend the generated project verification procedure as features arrive.

The scaffold uses Vite+ for workspace execution, formatting, linting, tests and Git hooks. Local runtime uses project-local state and placeholder provider credentials; deployment guards require an explicit stage and project profile. Registry templates can change despite a pinned generator: keep the generated lockfile and rerun the smoke test before updating pins.

Reference: [Better-T-Stack](https://www.better-t-stack.dev/docs), [Vite+ workspace execution](https://github.com/voidzero-dev/vite-plus/blob/main/docs/guide/run.md).
