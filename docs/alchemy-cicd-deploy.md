# Alchemy CI/CD: From Local Deploy to GitHub Actions

How to configure a project that was first deployed locally with Alchemy to deploy automatically via GitHub Actions. Covers the gotchas you'll hit when Alchemy's local state doesn't exist in CI.

## Prerequisites

- Project deployed locally via `alchemy deploy` (resources exist on Cloudflare)
- Turborepo monorepo with `packages/infra/alchemy.run.ts`
- GitHub repository with push access

## The Problem

Alchemy stores deployment state in `.alchemy/` on your local filesystem (encrypted JSON files tracking what resources exist). In CI:

1. **No `.alchemy/` state** — CI runners are ephemeral, no local state between runs
2. **No OAuth profile** — Alchemy's local auth uses `~/.alchemy/credentials/`, not available in CI
3. **CI detection** — Alchemy detects `process.env.CI` and **throws an error** if using the default filesystem state store
4. **Turbo env filtering** — Turborepo doesn't pass environment variables to child tasks by default
5. **Existing resources** — Workers/databases already exist from local deploy, Alchemy's `create` phase fails

## Solution Overview

```
Local Dev                          CI (GitHub Actions)
─────────                          ──────────────────
OAuth profile auth         →       CLOUDFLARE_API_TOKEN env var
FileSystemStateStore       →       CloudflareStateStore (Durable Objects)
Stage: $USER (e.g. "dev")  →       Stage: "production" (explicit)
.alchemy/ directory        →       Remote state in Cloudflare DO
```

## Step 1: Update `alchemy.run.ts`

Three changes: conditional state store, conditional auth, explicit stage.

```ts
import alchemy from "alchemy";
import { CloudflareStateStore } from "alchemy/state";
import {} from /* your resources */ "alchemy/cloudflare";

const app = await alchemy("my-app", {
  // Explicit stage — don't rely on DEFAULT_STAGE (reads USER env at module load)
  stage: process.env.ALCHEMY_STAGE,
  // Remote state in CI, local filesystem in dev
  stateStore: process.env.CI ? (scope) => new CloudflareStateStore(scope) : undefined,
  // OAuth profile locally, API token in CI
  profile: process.env.CI ? undefined : "my-profile",
});
```

### Why `stage` must be explicit

`DEFAULT_STAGE` is a module-level constant evaluated at import time:

```ts
// In alchemy/src/scope.ts
export const DEFAULT_STAGE =
  process.env.ALCHEMY_STAGE ??
  process.env.USER ?? // "runner" on GitHub Actions
  process.env.USERNAME ??
  "dev";
```

Even though `ALCHEMY_STAGE` is set in the workflow, the module may evaluate before the env is available in some bundler scenarios. Passing `stage` explicitly to `alchemy()` is reliable.

## Step 2: Pin Resource Names

Alchemy generates resource names using `scope.createPhysicalName(id)` which includes the stage. Different stages = different Cloudflare resource names = **new resources instead of updating existing ones**.

Workers with explicit `name` props are fine — the name is used directly regardless of stage. But D1Database without a `name` prop gets `my-app-database-production` in CI vs `my-app-database-dev` locally.

**Pin all resource names:**

```ts
// BAD: name depends on stage
const db = await D1Database("database", {
  migrationsDir: "../../packages/db/src/migrations",
});
// Creates: my-app-database-dev (local) vs my-app-database-production (CI)

// GOOD: name is fixed
const db = await D1Database("database", {
  name: "my-app-database", // ← Always this name
  migrationsDir: "../../packages/db/src/migrations",
  adopt: true,
});
```

Resources that already have explicit names (like `R2Bucket("assets", { name: "my-assets" })`) don't need changes.

## Step 3: Add `adopt: true` to All Resources

The first CI deploy has empty state (fresh `CloudflareStateStore`). Alchemy's create phase will try to create resources that already exist from local deploy. Without `adopt: true`, you get:

```
error: Worker with name 'api' already exists. Please use a unique name.
error: Database with name: 'my-app-database' already exists [7502]
```

Add `adopt: true` to every resource:

```ts
const db = await D1Database("database", {
  name: "my-app-database",
  adopt: true, // ← Adopt if already exists
  migrationsDir: "...",
});

const assets = await R2Bucket("assets", {
  name: "my-app-assets",
  adopt: true, // ← Already had this
});

const web = await TanStackStart("web", {
  name: "www",
  adopt: true, // ← Add this
  url: true,
  cwd: "../../apps/web",
  bindings: {
    /* ... */
  },
});

const server = await Worker("server", {
  name: "api",
  adopt: true, // ← Add this
  url: true,
  cwd: "../../apps/server",
  bindings: {
    /* ... */
  },
});
```

After the first CI deploy succeeds, state is stored in `CloudflareStateStore`. Subsequent deploys will see existing state and use `update` instead of `create`. The `adopt: true` only matters for the initial migration.

## Step 4: Configure Turbo `passThroughEnv`

**This is the gotcha that wastes the most time.** Turborepo filters environment variables by default. Your secrets will be set in the GitHub Actions step but **won't reach `alchemy.run.ts`**.

Symptoms: `Stage: runner` instead of `production`, `ALCHEMY_STATE_TOKEN` not found.

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
        "WAVESPEED_API_KEY",
        "R2_ACCESS_KEY_ID",
        "R2_SECRET_ACCESS_KEY",
        "R2_ACCOUNT_ID",
        "CORS_ORIGIN",
        "BETTER_AUTH_URL",
        "VITE_SERVER_URL"
      ]
    }
  }
}
```

List every env var that `alchemy.run.ts` reads (via `alchemy.env.*`, `alchemy.secret.env.*`, or `process.env.*`).

## Step 5: Create Cloudflare API Token

Go to https://dash.cloudflare.com/profile/api-tokens → Create Token → Custom Token:

| Permission                   | Access |
| ---------------------------- | ------ |
| Account > Workers Scripts    | Edit   |
| Account > D1                 | Edit   |
| Account > Workers R2 Storage | Edit   |
| Account > Workers AI         | Read   |
| Account > Account Settings   | Read   |

Scope to your specific account. Copy the token.

## Step 6: Set GitHub Secrets

```bash
# Generate state token
ALCHEMY_STATE_TOKEN=$(openssl rand -hex 32)

# Set secrets (values from your local .env files)
gh secret set CLOUDFLARE_API_TOKEN --body "your-cf-token"
gh secret set CLOUDFLARE_ACCOUNT_ID --body "your-account-id"
gh secret set ALCHEMY_PASSWORD --body "your-alchemy-password"
gh secret set ALCHEMY_STATE_TOKEN --body "$ALCHEMY_STATE_TOKEN"
gh secret set BETTER_AUTH_SECRET --body "your-auth-secret"
gh secret set WAVESPEED_API_KEY --body "your-wavespeed-key"
gh secret set R2_ACCESS_KEY_ID --body "your-r2-key-id"
gh secret set R2_SECRET_ACCESS_KEY --body "your-r2-secret"
```

The `ALCHEMY_PASSWORD` must match the one used locally (in `packages/infra/.env`). It encrypts/decrypts secrets in state.

## Step 7: Create the Workflow

`.github/workflows/deploy.yml`:

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
          WAVESPEED_API_KEY: ${{ secrets.WAVESPEED_API_KEY }}
          R2_ACCESS_KEY_ID: ${{ secrets.R2_ACCESS_KEY_ID }}
          R2_SECRET_ACCESS_KEY: ${{ secrets.R2_SECRET_ACCESS_KEY }}
          R2_ACCOUNT_ID: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          CORS_ORIGIN: "https://www.your-app.workers.dev"
          BETTER_AUTH_URL: "https://api.your-app.workers.dev"
          VITE_SERVER_URL: "https://api.your-app.workers.dev"
```

**Key decisions:**

- `concurrency: deploy` with `cancel-in-progress: false` — prevents overlapping deploys corrupting state
- No `environment:` — requires GitHub environment setup; repo-level secrets are simpler
- `--frozen-lockfile` — reproducible installs
- `ALCHEMY_STAGE: production` — different state namespace from local dev

## Troubleshooting

### Error: `Missing token for CloudflareStateStore`

`ALCHEMY_STATE_TOKEN` isn't reaching Alchemy. Check:

1. Secret is set: `gh secret list` should show it
2. It's in the workflow `env:` block
3. It's in `turbo.json` `passThroughEnv`

### Error: `Worker with name 'X' already exists`

Resource exists on Cloudflare but CI has no state for it. Add `adopt: true` to the resource.

### `Stage: runner` instead of `production`

`ALCHEMY_STAGE` isn't reaching Alchemy. Same diagnosis as above. Also ensure `stage: process.env.ALCHEMY_STAGE` is passed explicitly to `alchemy()`.

### First deploy works, second fails with crypto errors

`ALCHEMY_PASSWORD` changed between deploys. The password encrypts secrets in state — it must be the same across all deploys (local and CI).

### dotenv injects 0 vars

Expected. CI doesn't have `.env` files. All config comes from GitHub Actions env vars.

## How It All Fits Together

```
Push to main
  → GitHub Actions triggers
    → bun install
    → bun run deploy
      → turbo -F @my-app/infra deploy (passThroughEnv forwards secrets)
        → alchemy deploy
          → CloudflareStateStore reads/writes state via Durable Objects
          → CLOUDFLARE_API_TOKEN authenticates to Cloudflare API
          → Resources are created (first run) or updated (subsequent)
          → Workers, D1, R2 deployed to Cloudflare
```

**First CI deploy (~1 min):** Creates `CloudflareStateStore` worker, adopts all existing resources, stores state.

**Subsequent deploys (~1 min):** Reads state from DO, diffs against desired state, updates changed resources.

## Related

- [workers.dev Auth Cookie Gotcha](./cloudflare-workers-dev-auth.md) — cross-subdomain cookies don't work on workers.dev
- [Alchemy CloudflareStateStore Guide](https://alchemy.run/guides/cloudflare-state-store/)
- [Turborepo Environment Variables](https://turbo.build/repo/docs/crafting-your-repository/using-environment-variables)
