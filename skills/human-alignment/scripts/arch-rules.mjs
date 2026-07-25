#!/usr/bin/env node
// arch-rules.mjs — Phase 1.5b of /aep-human-alignment.
//
// Transforms the code graph into typed archify IR through an auditable rule
// table. The rules are DATA, not prose: determinism is the claim this file
// makes, so every rule must be executable and every output must be a pure
// function of (graph, declared architecture).
//
//   R1  exclude test/fixture packages
//   R2  fold ubiquitous dependencies (in-degree >= 60%) into a card
//   R3  layered/cascade layout (longest-path layering)
//   R4  semantic type map
//   R5  boundaries from directory structure
//   R6  transitive reduction
//   R7  package -> domain grouping
//   R8  domain-level edge aggregation
//   R9  generated guided views
//   R10 row-order search minimizing edge crossings
//
// Emits three tiers: overview (domains), package (full graph), declared (YAML).
//
// Usage: node arch-rules.mjs --code-graph <path> [--context <path>] --out-dir <dir>

import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

// ─── Rule table ────────────────────────────────────────────────────────────
export const RULES = {
  R1_EXCLUDE: /(^|[-/@])(test|tests|e2e|fixture|fixtures|mock|mocks|bench)($|[-/])/,
  R2_UBIQUITY: 0.6,
  // archify's component type enum is closed; every branch must land inside it.
  R4_TYPES: [
    [/(^|[-/])(web|ui|frontend|dashboard|site|client)($|[-/])/, "frontend"],
    [/(^|[-/])(db|database|schema|drizzle|prisma|store|migrations)($|[-/])/, "database"],
    [/(^|[-/])(auth|session|identity|security|secrets)($|[-/])/, "security"],
    [/(^|[-/])(infra|deploy|terraform|alchemy|cdk|cloud|worker|workers)($|[-/])/, "cloud"],
    [/(^|[-/])(queue|bus|event|protocol|sdk|mail|email|messaging|relay)($|[-/])/, "messagebus"],
  ],
  R4_DEFAULT: "backend",
  R4_ENUM: ["frontend", "backend", "database", "cloud", "security", "messagebus", "external"],
  R10_MAX_PERMUTATION_ROWS: 6,
};

const COL_X = 90;
const COL_STEP = 380;
const ROW_Y = 90;
const ROW_STEP = 130;

const unscoped = (name) => name.replace(/^@[^/]+\//, "");

// archify ids must match ^[a-zA-Z][a-zA-Z0-9_-]*$. Scoped package names
// (@scope/pkg) would otherwise produce a leading dash and fail validation, so
// every id in every tier goes through here — components, connections, views.
export const slug = (name) => {
  const s = String(name)
    .replace(/[^a-z0-9]+/gi, "-")
    .replace(/^-+|-+$/g, "")
    .toLowerCase();
  return /^[a-z]/.test(s) ? s : `n-${s}`;
};

// Layout gate: a label wider than its component is a hard failure. Size the box
// from the label instead of emitting a known-bad width and repairing it later.
const CHAR_PX = 7.4;
const sizeFor = (label, sublabel = "") => {
  const width = Math.max(
    170,
    Math.ceil(Math.max(String(label).length * CHAR_PX, String(sublabel).length * 6.2)) + 28,
  );
  return [Math.min(width, 320), 62];
};

// R4 — semantic type from name + directory.
export function semanticType(name, dir = "") {
  const probe = `${dir}/${unscoped(name)}`;
  for (const [re, type] of RULES.R4_TYPES) if (re.test(probe)) return type;
  return RULES.R4_DEFAULT;
}

// R6 — transitive reduction: drop an edge a→c when a path a→…→c of length ≥ 2
// already exists. Keeps the graph readable without changing reachability.
export function transitiveReduction(nodes, edges) {
  const out = new Map(nodes.map((n) => [n, new Set()]));
  for (const e of edges) out.get(e.from)?.add(e.to);
  const reachable = (from, skipTo) => {
    const seen = new Set();
    const stack = [...(out.get(from) ?? [])].filter((n) => n !== skipTo);
    while (stack.length) {
      const cur = stack.pop();
      if (seen.has(cur)) continue;
      seen.add(cur);
      for (const nxt of out.get(cur) ?? []) stack.push(nxt);
    }
    return seen;
  };
  return edges.filter((e) => !reachable(e.from, e.to).has(e.to));
}

// R3 — longest-path layering. A node sits one column right of its deepest
// dependent, so arrows read left-to-right and never point backwards.
export function layer(nodes, edges) {
  const deps = new Map(nodes.map((n) => [n, new Set()]));
  for (const e of edges) deps.get(e.from)?.add(e.to);
  const depth = new Map();
  const visiting = new Set();
  const walk = (n) => {
    if (depth.has(n)) return depth.get(n);
    if (visiting.has(n)) return 0; // cycle: break deterministically
    visiting.add(n);
    let d = 0;
    for (const t of deps.get(n) ?? []) d = Math.max(d, walk(t) + 1);
    visiting.delete(n);
    depth.set(n, d);
    return d;
  };
  for (const n of [...nodes].sort()) walk(n);
  const maxDepth = Math.max(0, ...depth.values());
  // invert so dependents (depth high) sit left, leaves right
  return new Map([...depth].map(([n, d]) => [n, maxDepth - d]));
}

// R10 — order rows within each column to minimize crossings. Full permutation
// search up to a bound, barycenter heuristic beyond it. Both deterministic.
export function orderRows(columns, edges) {
  const posOf = new Map();
  columns.forEach((col, ci) => col.forEach((n, ri) => posOf.set(n, { ci, ri })));
  const crossings = (assign) => {
    let count = 0;
    const rowOf = (n) => assign.get(n) ?? posOf.get(n)?.ri ?? 0;
    for (let i = 0; i < edges.length; i++) {
      for (let j = i + 1; j < edges.length; j++) {
        const a = edges[i];
        const b = edges[j];
        const ca = posOf.get(a.from)?.ci;
        const cb = posOf.get(b.from)?.ci;
        if (ca === undefined || ca !== cb) continue;
        const a1 = rowOf(a.from);
        const a2 = rowOf(a.to);
        const b1 = rowOf(b.from);
        const b2 = rowOf(b.to);
        if ((a1 - b1) * (a2 - b2) < 0) count++;
      }
    }
    return count;
  };
  const permutations = (arr) =>
    arr.length <= 1
      ? [arr]
      : arr.flatMap((v, i) =>
          permutations([...arr.slice(0, i), ...arr.slice(i + 1)]).map((p) => [v, ...p]),
        );

  return columns.map((col, ci) => {
    if (col.length <= 1) return col;
    if (col.length <= RULES.R10_MAX_PERMUTATION_ROWS) {
      let best = col;
      let bestScore = Infinity;
      for (const perm of permutations([...col].sort())) {
        const assign = new Map(perm.map((n, ri) => [n, ri]));
        const score = crossings(assign);
        if (score < bestScore) {
          bestScore = score;
          best = perm;
        }
      }
      best.forEach((n, ri) => posOf.set(n, { ci, ri }));
      return best;
    }
    // barycenter: order by mean row of neighbours, ties by name
    const neighbourRow = (n) => {
      const rows = edges
        .filter((e) => e.from === n || e.to === n)
        .map((e) => posOf.get(e.from === n ? e.to : e.from)?.ri ?? 0);
      return rows.length ? rows.reduce((a, b) => a + b, 0) / rows.length : 0;
    };
    const sorted = [...col].sort((a, b) => neighbourRow(a) - neighbourRow(b) || a.localeCompare(b));
    sorted.forEach((n, ri) => posOf.set(n, { ci, ri }));
    return sorted;
  });
}

function place(nodes, edges) {
  const depth = layer(nodes, edges);
  const maxCol = Math.max(0, ...depth.values());
  const columns = [];
  for (let c = 0; c <= maxCol; c++)
    columns.push([...nodes].filter((n) => depth.get(n) === c).sort());
  const ordered = orderRows(columns, edges);
  const pos = new Map();
  ordered.forEach((col, ci) =>
    col.forEach((n, ri) => pos.set(n, [COL_X + ci * COL_STEP, ROW_Y + ri * ROW_STEP])),
  );
  return pos;
}

// R9 — one guided view per hub: the node plus what it directly depends on.
function guidedViews(nodes, edges, labelOf) {
  const outDeg = new Map(nodes.map((n) => [n, 0]));
  for (const e of edges) outDeg.set(e.from, (outDeg.get(e.from) ?? 0) + 1);
  const hubs = [...outDeg.entries()]
    .filter(([, d]) => d > 0)
    .sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0]))
    .slice(0, 3)
    .map(([n]) => n);
  return hubs.map((hub) => ({
    id: `${slug(hub)}-view`,
    label: `what ${labelOf(hub)} is built on`,
    focus: [hub, ...edges.filter((e) => e.from === hub).map((e) => e.to)].slice(0, 6).map(slug),
    note: `${labelOf(hub)} and everything it depends on directly.`,
  }));
}

// ─── Tier builders ─────────────────────────────────────────────────────────
export function buildPackageTier(graph) {
  // R1
  const kept = graph.nodes_detail.filter(
    (n) => !RULES.R1_EXCLUDE.test(`${n.dir}/${unscoped(n.name)}`),
  );
  const keptNames = new Set(kept.map((n) => n.name));
  let edges = graph.edges_detail.filter((e) => keptNames.has(e.from) && keptNames.has(e.to));

  // R2 — fold ubiquitous dependencies out of the graph and into a card
  const inDeg = new Map(kept.map((n) => [n.name, 0]));
  for (const e of edges) inDeg.set(e.to, (inDeg.get(e.to) ?? 0) + 1);
  const threshold = Math.ceil(kept.length * RULES.R2_UBIQUITY);
  const folded = kept.filter((n) => (inDeg.get(n.name) ?? 0) >= threshold).map((n) => n.name);
  const foldedSet = new Set(folded);
  const nodes = kept.filter((n) => !foldedSet.has(n.name));
  const names = nodes.map((n) => n.name);
  edges = edges.filter((e) => !foldedSet.has(e.from) && !foldedSet.has(e.to));

  // R6
  edges = transitiveReduction(names, edges);
  const pos = place(names, edges);

  const components = nodes
    .map((n) => ({
      id: slug(n.name),
      type: semanticType(n.name, n.dir),
      label: unscoped(n.name),
      sublabel: n.dir,
      pos: pos.get(n.name),
      size: sizeFor(unscoped(n.name), n.dir),
    }))
    .sort((a, b) => a.id.localeCompare(b.id));
  const idOf = new Map(nodes.map((n) => [n.name, slug(n.name)]));

  // R5 — boundaries from the top directory segment
  const groups = new Map();
  for (const n of nodes) {
    const top = n.dir.split("/")[0];
    if (!groups.has(top)) groups.set(top, []);
    groups.get(top).push(idOf.get(n.name));
  }
  const boundaries = [...groups.entries()]
    .filter(([, members]) => members.length > 1)
    .sort()
    .map(([top, members]) => ({ kind: "region", label: top, wraps: members.sort() }));

  return {
    schema_version: 1,
    diagram_type: "architecture",
    meta: {
      title: `${graph.repo} · package graph`,
      subtitle: `deterministic scan @ ${graph.commit} · ${components.length} packages · ${edges.length} edges after folding and transitive reduction`,
      output: "architecture-packages.html",
      quality_profile: "standard",
      views: guidedViews(names, edges, (n) => unscoped(n)),
    },
    components,
    boundaries,
    connections: edges
      .map((e) => ({ from: idOf.get(e.from), to: idOf.get(e.to) }))
      .sort((a, b) => a.from.localeCompare(b.from) || a.to.localeCompare(b.to)),
    cards: [
      {
        dot: "cyan",
        title: "How this graph was reduced (auditable rules)",
        items: [
          `R1 excluded test packages · R2 folded ${folded.length} ubiquitous dependenc${folded.length === 1 ? "y" : "ies"} (in-degree >= ${threshold})`,
          `R6 transitive reduction · R10 row order minimizing crossings`,
          folded.length
            ? `folded: ${folded.map(unscoped).sort().join(" · ")}`
            : "nothing met the ubiquity threshold",
        ],
      },
      {
        dot: "emerald",
        title: "Scan receipt",
        items: [
          `${graph.nodes} packages, ${graph.edges} edges before rules`,
          `${components.length} nodes, ${edges.length} edges after`,
          `tool: ${graph.tool}`,
        ],
      },
    ],
  };
}

// R7 — concepts are BOUND to code units, never grouped by name. The binding is
// measured in derive.mjs from stories[].module x files_affected; this tier only
// renders it. There is no synthesized "domain" layer: inventing one from naming
// conventions produced 14 groups from 19 packages, 11 of them singletons, and
// discarded the project's own prose vocabulary (decision doc, revision 8).
export function buildNowTier(graph, structure) {
  const base = buildPackageTier(graph);
  const carried = new Map();
  for (const c of structure?.concepts ?? []) {
    for (const u of c.units) {
      if (!carried.has(u)) carried.set(u, []);
      carried.get(u).push(c.module);
    }
  }
  for (const comp of base.components) {
    const unit = (graph.nodes_detail ?? []).find((n) => slug(n.name) === comp.id)?.name;
    const concepts = (carried.get(unit) ?? []).sort();
    if (concepts.length) comp.sublabel = `${comp.sublabel} · ${concepts.length} concepts`;
  }
  base.meta.title = `${graph.repo} · what exists now`;
  base.meta.subtitle = `code-verified @ ${graph.commit} · ${base.components.length} of ${graph.nodes} units shown after R1/R2 reduction · ${structure?.concepts_code_bound ?? 0} of ${structure?.concepts_total ?? 0} concepts bound to code`;
  base.meta.output = "architecture-now.html";
  base.cards = [
    base.cards[0],
    {
      dot: "emerald",
      title: "Concepts carried, measured not declared",
      items: [
        `${structure?.concepts_code_bound ?? 0} of ${structure?.concepts_total ?? 0} concept modules resolve to a real unit`,
        `binding source: stories[].module x stories[].files_affected`,
        `${(structure?.used_not_declared ?? []).length} concepts are used by work but never declared`,
      ],
    },
  ];
  return base;
}

// Next — the queued design projected onto what exists. Units receiving net-new
// concepts are marked; this is the tier archify `compare` reads as "After".
export function buildNextTier(graph, structure) {
  const base = buildPackageTier(graph);
  const incoming = new Map();
  for (const c of structure?.next ?? []) {
    for (const u of c.units) {
      if (!incoming.has(u)) incoming.set(u, []);
      incoming.get(u).push(c.module);
    }
  }
  const byId = new Map((graph.nodes_detail ?? []).map((n) => [slug(n.name), n.name]));
  const untouched = [];
  for (const comp of base.components) {
    const unit = byId.get(comp.id);
    const arriving = (incoming.get(unit) ?? []).sort();
    if (arriving.length === 0) {
      untouched.push(comp.id);
      continue;
    }
    comp.sublabel = `${arriving.length} arriving`;
  }
  base.meta.title = `${graph.repo} · where the next design lands`;
  base.meta.subtitle = `${(structure?.next ?? []).length} concept modules in queued work, ${(structure?.next_net_new ?? []).length} of them net-new · ${base.components.length} of ${graph.nodes} units shown · projected from open stories`;
  base.meta.output = "architecture-next.html";
  base.cards = [
    {
      dot: "amber",
      title: "Net-new concepts the queued design introduces",
      items: (structure?.next_net_new ?? []).slice(0, 6).map((m) => m),
    },
    {
      dot: "cyan",
      title: "How this projection was made",
      items: [
        "open stories -> stories[].module -> stories[].files_affected -> code unit",
        `${untouched.length} existing units receive no queued work`,
        "no new code units are planned; every concept lands in something that exists",
      ],
    },
  ];
  return base;
}

export function buildDeclaredTier(ctx, graph) {
  const modules = (ctx?.architecture?.modules ?? []).slice(0, 14);
  if (modules.length === 0) return null;
  const names = modules.map((m) => String(m.name));
  const idOf = new Map(names.map((n) => [n, slug(n)]));
  const nameSet = new Set(names);
  let edges = [];
  for (const m of modules) {
    for (const dep of m.depends_on ?? []) {
      if (!nameSet.has(String(dep)) || String(dep) === String(m.name)) continue;
      edges.push({ from: String(m.name), to: String(dep) });
    }
  }
  edges = transitiveReduction(names, edges);
  const pos = place(names, edges);
  const overlap = graph ? names.filter((n) => (graph.package_names ?? []).includes(n)).length : 0;

  return {
    schema_version: 1,
    diagram_type: "architecture",
    meta: {
      title: `${ctx.project ?? "project"} · declared architecture`,
      subtitle: `authored narrative from product-context.yaml · ${modules.length} of ${(ctx.architecture?.modules ?? []).length} declared modules · unverified`,
      output: "architecture-declared.html",
      quality_profile: "standard",
    },
    components: modules
      .map((m) => ({
        id: idOf.get(String(m.name)),
        type: semanticType(String(m.name), String(m.kind ?? "")),
        label: String(m.name),
        sublabel: String(m.technology ?? m.kind ?? "declared module"),
        pos: pos.get(String(m.name)),
        size: sizeFor(String(m.name), String(m.technology ?? m.kind ?? "")),
      }))
      .sort((a, b) => a.id.localeCompare(b.id)),
    boundaries: [],
    connections: edges
      .map((e) => ({ from: idOf.get(e.from), to: idOf.get(e.to) }))
      .sort((a, b) => a.from.localeCompare(b.from) || a.to.localeCompare(b.to)),
    cards: [
      {
        dot: "amber",
        title: "This tier is meaning, not verification",
        items: [
          "Edges come from architecture.modules[].depends_on — what the plan says.",
          graph
            ? `${overlap} of ${names.length} declared names match a real package name.`
            : "No code scan available to compare against.",
          "Code is the source of truth for edges; disagreement is a drift fact.",
        ],
      },
    ],
  };
}

// ─── CLI ───────────────────────────────────────────────────────────────────
function arg(name, fallback = null) {
  const i = process.argv.indexOf(`--${name}`);
  return i > -1 && process.argv[i + 1] ? process.argv[i + 1] : fallback;
}

if (process.argv[1] && process.argv[1].endsWith("arch-rules.mjs")) {
  const { loadYaml } = await import("./derive.mjs");
  const graphPath = arg("code-graph");
  const outDir = resolve(arg("out-dir", "."));
  const contextPath = arg("context");
  if (!graphPath || !existsSync(graphPath)) {
    console.error("ERROR: --code-graph <path> is required (run scan-workspace.mjs first).");
    process.exit(2);
  }
  const graph = JSON.parse(readFileSync(graphPath, "utf8"));
  const ctx = contextPath && existsSync(contextPath) ? await loadYaml(contextPath) : null;

  const factsPath = arg("facts");
  const structure =
    factsPath && existsSync(factsPath)
      ? (JSON.parse(readFileSync(factsPath, "utf8")).structure ?? null)
      : null;
  const tiers = {
    "architecture-now": buildNowTier(graph, structure),
    "architecture-next": buildNextTier(graph, structure),
    "architecture-declared": buildDeclaredTier(ctx, graph),
  };
  for (const [name, ir] of Object.entries(tiers)) {
    if (!ir) continue;
    const out = join(outDir, `${name}.architecture.json`);
    writeFileSync(out, JSON.stringify(ir, null, 2) + "\n");
    console.log(
      `arch-rules: ${out} (${ir.components.length} components, ${ir.connections.length} connections)`,
    );
  }
}
