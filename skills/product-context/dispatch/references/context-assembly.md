# Context-Package Assembly

Consulted by `/aep-dispatch` Step 6. For each dispatched story, assemble the `.context/` handoff package written into the OpenSpec change:

```
openspec/changes/<story-id>/
├── proposal.md          ← story description + why + business value
├── design.md            ← module definition + interface contracts + dependency APIs
├── specs/<module>.md    ← acceptance criteria + interface obligations + verification
├── tasks.md             ← story decomposed into implementation tasks
└── .context/            ← pre-assembled context package
    ├── stable-prefix.md ← shared product/architecture context (cacheable)
    ├── dependencies.md  ← public APIs from completed dependency stories
    └── retrieval.md     ← what to explore at runtime
```

Prune the assembled package to the role's token budget from topology (see Assembly Rules).

---

## Part 1: Stable Prefix (~10K tokens, shared across agents in same layer)

Extracted from product definition (`product/index.yaml` in split mode, `product-context.yaml` in v1 mode):

- `product.problem` — what we're solving
- `product.constraints` — tech stack, infrastructure
- `product.layers[active_layer]` — what the user can do at this layer
- `architecture.overview` — high-level structure
- `architecture.technical_spec` — if set, include the technical specification document (or relevant sections for the story's module). This provides Symphony-style precision for protocol-heavy systems.
- Coding conventions (conventional commits, git + worktree workflow, trunk-based)

## Part 2: Story-Specific Payload (~20K tokens, unique per agent)

- **Full story spec** from the `stories` section
- **Module definition** from `architecture.modules` matching `story.module`
- **Adjacent interfaces** from `architecture.interfaces` where `from` or `to` = story module
- **Dependency outputs** — for each completed dependency: public API surface (types, exports, endpoints). NOT internal implementation.

## Part 3: Retrieval Instructions (~500 tokens)

```markdown
## Files to read first

- <files_affected from story spec>

## Patterns to explore

- Check existing patterns in <module> directory
- Read interface contract tests for consumed interfaces

## Do not read

- Other module internals — use dependency_outputs above
```

---

## Calibration Context (for `.5` alignment layers and calibrated stories)

For stories with `calibration_type` set, or stories in `.5` alignment layers:

**Heavy calibrations** (visual-design, ux-flow, copy-tone):

1. **Include the calibration artifact** — `calibration/<type>.yaml` (e.g., `calibration/visual-design.yaml`)
2. **For visual-design:** Also include reference design files from `docs/design-references/` matching the story's page (by story activity or title)
3. **Include calibration constraint directive:**

   ```markdown
   This story has calibrated <dimension> decisions.
   Follow the calibration artifact in calibration/<type>.yaml strictly.
   Do not introduce new [visual tokens / flow patterns / voice patterns]
   not defined in the calibration artifact.
   ```

**Gate:** If the required `calibration/<type>.yaml` does not exist, **do not dispatch** — instruct the user to run `/aep-calibrate <type>` first.

**Light calibrations** (api-surface, data-model, scope-direction, performance-quality):

No additional context needed — decisions are already in the architecture section of `product-context.yaml` and the product section of `product/index.yaml` (split mode), which flow through the stable prefix (Part 1) and story-specific payload (Part 2).

**Backward compatibility:** For `.5` layer stories without `calibration_type` set, default to visual-design. Check both `calibration/visual-design.yaml` and `design-context.yaml` (legacy path).

---

## Object Map Context (UI-facing stories)

A story is **UI-facing** when it has `object_model_refs` set, `calibration_type` in
{visual-design, ux-flow}, or a non-null `activity` whose module has `kind: ui`
(`architecture.modules[].kind`). For these, inject the **Object Map slice**, not the
whole model:

1. **Resolve the capability:** use `story.capability` if set; otherwise (v1 /
   single-journey) the default capability is the project slug. The Object Map is
   `product/maps/<capability>/object-map.yaml`.
2. **Gate (must pass to dispatch):** the resolved object-map must exist, have
   `status: approved`, and its `coverage[]` must list this story id. If it is missing,
   `status` is `draft` or `stale`, or it does not cover the story → **do not
   dispatch**; instruct the user to run `/aep-model` first (same posture as the
   calibration gate). If `story.capability` is unset, fall back to scanning
   `product/maps/*/object-map.yaml` for a `coverage[].story` match — an `approved`
   match resolves the capability.
3. From the map's `coverage` index, select only the objects this story realizes and
   include just those entries: their attributes (core/secondary/metadata), the
   relationships among them, the CTAs (object × role) on them, and the screen(s) the
   story builds. Skip unrelated objects — keep the slice minimal.
4. Include the object-first directive:

   ```markdown
   This story realizes objects from an approved Object Map.
   Build object-first (noun→verb): the listed screens, object cards/detail,
   attributes, and CTA placements come from product/maps/<capability>/object-map.yaml.
   Do not introduce objects or screen structures not in this slice, and do not
   collapse the flow into a step-by-step wizard unless the map marks it task_oriented.
   ```

Visual look, copy voice, and journey/transition still come from
`calibration/{visual-design,copy-tone,ux-flow}.yaml` — the Object Map governs object
structure and CTA grammar, not taste.

---

## Assembly Rules

1. **Dependency outputs = public API only** — types, exports, endpoint signatures, never internals.
2. **Measure the package** — if it exceeds the role's token budget from topology, prune harder or split the story.
3. **Stable prefix is cacheable** — when dispatching multiple stories in the same layer, write it once.
