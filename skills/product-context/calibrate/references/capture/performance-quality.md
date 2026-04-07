# Performance/Quality — Capture Questions

Ask these questions one at a time during `/calibrate capture` for performance-quality.

## Questions

1. **Latency thresholds:** For each key user action, what's the maximum acceptable latency? (e.g., "page load < 2s", "API response < 200ms", "search results < 500ms")
2. **Retry policy:** For each external dependency, should failures retry? How many times? With what backoff? (exponential, fixed, none)
3. **Caching strategy:** What should be cached? For how long? What's the staleness tolerance? (e.g., "user profile: 5 min", "dashboard data: 30s", "config: until restart")
4. **Degradation behavior:** When [service X] is down, should the feature: degrade gracefully (show stale data), show an error, or block entirely?
5. **Cost ceilings:** What's the acceptable cost per [unit]? (per request, per user/month, total monthly)
6. **Sync vs async:** Which operations must complete before the user sees a response? Which can happen in the background?
7. **SLA commitments:** What uptime/performance guarantees do you plan to make? (or "none for MVP")

## Output

Update `product.success_criteria.non_functional` and `product.failure_model` in `product-context.yaml` with calibrated thresholds, retry policies, and degradation behaviors.
