# Alchemy CI/CD: Deploying to Cloudflare from GitHub Actions / GitLab CI

How to deploy an Alchemy project to Cloudflare via CI/CD. Covers two scenarios: migrating a locally-deployed project to CI, and setting up CI-only deployment from the start. Distilled from production experience across multiple projects.

## Prerequisites

- Turborepo monorepo with `packages/infra/alchemy.run.ts`
- Better-T-Stack (Hono + oRPC + TanStack Start + Better Auth) or similar
- Cloudflare account with Workers, D1, R2

## The 5 Gotchas

Every Alchemy CI/CD setup hits some combination of these. Knowing them upfront saves hours.

| #   | Gotcha                      | Symptom                                           | Fix                                 |
| --- | --------------------------- | ------------------------------------------------- | ----------------------------------- |
| 1   | **Turbo strips env vars**   | `ALCHEMY_STATE_TOKEN` not found, `Stage: runner`  | Add `passThroughEnv` to deploy task |
| 2   | **No remote state store**   | Alchemy throws error in CI                        | Use `CloudflareStateStore`          |
| 3   | **Stage-dependent naming**  | CI creates new D1/R2 instead of updating existing | Pin resource `name` props           |
| 4   | **Resources already exist** | `Worker with name 'X' already exists`             | Add `adopt: true`                   |
| 5   | **No OAuth in CI**          | Auth fails (no `~/.alchemy/credentials/`)         | Use `CLOUDFLARE_API_TOKEN` env var  |

## Two Approaches

### Approach A: CI-Only Deployment (Recommended for New Projects)

CI is the **only** deployer. Local machines never run `alchemy deploy`. This avoids gotcha #4 entirely.

```ts
// packages/infra/alchemy.run.ts
import { CloudflareStateStore } from "alchemy/state";

const app = await alchemy("my-app", {
  stage: process.env.ALCHEMY_STAGE ?? "production",
  stateStore: (scope) => new CloudflareStateStore(scope),
  // No profile — always use CLOUDFLARE_API_TOKEN
});
```

Advantages:

- No state divergence between local and CI
- No `adopt` needed (CI creates everything on first deploy)
- Simpler `alchemy.run.ts` (no conditionals)

### Approach B: Migrate Local Deploy to CI (Existing Projects)

Project was deployed locally first. CI needs to adopt existing resources.

```ts
// packages/infra/alchemy.run.ts
import { CloudflareStateStore } from "alchemy/state";

const app = await alchemy("my-app", {
  stage: process.env.ALCHEMY_STAGE,
  // Remote state in CI, local filesystem in dev
  stateStore: process.env.CI ? (scope) => new CloudflareStateStore(scope) : undefined,
  // OAuth profile locally, API token in CI
  profile: process.env.CI ? undefined : "my-profile",
});
```

This approach requires `adopt: true` on all resources for the first CI deploy, plus pinned `name` props to avoid stage-dependent naming.

## Step-by-Step Setup

### 1. Configure `alchemy.run.ts`

Choose Approach A or B above. Then apply these rules to all resources:

**Pin resource names** — Alchemy generates names using `scope.createPhysicalName(id)` which includes the stage. Different stages = different Cloudflare resource names = new resources instead of updating existing ones.

```ts
// BAD: name depends on stage
const db = await D1Database("database", {
  migrationsDir: "../../packages/db/src/migrations",
});
// Creates: my-app-database-dev (local) vs my-app-database-production (CI)

// GOOD: name is fixed
const db = await D1Database("database", {
  name: "my-app-database", // ← Always this name, any stage
  migrationsDir: "../../packages/db/src/migrations",
  adopt: true, // ← Only needed for Approach B (migrate from local)
});
```

Workers and R2 buckets that already have explicit `name` props are fine. Add `adopt: true` to everything if migrating from local deploy (Approach B).

```ts
const web = await TanStackStart("web", {
  name: "my-web",
  adopt: true, // Approach B only
  url: true,
  cwd: "../../apps/web",
  bindings: {
    /* ... */
  },
});

const server = await Worker("server", {
  name: "my-api",
  adopt: true, // Approach B only
  url: true,
  cwd: "../../apps/server",
  bindings: {
    /* ... */
  },
});
```

After the first CI deploy succeeds, state is stored in `CloudflareStateStore`. Subsequent deploys use `update` instead of `create` — `adopt: true` only matters for the initial migration.

### Why `stage` must be explicit

`DEFAULT_STAGE` is a module-level constant evaluated at import time:

```ts
// In alchemy/src/scope.ts — evaluated when module is first imported
export const DEFAULT_STAGE =
  process.env.ALCHEMY_STAGE ??
  process.env.USER ?? // "runner" on GitHub Actions
  process.env.USERNAME ??
  "dev";
```

Even when `ALCHEMY_STAGE` is set in the workflow step, Turbo's env filtering (gotcha #1) can prevent it from reaching the subprocess. Passing `stage` explicitly to `alchemy()` is reliable.

### 2. Configure Turbo `passThroughEnv`

**This is the gotcha that wastes the most time.** Turborepo filters environment variables by default. Your secrets will be set in the CI step but won't reach `alchemy.run.ts`.

Add `passThroughEnv` to the deploy task in `turbo.json`:

```json
{
  "tasks": {
    "deploy": {
      "cache": false,
      "passThroughEnv": [
        "CI",
        "CLOUDFLARE_API_TOKEN",
        "CLOUDFLARE_ACCOUNT_ID",
        "ALCHEMY_PASSWORD",
        "ALCHEMY_STATE_TOKEN",
        "ALCHEMY_STAGE",
        "BETTER_AUTH_SECRET",
        "CORS_ORIGIN",
        "BETTER_AUTH_URL",
        "VITE_SERVER_URL"
      ]
    }
  }
}
```

**Rule:** Every env var that `alchemy.run.ts` reads (via `alchemy.env.*`, `alchemy.secret.env.*`, or `process.env.*`) must be listed here. When you add a new env var to `alchemy.run.ts`, also add it to this list.

### 3. Create Cloudflare API Token

Go to https://dash.cloudflare.com/profile/api-tokens → Create Token → Custom Token:

| Permission                   | Access |
| ---------------------------- | ------ |
| Account > Workers Scripts    | Edit   |
| Account > D1                 | Edit   |
| Account > Workers R2 Storage | Edit   |
| Account > Workers AI         | Read   |
| Account > Account Settings   | Read   |

Scope to your specific account. Copy the token (shown only once).

### 4. Set CI Secrets

**GitHub Actions:**

```bash
# Generate tokens
ALCHEMY_STATE_TOKEN=$(openssl rand -hex 32)

# Set secrets
gh secret set CLOUDFLARE_API_TOKEN --body "your-cf-token"
gh secret set CLOUDFLARE_ACCOUNT_ID --body "your-account-id"
gh secret set ALCHEMY_PASSWORD --body "your-alchemy-password"
gh secret set ALCHEMY_STATE_TOKEN --body "$ALCHEMY_STATE_TOKEN"
gh secret set BETTER_AUTH_SECRET --body "your-auth-secret"
# ... add all app-specific secrets
```

**GitLab CI:**

```bash
glab variable set CLOUDFLARE_API_TOKEN --value "your-cf-token" --masked
glab variable set ALCHEMY_STATE_TOKEN --value "$ALCHEMY_STATE_TOKEN" --masked
glab variable set ALCHEMY_PASSWORD --value "your-alchemy-password" --masked
# ... add all app-specific variables
```

**Important:** `ALCHEMY_PASSWORD` must match the one used locally (in `packages/infra/.env`). It encrypts/decrypts secrets in state. Changing it between deploys breaks decryption.

### 5. Create the Workflow

**GitHub Actions** (`.github/workflows/deploy.yml`):

```yaml
name: Deploy

on:
  push:
    branches: [main]

concurrency:
  group: deploy
  cancel-in-progress: false # Don't cancel in-flight deploys

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: oven-sh/setup-bun@v2
        with:
          bun-version: "1.2.18" # Match your packageManager field

      - run: bun install --frozen-lockfile

      - name: Deploy to Cloudflare
        run: bun run deploy
        env:
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          CLOUDFLARE_ACCOUNT_ID: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          ALCHEMY_PASSWORD: ${{ secrets.ALCHEMY_PASSWORD }}
          ALCHEMY_STATE_TOKEN: ${{ secrets.ALCHEMY_STATE_TOKEN }}
          ALCHEMY_STAGE: production
          BETTER_AUTH_SECRET: ${{ secrets.BETTER_AUTH_SECRET }}
          # ... all app-specific env vars
          CORS_ORIGIN: "https://www.your-app.workers.dev"
          BETTER_AUTH_URL: "https://api.your-app.workers.dev"
          VITE_SERVER_URL: "https://api.your-app.workers.dev"
```

**GitLab CI** (`.gitlab-ci.yml`):

```yaml
deploy:
  image: oven/bun:1
  stage: deploy
  only:
    - main
  script:
    - bun install --frozen-lockfile
    - bun run deploy
  variables:
    ALCHEMY_STAGE: production
    CORS_ORIGIN: "https://my-web.my-subdomain.workers.dev"
    BETTER_AUTH_URL: "https://my-api.my-subdomain.workers.dev"
    VITE_SERVER_URL: "https://my-api.my-subdomain.workers.dev"
```

GitLab injects CI/CD variables automatically — no need to list secrets in the YAML.

**Key decisions:**

- `concurrency` / single-pipeline — prevents overlapping deploys corrupting state
- No `environment:` in GitHub Actions — requires GitHub environment setup; repo-level secrets are simpler to start with
- `--frozen-lockfile` — reproducible installs
- `ALCHEMY_STAGE: production` — separate state namespace from local dev

## Environment Variable Flow

Env vars travel through a multi-hop chain. Understanding this flow is critical for debugging "env var not reaching the app" issues.

```
+-------------------------------------------------------+
| 1. SOURCE                                             |
|                                                       |
|   .env (local) ─── or ─── CI variable (prod)         |
+---------------------------+---------------------------+
                            |
                            v
+-------------------------------------------------------+
| 2. TURBO passThroughEnv GATE                          |
|                                                       |
|   turbo.json: deploy.passThroughEnv: [...]            |
|                                                       |
|   ⚠ If a var is NOT listed here, Turbo strips it      |
+---------------------------+---------------------------+
                            |
                            v
+-------------------------------------------------------+
| 3. ALCHEMY (packages/infra/alchemy.run.ts)            |
|                                                       |
|   Reads process.env via alchemy.env.* / .secret.env.* |
|   Falls back to defaults from config package          |
|   Binds values to Cloudflare Workers                  |
+---------------------------+---------------------------+
                            |
                            v
+-------------------------------------------------------+
| 4. CLOUDFLARE WORKERS                                 |
|                                                       |
|   Receives env vars as Worker bindings (env.*)        |
|   Available at runtime in request handlers            |
+-------------------------------------------------------+
```

### Debugging Checklist

If an env var isn't reaching your Worker:

| Step | Check                                   | Fix                                       |
| ---- | --------------------------------------- | ----------------------------------------- |
| 1    | Is it in `.env` / CI variables?         | Add it                                    |
| 2    | Is it in `turbo.json` `passThroughEnv`? | Add it (Turbo strips unlisted vars)       |
| 3    | Is it read in `alchemy.run.ts`?         | Add binding to Worker                     |
| 4    | Is it available in the Worker handler?  | Check `env.VAR_NAME` in your Hono context |

## Interacting with Remote State

The `CloudflareStateStore` deploys a Worker called `alchemy-state-service` on your Cloudflare account. You can query it directly:

```bash
# List all resources in a scope
curl -s -X POST "https://alchemy-state-service.your-subdomain.workers.dev" \
  -H "Authorization: Bearer $ALCHEMY_STATE_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"method":"list","params":[],"context":{"chain":["my-app","production"]}}'

# Delete a specific orphaned entry
curl -s -X POST "https://alchemy-state-service.your-subdomain.workers.dev" \
  -H "Authorization: Bearer $ALCHEMY_STATE_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"method":"delete","params":["resource-name"],"context":{"chain":["my-app","production"]}}'
```

For local `alchemy read` or `alchemy destroy` commands against production state, set matching credentials in `packages/infra/.env`:

```
ALCHEMY_PASSWORD=<same-as-CI>
ALCHEMY_STATE_TOKEN=<same-as-CI>
```

## Troubleshooting

### `Missing token for CloudflareStateStore`

`ALCHEMY_STATE_TOKEN` isn't reaching Alchemy. Check:

1. Secret is set in CI (`gh secret list` / `glab variable list`)
2. It's in the workflow `env:` block (GitHub) or CI variables (GitLab)
3. It's in `turbo.json` `passThroughEnv`

### `Worker with name 'X' already exists`

Resource exists on Cloudflare but CI has no state for it (first CI deploy after local). Add `adopt: true` to the resource. Only needed for Approach B.

### `Stage: runner` instead of `production`

`ALCHEMY_STAGE` isn't reaching Alchemy. Same 3-step diagnosis as above. Also ensure `stage: process.env.ALCHEMY_STAGE` is passed explicitly to `alchemy()`.

### `Cannot serialize secret without password`

`ALCHEMY_PASSWORD` is not set. Same 3-step diagnosis.

### First deploy works, second fails with crypto errors

`ALCHEMY_PASSWORD` changed between deploys. The password encrypts secrets in state — it must be identical across all deploys (local and CI).

### Nuclear option: destroy and redeploy

If state is too corrupted:

1. Remove `CloudflareStateStore` from `alchemy.run.ts` temporarily (use default FileSystemStateStore)
2. Run `bun run destroy` locally (uses local `.alchemy/` state files)
3. Clear remaining entries from the remote state store via curl
4. Restore `CloudflareStateStore` in `alchemy.run.ts`
5. Push to `main` — CI creates all resources fresh

### Skipping CI

Add `[skip ci]` to your commit message:

```bash
git commit -m "docs: update readme [skip ci]"
```

## Timeline: What Happens on Each Deploy

**First CI deploy (~1-2 min):**

1. Alchemy provisions `alchemy-state-service` Worker (CloudflareStateStore backend)
2. Creates (or adopts) D1 database, R2 buckets, Workers
3. Builds and deploys web + server bundles
4. Stores all resource state in Durable Objects

**Subsequent deploys (~1 min):**

1. Reads state from Durable Objects
2. Diffs desired state against current state
3. Updates only changed resources (Workers rebundled, bindings updated)
4. Writes updated state back

## Related

- [workers.dev Auth Cookie Gotcha](./cloudflare-workers-dev-auth.md) — cross-subdomain cookies don't work on workers.dev
- [Alchemy CloudflareStateStore Guide](https://alchemy.run/guides/cloudflare-state-store/)
- [Turborepo Environment Variables](https://turbo.build/repo/docs/crafting-your-repository/using-environment-variables)
