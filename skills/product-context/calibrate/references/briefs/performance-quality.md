# Performance/Quality Brief: [project name]

## Product Identity

- **What:** [from opportunity.bet + product.problem]
- **For whom:** [from product.persona.description]
- **Infrastructure:** [from product.constraints.infrastructure]

## Current Tolerances

[from product.success_criteria.non_functional]

| Metric                  | Current Target  | Source                          |
| ----------------------- | --------------- | ------------------------------- |
| [e.g., API p95 latency] | [e.g., < 200ms] | [success_criteria or undefined] |

## Current Failure Model

[from product.failure_model]

- **Failure classes:** [list from failure_model.classes]
- **Degraded operation:** [from failure_model.degraded_operation]

## Observed Behavior

[from /reflect observation, error logs, cost data, or user complaints]

- **Performance:** [what's slow, what's fast enough]
- **Reliability:** [what fails, how often, what recovers]
- **Resource usage:** [what's expensive, what's efficient]

## Questions for Calibration

1. Is [X]ms latency acceptable for [action]? What's the threshold where users notice?
2. Should [operation] retry on failure? How many times? With what backoff?
3. Should we cache [resource]? For how long? What's the staleness tolerance?
4. When [external service] is down, should the feature degrade gracefully or block entirely?
5. What's the cost ceiling per [unit]? (e.g., per API call, per user, per month)
6. Are there operations that must be synchronous vs. can be async?
7. What SLAs do we promise (or plan to promise) to users?

---

## Extraction Map

| Brief section  | Source                                    |
| -------------- | ----------------------------------------- |
| Tolerances     | `product.success_criteria.non_functional` |
| Failure model  | `product.failure_model`                   |
| External deps  | `product.constraints.external_deps`       |
| Cost data      | `cost` section (if populated)             |
| Infrastructure | `product.constraints.infrastructure`      |
