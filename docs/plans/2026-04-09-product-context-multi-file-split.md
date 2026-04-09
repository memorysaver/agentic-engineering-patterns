# Product Context Multi-File Split — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Split product-context.yaml into two files — `product/index.yaml` (stable product definition) and `product-context.yaml` (operational state) — and update all 6 AEP skills to support split-mode with v1 fallback.

**Architecture:** Convention-based discovery: skills check `product/index.yaml` first; if absent, fall back to `product-context.yaml`. Each skill gets a File Resolution block at the top. No new dependencies or tooling — just YAML file reorganization and skill instruction updates.

**Tech Stack:** YAML, Markdown (skill definitions)

**Spec:** `docs/specs/2026-04-09-product-context-multi-file-split.md`

---

## File Map

**Modified:**

- `skills/product-context/envision/SKILL.md` — split-mode write targets
- `skills/product-context/map/SKILL.md` — split-mode read sources
- `skills/product-context/dispatch/SKILL.md` — split-mode context assembly
- `skills/product-context/calibrate/SKILL.md` — split-mode read/write
- `skills/product-context/reflect/SKILL.md` — split-mode cross-file updates
- `skills/product-context/validate/SKILL.md` — split-mode cross-file validation
- `skills/product-context/templates/product-context-schema.yaml` — add split-mode header comment

**No new files created** — all changes are edits to existing skill definitions.

---

### Task 1: Add File Resolution Convention to Schema Template

The schema template is the canonical reference. Add a comment block explaining split-mode at the top.

**Files:**

- Modify: `skills/product-context/templates/product-context-schema.yaml:1-20`

- [ ] **Step 1: Add split-mode documentation header**

At the top of `product-context-schema.yaml`, after the existing header comment block (lines 1-14), insert:

```yaml
# ─── SPLIT MODE (v2) ────────────────────────────────────────
# Products can operate in two modes:
#
# 1. Single-file (v1): All sections live in product-context.yaml.
#    This is the default for simple projects.
#
# 2. Split-mode (v2): Stable product definition lives in
#    product/index.yaml. Operational state lives in
#    product-context.yaml. Skills check product/index.yaml first;
#    if absent, fall back to product-context.yaml.
#
# In split mode, product-context.yaml omits `opportunity` and
# `product` sections. The `product/index.yaml` file uses the
# `personas` field (list) instead of `product.persona` (object).
#
# Discovery convention used by all skills:
#   product_def = product/index.yaml if exists, else product-context.yaml
#   operational = product-context.yaml
```

Insert this AFTER line 20 (after `dispatch_epoch: 0`), BEFORE the concurrency protocol comment.

- [ ] **Step 2: Verify YAML still parses**

```bash
cd /Users/memorysaver/Documents/github/agentic-engineering-patterns
python3 -c "import yaml; yaml.safe_load(open('skills/product-context/templates/product-context-schema.yaml')); print('OK')"
```

Expected: `OK`

- [ ] **Step 3: Commit**

```bash
git add skills/product-context/templates/product-context-schema.yaml
git commit -m "docs: add split-mode convention to product-context schema template"
```

---

### Task 2: Update `/envision` Skill

The most impactful change — envision now writes to `product/index.yaml` for product definition and `product-context.yaml` for operational sections only.

**Files:**

- Modify: `skills/product-context/envision/SKILL.md`

- [ ] **Step 1: Update the Output line (line 19)**

Replace:

```
**Output:** `product-context.yaml` with `opportunity` and `product` sections populated
```

With:

```
**Output:** `product/index.yaml` with `opportunity`, `personas`, `capabilities`, and `product` sections; `product-context.yaml` with `calibration` and `changelog` sections. In v1 fallback mode (no split), writes everything to `product-context.yaml`.
```

- [ ] **Step 2: Update the Before Starting section (lines 25-34)**

Replace the entire Before Starting section with:

```markdown
## Before Starting

Check which mode to operate in:

\`\`\`bash
ls product/index.yaml 2>/dev/null
ls product-context.yaml 2>/dev/null
\`\`\`

**File Resolution:**

- If `product/index.yaml` exists → **split mode**. Product definition lives in `product/index.yaml`, operational state in `product-context.yaml`.
- If only `product-context.yaml` exists → **v1 mode** OR **migration candidate**. Ask the user: "Do you want to migrate to split mode (product/index.yaml + product-context.yaml) or keep the single file?"
- If neither exists → **new project**. Default to split mode. Create `product/` directory.

In update mode, read the existing file(s) and ask whether they want to revise or start fresh. Preserve all sections you are not updating.
```

- [ ] **Step 3: Update the Phase 0 output section (around line 68)**

Replace:

```
Write the finalized Opportunity Brief to the `opportunity` section of `product-context.yaml`.
```

With:

```
**Split mode:** Write the finalized Opportunity Brief to the `opportunity` section of `product/index.yaml`.
**V1 mode:** Write to the `opportunity` section of `product-context.yaml`.
```

- [ ] **Step 4: Update the Phase 1 output section (around lines 126-144)**

Replace the block starting with "Write the finalized Context Document" through the capability maps section. The new text should be:

```markdown
**Split mode:**

1. Write the finalized Context Document to `product/index.yaml`:
   - `opportunity` (from Phase 0)
   - `personas` (extracted from the persona work — use list format with `id`, `description`, `jtbd`)
   - `capabilities` (at least one entry; single-journey products get one capability)
   - `product` subsection: `problem`, `goals`, `non_goals`, `mvp_boundary`, `constraints`, `layers`, `activities`, `failure_model`, `security_model`, `success_criteria`, `quality_dimensions`, `open_questions`, `decisions`, `stress_test`
2. Write operational initialization to `product-context.yaml`:
   - Header: `schema: v1`, `project`, `version`, `updated_at`, `dispatch_epoch: 0`
   - `calibration.plan` (mapped from quality_dimensions)
   - `calibration.history: []`
   - `changelog` entry recording what was created
   - All other operational sections left empty (populated by `/map`)

**V1 mode:** Write everything to `product-context.yaml` using `templates/product-context-schema.yaml` as the structural reference.

#### Capability Maps (for multi-journey products)

If the product has **2+ distinct user journeys**, also create capability map files:

1. Ensure `product/index.yaml` has multiple entries in `capabilities[]`
2. For each capability, create:
   - `product/maps/<capability-id>/frame.yaml` — scope, boundary, primary user, outcome contract
   - Story stubs are populated later by `/map`

Simple single-journey products get one capability entry but skip `frame.yaml` and `map.yaml`.
```

- [ ] **Step 5: Update the YAML validation command (around line 151)**

Replace:

```bash
npx js-yaml product-context.yaml > /dev/null && echo "YAML OK"
```

With:

```bash
# Split mode
python3 -c "import yaml; [yaml.safe_load(open(f)) for f in ('product/index.yaml', 'product-context.yaml')]; print('YAML OK')"
# V1 mode
python3 -c "import yaml; yaml.safe_load(open('product-context.yaml')); print('YAML OK')"
```

- [ ] **Step 6: Update the checklist items (around lines 159-171)**

Update any references to "Write product-context.yaml (opportunity + product sections)" to reflect the split:

```
# Split mode: Write product/index.yaml (opportunity + personas + capabilities + product)
# Split mode: Write product-context.yaml (calibration + changelog, operational sections empty)
# V1 mode: Write product-context.yaml (all sections)
```

- [ ] **Step 7: Commit**

```bash
git add skills/product-context/envision/SKILL.md
git commit -m "feat: update /envision for split-mode product-context"
```

---

### Task 3: Update `/map` Skill

Map reads product definition from the split source and writes operational sections to product-context.yaml.

**Files:**

- Modify: `skills/product-context/map/SKILL.md`

- [ ] **Step 1: Update Input/Output lines (lines 18-19)**

Replace:

```
**Input:** `product-context.yaml` (specifically the `opportunity` and `product` sections from `/envision`)
**Output:** `product-context.yaml` updated with `architecture`, `stories`, `waves`, `topology`, `layer_gates`, and `cost` sections
```

With:

```
**Input:** Product definition from `product/index.yaml` (split mode) or `product-context.yaml` (v1 mode)
**Output:** `product-context.yaml` updated with `architecture`, `stories`, `waves`, `topology`, `layer_gates`, `cost`, and `changelog` sections
```

- [ ] **Step 2: Update Before Starting section (lines 25-33)**

Replace the entire section with:

```markdown
## Before Starting

**File Resolution:**
\`\`\`bash
ls product/index.yaml 2>/dev/null && echo "SPLIT MODE" || echo "V1 MODE"
cat product-context.yaml
\`\`\`

- **Split mode** (`product/index.yaml` exists): Read product definition (opportunity, personas, product.\*) from `product/index.yaml`. Read operational state from `product-context.yaml`.
- **V1 mode**: Read everything from `product-context.yaml`.

If product definition is missing (no `product` section in either file), run `/envision` first.
```

- [ ] **Step 3: Update the calibration plan reference (around line 142)**

Replace:

```
After defining each implementation layer, review `calibration.plan` from `product-context.yaml`
```

With:

```
After defining each implementation layer, review `calibration.plan` from `product-context.yaml` (operational file, both modes)
```

- [ ] **Step 4: Update capability maps section (around lines 132-137)**

After the existing text about story stubs, add:

```markdown
> **Split mode note:** In split mode, the capability map's `map.yaml` story stubs are narrative sketches. The full stories are written to `product-context.yaml`, and `product/index.yaml` is NOT modified by `/map` (it only reads from it).
```

- [ ] **Step 5: Commit**

```bash
git add skills/product-context/map/SKILL.md
git commit -m "feat: update /map for split-mode product-context"
```

---

### Task 4: Update `/dispatch` Skill

Dispatch reads product definition from the split source for context assembly.

**Files:**

- Modify: `skills/product-context/dispatch/SKILL.md`

- [ ] **Step 1: Update Input line (line 20)**

Replace:

```
**Input:** `product-context.yaml` with stories
```

With:

```
**Input:** Product definition from `product/index.yaml` (split mode) or `product-context.yaml` (v1 mode); operational state from `product-context.yaml`
```

- [ ] **Step 2: Update Before Starting section (lines 27-34)**

Replace:

```markdown
## Before Starting

\`\`\`bash
cat product-context.yaml
\`\`\`

If `product-context.yaml` doesn't exist, run `/envision` then `/map` first.
If the `stories` section is empty, run `/map` to decompose the product.
```

With:

```markdown
## Before Starting

**File Resolution:**
\`\`\`bash
ls product/index.yaml 2>/dev/null && echo "SPLIT MODE" || echo "V1 MODE"
cat product-context.yaml
\`\`\`

- **Split mode** (`product/index.yaml` exists): Read product definition from `product/index.yaml` for context assembly. Read stories, topology, architecture, cost from `product-context.yaml`.
- **V1 mode**: Read everything from `product-context.yaml`.

If `product-context.yaml` doesn't exist, run `/envision` then `/map` first.
If the `stories` section is empty, run `/map` to decompose the product.
```

- [ ] **Step 3: Update context assembly section (around line 357)**

Find the line that says:

```
Extracted from `product-context.yaml`:
```

Replace with:

```
Extracted from product definition (`product/index.yaml` in split mode, `product-context.yaml` in v1 mode):
```

- [ ] **Step 4: Commit**

```bash
git add skills/product-context/dispatch/SKILL.md
git commit -m "feat: update /dispatch for split-mode product-context"
```

---

### Task 5: Update `/calibrate` Skill

Calibrate reads quality_dimensions from the product definition file, writes to both files depending on calibration type.

**Files:**

- Modify: `skills/product-context/calibrate/SKILL.md`

- [ ] **Step 1: Update Input line (line 21)**

Replace:

```
**Input:** Calibration type + `product-context.yaml`
```

With:

```
**Input:** Calibration type + product definition from `product/index.yaml` (split mode) or `product-context.yaml` (v1 mode) + operational state from `product-context.yaml`
```

- [ ] **Step 2: Update Type Detection Path C (line 36)**

Replace:

```
1. Check `calibration.plan` in `product-context.yaml` — which dimension is next for the current layer?
```

With:

```
1. Check `calibration.plan` in `product-context.yaml` (operational file, both modes) — which dimension is next for the current layer?
```

- [ ] **Step 3: Add File Resolution block after Type Detection, before Phase 1**

Insert after the Type Detection section (around line 39):

```markdown
## File Resolution

\`\`\`bash
ls product/index.yaml 2>/dev/null && echo "SPLIT MODE" || echo "V1 MODE"
cat product-context.yaml
\`\`\`

- **Split mode** (`product/index.yaml` exists): Read `quality_dimensions`, `layers`, `activities`, `constraints`, `success_criteria`, `failure_model` from `product/index.yaml`. Read `calibration.plan`, `calibration.history`, `stories`, `architecture` from `product-context.yaml`.
- **V1 mode**: Read everything from `product-context.yaml`.

**Write targets by calibration type:**

- **Heavy** (visual-design, ux-flow, copy-tone): Write `calibration/<type>.yaml`. Append `calibration.history` + `changelog` in `product-context.yaml`.
- **Light — architecture** (api-surface, data-model): Update `architecture.interfaces` or `architecture.domain_model` in `product-context.yaml`. Append `calibration.history` + `changelog`.
- **Light — product intent** (scope-direction, performance-quality): Update `product.goals`, `product.mvp_boundary`, `product.layers`, `product.success_criteria`, or `product.failure_model` in `product/index.yaml` (split mode) or `product-context.yaml` (v1 mode). Append `calibration.history` + `changelog` in `product-context.yaml`.
```

- [ ] **Step 4: Update Phase 2 write targets (around line 175)**

Replace:

```
Update the relevant section(s) of `product-context.yaml` directly:
```

With:

```
Update the relevant section(s) — see File Resolution above for which file to write to per calibration type:
```

- [ ] **Step 5: Commit**

```bash
git add skills/product-context/calibrate/SKILL.md
git commit -m "feat: update /calibrate for split-mode product-context"
```

---

### Task 6: Update `/reflect` Skill

Reflect reads both files and may write to both.

**Files:**

- Modify: `skills/product-context/reflect/SKILL.md`

- [ ] **Step 1: Update Output line (line 19)**

Replace:

```
**Output:** Classified feedback + updated `product-context.yaml`
```

With:

```
**Output:** Classified feedback + updated `product-context.yaml`; if product intent changed, also updated `product/index.yaml` (split mode)
```

- [ ] **Step 2: Update Before Starting section (lines 23-31)**

Replace:

```markdown
## Before Starting

Read the current product context:

\`\`\`bash
cat product-context.yaml
\`\`\`

If `product-context.yaml` does not exist, there is nothing to reflect on — run `/envision` and `/map` first.
```

With:

```markdown
## Before Starting

**File Resolution:**
\`\`\`bash
ls product/index.yaml 2>/dev/null && echo "SPLIT MODE" || echo "V1 MODE"
cat product-context.yaml
\`\`\`

- **Split mode** (`product/index.yaml` exists): Read product definition from `product/index.yaml` (what was intended). Read operational state from `product-context.yaml` (what happened).
- **V1 mode**: Read everything from `product-context.yaml`.

If `product-context.yaml` does not exist, there is nothing to reflect on — run `/envision` and `/map` first.
```

- [ ] **Step 3: Update Step 1 Gather Feedback (line 37)**

Replace:

```
Collect observations from all sources. Read from `product-context.yaml` to ground the conversation — review the `product` section for what was intended and the `cost` section for execution data.
```

With:

```
Collect observations from all sources. Read product definition (from `product/index.yaml` in split mode, `product-context.yaml` in v1 mode) for what was intended, and `product-context.yaml` `cost` section for execution data.
```

- [ ] **Step 4: Update the update instructions (around line 157)**

Replace:

```
Based on the classified feedback, update `product-context.yaml` directly:
```

With:

```
Based on the classified feedback, update the appropriate file:
- **Operational changes** (new stories, architecture amendments, topology, cost, changelog) → `product-context.yaml`
- **Product intent changes** (opportunity shift, persona change, goals, mvp_boundary, layers, activities) → `product/index.yaml` (split mode) or `product-context.yaml` (v1 mode)
```

- [ ] **Step 5: Commit**

```bash
git add skills/product-context/reflect/SKILL.md
git commit -m "feat: update /reflect for split-mode product-context"
```

---

### Task 7: Update `/validate` Skill

Validate reads from both files and checks cross-file consistency.

**Files:**

- Modify: `skills/product-context/validate/SKILL.md`

- [ ] **Step 1: Update Input line (line 37)**

Replace:

```
**Input:** Any AEP artifact (product-context.yaml, OpenSpec change, design doc, code)
```

With:

```
**Input:** Any AEP artifact (`product/index.yaml` + `product-context.yaml` in split mode, or `product-context.yaml` alone in v1 mode, or OpenSpec change, design doc, code)
```

- [ ] **Step 2: Update Before Starting section (around lines 42-55)**

After the existing `ls` checks, add a File Resolution block:

```markdown
**File Resolution:**
\`\`\`bash
ls product/index.yaml 2>/dev/null && echo "SPLIT MODE" || echo "V1 MODE"
\`\`\`

- **Split mode**: Validate both files. Check cross-file consistency.
- **V1 mode**: Validate `product-context.yaml` only.
```

- [ ] **Step 3: Find the product context validation section (around line 67)**

After the line:

```
**When:** After `/envision` or `/map` — validating `product-context.yaml`
```

Add:

```markdown
**Split-mode cross-file checks:**

- `stories[].layer` values must exist in `product/index.yaml` `product.layers[]`
- `stories[].activity` values must exist in `product/index.yaml` `product.activities[]`
- `calibration.plan[].dimensions[]` must reference `product/index.yaml` `product.quality_dimensions[]`
- No `opportunity` or `product` section should exist in `product-context.yaml` when split mode is active
- `product/index.yaml` must have `personas`, `capabilities`, and `product` sections
```

- [ ] **Step 4: Commit**

```bash
git add skills/product-context/validate/SKILL.md
git commit -m "feat: update /validate for split-mode product-context"
```

---

### Task 8: Sync Updated Skills to Looplia

After updating all source skills, sync to the downstream project.

**Files:**

- Modify: Multiple files in `/Users/memorysaver/Documents/github/looplia/.claude/skills/`

- [ ] **Step 1: Copy updated SKILL.md files**

```bash
SRC=/Users/memorysaver/Documents/github/agentic-engineering-patterns/skills
DST=/Users/memorysaver/Documents/github/looplia/.claude/skills

cp "$SRC/product-context/envision/SKILL.md" "$DST/aep-envision/SKILL.md"
cp "$SRC/product-context/map/SKILL.md" "$DST/aep-map/SKILL.md"
cp "$SRC/product-context/dispatch/SKILL.md" "$DST/aep-dispatch/SKILL.md"
cp "$SRC/product-context/calibrate/SKILL.md" "$DST/aep-calibrate/SKILL.md"
cp "$SRC/product-context/reflect/SKILL.md" "$DST/aep-reflect/SKILL.md"
cp "$SRC/product-context/validate/SKILL.md" "$DST/aep-validate/SKILL.md"
```

- [ ] **Step 2: Copy updated schema template to all skill dirs that carry it**

```bash
for dir in aep-envision aep-map aep-dispatch aep-calibrate aep-reflect aep-validate; do
  cp "$SRC/product-context/templates/product-context-schema.yaml" "$DST/$dir/templates/product-context-schema.yaml"
done
```

- [ ] **Step 3: Migrate looplia product-context.yaml (remove opportunity + product)**

Read `/Users/memorysaver/Documents/github/looplia/product-context.yaml` and remove the `opportunity` and `product` sections (they now live in `product/index.yaml`). Keep: `schema`, `project`, `version`, `updated_at`, `dispatch_epoch`, `calibration`, `architecture`, `stories`, `topology`, `layer_gates`, `waves`, `cost`, `changelog`.

- [ ] **Step 4: Expand looplia product/index.yaml**

Read `/Users/memorysaver/Documents/github/looplia/product/index.yaml` and ensure it has ALL fields from the current `product-context.yaml` opportunity + product sections. Specifically add any missing fields that were in product-context.yaml but not yet in index.yaml (failure_model, security_model, goals, non_goals, open_questions, decisions, stress_test, success_criteria, quality_dimensions, mvp_boundary, etc.).

- [ ] **Step 5: Validate the migration**

```bash
cd /Users/memorysaver/Documents/github/looplia
python3 -c "
import yaml
with open('product/index.yaml') as f:
    idx = yaml.safe_load(f)
with open('product-context.yaml') as f:
    pc = yaml.safe_load(f)

# No duplication
assert 'opportunity' not in pc, 'opportunity still in product-context.yaml'
assert 'product' not in pc, 'product still in product-context.yaml'

# Index has required fields
assert 'opportunity' in idx
assert 'personas' in idx
assert 'capabilities' in idx
assert 'product' in idx

# Operational file has required fields
assert 'stories' in pc
assert 'architecture' in pc
assert 'topology' in pc
assert 'waves' in pc

# Cross-file consistency
layers = {l['layer'] for l in idx['product']['layers']}
activities = {a['id'] for a in idx['product']['activities']}
for s in pc['stories']:
    assert s['layer'] in layers, f'Story {s[\"id\"]} references invalid layer {s[\"layer\"]}'
    if s.get('activity'):
        assert s['activity'] in activities, f'Story {s[\"id\"]} references invalid activity {s[\"activity\"]}'

print('Migration validated: no duplication, cross-file consistency OK')
"
```

- [ ] **Step 6: Diff-verify skill sync**

```bash
SRC=/Users/memorysaver/Documents/github/agentic-engineering-patterns/skills
DST=/Users/memorysaver/Documents/github/looplia/.claude/skills
for pair in "product-context/envision:aep-envision" "product-context/map:aep-map" "product-context/dispatch:aep-dispatch" "product-context/calibrate:aep-calibrate" "product-context/reflect:aep-reflect" "product-context/validate:aep-validate"; do
  src_dir="${pair%%:*}"; dst_dir="${pair##*:}"
  diff -q "$SRC/$src_dir/SKILL.md" "$DST/$dst_dir/SKILL.md" > /dev/null && echo "OK $dst_dir" || echo "MISMATCH $dst_dir"
done
```

Expected: All `OK`.

- [ ] **Step 7: Add changelog entry to looplia product-context.yaml**

Append to changelog:

```yaml
- date: "2026-04-09"
  type: architecture_review
  author: human
  summary: "Split product-context into product/index.yaml (stable definition) + product-context.yaml (operational state). Removed opportunity and product sections from product-context.yaml. Updated all AEP skills for split-mode with v1 fallback."
  sections_changed: [opportunity, product, calibration, architecture, stories, topology]
```

- [ ] **Step 8: Commit in looplia**

```bash
cd /Users/memorysaver/Documents/github/looplia
git add -A
git commit -m "feat: migrate to split-mode product-context (product/index.yaml + product-context.yaml)"
```
