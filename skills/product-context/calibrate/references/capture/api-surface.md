# API Surface — Capture Questions

Ask these questions one at a time during `/calibrate capture` for api-surface.

## Questions

1. **Naming convention:** What convention should all endpoints follow? (e.g., kebab-case paths, camelCase body params, snake_case query params)
2. **Resource naming:** For each resource, what's the canonical name? (e.g., "session" vs "connection", "rule" vs "guardrail" vs "policy")
3. **Grouping:** Which endpoints should be grouped together? Under what path prefix?
4. **Error contract:** What should error responses look like? (shape, codes, messages, include stack traces in dev?)
5. **Versioning:** Path-based (`/v1/`), header-based, or none for now?
6. **Pagination:** Cursor-based, offset-based, or not needed? Default page size?
7. **Authentication:** How should API auth work? (Bearer token, API key, session cookie)

## Output

Update `architecture.interfaces` in `product-context.yaml` with calibrated naming, grouping, and error contracts.
