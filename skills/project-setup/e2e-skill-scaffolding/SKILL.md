---
name: aep-e2e-skill-scaffolding
description: Generate (or upgrade) a project's e2e-test skill into the BDD layer-gate three-tier shape. Use when setting up E2E testing for a project, scaffolding the e2e-test skill, adding a BDD journey library, migrating a thin/bash e2e-test skill to BDD, or when the user says "scaffold e2e", "set up e2e testing", "add BDD journeys", "upgrade the e2e-test skill". Delegated to by /aep-scaffold. Emits a canonical, cross-tool skill (Claude Code + Codex + Pi) with a separate test-tool-selection guide.
---

# E2E Skill Scaffolding

Generate a project-local **`e2e-test` skill** in the BDD + layer-gate **three-tier** shape — the pattern
proven in production downstream. The generated skill documents tests in natural-language **journeys**
(intent, not click scripts), maps each to a **layer gate**, and ships a **separate tool-selection guide**
so the actual browser/device automation tool is detected per environment or pinned by preference.

**Where this fits:**

```
/aep-onboard → /aep-scaffold ──delegates──► /aep-e2e-skill-scaffolding
                                                   │ emits skills/e2e-test/ (canonical)
                   [ /aep-design → /aep-launch → /aep-build → /aep-wrap ]
                                                   ▲ Phase 6-8 run the journeys, flip layer gates
```

`/aep-scaffold` creates the project and calls this skill for the e2e infrastructure. `/aep-build` runs
the journeys (the manual half of each layer gate). This skill is **idempotent** — re-run it to upgrade a
project or refresh after the standard evolves.

**Input:** the project's stack (`package.json` / `bts.jsonc`), optional `product-context.yaml`.
**Output:** real `skills/e2e-test/` + `.claude/skills/e2e-test` + `.agents/skills/e2e-test` symlinks.

---

## When this skill applies

- A new project needs its e2e-test skill (called by `/aep-scaffold` Phase 8).
- An existing project has **no** e2e-test skill, or a **thin/bash** one to upgrade to BDD.
- The user wants to add a BDD journey library or the separate tool-selection guidance.

For the conceptual model (why three tiers, how journeys map to gates), read
[`references/three-tier-model.md`](references/three-tier-model.md) and
[`references/layer-gate-loop.md`](references/layer-gate-loop.md). For journey authoring conventions, read
[`references/bdd-journeys.md`](references/bdd-journeys.md).

---

## Canonical cross-tool placement

The generated skill is **project-owned** (not installed from the AEP marketplace), so it must be placed
canonically by hand. The verified layout that makes one skill visible to **Claude Code, Codex, and Pi**:

```
skills/e2e-test/                              # REAL dir — source of truth, tracked in git as itself
.claude/skills/e2e-test → ../../skills/e2e-test   # symlink (Claude Code discovery)
.agents/skills/e2e-test → ../../skills/e2e-test   # symlink (Codex / Pi discovery)
```

This is independent of how the AEP `aep-*` packages are installed (the skills CLI manages those). No
migration script and no user-level skill is required — Phase 4 creates the two symlinks directly.

---

## Phase 1: Detect state

```bash
REAL="skills/e2e-test"
LEGACY=".claude/skills/e2e-test"   # the old Claude-only placement

if [ -d "$REAL" ] && { [ -f "$REAL/policy.md" ] || [ -f "$REAL/journeys/README.md" ]; }; then echo "state: canonical (upgrade in place)";
elif [ -d "$REAL" ]; then echo "state: real-non-bdd (upgrade)";
elif [ -d "$LEGACY" ] && [ ! -L "$LEGACY" ]; then echo "state: thin-legacy (migrate $LEGACY → $REAL, then upgrade)";
else echo "state: absent (create fresh)"; fi
```

- **absent** → create fresh (Phases 2–6).
- **thin-legacy** → `mkdir -p skills && git mv .claude/skills/e2e-test skills/e2e-test` first (preserve
  history — `git mv` fails if the destination parent `skills/` doesn't exist yet, which is always the case
  in this state). Never delete a real dir without moving its content.
- **real-non-bdd / canonical** → upgrade in place. **Never overwrite hand-written journeys** or silently
  rewrite `policy.md` — re-confirm the policy (Phase 2), then add only the missing scaffold files
  (`policy.md`, `journeys/README.md` + `tool-selection.md` when the target ≠ `none`,
  `00-walking-skeleton.md` if no journeys exist, `layer-gate-evidence.template.md`) and reconcile
  `SKILL.md`.

---

## Phase 2: Resolve stack

Read the stack to fill template placeholders. Reuse the `/aep-scaffold` **Default Tooling** table:

| Placeholder        | Source                                                               | Default                 |
| ------------------ | -------------------------------------------------------------------- | ----------------------- |
| `{{PROJECT_NAME}}` | `package.json` `name` / repo dir                                     | repo dir name           |
| `{{PKG_MANAGER}}`  | lockfile (`bun.lockb`/`pnpm-lock.yaml`/`uv.lock`…)                   | `bun`                   |
| `{{TEST_RUNNER}}`  | stack (TS→vitest, Py→pytest, Rust→cargo test, Go→go test)            | `vitest`                |
| `{{DEV_SERVER}}`   | scripts (`bun run dev` / `uv run dev` …)                             | `bun run dev`           |
| `{{BASE_URL}}`     | `.dev-workflow/ports.env` contract                                   | `http://localhost:3001` |
| `{{SERVER_URL}}`   | `.dev-workflow/ports.env` contract                                   | `http://localhost:3000` |
| `{{TARGET_TYPE}}`  | frontend (native-uniwind→mobile, tauri/electrobun→desktop, else web) | `web`                   |

### E2E policy — **propose, then confirm with the user**

The policy (which tiers gate a layer, where to dogfood, when) is a **per-project decision, not a default
to assume** — a CLI tool needs no Cloudflare check; a pre-release web app may dogfood post-deploy against
prod. Propose from the stack, **then ask the user to confirm/adjust** before rendering `policy.md`:

| Placeholder              | Propose from                                                                                                             | Confirm? |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------ | -------- |
| `{{E2E_TIERS}}`          | project type → tier table (`references/three-tier-model.md`): CLI/lib `[1]`, API `[1,3]`, web/mobile `[1,2,3]`           | **yes**  |
| `{{E2E_TARGET}}`         | no web frontend → `none`; web frontend + deploy config (`wrangler.*`, `vercel.*`) → offer `deployed:<url>`; else `local` | **yes**  |
| `{{E2E_JOURNEY_TIMING}}` | `local` target → `pre-merge`; `deployed:<url>` → `post-deploy`                                                           | **yes**  |

Ask plainly, e.g. _"This looks like a {type}. Proposed e2e policy: tiers `{tiers}`, dogfood target
`{target}`, timing `{timing}`. Keep, or adjust (e.g. dogfood against a deployed Cloudflare URL
post-merge)?"_ Record the confirmed values; they fill `policy.md` and decide what Phase 3 emits.

---

## Phase 3: Emit the skill

Render each `templates/*.tmpl` with the Phase 2 substitutions into real `skills/e2e-test/`:

```
skills/e2e-test/
├── SKILL.md                        ← templates/e2e-test.SKILL.md.tmpl
├── policy.md                       ← templates/policy.md.tmpl  (E2E_TIERS/TARGET/TIMING from Phase 2)
├── journeys/                       ← omit entirely when E2E_TARGET == none (Tier-2 N/A)
│   ├── README.md                   ← templates/journeys-README.md.tmpl
│   └── 00-walking-skeleton.md      ← templates/journey-00-walking-skeleton.md.tmpl
├── tool-selection.md               ← templates/tool-selection.md.tmpl  (omit when E2E_TARGET == none)
├── layer-gate-evidence.template.md ← templates/layer-gate-evidence.md.tmpl
└── scripts/
    └── seed.sh                     ← templates/seed.sh.tmpl   (chmod +x)
```

**Conditional on the policy:** when `E2E_TARGET == none` (CLI/library, `[1]`-only), there is no UI to
dogfood — **skip `journeys/` and `tool-selection.md`**; the gate is Tier-1 (+ Tier-3) + coverage, and
`policy.md` records why. When journeys _are_ emitted, the walking skeleton's `target:` is `{{TARGET_TYPE}}`.

On **upgrade**, write only files that are absent; for `SKILL.md` present-but-thin, replace it (it's
generated infrastructure docs, not hand-authored journeys) and tell the user. **`policy.md` is never
silently overwritten** — read the existing one, re-confirm with the user (Phase 2), then update it.

```bash
chmod +x skills/e2e-test/scripts/seed.sh
```

---

## Phase 4: Expose canonical (the two symlinks)

Idempotent and gotcha-safe — skip a correct symlink, fix a wrong one, refuse to clobber a real path
(Phase 1 already migrated the legacy real dir):

```bash
expose() {                       # $1 = skill name (e2e-test)
  local name="$1"
  # Precondition: the real source must exist, else every symlink we create/keep is dangling.
  [ -d "skills/$name" ] || { echo "REFUSE: skills/$name missing — emit the real skill dir (Phase 3) first"; return 1; }
  for rt in .claude .agents; do
    mkdir -p "$rt/skills"
    local link="$rt/skills/$name"
    if [ -L "$link" ] && [ -e "$link" ]; then
      [ "$(readlink "$link")" = "../../skills/$name" ] || { rm -f "$link"; ( cd "$rt/skills" && ln -s "../../skills/$name" "$name" ); }
    elif [ -L "$link" ]; then           # dangling symlink — repoint it at the (now-present) source
      rm -f "$link"; ( cd "$rt/skills" && ln -s "../../skills/$name" "$name" )
    elif [ -e "$link" ]; then
      echo "REFUSE: $link is a real path — migrate it into skills/ first"; return 1
    else
      ( cd "$rt/skills" && ln -s "../../skills/$name" "$name" )
    fi
  done
}
expose e2e-test
```

Verify both resolve to the same source:

```bash
readlink -f .claude/skills/e2e-test   # → …/skills/e2e-test
readlink -f .agents/skills/e2e-test   # → …/skills/e2e-test
```

---

## Phase 5: Wire layer gates

The gate is **two-phase + covered** — `scripted_passed` (Tier-1 green) → `passed` (all applicable tiers
green + every acceptance criterion proven + prior-layer journeys replay). See
[`references/layer-gate-loop.md`](references/layer-gate-loop.md) and
[`references/three-tier-model.md`](references/three-tier-model.md).

- If `product-context.yaml` exists:
  - Ensure each `layer_gates[]` entry uses the **canonical enriched shape** — `status` (including
    `scripted_passed`), a `coverage` block (`criteria_total` / `criteria_covered` / `uncovered`), and
    structured `evidence` (`scripted` / `journeys` / `matrix`). Don't rewrite existing gate state; just
    add the missing `coverage` / `evidence` keys (back-compat: older gates without them still parse).
  - Ensure `docs/layer-gates/` exists; seed `docs/layer-gates/0.md` from
    `skills/e2e-test/layer-gate-evidence.template.md` if absent (never overwrite a filled-in one).
- If not (standalone project): journeys still organize by layer; the gate is a manual checkbox in
  `docs/layer-gates/<layer>.md` (copy the evidence template). The loop works without the YAML state
  machine — the two matrices + checklist are the record.

---

## Phase 6: Verify + report

```bash
# Core files (always emitted):
test -f skills/e2e-test/SKILL.md && test -f skills/e2e-test/policy.md \
  && test -f skills/e2e-test/layer-gate-evidence.template.md \
  && test -x skills/e2e-test/scripts/seed.sh && echo "core files OK"
# Journey tier (only when E2E_TARGET != none):
if [ -d skills/e2e-test/journeys ]; then
  test -f skills/e2e-test/journeys/README.md && test -f skills/e2e-test/tool-selection.md \
    && echo "journey tier OK"
fi
# Syntax-check only — do NOT run seed.sh here: at scaffold time there is no dev server, so a full run
# would block in its wait loop. seed.sh runs for real later via the workspace hook once the server is up.
bash -n skills/e2e-test/scripts/seed.sh && echo "seed.sh syntax OK"
```

Report to the user: what was created vs. upgraded, the canonical symlinks, the **confirmed e2e policy**
(`policy.md`: tiers / target / timing), the chosen `{{TARGET_TYPE}}` and tool track, and the next step
(write the Layer-0 journey's project specifics, then run it via `/aep-build` Phase 6 — or, for a
`none`-target project, prove Layer-0 criteria via Tier-1/Tier-3).

---

## Guardrails

- **Never overwrite hand-written journeys.** Upgrades add missing scaffold; they don't replace authored content.
- **Never clobber a real `.claude/skills/e2e-test` dir** — migrate it into `skills/` (git mv) first.
- **Real dir is the source of truth**; `.claude/skills` and `.agents/skills` entries are symlinks only.
- **seed.sh must stay idempotent** and exit 0 on a fresh project (no project-specific seeding yet).
- **Tool choice is never hard-coded in journeys** — journeys are tool-agnostic; the tool is resolved by
  `tool-selection.md`.
- **`policy.md` is the single source of truth for tiers / target / timing** — propose from the stack but
  **confirm with the user**; never silently overwrite it on re-run (re-confirm, then update). No copy goes
  in `AGENTS.md` — the skill is canonical cross-tool, so all runtimes read this one file.

## Next step

| Command          | What it does                                                                |
| ---------------- | --------------------------------------------------------------------------- |
| `/aep-build`     | Phases 6–8 run the journey, record evidence, flip the layer gate            |
| `/aep-dispatch`  | Blocks the next layer until the prior layer gate passes                     |
| edit `journeys/` | Add a journey per new capability/layer (copy `journeys/README.md` template) |
