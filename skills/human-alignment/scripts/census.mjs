#!/usr/bin/env node
// census.mjs — Phase 1a of /aep-human-alignment.
//
// The system must know what it has not looked at.
//
// Enumerates every populated key path in the consumer's plan file, collapses it
// to a path template (arrays become `[]`), and classifies it against the
// committed manifest in source-census.json:
//
//   derived   — this path reaches facts JSON
//   ignored   — deliberately not read, WITH A REASON (never a bare checkmark)
//   unhandled — neither, and therefore a gap this run is blind to
//
// Every content defect the first implementation shipped was an unhandled path:
// cost_usd, files_affected, attempt_count, closure_status. Reporting them is
// what turns completeness from a hope into a number.
//
// Usage: node census.mjs --context <plan.yaml> [--out <census.json>] [--strict]

import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const HERE = dirname(fileURLToPath(import.meta.url));

const isEmpty = (v) =>
  v === null ||
  v === undefined ||
  v === "" ||
  (Array.isArray(v) && v.length === 0) ||
  (typeof v === "object" && !Array.isArray(v) && Object.keys(v).length === 0);

// Collapse a document into path templates. `stories[0].module` and
// `stories[395].module` are one template — the unit the owner rules the census
// classifies at, because leaf-level entries are dominated by per-consumer noise.
export function walkTemplates(node, path = "", out = new Map()) {
  const bump = (p, populated) => {
    if (!out.has(p)) out.set(p, { occurrences: 0, populated: 0, sample: null });
    const rec = out.get(p);
    rec.occurrences++;
    if (populated) rec.populated++;
    return rec;
  };

  if (Array.isArray(node)) {
    if (node.length === 0) bump(path + "[]", false);
    for (const v of node) walkTemplates(v, path + "[]", out);
    return out;
  }
  if (node && typeof node === "object") {
    if (Object.keys(node).length === 0 && path) bump(path, false);
    for (const [k, v] of Object.entries(node)) {
      walkTemplates(v, path ? `${path}.${k}` : k, out);
    }
    return out;
  }
  const rec = bump(path, !isEmpty(node));
  if (rec.sample === null && !isEmpty(node)) rec.sample = String(node).slice(0, 60);
  return out;
}

// A rule may name an exact template or a subtree (`architecture.adrs[].*`).
// Subtree rules exist so one genuine reason can cover one genuinely homogeneous
// group — 486 hand-written entries would degrade into the checkmark the owner
// ruled out. The most specific rule wins, so an exception inside an ignored
// subtree is always expressible.
export function matchRule(rules, path) {
  if (rules[path]) return rules[path];
  let best = null;
  let bestLen = -1;
  for (const [pattern, rule] of Object.entries(rules)) {
    if (!pattern.endsWith(".*")) continue;
    const prefix = pattern.slice(0, -2);
    if ((path === prefix || path.startsWith(prefix + ".")) && prefix.length > bestLen) {
      best = rule;
      bestLen = prefix.length;
    }
  }
  return best;
}

export function census(doc, manifest) {
  const found = walkTemplates(doc);
  const rules = manifest.paths ?? {};
  const derived = [];
  const ignored = [];
  const unhandled = [];

  for (const [path, rec] of [...found].sort()) {
    if (rec.populated === 0) continue; // an absent field is not a gap
    const rule = matchRule(rules, path);
    if (!rule) {
      unhandled.push({ path, populated: rec.populated, sample: rec.sample });
    } else if (rule.status === "derived") {
      derived.push({ path, populated: rec.populated });
    } else {
      ignored.push({ path, populated: rec.populated, reason: rule.reason });
    }
  }

  const declaredButAbsent = Object.keys(rules)
    .filter((p) => !p.endsWith(".*"))
    .filter((p) => !found.has(p) || found.get(p).populated === 0)
    .sort();

  // An ignore without a reason is a checkmark, and a checkmark records only that
  // someone clicked past the field. Fail the census rather than accept one.
  const reasonless = Object.entries(rules)
    .filter(([, r]) => r.status === "ignored" && !String(r.reason ?? "").trim())
    .map(([p]) => p);

  const total = derived.length + ignored.length + unhandled.length;
  return {
    schema_version: 1,
    populated_paths: total,
    derived_count: derived.length,
    ignored_count: ignored.length,
    unhandled_count: unhandled.length,
    read_coverage: total === 0 ? 1 : Number((derived.length / total).toFixed(3)),
    derived,
    ignored,
    unhandled,
    declared_but_absent: declaredButAbsent,
    reasonless_ignores: reasonless,
  };
}

function arg(name, fallback = null) {
  const i = process.argv.indexOf(`--${name}`);
  return i > -1 && process.argv[i + 1] ? process.argv[i + 1] : fallback;
}

if (process.argv[1] && process.argv[1].endsWith("census.mjs")) {
  const { loadYaml } = await import("./derive.mjs");
  const contextPath = resolve(arg("context", "product-context.yaml"));
  if (!existsSync(contextPath)) {
    console.error(`ERROR: ${contextPath} not found.`);
    process.exit(2);
  }
  const manifest = JSON.parse(readFileSync(resolve(HERE, "source-census.json"), "utf8"));
  const result = census(await loadYaml(contextPath), manifest);
  const outPath = arg("out");
  if (outPath) writeFileSync(outPath, JSON.stringify(result, null, 2) + "\n");

  console.log(
    `census: ${result.populated_paths} populated path templates · ${result.derived_count} derived · ${result.ignored_count} ignored · ${result.unhandled_count} unhandled`,
  );
  console.log(`  read coverage ${(result.read_coverage * 100).toFixed(1)}%`);
  if (result.unhandled.length) {
    console.log("  UNHANDLED (carrying data, and invisible to this brief):");
    for (const u of result.unhandled.slice(0, 40)) {
      console.log(`    ${u.path}  ×${u.populated}  e.g. ${JSON.stringify(u.sample)}`);
    }
    if (result.unhandled.length > 40) console.log(`    … ${result.unhandled.length - 40} more`);
  }
  if (result.declared_but_absent.length) {
    console.log(
      `  declared in the manifest but absent here (${result.declared_but_absent.length}) — normal across schema versions`,
    );
  }
  if (result.reasonless_ignores.length) {
    console.error(
      `census: FAIL — ${result.reasonless_ignores.length} ignore rule(s) carry no reason:`,
    );
    for (const p of result.reasonless_ignores) console.error(`    ${p}`);
    process.exit(1);
  }
  process.exit(process.argv.includes("--strict") && result.unhandled.length ? 1 : 0);
}
