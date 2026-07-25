# Delivery Checklist

`audit.mjs` runs the mechanical half. This file is the judgment half — the
checks a script cannot make. **P0 items block delivery.**

## Receipt format

Every failure, mechanical or judgment, is reported as a structured receipt:

```json
{
  "code": "provenance/hardcoded-number",
  "subject": { "number": "38" },
  "evidence": { "context": "…38 acceptance criteria, none verified…" },
  "supportedFixes": ["replace the literal with <span data-fact=\"<path>\">", "…"]
}
```

Apply a **listed** fix and re-run. Never invent a fix. Never exceed **two**
correction rounds — a third failure is reported to the owner, not retried.

## P0 — mechanical (audit.mjs)

- [ ] **No unfilled slot.** Every `{{SLOT}}` filled or deleted (`*_JSON` slots
      are injected at assembly and exempt).
- [ ] **Number provenance.** Every `data-fact` path resolves in facts JSON, and
      no digit appears in authored markup outside a `data-fact` element or an
      anchor.
- [ ] **Class preflight.** Every class used exists in `assets/template.html`.
      Authoring begins by reading its `<style>` block, never by inventing a class.
- [ ] **Chip grammar.** GOAL chips carry a binding id; EXP chips name a settling
      event; at most one chip per block.
- [ ] **Anchors present.** Every band cites at least one source.
- [ ] **Vocabulary budget.** At most seven system words in the prose channel.

## P0 — judgment

- [ ] **Vocabulary audit.** No part or stage is named by a word outside the
      canonical set in `guideline.md`. Check the diagrams too.
- [ ] **Evidence language.** No causal claim (_blocks, breaks, guarantees,
      unblocks, proves_) without a cited fact id.
- [ ] **Never chip a fact.** Read the page at arm's length: the proportion of
      chipped text is the honesty meter. If a fact wears a chip, the meter lies.
- [ ] **Gate honesty.** The unchipped capability list contains only `passed`
      gates. Any `scripted_passed` capability wears an EXP chip naming the
      acceptance run. _This is the check the reference example failed before it
      existed._
- [ ] **Cold-reader test.** Hand the page to someone who has never seen the
      project. Can they answer where are we / what needs me / where did reality
      drift, without asking what a word means?
- [ ] **Silence over fabrication.** Every drift row is derived. If a detector was
      skipped for missing fields, the page says so rather than reporting "clear".

## P1 — presentation

- [ ] **Glance gate.** Every diagram is readable in one look. A diagram that
      needs study is decomposed, bounded, or moved behind a disclosure. Record
      the failure rather than shipping an unreadable graph.
- [ ] **Answer first.** Every section opens with its own conclusion.
- [ ] **So-what test.** A row that cannot state why the reader should care moves
      to the anchor or disclosure channel; it does not occupy the surface.
- [ ] **Fold the queue.** A cold reader meets sentences, not dozens of rows.
- [ ] **Degrade ladder.** With WebGL and the font CDN blocked, the page stays
      legible; embedded diagrams still render because they are inlined. With the
      archify CLI absent at generation, diagram sections show the named degraded
      rung and say they are degraded.

## P2 — provenance

- [ ] Filename's commit equals HEAD at generation.
- [ ] `manifest.json` content SHA-256 matches the delivered file.
- [ ] The prior generation moved into `history[]`.
- [ ] At most three briefs remain in `docs/human-alignment/`.
