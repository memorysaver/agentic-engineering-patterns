# AEP Skills Workflow Demo

This demo is a single-file web slide deck introducing Agentic Engineering Patterns in Taiwan Mandarin.

## Purpose

The deck explains AEP as a specification operating system for agentic software development:

- Human judgment stays in the control plane.
- Agents execute in the execution plane.
- Structured artifacts carry intent, status, and feedback between sessions.
- Product planning, story mapping, dispatch, build, evaluation, and wrap-up form one closed loop.

## Audience

The presentation is meant for an engineering or AI product audience that already understands coding agents, but needs a clearer model for turning them into a reliable workflow rather than a loose chat-based swarm.

## Core Narrative

1. Agent execution capacity is no longer the scarce resource.
2. Specification quality becomes the bottleneck.
3. AEP separates thinking from doing.
4. `product-context.yaml` is the durable control-plane map.
5. Story maps turn parallel execution into scheduled work.
6. `.dev-workflow/` harness files let long-running agents recover, verify, and report progress.
7. Generator/evaluator separation keeps validation honest.
8. Reflection turns shipped work and process lessons back into the next dispatch cycle.

## Files

- `index.html` — the slide deck.
- `assets/motion.min.js` — local Motion One fallback for offline playback.
- `qa/` — screenshots used to visually verify key slides.

The deck intentionally uses HTML/CSS diagrams for the core workflow modules so the diagrams remain editable and explanatory.
