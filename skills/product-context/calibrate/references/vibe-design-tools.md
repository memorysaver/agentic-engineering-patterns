# Vibe Design Tool Guide

Tools for human-driven design exploration during `/calibrate` Phase 1 → Phase 2 interlude.

---

## Google Stitch

- **What:** AI design tool from Google Labs, powered by Gemini
- **Input:** Text prompts (natural language or structured markdown), hand-drawn sketches, wireframes, screenshots, voice
- **Output:** Interactive UI mockups, Figma files, HTML/CSS code
- **Export:** Direct to Figma or HTML/CSS
- **Cost:** Free
- **Best for:** Rapid exploration of multiple directions, visual brainstorming
- **Workflow:** Paste design brief → generate designs → iterate via conversation → export HTML/CSS or Figma for chosen direction

## Pencil.dev

- **What:** AI design tool that runs inside VS Code / Cursor
- **Input:** Text prompts, Figma imports, manual canvas editing
- **Output:** `.pen` JSON design files (stored in `/design` folder), pixel-perfect React/HTML/CSS code
- **Export:** React code, HTML/CSS, committed directly to Git
- **Cost:** Free in early access (requires Claude Code subscription)
- **Best for:** Design-to-code integration, Git-native design files, IDE workflow
- **Workflow:** Open Pencil in IDE → paste brief → design on canvas → `.pen` files saved to repo → Claude Code generates code from specs

## Other Tools

- **Galileo AI** (now part of Stitch) — text-to-UI, Figma export
- **Uizard** — fast browser-based prototyping, interactive prototypes
- **Banani** — generates style variations (minimalist, enterprise, playful, sleek)
- **Framer AI** — text-to-website, complete layouts with interactions

---

## What to Save

After exploring, save reference files to `docs/design-references/`:

```
docs/design-references/
├── landing.html          <- HTML/CSS from Stitch (or screenshot)
├── auth.html             <- HTML/CSS from Stitch
├── dashboard.html        <- HTML/CSS from Stitch
├── landing.png           <- screenshot alternative
└── notes.md              <- human notes on what they liked and why
```

These files are committed to Git and referenced in `design-context.yaml`. Agents read them as visual guidance — they translate the reference into the project's component system, not copy code verbatim.

## Format Note

A single structured markdown design brief works for both Stitch and Pencil.dev. No tool-specific rewrites needed — both tools handle structured markdown well. Copy the full brief or relevant sections into the tool.
