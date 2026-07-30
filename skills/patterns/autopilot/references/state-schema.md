# Autopilot State

All autopilot state lives in `.dev-workflow/` on the **main workspace** (repo root), reusing the
directory workspace agents already report into. Three files, three audiences:

| File                     | Audience              | Shape                                                                                   |
| ------------------------ | --------------------- | ----------------------------------------------------------------------------------------- |
| `autopilot-state.json`   | the tick              | [`autopilot-state.schema.json`](autopilot-state.schema.json) — every legal field, typed |
| `autopilot-history.jsonl`| audit / post-mortem   | one line per tick, `$defs.history_entry` in the same schema                              |
| `autopilot-status.md`    | the human             | [`../templates/autopilot-status.md.tmpl`](../templates/autopilot-status.md.tmpl)         |

The schema is the field reference: it carries the enums (`status`, `backend`, `last_action`,
escalation `type`, `deploy_status`), the bounds (`phase` 0–12, `completion_pct` 0–100), and what each
field means. `story_status` there is tagged `x-aep-vocab: story_status_signal` — the four states a
workspace may report — and `scripts/check-vocabulary.mjs` fails CI if that copy drifts from the
canonical vocabulary. Read the schema when you need a field; this file covers only the two protocols
that are behavior rather than shape.

---

## Tick lock

`tick_in_progress` keeps two ticks from interleaving:

1. **Before the tick** — read it. Set, and younger than `thresholds.tick_lock_stale_minutes` ([tick-protocol.json](tick-protocol.json)) → skip this tick.
2. **Start of tick** — set it to now and write immediately.
3. **End of tick** — set it to `null` and write.

A crashed tick leaves the lock set; the first tick past that window treats it as stale,
clears it, and proceeds. The staleness window is what makes a crash self-healing rather than a
permanent stop.

```bash
# Postcondition after step 3: no tick claims to be running.
node -e 'process.exit(JSON.parse(require("fs").readFileSync(".dev-workflow/autopilot-state.json")).tick_in_progress === null ? 0 : 1)'
```

---

## Atomic write

State is written whole, never edited in place, so a crash mid-write leaves the previous state intact
rather than a truncated file:

```bash
cat > .dev-workflow/autopilot-state.json.tmp << 'EOF'
{ ... }
EOF
mv .dev-workflow/autopilot-state.json.tmp .dev-workflow/autopilot-state.json   # POSIX-atomic
```

---

## Validating a state file

```bash
node skills/patterns/autopilot/scripts/validate-state.mjs .dev-workflow/autopilot-state.json
```

Run it after a hand-edit or a crash recovery. It reports every violation with its path, so an
unknown field or a value outside an enum names itself instead of surfacing three ticks later as
behavior nobody can explain.
