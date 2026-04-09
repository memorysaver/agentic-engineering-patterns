import { existsSync, readFileSync, watch, type FSWatcher } from "node:fs";
import { dirname, join, resolve } from "node:path";
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
      const prefix = match[1]!;
      const content = match[2]!;
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

function parseYamlFile(filePath: string): any {
  const raw = readFileSync(filePath, "utf-8");
  const preprocessed = preprocessYaml(raw);
  return yaml.load(preprocessed);
}

// ─── Split-mode helpers ───────────────────────────────────

function resolveBasePath(filePath: string): string {
  return dirname(resolve(filePath));
}

function detectSplitMode(basePath: string): boolean {
  return existsSync(join(basePath, "product", "index.yaml"));
}

function loadCapabilityMaps(
  basePath: string,
  capabilities: Array<{ id: string; name: string; map_path?: string }>,
): Record<string, { frame?: any; map?: any }> {
  const maps: Record<string, { frame?: any; map?: any }> = {};

  for (const cap of capabilities) {
    if (!cap.map_path) continue;
    const mapDir = join(basePath, "product", cap.map_path);
    if (!existsSync(mapDir)) continue;

    const entry: { frame?: any; map?: any } = {};

    const framePath = join(mapDir, "frame.yaml");
    if (existsSync(framePath)) {
      try {
        entry.frame = parseYamlFile(framePath);
      } catch {
        // Non-critical — skip
      }
    }

    const mapPath = join(mapDir, "map.yaml");
    if (existsSync(mapPath)) {
      try {
        entry.map = parseYamlFile(mapPath);
      } catch {
        // Non-critical — skip
      }
    }

    maps[cap.id] = entry;
  }

  return maps;
}

function loadAndMergeSplitMode(basePath: string): ProductContext {
  // Load product definition (stable)
  const indexPath = join(basePath, "product", "index.yaml");
  const indexRaw = parseYamlFile(indexPath);

  // Load operational state
  const opsPath = join(basePath, "product-context.yaml");
  let opsRaw: any = {};
  if (existsSync(opsPath)) {
    opsRaw = parseYamlFile(opsPath);
  }

  // Merge: index.yaml provides stable definition, product-context.yaml provides operational state
  const merged: any = {
    // Metadata — prefer index.yaml
    schema: indexRaw?.schema || opsRaw?.schema,
    project: indexRaw?.project || opsRaw?.project,
    version: indexRaw?.version || opsRaw?.version,
    updated_at: opsRaw?.updated_at,
    dispatch_epoch: opsRaw?.dispatch_epoch,
    // Stable definition from index.yaml
    opportunity: indexRaw?.opportunity,
    personas: indexRaw?.personas,
    capabilities: indexRaw?.capabilities,
    product: indexRaw?.product,
    // Operational state from product-context.yaml
    calibration: opsRaw?.calibration,
    architecture: opsRaw?.architecture,
    stories: opsRaw?.stories || [],
    topology: opsRaw?.topology,
    layer_gates: opsRaw?.layer_gates || [],
    waves: opsRaw?.waves,
    cost: opsRaw?.cost,
    changelog: opsRaw?.changelog || [],
    execution_slices: opsRaw?.execution_slices,
  };

  // Load capability maps if capabilities exist
  const capabilities = merged.capabilities || [];
  if (capabilities.length > 0) {
    const capMaps = loadCapabilityMaps(basePath, capabilities);
    if (Object.keys(capMaps).length > 0) {
      merged.capability_maps = capMaps;

      // Merge capability map activities into product.activities (dedup by id)
      if (!merged.product) merged.product = {};
      const existingActivities = merged.product.activities || [];
      const existingIds = new Set(existingActivities.map((a: any) => a.id));

      for (const [, capMap] of Object.entries(capMaps) as [string, any][]) {
        const mapActivities = capMap.map?.activities || [];
        let order = existingActivities.length;
        for (const a of mapActivities) {
          if (!existingIds.has(a.id)) {
            existingActivities.push({
              id: a.id,
              name: a.verb_phrase || a.name || a.id,
              description: a.description,
              order: a.order ?? order++,
              layer_introduced: a.layer_introduced ?? 0,
            });
            existingIds.add(a.id);
          }
        }

        // Merge capability map layers into product.layers (dedup by layer number)
        const existingLayers = merged.product.layers || [];
        const existingLayerNums = new Set(existingLayers.map((l: any) => l.layer));
        const mapLayers = capMap.map?.layers || [];
        for (const l of mapLayers) {
          if (!existingLayerNums.has(l.layer)) {
            existingLayers.push({
              layer: l.layer,
              name: l.name,
              outcome_contract: l.outcome_hypothesis
                ? { hypothesis: l.outcome_hypothesis, success_metric: l.success_metric }
                : undefined,
            });
            existingLayerNums.add(l.layer);
          }
        }
        merged.product.layers = existingLayers;
      }

      merged.product.activities = existingActivities;
    }
  }

  // Validate through Zod
  try {
    return ProductContextSchema.parse(merged);
  } catch {
    return merged as ProductContext;
  }
}

// ─── Cache & watchers ─────────────────────────────────────

let cached: ProductContext | null = null;
let cachedKey: string | null = null;
let watchers: FSWatcher[] = [];

function getFilePath(): string {
  return process.env.PRODUCT_CONTEXT_PATH || "./product-context.yaml";
}

function clearWatchers() {
  for (const w of watchers) {
    try {
      w.close();
    } catch {
      /* ignore */
    }
  }
  watchers = [];
}

function invalidateAll() {
  cached = null;
  cachedKey = null;
}

function setupWatchers(paths: string[]) {
  clearWatchers();
  for (const p of paths) {
    try {
      if (existsSync(p)) {
        watchers.push(watch(p, invalidateAll));
      }
    } catch {
      // Watcher not critical
    }
  }
}

export function loadProductContext(filePath?: string): ProductContext {
  const resolvedPath = filePath || getFilePath();
  const basePath = resolveBasePath(resolvedPath);
  const isSplit = detectSplitMode(basePath);
  const cacheKey = isSplit ? `split:${basePath}` : `single:${resolvedPath}`;

  if (cached && cachedKey === cacheKey) {
    return cached;
  }

  let parsed: ProductContext;

  if (isSplit) {
    parsed = loadAndMergeSplitMode(basePath);
    // Watch both files + maps directory
    const watchPaths = [
      join(basePath, "product", "index.yaml"),
      join(basePath, "product-context.yaml"),
    ];
    // Also watch capability map files
    const capabilities = parsed.capabilities || [];
    for (const cap of capabilities) {
      if (cap.map_path) {
        const mapDir = join(basePath, "product", cap.map_path);
        watchPaths.push(join(mapDir, "map.yaml"));
        watchPaths.push(join(mapDir, "frame.yaml"));
      }
    }
    setupWatchers(watchPaths);
  } else {
    // Single-file mode (v1)
    const raw = readFileSync(resolvedPath, "utf-8");
    const preprocessed = preprocessYaml(raw);
    const rawParsed = yaml.load(preprocessed);
    try {
      parsed = ProductContextSchema.parse(rawParsed);
    } catch {
      parsed = rawParsed as ProductContext;
    }
    setupWatchers([resolvedPath]);
  }

  cached = parsed;
  cachedKey = cacheKey;
  return parsed;
}

export function invalidateCache() {
  invalidateAll();
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

export type CapabilityJourney = {
  capabilityId: string;
  capabilityName: string;
  activities: ActivityItem[];
  lanes: Array<{ id: string; name: string; theme?: string }>;
};

export type StoryMapData = {
  backbone: Array<{ id: string; name: string; package?: string; status?: string }>;
  activities: ActivityItem[];
  lanes: Array<{ id: string; name: string; theme?: string; capabilities?: string[] }>;
  cards: StoryMapCard[];
  edges: StoryMapEdge[];
  slices: Array<{ slice: number; theme: string; storyIds: string[] }>;
  capabilityJourneys?: CapabilityJourney[];
};

export function buildStoryMap(ctx: ProductContext): StoryMapData {
  const modules = ctx.architecture?.modules || [];
  const layers = ctx.product?.layers || [];
  const stories = ctx.stories || [];
  const rawActivities = ctx.product?.activities || [];

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
    activity: s.activity ?? null,
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

  // Build per-capability journey data
  let capabilityJourneys: CapabilityJourney[] | undefined;
  if (ctx.capability_maps && Object.keys(ctx.capability_maps).length > 0) {
    capabilityJourneys = [];
    const capabilities = ctx.capabilities || [];

    for (const cap of capabilities) {
      const capMap = ctx.capability_maps[cap.id] as { frame?: any; map?: any } | undefined;
      if (!capMap?.map) continue;

      const mapData = capMap.map;
      const capActivities: ActivityItem[] = (mapData.activities || [])
        .map((a: any, i: number) => ({
          id: a.id,
          name: a.verb_phrase || a.name || a.id,
          description: a.description,
          order: a.order ?? i,
          layerIntroduced: a.layer_introduced ?? 0,
        }))
        .sort((a: ActivityItem, b: ActivityItem) => a.order - b.order);

      const capLanes = (mapData.layers || [])
        .sort((a: any, b: any) => (a.layer ?? 0) - (b.layer ?? 0))
        .map((l: any) => ({
          id: `layer-${l.layer}`,
          name: l.name,
          theme: l.outcome_hypothesis,
        }));

      capabilityJourneys.push({
        capabilityId: cap.id,
        capabilityName: cap.name,
        activities: capActivities,
        lanes: capLanes,
      });
    }
  }

  return { backbone, activities, lanes, cards, edges, slices, capabilityJourneys };
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
      const execSlice = ctx.execution_slices?.find((es: any) => es.slice === slice);
      return {
        slice,
        theme: execSlice?.theme || `Slice ${slice}`,
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
