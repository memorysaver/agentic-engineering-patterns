// Minimal JSON-Schema validator — the canonical implementation for every AEP
// skill that ships a *.schema.json.
//
// Covers exactly the keywords AEP schemas use ($ref to internal $defs, oneOf,
// const, enum, type unions, numeric bounds, items, required, properties,
// additionalProperties as false or as a schema). Kept in-repo, dependency-free,
// and materialized into each consuming skill by scripts/build-skills.sh: a
// skill is installed as one directory, so it cannot reach a validator that
// lives in another skill, and a second copy of a validator is a second set of
// bugs. Unknown keywords (x-aep-vocab, x-aep-stages, title, description) are
// annotations and are ignored by design.
//
// Returns an array of human-readable error strings, empty when valid.

import { readFileSync } from "node:fs";

export function validate(schema, data, root = schema, path = "$") {
  const errs = [];
  const fail = (msg) => errs.push(`${path}: ${msg}`);
  if (schema.$ref) {
    const target = schema.$ref
      .replace(/^#\//, "")
      .split("/")
      .reduce((o, k) => o?.[k.replace(/~1/g, "/").replace(/~0/g, "~")], root);
    if (!target) {
      fail(`unresolvable $ref ${schema.$ref}`);
      return errs;
    }
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
  if (typeof data === "string" && schema.pattern && !new RegExp(schema.pattern).test(data))
    fail(`does not match /${schema.pattern}/`);
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

// The named vocabulary sets, read from the materialized copy that ships beside
// the calling skill's references/. Throws with a repair instruction rather than
// falling back to a hardcoded set: a validator that silently uses a stale
// vocabulary is worse than one that stops.
export function loadVocabulary(referencesDir, name) {
  const path = `${referencesDir}/aep-vocabulary.schema.json`;
  let doc;
  try {
    doc = JSON.parse(readFileSync(path, "utf8"));
  } catch (error) {
    throw new Error(
      `cannot read the AEP vocabulary at ${path} (${error.message}). ` +
        `Run: bash scripts/build-skills.sh`,
    );
  }
  const def = doc.$defs?.[name];
  if (!def?.enum) throw new Error(`no enumerated $defs.${name} in ${path}`);
  return new Set(def.enum);
}
