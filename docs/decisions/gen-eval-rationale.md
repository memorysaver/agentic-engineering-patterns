# Gen/Eval as a Utility Skill — Rationale

> Decision record. Migrated out of `skills/patterns/gen-eval/SKILL.md` under the
> [skill-authoring standard](./skill-authoring-standard.md) (R5: rationale lives in
> `docs/decisions/`, not the agent hot path). Records why the generator/evaluator
> pattern is its own invocable skill that also serves as a reference library.

## Why a utility skill, not just reference files

- Invocable directly (`/aep-gen-eval`) for ad-hoc validation of any artifact.
- Appears in the skill list, so the pattern is discoverable.
- Carries its own description, so agents reach for it when appropriate.
- Its `references/` directory stays accessible to consumer skills by path.

## Why not merge into `/aep-validate`

`/aep-validate` is a product-context skill with four fixed modes (product, design,
code, document). The gen/eval pattern is more general — it applies to any evaluation
scenario. `/aep-validate` _consumes_ the pattern; it is not the pattern itself.

## Why not keep it in `/aep-launch`

Launch only sets up evaluation criteria; it does not run the loop. The scoring
framework is consumed by build, validate, _and_ launch, so housing it in launch
creates a confusing ownership model. Gen/eval owns the framework; launch reads it.
