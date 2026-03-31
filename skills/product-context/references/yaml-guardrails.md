# YAML Guardrails for product-context.yaml

Every skill that writes to `product-context.yaml` must validate the file before committing. Invalid YAML silently breaks the dashboard and blocks all downstream consumers.

## Validation Command

Run this after every edit to `product-context.yaml`:

```bash
npx js-yaml product-context.yaml > /dev/null && echo "YAML OK"
```

If the project has the `@agentic-engineering-patterns/api` package, use the actual loader for deeper validation (Zod schema + preprocessing):

```bash
npx tsx -e "
  const { loadProductContext } = require('@agentic-engineering-patterns/api/lib/product-context-loader');
  loadProductContext(process.env.PRODUCT_CONTEXT_PATH || './product-context.yaml');
  console.log('YAML + schema OK');
"
```

**If validation fails, fix the YAML before committing.** Do not commit broken YAML under any circumstances.

## Common YAML Pitfalls in product-context.yaml

These are the patterns that most frequently break the parser when agents write to the file.

### 1. List items ending with a colon

A trailing colon makes YAML interpret the item as a mapping key. If the next lines are indented, YAML expects a value — and fails.

```yaml
# BROKEN — YAML treats this as a mapping key
acceptance_criteria:
  - Generate page redesigned for multi-step video workflow:
    - Intent prompt input
    - Multi-step progress display

# FIXED — quote the entire item, flatten sub-items
acceptance_criteria:
  - "Generate page redesigned for multi-step video workflow: intent prompt input, multi-step progress display"
```

**Rule:** Never end a list item with `:` followed by indented sub-items. Either quote the item or flatten the sub-list.

### 2. Embedded double quotes inside list items

YAML interprets `"text"` as a quoted string boundary. Content after the closing quote is invalid.

```yaml
# BROKEN — YAML sees "Complete Your Profile" as the full string, then chokes on the rest
- "Complete Your Profile" guard includes link to /profile

# FIXED — wrap in double quotes, use single quotes inside
- "'Complete Your Profile' guard includes link to /profile"

# ALSO FIXED — escape inner quotes
- "\"Complete Your Profile\" guard includes link to /profile"
```

**Rule:** If a list item contains embedded double quotes, wrap the entire value in double quotes and use single quotes (or escaped quotes) inside.

### 3. Colons in the middle of list items

A colon followed by a space (`: `) triggers YAML key-value parsing.

```yaml
# BROKEN — YAML tries to parse "Dashboard" as a key
- Dashboard: creator dashboard showing recent generations

# WORKS (preprocessor handles this) — but quoting is safer
- "Dashboard: creator dashboard showing recent generations"
```

**Rule:** The `preprocessYaml` function in the loader auto-quotes most of these, but when writing new content, prefer explicit quoting for items containing `: `.

### 4. Special characters: @, {, }

```yaml
# BROKEN — @ is a YAML tag indicator, { starts a flow mapping
- @mention the user
- Use {variable} interpolation

# FIXED
- "@mention the user"
- "Use {variable} interpolation"
```

**Rule:** Quote list items containing `@`, `{`, or `}`.

### 5. Nested sub-lists under string items

YAML list items are scalar values — they cannot have children unless the item is a mapping key.

```yaml
# BROKEN — a string item cannot have sub-items
- Main feature description
  - Sub-feature A
  - Sub-feature B

# FIXED — flatten into one item or use a mapping structure
- "Main feature description: Sub-feature A, Sub-feature B"
```

## Pre-commit Checklist

Before committing any change to `product-context.yaml`:

1. Run the validation command above
2. If adding `acceptance_criteria`, `description`, or any free-text list: scan for colons, quotes, and special characters
3. If the validation command is not available (e.g., no Node.js), at minimum review list items for the patterns above
