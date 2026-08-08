---
name: aep-easy-explain
description: Stop. That last message did not land — re-pitch it.
disable-model-invocation: true
---

Wait — I don't understand where you've got to here. Re-pitch that: give me a
little bit of context, talk in ASD-STE100 Simplified Technical English, and use
this project's own nouns — `product-context.yaml` and `docs/glossary.md` where
they exist.

---

## The register (for orchestrator skills)

The repair line above is also AEP's standing **explain-to-a-human register**.
Main-session skills that render status, escalations, or decisions for a person
(`/aep-autopilot` status and escalations, `/aep-dispatch` handoff summaries,
`/aep-wrap` layer-advance asks) write in it by default:

1. **One line of context first** — what was happening when this became worth
   saying.
2. **ASD-STE100 Simplified Technical English** — short sentences, one clause,
   active voice, one meaning per word.
3. **The project's own nouns** — the ubiquitous language from
   `product-context.yaml` and the glossary, in place of invented terms and
   stacked acronyms.

A re-pitch is working if it is **shorter and clearer, not shorter and blunter**:
it adds the premise that was missing instead of only deleting words, and it
survives being invoked twice in a row without degrading into terseness.

This skill is **user-typed only** (`/aep-easy-explain`), and that is the design:
only the human knows when they stopped following. The model reaching for it on
its own would grade its own clarity — the same self-evaluation `/aep-gen-eval`
exists to forbid.

> Adapted from [mattpocock/skills](https://github.com/mattpocock/skills)
> `productivity/wait-what` — the three-line original, whose insight is that
> naming the **listener's state** ("wait, you lost me") repairs comprehension,
> where naming the output ("be concise") produces telegrams.
