# Method & Templates

How `/aep-design-lens` turns _any_ product into (1) design suggestions, (2) a design
guideline, and (3) a severity-scored health-check. The method is product-agnostic: it
classifies **tasks + data** and selects lenses from
[`theory-catalog.md`](theory-catalog.md), rather than matching a fixed product type.

---

## The 7-step method

### 1. Characterize (domain problem)

Establish the product in the user's terms. Ask (or infer from the spec / running UI):

- What is it, in one sentence?
- Who are the primary users, and what is their expertise?
- What are they trying to accomplish, and in what context (frequency, stakes, device)?

Keep this short — 3–5 sentences. It anchors everything downstream. This is Munzner's
**domain** layer.

### 2. Abstract (tasks + data)

Translate the domain into two classifications — the product-agnostic core of the method.

**Primary user tasks** (pick the 1–3 that dominate):

| Task                       | Meaning                                                      |
| -------------------------- | ------------------------------------------------------------ |
| **monitor**                | watch state over time, notice when something needs attention |
| **compare**                | judge items against each other                               |
| **look-up**                | retrieve a known item                                        |
| **explore / browse**       | wander an information space without a known target           |
| **diagnose**               | find the cause of a problem                                  |
| **decide**                 | choose among options / approve                               |
| **create / author**        | produce an artifact (write, configure, compose)              |
| **reconstruct-provenance** | understand how a result came to be (history/trace)           |
| **converse**               | interact in natural language with a system/agent             |

**Data / content type** (pick what applies): quantitative/metrics · categorical ·
time-series · hierarchical/tree · network/graph · geospatial · text/documents ·
media · records/entities · logs/traces · conversational/free-text.

### 3. Select lenses

Apply the selection rules to the task+data profile. Families A, B, F, G always fire;
C, D, E fire on triggers.

| If the product…                                                                                                  | Add family                               |
| ---------------------------------------------------------------------------------------------------------------- | ---------------------------------------- |
| has any human-facing UI (always)                                                                                 | **A** Usability & interaction heuristics |
| has any human-facing UI (always)                                                                                 | **B** Cognitive & perceptual laws        |
| involves a **large/complex info space** or **explore/browse/look-up** tasks                                      | **C** Information seeking & navigation   |
| shows **quantitative/metric/time-series/graph data** or **compare/monitor/diagnose** tasks                       | **D** Data visualization & encoding      |
| is an **LLM/agent/chat/prompt/workflow-composer**, or **converse** tasks, or the system takes autonomous actions | **E** Human-AI & agent-specific UX       |
| always (to run and score the review)                                                                             | **F** Evaluation & process methods       |
| has any human-perceived UI, incl. terminal output (always)                                                       | **G** Accessibility & inclusive design   |

Within each selected family, pull the lenses whose **When** matches. Note _why_ each
family fired — that traceability is what makes the output defensible, not generic.

### 4. Synthesize (suggestions)

For each selected lens, turn its **Apply** line into 1–3 concrete, product-specific
suggestions. Be specific to the characterized product — "show the last tool call and
cost on each run card (information scent)" beats "improve information scent."

### 5. Emit the design guideline

Reorganize the suggestions into prescriptive **"the product should…"** statements,
grouped by family. This is the build-against artifact (feeds `/aep-model` and
`/aep-calibrate`). Use the [guideline template](#design-guideline-template).

### 6. Emit the health-check table

Convert each lens's **Check** line into one or more checkable rows, each scored with
Nielsen's 0–4 severity (F3). Score against the current design if one exists; if it's
pre-build, mark rows as "target" (the guideline to meet). Use the
[health-check template](#health-check-table-template). Rank by severity; summarize.

### 7. (Opt-in) persist

By default, present the report in the conversation. If the user asks to save it, write a
standalone markdown file — see [Persisted-file layout](#opt-in-persisted-file-layout).
**Never** write to `product-context.yaml` or any AEP schema file.

---

## The Baseline Ten (quick check)

The fast path for "is this OK?" questions: skip steps 3–5 and score only these ten
distilled, cross-family checks (0–4 each, same scale as the full table). Skip a row
only if its family genuinely doesn't apply (8 needs data, 9 needs an agent).

| #   | Lens  | Check (does the design…?)                                                   |
| --- | ----- | --------------------------------------------------------------------------- |
| 1   | A1    | …show status for every async action?                                        |
| 2   | A1/A2 | …offer undo/exit for destructive or wrong-turn actions?                     |
| 3   | A1    | …phrase errors in plain language with a recovery path?                      |
| 4   | B1/C1 | …default to a summary, with detail on demand?                               |
| 5   | B3    | …keep choice sets small/grouped, with a recommended default?                |
| 6   | B5    | …acknowledge every action fast (~400ms), with progress for longer waits?    |
| 7   | C2    | …give each list/card item enough scent to decide whether to open it?        |
| 8   | D2    | …encode precise comparisons by position/length, not area/angle/color?       |
| 9   | E5/E6 | …make agent actions legible (summary→detail) and reversible/interruptible?  |
| 10  | G2/G3 | …work keyboard-only, meet contrast minima, and never signal by color alone? |

**Escalation rule:** any row scoring 3–4, or the user asking "why" / "what should we
do about it", upgrades the run to the full deep audit. Report the ten rows + a
two-sentence verdict; don't pad.

---

## Auditing a live UI (evidence gathering)

When a running product exists, score against **what you observe, not what the spec
claims**. Before scoring:

1. **Walk the top 1–3 tasks as a first-time user** (F2's four questions per step),
   from entry to completion — not just the screens in isolation.
2. **Capture evidence per finding** — a screenshot, the exact copy string, or the
   step where the walkthrough broke. The Evidence column must cite something
   observed; "probably fails" is not a finding.
3. **Exercise the unhappy paths** — submit an empty form, kill the network mid-run,
   stop an in-flight agent action. Most 4s live there.
4. **Probe G cheaply** — tab through the primary flow, check contrast on the worst
   text/background pair, squint-test whether state survives without color.

If the product is deployed, drive it with the agent browser (same tooling as
post-deploy e2e verification) and screenshot each primary screen. If it's pre-build,
audit the spec/wireframes and mark rows `target` instead of scoring.

---

## Worked selection examples

These show the abstraction → selection step; they are illustrations, not a lookup table.

**Agent observability console** — _monitor + diagnose + reconstruct-provenance_ over
_logs/traces + metrics_. Fires **all of A–G**: overview→filter→detail for runs (C1),
scent on run cards (C2), preattentive error spotting (B7), position/length for
cost/latency (D2), coordinated linked views + provenance (D4), progressive disclosure
of agent reasoning + steerability (E5, E6), status never encoded by color alone (G3).
The richest case.

**Marketing landing page** — _look-up + decide_ over _text/media_. Fires **A + B
(+ minimal C) + F + G**; **not D** (no data), **not E** (no agent). Emphasis: Gulf of
Execution for the CTA (A3), visual hierarchy & Gestalt (B6), aesthetic-usability (B8),
scent toward the single conversion action (C2), contrast + alt text + captions (G1, G3).
Applying data-viz rules here would be noise.

**CLI tool that wraps an agent** — _create/author + converse_ over _conversational/text_.
Fires **A + B + E + F + G**; **not D**. Even without a GUI: Gulf of Envisioning (does the
user know what the agent can do and how to instruct it, E1), conversational maxims for
output (E7), steerability/reversibility of actions (E6), feedback & error recovery (A1),
and terminal accessibility — meaning must survive without color, output must read well
in a screen reader or plain pipe (G3).

---

## Design guideline template

```markdown
# Design Guideline — <product>

**Characterization:** <1–2 sentences: what it is, who uses it, what they do.>
**Primary tasks:** <e.g. monitor, diagnose> **Data:** <e.g. logs/traces, metrics>
**Lenses selected:** A, B, C, D, E, F, G — <one clause on why C/D/E fired (or didn't)>

## A. Usability & interaction

- The product should <prescriptive statement>. _(→ A1 visibility of status)_
- ...

## B. Cognition & perception

- The product should <…>. _(→ B1 progressive disclosure)_

## C. Information seeking <!-- include only if family fired -->

- ...

## D. Data visualization <!-- include only if family fired -->

- ...

## E. Human-AI / agent UX <!-- include only if family fired -->

- ...

## G. Accessibility & inclusion

- The product should <…>. _(→ G3 color independence)_

## Top 5 (do these first)

1. <highest-leverage guideline, cross-family>
```

Every statement carries a `(→ lens id)` tag so a reader can trace it back to the catalog.
Only include families that fired. End with a cross-family "Top 5" so the guideline is
actionable, not just exhaustive.

---

## Health-check table template

One row per checkable expectation. Severity uses **Nielsen 0–4** (see F3): 0 not a
problem · 1 cosmetic · 2 minor · 3 major · 4 catastrophe. For a pre-build product, use
the **Status** column value `target` instead of a severity.

```markdown
# Design Health-Check — <product> (<date>)

**Scope:** <what was reviewed — a running URL, a spec, a set of screens>
**Lenses:** A, B, C, D, E, F, G

| #   | Lens | Check (does the design…?)                       | Status  | Severity | Evidence / fix                                                     |
| --- | ---- | ----------------------------------------------- | ------- | -------- | ------------------------------------------------------------------ |
| 1   | A1   | …show status for every async action?            | fail    | 4        | Agent runs show no progress → users refresh. Add streaming status. |
| 2   | A3   | …let users tell each primary action succeeded?  | pass    | 0        | Toasts + state change present.                                     |
| 3   | C2   | …give each run card enough scent to triage?     | partial | 3        | Cards show name only; add status, cost, last tool call.            |
| 4   | D2   | …encode precise comparisons by position/length? | fail    | 3        | Cost shown as donut; switch to bars.                               |
| 5   | E6   | …make pause/rollback consequences predictable?  | fail    | 4        | "Stop" gives no confirmation of effect on in-flight work.          |
| 6   | G3   | …signal run state by more than color alone?     | fail    | 3        | Red/green dots only; add icon or label per state.                  |

## Summary

- **Catastrophes (4):** <count> — <one-line each>
- **Major (3):** <count>
- **Minor/cosmetic (1–2):** <count>
- **Overall:** <does the design meet human expectations? one honest paragraph, led by the worst findings.>
```

**Scoring guidance:** severity = frequency × impact × persistence. A rare cosmetic glitch
is a 1; a blocker on the primary task hit every session is a 4. Rank the table by severity
descending. The **Overall** verdict should be led by the catastrophes/majors — a design
with any unaddressed 4 does not meet human expectations, however many rows pass.

**Honesty rule (from `/aep-gen-eval`):** do not inflate. If a check passes, score it 0 and
move on; if the design is genuinely good, a short table with low severities is the correct
output. The value is in the true 3s and 4s, not in padding.

---

## Opt-in persisted-file layout

Only when the user asks to save. Write one self-contained markdown file (guideline +
health-check together), default path `docs/design-review/<scope>-<YYYY-MM-DD>.md`
(create the directory; honor any path the user gives). Structure:

```markdown
# Design Review — <product> (<date>)

## 1. Characterization & lens selection

<step 1–3 output: characterization, task/data abstraction, which families fired and why>

## 2. Design guideline

<the guideline template output>

## 3. Health-check

<the health-check table + summary>
```

Do **not** touch `product-context.yaml`, `calibration/`, `product/`, or any schema file —
this skill is advisory and adds no taxonomy. If the findings should drive AEP work, route
them (not this file) into `/aep-calibrate`, `/aep-model`, or `/aep-reflect`.
