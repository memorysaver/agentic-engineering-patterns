import { z } from "zod";

// ─── Personas (v2 top-level) ──────────────────────────────
const PersonaSchema = z
  .object({
    id: z.string(),
    description: z.string(),
    jtbd: z.string().optional(),
  })
  .passthrough();

// ─── Capabilities (v2 top-level) ──────────────────────────
const CapabilitySchema = z
  .object({
    id: z.string(),
    name: z.string(),
    description: z.string().optional(),
    map_path: z.string().optional(),
    status: z.string().optional(),
    depends_on: z.array(z.string()).optional(),
  })
  .passthrough();

// ─── Capability Maps (from product/maps/) ─────────────────
const CapabilityFrameSchema = z
  .object({
    capability: z.string(),
    scope: z.string().optional(),
    primary_user: z.string().optional(),
    boundary: z.record(z.unknown()).optional(),
    outcome_contract: z.record(z.unknown()).optional(),
    out_of_scope: z.array(z.string()).optional(),
  })
  .passthrough();

const StoryStubSchema = z
  .object({
    id: z.string(),
    title: z.string(),
    layer: z.number().optional(),
    description: z.string().optional(),
    acceptance_criteria_sketch: z.array(z.string()).optional(),
    complexity: z.string().optional(),
    dependencies: z.array(z.string()).optional(),
  })
  .passthrough();

const CapabilityMapActivitySchema = z
  .object({
    id: z.string(),
    verb_phrase: z.string().optional(),
    name: z.string().optional(),
    description: z.string().optional(),
    layer_introduced: z.number().optional().default(0),
    order: z.number().optional(),
  })
  .passthrough();

const CapabilityMapLayerSchema = z
  .object({
    layer: z.number(),
    name: z.string(),
    outcome_hypothesis: z.string().optional(),
    success_metric: z.record(z.unknown()).optional(),
  })
  .passthrough();

const CapabilityMapSchema = z
  .object({
    capability: z.string(),
    activities: z.array(CapabilityMapActivitySchema).optional(),
    layers: z.array(CapabilityMapLayerSchema).optional(),
    stories: z.array(StoryStubSchema).optional(),
  })
  .passthrough();

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
    // v2 fields
    bet: z.string().optional(),
    why_now: z.string().optional(),
    counter_arguments: z.array(z.string()).optional(),
    scale_of_impact: z.string().optional(),
    kill_criteria: z.array(z.string()).optional(),
    decision: z.enum(["proceed", "kill", "defer"]),
    decided_at: z.string().optional(),
    // v1 compat fields
    core_bet: z.string().optional(),
    validated: z.string().optional(),
    target_user: z.record(z.string()).optional(),
    persona: z.string().optional(),
    problem_cluster: z.array(ProblemClusterSchema).optional(),
    advantage: z.array(z.string()).optional(),
    risks: z.array(RiskSchema).optional(),
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
    outcome_contract: z.record(z.unknown()).optional(),
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

const NonGoalSchema = z
  .object({
    statement: z.string(),
    reasoning: z.string().optional(),
  })
  .passthrough();

const MvpBoundarySchema = z
  .object({
    in_scope: z.array(z.string()).optional(),
    out_of_scope: z.array(z.string()).optional(),
    deferred: z.array(z.string()).optional(),
  })
  .passthrough();

const FailureClassSchema = z
  .object({
    name: z.string(),
    examples: z.string().optional(),
    detection: z.string().optional(),
    recovery: z.string().optional(),
    escalation: z.string().optional(),
  })
  .passthrough();

const FailureModelSchema = z
  .object({
    classes: z.array(FailureClassSchema).optional(),
    degraded_operation: z.string().optional(),
  })
  .passthrough();

const SecurityModelSchema = z
  .object({
    trust_boundaries: z.string().optional(),
    auth: z.string().optional(),
    secret_handling: z.string().optional(),
  })
  .passthrough();

const QualityDimensionSchema = z
  .object({
    dimension: z.string(),
    criticality: z.string().optional(),
    first_calibration_layer: z.number().optional(),
    rationale: z.string().optional(),
  })
  .passthrough();

const StressTestEntrySchema = z
  .object({
    challenge: z.string(),
    angle: z.string().optional(),
    resolution: z.string().optional(),
  })
  .passthrough();

const OpenQuestionSchema = z
  .object({
    question: z.string(),
    default_assumption: z.string().optional(),
    revisit_trigger: z.string().optional(),
  })
  .passthrough();

const DecisionSchema = z
  .object({
    decision: z.string(),
    reasoning: z.string().optional(),
    alternatives: z.array(z.string()).optional(),
  })
  .passthrough();

const SuccessCriteriaSchema = z
  .object({
    functional: z.array(z.string()).optional(),
    non_functional: z.array(z.string()).optional(),
  })
  .passthrough();

const ExternalDepSchema = z
  .object({
    name: z.string(),
    provides: z.string().optional(),
    failure_mode: z.string().optional(),
  })
  .passthrough();

const ConstraintsSchema = z
  .object({
    required_stack: z.record(z.unknown()).optional(),
    preferred_stack: z.record(z.unknown()).optional(),
    infrastructure: z.string().optional(),
    external_deps: z.array(ExternalDepSchema).optional(),
  })
  .passthrough();

const ProductSchema = z
  .object({
    name: z.string().optional(),
    tagline: z.string().optional(),
    problem_statement: z.string().optional(),
    problem: z.string().optional(),
    // v2 typed fields
    goals: z.array(z.string()).optional(),
    non_goals: z.array(NonGoalSchema).optional(),
    mvp_boundary: z.union([MvpBoundarySchema, z.record(z.unknown())]).optional(),
    constraints: z.union([ConstraintsSchema, z.record(z.unknown())]).optional(),
    failure_model: z.union([FailureModelSchema, z.record(z.unknown())]).optional(),
    security_model: z.union([SecurityModelSchema, z.record(z.unknown())]).optional(),
    quality_dimensions: z.array(QualityDimensionSchema).optional(),
    success_criteria: z.union([SuccessCriteriaSchema, z.record(z.unknown())]).optional(),
    open_questions: z.array(z.union([OpenQuestionSchema, z.record(z.unknown())])).optional(),
    decisions: z.array(z.union([DecisionSchema, z.record(z.unknown())])).optional(),
    stress_test: z.union([z.array(StressTestEntrySchema), z.unknown()]).optional(),
    // Shared fields
    layers: z.array(LayerSchema).optional(),
    activities: z.array(ActivitySchema).optional(),
    persona: z.record(z.unknown()).optional(),
    // v1 compat fields
    jobs_to_be_done: z.array(z.string()).optional(),
    mvp_journey: z.string().optional(),
    ux_model: z.record(z.string()).optional(),
    technical_constraints: z.record(z.unknown()).optional(),
    references: z.record(z.unknown()).optional(),
  })
  .passthrough();

// ─── Calibration ──────────────────────────────────────────
const CalibrationSchema = z
  .object({
    plan: z.array(z.record(z.unknown())).optional(),
    history: z.array(z.record(z.unknown())).optional(),
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

// ─── Waves ────────────────────────────────────────────────
const WaveSchema = z
  .object({
    layer: z.number(),
    wave: z.number().optional(),
    stories: z.array(z.string()).optional(),
    theme: z.string().optional(),
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
    // v2 top-level fields
    personas: z.array(PersonaSchema).optional(),
    capabilities: z.array(CapabilitySchema).optional(),
    calibration: CalibrationSchema.optional(),
    // Shared fields
    opportunity: OpportunitySchema.optional(),
    product: ProductSchema.optional(),
    architecture: ArchitectureSchema.optional(),
    stories: z.array(StorySchema).optional().default([]),
    topology: TopologySchema.optional(),
    layer_gates: z.array(LayerGateSchema).optional().default([]),
    waves: z.array(WaveSchema).optional(),
    cost: CostSchema.optional(),
    changelog: z.array(ChangelogEntrySchema).optional().default([]),
    execution_slices: z.array(ExecutionSliceSchema).optional(),
    // Populated by loader when capability maps are loaded
    capability_maps: z
      .record(
        z
          .object({
            frame: CapabilityFrameSchema.optional(),
            map: CapabilityMapSchema.optional(),
          })
          .passthrough(),
      )
      .optional(),
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
export type Persona = z.infer<typeof PersonaSchema>;
export type Capability = z.infer<typeof CapabilitySchema>;
export type CapabilityMap = z.infer<typeof CapabilityMapSchema>;
export type CapabilityFrame = z.infer<typeof CapabilityFrameSchema>;
export type QualityDimension = z.infer<typeof QualityDimensionSchema>;
export type Decision = z.infer<typeof DecisionSchema>;
export type NonGoal = z.infer<typeof NonGoalSchema>;
export type StressTestEntry = z.infer<typeof StressTestEntrySchema>;
export type OpenQuestion = z.infer<typeof OpenQuestionSchema>;
