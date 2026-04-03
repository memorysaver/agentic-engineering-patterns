# Design Calibration Workflow

> Improvement proposal for `.5` polish layers (2026-04-03).
> Operationalizes the UI Polish Layer pattern from `aep-v2-lesson-learning.md`.
> Introduces the `/calibrate` skill and `design-context.yaml` artifact.

---

## Problem

The AEP v2 lesson learning identifies `.5` polish layers as the structural fix for agent-generated UI: agents build **functional** UI but cannot judge **visual design quality**. Layer 0.5 exists as a concept with 3 stories (landing page, auth pages, dashboard detail), but the workflow has no defined process for HOW the human explores and captures design direction.

The gap is specific:

1. No mechanism to extract a design brief from `product-context.yaml`
2. No workflow for human-driven design exploration using external tools
3. No artifact format for capturing design decisions in a way agents can consume
4. No integration point for feeding design context into `/dispatch` → `/launch` → `/build`

Without this, `.5` layer stories get dispatched like any other story — agents implement them with no visual direction, reproducing the same generic UI that created the need for polish in the first place.

---

## Solution: `/calibrate` Skill

A new skill with two phases that sits between `/reflect` (which identified the UI gap) and `/dispatch` (which sends stories to agents):

```
/reflect (identified UI gap, created .5 layer stories)
  → /calibrate           (Phase 1: generate design brief)
  → human explores       (external: Stitch, Pencil.dev)
  → /calibrate capture   (Phase 2: capture design decisions)
  → /dispatch            (dispatches .5 stories with design context)
```

### Why a New Skill

| Existing skill | Why it doesn't fit                                                                 |
| -------------- | ---------------------------------------------------------------------------------- |
| `/dispatch`    | Scoring/routing machine. Adding design exploration violates single responsibility. |
| `/map`         | Decomposes architecture, not visual design.                                        |
| `/design`      | Handles functional specs via OpenSpec. Not visual direction.                       |
| `/reflect`     | Reviews what happened. Doesn't generate forward-looking design briefs.             |

The "generate prompt → human explores externally → human returns with results" pattern has no precedent in the current skill set. It requires its own skill.

---

## Phase 1: Generate Design Brief (`/calibrate`)

### What It Does

1. **Read** `product-context.yaml` — extract opportunity, persona, JTBD, constraints, activities
2. **Read** current `.5` layer stories — extract pages to design with purposes
3. **Scan** existing UI state:
   - `packages/ui/src/styles/globals.css` → current theme tokens (color space, font, radius)
   - `packages/ui/src/components/` → available shadcn components
   - `apps/web/src/routes/` → existing pages
4. **Generate** `docs/design-brief.md` — one structured markdown prompt
5. **Output** the brief to terminal for copy-paste to design tools

### Design Brief Template

```markdown
# Design Brief: [project name]

## Product Identity

- **What:** [from opportunity.bet + product.problem]
- **For whom:** [from product.persona.description]
- **Core job:** [from product.persona.jtbd]
- **Why now:** [from opportunity.why_now]

## Pages to Design

### 1. [Page name] ([route])

- **Purpose:** [from story description]
- **Content blocks:** [inferred from story acceptance criteria]
- **Key interactions:** [from activity definition]

### 2. [Page name] ([route])

[...]

## Design Directions

### Direction A: "[Name]"

[Description of the visual direction — mood, color family, layout approach]
**Reference products:** [2-3 existing products as mood board anchors]
**Strengths:** [what this direction does well for the persona]
**Risks:** [where it could miss]

### Direction B: "[Name]"

[...]

### Direction C: "[Name]"

[...]

## Technical Constraints

- **Stack:** [from product.constraints.required_stack + preferred_stack]
- **Current theme:** [extracted from globals.css — color space, font, radius, etc.]
- **Available components:** [list from packages/ui/src/components/]
- **Requirements:** Responsive, light + dark mode

## Deliverable

Explore the directions. Produce designs for all pages listed above.
The goal is to establish: color palette, typography, component styling,
layout patterns, and overall brand feel for the product.
```

### Extraction Map

| Brief section        | `product-context.yaml` source                            |
| -------------------- | -------------------------------------------------------- |
| What                 | `opportunity.bet` + `product.problem`                    |
| For whom             | `product.persona.description`                            |
| Core job             | `product.persona.jtbd`                                   |
| Why now              | `opportunity.why_now`                                    |
| Pages                | `stories` where `layer` = active `.5` layer              |
| Stack constraints    | `product.constraints.required_stack` + `preferred_stack` |
| Current theme        | File scan: `packages/ui/src/styles/globals.css`          |
| Available components | Directory scan: `packages/ui/src/components/`            |

### Design Direction Generation

Directions are generated based on the product's persona and category. The skill should:

1. **Span a spectrum** from "maximum technical" to "maximum approachable"
2. **Name 2-3 reference products** per direction as visual mood board anchors (real products the human can look at for inspiration)
3. **Note strengths and risks** for each direction relative to the target persona
4. **Suggest 3 directions** — the human may choose one, hybridize, or ignore all three

For a developer tool like Looplia (terminal-adjacent, security-conscious audience), example directions might span:

| Direction         | Mood                                                     | References                 |
| ----------------- | -------------------------------------------------------- | -------------------------- |
| "Terminal Native" | Dark, monospace accents, green/amber terminal colors     | Warp, Ghostty, iTerm       |
| "Modern Dev Tool" | Clean light/dark toggle, blue-purple accents, sans-serif | Linear, Vercel, Supabase   |
| "Warm Technical"  | Warm neutrals, generous whitespace, subtle color         | Tailscale, Render, Railway |

### Format: One Structured Markdown

A single structured markdown document works for both Google Stitch and Pencil.dev. No tool-specific rewrites needed — both tools handle structured markdown well. The human copies the full brief or relevant sections into the tool.

---

## Human Design Exploration (External)

This phase is explicitly **out of the agent's hands**. After generating the brief, the skill outputs:

```
Design brief written to: docs/design-brief.md

Next steps (you do these):

  1. Copy the brief to a vibe design tool:
     - Google Stitch (stitch.withgoogle.com) — free, Figma/HTML/CSS export
     - Pencil.dev — IDE-native (VS Code/Cursor), .pen files in Git

  2. Try each direction. Generate at least 2 variations per direction.

  3. Pick what feels right. Save reference files to docs/design-references/

  4. When ready, come back and run:
     /calibrate capture
```

### Vibe Design Tool Reference

#### Google Stitch

- **What:** AI design tool from Google Labs, powered by Gemini
- **Input:** Text prompts (natural language or structured markdown), hand-drawn sketches, wireframes, screenshots, voice
- **Output:** Interactive UI mockups, Figma files, HTML/CSS code
- **Export:** Direct to Figma or HTML/CSS
- **Cost:** Free
- **Best for:** Rapid exploration of multiple directions, visual brainstorming
- **Workflow:** Paste design brief → generate designs → iterate via conversation → export HTML/CSS or Figma for chosen direction

#### Pencil.dev

- **What:** AI design tool that runs inside VS Code / Cursor
- **Input:** Text prompts, Figma imports, manual canvas editing
- **Output:** `.pen` JSON design files (stored in `/design` folder), pixel-perfect React/HTML/CSS code
- **Export:** React code, HTML/CSS, committed directly to Git
- **Cost:** Free in early access (requires Claude Code subscription)
- **Best for:** Design-to-code integration, Git-native design files, IDE workflow
- **Workflow:** Open Pencil in IDE → paste brief → design on canvas → `.pen` files saved to repo → Claude Code generates code from specs

#### Other Tools

- **Galileo AI** (now part of Stitch) — text-to-UI, Figma export
- **Uizard** — fast browser-based prototyping, interactive prototypes
- **Banani** — generates style variations (minimalist, enterprise, playful, sleek)
- **Framer AI** — text-to-website, complete layouts with interactions

### What to Save

After exploring, the human saves reference files to `docs/design-references/`:

```
docs/design-references/
├── landing.html          ← HTML/CSS from Stitch (or screenshot)
├── auth.html             ← HTML/CSS from Stitch
├── dashboard.html        ← HTML/CSS from Stitch
├── landing.png           ← Screenshot alternative
└── notes.md              ← Human notes on what they liked and why
```

These files are committed to Git and referenced in `design-context.yaml`. Agents read them as visual guidance — they translate the reference into the project's component system, not copy code verbatim.

---

## Phase 2: Capture Design Decisions (`/calibrate capture`)

### Interactive Q&A

The skill asks structured questions:

1. **Direction:** Which direction did you choose? (or describe the hybrid)
2. **Palette:** Primary, secondary, accent, destructive, muted colors (any format — hex, rgb, hsl — the skill converts to oklch for `globals.css`)
3. **Typography:** Font family changes? Size scale adjustments? Weight usage patterns?
4. **Components:** Recurring patterns observed? Card styles, button variants, spacing rhythm?
5. **Layout:** Max-width, grid vs flex, sidebar vs top-nav, content density?
6. **Brand signals:** What visual cues communicate "this is [product name]"?
7. **Reference files:** Did you save HTML/CSS/screenshots? Where?

### Artifact: `design-context.yaml`

Stored at **repo root** alongside `product-context.yaml`. This is the authoritative design reference for all future UI work.

```yaml
schema: v1
project: looplia-run
calibrated_at: "2026-04-03"
calibrated_from_layer: 0.5

direction:
  name: "[chosen direction name]"
  description: "[1-2 sentence description of the visual direction]"
  references: ["Product A", "Product B", "Product C"]

palette:
  light:
    background: "oklch(...)"
    foreground: "oklch(...)"
    primary: "oklch(...)"
    primary-foreground: "oklch(...)"
    secondary: "oklch(...)"
    secondary-foreground: "oklch(...)"
    accent: "oklch(...)"
    accent-foreground: "oklch(...)"
    muted: "oklch(...)"
    muted-foreground: "oklch(...)"
    destructive: "oklch(...)"
    border: "oklch(...)"
    ring: "oklch(...)"
  dark:
    background: "oklch(...)"
    foreground: "oklch(...)"
    # ... same structure as light

typography:
  font_family_sans: "Inter Variable"
  font_family_mono: "JetBrains Mono"
  heading_weight: 600
  body_weight: 400
  scale_notes: "Default Tailwind scale, no adjustments"

spacing:
  rhythm: "4px"
  page_padding: "1.5rem"
  section_gap: "2rem"
  card_padding: "1.5rem"

layout:
  max_width: "1200px"
  nav_style: "top"
  default_mode: "dark"

components:
  border_radius: "0.625rem"
  button:
    style_notes: "[description of button treatment]"
  card:
    style_notes: "[description of card treatment]"
  input:
    style_notes: "[description of input treatment]"

brand:
  tagline: "[product tagline]"
  visual_signals:
    - "[signal 1]"
    - "[signal 2]"
    - "[signal 3]"

reference_designs:
  - page: "landing"
    file_path: "docs/design-references/landing.html"
    notes: "[what makes this design work]"
  - page: "auth"
    file_path: "docs/design-references/auth.html"
    notes: "[...]"
  - page: "dashboard"
    file_path: "docs/design-references/dashboard.html"
    notes: "[...]"
```

### Why YAML

- **Machine-readable** — agents dispatched for UI stories query specific tokens (`palette.dark.primary`, `components.border_radius`). Markdown is readable but not queryable.
- **Convention match** — same format as `product-context.yaml`, consistent with the rest of AEP.
- **Structured sections** — palette, typography, spacing, layout, components, brand are all distinct concerns. YAML separates them cleanly.

### Companion: Update `globals.css`

Phase 2 also produces an updated `globals.css` with the chosen palette. The CSS is the **live implementation** — `design-context.yaml` documents the decisions (why), `globals.css` enacts them (what), and reference files show the visual target (how it looks).

The skill reads the captured palette values, converts to oklch if needed, and writes them as CSS custom properties in `globals.css` under `:root` and `.dark` selectors.

### Commit

After producing both artifacts:

```bash
# Commit design artifacts
jj describe -m "feat: establish design system via /calibrate — [direction name]"
jj new
jj git push --change @-
```

---

## Skill Integration Points

### `/dispatch` — Step 6: Context Assembly

**Current:** Part 2 (Story-Specific Payload) assembles module definition, adjacent interfaces, and dependency outputs.

**Change:** Add a conditional block after the existing Part 2 assembly:

```
If story.layer is a .5 layer (e.g., 0.5, 1.5, 2.5)
   OR story has design_context: true:

  Include in context package:
    1. Full design-context.yaml
    2. Reference design files from docs/design-references/
       matching the story's page (by story.activity or title)
    3. Design constraint directive:
       "This story is part of a visual design calibration layer.
        Follow the design system in design-context.yaml strictly.
        Use ONLY colors, fonts, and spacing defined there.
        Reference docs/design-references/ for layout guidance.
        Do not introduce new visual tokens not in design-context.yaml."
```

This is a surgical addition to the existing context assembly rules. The stable prefix (Part 1) and retrieval instructions (Part 3) remain unchanged.

### `/launch` — Bootstrap Prompt

**Current:** Bootstrap sends `/build execute implementation for openspec change <name>`.

**Change:** When launching a `.5` layer story, prepend design context to the bootstrap message:

```
Before sending the bootstrap prompt, if the story belongs to a .5 layer:
  1. Verify design-context.yaml exists at repo root
  2. If missing, ABORT — run /calibrate first
  3. If present, the context is already assembled by /dispatch (Step 6)
     No additional launch-time action needed
```

The design context flows through the standard context assembly pipeline in `/dispatch`. `/launch` only needs to verify the prerequisite exists.

### `/map` — Step 3: Dependency Resolution & Execution Slices

**Current:** Groups stories into layers and execution slices. Layer gates defined per layer.

**Change:** Add guidance after layer planning:

```
After defining each implementation layer, if the layer contains
UI-facing stories (stories where module = web, or activity involves
user-facing pages):

  Consider adding a .5 calibration layer after it.

  - Layer 0.5 (first .5 layer): Establishes the design system.
    Run /calibrate before dispatching .5 stories.
    Stories in this layer implement the visual direction.

  - Layer 1.5, 2.5 (subsequent .5 layers): Extend the design system.
    /calibrate detects existing design-context.yaml and generates
    a focused brief for NEW UI patterns only (e.g., data tables,
    terminal UI, settings pages).

  .5 layers are opt-in, not automatic. The /reflect step after each
  layer explicitly asks: "Does this layer need a UI polish pass?"
```

---

## Reusability Across Layers

The `/calibrate` skill detects whether `design-context.yaml` exists to choose its mode:

### First Run (Layer 0.5) — Establishment Mode

- Full design brief: all pages, 3 directions, complete palette + typography + layout
- Produces `design-context.yaml` from scratch
- Updates `globals.css` with complete new palette
- **Goal:** Establish the visual design principles for the product

### Subsequent Runs (Layer 1.5, 2.5) — Extension Mode

- Reads existing `design-context.yaml`
- Identifies NEW UI patterns needed by the current `.5` layer's stories that aren't covered by existing `components` section
- Generates a focused brief: "Here's your current design system. These new pages need patterns not yet covered: [list]. Explore additions."
- Human explores only the delta
- Capture updates `design-context.yaml` incrementally (adds new component patterns, new reference designs) rather than replacing

```
Layer 0.5 — Establishes:
  palette, typography, spacing, layout, card/button/input patterns,
  brand signals, landing/auth/dashboard references

Layer 1.5 — Extends with:
  data table pattern (audit log viewer)
  form pattern (guardrails configuration)
  status visualization (heartbeat display)
  audit-viewer and guardrails-page reference designs

Layer 2.5 — Extends with:
  terminal UI pattern (xterm.js integration)
  session management pattern (list/resume/disconnect)
  web-terminal and session-management reference designs
```

---

## Workflow Summary

```
┌──────────────────────────────────────────────┐
│ Layer N complete → /reflect identifies UI gap │
│ Creates .5 layer stories in product-context   │
└──────────────────┬───────────────────────────┘
                   │
                   ▼
┌──────────────────────────────────────────────┐
│ /calibrate                                    │
│  • Reads product-context.yaml                 │
│  • Scans existing UI state                    │
│  • Generates docs/design-brief.md             │
│  • Outputs brief for copy-paste               │
│  • If design-context.yaml exists → extension  │
│    mode (brief covers only new patterns)      │
└──────────────────┬───────────────────────────┘
                   │
                   ▼
┌──────────────────────────────────────────────┐
│ Human explores externally (hours/days)        │
│  • Pastes brief into Stitch / Pencil / other  │
│  • Generates variations per direction         │
│  • Reviews visually, picks what feels right   │
│  • Saves references to docs/design-references/│
└──────────────────┬───────────────────────────┘
                   │
                   ▼
┌──────────────────────────────────────────────┐
│ /calibrate capture                            │
│  • Interactive Q&A about chosen direction     │
│  • Ingests reference files                    │
│  • Produces/updates design-context.yaml       │
│  • Updates globals.css with palette           │
│  • Commits all artifacts                      │
└──────────────────┬───────────────────────────┘
                   │
                   ▼
┌──────────────────────────────────────────────┐
│ /dispatch → /launch → /build → /wrap          │
│  .5 layer stories dispatched with design ctx  │
│  Agents receive palette, typography, layout,  │
│  component patterns, reference designs        │
└──────────────────┬───────────────────────────┘
                   │
                   ▼
┌──────────────────────────────────────────────┐
│ Layer N + N.5 = release candidate             │
│ (e.g., Layer 0 + 0.5 = high-quality MVP)      │
└──────────────────────────────────────────────┘
```

---

## Skill File Structure

```
skills/product-context/calibrate/
├── SKILL.md                    ← main skill definition (phases 1 + 2)
└── references/
    ├── design-brief-template.md ← template for Phase 1 output
    ├── design-context-schema.yaml ← schema for the design-context.yaml artifact
    └── vibe-design-tools.md    ← guide for Stitch, Pencil, and alternatives
```

The skill lives under `skills/product-context/` alongside `/dispatch`, `/envision`, `/map`, and `/reflect` — it's a product-level skill that shapes the context agents receive, not an execution-level skill.

---

## Implementation Priority

1. **Design brief template** (`references/design-brief-template.md`) — the template the skill fills from product-context.yaml
2. **`design-context.yaml` schema** (`references/design-context-schema.yaml`) — what the capture phase produces
3. **Skill definition** (`SKILL.md`) — the two-phase workflow with extraction logic and Q&A flow
4. **`/dispatch` integration** — add design context to Part 2 context assembly for `.5` layers
5. **`/map` guidance** — add `.5` calibration layer guidance to Step 3
6. **Vibe design tool guide** (`references/vibe-design-tools.md`) — reference material for the human

Items 1-3 are the **minimum viable `/calibrate` skill**. Items 4-5 close the integration loop. Item 6 is reference documentation.

---

## Key Decisions

| Decision                                                           | Reasoning                                                                                                                                                         | Alternatives                                         |
| ------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------- |
| New skill (`/calibrate`) rather than extending existing            | No existing skill handles "generate → human explores → capture" pattern                                                                                           | Extend `/dispatch` or `/reflect`                     |
| One structured markdown format for all tools                       | Both Stitch and Pencil handle structured markdown well. Tool-specific rewrites add maintenance burden for marginal gain.                                          | Separate prompts per tool                            |
| `design-context.yaml` + `globals.css` (both)                       | YAML documents decisions (queryable by agents). CSS enacts them (live implementation). Reference files show the target (visual). Three concerns, three artifacts. | YAML only, CSS only                                  |
| Reusable but evolving (establishment → extension)                  | First .5 layer is a full calibration. Subsequent .5 layers only need to cover new patterns. Rebuilding the entire design system each time is wasteful.            | Purpose-built for 0.5 only                           |
| Design directions with mood board references                       | Real product references give the human visual anchors. Abstract descriptions ("clean and modern") are too vague for design exploration.                           | Product context only, no directions                  |
| Skill under `product-context/` not `agentic-development-workflow/` | `/calibrate` shapes the context agents receive (like `/dispatch` and `/map`). It's a control-plane skill, not an execution-plane skill.                           | Under `patterns/` or `agentic-development-workflow/` |
| `.5` layers are opt-in, gated by `/reflect`                        | Not every layer needs UI polish. The human decides after reviewing the implemented layer.                                                                         | Automatic .5 after every integer layer               |

---

## Appendix: Evidence from Layer 0

| Observation                                             | Impact                                                                           | Solution                                                |
| ------------------------------------------------------- | -------------------------------------------------------------------------------- | ------------------------------------------------------- |
| No landing page after 17 stories shipped                | Product has no public face. Cannot share or demo.                                | LR-P01 landing page story, needs visual direction       |
| Auth pages are generic Better Auth defaults             | First interaction with product looks unbranded                                   | LR-P02 auth polish story, needs brand identity          |
| Dashboard shows daemon list only                        | No connection status, no actions, no session history                             | LR-P03 dashboard detail story, needs layout direction   |
| Agents consistently produce "functional but generic" UI | Structural property, not a bug. Agents optimize for correctness, not aesthetics. | Design context artifact gives agents visual constraints |
| Layer 0 + 0.5 = first release                           | The combined result is the MVP that users actually see                           | Design calibration must happen BEFORE .5 implementation |
