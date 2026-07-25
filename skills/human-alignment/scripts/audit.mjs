#!/usr/bin/env node
// audit.mjs — Phase 3 of /aep-human-alignment.
//
// The independent mechanical audit. It never authors and never renders: it
// reads the authored HTML plus the facts JSON and decides whether the page is
// allowed to ship. Evaluator independence (verification-economics v3.1.0) is
// the point — the agent that wrote the prose does not get to grade it.
//
// Every failure is a structured receipt: stable code, subject, evidence,
// supportedFixes. The authoring agent applies a listed fix and re-runs; it
// never guesses, and never exceeds two correction rounds.
//
// Usage: node audit.mjs --html <brief.html> --facts <facts.json> [--json]

import { readFileSync } from "node:fs";
import { resolve } from "node:path";

// ─── Text extraction ───────────────────────────────────────────────────────
// "Authored markup" = what a human reads, minus everything machine-owned:
// style, script (including the embedded JSON), and the provenance anchors that
// are explicitly allowed to carry identifiers and numbers.
export function authoredText(html) {
  let s = html;
  s = s.replace(/<style[\s\S]*?<\/style>/gi, " ");
  s = s.replace(/<script[\s\S]*?<\/script>/gi, " ");
  s = s.replace(/<!--[\s\S]*?-->/g, " ");
  s = s.replace(/<[^>]*class="[^"]*\banchor\b[^"]*"[^>]*>[\s\S]*?<\/[a-z]+>/gi, " ");
  s = s.replace(/<[^>]*class="[^"]*\bstamp\b[^"]*"[^>]*>[\s\S]*?<\/[a-z]+>/gi, " ");
  s = s.replace(/<span[^>]*data-fact="[^"]*"[^>]*>[\s\S]*?<\/span>/gi, " ");
  s = s.replace(/<[^>]+>/g, " ");
  return s.replace(/&[a-z]+;/gi, " ").replace(/\s+/g, " ");
}

const receipt = (code, subject, evidence, supportedFixes) => ({
  code,
  subject,
  evidence,
  supportedFixes,
});

// ─── Checks ────────────────────────────────────────────────────────────────
export function auditNumberProvenance(html, facts) {
  const out = [];
  const resolvePath = (path) =>
    path
      .split(".")
      .reduce((o, k) => (o == null ? undefined : k === "length" ? o.length : o[k]), facts);

  // every data-fact path must resolve
  for (const m of html.matchAll(/data-fact="([^"]+)"/g)) {
    if (resolvePath(m[1]) === undefined) {
      out.push(
        receipt(
          "provenance/unresolved-fact",
          { attribute: "data-fact", path: m[1] },
          { resolved: null },
          [
            "correct the path to one that exists in facts JSON",
            "add the field to derive.mjs so the fact exists",
          ],
        ),
      );
    }
  }

  // no digit may appear in authored prose
  const text = authoredText(html);
  for (const m of text.matchAll(/(?<![\w.])(\d[\d,.]*)(?![\w])/g)) {
    const around = text.slice(Math.max(0, m.index - 45), m.index + 45).trim();
    out.push(
      receipt("provenance/hardcoded-number", { number: m[1] }, { context: around }, [
        'replace the literal with <span data-fact="<path>"></span>',
        "move the number into the provenance anchor if it is a citation",
      ]),
    );
  }
  return out;
}

export function auditClasses(html, template) {
  const declared = new Set();
  for (const style of template.matchAll(/<style[\s\S]*?<\/style>/gi)) {
    for (const m of style[0].matchAll(/\.([a-z][a-z0-9-]*)/gi)) declared.add(m[1]);
  }
  const used = new Set();
  for (const m of html.matchAll(/class="([^"]+)"/g))
    for (const c of m[1].split(/\s+/)) if (c) used.add(c);
  return [...used]
    .filter((c) => !declared.has(c))
    .sort()
    .map((c) =>
      receipt("classes/undeclared", { class: c }, { declaredCount: declared.size }, [
        "use an existing class from assets/template.html",
        "add the class to the template stylesheet first, then use it",
      ]),
    );
}

export function auditChipGrammar(html) {
  const out = [];
  for (const m of html.matchAll(/<span class="chip (goal|exp)"[^>]*>([\s\S]*?)<\/span>/gi)) {
    const kind = m[1];
    const body = m[2].replace(/<[^>]+>/g, "").trim();
    if (!body) {
      out.push(
        receipt("chip/empty", { kind }, { body }, [
          "give the chip its binding text",
          "remove the chip",
        ]),
      );
      continue;
    }
    // a GOAL chip must name what binds it; an EXP chip must name what settles it
    if (kind === "goal" && !/\b(L\d|layer|story|[A-Z]{2,}-)/.test(body)) {
      out.push(
        receipt(
          "chip/goal-unbound",
          { kind, body },
          { rule: "a GOAL chip must carry a story or layer id" },
          [
            "add the binding story or layer id to the chip text",
            "demote the claim to prose if nothing binds it",
          ],
        ),
      );
    }
    if (kind === "exp" && !/→|->|when|after|once|until/i.test(body)) {
      out.push(
        receipt(
          "chip/exp-unsettled",
          { kind, body },
          { rule: "an EXP chip must name the event that settles it" },
          [
            'add the settling event to the chip text (e.g. "→ once the acceptance run passes")',
            "remove the claim if no event would settle it",
          ],
        ),
      );
    }
  }
  // one chip governs one clause, never a paragraph
  for (const m of html.matchAll(/<p[^>]*>([\s\S]*?)<\/p>/gi)) {
    const chips = (m[1].match(/class="chip /g) ?? []).length;
    if (chips > 1) {
      out.push(
        receipt(
          "chip/multiple-per-block",
          { chips },
          { excerpt: m[1].replace(/<[^>]+>/g, "").slice(0, 90) },
          ["split the paragraph so each chip governs one clause"],
        ),
      );
    }
  }
  return out;
}

export function auditSlots(html) {
  const out = [];
  for (const m of html.matchAll(/\{\{([A-Z0-9_]+)\}\}/g)) {
    // *_JSON slots are data planes injected by assemble.mjs at delivery, not
    // authored content — an audit that ran before assembly would always fail.
    if (m[1].endsWith("_JSON")) continue;
    out.push(
      receipt("template/unfilled-slot", { slot: m[1] }, {}, [
        "fill the slot with authored content",
        "delete the slot if the section does not apply",
      ]),
    );
  }
  return out;
}

export function auditVocabularyBudget(html, budget = 7) {
  // The prose channel may leave at most `budget` system words undefined.
  const SYSTEM_WORDS = [
    "layer",
    "gate",
    "story",
    "epoch",
    "wave",
    "sha",
    "dispatch",
    "wrap",
    "envision",
    "reflect",
    "scripted_passed",
    "not_started",
    "in_review",
    "product-context",
    "yaml",
    "schema",
    "coverage",
    "amendment",
    "calibration",
    "object-map",
  ];
  const text = authoredText(html).toLowerCase();
  const seen = SYSTEM_WORDS.filter((w) => new RegExp(`(^|[^a-z-])${w}([^a-z-]|$)`).test(text));
  if (seen.length <= budget) return [];
  return [
    receipt("vocabulary/over-budget", { used: seen.length, budget }, { words: seen }, [
      "translate the excess system words into plain language",
      "move the identifier into the provenance anchor, where it does not count",
    ]),
  ];
}

export function auditAnchors(html) {
  // Translation-anchor 1:1 — a band that surfaces items must cite sources.
  const out = [];
  for (const m of html.matchAll(/<section class="band[^"]*" id="([^"]+)"[\s\S]*?<\/section>/gi)) {
    const id = m[1];
    const body = m[0];
    const hasAnchor = /class="anchor"/.test(body) || /data-fact=/.test(body);
    if (!hasAnchor) {
      out.push(
        receipt("anchor/band-uncited", { band: id }, {}, [
          "add a provenance anchor under each surfaced item",
          "bind at least one number in the band to facts JSON",
        ]),
      );
    }
  }
  return out;
}

// ─── Runner ────────────────────────────────────────────────────────────────
export function audit({ html, facts, template }) {
  // Comments are authoring instructions, not surface. The template's own
  // contract block contains illustrative markup (`<span class="chip goal">…`,
  // `data-fact="<path>"`); auditing it would fail every page on its own docs.
  const source = html.replace(/<!--[\s\S]*?-->/g, " ");
  return [
    ...auditSlots(source),
    ...auditNumberProvenance(source, facts),
    ...auditClasses(source, template ?? source),
    ...auditChipGrammar(source),
    ...auditAnchors(source),
    ...auditVocabularyBudget(source),
  ];
}

const JUDGMENT_CHECKS = [
  "vocabulary audit — no part or stage named by a word outside the canonical set",
  "evidence language — no causal claim (blocks/breaks/guarantees/proves) without a cited fact id",
  "glance gate — every diagram readable in one look, or decomposed",
  "cold-reader test — a first-time reader can answer where are we / what needs me / where did reality drift",
  "so-what test — every surfaced row states why the reader should care",
];

function arg(name, fallback = null) {
  const i = process.argv.indexOf(`--${name}`);
  return i > -1 && process.argv[i + 1] ? process.argv[i + 1] : fallback;
}

if (process.argv[1] && process.argv[1].endsWith("audit.mjs")) {
  const htmlPath = resolve(arg("html", ""));
  const factsPath = resolve(arg("facts", ""));
  const templatePath = arg("template");
  const html = readFileSync(htmlPath, "utf8");
  const facts = JSON.parse(readFileSync(factsPath, "utf8"));
  const template = templatePath ? readFileSync(resolve(templatePath), "utf8") : html;

  const receipts = audit({ html, facts, template });
  if (process.argv.includes("--json")) {
    console.log(
      JSON.stringify(
        { ok: receipts.length === 0, receipts, judgment_checks: JUDGMENT_CHECKS },
        null,
        2,
      ),
    );
  } else if (receipts.length === 0) {
    console.log(`audit: PASS — ${htmlPath}`);
    console.log("  mechanical checks green. Judgment checks remain (references/checklist.md):");
    for (const c of JUDGMENT_CHECKS) console.log(`    · ${c}`);
  } else {
    console.error(`audit: FAIL — ${receipts.length} receipt(s)`);
    const byCode = {};
    for (const r of receipts) (byCode[r.code] ??= []).push(r);
    for (const [code, list] of Object.entries(byCode)) {
      console.error(`  ${code} × ${list.length}`);
      for (const r of list.slice(0, 4)) {
        console.error(`    subject ${JSON.stringify(r.subject)}`);
        if (r.evidence && Object.keys(r.evidence).length)
          console.error(`    evidence ${JSON.stringify(r.evidence)}`);
        console.error(`    fixes: ${r.supportedFixes.join(" | ")}`);
      }
      if (list.length > 4) console.error(`    … ${list.length - 4} more`);
    }
  }
  process.exit(receipts.length === 0 ? 0 : 1);
}
