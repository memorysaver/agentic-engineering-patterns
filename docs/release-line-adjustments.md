# Release Line Adjustments After Completing a Layer

When and how to adjust release lines (layer boundaries) based on what you learned. Grounded in Jeff Patton's User Story Mapping methodology.

---

## The Principle

Release lines are pencil marks, not commitments. They represent your current best guess about delivery order — and that guess improves every time you ship.

Patton's cycle:

```
Build layer → Learn from it → Adjust the map → Build next layer
                  │                    │
                  │                    ├── Re-slice (reflect)
                  │                    │   Move stories between layers,
                  │                    │   add stories to existing activities
                  │                    │
                  │                    └── Reshape (envision)
                  │                        Change the backbone, reframe
                  │                        the product, new activities
                  │
                  └── Most learning leads to re-slicing, not reshaping
```

The distinction matters because re-slicing is cheap (adjust priorities within a stable structure) while reshaping is expensive (may invalidate completed work, requires re-mapping).

---

## The Spectrum

```
                        ◄── handled in /reflect ──►  ◄── triggers /envision ──►

 ┌──────────┬───────────┬────────────┬──────────────┬─────────────┬────────────┐
 │ Fix bug  │ Promote a │ Add story  │ New activity  │ Reframe     │ Pivot the  │
 │ in       │ story to  │ to existing│ extends       │ persona or  │ opportunity│
 │ current  │ an earlier│ activity   │ backbone      │ JTBD        │ hypothesis │
 │ layer    │ layer     │            │               │             │            │
 └──────────┴───────────┴────────────┴──────────────┴─────────────┴────────────┘
  ◄─── low cost ──────────────────────────────────────── high cost ───────────►
  ◄─── frequent ──────────────────────────────────────── rare ────────────────►
```

Everything left of the boundary is a **release line adjustment** — you're re-slicing the same map. The backbone (user activities) and product framing stay intact.

Everything right of the boundary **changes the map itself** — new activities, different user journey, different persona. This requires `/envision` to revise the product framing before `/map` can re-decompose.

---

## Decision Table

| What you learned                                             | Route to                 | What changes in YAML                                             | Release line impact                                  |
| ------------------------------------------------------------ | ------------------------ | ---------------------------------------------------------------- | ---------------------------------------------------- |
| Feature X has a bug                                          | `/reflect` → `/dispatch` | New fix story in current layer, `priority: high`                 | None — current layer gets a patch                    |
| Feature Y needs polish                                       | `/reflect` → `/dispatch` | New refinement story in next layer                               | Next layer grows                                     |
| Story in Layer 2 is actually needed in Layer 1               | `/reflect`               | Story's `layer` field updated                                    | Release line shifts — Layer 1 grows, Layer 2 shrinks |
| New capability needed for existing activity                  | `/reflect`               | New story added to existing activity in appropriate layer        | Release line may shift                               |
| Users don't do Activity C before Activity D — order is wrong | `/envision` → `/map`     | `product.activities` reordered, stories re-mapped                | Backbone changes, layers may need full re-slice      |
| Persona is wrong — it's teams, not solo devs                 | `/envision`              | `product.persona`, `product.jtbd`, possibly `product.activities` | Backbone may change, all layers reassessed           |
| The problem we're solving isn't the real problem             | `/envision` Phase 0      | `opportunity` section revised                                    | Everything downstream may change                     |

---

## Examples

### Re-slicing (handled in `/reflect`)

**Scenario:** You shipped Layer 0 of a content creation tool. During user testing, you learned that the export feature (Layer 2) is more important than the collaboration feature (Layer 1) — users can't do anything useful without export.

**Action in reflect:**

1. Classify as **Refinement** — working behavior that needs re-prioritization
2. Move export stories from Layer 2 to Layer 1
3. Move collaboration stories from Layer 1 to Layer 2 (or keep them if Layer 1 capacity allows)
4. Update `product-context.yaml` story layer assignments
5. Proceed to `/dispatch` for the revised Layer 1

**Why this stays in reflect:** The backbone (Create → Edit → Export → Collaborate) hasn't changed. The activities are the same. You're just re-drawing where the release line cuts across them.

### Reshaping (triggers `/envision`)

**Scenario:** You shipped Layer 0 of the same tool. During user testing, you discovered that users don't want to "create" content from scratch — they want to "import" existing content and enhance it. "Create" should be "Import + Transform", which is a fundamentally different activity with different stories.

**Action in reflect:**

1. Classify as **Discovery (product)** — an assumption about the user journey was wrong
2. Route to `/envision` to revise the product framing
3. In envision: update the backbone — replace "Create" activity with "Import" and "Transform"
4. After envision: run `/map` to decompose the new activities into stories
5. Re-slice layers based on the new backbone

**Why this triggers envision:** The backbone changed. "Create" → "Import + Transform" isn't just a re-prioritization — it's a different user journey that produces different stories.

---

## Anti-Patterns

### Going to `/envision` just to move stories between layers

**Symptom:** After every layer completion, you re-run envision to "update the layered MVP contract."

**Problem:** Envision is for product framing — persona, JTBD, activity backbone. Using it to shuffle stories between layers adds ceremony without value and risks destabilizing the product framing that's already working.

**Fix:** Do release line adjustments in `/reflect`. Only go to envision when reflect classifies something as a Discovery or Opportunity Shift.

### Staying in `/reflect` when the backbone is clearly wrong

**Symptom:** You keep adding stories and moving things between layers, but the map feels increasingly incoherent. Stories don't cleanly map to activities. The user journey doesn't read as a sentence anymore.

**Problem:** The backbone itself is wrong — you're patching a structural problem with tactical adjustments. No amount of re-slicing fixes a broken map.

**Fix:** When you notice that stories are orphaned from activities or the journey narrative breaks down, classify as Discovery and route to `/envision`. Reshape the backbone, then re-map.

### Never adjusting release lines

**Symptom:** You treat the original layer assignments as fixed. Layer 1 contains exactly what was planned during `/map`, regardless of what you learned from Layer 0.

**Problem:** This defeats the purpose of iterative delivery. You're building a waterfall inside an agile wrapper.

**Fix:** Every `/reflect` cycle should explicitly ask: "Given what we learned, is the next layer still the right next layer?" Re-slice is the normal outcome, not the exception.

---

## References

- Jeff Patton, _User Story Mapping_ — Chapter 12: "Rock Breaking" (slicing strategies), Chapter 14: "Using Discovery to Build Shared Understanding" (learning loops)
- `/reflect` skill — Step 2.5 (Re-slice the Map) for the operational procedure
- `/envision` skill — "For Iteration" section for the boundary definition
- `templates/context-document.md` — Layered MVP Contract structure
