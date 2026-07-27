#!/usr/bin/env node
// receipt-consumer.mjs — Phase 1.5c of /aep-human-alignment.
//
// Runs archify `validate`, applies its structured repair receipts
// mechanically, and delivers. No agent judgment and no guessing: a receipt
// this file does not know how to repair is REPORTED, never patched by
// invention. The correction loop is bounded at two rounds — a third failure is
// surfaced, not silently retried.
//
// Usage:
//   node receipt-consumer.mjs --type architecture --in <ir.json> --out <out.html>
//                             [--archify <path-to-archify-cli>] [--rounds 2]

import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { basename, resolve } from "node:path";

const MAX_ROUNDS = 2;

function archifyCli() {
  const explicit = arg("archify");
  if (explicit) return ["node", [resolve(explicit)]];
  if (process.env.ARCHIFY_CLI) return ["node", [resolve(process.env.ARCHIFY_CLI)]];
  return ["archify", []];
}

function runArchify(args) {
  const [bin, prefix] = archifyCli();
  try {
    const out = execFileSync(bin, [...prefix, ...args], {
      encoding: "utf8",
      maxBuffer: 64 * 1024 * 1024,
    });
    return { ok: true, stdout: out };
  } catch (err) {
    if (err.code === "ENOENT") return { ok: false, missing: true, stdout: "" };
    return { ok: false, stdout: err.stdout ?? "", stderr: err.stderr ?? "" };
  }
}

export function validate(type, file) {
  const res = runArchify(["validate", type, file, "--json"]);
  if (res.missing) return { missing: true, diagnostics: [] };
  const text = res.stdout || "";
  const start = text.indexOf("{");
  if (start === -1) return { ok: res.ok, diagnostics: [], raw: text };
  try {
    const parsed = JSON.parse(text.slice(start));
    return { ok: parsed.ok, diagnostics: parsed.diagnostics ?? [], stage: parsed.stage };
  } catch {
    return { ok: res.ok, diagnostics: [], raw: text };
  }
}

// ─── Mechanical repairs ────────────────────────────────────────────────────
// Each entry: a receipt code, and a pure function (ir, diagnostic, state) that
// applies exactly the fix the receipt names. Return true when it changed the IR.
const REPAIRS = {
  // An edge crossing an unrelated component. archify's supported fixes are
  // "adjust fromSide/toSide, set route/via, or move the component" — we take
  // the route/via branch and send the edge through a gutter below the diagram,
  // one lane per repaired edge so repairs never stack on each other.
  "clean-flow/edge-through-node": (ir, diag, state) => {
    const idx = diag.subject?.index;
    const conn = ir.connections?.[idx];
    if (!conn) return false;
    if (state.routed.has(idx)) return false;
    const byId = new Map((ir.components ?? []).map((c) => [c.id, c]));
    const from = byId.get(conn.from);
    const to = byId.get(conn.to);
    if (!from?.pos || !to?.pos) return false;

    // Every vertical run must sit in a column gutter and every horizontal run
    // below the whole diagram, because a target in a vertical stack has no
    // clear approach from above or below — only from its side.
    const w = (c) => c.size?.[0] ?? 170;
    const h = (c) => c.size?.[1] ?? 62;
    const cy = (c) => c.pos[1] + h(c) / 2;
    const floor = Math.max(...(ir.components ?? []).map((c) => (c.pos?.[1] ?? 0) + h(c)));
    const lane = floor + 48 + state.routed.size * 26;
    const exitX = from.pos[0] + w(from) + 24;
    const entryX = to.pos[0] - 40;

    conn.fromSide = "right";
    conn.toSide = "left";
    conn.route = "orthogonal-h";
    conn.via = [
      [exitX, cy(from)],
      [exitX, lane],
      [entryX, lane],
      [entryX, cy(to)],
    ];
    state.routed.add(idx);
    return true;
  },

  // A label wider than the box it sits in. The receipt's first supported fix is
  // "shorten the label, move detail to sublabel, or widen size" — widening is
  // the only one that preserves the label's meaning, so we widen.
  "layout/constraint": (ir, diag, state) => {
    const msg = diag.message ?? "";

    const wide = /Label "(.+?)" \(~(\d+)px\) is wider than component "(.+?)" \((\d+)px\)/.exec(msg);
    if (wide) {
      const [, , needPx, compId] = wide;
      const comp = (ir.components ?? []).find((c) => c.id === compId || c.label === compId);
      if (!comp?.size) return false;
      const want = Number(needPx) + 28;
      if (comp.size[0] >= want) return false;
      comp.size = [want, comp.size[1]];
      return true;
    }

    // Two edge labels colliding. The receipt's own suggested fix is
    // "add labelDy +24 on one edge" — apply it to the second label named,
    // whichever collection this diagram type keeps its edges in.
    const overlap = /Labels "(.+?)" and "(.+?)" overlap/.exec(msg);
    if (overlap) {
      const target = overlap[2];
      for (const key of ["transitions", "connections", "edges"]) {
        const edge = (ir[key] ?? []).find((e) => e.label === target);
        if (!edge) continue;
        const bump = state.nudged.get(target) ?? 0;
        if (bump >= 3) return false;
        edge.labelDy = (edge.labelDy ?? 0) + 24 * (bump + 1);
        state.nudged.set(target, bump + 1);
        return true;
      }
    }
    return false;
  },

  // A routed path entering the legend. archify's supported fixes are "move the
  // route or enlarge the viewBox"; enlarging is the mechanical one — moving the
  // route would need to know where the legend is, which the receipt does not say.
  "artifact/legend-clearance": (ir, _diag, state) => {
    if (state.viewBoxGrown >= 3) return false;
    const floor = Math.max(
      ...(ir.components ?? []).map((c) => (c.pos?.[1] ?? 0) + (c.size?.[1] ?? 62)),
    );
    const right = Math.max(
      ...(ir.components ?? []).map((c) => (c.pos?.[0] ?? 0) + (c.size?.[0] ?? 170)),
    );
    const lanes = (ir.connections ?? []).flatMap((c) => (c.via ?? []).map((p) => p[1]));
    const bottom = Math.max(floor, ...lanes);
    const current = ir.meta.viewBox ?? [right + 240, bottom + 240];
    ir.meta.viewBox = [
      Math.max(current[0], right + 320),
      Math.max(current[1], bottom + 200 + state.viewBoxGrown * 120),
    ];
    state.viewBoxGrown++;
    return true;
  },

  // Connection label colliding with geometry: nudge it along its own segment.
  "clean-flow/label-overlap": (ir, diag, state) => {
    const idx = diag.subject?.index;
    const conn = ir.connections?.[idx];
    if (!conn) return false;
    const step = 14;
    conn.labelDy = (conn.labelDy ?? 0) - step * ((state.nudged.get(idx) ?? 0) ? 2 : 1);
    state.nudged.set(idx, (state.nudged.get(idx) ?? 0) + 1);
    return true;
  },
};

export function repair(ir, diagnostics, state) {
  let changed = 0;
  const unrepairable = [];
  // One connection can raise one diagnostic per obstacle it crosses. A single
  // reroute clears all of them, so a repeat subject in the same round is
  // already covered — reporting it as unrepairable would overstate the failure.
  const coveredThisRound = new Set();
  for (const diag of diagnostics) {
    if (diag.severity !== "error") continue;
    const subjectKey = `${diag.code}|${diag.subject?.collection ?? ""}|${diag.subject?.index ?? ""}`;
    if (coveredThisRound.has(subjectKey)) continue;
    const fix = REPAIRS[diag.code] ?? REPAIRS[diag.code?.split("/")[0]];
    if (!fix) {
      unrepairable.push(diag);
      continue;
    }
    if (fix(ir, diag, state)) {
      changed++;
      coveredThisRound.add(subjectKey);
    } else unrepairable.push(diag);
  }
  return { changed, unrepairable };
}

// ─── Pipeline ──────────────────────────────────────────────────────────────
export function validateRepairDeliver({ type, inPath, outPath, rounds = MAX_ROUNDS }) {
  const receipts = [];
  const state = { routed: new Set(), nudged: new Map(), viewBoxGrown: 0 };
  let ir = JSON.parse(readFileSync(inPath, "utf8"));

  for (let round = 0; round <= rounds; round++) {
    const result = validate(type, inPath);
    if (result.missing) return { missing: true, receipts };
    if (result.ok) {
      const delivered = runArchify(["deliver", type, inPath, outPath]);
      return {
        ok: delivered.ok,
        rounds: round,
        receipts,
        output: outPath,
        error: delivered.ok ? null : delivered.stderr || delivered.stdout || "deliver failed",
      };
    }
    if (round === rounds) {
      return {
        ok: false,
        rounds: round,
        receipts,
        unrepairable: result.diagnostics.filter((d) => d.severity === "error"),
        error: `still failing after ${rounds} correction round${rounds === 1 ? "" : "s"}`,
      };
    }
    const { changed, unrepairable } = repair(ir, result.diagnostics, state);
    receipts.push({
      round: round + 1,
      diagnostics: result.diagnostics.length,
      repaired: changed,
      unrepairable: unrepairable.map((d) => ({
        code: d.code,
        subject: d.subject,
        evidence: d.evidence,
      })),
    });
    if (changed === 0) {
      return {
        ok: false,
        rounds: round + 1,
        receipts,
        unrepairable,
        error: "no receipt was mechanically repairable",
      };
    }
    writeFileSync(inPath, JSON.stringify(ir, null, 2) + "\n");
    ir = JSON.parse(readFileSync(inPath, "utf8"));
  }
  return { ok: false, receipts, error: "unreachable" };
}

function arg(name, fallback = null) {
  const i = process.argv.indexOf(`--${name}`);
  return i > -1 && process.argv[i + 1] ? process.argv[i + 1] : fallback;
}

if (process.argv[1] && process.argv[1].endsWith("receipt-consumer.mjs")) {
  const type = arg("type", "architecture");
  const inPath = resolve(arg("in", ""));
  const outPath = resolve(arg("out", inPath.replace(/\.json$/, ".html")));
  if (!inPath || !existsSync(inPath)) {
    console.error("ERROR: --in <ir.json> is required.");
    process.exit(2);
  }
  const result = validateRepairDeliver({
    type,
    inPath,
    outPath,
    rounds: Number(arg("rounds", MAX_ROUNDS)),
  });
  if (result.missing) {
    console.error(
      `receipt-consumer: archify CLI not found — ${basename(inPath)} degrades to the named fallback rung.`,
    );
    process.exit(3);
  }
  for (const r of result.receipts) {
    console.log(
      `  round ${r.round}: ${r.diagnostics} diagnostic(s), ${r.repaired} repaired, ${r.unrepairable.length} not`,
    );
    for (const u of r.unrepairable.slice(0, 5))
      console.log(`    unrepairable ${u.code} ${JSON.stringify(u.subject ?? {})}`);
  }
  if (!result.ok) {
    console.error(`receipt-consumer: FAILED ${basename(inPath)} — ${result.error}`);
    process.exit(1);
  }
  console.log(
    `receipt-consumer: ${result.output} (green after ${result.rounds} correction round${result.rounds === 1 ? "" : "s"})`,
  );
}
