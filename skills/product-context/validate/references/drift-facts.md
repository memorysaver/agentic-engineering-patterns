# Drift Facts

The canonical answer to **"where did reality drift from intent?"**

Nothing in AEP computes intent-vs-reality divergence. This spec defines the
five derivations that do, as a **derived view** over `product-context.yaml` plus
(optionally) the code. Like the attention set, it is computed on read.

> Decision record: [`docs/decisions/human-alignment.md`](https://github.com/memorysaver/agentic-engineering-patterns/blob/main/docs/decisions/human-alignment.md) D7.
> Companion spec: `attention-set.md` (same directory).

## Contents

- [The governing rule](#the-governing-rule)
- [1 · Intent without evidence](#1--intent-without-evidence)
- [2 · Plan behind the architecture](#2--plan-behind-the-architecture)
- [3 · Reality resisting intent](#3--reality-resisting-intent)
- [4 · Control-plane incoherence](#4--control-plane-incoherence)
- [5 · Declared vs. actual architecture](#5--declared-vs-actual-architecture)
- [Verification levels](#verification-levels)

## The governing rule

**Hand-authored drift is banned.** Every drift row a surface renders must cite a
fact this spec derives. An underived drift claim is either noise or a miss, and
both spend the surface's credibility. When nothing derives, the row is
**silent, not fabricated**.

Each derivation below yields facts of the shape:

```json
{
  "kind": "<derivation id>",
  "detail": "<what diverged>",
  "path": "<YAML path>"
}
```

## 1 · Intent without evidence

**Derivation:** a layer gate where
`coverage.criteria_total - coverage.criteria_covered > 0`.

**Not `coverage.uncovered`.** That array is the worklist `/aep-build` fills while
authoring the missing scenarios, so it is empty precisely when the gate never
opened — the highest-drift case. A detector pointed at that field derives nothing
from exactly the case it exists to catch.

`uncovered[]` remains the **detail channel**: when populated it names which
criteria and their `plan` / `WAIVER: <reason>`, and the provenance anchor cites
it.

**Scope: layers with work done or underway.** A future layer whose gate is
honestly `not_started` with zero stories done is **plan, not drift**. Flagging
it floods the surface with deferrals.

A layer qualifies when it holds at least one story in
`{in_progress, in_review, completed, failed, blocked}`. Note what is _excluded_:
`pending` **and `ready`**. A `ready` story is dispatchable, not started — a
layer whose only non-pending story is `ready` has had no work done, and
counting it re-opens exactly the noise this scope closes. State the detector's
boundary on the surface when it matters.

## 2 · Plan behind the architecture

**Derivation:** `architecture.amendment_log[].status == "pending"` — stories were
mapped against a structure that has since been amended.

Subject to the per-predicate tolerance rule in
`attention-set.md`: when entries carry no `status` field,
this detector is **skipped and recorded**, never read as "no pending
amendments".

## 3 · Reality resisting intent

**Derivation:** `failure_logs` on **open** stories. A story that repeatedly
fails is evidence that the spec and the code disagree.

Closed stories' failure logs are **history** — overcome, kept as record,
counted separately. Counting them alongside the open ones reports a project far
more troubled than it is.

Open = not in `{completed, deferred}`.

## 4 · Control-plane incoherence

**Derivation:** at least one `completed` story in a layer whose gate status is
`not_started`.

Write the predicate down and use exactly it. The same situation can be counted
three ways — layers where _every_ story is completed, layers where every story
is completed-or-deferred, layers with _any_ completed story — and a surface that
reports one number under another's definition is a drift fact about itself.

A control plane can be semantically incoherent and still pass validation, which
is why `/aep-validate` Step 0 runs the same derivation mechanically
(`scripts/coherence.mjs`) before any agent spends judgment on it.

## 5 · Declared vs. actual architecture

**Derivation:** the declared module graph (`architecture.modules[].depends_on`)
compared against the real code topology.

The strongest drift form — _"the YAML says A does not depend on B; the code says
it does"_ — and the one that earns human trust, because it cannot be produced by
restating the YAML.

**Today it usually cannot be computed.** No schema field binds a declared module
to source paths, so the conceptual modules in the YAML and the packages in the
repo routinely share **no names at all**: the declared architecture is not
code-addressable. That gap is itself a drift fact — report it rather than
pretending the graphs agree.

**Framework recommendation:** add `architecture.modules[].paths` — a glob
binding from declared module to source paths — so dependency-cruiser-class
tooling can turn every declared `depends_on` into a lint rule with `file:line`
evidence.

Toolchain-dependent, so it degrades honestly: no scanner → the view is marked
`unverified`, never blocked.

## Verification levels

Every drift fact carries how well it is known:

| Level           | Meaning                                                      |
| --------------- | ------------------------------------------------------------ |
| `derived`       | computed from `product-context.yaml` alone (1–4)             |
| `code-verified` | computed against the real code topology (5, scanner present) |
| `unverified`    | the derivation ran but its evidence source was unavailable   |

Never render a fact at a level above the one that produced it.
