#!/usr/bin/env node
// Validate an autopilot state file against references/autopilot-state.schema.json.
//
// Run after a hand-edit, a crash recovery, or whenever a tick behaves in a way
// the state file does not explain. Every violation names its own path, so an
// unknown field or an out-of-enum value reports itself here instead of
// surfacing three ticks later as behavior nobody can account for.
//
// Usage:
//   node scripts/validate-state.mjs [.dev-workflow/autopilot-state.json]
//   node scripts/validate-state.mjs --history .dev-workflow/autopilot-history.jsonl
//
// Exit 0 when valid, 1 on violations, 2 on bad input.

import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { validate } from "./json-schema.mjs";

const HERE = dirname(fileURLToPath(import.meta.url));
const SCHEMA = join(HERE, "../references/autopilot-state.schema.json");

const args = process.argv.slice(2);
const historyMode = args.includes("--history");
const target =
  args.find((a) => !a.startsWith("--")) ??
  (historyMode ? ".dev-workflow/autopilot-history.jsonl" : ".dev-workflow/autopilot-state.json");

let schema;
try {
  schema = JSON.parse(readFileSync(SCHEMA, "utf8"));
} catch (error) {
  console.error(`FAIL: cannot read ${SCHEMA} — ${error.message}`);
  process.exit(2);
}

let raw;
try {
  raw = readFileSync(resolve(target), "utf8");
} catch (error) {
  console.error(`FAIL: cannot read ${target} — ${error.message}`);
  process.exit(2);
}

const errors = [];

if (historyMode) {
  const entrySchema = { $ref: "#/$defs/history_entry" };
  raw
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean)
    .forEach((line, i) => {
      let entry;
      try {
        entry = JSON.parse(line);
      } catch (error) {
        errors.push(`line ${i + 1}: not JSON — ${error.message}`);
        return;
      }
      for (const err of validate(entrySchema, entry, schema, `line ${i + 1}`)) errors.push(err);
    });
} else {
  let state;
  try {
    state = JSON.parse(raw);
  } catch (error) {
    console.error(`FAIL: ${target} is not JSON — ${error.message}`);
    process.exit(2);
  }
  errors.push(...validate(schema, state, schema));
}

if (errors.length) {
  for (const error of errors) console.error(`FAIL: ${error}`);
  process.exit(1);
}
console.log(`${target}: valid`);
