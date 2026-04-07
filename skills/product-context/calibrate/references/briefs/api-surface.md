# API Surface Brief: [project name]

## Product Identity

- **What:** [from opportunity.bet + product.problem]
- **For whom:** [from product.persona.description]
- **Consumers:** [who calls this API — internal frontend, external developers, other services]

## Current API Shape

[from architecture.interfaces, grouped by module boundary]

| Module   | Endpoint | Method   | Purpose        |
| -------- | -------- | -------- | -------------- |
| [module] | [path]   | [method] | [what it does] |

## Naming/Grouping Concerns

[from /reflect observation or ambient detection]

- **Naming inconsistency:** [examples — mixed conventions, unclear verbs, ambiguous nouns]
- **Grouping issue:** [examples — related endpoints spread across modules, unrelated endpoints grouped]
- **Error contract:** [current error shape vs. what consumers expect]

## Conventions Observed in Codebase

[scan existing route/controller files for patterns already in use]

- Naming convention: [e.g., kebab-case paths, camelCase params]
- Error format: [e.g., `{ error: { code, message } }` or `{ status, detail }`]
- Versioning: [current approach or none]

## Questions for Calibration

1. When you think about [resource], what verb do you reach for?
2. These endpoints do related things — should they be grouped under [X] or [Y]?
3. Error response: machine-readable codes, human-readable messages, or both?
4. Versioning: path-based (`/v1/`), header-based, or none for now?
5. Pagination: cursor-based, offset-based, or not needed?
6. Should any endpoints be renamed, merged, or split?

---

## Extraction Map

| Brief section     | Source                                         |
| ----------------- | ---------------------------------------------- |
| Current API shape | `architecture.interfaces`                      |
| Module boundaries | `architecture.modules`                         |
| Naming patterns   | File scan: route handler / controller files    |
| Consumer context  | `product.persona` + `architecture.third_party` |
