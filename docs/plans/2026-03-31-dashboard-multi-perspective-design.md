# Dashboard Multi-Perspective Design

## Problem

The current dashboard has a single story map view (module × layer grid). Users need multiple perspectives to understand progress: by execution slice, by status (kanban), and by layer drilldown.

## Design

### Page Structure

The `/story-map` page becomes a unified view with 4 tabbed perspectives and a persistent summary stats bar.

**Summary Stats Bar** (always visible above tabs):

- Total / completed / in-progress / pending as compact pill stats
- Overall progress bar
- Dispatch epoch badge
- Layer gate status indicators

**Tabs**: `Module × Layer` | `By Slice` | `By Status` | `By Layer`

Filters (status, layer, module) persist across tab switches.

### Tab Views

**Module × Layer** — Current Jeff Patton story map grid, refined with better card design, hover states, responsive columns.

**By Slice** — Horizontal swimlanes per execution slice. Each lane shows stories as cards with progress bar and completion count. Stories within a slice laid out horizontally (parallel execution). Dependency arrows between slices flow downward.

**By Status (Kanban)** — Columns: pending → ready → in_progress → in_review → completed. Column count badges. Cards show title, module chip, complexity, slice number.

**By Layer** — Accordion drilldown per layer. Shows name, theme, capabilities, gate status, progress bar, stories as compact list. Expanding reveals full story cards with dependencies.

### Visual Polish

- Card border-left accent color by status
- Tab transitions with fade animation
- Summary stats bar with frosted glass (`backdrop-blur`)
- Module shown as colored chip on cards
- Hover states with subtle scale transform

### Files to modify

- `apps/web/src/routes/story-map.tsx` — Add tabs, summary bar, render perspective components
- `apps/web/src/components/story-map/summary-stats.tsx` — New: stats bar
- `apps/web/src/components/story-map/slice-view.tsx` — New: slice swimlane perspective
- `apps/web/src/components/story-map/kanban-view.tsx` — New: status kanban perspective
- `apps/web/src/components/story-map/layer-view.tsx` — New: layer drilldown perspective
- `apps/web/src/components/story-map/story-card.tsx` — Refine card design
- `apps/web/src/components/story-map/story-map-grid.tsx` — Refine existing grid
