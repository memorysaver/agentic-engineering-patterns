#!/usr/bin/env node
// Derive each workspace's logical state from autopilot-state.json, by
// evaluating the ordered rules in references/tick-protocol.json.
//
// The tick used to recompute this from a prose table every time it wrote the
// status file, which is how a derivation drifts from the fields it derives
// from. Here the rules are data and this is the only evaluator, so the tick,
// `/aep-autopilot status`, and any post-mortem all read the same answer.
//
// Usage:
//   node scripts/derive-workspace-state.mjs [.dev-workflow/autopilot-state.json] [--json]
//
// Exit 0 on success, 2 on bad input.

import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const HERE = dirname(fileURLToPath(import.meta.url));
const PROTOCOL = join(HERE, "../references/tick-protocol.json");

const args = process.argv.slice(2);
const asJson = args.includes("--json");
const statePath = args.find((a) => !a.startsWith("--")) ?? ".dev-workflow/autopilot-state.json";

const OPS = {
  eq: (a, b) => a === b,
  neq: (a, b) => a !== b,
  gte: (a, b) => typeof a === "number" && a >= b,
  lte: (a, b) => typeof a === "number" && a <= b,
};

function matches(condition, workspace) {
  const clauses = condition.all ?? [];
  return clauses.every(({ field, op, value }) => {
    const compare = OPS[op];
    if (!compare) throw new Error(`unknown op "${op}" in tick-protocol.json`);
    return compare(workspace[field], value);
  });
}

let protocol, state;
try {
  protocol = JSON.parse(readFileSync(PROTOCOL, "utf8"));
} catch (error) {
  console.error(`FAIL: cannot read ${PROTOCOL} — ${error.message}`);
  process.exit(2);
}
try {
  state = JSON.parse(readFileSync(resolve(statePath), "utf8"));
} catch (error) {
  console.error(`FAIL: cannot read ${statePath} — ${error.message}`);
  process.exit(2);
}

const { rules, fallback, flags } = protocol.derived_states;
const rows = Object.entries(state.workspaces ?? {}).map(([name, workspace]) => ({
  workspace: name,
  story_id: workspace.story_id ?? null,
  phase: workspace.phase ?? null,
  derived_state: rules.find((rule) => matches(rule.when, workspace))?.state ?? fallback,
  flags: flags.filter((flag) => matches(flag.when, workspace)).map((flag) => flag.flag),
}));

if (asJson) {
  console.log(JSON.stringify(rows, null, 2));
} else if (!rows.length) {
  console.log("no active workspaces");
} else {
  for (const row of rows) {
    const flagged = row.flags.length ? `  [${row.flags.join(", ")}]` : "";
    console.log(
      `${row.workspace.padEnd(20)} ${String(row.story_id).padEnd(10)} phase ${String(row.phase).padEnd(3)} ${row.derived_state}${flagged}`,
    );
  }
}
