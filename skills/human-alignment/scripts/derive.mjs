#!/usr/bin/env node
// derive.mjs — Phase 1 of /aep-human-alignment.
//
// Extracts the facts JSON from product-context.yaml + git. This is the ONLY
// legal source of numbers for a brief: the authoring agent never counts, and
// audit.mjs re-checks that nothing on the page escaped this file.
//
// Implements the two framework specs verbatim:
//   references/attention-set.md   — what needs a human
//   references/drift-facts.md     — where reality drifted
//
// Usage: node derive.mjs [--context <path>] [--manifest <path>] [--out <path>]

import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { readFileSync, existsSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const HERE = dirname(fileURLToPath(import.meta.url));

// ─── YAML ──────────────────────────────────────────────────────────────────
// Prefer a locally installed parser; fall back to the CLI form the AEP yaml
// guardrails already prescribe (`npx js-yaml <file>` prints JSON). Both are
// deterministic; neither is allowed to be skipped silently.
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
  const json = execFileSync("npx", ["--yes", "js-yaml", path], {
    encoding: "utf8",
    maxBuffer: 256 * 1024 * 1024,
  });
  return JSON.parse(json);
}

// ─── Minimal JSON-Schema validator ─────────────────────────────────────────
// Covers exactly the keywords facts.schema.json uses. Kept in-repo so the
// pipeline has no network dependency and audit.mjs can reuse it.
export function validate(schema, data, root = schema, path = "$") {
  const errs = [];
  const fail = (msg) => errs.push(`${path}: ${msg}`);
  if (schema.$ref) {
    const target = schema.$ref
      .replace(/^#\//, "")
      .split("/")
      .reduce((o, k) => o?.[k.replace(/~1/g, "/").replace(/~0/g, "~")], root);
    return validate(target, data, root, path);
  }
  if (schema.oneOf) {
    const ok = schema.oneOf.filter((s) => validate(s, data, root, path).length === 0);
    if (ok.length !== 1) fail(`matched ${ok.length} of oneOf, expected exactly 1`);
    return errs;
  }
  if ("const" in schema && data !== schema.const)
    fail(`expected const ${JSON.stringify(schema.const)}`);
  if (schema.enum && !schema.enum.includes(data)) fail(`${JSON.stringify(data)} not in enum`);
  if (schema.type) {
    const types = Array.isArray(schema.type) ? schema.type : [schema.type];
    const actual =
      data === null
        ? "null"
        : Array.isArray(data)
          ? "array"
          : Number.isInteger(data)
            ? "integer"
            : typeof data;
    const ok = types.some(
      (t) => t === actual || (t === "number" && (actual === "integer" || actual === "number")),
    );
    if (!ok) {
      fail(`expected ${types.join("|")}, got ${actual}`);
      return errs;
    }
  }
  if (typeof data === "number") {
    if ("minimum" in schema && data < schema.minimum) fail(`< minimum ${schema.minimum}`);
    if ("maximum" in schema && data > schema.maximum) fail(`> maximum ${schema.maximum}`);
  }
  if (Array.isArray(data) && schema.items) {
    data.forEach((v, i) => errs.push(...validate(schema.items, v, root, `${path}[${i}]`)));
  }
  if (data && typeof data === "object" && !Array.isArray(data)) {
    for (const key of schema.required ?? []) if (!(key in data)) fail(`missing required "${key}"`);
    for (const [key, value] of Object.entries(data)) {
      const sub = schema.properties?.[key];
      if (sub) errs.push(...validate(sub, value, root, `${path}.${key}`));
      else if (schema.additionalProperties === false) fail(`unexpected property "${key}"`);
      else if (typeof schema.additionalProperties === "object")
        errs.push(...validate(schema.additionalProperties, value, root, `${path}.${key}`));
    }
  }
  return errs;
}

// ─── Vocabulary ────────────────────────────────────────────────────────────
const STAGE = { pending: 1, ready: 2, in_progress: 3, in_review: 4, completed: 5 };
const EXCEPTION_STAGE = { failed: 3, blocked: 3, deferred: 1 };
const CLOSED = new Set(["completed", "deferred"]);
const STARTED = new Set(["in_progress", "in_review", "completed", "failed", "blocked"]);
const GATE_VOCABULARY = new Set(["not_started", "scripted_passed", "passed", "deferred", "waived"]);

const VERBS = {
  failed: "reset",
  amendment_pending: "approve",
  in_review: "review",
  calibration_due: "/aep-calibrate",
  draft_object_map: "/aep-model",
  open_question: "answer",
};
const RANK = {
  failed: 1,
  amendment_pending: 2,
  in_review: 3,
  calibration_due: 4,
  draft_object_map: 5,
  open_question: 6,
};

const isProse = (v) =>
  typeof v === "string" && v.trim().length > 0 && Number.isNaN(Number(v.trim()));
const num = (v) => (typeof v === "number" && Number.isFinite(v) ? v : null);

function git(args, cwd) {
  try {
    return execFileSync("git", args, { cwd, encoding: "utf8" }).trim();
  } catch {
    return null;
  }
}

// ─── Attention set ─────────────────────────────────────────────────────────
export function deriveAttentionSet(ctx, currentLayer) {
  const items = [];
  const absent = [];
  const stories = ctx.stories ?? [];

  for (const s of stories) {
    if (s.status === "failed") {
      items.push({
        rank: RANK.failed,
        kind: "failed",
        verb: VERBS.failed,
        id: String(s.id),
        title: String(s.title ?? s.id),
        layer: num(s.layer),
        path: `stories[id=${s.id}].status`,
        why: isProse(s.business_value) ? String(s.business_value) : null,
        predicates: storyPredicates(s),
      });
    }
  }

  // in_review, waived by the layer gate's skip_human_eval
  const gates = ctx.layer_gates ?? [];
  const gateByLayer = new Map(gates.map((g) => [g.layer, g]));
  const anySkipField = gates.some((g) => "skip_human_eval" in g);
  if (!anySkipField && stories.some((s) => s.status === "in_review")) {
    absent.push({
      predicate: "in_review_waiver",
      path: "layer_gates[].skip_human_eval",
      reason: "field absent in this schema version — in_review stories are reported unwaived",
    });
  }
  for (const s of stories) {
    if (s.status !== "in_review") continue;
    const waiver = gateByLayer.get(s.layer)?.skip_human_eval;
    if (waiver === "all") continue;
    items.push({
      rank: RANK.in_review,
      kind: "in_review",
      verb: VERBS.in_review,
      id: String(s.id),
      title: String(s.title ?? s.id),
      layer: num(s.layer),
      path: `stories[id=${s.id}].status`,
      why: isProse(s.business_value) ? String(s.business_value) : null,
    });
  }

  // amendment_log — field-level tolerance
  const amendments = ctx.architecture?.amendment_log ?? [];
  if (amendments.length > 0 && !amendments.some((a) => "status" in a)) {
    absent.push({
      predicate: "amendment_pending",
      path: "architecture.amendment_log[].status",
      reason: "field absent in this schema version — pending amendments are not derivable",
    });
  } else {
    amendments.forEach((a, i) => {
      if (a.status !== "pending") return;
      items.push({
        rank: RANK.amendment_pending,
        kind: "amendment_pending",
        verb: VERBS.amendment_pending,
        id: `amendment-${i + 1}`,
        title: String(a.proposed_change ?? a.summary ?? `amendment ${i + 1}`),
        layer: null,
        path: `architecture.amendment_log[${i}].status`,
        why: isProse(a.reasoning) ? String(a.reasoning) : null,
      });
    });
  }

  // calibration — plan minus history, never a status field
  const plan = ctx.calibration?.plan ?? [];
  const history = ctx.calibration?.history ?? [];
  // Layer ids arrive as numbers in some history entries and strings in others
  // (real consumer data). Normalize both sides of the join: `7` and `7.0` and
  // `"7"` are one layer, and an unnormalized miss silently reports a completed
  // calibration as still due — the field-typing rule, applied to a join key.
  const layerKey = (v) => {
    const n = Number(v);
    return Number.isFinite(n) ? String(n) : String(v ?? "");
  };
  const done = new Set(
    history.map((h) => `${layerKey(h.calibrated_from_layer ?? h.layer)}|${h.dimension}`),
  );
  plan.forEach((p, i) => {
    const layer = num(p.layer);
    if (layer === null || currentLayer === null || layer > currentLayer) return;
    for (const dim of p.dimensions ?? []) {
      if (done.has(`${layerKey(layer)}|${dim}`)) continue;
      items.push({
        rank: RANK.calibration_due,
        kind: "calibration_due",
        verb: VERBS.calibration_due,
        id: `calibration-${layer}-${dim}`,
        title: String(dim),
        layer,
        path: `calibration.plan[${i}]`,
        why: isProse(p.trigger) ? String(p.trigger) : null,
      });
    }
  });

  // open questions
  if (!ctx.product) {
    absent.push({
      predicate: "open_question",
      path: "product.open_questions[]",
      reason: "section absent in this schema version — open questions are not derivable",
    });
  } else {
    (ctx.product.open_questions ?? []).forEach((q, i) => {
      items.push({
        rank: RANK.open_question,
        kind: "open_question",
        verb: VERBS.open_question,
        id: `open-question-${i + 1}`,
        title: typeof q === "string" ? q : String(q.question ?? `question ${i + 1}`),
        layer: null,
        path: `product.open_questions[${i}]`,
        why: null,
      });
    });
  }

  // object maps live outside product-context.yaml; not derivable from it alone
  absent.push({
    predicate: "draft_object_map",
    path: "product/maps/*/object-map.yaml#status",
    reason: "object maps are separate artifacts — not derivable from product-context.yaml alone",
  });

  items.sort(
    (a, b) =>
      a.rank - b.rank ||
      (a.layer ?? Infinity) - (b.layer ?? Infinity) ||
      (a.id < b.id ? -1 : a.id > b.id ? 1 : 0),
  );
  return { attention_set: items, schema_absent: absent };
}

// ─── Drift facts ───────────────────────────────────────────────────────────
export function deriveDriftFacts(ctx) {
  const facts = [];
  const stories = ctx.stories ?? [];
  const gates = ctx.layer_gates ?? [];

  const byLayer = new Map();
  for (const s of stories) {
    if (!byLayer.has(s.layer)) byLayer.set(s.layer, []);
    byLayer.get(s.layer).push(s);
  }

  // 1 · intent without evidence — counters, NOT coverage.uncovered
  for (const g of gates) {
    const c = g.coverage;
    if (!c) continue;
    const total = num(c.criteria_total);
    const covered = num(c.criteria_covered);
    if (total === null || covered === null || total - covered <= 0) continue;
    // Scope: layers with work done or underway. `pending` and `ready` are both
    // plan — a dispatchable story is not a started one — so a gate with zero
    // covered criteria above them is intent, not drift.
    const inLayer = byLayer.get(g.layer) ?? [];
    const started = inLayer.some((s) => STARTED.has(s.status));
    if (!started) continue;
    facts.push({
      kind: "intent_without_evidence",
      layer: num(g.layer),
      detail: `${total} acceptance criteria declared, ${covered} covered`,
      path: `layer_gates[layer=${g.layer}].coverage`,
      level: "derived",
      counters: {
        criteria_total: total,
        criteria_covered: covered,
        uncovered_listed: (c.uncovered ?? []).length,
      },
    });
  }

  // 2 · plan behind the architecture — subject to field tolerance
  const amendments = ctx.architecture?.amendment_log ?? [];
  if (amendments.some((a) => "status" in a)) {
    amendments.forEach((a, i) => {
      if (a.status !== "pending") return;
      facts.push({
        kind: "plan_behind_architecture",
        layer: null,
        detail: String(a.proposed_change ?? a.summary ?? `amendment ${i + 1}`),
        path: `architecture.amendment_log[${i}]`,
        level: "derived",
      });
    });
  }

  // 3 · reality resisting intent — open stories only
  for (const s of stories) {
    const logs = s.failure_logs ?? [];
    if (logs.length === 0 || CLOSED.has(s.status)) continue;
    facts.push({
      kind: "reality_resisting_intent",
      layer: num(s.layer),
      detail: `${s.id} has ${logs.length} failure log${logs.length === 1 ? "" : "s"} and is still open (${s.status})`,
      path: `stories[id=${s.id}].failure_logs`,
      level: "derived",
      counters: { failure_logs: logs.length },
    });
  }

  // 4 · control-plane incoherence — >=1 completed story under a not_started gate
  for (const g of gates) {
    if (g.status !== "not_started") continue;
    const inLayer = byLayer.get(g.layer) ?? [];
    const completed = inLayer.filter((s) => s.status === "completed").length;
    if (completed === 0) continue;
    facts.push({
      kind: "control_plane_incoherence",
      layer: num(g.layer),
      detail: `${completed} completed stor${completed === 1 ? "y" : "ies"} under a not_started gate`,
      path: `layer_gates[layer=${g.layer}].status`,
      level: "derived",
      counters: { completed_stories: completed, layer_stories: inLayer.length },
    });
  }

  return facts;
}

// ─── Structure: Now · Concepts · Next · Options (D3/D8) ────────────────────
// The Engineering band's plane. Every value here is measured from the work
// record — `stories[].module` says which concept a piece of work belongs to,
// `stories[].files_affected` says where it landed. The concept-to-code binding
// is therefore observed, never declared and never guessed.

const OPEN_WORK = new Set(["pending", "ready", "in_progress"]);

// Fixed thresholds (D8). Configurable thresholds are knobs nobody turns.
export const OPTION_TRIGGERS = {
  CROWDING_MIN_CONCEPTS: 4,
  CROWDING_MIN_FILES: 30,
  CROWDING_MIN_DISJOINT: 0.5,
};

const resolverFor = (codeGraph) => {
  const dirs = (codeGraph?.nodes_detail ?? [])
    .map((n) => [n.dir, n.name])
    .sort((a, b) => b[0].length - a[0].length);
  return (p) => dirs.find(([dir]) => String(p).startsWith(dir + "/"))?.[1] ?? null;
};

export function deriveStructure(ctx, codeGraph) {
  if (!codeGraph) return null;
  const toUnit = resolverFor(codeGraph);
  const stories = ctx.stories ?? [];
  const declared = new Set((ctx.architecture?.modules ?? []).map((m) => String(m.name)));
  const responsibility = new Map(
    (ctx.architecture?.modules ?? []).map((m) => [String(m.name), m.responsibility ?? null]),
  );

  // Concept → files, split by whether the work is done or still queued.
  const filesOf = (pool) => {
    const out = new Map();
    for (const s of pool) {
      const mod = s.module == null ? null : String(s.module);
      if (!mod) continue;
      if (!out.has(mod)) out.set(mod, new Set());
      for (const p of s.files_affected ?? []) out.get(mod).add(String(p));
    }
    return out;
  };
  const doneFiles = filesOf(stories.filter((s) => s.status === "completed"));
  const nextFiles = filesOf(stories.filter((s) => OPEN_WORK.has(s.status)));
  const used = new Set(
    stories.map((s) => (s.module == null ? null : String(s.module))).filter(Boolean),
  );

  // Concepts: the vocabulary, bound to the code units that actually carry it.
  const concepts = [...used].sort().map((m) => {
    const files = doneFiles.get(m) ?? new Set();
    const units = [...new Set([...files].map(toUnit).filter(Boolean))].sort();
    return {
      module: m,
      declared: declared.has(m),
      responsibility: isProse(responsibility.get(m)) ? String(responsibility.get(m)) : null,
      files: files.size,
      units,
    };
  });

  // Next: the queued design projected onto what exists.
  const nextConcepts = [...nextFiles.keys()].sort().map((m) => {
    const files = nextFiles.get(m);
    const units = [...new Set([...files].map(toUnit).filter(Boolean))].sort();
    return {
      module: m,
      declared: declared.has(m),
      net_new: !doneFiles.has(m),
      responsibility: isProse(responsibility.get(m)) ? String(responsibility.get(m)) : null,
      files: files.size,
      units,
      homeless: units.length === 0,
    };
  });

  // Concept load per code unit, restricted to the Next projection (D8 scope:
  // running these detectors over history is measured noise, not signal).
  const perUnit = new Map();
  for (const m of nextFiles.keys()) {
    for (const p of nextFiles.get(m)) {
      const unit = toUnit(p);
      if (!unit) continue;
      if (!perUnit.has(unit)) perUnit.set(unit, new Map());
      if (!perUnit.get(unit).has(m)) perUnit.get(unit).set(m, new Set());
      perUnit.get(unit).get(m).add(p);
    }
  }

  const options = [];
  for (const [unit, byModule] of [...perUnit].sort()) {
    const mods = [...byModule.keys()].sort();
    const mass = new Set([...byModule.values()].flatMap((s) => [...s])).size;
    if (mods.length < OPTION_TRIGGERS.CROWDING_MIN_CONCEPTS) continue;
    if (mass < OPTION_TRIGGERS.CROWDING_MIN_FILES) continue;
    let pairs = 0;
    let disjoint = 0;
    const seams = [];
    for (let i = 0; i < mods.length; i++) {
      for (let j = i + 1; j < mods.length; j++) {
        pairs++;
        const shared = [...byModule.get(mods[i])].filter((p) =>
          byModule.get(mods[j]).has(p),
        ).length;
        if (shared === 0) disjoint++;
        seams.push({ a: mods[i], b: mods[j], shared });
      }
    }
    const share = pairs === 0 ? 1 : disjoint / pairs;
    if (share < OPTION_TRIGGERS.CROWDING_MIN_DISJOINT) continue;
    // The cleanest first cut: the concept sharing fewest files with the rest.
    const sharedTotal = new Map(mods.map((m) => [m, 0]));
    for (const s of seams) {
      sharedTotal.set(s.a, sharedTotal.get(s.a) + s.shared);
      sharedTotal.set(s.b, sharedTotal.get(s.b) + s.shared);
    }
    const cleanest = [...sharedTotal].sort((a, b) => a[1] - b[1] || a[0].localeCompare(b[0]))[0];
    options.push({
      trigger: "concept_crowding",
      unit,
      concepts: mods,
      concept_count: mods.length,
      files: mass,
      pairs,
      disjoint_pairs: disjoint,
      disjoint_share: Number(share.toFixed(2)),
      cleanest_seam: { module: cleanest[0], shared_files: cleanest[1] },
      path: "stories[status=open].module × files_affected",
    });
  }
  for (const c of nextConcepts.filter((c) => c.homeless)) {
    options.push({
      trigger: "homeless_concept",
      unit: null,
      concepts: [c.module],
      concept_count: 1,
      files: c.files,
      pairs: 0,
      disjoint_pairs: 0,
      disjoint_share: 1,
      cleanest_seam: null,
      path: `stories[module=${c.module}].files_affected`,
    });
  }

  const codeBound = concepts.filter((c) => c.units.length > 0).length;
  return {
    now: { units: codeGraph.nodes, edges: codeGraph.edges, commit: codeGraph.commit },
    concepts,
    concepts_total: concepts.length,
    concepts_code_bound: codeBound,
    used_not_declared: [...used].filter((m) => !declared.has(m)).sort(),
    declared_not_used: [...declared].filter((m) => !used.has(m)).sort(),
    next: nextConcepts,
    next_net_new: nextConcepts.filter((c) => c.net_new).map((c) => c.module),
    options,
  };
}

// ─── Predicates (D9 mechanism 3) ───────────────────────────────────────────
// Prose wants to say "retries are exhausted", "the sign-off was withdrawn",
// "eight landed in two days". Each of those must exist as a derived predicate,
// because an agent asked to state something the facts do not carry will supply
// it from a diagram label or by counting manually — which is exactly how this
// pipeline shipped four false sentences.

const DISCLAIMS_ROOT =
  /\b(unproven|not (?:yet )?(?:proven|established|determined)|undetermined|unknown|inconclusive|requires? (?:a )?(?:bounded |focused )?(?:diagnostic |transport-boundary )?follow-up)\b/i;

export function storyPredicates(s) {
  const attempts = num(s.attempt_count);
  const max = num(s.max_retries);
  const logs = s.failure_logs ?? [];
  const logText = logs.map((l) => (typeof l === "string" ? l : JSON.stringify(l))).join(" ");
  return {
    // "automatic repair has already given up" is only true when this is true.
    retries_exhausted: attempts !== null && max !== null ? attempts >= max : null,
    attempts_used: attempts,
    attempts_allowed: max,
    failure_log_count: logs.length,
    // A failure log that disclaims its own root cause forbids the surface from
    // asserting one. Silence over diagnosis.
    root_cause_stated: logs.length === 0 ? null : !DISCLAIMS_ROOT.test(logText),
  };
}

export function gatePredicates(g) {
  const closure = g.closure_status ? String(g.closure_status) : null;
  const realign = g.decision_realignment ?? null;
  return {
    closure_status: closure,
    // A withdrawn sign-off is not a pending one. Without this the brief says
    // "the last thing standing before sign-off" about a layer whose acceptance
    // criterion an owner has retired.
    sign_off_withdrawn: closure !== null && closure !== "open",
    successor_layer: realign ? num(realign.successor_layer) : null,
    retired_acceptance:
      realign && isProse(realign.retired_acceptance) ? String(realign.retired_acceptance) : null,
  };
}

// "N landed in the last M days" must be counted by the deriver, never by hand.
export function completionsInWindow(stories, days, asOf) {
  const cutoff = new Date(asOf);
  cutoff.setDate(cutoff.getDate() - days);
  const iso = cutoff.toISOString().slice(0, 10);
  const landed = stories.filter(
    (s) => s.status === "completed" && String(s.completed_at ?? "").slice(0, 10) >= iso,
  );
  return { days, since: iso, count: landed.length, ids: landed.map((s) => String(s.id)).sort() };
}

// ─── Cost: roll up from the work record, not the roll-up field ─────────────
export function deriveCost(ctx) {
  const stories = ctx.stories ?? [];
  const priced = stories.filter((s) => typeof s.cost_usd === "number" && s.cost_usd > 0);
  const derivedTotal = priced.reduce((sum, s) => sum + s.cost_usd, 0);
  const byModule = {};
  for (const s of priced) {
    const m = s.module == null ? "unattributed" : String(s.module);
    byModule[m] = Number(((byModule[m] ?? 0) + s.cost_usd).toFixed(2));
  }
  const declaredTotal = num(ctx.cost?.total_usd);
  return {
    total_usd: declaredTotal,
    derived_total_usd: Number(derivedTotal.toFixed(2)),
    priced_story_count: priced.length,
    story_total: stories.length,
    by_module: byModule,
    // A zeroed roll-up beside a non-zero story sum is a statement about the
    // roll-up, not about the data — and their disagreement is itself drift.
    rollup_disagrees: declaredTotal !== null && Math.abs(declaredTotal - derivedTotal) > 0.01,
    instrumented: derivedTotal > 0,
  };
}

// ─── Main derivation ───────────────────────────────────────────────────────
export async function derive({
  contextPath,
  manifestPath,
  repoRoot,
  codeGraph = null,
  censusResult = null,
}) {
  const ctx = await loadYaml(contextPath);
  const stories = ctx.stories ?? [];
  const gates = ctx.layer_gates ?? [];

  const byStatus = {};
  for (const s of stories) byStatus[s.status] = (byStatus[s.status] ?? 0) + 1;
  const openTotal = stories.filter((s) => !CLOSED.has(s.status)).length;

  const openLayers = stories.filter((s) => !CLOSED.has(s.status)).map((s) => s.layer);
  const currentLayer = openLayers.length ? Math.min(...openLayers.map(Number)) : null;

  const { attention_set, schema_absent } = deriveAttentionSet(ctx, currentLayer);
  const drift = deriveDriftFacts(ctx);

  const commit = git(["rev-parse", "--short", "HEAD"], repoRoot) ?? "unknown";
  let baselineCommit = null;
  if (manifestPath && existsSync(manifestPath)) {
    try {
      baselineCommit = JSON.parse(readFileSync(manifestPath, "utf8")).commit ?? null;
    } catch {
      baselineCommit = null;
    }
  }

  // Delta vs baseline: re-read the YAML at the baseline commit when reachable.
  const delta = {
    first_run: baselineCommit === null,
    commits_since_baseline: null,
    status_changes: [],
    changelog_new: 0,
  };
  if (baselineCommit) {
    const count = git(["rev-list", "--count", `${baselineCommit}..HEAD`], repoRoot);
    delta.commits_since_baseline = count === null ? null : Number(count);
    const rel = git(["ls-files", "--full-name", contextPath], repoRoot);
    if (rel) {
      try {
        const prevText = execFileSync("git", ["show", `${baselineCommit}:${rel}`], {
          cwd: repoRoot,
          encoding: "utf8",
          maxBuffer: 256 * 1024 * 1024,
        });
        const tmp = resolve(repoRoot, ".aep-baseline-context.yaml");
        writeFileSync(tmp, prevText);
        const prev = await loadYaml(tmp);
        execFileSync("rm", ["-f", tmp]);
        const prevById = new Map((prev.stories ?? []).map((s) => [String(s.id), s.status]));
        for (const s of stories) {
          const before = prevById.get(String(s.id)) ?? null;
          if (before !== s.status)
            delta.status_changes.push({ id: String(s.id), from: before, to: s.status });
        }
        delta.changelog_new = (ctx.changelog ?? []).length - (prev.changelog ?? []).length;
      } catch {
        /* baseline unreachable — delta stays first-run shaped */
      }
    }
  }

  const gateDetail = gates.map((g) => ({
    layer: num(g.layer),
    status: String(g.status ?? "unknown"),
    criteria_total: num(g.coverage?.criteria_total),
    criteria_covered: num(g.coverage?.criteria_covered),
    completed_at: g.completed_at ? String(g.completed_at) : null,
    ...gatePredicates(g),
    // A withdrawn sign-off is not shippable either — closure retires the
    // acceptance rather than granting it.
    shippable: g.status === "passed" && !gatePredicates(g).sign_off_withdrawn,
  }));
  const gateByStatus = {};
  for (const g of gateDetail) gateByStatus[g.status] = (gateByStatus[g.status] ?? 0) + 1;
  const vocabularyViolations = gateDetail
    .filter((g) => !GATE_VOCABULARY.has(g.status))
    .map((g) => ({
      layer: g.layer,
      status: g.status,
      path: `layer_gates[layer=${g.layer}].status`,
    }));

  const layerIds = [
    ...new Set([...stories.map((s) => Number(s.layer)), ...gates.map((g) => Number(g.layer))]),
  ]
    .filter((n) => Number.isFinite(n))
    .sort((a, b) => a - b);
  const gateStatusByLayer = new Map(gateDetail.map((g) => [g.layer, g.status]));
  const layers = layerIds.map((layer) => {
    const inLayer = stories.filter((s) => Number(s.layer) === layer);
    const counts = {};
    for (const s of inLayer) counts[s.status] = (counts[s.status] ?? 0) + 1;
    return {
      layer,
      counts,
      total: inLayer.length,
      gate_status: gateStatusByLayer.get(layer) ?? null,
      is_current: layer === currentLayer,
      stories: inLayer.map((s) => ({
        id: String(s.id),
        title: String(s.title ?? s.id),
        status: String(s.status),
        stage: STAGE[s.status] ?? EXCEPTION_STAGE[s.status] ?? 1,
        exception: s.status in EXCEPTION_STAGE ? String(s.status) : null,
        why: isProse(s.business_value) ? String(s.business_value) : null,
      })),
    };
  });

  // The changelog is NOT stored in date order (26 out-of-order adjacent pairs in
  // the reference consumer). Slicing the tail therefore misses the newest
  // entries — including, once, the owner decision that closed the current layer.
  // Sort before slicing; keep the original index so ids stay stable.
  const changelog = (ctx.changelog ?? []).map((e, i) => ({ e, i }));
  const changelogSorted = [...changelog].sort(
    (a, b) => String(a.e.date ?? "").localeCompare(String(b.e.date ?? "")) || a.i - b.i,
  );
  const changelogRecent = changelogSorted.slice(-12).map(({ e, i }) => ({
    id: `${e.date ?? "undated"}-${e.type ?? "entry"}-${i}`,
    date: String(e.date ?? ""),
    kind: e.type ? String(e.type) : null,
    author: e.author ? String(e.author) : null,
    summary: String(e.summary ?? ""),
    since_baseline: delta.changelog_new > 0 && i >= changelog.length - delta.changelog_new,
  }));

  const capabilities = gateDetail
    .filter((g) => !g.sign_off_withdrawn)
    .filter((g) => g.status === "passed" || g.status === "scripted_passed")
    .map((g) => ({
      layer: g.layer,
      gate_status: g.status,
      tense: g.status === "passed" ? "IS" : "EXP",
      settles_on: g.status === "scripted_passed" ? `layer ${g.layer} human acceptance run` : null,
      definition: (() => {
        const raw = gates.find((x) => num(x.layer) === g.layer)?.test_definition;
        return isProse(raw) ? String(raw) : null;
      })(),
      completed_at: g.completed_at,
      source: `layer_gates[layer=${g.layer}]`,
    }));

  const cost = deriveCost(ctx);
  const structure = deriveStructure(ctx, codeGraph);
  const facts = {
    schema_version: 1,
    generated_from: "product-context.yaml",
    commit,
    baseline_commit: baselineCommit,
    project: String(ctx.project ?? "unnamed project"),
    version: ctx.version ? String(ctx.version) : null,
    updated_at: ctx.updated_at ? String(ctx.updated_at) : null,
    story_total: stories.length,
    by_status: byStatus,
    open_total: openTotal,
    current_layer: currentLayer,
    attention_set,
    one_ask: attention_set[0] ?? null,
    schema_absent,
    drift_facts: drift,
    gates: {
      total: gates.length,
      // Keyed access. Positional indexing into a fact array is the same defect
      // the changelog binding already bans: `gates.detail[31]` silently means a
      // different layer the moment a half-step gate is inserted, and the audit
      // cannot catch it because the path still resolves.
      by_layer: Object.fromEntries(gateDetail.map((g) => [String(g.layer), g])),
      by_status: gateByStatus,
      vocabulary_violations: vocabularyViolations,
      detail: gateDetail,
    },
    layers,
    changelog_recent: changelogRecent,
    capabilities,
    cost,
    completions: [7, 2].map((d) =>
      completionsInWindow(stories, d, ctx.updated_at ?? new Date().toISOString().slice(0, 10)),
    ),
    modules: {
      declared_count: (ctx.architecture?.modules ?? []).length,
      path_bound_count: (ctx.architecture?.modules ?? []).filter(
        (m) => Array.isArray(m.paths) && m.paths.length > 0,
      ).length,
      used_count: structure ? structure.concepts_total : null,
      code_bound_count: structure ? structure.concepts_code_bound : null,
      used_not_declared: structure ? structure.used_not_declared : [],
      declared_not_used: structure ? structure.declared_not_used : [],
      names: (ctx.architecture?.modules ?? []).map((m) => String(m.name)),
    },
    structure,
    code_graph: codeGraph,
    reading_coverage: censusResult
      ? {
          populated_paths: censusResult.populated_paths,
          derived_count: censusResult.derived_count,
          ignored_count: censusResult.ignored_count,
          unhandled_count: censusResult.unhandled_count,
          read_coverage: censusResult.read_coverage,
        }
      : null,
    delta,
  };

  // Cost drift: a zeroed roll-up beside a non-zero story sum.
  if (cost.rollup_disagrees) {
    facts.drift_facts.push({
      kind: "control_plane_incoherence",
      layer: null,
      detail: `cost roll-up declares ${cost.total_usd} while ${cost.priced_story_count} stories sum to ${cost.derived_total_usd}`,
      path: "cost.total_usd vs stories[].cost_usd",
      level: "derived",
      counters: { priced_stories: cost.priced_story_count },
    });
  }

  // Module vocabulary drift: the control plane and the work using different words.
  if (structure && (structure.used_not_declared.length || structure.declared_not_used.length)) {
    facts.drift_facts.push({
      kind: "control_plane_incoherence",
      layer: null,
      detail: `${structure.used_not_declared.length} module names are used by stories but never declared, and ${structure.declared_not_used.length} declared modules have never been worked`,
      path: "architecture.modules[].name vs stories[].module",
      level: "derived",
      counters: {
        used_not_declared: structure.used_not_declared.length,
        declared_not_used: structure.declared_not_used.length,
      },
    });
  }

  // 5 · declared vs actual — only when a scan is present
  if (codeGraph) {
    const declared = new Set(facts.modules.names);
    const overlap = (codeGraph.package_names ?? []).filter((n) => declared.has(n)).length;
    // Only defined keys: an explicit `undefined` value is still a key, and the
    // schema types every property it declares.
    facts.code_graph = Object.fromEntries(
      Object.entries({
        nodes: codeGraph.nodes,
        edges: codeGraph.edges,
        apps: codeGraph.apps,
        packages: codeGraph.packages,
        domains: codeGraph.domains,
        domain_edges: codeGraph.domain_edges,
        name_overlap: overlap,
        tool: codeGraph.tool,
        commit: codeGraph.commit,
        coverage_complete: codeGraph.coverage?.complete,
        unscanned_count: codeGraph.coverage?.unscanned?.length,
      }).filter(([, v]) => v !== undefined),
    );
    // Drift 5 restated (revision 8). The declared architecture IS
    // code-addressable — through the work record. Report what the measurement
    // found, not the absence of a schema field.
    if (structure) {
      const unbound = structure.concepts.filter((c) => c.units.length === 0).length;
      facts.drift_facts.push({
        kind: "declared_vs_actual",
        layer: null,
        detail: `${structure.concepts_code_bound} of ${structure.concepts_total} concept modules resolve to real code through the work record; ${unbound} carry no resolvable path. The binding is measured, not declared — architecture.modules[].paths would record it, and ${facts.modules.path_bound_count} modules carry it today`,
        path: "stories[].module × stories[].files_affected",
        level: "code-verified",
        counters: {
          code_units: codeGraph.nodes,
          concepts_used: structure.concepts_total,
          concepts_code_bound: structure.concepts_code_bound,
          declared_modules: facts.modules.declared_count,
          paths_declared: facts.modules.path_bound_count,
        },
      });
    }
  }

  return facts;
}

// ─── CLI ───────────────────────────────────────────────────────────────────
function arg(name, fallback = null) {
  const i = process.argv.indexOf(`--${name}`);
  return i > -1 && process.argv[i + 1] ? process.argv[i + 1] : fallback;
}

const isMain =
  process.argv[1] && resolve(process.argv[1]) === resolve(fileURLToPath(import.meta.url));
if (isMain) {
  const contextPath = resolve(arg("context", "product-context.yaml"));
  const repoRoot = arg("repo", dirname(contextPath));
  const manifestPath = arg("manifest", resolve(repoRoot, "docs/human-alignment/manifest.json"));
  const outPath = arg("out", resolve(repoRoot, "docs/human-alignment/facts.json"));
  const graphPath = arg("code-graph");

  if (!existsSync(contextPath)) {
    console.error(`ERROR: ${contextPath} not found — run /aep-envision first.`);
    process.exit(2);
  }
  const codeGraph =
    graphPath && existsSync(graphPath) ? JSON.parse(readFileSync(graphPath, "utf8")) : null;

  const { census } = await import("./census.mjs");
  const censusManifest = JSON.parse(readFileSync(resolve(HERE, "source-census.json"), "utf8"));
  const censusResult = census(await loadYaml(contextPath), censusManifest);
  const facts = await derive({ contextPath, manifestPath, repoRoot, codeGraph, censusResult });
  const schema = JSON.parse(readFileSync(resolve(HERE, "facts.schema.json"), "utf8"));
  const errors = validate(schema, facts);
  if (errors.length) {
    console.error("ERROR: facts JSON failed its own schema:");
    for (const e of errors.slice(0, 20)) console.error("  " + e);
    process.exit(1);
  }
  const json = JSON.stringify(facts, null, 2);
  writeFileSync(outPath, json + "\n");
  console.log(`derive: ${outPath}`);
  console.log(
    `  ${facts.story_total} stories · ${facts.open_total} open · attention ${facts.attention_set.length} · drift ${facts.drift_facts.length} · skipped predicates ${facts.schema_absent.length}`,
  );
  console.log(
    `  sha256 ${createHash("sha256").update(json).digest("hex").slice(0, 16)} @ ${facts.commit}`,
  );
}
