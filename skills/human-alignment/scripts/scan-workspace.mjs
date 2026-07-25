#!/usr/bin/env node
// scan-workspace.mjs — Phase 1.5a of /aep-human-alignment.
//
// Reads the real package topology from the workspace manifest. No agent
// judgment: the same commit in produces the same graph out. This is the
// "code is the source of truth for edges" half of the drift-facts spec.
//
// Import-level resolution (dependency-cruiser / import-linter) is the later
// rung; package level is what every JS/TS workspace can answer today.
//
// Usage: node scan-workspace.mjs [--repo <path>] [--out <path>]

import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";

const readJson = (p) => JSON.parse(readFileSync(p, "utf8"));

// Expand a workspace glob ("apps/*", "packages/**") to directories holding a
// package.json. Deliberately supports only the two forms real manifests use.
function expandGlob(root, pattern) {
  const parts = pattern.split("/");
  let dirs = [root];
  for (const part of parts) {
    const next = [];
    for (const dir of dirs) {
      if (part === "*" || part === "**") {
        if (!existsSync(dir)) continue;
        for (const entry of readdirSync(dir)) {
          if (entry.startsWith(".") || entry === "node_modules") continue;
          const full = join(dir, entry);
          if (statSync(full).isDirectory()) {
            next.push(full);
            if (part === "**") dirs.push(full);
          }
        }
      } else {
        const full = join(dir, part);
        if (existsSync(full) && statSync(full).isDirectory()) next.push(full);
      }
    }
    dirs = next;
  }
  return dirs.filter((d) => existsSync(join(d, "package.json")));
}

export function scanWorkspace(repoRoot) {
  const rootPkgPath = join(repoRoot, "package.json");
  if (!existsSync(rootPkgPath)) return null;
  const rootPkg = readJson(rootPkgPath);
  const ws = rootPkg.workspaces;
  const patterns = Array.isArray(ws) ? ws : (ws?.packages ?? []);
  if (patterns.length === 0) return null;

  const dirs = [...new Set(patterns.flatMap((p) => expandGlob(repoRoot, p)))].sort();
  const packages = [];
  for (const dir of dirs) {
    const pkg = readJson(join(dir, "package.json"));
    if (!pkg.name) continue;
    packages.push({
      name: pkg.name,
      dir: relative(repoRoot, dir),
      private: pkg.private === true,
      deps: { ...pkg.dependencies, ...pkg.devDependencies, ...pkg.peerDependencies },
    });
  }

  const byName = new Map(packages.map((p) => [p.name, p]));
  const edges = [];
  for (const p of packages) {
    for (const dep of Object.keys(p.deps ?? {})) {
      if (!byName.has(dep) || dep === p.name) continue;
      edges.push({ from: p.name, to: dep });
    }
  }
  edges.sort((a, b) => a.from.localeCompare(b.from) || a.to.localeCompare(b.to));

  let commit = "unknown";
  try {
    commit = execFileSync("git", ["rev-parse", "--short", "HEAD"], {
      cwd: repoRoot,
      encoding: "utf8",
    }).trim();
  } catch {
    /* not a git repo — the graph is still valid, its provenance is not */
  }

  const topDir = (d) => d.split("/")[0];
  return {
    repo: rootPkg.name ?? relative(resolve(repoRoot, ".."), repoRoot),
    commit,
    tool: "scan-workspace.mjs (package.json workspace graph, deterministic)",
    nodes: packages.length,
    edges: edges.length,
    apps: packages.filter((p) => topDir(p.dir) === "apps").length,
    packages: packages.filter((p) => topDir(p.dir) === "packages").length,
    package_names: packages.map((p) => p.name),
    nodes_detail: packages.map((p) => ({ name: p.name, dir: p.dir, private: p.private })),
    edges_detail: edges,
  };
}

function arg(name, fallback = null) {
  const i = process.argv.indexOf(`--${name}`);
  return i > -1 && process.argv[i + 1] ? process.argv[i + 1] : fallback;
}

if (process.argv[1] && process.argv[1].endsWith("scan-workspace.mjs")) {
  const repoRoot = resolve(arg("repo", "."));
  const graph = scanWorkspace(repoRoot);
  if (!graph) {
    console.error(
      "scan-workspace: no workspace manifest found — the architecture view degrades to declared-only.",
    );
    process.exit(3);
  }
  const out = arg("out", join(repoRoot, "docs/human-alignment/code-graph.json"));
  writeFileSync(out, JSON.stringify(graph, null, 2) + "\n");
  console.log(`scan-workspace: ${out}`);
  console.log(
    `  ${graph.nodes} packages (${graph.apps} apps, ${graph.packages} packages) · ${graph.edges} edges @ ${graph.commit}`,
  );
}
