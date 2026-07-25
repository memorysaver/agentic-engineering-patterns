#!/usr/bin/env node
// assemble.mjs — Phase 4 of /aep-human-alignment.
//
// Takes the authored HTML, injects the data planes (facts, manifest, delivered
// artifacts) and writes the one file the owner opens. Then it does the two
// things that keep the surface honest over time:
//
//   · records the generation in manifest.json, appending the previous record to
//     history[] so the ledger outlives the files it describes;
//   · prunes every brief beyond the newest three (retention ruling), because a
//     ~3 MB artifact per run with "pruning is the owner's choice" is a policy
//     that never runs.
//
// Usage:
//   node assemble.mjs --authored <authored.html> --facts <facts.json>
//                     --artifacts <dir> --out-dir <docs/human-alignment>
//                     [--keep 3] [--commit <sha>]

import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { basename, join, resolve } from "node:path";

const ARTIFACT_KEYS = [
  "architecture-now",
  "architecture-next",
  "architecture-declared",
  "aep-loop",
  "story-states",
];

const BRIEF_RE = /^brief-(\d{4}-\d{2}-\d{2})T(\d{4})Z-([0-9a-f]+)\.html$/;

export function collectArtifacts(dir) {
  const out = {};
  if (!dir || !existsSync(dir)) return out;
  for (const key of ARTIFACT_KEYS) {
    const path = join(dir, `${key}.html`);
    if (existsSync(path)) out[key] = readFileSync(path, "utf8");
  }
  return out;
}

// The three data planes are injected as JSON into <script type="application/json">.
// `</script>` inside a payload would end the block early, so the sequence is
// escaped — the only transformation applied to artifact bytes.
const safeJson = (value) => JSON.stringify(value).replace(/<\/(script)/gi, "<\\/$1");

export function inject(templateHtml, { facts, manifest, artifacts }) {
  return templateHtml
    .replace("{{FACTS_JSON}}", safeJson(facts))
    .replace("{{MANIFEST_JSON}}", safeJson(manifest))
    .replace("{{ARTIFACTS_JSON}}", safeJson(artifacts));
}

export function prune(dir, keep) {
  const briefs = readdirSync(dir)
    .filter((f) => BRIEF_RE.test(f))
    .sort()
    .reverse();
  const removed = briefs.slice(keep);
  for (const f of removed) rmSync(join(dir, f));
  return { kept: briefs.slice(0, keep), removed };
}

function arg(name, fallback = null) {
  const i = process.argv.indexOf(`--${name}`);
  return i > -1 && process.argv[i + 1] ? process.argv[i + 1] : fallback;
}

if (process.argv[1] && process.argv[1].endsWith("assemble.mjs")) {
  const authoredPath = resolve(arg("authored", ""));
  const factsPath = resolve(arg("facts", ""));
  const artifactsDir = arg("artifacts");
  const outDir = resolve(arg("out-dir", "docs/human-alignment"));
  const keep = Number(arg("keep", 3));

  if (!existsSync(authoredPath) || !existsSync(factsPath)) {
    console.error("ERROR: --authored <html> and --facts <json> are required.");
    process.exit(2);
  }
  mkdirSync(outDir, { recursive: true });

  const facts = JSON.parse(readFileSync(factsPath, "utf8"));
  const commit = arg("commit", facts.commit);
  const now = new Date();
  const stamp = `${now.toISOString().slice(0, 10)}T${now.toISOString().slice(11, 13)}${now.toISOString().slice(14, 16)}Z`;
  const filename = `brief-${stamp}-${commit}.html`;

  // (2) Diagrams and facts must describe the same commit. Round 2 shipped a
  // code graph one commit behind the facts, and three diagram captions
  // contradicted the prose beside them as a result.
  const graphPath = join(artifactsDir ?? ".", "code-graph.json");
  if (existsSync(graphPath)) {
    const graphCommit = JSON.parse(readFileSync(graphPath, "utf8")).commit;
    if (graphCommit && facts.commit && graphCommit !== facts.commit) {
      console.error(
        `ERROR: commit skew — facts are at ${facts.commit}, the code graph at ${graphCommit}.\n` +
          `  Re-run scan-workspace.mjs + arch-rules.mjs, or re-run derive.mjs, so both describe one commit.`,
      );
      process.exit(1);
    }
  }

  const artifacts = collectArtifacts(artifactsDir);
  const missing = ARTIFACT_KEYS.filter((k) => !artifacts[k]);
  const factsSha = createHash("sha256").update(JSON.stringify(facts)).digest("hex");

  const manifestPath = join(outDir, "manifest.json");
  const previous = existsSync(manifestPath) ? JSON.parse(readFileSync(manifestPath, "utf8")) : null;

  // (3) A destroyed baseline makes "this is the first brief" true by accident.
  // If the ledger names a previous generation whose file is gone and whose
  // history is empty, the delta was lost — almost always because a brief was
  // removed by hand instead of pruned by this script.
  if (
    previous?.output &&
    !existsSync(join(outDir, previous.output)) &&
    (previous.history ?? []).length === 0
  ) {
    console.error(
      `ERROR: the delta baseline was lost. manifest.json names ${previous.output}, which is not in ${outDir},` +
        ` and history[] is empty.\n  Regenerate through this script rather than removing briefs by hand,` +
        ` or restore the manifest from version control.`,
    );
    process.exit(1);
  }

  const manifest = {
    schema_version: 1,
    generated: stamp,
    commit,
    output: filename,
    facts_sha256: factsSha,
    artifacts: Object.keys(artifacts).sort(),
    degraded: missing,
    content_sha256: null,
    history: previous ? [...(previous.history ?? []), stripHistory(previous)] : [],
  };

  const html = inject(readFileSync(authoredPath, "utf8"), { facts, manifest, artifacts });
  const outPath = join(outDir, filename);
  writeFileSync(outPath, html);
  manifest.content_sha256 = createHash("sha256").update(html).digest("hex");
  writeFileSync(manifestPath, JSON.stringify(manifest, null, 2) + "\n");

  const { kept, removed } = prune(outDir, keep);

  console.log(`assemble: ${outPath}`);
  console.log(
    `  ${(html.length / 1024 / 1024).toFixed(2)} MB · ${Object.keys(artifacts).length}/${ARTIFACT_KEYS.length} artifacts embedded`,
  );
  if (missing.length) console.log(`  DEGRADED (named on the surface): ${missing.join(", ")}`);
  console.log(
    `  content sha256 ${manifest.content_sha256.slice(0, 16)} · facts sha256 ${factsSha.slice(0, 16)}`,
  );
  console.log(
    `  retention: kept ${kept.length}${removed.length ? `, pruned ${removed.map(basename).join(", ")}` : ""}`,
  );
}

function stripHistory(record) {
  const { history: _history, ...rest } = record;
  return rest;
}
