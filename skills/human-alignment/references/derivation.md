# Derivation: YAML path → fact

`derive.mjs` is the implementation; this is the map it implements and the typing
rules that keep it honest. The two cross-cutting derivations —
`attention-set.md` and `drift-facts.md` —
are framework specs with other future consumers and are **not** restated here.

## Field map

| Fact path                     | Source                                                       |
| ----------------------------- | ------------------------------------------------------------ |
| `story_total`, `by_status`    | `stories[]`, grouped by `status`                             |
| `open_total`                  | stories not in `{completed, deferred}`                       |
| `current_layer`               | lowest layer holding an open story                           |
| `layers[]`                    | stories and gate status per layer, with the five-cell stage  |
| `gates.by_status`             | `layer_gates[].status`                                       |
| `gates.vocabulary_violations` | a gate status outside the defined set (below)                |
| `capabilities[]`              | `passed` → `IS`; `scripted_passed` → `EXP`                   |
| `changelog_recent[]`          | last 12 `changelog[]` entries, each with a stable id         |
| `cost`                        | `cost.total_usd`, plus whether the plane is wired at all     |
| `modules`                     | `architecture.modules[]` and how many carry `paths`          |
| `code_graph`                  | `scan-workspace.mjs` output, joined to declared module names |
| `delta`                       | this commit vs. `manifest.json`'s baseline commit            |

## Typing rules

**Type every field before rendering it.** The rules that earned their place:

- **`business_value` may be a number.** In real consumer data it is a numeric
  score in some repos and prose in others. A why-line derivation must require
  **prose-typed** sources and drop numeric ones — rendering a bare `10` as a
  reason is worse than rendering nothing.
- **Layer ids may be strings or numbers.** `7`, `7.0`, and `"7"` are one layer.
  Normalize both sides of any join (plan ↔ history, story ↔ gate) or a matched
  record silently reads as unmatched.
- **Absent ≠ empty.** A missing field means _not derivable_, never _zero_. See
  the per-predicate tolerance rule in `attention-set.md`.

## The gate vocabulary

Defined states: `not_started` → `scripted_passed` → `passed`, plus `deferred`
and `waived`. Anything else is a vocabulary violation and is surfaced as such —
control files drift in their wording before they drift in their content.

The two-phase distinction is load-bearing for the product band:
`scripted_passed` means the scripted run went green; `passed` means coverage was
checked **and** human acceptance ran. Only `passed` yields a fact.

## Delta

The baseline is `manifest.json`'s `commit`, not a filename sort — the manifest
is branch-tracked, so every branch carries its own correct baseline and pruning
never corrupts the delta.

`derive.mjs` re-reads the plan file at the baseline commit via `git show` to
compute per-story status transitions. If the baseline is unreachable (pruned
branch, shallow clone), the run degrades to first-run shape and says so rather
than reporting a delta it cannot compute.

## Changelog binding

Changelog translations bind by **entry id** (`date` + kind + index), never by
list position. A single new entry shifts every position; a translation bound to
position silently re-labels the wrong change.

## Cost

`total_usd == 0` while `by_story` holds entries means the cost plane is
**declared but unwired**. Say that, rather than reporting a cost of zero.
