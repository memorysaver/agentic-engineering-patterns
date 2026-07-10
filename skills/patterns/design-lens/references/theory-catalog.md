# Theory Catalog

The reusable knowledge base for `/aep-design-lens`. Each lens is a compact,
established result from HCI / design / visualization research. Lenses are grouped
into **families**; the method (`method-and-templates.md`) selects which families
fire for a given product, then draws suggestions and health-check items from the
lenses inside them.

Every entry follows the same shape:

- **Q** — the core question the lens answers.
- **When** — the product profile where it fires.
- **Apply** — how to turn it into a design suggestion / guideline statement.
- **Check** — the health-check item(s) it produces (phrase as a yes/no you can score 0–4).
- **Source** — the canonical reference.

> **This catalog is append-only and extensible.** To add a lens, see
> [Extending the catalog](#extending-the-catalog) at the end. Keep entries in the
> family that matches their selection trigger so the method keeps working.

---

## Family A — Usability & interaction heuristics

_Fires for: any interactive UI (baseline)._ The general-purpose "is this usable?"
layer. Apply to essentially every product with a human-facing surface.

### A1. Nielsen's 10 Usability Heuristics

- **Q:** Will users get lost, make errors, or have to remember too much?
- **When:** Any UI. This is the default checklist.
- **Apply:** Design each surface to satisfy all ten: (1) visibility of system status,
  (2) match between system and the real world, (3) user control & freedom (undo/exit),
  (4) consistency & standards, (5) error prevention, (6) recognition rather than recall,
  (7) flexibility & efficiency (shortcuts), (8) aesthetic & minimalist design,
  (9) help users recognize/diagnose/recover from errors, (10) help & documentation.
- **Check:** One row per heuristic — e.g. "Does every async action show its status?",
  "Can the user undo destructive actions?", "Are errors phrased in plain language with a recovery path?"
- **Source:** Nielsen, "10 Usability Heuristics for User Interface Design" (NN/g, 1994/2020).

### A2. Shneiderman's Eight Golden Rules of Interface Design

- **Q:** What are the durable rules of direct-manipulation interface design?
- **When:** Any UI; especially interactive tools with frequent actions.
- **Apply:** Strive for consistency; enable frequent users to use shortcuts; offer
  informative feedback; design dialogs to yield closure; prevent errors; permit easy
  reversal of actions; support internal locus of control; reduce short-term memory load.
- **Check:** "Do multi-step flows have a clear completion state (closure)?", "Is the
  user in control (system doesn't act surprisingly on their behalf)?"
- **Source:** Shneiderman & Plaisant, _Designing the User Interface_.

### A3. Norman's Design Principles + the Two Gulfs + Seven Stages of Action

- **Q:** Do users know what they can do, how to do it, and whether it worked?
- **When:** Any UI; strongest where the mapping from intent → action → result is non-obvious.
- **Apply:** Use Norman's six principles — **visibility, feedback, affordances/signifiers,
  mapping, constraints, consistency**. Close the **Gulf of Execution** (make possible
  actions and how to invoke them visible) and the **Gulf of Evaluation** (make the
  resulting system state and whether it matched intent perceivable). For each action,
  answer the seven stages: goal → plan → specify → perform → perceive → interpret → compare.
- **Check:** "For each primary action: can the user tell it's possible, how to do it,
  and that it succeeded?", "Do controls afford their use (look clickable/draggable)?"
- **Source:** Norman, _The Design of Everyday Things_.

### A4. Jakob's Law

- **Q:** Do users' expectations from other products carry over to this one?
- **When:** Any UI; especially conventional patterns (nav, forms, tables, auth).
- **Apply:** Users spend most of their time on _other_ products, so meet established
  conventions unless you have a strong reason not to. Novelty has a usability cost.
- **Check:** "Do standard patterns (search box, primary nav, form validation) behave the
  way users expect from mainstream products?"
- **Source:** Nielsen / Jakob's Law (Laws of UX).

### A5. Tesler's Law (Conservation of Complexity)

- **Q:** Where does the irreducible complexity live — in the product or in the user?
- **When:** Any UI with inherent domain complexity (config, pipelines, agent setup).
- **Apply:** Every application has irreducible complexity; the design decides who
  absorbs it. Push it into the system (smart defaults, inference) rather than onto the user.
- **Check:** "Is the user forced to supply information the system could infer or default?"
- **Source:** Larry Tesler / Laws of UX.

### A6. Postel's Law (Robustness Principle)

- **Q:** Is the interface forgiving of varied input while producing clean output?
- **When:** Inputs (forms, search, prompts, uploads, command entry).
- **Apply:** Be liberal in what you accept (tolerate formats, whitespace, casing, paste),
  conservative in what you emit (consistent, validated output). Reduce input friction.
- **Check:** "Does input parsing tolerate reasonable variation instead of rejecting it?"
- **Source:** Jon Postel / Laws of UX.

---

## Family B — Cognitive & perceptual laws

_Fires for: any interactive UI (baseline)._ How the human mind and eye process the
interface. Governs density, grouping, timing, and where attention goes.

### B1. Cognitive Load Theory (+ progressive disclosure)

- **Q:** Will the user's limited working memory get overloaded?
- **When:** Any information-bearing UI; critical for dense/complex screens.
- **Apply:** Minimize **extraneous** load (chrome, noise, redundant options) so effort
  goes to the real task. Chunk, layer, and **progressively disclose** — summary first,
  detail on demand. Provide sensible defaults.
- **Check:** "Is the default view a summary rather than the full firehose?", "Are options
  and controls limited to what the current task needs?"
- **Source:** Sweller, Cognitive Load Theory.

### B2. Miller's Law (chunking, 7±2)

- **Q:** How much can the user hold in mind at once?
- **When:** Lists, menus, groupings, steps, codes/IDs.
- **Apply:** Don't rely on users holding more than a handful of items in working memory;
  chunk long sequences (phone numbers, IDs, steps) into groups.
- **Check:** "Are long strings/lists chunked into digestible groups?"
- **Source:** Miller, "The Magical Number Seven, Plus or Minus Two" (1956).

### B3. Hick's Law

- **Q:** How does the number of choices affect decision time?
- **When:** Menus, toolbars, option sets, onboarding, command palettes.
- **Apply:** Decision time grows with the number and complexity of choices. Reduce or
  categorize options; highlight recommended defaults; stage advanced options.
- **Check:** "Are choice sets small or grouped, with a recommended default surfaced?"
- **Source:** Hick–Hyman Law / Laws of UX.

### B4. Fitts's Law

- **Q:** How fast can users acquire a target given its size and distance?
- **When:** Any pointer/touch target — buttons, hit areas, menus, drag handles.
- **Apply:** Make frequent/important targets larger and closer to where the pointer is;
  exploit screen edges/corners (infinite width); avoid tiny or crowded controls.
- **Check:** "Are primary actions large enough and not crowded by adjacent controls?",
  "Are touch targets ≥ ~44px?"
- **Source:** Fitts (1954); MacKenzie's HCI formulation.

### B5. Doherty Threshold

- **Q:** Is the system responsive enough to keep users engaged?
- **When:** Any interactive action, especially latency-prone ones (queries, agent runs).
- **Apply:** Keep response under ~400ms where possible; otherwise show immediate
  acknowledgment, progress, and (for agents) streaming/partial output so the wait feels
  productive.
- **Check:** "Does every action acknowledge within ~400ms, with progress for longer waits?"
- **Source:** Doherty & Thadani (IBM, 1982) / Laws of UX.

### B6. Gestalt Principles of Grouping

- **Q:** How does the eye group elements into structure?
- **When:** Any visual layout — spacing, grouping, cards, sections, diagrams.
- **Apply:** Use **proximity, similarity, common region, closure, continuity,
  figure/ground** to make relationships visible without borders/labels. Group related
  controls; separate unrelated ones with whitespace.
- **Check:** "Do visually grouped elements actually belong together (proximity/common
  region match the data model)?"
- **Source:** Gestalt psychology (Wertheimer et al.).

### B7. Preattentive Attributes

- **Q:** What can users perceive in <200ms without focused scanning?
- **When:** Dense displays where users must spot the exception fast (dashboards, logs,
  status grids).
- **Apply:** Encode the "thing to notice" (error, outlier, selection) with a preattentive
  channel — hue, intensity, motion, size, orientation — so it pops without reading.
- **Check:** "Can a user spot the failing/anomalous item at a glance, without scanning
  every row?"
- **Source:** Healey & Enns, "Attention and Visual Memory in Visualization."

### B8. Aesthetic-Usability Effect

- **Q:** Does perceived beauty change perceived (and actual) usability?
- **When:** Any UI; first impressions and trust.
- **Apply:** A clean, aesthetically coherent design is perceived as more usable and buys
  tolerance for minor issues — but don't let polish hide real usability problems.
- **Check:** "Is the visual design coherent enough to earn trust, without masking
  unresolved usability defects?"
- **Source:** Kurosu & Kashimura (1995); NN/g.

### B9. Memory & motivation biases

- **Q:** Which moments and items will users actually remember / be motivated by?
- **When:** Multi-step flows, onboarding, long lists, progress-bearing tasks.
- **Apply:** **Peak-End Rule** — design a strong peak and a clean ending; **Serial
  Position** — put key items first/last; **Von Restorff** — make the one thing you want
  remembered distinct; **Zeigarnik** — show incomplete tasks to pull completion;
  **Goal-Gradient** — show progress so motivation rises near the finish.
- **Check:** "Do long tasks show progress toward completion?", "Is the most important
  item positioned/emphasized to be remembered?"
- **Source:** Laws of UX (Kahneman/Tversky lineage; Ebbinghaus; Zeigarnik; Hull).

---

## Family C — Information seeking, foraging & navigation

_Fires for: large or complex information spaces, exploration-heavy products_ (log
explorers, search, doc libraries, catalogs, run/trace browsers).

### C1. Shneiderman's Visual Information-Seeking Mantra

- **Q:** In what order should users explore a complex information space?
- **When:** Any product where users browse/search/filter a large set.
- **Apply:** **Overview first → zoom and filter → details-on-demand.** Give a scannable
  overview, powerful filtering/zoom, and detail only when requested. This is the spine of
  dashboards, list→detail flows, and run/trace explorers.
- **Check:** "Is there an overview before detail?", "Can users filter/zoom to a subset?",
  "Is detail available on demand rather than always-on?"
- **Source:** Shneiderman, "The Eyes Have It" (1996).

### C2. Information Foraging Theory (information scent)

- **Q:** How do users decide whether to click in, keep reading, or leave?
- **When:** Search results, list/card views, navigation, documentation, dashboards.
- **Apply:** Strengthen **information scent** — titles, summaries, previews, breadcrumbs,
  filter counts, status, "next" cues — so users can judge value before paying the click
  cost. Reduce the cost of the next step.
- **Check:** "Does each list/card item carry enough scent (title, status, cost, summary,
  last action) to decide whether to open it?"
- **Source:** Pirolli & Card, Information Foraging Theory (PARC).

### C3. Focus + Context / Fisheye + the multiscale trio

- **Q:** When users zoom into a part, how do they keep the whole in view?
- **When:** Maps, timelines, graphs, large trees, agent run/DAG views.
- **Apply:** Choose a multiscale strategy — **Overview+Detail** (two coordinated views),
  **Zoom+Pan** (one view, navigable), or **Focus+Context / fisheye** (magnify locally
  while compressing the surround). Preserve the user's sense of global position.
- **Check:** "When a user drills into one node/region, can they still see where it sits in
  the whole?"
- **Source:** Cockburn, Karlson & Bederson, "A Review of Overview+Detail, Zooming, and
  Focus+Context Interfaces."

### C4. Information Architecture & wayfinding

- **Q:** Do users always know where they are, how they got here, and where they can go?
- **When:** Multi-section products, nested navigation.
- **Apply:** Provide consistent, labeled structure — breadcrumbs, clear active state,
  predictable back/up, meaningful URLs. Names should match the user's vocabulary.
- **Check:** "At any screen, is the user's location and path back obvious?"
- **Source:** Rosenfeld, Morville & Arango, _Information Architecture_.

> Note: IA here is **wayfinding/orientation** (does the user stay oriented). The
> object/screen **structure** itself (which objects, what screens) is
> [`/aep-model`](../../../product-context/model/SKILL.md)'s job — reference it, don't
> redo it.

---

## Family D — Data visualization & graphical encoding

_Fires for: data-dense products_ (charts, dashboards, BI, analytics, observability,
metrics, agent-cost/latency views).

### D1. Munzner's Nested Model

- **Q:** Where should visualization design start — and how do you avoid solving the wrong problem?
- **When:** Designing any non-trivial data display; also the meta-method this skill uses.
- **Apply:** Work top-down through four nested layers: **domain problem →
  data/task abstraction → visual encoding + interaction idiom → algorithm.** Decide the
  _task_ (lookup / compare / summarize / browse / locate / explore / monitor / diagnose /
  reconstruct-provenance) before picking a chart. A mismatch at an outer layer invalidates
  everything inside it.
- **Check:** "Is each view justified by a named user task, not chosen chart-first?"
- **Source:** Munzner, "A Nested Model for Visualization Design and Validation" (2009);
  _Visualization Analysis and Design_.

### D2. Graphical Perception (encoding-accuracy ranking)

- **Q:** Which visual encodings are read most accurately?
- **When:** Any quantitative display.
- **Apply:** Prefer encodings people decode accurately: **position on a common scale >
  position on non-aligned scales > length > angle/slope > area > volume > color
  saturation/density.** Use position/length for values that must be compared precisely;
  reserve color/area for categorical or approximate signals.
- **Check:** "Are precise comparisons encoded by position/length rather than area/angle/
  color?", "Do pie/donut charts stand in for comparisons better shown as bars?"
- **Source:** Cleveland & McGill (1984); Heer & Bostock replications.

### D3. Grammar of Graphics / Bertin's visual variables

- **Q:** What is the systematic vocabulary for building a chart?
- **When:** Designing custom or non-standard visualizations.
- **Apply:** Compose a graphic from independent layers — data → mapping of variables to
  **visual channels** (Bertin: position, size, shape, value, hue, orientation, texture) →
  scales → geometry → guides. Match each channel to whether the variable is nominal,
  ordinal, or quantitative.
- **Check:** "Is each data variable mapped to a channel appropriate to its type
  (quantitative→position/size, nominal→hue/shape)?"
- **Source:** Wilkinson, _The Grammar of Graphics_; Bertin, _Semiology of Graphics_.

### D4. Heer & Shneiderman's Interactive Dynamics

- **Q:** What interaction capabilities does exploratory analysis need?
- **When:** Interactive analytics / exploratory tools (not static charts).
- **Apply:** Provide the taxonomy's capabilities as needed: **data & view specification**
  (visualize, filter, sort, derive); **view manipulation** (select, navigate, coordinate/
  link, organize); **process & provenance** (record history, annotate, share, guide).
  Coordinated multiple views + brushing-and-linking are the workhorses.
- **Check:** "Can users filter, sort, select, and see linked views update together?",
  "Is exploration history recoverable (undo/record/provenance)?"
- **Source:** Heer & Shneiderman, "Interactive Dynamics for Visual Analysis" (2012).

### D5. Tufte's principles

- **Q:** Is the display honest, dense, and free of clutter?
- **When:** Any chart or data-dense layout.
- **Apply:** Maximize the **data-ink ratio** (remove chartjunk, heavy gridlines, 3D,
  redundant decoration); use **small multiples** for comparison; **sparklines** for inline
  trends; layer & separate to manage density; never distort scale (start bars at zero).
- **Check:** "Is non-data ink minimized?", "Are axes/scales non-deceptive?", "Are repeated
  comparisons shown as small multiples?"
- **Source:** Tufte, _The Visual Display of Quantitative Information_.

---

## Family E — Human-AI & agent-specific UX

_Fires for: LLM / agent / chat / prompt / workflow-composer products._ The highest-
value family for the AEP audience (agent tooling, observability, autopilot consoles).

### E1. Gulf of Envisioning

- **Q:** Does the user know what the model can do, how to instruct it, and how to judge success?
- **When:** Any LLM/agent UI — prompt boxes, agent builders, workflow composers.
- **Apply:** Extend Norman's gulfs with a third: bridge the **Gulf of Envisioning** —
  surface **capabilities** (what the model/agent can do), **instruction** support (examples,
  templates, affordances for phrasing intent), and **evaluation** support (how to tell the
  output is good). Don't assume the user can envision the model's competence unaided.
- **Check:** "Does the UI communicate what the agent can/can't do?", "Does it help the user
  phrase intent (examples/templates)?", "Does it help the user judge if the result is correct?"
- **Source:** Subramonyam et al., "Bridging the Gulf of Envisioning" (CHI 2024).

### E2. Microsoft's 18 Guidelines for Human-AI Interaction

- **Q:** What are the validated rules for AI-feature UX across the interaction lifecycle?
- **When:** Any AI-powered feature.
- **Apply:** Apply the 18 across four phases — **initially** (make clear what the system can
  do and how well); **during interaction** (show contextually relevant info, match social
  norms, mitigate bias); **when wrong** (support efficient invocation/dismissal/correction,
  scope services when in doubt, explain why); **over time** (remember, learn from behavior,
  update cautiously, notify of changes, give feedback control).
- **Check:** "Can the user easily dismiss/correct/undo an AI action?", "Does the system set
  expectations about accuracy up front?"
- **Source:** Amershi et al., "Guidelines for Human-AI Interaction" (CHI 2019).

### E3. Google PAIR People + AI patterns

- **Q:** How do we design for appropriate reliance, feedback, and error recovery in AI?
- **When:** Consumer/prosumer AI features.
- **Apply:** Set expectations, explain the model's confidence and limits, design for
  feedback loops, and handle errors gracefully (calibrate how much the UI implies certainty).
- **Check:** "Does the UI express uncertainty/confidence honestly rather than implying
  false precision?"
- **Source:** Google People + AI Guidebook (PAIR).

### E4. Trust calibration / appropriate reliance

- **Q:** Is the user's trust matched to the system's actual reliability?
- **When:** Agents/models whose output the user must accept, reject, or verify.
- **Apply:** Avoid over-trust (blind acceptance) and under-trust (ignoring good output).
  Show confidence signals, sources/citations, and easy verification paths; make failure
  modes legible so reliance tracks reality.
- **Check:** "Can the user cheaply verify a claim/action before relying on it?", "Are
  low-confidence outputs flagged?"
- **Source:** Lee & See, "Trust in Automation" (2004); HCAI literature.

### E5. Observability & progressive disclosure of agent reasoning

- **Q:** Can the user see what the agent did and why, at the right level of detail?
- **When:** Agent runs, tool-call chains, autopilot/observability consoles.
- **Apply:** Summarize first (status, cost, outcome, last tool call), expand on demand into
  steps/tool-calls/logs. Don't dump the full transcript by default (Cognitive Load) and don't
  hide it entirely (Gulf of Evaluation). Make provenance reconstructable.
- **Check:** "Is a run summarized before its full log?", "Can the user drill from summary →
  steps → raw tool I/O?"
- **Source:** synthesis of C1/B1 for agent traces; Heer & Shneiderman provenance (D4).

### E6. Steerability, interruptibility & reversibility (human-in-the-loop)

- **Q:** Can the user redirect, pause, approve, or undo the agent's actions — and predict
  each action's consequence?
- **When:** Autonomous/agentic systems that take actions (autopilot, workflow runners).
- **Apply:** Provide legible controls for **approve / pause / resume / rollback / retry /
  stop**, and make each control's effect predictable (Norman's Gulf of Execution for agents).
  Prefer reversible actions; gate irreversible ones behind confirmation.
- **Check:** "For pause/rollback/approve/retry: does the user know what each will do before
  clicking?", "Are irreversible agent actions gated?"
- **Source:** HCAI control principles; Shneiderman, _Human-Centered AI_.

### E7. Conversational UX / Grice's Maxims

- **Q:** Are the system's messages cooperative and well-formed?
- **When:** Chat, conversational agents, assistant copy, agent status messages.
- **Apply:** Follow Grice's cooperative maxims — **quantity** (enough, not too much),
  **quality** (truthful, no fabrication), **relation** (relevant), **manner** (clear,
  unambiguous, orderly). Design turn-taking, repair, and confirmation.
- **Check:** "Are messages appropriately concise, relevant, and honest about uncertainty?"
- **Source:** Grice, "Logic and Conversation" (1975); conversational-UX practice.

### E8. CASA (Computers Are Social Actors)

- **Q:** Do users apply social rules to the system, and does its persona respect them?
- **When:** Conversational/assistant/agent personas.
- **Apply:** Users unconsciously treat responsive systems as social actors — be consistent,
  polite, and honest about being a machine; avoid dark-pattern social pressure.
- **Check:** "Is the persona consistent and non-manipulative, and honest about being AI?"
- **Source:** Nass, Steuer & Tauber, "Computers are Social Actors" (1994).

---

## Family F — Evaluation & process methods

_Fires: always._ How to run the design review and score the health-check.

### F1. Heuristic Evaluation (the method)

- **Q:** How do you systematically find usability problems without users?
- **When:** Every health-check run.
- **Apply:** Inspect the interface against a set of heuristics (Family A as the default
  set, plus the selected families). Ideally 3–5 independent passes/evaluators; merge and
  dedupe findings. This skill's health-check table _is_ a heuristic evaluation.
- **Check:** (method) — produces the checklist itself.
- **Source:** Nielsen & Molich, "Heuristic Evaluation of User Interfaces" (1990).

### F2. Cognitive Walkthrough

- **Q:** Can a first-time user accomplish each task by exploration?
- **When:** Task-critical or onboarding flows.
- **Apply:** Pick the top tasks; step through each action asking: will the user try to
  achieve the right effect? notice the correct control is available? associate it with their
  goal? see progress after acting? Failures at any step are findings.
- **Check:** "For each top task, does every step pass the four walkthrough questions?"
- **Source:** Wharton et al., "The Cognitive Walkthrough Method" (1994).

### F3. Nielsen's 0–4 Severity Scale

- **Q:** How bad is each finding — what should be fixed first?
- **When:** Scoring every health-check row.
- **Apply:** Rate each issue: **0** = not a usability problem · **1** = cosmetic (fix if
  time) · **2** = minor · **3** = major (important to fix) · **4** = usability catastrophe
  (must fix). Severity combines frequency × impact × persistence. Rank the report by severity.
- **Check:** (scoring rule) — every health-check row carries a 0–4 severity.
- **Source:** Nielsen, "Severity Ratings for Usability Problems" (NN/g).

### F4. GOMS / Keystroke-Level Model

- **Q:** How efficient is a skilled user's path through a frequent task?
- **When:** High-frequency expert tasks (power tools, ops consoles) where speed matters.
- **Apply:** Estimate task time from operators (keystrokes, pointing, mental prep,
  system response). Use it to compare designs or justify shortcuts for frequent actions.
- **Check:** "For the most frequent task, is the operator/step count near-minimal (are
  there shortcuts for experts)?"
- **Source:** Card, Moran & Newell, _The Psychology of Human-Computer Interaction_ (1983).

### F5. Norman's Stages-of-Action as an evaluation lens

- **Q:** Where in the action cycle does a task break down?
- **When:** Diagnosing a specific failing interaction.
- **Apply:** Walk the seven stages (A3) for the failing task and locate the break — is it a
  Gulf of Execution problem (couldn't form/perform the action) or a Gulf of Evaluation
  problem (couldn't tell what happened)? The location dictates the fix.
- **Check:** (diagnostic) — classifies each failure as execution-side or evaluation-side.
- **Source:** Norman, _The Design of Everyday Things_.

---

## Family G — Accessibility & inclusive design

_Fires: always (any human-perceived UI, including terminal output)._ The other
families assume the user can perceive and operate the interface at all; this family
checks that precondition. Score failures with the same 0–4 scale — a
keyboard-unreachable primary action is a catastrophe for the user it excludes.

### G1. WCAG POUR principles

- **Q:** Is the interface Perceivable, Operable, Understandable, and Robust for users
  with differing abilities?
- **When:** Any human-facing UI. This is the accessibility baseline checklist.
- **Apply:** Design to the four principles: **Perceivable** (text alternatives,
  captions, content distinguishable), **Operable** (keyboard access, enough time, no
  seizure triggers, navigable), **Understandable** (readable, predictable, input
  assistance), **Robust** (works with assistive technologies — semantic markup,
  ARIA only where semantics fall short).
- **Check:** "Does every non-text element have a text alternative?", "Is every function
  reachable and operable without a pointer?", "Do form errors identify the field and
  suggest a fix?"
- **Source:** W3C, Web Content Accessibility Guidelines (WCAG) 2.2.

### G2. Keyboard & focus

- **Q:** Can the whole product be driven from the keyboard, with the focus always visible?
- **When:** Any UI with interactive controls; critical for expert/ops tools (overlaps F4).
- **Apply:** Every interactive element is focusable in a logical order; focus is clearly
  visible; no keyboard traps; provide skip links past repeated chrome; keep shortcuts
  discoverable and non-conflicting.
- **Check:** "Can a user complete the primary task keyboard-only?", "Is the focus
  indicator visible at every step?", "Does tab order match visual order?"
- **Source:** WCAG 2.2 (2.1 Keyboard Accessible, 2.4 Navigable); WAI-ARIA Authoring Practices.

### G3. Contrast, legibility & color independence

- **Q:** Can the content be read, and does meaning survive without color?
- **When:** All text, icons, states, charts — anywhere meaning is visually encoded.
- **Apply:** Meet contrast minima (≥4.5:1 body text, ≥3:1 large text/UI components);
  never encode a state by **color alone** — pair hue with shape, icon, or label
  (color-blind users, mono displays, terminals); keep text resizable without breakage.
- **Check:** "Do text and essential icons meet contrast ratios?", "Is every
  color-encoded state (error/success, chart series) also distinguishable by a
  non-color channel?"
- **Source:** WCAG 2.2 (1.4 Distinguishable); Okabe & Ito color-universal-design palette.

### G4. Inclusive design (the impairment spectrum)

- **Q:** Who is excluded by this design — permanently, temporarily, or situationally?
- **When:** Any product; strongest when "our users don't need accessibility" is claimed.
- **Apply:** Design for the spectrum: one-armed / arm-injury / carrying-a-child is one
  interaction profile; deaf / ear-infection / loud-bar is one auditory profile. Fixes for
  the permanent case (captions, large targets, keyboard paths) serve everyone in the
  situational case — accessibility work compounds into general usability.
- **Check:** "Does the primary task survive one-handed use, bright glare, and muted
  audio?", "Are 'accessibility' features (captions, zoom, keyboard paths) first-class
  rather than bolted on?"
- **Source:** Microsoft Inclusive Design toolkit; Holmes, _Mismatch_.

---

## Extending the catalog

To add a lens:

1. **Place it in the family that matches its selection trigger** (A/B/F/G always fire;
   C = info spaces; D = data viz; E = AI/agents). If it introduces a genuinely new
   trigger, add a family here _and_ a selection rule in `method-and-templates.md` — keep the
   two in sync.
2. **Use the entry shape** — `Q / When / Apply / Check / Source`. The **Check** line is
   load-bearing: it must be phrasable as a yes/no the health-check can score 0–4.
3. **Keep it established.** This catalog is for durable, citable results, not trend takes.
   Cite the canonical source.
4. **One lens = one idea.** Split compound theories (as Norman's principles vs gulfs vs
   stages were kept in one entry only because they are one author's single framework).

Candidate lenses noted for future addition (not yet written up): Emotional Design
(visceral/behavioral/reflective), Distributed Cognition, Activity Theory, Diátaxis (for
doc-heavy products), Dark-Patterns / ethical-design checklist.
