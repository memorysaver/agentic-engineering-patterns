#!/usr/bin/env node
// C1/C6: steering ratchets, held per skill rather than corpus-wide.
//
// A single corpus number is bump-bait. Most of what this counts is prose that
// describes behavior ("stories that don't map to an activity"), so a corpus cap
// with no headroom fires on innocuous edits, and the cheapest fix — raising the
// number — makes the ratchet decorative. Per skill, the failure names the file
// that grew a prohibition and the trade is local and explicit: remove one from
// this skill, or record deliberately that this skill now carries more.
//
// Usage:
//   node scripts/check-steering.mjs            # compare against the baseline
//   node scripts/check-steering.mjs --update   # re-record it (review the diff)
//
// Exit 0 when no skill exceeds its baseline, 1 otherwise, 2 on bad input.

import { readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const REPO = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const SKILLS = join(REPO, "skills");
const BASELINE = join(REPO, "evals/steering-baseline.json");
const update = process.argv.includes("--update");

// The two ratcheted measures. Kept as literal regexes rather than a heuristic
// for "is this steering?": a heuristic that guesses wrong in either direction
// is worse than a blunt count everyone can reproduce with grep.
const NEGATION = /never |do not |don't |must not |avoid /i;
const IMPERATIVE = /\bNEVER\b|\bMUST\b|\bALWAYS\b/;

function skillFiles(dir, out = []) {
  for (const entry of readdirSync(dir)) {
    const path = join(dir, entry);
    if (statSync(path).isDirectory()) skillFiles(path, out);
    else if (entry === "SKILL.md") out.push(path);
  }
  return out;
}

const counts = {};
for (const file of skillFiles(SKILLS).sort()) {
  const lines = readFileSync(file, "utf8").split("\n");
  counts[relative(REPO, file)] = {
    negations: lines.filter((l) => NEGATION.test(l)).length,
    imperatives: lines.filter((l) => IMPERATIVE.test(l)).length,
  };
}

const total = (key) => Object.values(counts).reduce((n, c) => n + c[key], 0);

if (update) {
  writeFileSync(
    BASELINE,
    JSON.stringify(
      {
        schema_version: 1,
        note: "Per-skill steering ceilings (C1/C6 of docs/decisions/claude-5-context-engineering.md). A skill may fall below its entry; rising above it fails CI. Raising an entry is a reviewable decision, not a fix — the question it asks is which prohibition in THIS skill earned its place.",
        skills: counts,
      },
      null,
      2,
    ) + "\n",
  );
  console.log(
    `steering baseline recorded: ${Object.keys(counts).length} skills, ${total("negations")} negation lines, ${total("imperatives")} hard imperatives`,
  );
  process.exit(0);
}

let baseline;
try {
  baseline = JSON.parse(readFileSync(BASELINE, "utf8")).skills;
} catch (error) {
  console.error(`FAIL: cannot read ${relative(REPO, BASELINE)} — ${error.message}`);
  console.error("      Record it with: node scripts/check-steering.mjs --update");
  process.exit(2);
}

const errors = [];
for (const [file, count] of Object.entries(counts)) {
  const before = baseline[file];
  if (!before) {
    errors.push(
      `${file} has no steering baseline — record one with 'node scripts/check-steering.mjs --update' and say in review what its ${count.negations} negation line(s) are for`,
    );
    continue;
  }
  if (count.negations > before.negations)
    errors.push(
      `${file}: negation lines ${before.negations} → ${count.negations}. Which prohibition here earned its place, and which one leaves?`,
    );
  if (count.imperatives > before.imperatives)
    errors.push(
      `${file}: hard imperatives ${before.imperatives} → ${count.imperatives}. A NEVER/MUST/ALWAYS needs a machine check behind it (C1).`,
    );
}

for (const file of Object.keys(baseline)) {
  if (!counts[file]) console.log(`note: ${file} is in the baseline but no longer exists`);
}

if (errors.length) {
  for (const error of errors) console.error(`FAIL: ${error}`);
  process.exit(1);
}
console.log(
  `steering: ${total("negations")} negation lines, ${total("imperatives")} hard imperatives across ${Object.keys(counts).length} skills — none above its baseline`,
);
