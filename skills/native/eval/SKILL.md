---
name: eval
description: Watch another agent's work as a neutral observer (aep eval watch), judge AEP adherence and engineering practice from structural evidence, and record findings outside the project.
---

# Neutral engineering-quality observation

`aep eval` looks at how a project is run, not at what it builds. The observer answers two questions: does the work follow the AEP process, and which software-engineering or DevOps practices would improve it. Product decisions, feature value, priorities and technical choices are the project's own business and stay out of every finding.

Everything the observer produces lives under the machine-level AEP home (`~/.aep/eval/<project>/<run>/`), never inside the project. The CLI computes the structural facts; the observer reads, compares and explains.

## Two ways to run it

Without a live agent, `aep eval snapshot` collects facts and `aep eval report` applies the rules; `aep eval show <run>` and `aep eval list` read past runs. Read [Rules](references/rules.md) (`aep --skill eval --ref rules`) for what each finding means and what it does not claim.

Under Herdr, the person opens a pane where they want the observer, starts an agent there and says which agent to watch. That agent runs `aep eval watch` to list the agents working in the project, confirms the target with the person, then runs `aep eval watch --target <pane-id>` and becomes the observer, following [Observer procedure](references/observer.md) (`aep --skill eval --ref observer`): snapshot at the target's milestones, compare its claims with records and Git, and record each observation with `aep eval record`.

## What counts as evidence

Structural facts first: `aep check`, readiness, record links, delivery receipts, evidence bound to revisions, Git state. A transcript shows what an agent said; records and Git show what happened. When they disagree, the observation says so and cites both. An agent's completion claim is not evidence of completion.

Describe model behavior as an observation, not as grounds for a new rule. A finding names the practice, the structural fact and the evidence path.

## Boundaries

The observer reads the project and writes only to its run directory. It sends no input to the working agent, answers none of its questions, edits no project file and commits nothing on the project's behalf. Findings reach the project only when a person asks for them.
