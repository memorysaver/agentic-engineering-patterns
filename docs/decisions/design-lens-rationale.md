# Design Lens — Rationale

> Decision record. Migrated out of `skills/patterns/design-lens/SKILL.md` under the
> [skill-authoring standard](./skill-authoring-standard.md) (R5: rationale lives in
> `docs/decisions/`, not the agent hot path). Records why `/aep-design-lens` is shaped
> the way it is.

## Why a product-agnostic method, not a product-type table

A fixed "dashboard → theories X" lookup drifts the moment a product is a hybrid, and it
never covers the long tail. Classifying **tasks + data** (Munzner's abstraction layer)
selects the right lenses for _any_ product, and the reusable value moves into the theory
catalog + the selection rules — which are stable — rather than a brittle lookup that
needs constant maintenance.

## Why it sits beside `/aep-model` and `/aep-calibrate`, not inside them

They own different jobs: `/aep-model` decides the object/screen **structure** (the
_what_); `/aep-calibrate` captures a human's **taste** decision (the _chosen_
look/voice/flow). Neither owns the **external, evidence-based theory** — the _why_.
Folding theory into either would blur a clean separation and hide the health-check. This
skill supplies the theory both can lean on and points back to them for their jobs.

## Why hybrid output (report by default, opt-in file)

Most runs are a quick guide or audit inside a conversation; forcing a persisted artifact
every time adds ceremony. But a design guideline is worth keeping, so persistence is one
ask away — as a standalone markdown deliverable that touches no schema, keeping this a
zero-drift leaf skill.

## Why the health-check uses Nielsen's 0–4 severity

A checklist without severity invites "everything is minor." Nielsen's scale (0 = not a
problem … 4 = usability catastrophe) forces triage and makes "does it meet human
expectations?" answerable rather than vibes-based.

## Why two depths (quick check vs deep audit)

Tesler's Law applied to the skill itself: forcing the full 7-step audit on every "is this
OK?" question pushes the method's cost onto the user, so the cheap path would get skipped.
The Baseline Ten gives a minutes-scale sweep with an honest escalation rule — any
major/catastrophe finding upgrades to a deep audit — so speed never silently trades away
rigor.

## Why accessibility always fires (family G)

The other families assume the user can perceive and operate the UI at all; accessibility
is that precondition, not a nice-to-have. WCAG criteria are checkable and severity-scorable
exactly like heuristic failures, and impairments are situational as often as permanent
(glare, one hand, noise), so G applies to every human-perceived surface — including
terminal output.
