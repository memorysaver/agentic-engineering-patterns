import { readFileSync, watch, type FSWatcher } from "node:fs";
import yaml from "js-yaml";
import { ProductContextSchema, type ProductContext } from "./product-context-schema";

/**
 * Preprocess YAML to handle list items containing colons, @tags,
 * or braces that YAML parsers misinterpret as flow mappings/tags.
 */
function preprocessYaml(raw: string): string {
  const lines = raw.split("\n");
  const result: string[] = [];
  for (const line of lines) {
    const match = line.match(/^(\s+- )(.+)$/);
    if (match) {
      const prefix = match[1];
      const content = match[2];
      // Skip key-value pairs, already-quoted strings, block scalars
      if (
        /^[a-zA-Z_][a-zA-Z0-9_]*:/.test(content) ||
        content.startsWith('"') ||
        content.startsWith("'") ||
        content === "|" ||
        content === ">"
      ) {
        result.push(line);
      } else if (
        content.includes(":") ||
        content.startsWith("@") ||
        content.includes("{") ||
        content.includes("}")
      ) {
        result.push(`${prefix}"${content.replace(/\\/g, "\\\\").replace(/"/g, '\\"')}"`);
      } else {
        result.push(line);
      }
    } else {
      result.push(line);
    }
  }
  return result.join("\n");
}

let cached: ProductContext | null = null;
let cachedPath: string | null = null;
let watcher: FSWatcher | null = null;

function getFilePath(): string {
  return process.env.PRODUCT_CONTEXT_PATH || "./product-context.yaml";
}

export function loadProductContext(filePath?: string): ProductContext {
  const resolvedPath = filePath || getFilePath();

  if (cached && cachedPath === resolvedPath) {
    return cached;
  }

  const raw = readFileSync(resolvedPath, "utf-8");
  const preprocessed = preprocessYaml(raw);
  const parsed = yaml.load(preprocessed) as ProductContext;

  cached = parsed;
  cachedPath = resolvedPath;

  // Set up file watcher for dev mode
  if (watcher) {
    watcher.close();
  }
  try {
    watcher = watch(resolvedPath, () => {
      cached = null;
      cachedPath = null;
    });
  } catch {
    // Watcher not critical
  }

  return parsed;
}

export function invalidateCache() {
  cached = null;
  cachedPath = null;
}

// ─── Derived data helpers ──────────────────────────────────

export type StoryMapCard = {
  storyId: string;
  title: string;
  moduleId: string;
  activity?: string | null;
  layer: number;
  slice: number;
  status: string;
  complexity?: string;
  priority?: string;
  businessValue?: string;
  dependencies: string[];
  prUrl?: string | null;
  completedAt?: string | null;
  attemptCount: number;
};

export type StoryMapEdge = {
  from: string;
  to: string;
  fromStatus: string;
};

export type ActivityItem = {
  id: string;
  name: string;
  description?: string;
  order: number;
  layerIntroduced: number;
};

export type StoryMapData = {
  backbone: Array<{ id: string; name: string; package?: string; status?: string }>;
  activities: ActivityItem[];
  lanes: Array<{ id: string; name: string; theme?: string; capabilities?: string[] }>;
  cards: StoryMapCard[];
  edges: StoryMapEdge[];
  slices: Array<{ slice: number; theme: string; storyIds: string[] }>;
};

export function buildStoryMap(ctx: ProductContext): StoryMapData {
  const modules = ctx.architecture?.modules || [];
  const layers = ctx.product?.layers || [];
  const stories = ctx.stories || [];
  const rawActivities = (ctx.product as any)?.activities || [];

  const backbone = modules.map((m) => ({
    id: m.id || m.name || "",
    name: m.name || m.id || "",
    package: m.package,
    status: m.status,
  }));

  const activities: ActivityItem[] = rawActivities
    .map((a: any) => ({
      id: a.id,
      name: a.name,
      description: a.description,
      order: a.order ?? 0,
      layerIntroduced: a.layer_introduced ?? 0,
    }))
    .sort((a: ActivityItem, b: ActivityItem) => a.order - b.order);

  const lanes = layers.map((l) => ({
    id: l.id || `layer-${l.layer ?? 0}`,
    name: l.name,
    theme: l.theme,
    capabilities: l.capabilities,
  }));

  const cards: StoryMapCard[] = stories.map((s) => ({
    storyId: s.id,
    title: s.title,
    moduleId: s.module,
    activity: (s as any).activity ?? null,
    layer: s.layer,
    slice: s.slice,
    status: s.status,
    complexity: s.complexity ?? undefined,
    priority: s.priority ?? s.business_value ?? undefined,
    businessValue: s.business_value ?? undefined,
    dependencies: s.dependencies ?? [],
    prUrl: s.pr_url,
    completedAt: s.completed_at,
    attemptCount: s.attempt_count ?? 0,
  }));

  const edges: StoryMapEdge[] = [];
  for (const story of stories) {
    for (const dep of story.dependencies ?? []) {
      const depStory = stories.find((s) => s.id === dep);
      edges.push({
        from: dep,
        to: story.id,
        fromStatus: depStory?.status ?? "pending",
      });
    }
  }

  // Group stories by slice
  const sliceMap = new Map<number, string[]>();
  for (const story of stories) {
    const ids = sliceMap.get(story.slice) || [];
    ids.push(story.id);
    sliceMap.set(story.slice, ids);
  }
  const slices = Array.from(sliceMap.entries())
    .sort(([a], [b]) => a - b)
    .map(([slice, storyIds]) => ({
      slice,
      theme: `Slice ${slice}`,
      storyIds,
    }));

  // Try to get slice themes from execution_slices
  if (ctx.execution_slices) {
    for (const es of ctx.execution_slices) {
      const found = slices.find((s) => s.slice === es.slice);
      if (found) {
        found.theme = es.theme;
      }
    }
  }

  return { backbone, activities, lanes, cards, edges, slices };
}

export type ProgressData = {
  statusCounts: Record<string, number>;
  sliceProgress: Array<{
    slice: number;
    theme: string;
    total: number;
    completed: number;
    inProgress: number;
    statuses: Record<string, number>;
  }>;
  layerProgress: Array<{
    layer: number;
    name: string;
    total: number;
    completed: number;
    gateStatus?: string;
  }>;
  dispatchEpoch: number;
  totalStories: number;
  completedStories: number;
  cost: {
    totalUsd: number;
    byLayer: Record<string, number>;
    byModule: Record<string, number>;
  };
};

export function buildProgress(ctx: ProductContext): ProgressData {
  const stories = ctx.stories || [];
  const layers = ctx.product?.layers || [];
  const gates = ctx.layer_gates || [];

  // Global status counts
  const statusCounts: Record<string, number> = {};
  for (const s of stories) {
    statusCounts[s.status] = (statusCounts[s.status] || 0) + 1;
  }

  // By slice
  const sliceMap = new Map<number, typeof stories>();
  for (const s of stories) {
    const arr = sliceMap.get(s.slice) || [];
    arr.push(s);
    sliceMap.set(s.slice, arr);
  }
  const sliceProgress = Array.from(sliceMap.entries())
    .sort(([a], [b]) => a - b)
    .map(([slice, sliceStories]) => {
      const statuses: Record<string, number> = {};
      for (const s of sliceStories) {
        statuses[s.status] = (statuses[s.status] || 0) + 1;
      }
      return {
        slice,
        theme: `Slice ${slice}`,
        total: sliceStories.length,
        completed: sliceStories.filter((s) => s.status === "completed" || s.status === "done")
          .length,
        inProgress: sliceStories.filter((s) => s.status === "in_progress").length,
        statuses,
      };
    });

  // By layer
  const layerMap = new Map<number, typeof stories>();
  for (const s of stories) {
    const arr = layerMap.get(s.layer) || [];
    arr.push(s);
    layerMap.set(s.layer, arr);
  }
  const layerProgress = Array.from(layerMap.entries())
    .sort(([a], [b]) => a - b)
    .map(([layer, layerStories]) => {
      const layerDef = layers.find((l) => l.layer === layer || l.id === `layer-${layer}`);
      const gate = gates.find((g) => g.layer === layer);
      return {
        layer,
        name: layerDef?.name || `Layer ${layer}`,
        total: layerStories.length,
        completed: layerStories.filter((s) => s.status === "completed" || s.status === "done")
          .length,
        gateStatus: gate?.status,
      };
    });

  const completedStories = stories.filter(
    (s) => s.status === "completed" || s.status === "done",
  ).length;

  return {
    statusCounts,
    sliceProgress,
    layerProgress,
    dispatchEpoch: ctx.dispatch_epoch || 0,
    totalStories: stories.length,
    completedStories,
    cost: {
      totalUsd: ctx.cost?.total_usd || 0,
      byLayer: (ctx.cost?.by_layer as Record<string, number>) || {},
      byModule: (ctx.cost?.by_module as Record<string, number>) || {},
    },
  };
}

export type ModuleGraphData = {
  nodes: Array<{
    id: string;
    name: string;
    package?: string;
    status?: string;
    responsibility?: string;
    does?: string | string[];
    doesNot?: string | string[];
  }>;
  edges: Array<{
    from: string;
    to: string;
  }>;
};

export function buildModuleGraph(ctx: ProductContext): ModuleGraphData {
  const modules = ctx.architecture?.modules || [];

  const nodes = modules.map((m) => ({
    id: m.id || m.name || "",
    name: m.name || m.id || "",
    package: m.package,
    status: m.status,
    responsibility: m.responsibility,
    does: m.does,
    doesNot: m.does_not,
  }));

  const edges: Array<{ from: string; to: string }> = [];
  for (const m of modules) {
    const id = m.id || m.name || "";
    const deps = m.depends_on || [];
    for (const dep of deps) {
      edges.push({ from: id, to: dep });
    }
    // Also parse consumes for module references
    const consumes = m.consumes;
    if (Array.isArray(consumes)) {
      for (const c of consumes) {
        if (typeof c === "string") {
          // Extract module id from package reference like "@org/module" or "packages/module"
          const match = modules.find(
            (other) =>
              c.includes(other.id || "") ||
              c.includes(other.package || "") ||
              c.includes(other.name || ""),
          );
          if (match && (match.id || match.name) !== id) {
            const targetId = match.id || match.name || "";
            if (!edges.some((e) => e.from === id && e.to === targetId)) {
              edges.push({ from: id, to: targetId });
            }
          }
        }
      }
    }
  }

  return { nodes, edges };
}
