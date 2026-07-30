#!/usr/bin/env node
// Validate a workspace progress signal against references/status-signal.schema.json.
//
// status.json is the orchestrator's only input from a workspace, so a malformed
// signal does not fail loudly — it degrades routing silently: a FAIL with no
// failure_class cannot be routed, a story_status outside the four reportable
// states desynchronizes the plan file, a tier the nudges do not know produces
// the wrong nudge. Run this from a workspace before trusting a signal, and from
// the main session when a workspace's reported state stops making sense.
//
// Usage: node scripts/validate-signal.mjs [path/to/status.json]
// Exit 0 when valid, 1 on violations, 2 on bad input.

import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { validate } from "./json-schema.mjs";

const HERE = dirname(fileURLToPath(import.meta.url));
const SCHEMA = join(HERE, "../references/status-signal.schema.json");
const target = process.argv[2] ?? ".dev-workflow/signals/status.json";

let schema, signal;
try {
  schema = JSON.parse(readFileSync(SCHEMA, "utf8"));
} catch (error) {
  console.error(`FAIL: cannot read ${SCHEMA} — ${error.message}`);
  process.exit(2);
}
try {
  signal = JSON.parse(readFileSync(resolve(target), "utf8"));
} catch (error) {
  console.error(`FAIL: cannot read ${target} — ${error.message}`);
  process.exit(2);
}

const errors = validate(schema, signal, schema);

// Cross-field rules the schema cannot express on its own: each is a state the
// orchestrator would read as complete while a required companion field is
// missing, which is how a story lands in the plan file with no way back to
// what happened.
if (signal.story_status === "failed" && !signal.failure_log)
  errors.push("$: story_status is failed but failure_log is absent — the failure cannot be routed");
if (signal.story_status === "in_review" && !signal.pr_url)
  errors.push("$: story_status is in_review but pr_url is absent");
if (signal.story_status === "completed" && !signal.completed_at)
  errors.push("$: story_status is completed but completed_at is absent");
if (signal.blocked_on === "human" && signal.blockers?.length === 0)
  errors.push("$: blocked_on is human but blockers is empty — say what the human must decide");

if (errors.length) {
  for (const error of errors) console.error(`FAIL: ${error}`);
  process.exit(1);
}
console.log(`${target}: valid`);
