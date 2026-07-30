#!/usr/bin/env node
// Prove that every skill-owned copy of an AEP enum still matches the canonical
// declaration in skills/product-context/_shared/references/aep-vocabulary.schema.json.
//
// A half-applied taxonomy change (a value added to one listing and not the
// others) is this repo's #1 bug class — three review rounds on PR #16, and at
// the time this check was written the layer-gate vocabulary existed in three
// mutually incompatible spellings across the corpus. Materialization keeps the
// canonical file byte-identical wherever it ships (build-skills.sh --check);
// this checker covers the other direction: schemas that restate a vocabulary
// inline because their own document shape needs it.
//
// Contract: any enum in any skills/**/*.schema.json that carries
//   "x-aep-vocab": "<name>"
// must match $defs.<name> in the canonical file exactly. Enums compare as sets
// (order is presentation); anything else compares as a structure.
//
// Usage: node scripts/check-vocabulary.mjs [--json]
// Exit 0 when every copy agrees, 1 on drift, 2 on bad input.

import { readdirSync, readFileSync, statSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const REPO = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const SKILLS = join(REPO, "skills");
const CANONICAL = join(SKILLS, "product-context/_shared/references/aep-vocabulary.schema.json");
const VOCAB_BASENAME = "aep-vocabulary.schema.json";
const ANNOTATION = "x-aep-vocab";

const jsonOutput = process.argv.includes("--json");

function walkFiles(dir, out = []) {
  for (const entry of readdirSync(dir)) {
    const path = join(dir, entry);
    if (statSync(path).isDirectory()) walkFiles(path, out);
    else if (entry.endsWith(".schema.json") && entry !== VOCAB_BASENAME) out.push(path);
  }
  return out;
}

// Every node carrying the annotation, with a JSON-pointer-ish path for the report.
function annotatedNodes(node, path = "$", out = []) {
  if (Array.isArray(node)) {
    node.forEach((v, i) => annotatedNodes(v, `${path}[${i}]`, out));
    return out;
  }
  if (!node || typeof node !== "object") return out;
  if (typeof node[ANNOTATION] === "string") out.push({ path, node });
  for (const [key, value] of Object.entries(node)) annotatedNodes(value, `${path}.${key}`, out);
  return out;
}

// The keywords that decide which values are legal. Titles, descriptions, and
// x-aep-* commentary are presentation and may differ per consumer.
const VALIDATION_KEYWORDS = ["enum", "const", "oneOf", "anyOf", "type", "pattern"];
function shape(schema) {
  const picked = {};
  for (const key of VALIDATION_KEYWORDS) if (key in schema) picked[key] = schema[key];
  return picked;
}

function sameEnum(a, b) {
  const norm = (xs) => [...xs].map((x) => JSON.stringify(x)).sort();
  const [x, y] = [norm(a), norm(b)];
  return x.length === y.length && x.every((v, i) => v === y[i]);
}

let canonical;
try {
  canonical = JSON.parse(readFileSync(CANONICAL, "utf8"));
} catch (error) {
  console.error(`FAIL: cannot read the canonical vocabulary at ${relative(REPO, CANONICAL)}`);
  console.error(`      ${error.message}`);
  process.exit(2);
}
const defs = canonical.$defs ?? {};

const drift = [];
let checked = 0;

for (const file of walkFiles(SKILLS)) {
  let doc;
  try {
    doc = JSON.parse(readFileSync(file, "utf8"));
  } catch (error) {
    drift.push({ file: relative(REPO, file), path: "$", reason: `unparseable: ${error.message}` });
    continue;
  }
  for (const { path, node } of annotatedNodes(doc)) {
    checked += 1;
    const name = node[ANNOTATION];
    const def = defs[name];
    const where = { file: relative(REPO, file), path, vocab: name };
    if (!def) {
      drift.push({ ...where, reason: `no $defs.${name} in ${VOCAB_BASENAME}` });
      continue;
    }
    if (Array.isArray(def.enum)) {
      if (!Array.isArray(node.enum)) {
        drift.push({ ...where, reason: `canonical is an enum; this copy declares none` });
      } else if (!sameEnum(def.enum, node.enum)) {
        const canon = new Set(def.enum.map((v) => JSON.stringify(v)));
        const local = new Set(node.enum.map((v) => JSON.stringify(v)));
        const missing = [...canon].filter((v) => !local.has(v));
        const extra = [...local].filter((v) => !canon.has(v));
        drift.push({
          ...where,
          reason: [
            missing.length ? `missing ${missing.join(", ")}` : null,
            extra.length ? `unknown ${extra.join(", ")}` : null,
          ]
            .filter(Boolean)
            .join("; "),
        });
      }
      continue;
    }
    if (JSON.stringify(shape(def)) !== JSON.stringify(shape(node))) {
      drift.push({ ...where, reason: `structure differs from the canonical $defs.${name}` });
    }
  }
}

// ─── Prose listings ────────────────────────────────────────────────────────
// A schema is not the only place a vocabulary gets restated: templates, YAML
// schemas, and reference prose all enumerate values inline where a reader needs
// them there. Those listings opt in with a marker naming the vocabulary —
//
//   status: not_started # not_started | running | passed   (aep-vocab: layer_gate_status)
//
// — and the values on that line must then be the canonical set, no more and no
// fewer. Marking is deliberate: prose that mentions three states in passing
// ("not_started → scripted_passed → passed") is narrative, not a listing, and
// must not be forced to spell out the whole enum.
const MARKER = /\(aep-vocab:\s*([a-z_]+)\)/;
const PROSE_EXTENSIONS = [".md", ".yaml", ".yml", ".tmpl", ".json"];

function proseFiles(dir, out = []) {
  for (const entry of readdirSync(dir)) {
    const path = join(dir, entry);
    if (statSync(path).isDirectory()) proseFiles(path, out);
    else if (PROSE_EXTENSIONS.some((ext) => entry.endsWith(ext))) out.push(path);
  }
  return out;
}

// A listing line carries prose around its values (a YAML key, a trailing
// comment), so the check is token containment rather than parsing: every
// canonical value must appear as its own token, and every separated segment
// must carry at least one — which is what catches a typo'd or invented value
// without demanding that prose be machine-shaped.
function hasToken(line, value) {
  const escaped = value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  return new RegExp(`(?<![\\w-])${escaped}(?![\\w-])`).test(line);
}

function segments(line) {
  const parts = line.split("|").map((s) => s.trim());
  return parts.length > 1 ? parts : [];
}

for (const file of proseFiles(SKILLS)) {
  const lines = readFileSync(file, "utf8").split("\n");
  lines.forEach((line, index) => {
    const marked = line.match(MARKER);
    if (!marked) return;
    checked += 1;
    const name = marked[1];
    const def = defs[name];
    const where = { file: relative(REPO, file), path: `line ${index + 1}`, vocab: name };
    if (!def) {
      drift.push({ ...where, reason: `no $defs.${name} in ${VOCAB_BASENAME}` });
      return;
    }
    // x-aep-listing exists for vocabularies whose prose form is not literally
    // the enum — dogfood_target's deployed:<url> is a pattern in the schema and
    // a placeholder on the page.
    const required = (
      def["x-aep-listing"] ??
      def.enum ??
      def.oneOf?.flatMap((s) => s.enum ?? []) ??
      []
    ).map(String);
    const body = line.replace(MARKER, "");
    const missing = required.filter((v) => !hasToken(body, v));
    const stray = segments(body).filter((seg) => seg && !required.some((v) => hasToken(seg, v)));
    if (missing.length || stray.length) {
      drift.push({
        ...where,
        reason: [
          missing.length ? `missing ${missing.join(", ")}` : null,
          stray.length ? `segment names no known value: "${stray[0]}"` : null,
        ]
          .filter(Boolean)
          .join("; "),
      });
    }
  });
}

if (jsonOutput) {
  console.log(JSON.stringify({ checked, vocabularies: Object.keys(defs), drift }, null, 2));
} else if (drift.length) {
  for (const d of drift)
    console.error(`FAIL: ${d.file} ${d.path} (${d.vocab ?? "?"}) — ${d.reason}`);
  console.error("");
  console.error(
    `Edit skills/product-context/_shared/references/${VOCAB_BASENAME}, then run bash scripts/build-skills.sh.`,
  );
} else {
  console.log(
    `check-vocabulary: ${checked} tagged listing(s) agree with ${Object.keys(defs).length} canonical vocabularies`,
  );
}

process.exit(drift.length ? 1 : 0);
