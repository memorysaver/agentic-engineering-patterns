---
name: calibrate
description: Two-phase design calibration for UI polish layers. Use when entering a .5 layer (0.5, 1.5, 2.5), when the user says "calibrate", "design brief", "capture design", or after /reflect identifies a UI quality gap. Phase 1 generates a design brief; Phase 2 captures decisions into design-context.yaml.
---

# Calibrate

Generate a design brief from product context, let the human explore with vibe design tools, then capture design decisions into `design-context.yaml` — the authoritative visual reference for all future UI work.

**Where this fits:**

```
/reflect (identified UI gap, created .5 layer stories)
  → /calibrate           (Phase 1: generate design brief)
  → human explores       (external: Stitch, Pencil.dev)
  → /calibrate capture   (Phase 2: capture design decisions)
  → /dispatch            (dispatches .5 stories with design context)
```

**Session:** Main, interactive with user
**Input:** `product-context.yaml` with `.5` layer stories defined
**Output:** `design-context.yaml` + updated `globals.css`

---

## Mode Detection

Check if `design-context.yaml` exists at repo root:

- **Does not exist** → **Establishment mode** (full calibration — palette, typography, layout, everything)
- **Exists** → **Extension mode** (focused brief covering only NEW UI patterns not in the existing design context)

---

## Phase 1: Generate Design Brief

### Step 1: Read Product Context

```bash
cat product-context.yaml
```

Extract:

- `opportunity.bet` + `product.problem` → product identity
- `product.persona.description` + `product.persona.jtbd` → target user
- `opportunity.why_now` → urgency/positioning
- `stories` where `layer` matches the active `.5` layer → pages to design
- `product.constraints.required_stack` + `preferred_stack` → technical constraints

### Step 2: Scan Existing UI State

- `packages/ui/src/styles/globals.css` → current theme tokens (color space, font, radius)
- `packages/ui/src/components/` → available shadcn components (list directory)
- `apps/web/src/routes/` → existing pages (list directory)

### Step 3: Generate Design Brief

**Establishment mode:** Use the full template at `references/design-brief-template.md`. Generate 3 design directions spanning a spectrum from "maximum technical" to "maximum approachable". Name 2-3 reference products per direction as visual mood board anchors.

**Extension mode:** Read existing `design-context.yaml`. Identify NEW UI patterns needed by the current `.5` layer's stories that aren't covered by the existing `components` section. Generate a focused brief: "Here's your current design system. These new pages need patterns not yet covered: [list]. Explore additions that complement the established direction."

Write the brief to `docs/design-brief.md` and output the full content to terminal for copy-paste.

### Step 4: Hand Off to Human

Output:

```
Design brief written to: docs/design-brief.md

Next steps (you do these):

  1. Copy the brief to a vibe design tool:
     - Google Stitch (stitch.withgoogle.com) — free, Figma/HTML/CSS export
     - Pencil.dev — IDE-native (VS Code/Cursor), .pen files in Git
     See references/vibe-design-tools.md for details.

  2. Try each direction. Generate at least 2 variations per direction.

  3. Pick what feels right. Save reference files to docs/design-references/

  4. When ready, come back and run:
     /calibrate capture
```

**Stop here.** Phase 1 is complete. The human explores externally — this is explicitly out of the agent's hands.

---

## Phase 2: Capture Design Decisions (`/calibrate capture`)

### Step 1: Interactive Q&A

Ask the user structured questions, one at a time:

1. **Direction:** Which direction did you choose? (or describe the hybrid)
2. **Palette:** Primary, secondary, accent, destructive, muted colors (any format — hex, rgb, hsl — the skill converts to oklch for `globals.css`)
3. **Typography:** Font family changes? Size scale adjustments? Weight usage patterns?
4. **Components:** Recurring patterns observed? Card styles, button variants, spacing rhythm?
5. **Layout:** Max-width, grid vs flex, sidebar vs top-nav, content density?
6. **Brand signals:** What visual cues communicate "this is [product name]"?
7. **Reference files:** Did you save HTML/CSS/screenshots to `docs/design-references/`? List them.

### Step 2: Produce `design-context.yaml`

**Establishment mode:** Create `design-context.yaml` at repo root from scratch using the schema at `references/design-context-schema.yaml`. Fill all sections from the Q&A answers.

**Extension mode:** Read existing `design-context.yaml`. Add new entries to the `components` section for newly covered patterns. Add new entries to `reference_designs` for new pages. Update `calibrated_at` and `calibrated_from_layer`. Do not replace existing values — extend them.

### Step 3: Update `globals.css`

Read the captured palette values. Convert to oklch if provided in other color spaces. Write as CSS custom properties in `globals.css` under `:root` and `.dark` selectors.

**Establishment mode:** Replace the full palette.
**Extension mode:** Only add new custom properties if the palette expanded. Do not touch existing values.

### Step 4: Commit

```bash
jj describe -m "feat: establish design system via /calibrate — [direction name]"
jj new
jj git push --change @-
```

---

## Key Principles

- **`design-context.yaml` documents decisions (why), `globals.css` enacts them (what), reference files show the visual target (how it looks).** Three concerns, three artifacts.
- **Agents query `design-context.yaml` for specific tokens** (`palette.dark.primary`, `components.border_radius`) — it must be machine-readable YAML, not prose.
- **Design directions are spectrum-based** — from "maximum technical" to "maximum approachable" — with real product references as visual anchors, not abstract descriptions.
- **Extension mode covers only the delta** — no full redesign on subsequent `.5` layers, just new patterns.
- **The human decides** — the skill generates options and captures decisions. It does not make design choices.
