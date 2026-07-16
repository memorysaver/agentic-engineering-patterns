# Resulting Project Structure

What a scaffolded project looks like after `/aep-scaffold` completes (new-project flow, Phases 1–8).
Read this when you want to confirm the final layout or explain it to the user.

The `skills/e2e-test/` sub-layout (real dir + the two cross-tool symlinks) is **owned by
`/aep-e2e-skill-scaffolding`** — it is the canonical home for that cross-tool shape; the tree below shows
it in context only.

```
<project>/
├── skills/
│   └── e2e-test/                # REAL dir — canonical, BDD layer-gate e2e (cross-tool)
│       ├── SKILL.md  ├── policy.md  └── scripts/seed.sh
│       │   # policy.md owns tiers/target/timing + live_policy (every_gate |
│       │   # milestone_gates_only | none), sensitive_paths, preflight probes,
│       │   # and the deterministic security gates — single source of truth
│       └── journeys/ + tool-selection.md   # only when dogfood_target ≠ none (emitted for cli too; omitted only when no runnable surface)
├── .claude/
│   ├── hooks/
│   │   └── workspace-setup.sh    # Project-specific workspace init
│   ├── skills/
│   │   ├── e2e-test → ../../skills/e2e-test   # symlink (Claude Code)
│   │   └── openspec-*/           # OpenSpec skills
│   └── commands/opsx/            # OpenSpec command aliases
├── .agents/
│   └── skills/
│       └── e2e-test → ../../skills/e2e-test   # symlink (Codex / Pi)
├── apps/
│   ├── web/                      # Frontend (TanStack/React/Next/etc.)
│   └── server/                   # Backend (Hono/Express/etc.)
├── packages/
│   ├── config/                   # Shared TypeScript/lint config
│   ├── ui/                       # Shared UI components (shadcn/ui)
│   ├── db/                       # Database schema + migrations
│   ├── auth/                     # Auth configuration
│   ├── api/                      # API layer (tRPC/oRPC router)
│   └── env/                      # Shared environment variables
├── openspec/                     # Spec-driven development
├── bts.jsonc                     # Better-T-Stack project config
├── turbo.json                    # Turborepo pipeline config
└── package.json                  # Root workspace config
```
