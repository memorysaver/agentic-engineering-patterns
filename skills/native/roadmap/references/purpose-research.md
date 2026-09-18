# Research for a product decision

Start with the user's purpose and the decision to be made. Identify the affected user, current workaround, desired observable improvement, constraints and alternatives, including keeping the current approach. State which unknown could change the recommendation. A retained candidate or a technically feasible idea needs evidence of its relevance before becoming a product priority.

Use existing project context, accepted decisions, actual usage and prior experiments to locate that gap. Check their scope and freshness: an old failure may have been fixed, and a planned experiment is still a hypothesis until it has results. Inspect focused sources and follow their links instead of treating a large context dump as proof of completeness.

Deepen research autonomously where the decision needs it. Trace consequential claims to original papers, documentation, datasets or implementation, retaining source links and revisions or observation dates. Examine methods, assumptions and counterevidence, including information that would favor another choice. Separate observed facts, inference, user preferences and untested hypotheses. Results from different systems can motivate an experiment without isolating the cause of their difference.

For benchmark selection, map measured capabilities to the product outcome and name the gaps needing direct product tests. Inspect data, scoring, failure handling, model/tool/context budgets, leakage risks, reproducibility and cost. Preserve historical results under their original definitions; a new evaluation contract describes future comparisons. A source commit used for research becomes an execution pin only when selected for that run.

Ask a concise question when human priorities materially change the choice; continue independent research while the answer is pending. Present a supported recommendation with its meaningful tradeoffs and invite feedback. Treat a tentative preference as a candidate, retaining unresolved choices explicitly. Reuse a clear direction already supplied by the user instead of repeating the same question.

Stop when the evidence supports a choice, rejection, deferral or a specific next experiment. If the missing evidence is behavior, use `aep --skill design --ref prototype` to obtain the smallest useful observation. Research effort follows remaining uncertainty and the task's scope, rather than a fixed source count, number of rounds or model topology. Missing sources or runtime access remain visible limits.

Retain the decision-relevant claims, provenance, alternatives, unknowns and next observable check in a concise research/design artifact. Once user feedback settles the direction, continue with `aep --skill roadmap --ref product-context` to update canonical context and hand off design under existing authorization. Finding sources is an intermediate result when the request includes refocusing the product or preparing the selected change.
