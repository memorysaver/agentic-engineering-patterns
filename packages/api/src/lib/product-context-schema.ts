import { z } from "zod";

// ─── Opportunity ───────────────────────────────────────────
const ProblemClusterSchema = z
  .object({
    id: z.string(),
    statement: z.string(),
    severity: z.enum(["high", "medium", "low"]),
    wedge: z.boolean().optional(),
    deferred_to: z.string().optional(),
  })
  .passthrough();

const RiskSchema = z
  .object({
    id: z.string(),
    description: z.string(),
    severity: z.enum(["high", "medium", "low"]),
    status: z.enum(["mitigated", "open", "acknowledged"]),
    mitigation: z.string().optional(),
  })
  .passthrough();

const OpportunitySchema = z
  .object({
    core_bet: z.string().optional(),
    bet: z.string().optional(),
    decision: z.enum(["proceed", "kill", "defer"]),
    validated: z.string().optional(),
    decided_at: z.string().optional(),
    target_user: z.record(z.string()).optional(),
    persona: z.string().optional(),
    problem_cluster: z.array(ProblemClusterSchema).optional(),
    advantage: z.array(z.string()).optional(),
    risks: z.array(RiskSchema).optional(),
    why_now: z.string().optional(),
    counter_arguments: z.array(z.string()).optional(),
    scale_of_impact: z.string().optional(),
    kill_criteria: z.array(z.string()).optional(),
  })
  .passthrough();

// ─── Product ───────────────────────────────────────────────
const LayerSchema = z
  .object({
    id: z.string().optional(),
    layer: z.number().optional(),
    name: z.string(),
    theme: z.string().optional(),
    capabilities: z.array(z.string()).optional(),
    user_can: z.string().optional(),
    verification: z.string().optional(),
  })
  .passthrough();

const ActivitySchema = z
  .object({
    id: z.string(),
    name: z.string(),
    description: z.string().optional(),
    order: z.number(),
    layer_introduced: z.number().optional().default(0),
  })
  .passthrough();

const ProductSchema = z
  .object({
    name: z.string().optional(),
    tagline: z.string().optional(),
    problem_statement: z.string().optional(),
    problem: z.string().optional(),
    jobs_to_be_done: z.array(z.string()).optional(),
    mvp_journey: z.string().optional(),
    ux_model: z.record(z.string()).optional(),
    layers: z.array(LayerSchema).optional(),
    activities: z.array(ActivitySchema).optional(),
    technical_constraints: z.record(z.unknown()).optional(),
    constraints: z.record(z.unknown()).optional(),
    persona: z.record(z.unknown()).optional(),
    mvp_boundary: z.record(z.unknown()).optional(),
    success_criteria: z.record(z.unknown()).optional(),
    open_questions: z.array(z.record(z.unknown())).optional(),
    decisions: z.array(z.record(z.unknown())).optional(),
    stress_test: z.unknown().optional(),
    references: z.record(z.unknown()).optional(),
  })
  .passthrough();

// ─── Architecture ──────────────────────────────────────────
const ModuleSchema = z
  .object({
    id: z.string().optional(),
    name: z.string().optional(),
    package: z.string().optional(),
    status: z.string().optional(),
    responsibility: z.string().optional(),
    does: z.union([z.string(), z.array(z.string())]).optional(),
    does_not: z.union([z.string(), z.array(z.string())]).optional(),
    exposes: z.union([z.string(), z.array(z.string())]).optional(),
    consumes: z.union([z.string(), z.array(z.string())]).optional(),
    owns: z.array(z.string()).optional(),
    depends_on: z.array(z.string()).optional(),
    technology: z.unknown().optional(),
    key_concepts: z.array(z.string()).optional(),
  })
  .passthrough();

const ArchitectureSchema = z
  .object({
    style: z.string().optional(),
    overview: z.string().optional(),
    modules: z.array(ModuleSchema).optional(),
    interfaces: z.unknown().optional(),
    data_flows: z.unknown().optional(),
    third_party: z.unknown().optional(),
    deployment: z.unknown().optional(),
    amendment_log: z.array(z.record(z.unknown())).optional(),
    adrs: z.array(z.record(z.unknown())).optional(),
    dependency_graph: z.string().optional(),
  })
  .passthrough();

// ─── Stories ───────────────────────────────────────────────
export const StoryStatusEnum = z.enum([
  "pending",
  "ready",
  "in_progress",
  "in_review",
  "completed",
  "blocked",
  "failed",
  "deferred",
  "done",
  "review",
]);
export type StoryStatus = z.infer<typeof StoryStatusEnum>;

const StorySchema = z
  .object({
    id: z.string(),
    title: z.string(),
    layer: z.number(),
    module: z.string(),
    activity: z.string().nullable().optional(),
    slice: z.number(),
    status: StoryStatusEnum,
    business_value: z.enum(["critical", "high", "medium", "low"]).optional(),
    priority: z.enum(["critical", "high", "medium", "low"]).optional(),
    complexity: z.enum(["S", "M", "L"]).optional(),
    dependencies: z.array(z.string()).optional().default([]),
    description: z
      .union([
        z.string(),
        z
          .object({
            what_changes: z.string().optional(),
            why: z.string().optional(),
          })
          .passthrough(),
      ])
      .optional(),
    acceptance_criteria: z.array(z.string()).optional().default([]),
    files_affected: z.array(z.string()).optional().default([]),
    interface_obligations: z.record(z.unknown()).optional(),
    technical_notes: z.string().nullable().optional(),
    verification: z.record(z.unknown()).optional(),
    // Dispatch tracking
    dispatch_score: z.number().nullable().optional(),
    on_critical_path: z.boolean().optional(),
    assigned_to: z.string().nullable().optional(),
    openspec_change: z.string().nullable().optional(),
    dispatched_at_epoch: z.number().nullable().optional(),
    attempt_count: z.number().optional().default(0),
    max_retries: z.number().optional(),
    cost_usd: z.number().nullable().optional(),
    started_at: z.string().nullable().optional(),
    completed_at: z.string().nullable().optional(),
    pr_url: z.string().nullable().optional(),
    failure_logs: z.array(z.record(z.unknown())).optional().default([]),
  })
  .passthrough();

// ─── Topology ──────────────────────────────────────────────
const TopologySchema = z
  .object({
    roles: z.array(z.record(z.unknown())).optional(),
    routing: z.record(z.unknown()).optional(),
    handoffs: z.array(z.record(z.unknown())).optional(),
    handoff_contracts: z.record(z.unknown()).optional(),
  })
  .passthrough();

// ─── Layer Gates ───────────────────────────────────────────
const LayerGateSchema = z
  .object({
    layer: z.number(),
    name: z.string().optional(),
    status: z.enum(["not_started", "running", "passed", "failed", "pending"]),
    test_definition: z.string().optional(),
    requires: z.array(z.string()).optional(),
    results: z.record(z.unknown()).optional(),
    completed_at: z.string().nullable().optional(),
  })
  .passthrough();

// ─── Cost ──────────────────────────────────────────────────
const CostSchema = z
  .object({
    total_usd: z.number().optional().default(0),
    by_layer: z.record(z.number()).optional(),
    by_module: z.record(z.number()).optional(),
    by_story: z.record(z.number()).optional(),
    estimates: z.record(z.unknown()).optional(),
    alerts: z.array(z.record(z.unknown())).optional(),
  })
  .passthrough();

// ─── Changelog ─────────────────────────────────────────────
const ChangelogEntrySchema = z
  .object({
    date: z.string(),
    type: z.string().optional(),
    author: z.string().optional(),
    summary: z.string(),
    sections_changed: z.array(z.string()).optional(),
    feedback: z.record(z.unknown()).optional(),
  })
  .passthrough();

// ─── Execution Slices ──────────────────────────────────────
const ExecutionSliceSchema = z
  .object({
    slice: z.number(),
    theme: z.string(),
    stories: z.array(z.string()),
    critical_path: z.string().optional(),
    parallel: z.boolean().optional(),
  })
  .passthrough();

// ─── Full Product Context ──────────────────────────────────
export const ProductContextSchema = z
  .object({
    schema: z.string().optional(),
    project: z.string().optional(),
    version: z.string().optional(),
    updated_at: z.string().optional(),
    dispatch_epoch: z.number().optional(),
    opportunity: OpportunitySchema.optional(),
    product: ProductSchema.optional(),
    architecture: ArchitectureSchema.optional(),
    stories: z.array(StorySchema).optional().default([]),
    topology: TopologySchema.optional(),
    layer_gates: z.array(LayerGateSchema).optional().default([]),
    cost: CostSchema.optional(),
    changelog: z.array(ChangelogEntrySchema).optional().default([]),
    execution_slices: z.array(ExecutionSliceSchema).optional(),
  })
  .passthrough();

export type ProductContext = z.infer<typeof ProductContextSchema>;
export type Story = z.infer<typeof StorySchema>;
export type Module = z.infer<typeof ModuleSchema>;
export type Layer = z.infer<typeof LayerSchema>;
export type Risk = z.infer<typeof RiskSchema>;
export type Activity = z.infer<typeof ActivitySchema>;
export type ChangelogEntry = z.infer<typeof ChangelogEntrySchema>;
export type LayerGate = z.infer<typeof LayerGateSchema>;
