# Context Engineering for the Claude 5 Generation: What Kind of Context, Not How Much

AEP v3.0.0 asked **how much** context a skill may spend and answered with budgets, single-sourcing, and progressive disclosure ([skill-authoring-standard.md](skill-authoring-standard.md)). Anthropic's [_The new rules of context engineering for Claude 5 generation models_](https://claude.com/blog/the-new-rules-of-context-engineering-for-claude-5-generation-models) (2026-07-24) asks a different question — **what kind** of context earns its place — and reports that removing >80% of Claude Code's system prompt for Opus 5 / Fable 5 cost nothing measurable on their coding evals. The corpus that v3.0.0 produced is lean by line count and still wrong by kind: **773,692 bytes of prose reference against one JSON schema**, 123 negation-steered lines telling a model things it does by default, and 23 always-advertised skills whose descriptions sit at 5,006 characters against a 5,000-character CI ceiling. This document adopts the four conversions that follow from the article, scoped to a framework whose consumers are unattended multi-agent executors rather than a human-supervised REPL.

> **Status:** Accepted — the contract that implementation on `migration/claude-5-context-engineering` is reviewed against. Per the owner's direction (2026-07-31) all phases land as **sequential commits on this one branch**, reviewed as a single v4.0.0 PR rather than five; the phase table below therefore reads as commit boundaries, not PR boundaries. Execution order is **C-P2 → C-P3 → C-P1 → C-P4 → C-P5**: the typed artifacts and schemas land first so that C1's "every invariant carries a machine check" has real checks to point at, instead of two passes over the same files. Ships as **v4.0.0** (skill surface, reference shapes, and downstream pointer shapes all change). This document **amends** [skill-authoring-standard.md](skill-authoring-standard.md): R1–R9 remain in force, C1–C6 below are added, and where R7's line budgets and C2's code-reference rule disagree, C2 wins (a 40-line JSON Schema replacing a 400-line prose reference is a win even though it moves bytes into a new file).

> **Measurement note:** all counts are from commit `b42ee2b` (v3.3.1), 2026-07-31. Reproduction commands are in [Diagnosis](#diagnosis).

---

## Why AEP is not Claude Code

The article's subject is a system prompt read by one model, in one session, with a human watching. AEP's skills are read by **workspace agents running unattended**, sometimes on non-Claude backends (`/aep-executor` supports `claude` and `codex`), inside a concurrency protocol where a single violated rule is unrecoverable rather than merely suboptimal — a workspace that writes `product-context.yaml` corrupts state for every other worker; `/aep-wrap` run from a worktree corrupts the archive.

So "let Claude use judgement" is adopted with a stated boundary, and the article itself draws it: _"Avoid making them overconstrained, except in highly important areas."_ The work is not deleting rules — it is **classifying** them, and that classification is the substance of C1.

---

## Diagnosis

### The corpus, measured

| Dimension                                   | v3.3.1 (measured)                          | After this refactor (target)            |
| ------------------------------------------- | -------------------------------------------- | ----------------------------------------- |
| SKILL.md files / lines                      | 23 / 5,197 (max 398)                         | unchanged — line count is not this pass's lever |
| Reference bytes: prose vs typed             | 773,692 in 102 `.md` — **1 `.json`, 4 `.yaml`** | ≥ 30% of reference bytes typed/executable |
| Negation-steered lines in SKILL.md          | 123                                          | ≤ 40, each paired with a check            |
| `NEVER` / `MUST` / `ALWAYS` (whole corpus)  | 64                                           | ≤ 25                                      |
| Always-loaded description chars             | 5,006 (CI cap 5,000)                         | ≤ 3,200                                   |
| Model-invocable skills                      | 23 of 23                                     | ~15, behind a router                      |

```bash
find skills -path "*/references/*" -name "*.md" -exec cat {} + | wc -c   # 773692
grep -rniE "never |do not |don't |must not |avoid " skills --include=SKILL.md | wc -l   # 123
grep -rnE "\bNEVER\b|\bMUST\b|\bALWAYS\b" skills | wc -l   # 64
```

### Failure modes the article names, located in this corpus

**Rules where judgment suffices.** `build/SKILL.md:342-350` opens "Cross-cutting rules with no single step home" and then lists five items of which two are genuine protocol invariants (archive-on-integration-branch, only-main-writes-`product-context.yaml`) and three are things any Claude 5 model does unprompted ("if returning to an in-progress workflow, read the progress file"; "users may ask to skip phases — update the progress file"). `autopilot/SKILL.md:326-336` is denser still: three bullets, three `never`s, of which the WIP-limit and dependency rules are already enforced by `topology.routing.concurrency_limit` and the dependency graph — the prose restates a machine check in English.

**Examples where an interface would do.** The article's worked example is an enum: _"listing status as an enumeration between pending, in_progress, and completed, hints to Claude about how to use it."_ AEP has exactly these enums — `story_status`, `dogfood_target`, `layer_gates`, journey `target_type` — and expresses them as **prose tables restated per skill**, which is precisely the propagation surface behind this repo's #1 bug class (half-applied taxonomy changes, three review rounds on PR #16, recorded in [aep-v2-lesson-learning.md](aep-v2-lesson-learning.md)). An enum that lives in a JSON Schema propagates by validation; an enum that lives in seven markdown tables propagates by review diligence.

**Prose where code is higher fidelity.** _"Generally you should prefer files that are in code as it provides clear, high-fidelity instructions to Claude in a language it knows very well."_ The four largest reference files are all prose descriptions of machine-checkable things: `tick-protocol.md` (36.9 KB — a state machine), `verification-economics.md` (38.0 KB), `backends.md` (32.3 KB — spawn/liveness recipes), `state-schema.md` (15.9 KB — a schema, in prose, named schema). `human-alignment/` is the counter-example that proves the rule and the model to copy: it already ships `facts.schema.json`, `*.lifecycle.json`, `template.html`, and nine `.mjs` scripts, and its SKILL.md does not re-teach any of them.

**An advertising surface at its ceiling.** All 23 skills are model-invocable, so all 23 descriptions load into every session of every downstream repo — 5,006 characters against `check-skills-package.sh:324`'s 5,000-character cap. The corpus is saturated: the 24th skill cannot be added without cutting the other 23. The article's answer is the ToolSearch pattern — _"the agent must search for their full definitions before using them"_ — which is what P4 of the v3.0.0 standard parked as "horizon, not committed". It is now committed, as C3.

**Prescribed memory where auto-memory exists.** `onboard/SKILL.md:44` spends a 6-line paragraph instructing the agent to hand-author a `## Memory & Learning Loop` section into a downstream `AGENTS.md`. The article retires this category outright: _memory in CLAUDE.md files → auto-memory_.

---

## The standard (normative, amends R1–R9)

- **C1 — Constraint classification.** Every imperative in a SKILL.md is one of three kinds, and each kind has one legal form:

  | Kind                     | Test                                                                                       | Legal form                                                                            |
  | ------------------------ | ------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------- |
  | **Protocol invariant**   | Violation corrupts shared state or is unrecoverable without human repair                   | One positive sentence **plus** a machine check (probe, schema, or CI rule) that fails loudly |
  | **Backend compatibility** | Needed because a non-Claude-5 executor reads this file (`codex`, pinned legacy modes)      | One sentence, tagged `<!-- backend-floor -->`, revisited when the executor floor moves |
  | **Taste**                | A capable model does it by default, or reasonable agents may differ                        | Deleted                                                                                |

  A rule with no check is not an invariant — it is taste with a stern voice. Prohibitions survive only under the first two kinds (R6 unchanged, C1 narrows what qualifies).

- **C2 — Code references outrank prose references.** When reference material describes something a machine can hold — a schema, a state machine, an enum, a recipe, a layout, a design — it ships as that artifact (JSON Schema, `.mjs`/`.sh` probe, fixture, lifecycle JSON, HTML mockup) with prose reduced to what the artifact cannot carry: intent and tradeoffs. New reference material may not be authored as prose when a typed form exists. `skills/human-alignment/` is the reference implementation.

- **C3 — Deferred skill surface.** The corpus advertises a router plus the skills a session may plausibly need; run-once and orchestrator-only skills set `disable-model-invocation: true` and are reached by explicit invocation or by the router. Description budget drops to ≤ 3,200 characters corpus-wide. Any invocability or trigger change ships with re-recorded `evals/skill-routing.json` observations and a passing triggering check (R7 unchanged) — this is the rule most able to silently un-wire a skill.

- **C4 — Interfaces, not examples.** Cross-skill taxonomies (`story_status`, `dogfood_target`, `layer_gates`, journey `target_type`, tier names) are declared once in a schema with enumerated values; skills reference the schema and never re-tabulate it. Worked examples survive only where they teach a judgment the schema cannot express.

- **C5 — Host memory over prescribed memory.** AEP prescribes no memory ritual the host already performs. `/aep-onboard` may point at optional memory skills; it does not author memory-loop prose into downstream `AGENTS.md`. AEP's own lessons loop (`/aep-build` → `/aep-wrap` → `/aep-launch`) stays — it is durable project state, not agent recall.

- **C6 — Ratchets, not one-time cuts.** `skills-check.yml` gains three non-increasing ratchets alongside the existing navigation-debt ones: negation count in SKILL.md, `NEVER`/`MUST`/`ALWAYS` count corpus-wide, and description-corpus characters; plus one non-decreasing ratchet: typed-reference byte share. Each implementation PR moves the recorded numbers; none may move them backward.

---

## The refactor (implementation contract)

Each phase is an independent PR against this document, carrying R9 parity evidence for every touched skill.

| Phase                             | Scope                                                                                                                                                                                                                                                 | Behavior change?                    |
| --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------- |
| **C-P0 — this document**          | Record the conversions, the boundary, the ratchets                                                                                                                                                                                                    | No                                  |
| **C-P1 — constraint audit**       | Classify all 123 negations + 64 imperatives per C1 across 23 skills; delete taste; pair each surviving invariant with its check; land the C6 ratchets warn-only. Highest density first: build (29), autopilot (14), human-alignment (12), watch (9)   | No (deletions are no-ops by C1's test) |
| **C-P2 — typed references**       | `state-schema.md` → JSON Schema + prose intent; `tick-protocol.md` → state-machine JSON (the `.lifecycle.json` shape human-alignment already uses) + thin prose; signals (`signals-spec.md`, 17.2 KB) → schema + validator script; `backends.md` recipes → executable probes. Skills stop re-teaching what the artifact declares | No                                  |
| **C-P3 — taxonomy schemas (C4)**  | One schema per cross-skill enum; delete the re-tabulations; wire validation into `skills:check` so a half-applied enum fails CI instead of review                                                                                                     | No                                  |
| **C-P4 — deferred surface (C3)**  | Router skill (upgrade `skills-quick-reference.md`), `disable-model-invocation` on run-once skills, description diet to ≤ 3,200 chars, re-recorded routing evals                                                                                        | **Invocation only** — the risk phase |
| **C-P5 — downstream diet (C5)**   | `/aep-onboard` + `/aep-scaffold` stop authoring memory-loop prose; AGENTS.md template = gotchas + pointers; activate C6 fail thresholds; v4.0.0 bump + migration guide                                                                                 | Downstream scaffolding output       |

**Non-goals.** The loop semantics (launch → build → wrap), the layer-gate coverage model, the orchestrator/workspace boundary, and the verification-economics tiers are out of scope — this pass changes how AEP states things, not what it does. No protocol invariant is deleted for leanness; C1 either keeps it with a check or proves it was never an invariant.

**Propagation discipline.** Product-context `references/` and `templates/` remain build-generated from `_shared/` — every edit goes to `_shared/` then `scripts/build-skills.sh` (files authored directly in a generated dir are silently wiped). Typed artifacts introduced by C-P2/C-P3 follow the same rule. Downstream consumers see nothing until the v4.0.0 tag is cut and each of the 6 repos re-pins.

---

## Anti-patterns this prevents

- **Stern taste.** A `never` with no check behind it, which costs tokens in every run, drifts from the behavior it claims to govern, and teaches reviewers that rules are decorative.
- **Schema in prose.** A file named `state-schema.md` that a machine cannot validate against — the fidelity loss the article's code-reference rule exists to stop.
- **Enum by review diligence.** A taxonomy propagated by remembering to edit seven markdown tables, when a JSON Schema propagates it by failing CI.
- **Advertising everything.** A saturated always-loaded description budget, where every new skill is paid for by every session in every downstream repo whether or not it is ever invoked.
- **Re-implementing the host.** Memory rituals, progress narration, and self-checks that the harness already performs.

---

## References

- [The new rules of context engineering for Claude 5 generation models](https://claude.com/blog/the-new-rules-of-context-engineering-for-claude-5-generation-models) — Anthropic, 2026-07-24. Source of the six then/now conversions and the >80% system-prompt reduction result.
- [skill-authoring-standard.md](skill-authoring-standard.md) — v3.0.0's R1–R9, amended (not replaced) by C1–C6; its P4 "horizon" becomes C-P4.
- [deterministic-orchestration.md](deterministic-orchestration.md) — the probe catalog C1's "machine check" and C2's executable references draw from.
- [aep-v2-lesson-learning.md](aep-v2-lesson-learning.md) + PR #16 review history — the half-applied-taxonomy bug class C4 is aimed at.
- `skills/human-alignment/` — in-repo reference implementation of C2 (schema + lifecycle JSON + HTML template + scripts).
- Affected: all 23 skills, `.github/workflows/skills-check.yml`, `scripts/check-skills-package.sh`, `evals/skill-routing.json`, `docs/glossary.md`, `_shared/` (via build).
