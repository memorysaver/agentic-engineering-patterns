#!/usr/bin/env node
// pulse.mjs — the read-time emission (decision doc D10, emission 2).
//
// Answers "what happened since I last looked" at the moment it is asked, and
// writes nothing down. Three properties make it survive a fast-moving project
// where a written brief cannot:
//
//   · It deals in EVENTS, not states. "L41-001 completed at 07-25" stays true
//     forever; "L41-001 is in progress" had a one-hour half-life. A report of
//     states rots; a log of events can at worst be incomplete.
//   · It carries no prose. Nothing is asserted, so nothing can be asserted
//     wrongly — the failure mode that produced every content defect in two
//     evaluation rounds.
//   · Obligations are AGED. "What needs a human" without a duration is a list,
//     not a signal: an item true for six weeks reads identically to one raised
//     this morning unless the surface says so.
//
// Usage:
//   node pulse.mjs --repo <consumer repo> [--since <commit>] [--json]
//
// The cursor defaults to docs/human-alignment/manifest.json's commit — the
// point the last orientation document described.

import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const OPEN = new Set(["pending", "ready", "in_progress", "failed", "blocked"]);
const PAST_SCRIPTED = new Set(["scripted_passed", "passed", "completed"]);

function git(args, cwd) {
  try {
    return execFileSync("git", args, {
      cwd,
      encoding: "utf8",
      maxBuffer: 256 * 1024 * 1024,
    }).trim();
  } catch {
    return null;
  }
}

const days = (iso, now) => {
  if (!iso) return null;
  const then = Date.parse(String(iso).slice(0, 10));
  return Number.isNaN(then) ? null : Math.floor((now - then) / 86400000);
};

// ─── Obligations: what is owed to a human, and for how long ────────────────
// The framework has no first-class obligation record (D12 recommends one), so
// this reads the evidence block that consumers actually use and recovers a date
// from whatever carries one. Where no date can be recovered the age is unknown
// and is shown as such — never as zero.
export function obligations(ctx, now) {
  const out = [];
  for (const g of ctx.layer_gates ?? []) {
    const ev = g.evidence ?? {};
    if (ev.manual_pending !== true) continue;
    if (!PAST_SCRIPTED.has(String(g.status))) continue; // not yet owed
    const note = String(g.notes ?? "");
    const dated =
      /(\d{4}-\d{2}-\d{2})/.exec(note)?.[1] ?? (g.completed_at ? String(g.completed_at) : null);
    // Recover the sentence that names the obligation, not the fragment after the
    // keyword — "on registering a real Notion integration" reads as a fragment;
    // the whole clause reads as a task.
    const sentence = note
      .split(/(?<=\.)\s+/)
      .find((s) => /\b(BLOCKED|PENDING|blocked on|requires? a human|awaiting)\b/i.test(s));
    const blocked = sentence
      ? sentence
          .replace(/^[^A-Z]*/, "")
          .replace(/\s+/g, " ")
          .replace(/^(Manual prod dogfood|Manual dogfood|Manual)\s+/i, "")
          .trim()
      : null;
    out.push({
      layer: g.layer,
      gate_status: String(g.status),
      since: dated ? String(dated).slice(0, 10) : null,
      age_days: days(dated, now),
      what: blocked ? blocked.replace(/\s+/g, " ") : "manual acceptance not run",
    });
  }
  return out.sort((a, b) => (b.age_days ?? -1) - (a.age_days ?? -1));
}

// ─── Events: transitions between the cursor and now ────────────────────────
export function events(before, after) {
  const ev = [];
  const bs = new Map((before?.stories ?? []).map((s) => [String(s.id), s]));
  const as = new Map((after.stories ?? []).map((s) => [String(s.id), s]));
  for (const [id, s] of as) {
    const b = bs.get(id);
    if (!b) {
      ev.push({ kind: "story_added", id, to: String(s.status), layer: s.layer });
    } else if (String(b.status) !== String(s.status)) {
      ev.push({
        kind: "story_moved",
        id,
        from: String(b.status),
        to: String(s.status),
        layer: s.layer,
      });
    }
  }
  const bg = new Map((before?.layer_gates ?? []).map((g) => [String(g.layer), g]));
  for (const g of after.layer_gates ?? []) {
    const b = bg.get(String(g.layer));
    if (b && String(b.status) !== String(g.status)) {
      ev.push({ kind: "gate_moved", layer: g.layer, from: String(b.status), to: String(g.status) });
    }
    if (g.closure_status && (!b || !b.closure_status)) {
      ev.push({
        kind: "layer_closed",
        layer: g.layer,
        to: String(g.closure_status),
        successor: g.decision_realignment?.successor_layer ?? null,
      });
    }
  }
  const bc = (before?.changelog ?? []).length;
  for (const e of (after.changelog ?? []).slice(bc)) {
    if (!["decision", "design", "milestone"].includes(String(e.type))) continue;
    ev.push({
      kind: "decision",
      date: String(e.date ?? ""),
      by: String(e.author ?? ""),
      summary: String(e.summary ?? ""),
    });
  }
  return ev;
}

// ─── Stalled: open work that has not moved ─────────────────────────────────
export function stalled(ctx, now, minDays = 2) {
  return (ctx.stories ?? [])
    .filter((s) => OPEN.has(String(s.status)) && String(s.status) !== "pending")
    .map((s) => ({
      id: String(s.id),
      status: String(s.status),
      layer: s.layer,
      age_days: days(s.started_at ?? s.completed_at, now),
    }))
    .filter((s) => (s.age_days ?? 0) >= minDays)
    .sort((a, b) => (b.age_days ?? 0) - (a.age_days ?? 0));
}

function arg(name, fallback = null) {
  const i = process.argv.indexOf(`--${name}`);
  return i > -1 && process.argv[i + 1] ? process.argv[i + 1] : fallback;
}

if (process.argv[1] && process.argv[1].endsWith("pulse.mjs")) {
  const { loadYaml, deriveAttentionSet } = await import("./derive.mjs");
  const repo = resolve(arg("repo", "."));
  const ctxPath = join(repo, "product-context.yaml");
  if (!existsSync(ctxPath)) {
    console.error(`ERROR: ${ctxPath} not found.`);
    process.exit(2);
  }

  const manifestPath = join(repo, "docs/human-alignment/manifest.json");
  const cursor =
    arg("since") ??
    (existsSync(manifestPath) ? JSON.parse(readFileSync(manifestPath, "utf8")).commit : null);

  const now = Date.now();
  const after = await loadYaml(ctxPath);
  let before = null;
  if (cursor) {
    const rel = git(["ls-files", "--full-name", ctxPath], repo) ?? "product-context.yaml";
    const prev = git(["show", `${cursor}:${rel}`], repo);
    if (prev) {
      const tmp = join(repo, ".aep-pulse-cursor.yaml");
      execFileSync("bash", ["-c", `cat > ${JSON.stringify(tmp)}`], { input: prev });
      before = await loadYaml(tmp);
      execFileSync("rm", ["-f", tmp]);
    }
  }

  const head = git(["rev-parse", "--short", "HEAD"], repo) ?? "unknown";
  const span = cursor ? git(["rev-list", "--count", `${cursor}..HEAD`], repo) : null;
  const obs = obligations(after, now);
  const evs = before ? events(before, after) : [];
  const st = stalled(after, now);
  const { attention_set } = deriveAttentionSet(after, null);
  const needs = attention_set.filter((a) => a.kind !== "calibration_due");

  if (process.argv.includes("--json")) {
    console.log(
      JSON.stringify(
        { head, cursor, commits: span, obligations: obs, events: evs, stalled: st, needs },
        null,
        2,
      ),
    );
    process.exit(0);
  }

  const B = (s) => `[1m${s}[0m`;
  const dim = (s) => `[2m${s}[0m`;
  const red = (s) => `[31m${s}[0m`;

  console.log(`\n${B("PULSE")}  ${after.project ?? "project"}  ${dim(`@ ${head}`)}`);
  console.log(
    dim(
      `  cursor ${cursor ?? "none"}${span ? ` · ${span} commits since` : " · no previous brief"}`,
    ),
  );

  console.log(`\n${B("OWED BY YOU")}${dim(obs.length ? "" : "  — nothing")}`);
  for (const o of obs) {
    const age = o.age_days === null ? "  ?d" : `${String(o.age_days).padStart(3)}d`;
    const flag = (o.age_days ?? 0) >= 14 ? red(age) : age;
    console.log(
      `  ${flag}  L${String(o.layer).padEnd(5)} ${o.what.length > 96 ? o.what.slice(0, 95) + "…" : o.what}`,
    );
  }

  console.log(
    `\n${B("SINCE YOU LAST LOOKED")}${dim(evs.length ? "" : cursor ? "  — nothing moved" : "  — no cursor")}`,
  );
  for (const e of evs.slice(0, 25)) {
    if (e.kind === "story_moved") console.log(`  ${e.id.padEnd(30)} ${e.from} → ${B(e.to)}`);
    else if (e.kind === "story_added")
      console.log(`  ${e.id.padEnd(30)} ${dim("new")} → ${B(e.to)}`);
    else if (e.kind === "gate_moved")
      console.log(`  ${`gate L${e.layer}`.padEnd(30)} ${e.from} → ${B(e.to)}`);
    else if (e.kind === "layer_closed")
      console.log(
        `  ${`layer L${e.layer}`.padEnd(30)} ${B(e.to)}${e.successor ? ` → successor L${e.successor}` : ""}`,
      );
    else if (e.kind === "decision")
      console.log(`  ${dim(e.date)} ${B(e.by)}  ${e.summary.slice(0, 84)}`);
  }
  if (evs.length > 25) console.log(dim(`  … ${evs.length - 25} more`));

  console.log(`\n${B("NEEDS YOU NOW")}${dim(needs.length ? "" : "  — nothing")}`);
  for (const n of needs)
    console.log(`  ${n.verb.padEnd(10)} ${n.id.padEnd(30)} ${dim(n.title.slice(0, 60))}`);

  console.log(`\n${B("OPEN AND NOT MOVING")}${dim(st.length ? "" : "  — nothing")}`);
  for (const s of st.slice(0, 10)) {
    console.log(`  ${String(s.age_days).padStart(3)}d  ${s.id.padEnd(30)} ${s.status}`);
  }
  console.log(dim("\n  nothing here was written to disk; re-run to recompute\n"));
}
