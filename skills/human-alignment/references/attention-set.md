# The Attention Set

The canonical answer to **"what in this project needs a human right now?"**

AEP scatters that answer across six places. Nothing in the framework collects
them, so every consumer that wants to escalate, alert, or report re-invents the
predicate list — and drifts. This spec is the single definition. It is a
**derived view**: computed on read, never stored.

> Decision record: [`docs/decisions/human-alignment.md`](https://github.com/memorysaver/agentic-engineering-patterns/blob/main/docs/decisions/human-alignment.md) D7.
> Consumer: `/aep-human-alignment`.

## Why derived, not stored

- A story status describes **lifecycle position**, not human duty. A `failed`
  story is failed _and_ needs a human; one enum cannot carry both.
- Half the signals are **not stories at all** (amendment log, object maps,
  calibration, open questions).
- A stored copy of a derivable truth is a second source; validation does not
  catch it when it drifts.

## The signal predicates

Each row is a predicate over `product-context.yaml`, the action verb the human
sees, and the priority rank used to pick **the one ask**.

| Rank | Signal            | Predicate                                                          | Verb               |
| ---- | ----------------- | ------------------------------------------------------------------ | ------------------ |
| 1    | Failed story      | `stories[].status == "failed"`                                     | `reset ▸`          |
| 2    | Amendment pending | `architecture.amendment_log[].status == "pending"`                 | `approve ▸`        |
| 3    | Awaiting review   | `stories[].status == "in_review"`, not waived by `skip_human_eval` | `review ▸`         |
| 4    | Calibration due   | plan entry with no matching history entry (see below)              | `/aep-calibrate ▸` |
| 5    | Draft object map  | object-map `status == "draft"`                                     | `/aep-model ▸`     |
| 6    | Open question     | `product.open_questions[]` non-empty                               | `answer ▸`         |

**Only a human may run `failed → pending`.** That is why rank 1 is rank 1: the
work is stopped and no agent can restart it.

**`skip_human_eval`** lives at `layer_gates[].skip_human_eval` (`none` |
`backend` | `all`). A value of `all` waives every `in_review` story in that
layer; `backend` waives those whose story has no UI surface; `none` waives
nothing. An absent field waives nothing — but see Tolerance below, because
absent is not the same as `none`.

**Calibration is derived from plan minus history, not from a status field.**
`calibration.plan[]` entries are `layer · dimensions · trigger`; there is no
`status` field to test, and never has been. A plan entry is **due** when:

1. its `layer` is at or below the project's current layer, and
2. no `calibration.history[]` entry matches its `(layer, dimension)` pair.

`trigger` is prose. Surface it next to the ask; never parse it.

## The one ask

A brief, an alert, or an escalation shows **one** item; the rest join a quiet
zone. Selection is deterministic — two runs at the same commit must choose the
same ask:

1. lowest rank number wins (failed beats amendment beats in_review …);
2. ties break by layer order (lowest layer first);
3. remaining ties break by story/entry id, ascending, byte-wise.

## Schema tolerance is per predicate

Real consumer YAMLs diverge from the current schema in two different ways, and
handling only the first is how an attention set goes silently wrong:

- a whole **section** may be absent (`product`, `opportunity`);
- a **field inside a present section** may be absent — a real consumer's
  `architecture.amendment_log[]` entries carry only `date` and `summary`, though
  the schema defines `status: pending | accepted | rejected`.

Both cases mean the same thing: **the signal is not derivable in this schema.**

A predicate whose fields are absent anywhere along its path is **skipped and
recorded**, never guessed and never treated as "no signal". Emit one record per
skipped predicate:

```json
{
  "predicate": "amendment_pending",
  "path": "architecture.amendment_log[].status",
  "reason": "field absent in this schema version"
}
```

**An empty attention set is only trustworthy when the skip list is also empty.**
A consumer that reports "nothing needs you" while three predicates were skipped
is lying by omission. Say which of the two it is.

## Field typing

The predicate list says which items qualify; a surface usually also wants a
_why_ line. Type every field before rendering it: `business_value` is a numeric
score in real consumer YAMLs (and prose in others), so a why-line derivation
must require prose-typed sources and drop numeric ones rather than render a
meaningless `10`.
