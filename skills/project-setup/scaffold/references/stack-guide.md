# Stack Selection Guide

Detailed guidance for choosing each part of the Better-T-Stack. Read this when users ask about specific options or need help deciding.

## Table of Contents

1. [API Layer: tRPC vs oRPC](#api-layer-trpc-vs-orpc)
2. [Frontend Framework](#frontend-framework)
3. [Backend Framework](#backend-framework)
4. [Database & ORM](#database--orm)
5. [Authentication](#authentication)
6. [Payments](#payments)
7. [Addons](#addons)
8. [Runtime & Package Manager](#runtime--package-manager)
9. [Database Hosting](#database-hosting)
10. [Recommended Combinations](#recommended-combinations)

---

## API Layer: tRPC vs oRPC

This is the most impactful architectural decision after frontend/backend. Both give you end-to-end type safety, but they serve different needs.

### tRPC — battle-tested, massive ecosystem

- **Maturity:** Used in production by Fortune 500 companies (Google, Netflix, PayPal), Cal.com, Langfuse, Mistral
- **Ecosystem:** Rich adapter support (React, Next.js, Express, Fastify, Solid, Svelte, AWS Lambda), extensive documentation, large community
- **DX:** Zero build step, automatic type inference, excellent IDE autocompletion
- **Best for:** TypeScript-only environments where you control both client and server and don't need to expose a REST API to third parties

### oRPC — modern, standards-first

- **OpenAPI native:** Auto-generates OpenAPI docs from your routes — your type-safe RPC endpoints are simultaneously a documented REST API. This is oRPC's killer feature over tRPC.
- **Contract-first option:** Define your API contract before implementation, or go implementation-first — your choice
- **File handling:** Native support for file uploads/downloads (tRPC requires workarounds)
- **Protocol support:** SSE, WebSocket, MessagePort with type safety
- **Native types:** Handles Date, File, Blob, BigInt, URL without serialization config
- **Server Actions:** Works with Next.js and TanStack Start server actions
- **Multi-runtime:** Cloudflare Workers, Deno, Bun, Node.js
- **Observability:** First-class OpenTelemetry integration
- **Schema flexibility:** Works with Zod, Valibot, and ArkType (tRPC is Zod-only)
- **v1 released:** Production-ready as of 2025

### Decision guide

| If you need... | Choose |
|---|---|
| Proven at scale, largest community, most tutorials/examples | **tRPC** |
| OpenAPI docs for external consumers or third-party integrations | **oRPC** |
| File uploads/downloads without workarounds | **oRPC** |
| Contract-first API design | **oRPC** |
| Server Actions (Next.js / TanStack Start) | **oRPC** |
| Multiple schema validators (not just Zod) | **oRPC** |
| Cloudflare Workers deployment | **oRPC** (better multi-runtime) |
| Maximum community support and learning resources | **tRPC** |
| Existing tRPC codebase (migration is possible but has a cost) | **tRPC** |
| Non-React frontend (Nuxt, Svelte, Solid, Astro) | **oRPC** (tRPC not supported) |

### Compatibility constraint

**tRPC only works with React-based frontends:** tanstack-router, react-router, tanstack-start, and next. If the user chose nuxt, svelte, solid, or astro, they must use oRPC or none — tRPC is not an option.

### Recommendation

- **Default to tRPC** for React-based web apps — it's the safe, proven choice with the most resources
- **Choose oRPC** when using non-React frontends (required), need OpenAPI docs, file handling, contract-first design, or plan to deploy to edge runtimes. It's the more modern choice and is production-ready.
- If the user is building an API that external teams or third-party clients will consume, strongly recommend oRPC — OpenAPI support is table stakes for public/shared APIs.

---

## Frontend Framework

### Web frontends

| Option | CLI flag | What it is | Best for |
|---|---|---|---|
| **TanStack Router** | `tanstack-router` | Type-safe client-side router for React SPA | Default choice. SPAs with excellent type safety, search param handling, route-level data loading |
| **React Router** | `react-router` | React Router v7 (evolved from Remix) | Teams familiar with Remix/React Router. Progressive enhancement, nested routes, web standards |
| **TanStack Start** | `tanstack-start` | Full-stack React meta-framework (SSR/SSG) | When you need SSR with TanStack's type safety. Built on TanStack Router + Vinxi. Still maturing (RC stage) |
| **Next.js** | `next` | React meta-framework with App Router | SEO-critical apps, ISR, large ecosystem. Note: brings its own backend — consider `--backend self` |
| **Nuxt** | `nuxt` | Vue meta-framework | Vue ecosystem. Full-featured with auto-imports, file-based routing |
| **SvelteKit** | `svelte` | Svelte meta-framework | Smaller bundles, compiler-driven reactivity, simpler mental model |
| **SolidStart** | `solid` | Solid.js meta-framework | Maximum runtime performance, fine-grained reactivity |
| **Astro** | `astro` | Content-focused meta-framework | Content sites, blogs, docs. Islands architecture, any UI framework |

### Mobile / native frontends

| Option | CLI flag | What it is | Best for |
|---|---|---|---|
| **React Native (bare)** | `native-bare` | Vanilla React Native | Full control, no styling opinions |
| **React Native + NativeWind** | `native-uniwind` | React Native with Tailwind CSS (via NativeWind) | Shared Tailwind knowledge from web, rapid styling |
| **React Native + Unistyles** | `native-unistyles` | React Native with Unistyles | High-performance styling, platform-specific themes |

### Decision guide

- **Building a SPA?** → TanStack Router (best type safety) or React Router (most familiar)
- **Need SSR/SEO?** → TanStack Start (modern, type-safe), Next.js (mature, huge ecosystem), or Nuxt (Vue)
- **Content/docs site?** → Astro (with starlight/fumadocs addon)
- **Mobile app?** → native-uniwind (if you know Tailwind) or native-bare (full control)
- **Performance-critical?** → Solid (runtime) or Svelte (compiler)
- **TanStack Router vs TanStack Start:** Router is client-side SPA only; Start adds SSR, server functions, and streaming. Start is in RC but rapidly stabilizing. Router is stable and production-ready.

---

## Backend Framework

| Option | CLI flag | What it is | Best for |
|---|---|---|---|
| **Hono** | `hono` | Ultrafast, lightweight, multi-runtime | Default. Works on Bun, Node, Cloudflare Workers, Deno. Tiny bundle, fast. |
| **Express** | `express` | Classic Node.js framework | Teams with Express experience, maximum middleware ecosystem |
| **Fastify** | `fastify` | Fast, schema-based Node.js framework | High-throughput APIs, built-in validation and serialization |
| **Elysia** | `elysia` | Bun-native framework | Maximum Bun performance, end-to-end type safety, Eden Treaty |
| **Convex** | `convex` | Managed backend-as-a-service | Real-time apps, rapid prototyping. Replaces database + ORM + backend. No separate DB/ORM needed. |
| **Self** | `self` | Use the frontend's built-in server | Next.js/Nuxt apps where the frontend framework handles API routes |
| **None** | omit flag | No backend | Frontend-only projects |

### Decision guide

- **Default to Hono** — it's fast, lightweight, and runs everywhere
- **Elysia** if you're committed to Bun and want maximum performance with Bun-native features
- **Convex** if you want a managed backend with real-time sync (skip database/ORM selection). Note: incompatible with solid, astro frontends.
- **Express** only if the team has strong Express expertise or needs specific Express middleware
- **Self** only works with meta-frameworks (Next.js, TanStack Start, Nuxt, Astro) — use when the framework's built-in API routes are sufficient

---

## Database & ORM

### Database

| Option | Best for |
|---|---|
| **SQLite** | Default. Local development, prototyping, small-medium apps. Deploy with Turso for production. |
| **PostgreSQL** | Production apps, complex queries, PostGIS, full-text search. Most versatile. |
| **MySQL** | Legacy compatibility, teams with MySQL expertise |
| **MongoDB** | Document-oriented data, flexible schemas. Pairs with Mongoose ORM. |

### ORM

| Option | Best for |
|---|---|
| **Drizzle** | Default. Type-safe, SQL-like syntax, lightweight, excellent migrations. Best DX for SQL databases. |
| **Prisma** | Schema-first approach, auto-generated client, visual studio. More abstraction over SQL. |
| **Mongoose** | MongoDB only. The standard MongoDB ODM. |

### Decision guide

- **SQLite + Drizzle** for getting started fast — zero config, embedded database
- **Postgres + Drizzle** for production apps that need relational data
- **MongoDB + Mongoose** for document-oriented data models
- If user picks **Convex** as backend, skip database/ORM entirely — Convex handles data

---

## Authentication

| Option | Best for |
|---|---|
| **Better Auth** | Default. Self-hosted, open-source, full-featured (social login, 2FA, sessions, email). Integrates with Polar for payments. |
| **Clerk** | Managed auth service. Fastest to integrate, handles UI components, but vendor lock-in. |

### Decision guide

- **Better Auth** for self-hosted, full control, no vendor lock-in, and Polar payments integration
- **Clerk** for rapid prototyping or when you want managed auth UI out of the box. Note: only works with React-based frontends (tanstack-router, react-router, tanstack-start, next). Required: Better Auth if using Polar payments.

---

## Payments

| Option | What it is |
|---|---|
| **Polar** | Developer-friendly payment platform with a Better Auth plugin. Handles checkout, subscriptions, customer portal, usage-based billing. Simpler than Stripe for indie/SaaS. |

Add `--payments polar` when the user is building a SaaS or any product that needs to accept payments. Polar integrates directly with Better Auth — when a user signs up, they're automatically created as a Polar customer.

---

## Addons

Addons extend the scaffold with additional tooling. Here's what each one does and when to recommend it.

### Build orchestration (pick one)

| Addon | What it does | When to use |
|---|---|---|
| **turborepo** | Monorepo task runner with smart caching, parallel execution | Default. Fast builds, standard choice for TS monorepos |
| **nx** | Full-featured monorepo build system with dependency graph | Larger teams, enterprise projects, or when you need fine-grained task orchestration |

### Code quality (pick one set)

| Addon | What it does | When to use |
|---|---|---|
| **biome** | Rust-based linter + formatter (replaces ESLint + Prettier) | Default. 10-100x faster than ESLint, single tool for lint + format |
| **oxlint** | Rust-based linter from the OxC project | Alternative fast linter, pairs well with a separate formatter |
| **ultracite** | Zero-config preset for Biome (and ESLint/Oxlint) with AI-optimized rules | When you want opinionated, zero-config code quality with rules optimized for AI-assisted development |

**Recommendation:** Use **biome** as the default. It replaces both ESLint and Prettier with a single fast tool. Add **ultracite** on top of biome if you want zero-config opinionated rules. Use **oxlint** if you specifically want OxC ecosystem tooling.

### Git hooks (pick one, optional)

| Addon | What it does | When to use |
|---|---|---|
| **lefthook** | Fast, zero-dependency Git hook manager (Go binary) | Recommended if you want git hooks. Fast, simple config |
| **husky** | Popular JS-based Git hook manager | Teams already using Husky, or JS-ecosystem preference |

### Documentation (pick one, optional)

| Addon | What it does | When to use |
|---|---|---|
| **starlight** | Astro-based documentation site generator | Full-featured docs sites, great DX, built on Astro |
| **fumadocs** | Next.js-compatible documentation framework | When your docs need to live in a Next.js ecosystem |

### Platform extensions

| Addon | What it does | When to use |
|---|---|---|
| **pwa** | Progressive Web App support (service worker, manifest) | When users need offline support or installable web apps |
| **tauri** | Desktop app framework (Rust + WebView) | Cross-platform desktop apps. Small binaries, native APIs, good security |
| **electrobun** | Desktop app framework (Bun + native WebView) | Bun-native desktop apps. No Chromium overhead, pure TypeScript |
| **wxt** | Browser extension framework | Building Chrome/Firefox extensions. HMR, Manifest V2/V3, cross-browser |
| **mcp** | Model Context Protocol integration | AI/LLM apps that need to expose tools to AI models |
| **opentui** | Terminal UI library | Building CLI/TUI interfaces for your app |
| **skills** | Claude Code skills scaffolding | Scaffolds skill files for Claude Code plugin development |

### Decision guide by project type

| Building... | Recommended addons |
|---|---|
| **SaaS web app** | turborepo, biome, skills, lefthook |
| **API service** | turborepo, biome, skills |
| **Desktop app** | turborepo, biome, tauri (or electrobun if Bun-only), skills |
| **Mobile app** | turborepo, biome, skills |
| **Browser extension** | turborepo, biome, wxt, skills |
| **AI/LLM product** | turborepo, biome, mcp, skills |
| **Docs site** | turborepo, biome, starlight (or fumadocs), skills |
| **CLI tool** | turborepo, biome, opentui, skills |

---

## Runtime & Package Manager

### Runtime

| Option | Best for |
|---|---|
| **Bun** | Default. Fastest runtime, built-in bundler/test runner, excellent TS support |
| **Node.js** | Maximum compatibility, largest ecosystem, most deployment targets |
| **Cloudflare Workers** | Edge deployment, serverless, global distribution. Requires Hono backend. Incompatible with MongoDB and Docker dbSetup. Pairs well with oRPC. |

### Package manager

| Option | Best for |
|---|---|
| **Bun** | Default. Fastest installs, integrated with Bun runtime |
| **pnpm** | Efficient disk usage, strict dependency resolution. Best Node.js package manager |
| **npm** | Maximum compatibility, simplest setup |

---

## Database Hosting

Match your database hosting to your database choice:

| DB Setup | Database | Best for |
|---|---|---|
| **turso** | SQLite | Production SQLite with edge replication. Recommended for SQLite in production. |
| **d1** | SQLite | Cloudflare D1 — pairs with Workers runtime |
| **neon** | PostgreSQL | Serverless Postgres, scales to zero, branching for dev/preview |
| **supabase** | PostgreSQL | Managed Postgres + auth + realtime + storage. Full BaaS option |
| **planetscale** | MySQL | Serverless MySQL with branching and non-blocking schema changes |
| **mongodb-atlas** | MongoDB | Managed MongoDB with global clusters |
| **prisma-postgres** | PostgreSQL | Prisma's managed Postgres — integrated with Prisma ORM |
| **docker** | PostgreSQL/MySQL | Local development with Docker Compose |

---

## Recommended Combinations

These are opinionated, production-tested stacks for common use cases.

### The Default — SaaS Web App
```
Frontend: tanstack-router | Backend: hono | DB: sqlite | ORM: drizzle
Auth: better-auth | API: trpc | Runtime: bun | Addons: turborepo,biome,skills
```
Why: Fast to start, type-safe end-to-end, proven stack. Graduate to Postgres + Turso/Neon when you need scale.

### The Modern API — Public/Shared APIs
```
Frontend: tanstack-router | Backend: hono | DB: postgres | ORM: drizzle
Auth: better-auth | API: orpc | Runtime: bun | Addons: turborepo,biome,skills
DB setup: neon or docker
```
Why: oRPC gives you type-safe RPC + OpenAPI docs from the same code. Essential when external teams consume your API.

### The SaaS with Payments
```
Frontend: tanstack-router | Backend: hono | DB: postgres | ORM: drizzle
Auth: better-auth | Payments: polar | API: trpc | Runtime: bun
Addons: turborepo,biome,lefthook,skills | DB setup: neon
```
Why: Better Auth + Polar integrate seamlessly for auth + billing. Postgres for production data.

### The Edge Stack — Global, Serverless
```
Frontend: tanstack-router | Backend: hono | DB: sqlite | ORM: drizzle
Auth: better-auth | API: orpc | Runtime: workers
Addons: turborepo,biome,skills | DB setup: turso or d1 | Deploy: cloudflare
```
Why: Hono + oRPC + Workers = edge-native. Turso/D1 for edge-replicated data.

### The Full-Stack SSR
```
Frontend: tanstack-start | Backend: hono | DB: postgres | ORM: drizzle
Auth: better-auth | API: orpc | Runtime: bun | Addons: turborepo,biome,skills
```
Why: TanStack Start for SSR + server actions. oRPC works with TanStack Start server actions natively.

### The Mobile App
```
Frontend: native-uniwind | Backend: hono | DB: postgres | ORM: drizzle
Auth: better-auth | API: orpc | Runtime: bun | Addons: turborepo,biome,skills
DB setup: neon
```
Why: NativeWind for Tailwind-based styling. oRPC for native type support (Date, File, Blob).

### The Desktop App
```
Frontend: tanstack-router | Backend: hono | DB: sqlite | ORM: drizzle
Auth: better-auth | API: trpc | Runtime: bun | Addons: turborepo,biome,tauri,skills
```
Why: Tauri for small, secure desktop binaries. SQLite for embedded data. Alternatively, use electrobun for a Bun-native desktop experience.

### The AI/LLM Product
```
Frontend: tanstack-router | Backend: hono | DB: postgres | ORM: drizzle
Auth: better-auth | API: trpc | Runtime: bun | Examples: ai
Addons: turborepo,biome,mcp,skills | DB setup: neon
```
Why: AI example scaffolds LLM integration. MCP addon for tool exposure to AI models.

### The Browser Extension
```
Frontend: tanstack-router | Backend: hono | DB: sqlite | ORM: drizzle
Auth: better-auth | API: trpc | Runtime: bun | Addons: turborepo,biome,wxt,skills
```
Why: WXT addon scaffolds a production-ready browser extension with HMR and cross-browser support.

### The Vue Stack — Nuxt
```
Frontend: nuxt | Backend: hono | DB: postgres | ORM: drizzle
Auth: better-auth | API: orpc | Runtime: bun | Addons: turborepo,biome,skills
```
Why: Nuxt for Vue ecosystem with SSR. oRPC is required (tRPC incompatible with Nuxt). Postgres for production.

### The Svelte Stack — SvelteKit
```
Frontend: svelte | Backend: hono | DB: postgres | ORM: drizzle
Auth: better-auth | API: orpc | Runtime: bun | Addons: turborepo,biome,skills
```
Why: SvelteKit for smaller bundles and simpler mental model. oRPC required (tRPC incompatible with Svelte).

### Minimal — API Only
```
Backend: hono | DB: postgres | ORM: drizzle | Auth: better-auth
API: orpc | Runtime: bun | Addons: turborepo,biome,skills | DB setup: docker
```
Why: No frontend, just a type-safe API with OpenAPI docs. Perfect for microservices.
