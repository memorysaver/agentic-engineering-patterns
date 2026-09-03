#!/usr/bin/env node
// coherence.mjs — plan-file coherence checks, shared by /aep-validate and
// /aep-human-alignment.
//
// These are the findings that want an ACTION, not a reader. A gate recorded as
// never started while its stories are finished is not news to narrate weekly —
// it is a defect to fix, and it belongs where it can block. `/aep-validate`
// runs this and fails; the brief renders what it returns.
//
// The detector is shared rather than duplicated for the obvious reason: two
// copies of a drift detector drift, and a drift detector that drifts is worse
// than none.
//
// Spec: references/drift-facts.md (the derivations) and references/attention-set.md
// (the vocabulary). This file is the executable half.
//
// Usage:
//   node coherence.mjs --context product-context.yaml [--json] [--warn-only]
//
// Exit 0 when coherent, 1 when a blocking incoherence is found, 2 on bad input.

import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { loadVocabulary } from "./json-schema.mjs";

// Read from references/aep-vocabulary.schema.json rather than restated here, so
// the gate vocabulary has exactly one spelling across the corpus.
const REFERENCES = join(dirname(fileURLToPath(import.meta.url)), "../references");
const GATE_VOCABULARY = loadVocabulary(REFERENCES, "layer_gate_status");

// Fields the schema defines on a layer gate. A consumer carrying others is
// recording real meaning in a place no framework consumer can read (D12).
const GATE_SCHEMA_FIELDS = new Set([
  "layer",
  "status",
  "test_definition",
  "coverage",
  "evidence",
  "results",
  "journeys",
  "completed_at",
  "skip_human_eval",
]);

export async function loadYaml(path) {
  for (const mod of ["yaml", "js-yaml"]) {
    try {
      const m = await import(mod);
      const text = readFileSync(path, "utf8");
      return mod === "yaml" ? m.parse(text) : (m.default ?? m).load(text);
    } catch (err) {
      if (err?.code !== "ERR_MODULE_NOT_FOUND") throw err;
    }
  }
  return JSON.parse(
    execFileSync("npx", ["--yes", "js-yaml", path], {
      encoding: "utf8",
      maxBuffer: 256 * 1024 * 1024,
    }),
  );
}

const finding = (code, severity, subject, detail, path, fix) => ({
  code,
  severity,
  subject,
  detail,
  path,
  fix,
});

// 1 · Work finished under a gate that says it never started.
export function checkGateStoryCoherence(ctx) {
  const out = [];
  const byLayer = new Map();
  for (const s of ctx.stories ?? []) {
    if (!byLayer.has(s.layer)) byLayer.set(s.layer, []);
    byLayer.get(s.layer).push(s);
  }
  for (const g of ctx.layer_gates ?? []) {
    if (String(g.status) !== "not_started") continue;
    const done = (byLayer.get(g.layer) ?? []).filter(
      (s) => String(s.status) === "completed",
    ).length;
    if (done === 0) continue;
    out.push(
      finding(
        "coherence/completed-under-unopened-gate",
        "blocking",
        `layer ${g.layer}`,
        `${done} completed stor${done === 1 ? "y" : "ies"} under a gate recorded as not_started`,
        `layer_gates[layer=${g.layer}].status`,
        "advance the gate to the phase the work reached, or reopen the stories",
      ),
    );
  }
  return out;
}

// 2 · A gate status outside the defined vocabulary.
export function checkGateVocabulary(ctx) {
  return (ctx.layer_gates ?? [])
    .filter((g) => !GATE_VOCABULARY.has(String(g.status)))
    .map((g) =>
      finding(
        "coherence/gate-status-undefined",
        "blocking",
        `layer ${g.layer}`,
        `gate status "${g.status}" is not in the defined set (${[...GATE_VOCABULARY].join(" | ")})`,
        `layer_gates[layer=${g.layer}].status`,
        "use a defined status; the control file's wording drifts before its content does",
      ),
    );
}

// 3 · A roll-up that disagrees with the record it summarizes.
export function checkCostRollup(ctx) {
  const declared = typeof ctx.cost?.total_usd === "number" ? ctx.cost.total_usd : null;
  const priced = (ctx.stories ?? []).filter(
    (s) => typeof s.cost_usd === "number" && s.cost_usd > 0,
  );
  const summed = Number(priced.reduce((a, s) => a + s.cost_usd, 0).toFixed(2));
  if (declared === null || priced.length === 0) return [];
  if (Math.abs(declared - summed) < 0.01) return [];
  return [
    finding(
      "coherence/rollup-disagrees",
      "warning",
      "cost",
      `cost.total_usd declares ${declared} while ${priced.length} stories sum to ${summed}`,
      "cost.total_usd",
      "wire the roll-up to the story record, or drop it — a zeroed roll-up beside a non-zero record is a statement about the roll-up",
    ),
  ];
}

// 4 · The control file and the work using different words for the same thing.
export function checkModuleVocabulary(ctx) {
  const declared = new Set((ctx.architecture?.modules ?? []).map((m) => String(m.name)));
  const used = new Set(
    (ctx.stories ?? [])
      .map((s) => s.module)
      .filter(Boolean)
      .map(String),
  );
  const undeclared = [...used].filter((m) => !declared.has(m)).sort();
  const unused = [...declared].filter((m) => !used.has(m)).sort();
  const out = [];
  if (undeclared.length) {
    out.push(
      finding(
        "coherence/module-undeclared",
        "warning",
        "architecture.modules",
        `${undeclared.length} module name(s) are used by stories but never declared: ${undeclared.slice(0, 6).join(", ")}${undeclared.length > 6 ? " …" : ""}`,
        "stories[].module vs architecture.modules[].name",
        "declare them, or correct the stories — work filed against an undeclared module is invisible to every architecture consumer",
      ),
    );
  }
  if (unused.length) {
    out.push(
      finding(
        "coherence/module-never-worked",
        "warning",
        "architecture.modules",
        `${unused.length} declared module(s) have never been worked: ${unused.slice(0, 6).join(", ")}`,
        "architecture.modules[].name",
        "confirm they are still planned, or remove them",
      ),
    );
  }
  return out;
}

// 5 · Fields a consumer invented that the schema does not define (D12).
export function checkSchemaInventions(ctx) {
  const seen = new Map();
  for (const g of ctx.layer_gates ?? []) {
    for (const k of Object.keys(g)) {
      if (GATE_SCHEMA_FIELDS.has(k)) continue;
      seen.set(k, (seen.get(k) ?? 0) + 1);
    }
  }
  if (seen.size === 0) return [];
  return [
    finding(
      "coherence/schema-invention",
      "warning",
      "layer_gates",
      `${seen.size} field(s) not defined by the schema carry meaning here: ${[...seen.keys()].sort().join(", ")}`,
      "layer_gates[]",
      "propose them to the schema, or move the meaning into a defined field — no framework consumer can read an invented one",
    ),
  ];
}

export function coherence(ctx) {
  const findings = [
    ...checkGateStoryCoherence(ctx),
    ...checkGateVocabulary(ctx),
    ...checkCostRollup(ctx),
    ...checkModuleVocabulary(ctx),
    ...checkSchemaInventions(ctx),
  ];
  return {
    findings,
    blocking: findings.filter((f) => f.severity === "blocking").length,
    warnings: findings.filter((f) => f.severity === "warning").length,
  };
}

function arg(name, fallback = null) {
  const i = process.argv.indexOf(`--${name}`);
  return i > -1 && process.argv[i + 1] ? process.argv[i + 1] : fallback;
}

if (process.argv[1] && process.argv[1].endsWith("coherence.mjs")) {
  const ctxPath = resolve(arg("context", "product-context.yaml"));
  if (!existsSync(ctxPath)) {
    console.error(`ERROR: ${ctxPath} not found.`);
    process.exit(2);
  }
  const result = coherence(await loadYaml(ctxPath));
  if (process.argv.includes("--json")) {
    console.log(JSON.stringify(result, null, 2));
  } else if (result.findings.length === 0) {
    console.log("coherence: PASS — the control file agrees with the work record");
  } else {
    for (const f of result.findings) {
      const tag = f.severity === "blocking" ? "BLOCKING" : "warning ";
      console.log(
        `${tag} ${f.code}\n  ${f.subject}: ${f.detail}\n  at ${f.path}\n  fix: ${f.fix}\n`,
      );
    }
    console.log(`coherence: ${result.blocking} blocking, ${result.warnings} warning(s)`);
  }
  process.exit(result.blocking > 0 && !process.argv.includes("--warn-only") ? 1 : 0);
}
