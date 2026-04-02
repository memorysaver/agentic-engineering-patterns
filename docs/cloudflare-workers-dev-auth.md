# Cloudflare workers.dev Auth Cookie Gotcha

## The Problem

When deploying a **Better-T-Stack** app (or any multi-worker app with separate API + Web workers) to Cloudflare using `*.workers.dev` subdomains, **auth cookies don't work across subdomains**.

Example setup:

- Web: `https://www.myapp.workers.dev`
- API: `https://api.myapp.workers.dev`

The user signs in via `POST https://api.myapp.workers.dev/api/auth/sign-in/email`. Better Auth sets a `Set-Cookie` on `api.myapp.workers.dev`. The browser then navigates to a protected page on `www.myapp.workers.dev` — but the auth cookie is **never sent** because it belongs to a different origin.

## Root Cause

**`workers.dev` is in the [Public Suffix List](https://publicsuffix.org/list/public_suffix_list.dat).**

Browsers treat each subdomain under a public suffix as an independent registrable domain. This means:

- `api.myapp.workers.dev` and `www.myapp.workers.dev` are treated as **completely separate sites**
- Setting `domain=myapp.workers.dev` on a cookie is **rejected by the browser** (same as trying to set `domain=.com`)
- Better Auth's `crossSubDomainCookies` config **does not work** on `workers.dev`
- `SameSite=None` doesn't help — the cookie is simply scoped to the wrong origin

This affects **all** public-suffix domains (e.g., `pages.dev`, `vercel.app`, `netlify.app`, `workers.dev`).

## The Fix: Same-Origin Auth

Run Better Auth on the **web worker** (same origin as the browser), not the API worker. The auth cookie stays on the web domain and is sent with every request.

### Architecture Change

```
BEFORE (broken on workers.dev):
┌─────────────────────┐     ┌──────────────────────┐
│ www.myapp.workers.dev│     │ api.myapp.workers.dev │
│  (TanStack Start)   │────▶│  (Hono + oRPC)       │
│                      │     │  + Better Auth        │
│  Cookie: ❌ wrong    │     │  Set-Cookie: ✅       │
│  domain              │     │                       │
└─────────────────────┘     └──────────────────────┘

AFTER (works everywhere):
┌──────────────────────┐     ┌──────────────────────┐
│ www.myapp.workers.dev │     │ api.myapp.workers.dev │
│  (TanStack Start)    │     │  (Hono + oRPC)       │
│  + Better Auth ✅     │────▶│  + Better Auth       │
│                       │     │  (still handles API  │
│  Cookie: ✅ same      │     │   auth for curl/m2m) │
│  origin               │     │                      │
└──────────────────────┘     └──────────────────────┘
```

### Implementation (3 files)

#### 1. `apps/web/src/server.ts` — Worker entry with auth handler

Alchemy's TanStack Start plugin auto-detects `src/server.ts` and uses it as the worker entry point. This intercepts `/api/auth/*` before TanStack Start handles the request.

```ts
import { createAuth } from "@myapp/auth";
import serverEntry from "@tanstack/react-start/server-entry";

export default {
  async fetch(request: Request, env: unknown, ctx: ExecutionContext) {
    const url = new URL(request.url);

    if (url.pathname.startsWith("/api/auth")) {
      return createAuth().handler(request);
    }

    return serverEntry.fetch(request, env, ctx);
  },
};
```

#### 2. `apps/web/src/lib/auth-client.ts` — Same-origin base URL

```ts
import { createAuthClient } from "better-auth/react";

export const authClient = createAuthClient({
  // Same origin = cookies work on any domain (workers.dev, custom, localhost)
  baseURL: typeof window !== "undefined" ? window.location.origin : "http://localhost:3051",
});
```

#### 3. `apps/web/src/middleware/auth.ts` — Direct server-side session check

Instead of making an HTTP round-trip via the auth client, call the auth API directly in SSR:

```ts
import { createAuth } from "@myapp/auth";
import { createMiddleware } from "@tanstack/react-start";

export const authMiddleware = createMiddleware().server(async ({ next, request }) => {
  const session = await createAuth().api.getSession({
    headers: request.headers,
  });
  return next({ context: { session } });
});
```

### Required Bindings

The web worker needs these bindings in `alchemy.run.ts` (it already has them if you followed Better-T-Stack conventions):

```ts
export const web = await TanStackStart("web", {
  bindings: {
    DB: db, // D1 database
    BETTER_AUTH_SECRET: alchemy.secret.env.BETTER_AUTH_SECRET!,
    BETTER_AUTH_URL: alchemy.env.CORS_ORIGIN!, // ← points to web URL, not API
    CORS_ORIGIN: alchemy.env.CORS_ORIGIN!,
    // ... other bindings
  },
});
```

Note: `BETTER_AUTH_URL` for the web worker should point to the **web worker's own URL** (the CORS_ORIGIN), not the API server.

## When Does This NOT Apply?

- **Custom domains**: If you use a custom domain (e.g., `app.mycompany.com` + `api.mycompany.com`), cross-subdomain cookies work normally via `domain=.mycompany.com`. Better Auth's `crossSubDomainCookies` config works fine.
- **Single-worker apps**: If your API and web are in the same worker, cookies are same-origin by default.
- **API-only (no SSR)**: If your frontend is a pure SPA on a CDN making direct API calls, you can use `credentials: 'include'` with the API cookie — but TanStack Start SSR still won't have access to it.

## Checklist

- [ ] `apps/web/src/server.ts` exists and intercepts `/api/auth/*`
- [ ] Auth client uses `window.location.origin` (not the API URL)
- [ ] Auth middleware calls `createAuth().api.getSession()` directly (not via HTTP)
- [ ] Web worker has `DB` + `BETTER_AUTH_SECRET` + `BETTER_AUTH_URL` bindings
- [ ] `BETTER_AUTH_URL` on the web worker points to the web URL, not the API URL
- [ ] API worker still has its own Better Auth (for curl/M2M/webhook auth)

## Related

- [Public Suffix List — workers.dev entry](https://publicsuffix.org/list/public_suffix_list.dat)
- [Better Auth — Cross Subdomain Cookies](https://www.better-auth.com/docs/concepts/cookies#cross-subdomain-cookies)
- [Cloudflare — workers.dev subdomain](https://developers.cloudflare.com/workers/configuration/routing/workers-dev/)
