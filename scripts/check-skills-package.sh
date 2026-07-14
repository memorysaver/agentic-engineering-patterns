#!/usr/bin/env bash
# Validate the source catalog, installed package, routing probes, and metadata budget.

set -euo pipefail

REPO_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
TMP_ROOT=$(mktemp -d "${TMPDIR:-/tmp}/aep-skills-package.XXXXXX")
trap 'rm -rf "$TMP_ROOT"' EXIT
cd "$REPO_ROOT"

# Pin validation tooling so an upstream release cannot break an unchanged AEP
# commit. Override explicitly when trialing an upgrade.
SKILLS_CLI_VERSION=${SKILLS_CLI_VERSION:-1.5.17}
SKILLS_REF_VERSION=${SKILLS_REF_VERSION:-0.1.1}

EXPECTED_NAMES="$TMP_ROOT/expected-names"

# Check the repository graph without introducing another YAML parser. Metadata
# is read later from the actual installed units by the official Agent Skills
# implementation, so discovery, validation, and budgets judge one artifact.
node - "$EXPECTED_NAMES" <<'NODE'
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const expectedNamesFile = process.argv[2];
const errors = [];

function walk(directory) {
  if (!fs.existsSync(directory)) return [];
  const output = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const item = path.join(directory, entry.name);
    if (entry.isDirectory()) output.push(...walk(item));
    else if (entry.isFile()) output.push(item);
  }
  return output;
}

function duplicates(values) {
  const counts = new Map();
  for (const value of values) counts.set(value, (counts.get(value) || 0) + 1);
  return [...counts].filter(([, count]) => count > 1).map(([value]) => value);
}

function normalized(relativePath) {
  return relativePath.split(path.sep).join("/");
}

const marketplace = JSON.parse(fs.readFileSync(".claude-plugin/marketplace.json", "utf8"));
const declared = marketplace.plugins
  .flatMap((plugin) => plugin.skills)
  .map((skillPath) => skillPath.replace(/^\.\//, "").replace(/\/$/, ""));
const source = walk("skills")
  .filter((file) => path.basename(file) === "SKILL.md")
  .map((file) => normalized(path.dirname(file)))
  .sort();

const duplicatePaths = duplicates(declared);
if (duplicatePaths.length) errors.push(`duplicate marketplace paths: ${duplicatePaths.join(", ")}`);
const missingDeclarations = source.filter((skillPath) => !declared.includes(skillPath));
const unknownDeclarations = declared.filter((skillPath) => !source.includes(skillPath));
if (missingDeclarations.length) errors.push(`marketplace is missing source skills: ${missingDeclarations.join(", ")}`);
if (unknownDeclarations.length) errors.push(`marketplace declares unknown skills: ${unknownDeclarations.join(", ")}`);

const routing = JSON.parse(fs.readFileSync("evals/skill-routing.json", "utf8"));
const routingEvidence = JSON.parse(fs.readFileSync("evals/skill-routing-observations.json", "utf8"));
const cases = routing.cases;
const observations = routingEvidence.observations;
if (!Array.isArray(cases)) errors.push("evals/skill-routing.json cases must be an array");
if (!Array.isArray(observations)) errors.push("evals/skill-routing-observations.json observations must be an array");
const safeCases = Array.isArray(cases) ? cases : [];
const safeObservations = Array.isArray(observations) ? observations : [];
const duplicateCaseIds = duplicates(safeCases.map((entry) => entry.id));
if (duplicateCaseIds.length) errors.push(`duplicate routing case ids: ${duplicateCaseIds.join(", ")}`);
const duplicateObservationIds = duplicates(safeObservations.map((entry) => entry.id));
if (duplicateObservationIds.length) errors.push(`duplicate routing observation ids: ${duplicateObservationIds.join(", ")}`);
const observationById = new Map(safeObservations.map((entry) => [entry.id, entry.selected]));
if (!routingEvidence.run?.model || !routingEvidence.run?.model_version || !/^[a-f0-9]{64}$/.test(routingEvidence.run?.description_sha256 || "")) {
  errors.push("routing observations must record model, model_version, and a description_sha256");
}
for (const entry of safeCases) {
  if (!entry.id || !entry.prompt || !["direct", "boundary"].includes(entry.kind)) {
    errors.push(`invalid routing case shape: ${JSON.stringify(entry)}`);
  }
  if (Object.hasOwn(entry, "observed")) {
    errors.push(`routing expectation ${entry.id} must not embed an observed result; record it in skill-routing-observations.json`);
  }
  if (!observationById.has(entry.id)) {
    errors.push(`missing routing observation for ${entry.id}`);
  } else if (entry.expected !== observationById.get(entry.id)) {
    errors.push(`routing regression in ${entry.id}: expected ${entry.expected}, selected ${observationById.get(entry.id)}`);
  }
}
for (const observation of safeObservations) {
  if (!safeCases.some((entry) => entry.id === observation.id)) errors.push(`routing observation has no expectation: ${observation.id}`);
}
const directNames = safeCases.filter((entry) => entry.kind === "direct").map((entry) => entry.expected);
const duplicateDirectNames = duplicates(directNames);
if (duplicateDirectNames.length) errors.push(`skills with duplicate direct probes: ${duplicateDirectNames.join(", ")}`);
const expectedNames = [...new Set(directNames)].sort();
if (expectedNames.length !== source.length) {
  errors.push(`direct routing coverage has ${expectedNames.length} skills; source catalog has ${source.length}`);
}
for (const entry of safeCases) {
  if (!expectedNames.includes(entry.expected)) errors.push(`routing case ${entry.id} expects unknown skill ${entry.expected}`);
  const selected = observationById.get(entry.id);
  if (selected && !expectedNames.includes(selected)) errors.push(`routing case ${entry.id} selected unknown skill ${selected}`);
}

const parity = JSON.parse(fs.readFileSync("evals/skill-behavior-parity.json", "utf8"));
const parityCases = Array.isArray(parity.cases) ? parity.cases : [];
const parityNames = parityCases.map((entry) => entry.skill);
const duplicateParityNames = duplicates(parityNames);
if (duplicateParityNames.length) errors.push(`duplicate behavior-parity skills: ${duplicateParityNames.join(", ")}`);
const missingParity = expectedNames.filter((name) => !parityNames.includes(name));
const unknownParity = parityNames.filter((name) => !expectedNames.includes(name));
if (missingParity.length) errors.push(`behavior parity is missing skills: ${missingParity.join(", ")}`);
if (unknownParity.length) errors.push(`behavior parity contains unknown skills: ${unknownParity.join(", ")}`);
if (!parity.run?.date || !parity.run?.method || !Array.isArray(parity.run?.review_scopes)) {
  errors.push("behavior parity must record date, method, and review_scopes");
}
const sourceByName = new Map();
for (const skillPath of source) {
  const frontmatter = fs.readFileSync(path.join(skillPath, "SKILL.md"), "utf8").match(/^---\n([\s\S]*?)\n---/);
  const name = frontmatter?.[1].match(/^name:\s*([^\s]+)\s*$/m)?.[1];
  if (name) sourceByName.set(name, skillPath);
}
for (const entry of parityCases) {
  if (!entry.scenario || entry.result !== "pass" || !Array.isArray(entry.loaded_references) || !entry.observed_postcondition) {
    errors.push(`invalid behavior-parity evidence for ${entry.skill}: scenario, pass result, loaded_references, and observed_postcondition are required`);
  }
  for (const reference of entry.loaded_references || []) {
    if (/^\/aep-[a-z0-9-]+$/.test(reference)) continue;
    if (!reference.startsWith("references/") || !fs.existsSync(path.join(sourceByName.get(entry.skill) || "", reference))) {
      errors.push(`behavior-parity evidence for ${entry.skill} names an invalid disclosed reference: ${reference}`);
    }
  }
}

function skillRoot(file) {
  let current = path.dirname(path.resolve(file));
  const skillsRoot = path.resolve("skills");
  while (current.startsWith(skillsRoot)) {
    if (fs.existsSync(path.join(current, "SKILL.md"))) return current;
    const parent = path.dirname(current);
    if (parent === current) break;
    current = parent;
  }
  return null;
}

let referenceHops = 0;
for (const sourceName of walk("skills").filter((file) => file.endsWith(".md")).sort()) {
  const sourceFile = path.resolve(sourceName);
  const sourceSkill = skillRoot(sourceFile);
  const text = fs.readFileSync(sourceFile, "utf8");
  for (const match of text.matchAll(/\[[^\]]*\]\(([^)]+)\)/g)) {
    const rawTarget = match[1];
    const target = rawTarget.replace(/#.*/, "");
    if (!target || /^(?:https?:\/\/|mailto:|\/aep-)/.test(target)) continue;
    const resolved = path.resolve(path.dirname(sourceFile), target);
    if (!fs.existsSync(resolved)) {
      errors.push(`broken local link in ${sourceName}: ${rawTarget}`);
      continue;
    }
    const targetSkill = skillRoot(resolved);
    if (sourceSkill && targetSkill && sourceSkill !== targetSkill) {
      errors.push(`cross-skill relative link in ${sourceName}: ${rawTarget}`);
    }
    if (normalized(sourceName).includes("/references/") && normalized(resolved).includes("/references/")) {
      referenceHops += 1;
    }
  }
}
if (referenceHops > 26) errors.push(`reference-to-reference link count increased above 26: ${referenceHops}`);

const longReferenceHashesWithoutToc = new Set();
for (const reference of walk("skills").filter((file) => normalized(file).includes("/references/") && file.endsWith(".md"))) {
  const text = fs.readFileSync(reference, "utf8");
  const lines = text.replace(/\r\n/g, "\n").replace(/\n$/, "").split("\n");
  if (lines.length <= 100) continue;
  const earlyText = lines.slice(0, 50).join("\n");
  if (/^(?:##+\s+(?:contents|table of contents)|- \[[^\]]+\]\(#[^)]+\))/im.test(earlyText)) continue;
  longReferenceHashesWithoutToc.add(crypto.createHash("sha256").update(text).digest("hex"));
}
const tocDebt = longReferenceHashesWithoutToc.size;
if (tocDebt > 28) errors.push(`unique long references without an early TOC increased above 28: ${tocDebt}`);

if (errors.length) {
  for (const error of errors) console.error(`FAIL: ${error}`);
  process.exit(1);
}

fs.writeFileSync(expectedNamesFile, `${expectedNames.join("\n")}\n`);
console.log(`source catalog: ${source.length} skills; routing probes: ${safeCases.length}`);
console.log(`behavior parity: ${parityCases.length}/${expectedNames.length} recorded scenario dry-runs`);
console.log(`navigation debt ratchets: ${referenceHops}/26 reference hops; ${tocDebt}/28 unique long references without early TOC`);
NODE

# Exercise the consumer path. This catches a parser silently omitting one
# malformed skill even when the CLI itself exits zero.
INSTALL_ROOT="$TMP_ROOT/install"
mkdir -p "$INSTALL_ROOT"
git -C "$INSTALL_ROOT" init -q
if ! (cd "$INSTALL_ROOT" && npx -y "skills@$SKILLS_CLI_VERSION" add "$REPO_ROOT" -a codex --skill '*' --copy -y) >"$TMP_ROOT/install.log" 2>&1; then
  cat "$TMP_ROOT/install.log" >&2
  exit 1
fi

find "$INSTALL_ROOT/.agents/skills" -mindepth 1 -maxdepth 1 -type d -exec basename {} \; | sort > "$TMP_ROOT/installed-names"
if ! diff -u "$EXPECTED_NAMES" "$TMP_ROOT/installed-names"; then
  echo "FAIL: skills CLI did not install the complete expected catalog" >&2
  exit 1
fi

# Validate links in the artifact users actually receive. Source-tree links can
# appear valid by escaping to repository docs or sibling skills that are not
# present when one skill is copied into a downstream project.
node - "$INSTALL_ROOT/.agents/skills" <<'NODE'
const fs = require("node:fs");
const path = require("node:path");

const installRoot = path.resolve(process.argv[2]);
const errors = [];

function walk(directory) {
  const output = [];
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const item = path.join(directory, entry.name);
    if (entry.isDirectory()) output.push(...walk(item));
    else if (entry.isFile()) output.push(item);
  }
  return output;
}

for (const entry of fs.readdirSync(installRoot, { withFileTypes: true })) {
  if (!entry.isDirectory()) continue;
  const skillRoot = path.join(installRoot, entry.name);
  for (const markdown of walk(skillRoot).filter((file) => file.endsWith(".md"))) {
    const text = fs.readFileSync(markdown, "utf8");
    for (const match of text.matchAll(/\[[^\]]*\]\(([^)]+)\)/g)) {
      const rawTarget = match[1];
      const target = rawTarget.replace(/#.*/, "");
      if (!target || /^(?:https?:\/\/|mailto:|\/aep-)/.test(target)) continue;
      const resolved = path.resolve(path.dirname(markdown), target);
      const contained = resolved === skillRoot || resolved.startsWith(`${skillRoot}${path.sep}`);
      if (!contained) errors.push(`installed link escapes ${entry.name}: ${path.relative(skillRoot, markdown)} -> ${rawTarget}`);
      else if (!fs.existsSync(resolved)) errors.push(`broken installed link in ${entry.name}: ${path.relative(skillRoot, markdown)} -> ${rawTarget}`);
    }
  }
}

if (errors.length) {
  for (const error of errors) console.error(`FAIL: ${error}`);
  process.exit(1);
}
console.log("installed links: self-contained and resolvable");
NODE

if command -v uvx >/dev/null 2>&1; then
  agent_skills=(uvx --from "skills-ref==$SKILLS_REF_VERSION" agentskills)
elif command -v agentskills >/dev/null 2>&1; then
  agent_skills=(agentskills)
elif command -v python3 >/dev/null 2>&1; then
  python3 -m venv "$TMP_ROOT/validator-venv"
  "$TMP_ROOT/validator-venv/bin/pip" install -q "skills-ref==$SKILLS_REF_VERSION"
  agent_skills=("$TMP_ROOT/validator-venv/bin/agentskills")
else
  echo "FAIL: Python or uv/uvx is required for the official Agent Skills validator" >&2
  exit 1
fi

PROPERTIES_ROOT="$TMP_ROOT/properties"
mkdir -p "$PROPERTIES_ROOT"
validation_failures=0
while IFS= read -r name; do
  installed="$INSTALL_ROOT/.agents/skills/$name"
  if ! "${agent_skills[@]}" validate "$installed" >"$TMP_ROOT/validate-$name.log" 2>&1; then
    echo "FAIL: official validation failed for $name" >&2
    cat "$TMP_ROOT/validate-$name.log" >&2
    validation_failures=$((validation_failures + 1))
    continue
  fi
  if ! "${agent_skills[@]}" read-properties "$installed" >"$PROPERTIES_ROOT/$name.json" 2>"$TMP_ROOT/properties-$name.log"; then
    echo "FAIL: could not read official properties for $name" >&2
    cat "$TMP_ROOT/properties-$name.log" >&2
    validation_failures=$((validation_failures + 1))
  fi
done < "$EXPECTED_NAMES"
[ "$validation_failures" -eq 0 ] || exit 1

# Enforce the fixed front-tier metadata cost using the properties the official
# implementation read from the installed package.
node - "$PROPERTIES_ROOT" "$EXPECTED_NAMES" "evals/skill-routing-observations.json" <<'NODE'
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const propertiesRoot = process.argv[2];
const expectedNames = fs.readFileSync(process.argv[3], "utf8").trim().split("\n");
const errors = [];
let descriptionChars = 0;
let descriptionWords = 0;
const descriptionRecords = [];

for (const expectedName of expectedNames) {
  const properties = JSON.parse(fs.readFileSync(path.join(propertiesRoot, `${expectedName}.json`), "utf8"));
  const { name, description } = properties;
  if (name !== expectedName) errors.push(`installed name mismatch: expected ${expectedName}, got ${name}`);
  if (typeof name !== "string" || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(name) || name.length > 64) {
    errors.push(`invalid installed skill name: ${JSON.stringify(name)}`);
  }
  if (typeof description !== "string" || !description.length) {
    errors.push(`missing description for ${expectedName}`);
    continue;
  }
  if (description.length > 300) errors.push(`description exceeds 300 characters for ${expectedName}: ${description.length}`);
  if (/[<>]/.test(description)) errors.push(`description contains angle brackets for ${expectedName}`);
  descriptionChars += description.length;
  descriptionWords += description.trim().split(/\s+/).length;
  descriptionRecords.push(`${name}\0${description}`);
}
if (descriptionChars > 5000) errors.push(`description corpus exceeds 5000 characters: ${descriptionChars}`);

const routingEvidence = JSON.parse(fs.readFileSync(process.argv[4], "utf8"));
const descriptionSha256 = crypto.createHash("sha256").update(descriptionRecords.join("\n")).digest("hex");
if (routingEvidence.run?.description_sha256 !== descriptionSha256) {
  errors.push(`routing observations describe a different metadata corpus: recorded ${routingEvidence.run?.description_sha256}, current ${descriptionSha256}`);
}

if (errors.length) {
  for (const error of errors) console.error(`FAIL: ${error}`);
  process.exit(1);
}
console.log(`installed metadata: ${descriptionChars} chars / ${descriptionWords} words`);
console.log(`routing evidence: ${routingEvidence.observations.length} recorded selections bound to ${descriptionSha256.slice(0, 12)}`);
NODE

skill_count=$(wc -l < "$EXPECTED_NAMES" | tr -d ' ')
echo "installed package: $skill_count/$skill_count skills discovered and officially valid"
