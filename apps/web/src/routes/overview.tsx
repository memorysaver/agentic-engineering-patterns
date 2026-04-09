import { createFileRoute } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import { orpc } from "@/utils/orpc";
import {
  Target,
  AlertTriangle,
  Layers,
  Briefcase,
  FileText,
  CheckCircle,
  XCircle,
  Activity,
  HelpCircle,
  Gauge,
  Shield,
  ServerCrash,
} from "lucide-react";
import { Section, StatusPill, SeverityDot, SubLabel } from "@/components/overview/section";
import { PersonasSection } from "@/components/overview/personas-section";
import { CapabilitiesSection } from "@/components/overview/capabilities-section";
import { MvpBoundarySection } from "@/components/overview/mvp-boundary-section";
import { ConstraintsSection } from "@/components/overview/constraints-section";
import { DecisionsSection } from "@/components/overview/decisions-section";
import { StressTestSection } from "@/components/overview/stress-test-section";

export const Route = createFileRoute("/overview")({
  component: OverviewPage,
});

function OverviewPage() {
  const { data, isLoading, error } = useQuery(orpc.productContext.getAll.queryOptions());

  if (isLoading) {
    return (
      <div className="flex h-full items-center justify-center">
        <div className="h-5 w-5 animate-spin rounded-full border-2 border-zinc-700 border-t-emerald-400" />
      </div>
    );
  }

  if (error || !data) {
    return (
      <div className="flex h-full items-center justify-center">
        <p className="text-sm text-red-400">Failed to load product context</p>
      </div>
    );
  }

  const { opportunity, product, personas, capabilities } = data as any;

  return (
    <div className="h-full overflow-auto bg-[#0a0a0b]">
      <div className="mx-auto max-w-4xl space-y-4 p-6">
        {/* 1. Header */}
        <div className="mb-6">
          <div className="flex items-center gap-3">
            <h2 className="text-base font-bold text-zinc-200">
              {product?.name || data.project || "Project"}
            </h2>
            {data.version && (
              <span className="rounded bg-zinc-800/80 border border-zinc-700/40 px-1.5 py-px text-[10px] font-medium text-zinc-500">
                v{data.version}
              </span>
            )}
          </div>
          {product?.tagline && <p className="text-xs text-zinc-500 mt-0.5">{product.tagline}</p>}
        </div>

        {/* 2. Opportunity */}
        {opportunity && <OpportunitySection opportunity={opportunity} />}

        {/* 3. Personas */}
        <PersonasSection personas={personas} legacyPersona={product?.persona} />

        {/* 4. Capabilities */}
        <CapabilitiesSection capabilities={capabilities} />

        {/* 5. Problem */}
        {product?.problem && (
          <Section icon={FileText} title="Problem" accent="#fb7185">
            <p className="text-[13px] leading-relaxed text-zinc-300">{product.problem}</p>
          </Section>
        )}

        {/* 6. Goals */}
        {product?.goals && product.goals.length > 0 && (
          <Section icon={CheckCircle} title="Goals" accent="#34d399">
            <div className="space-y-1.5">
              {product.goals.map((goal: string, i: number) => (
                <div key={i} className="flex gap-2">
                  <span className="text-[10px] font-bold text-emerald-600 mt-0.5">{i + 1}</span>
                  <p className="text-[12px] leading-relaxed text-zinc-400">{goal}</p>
                </div>
              ))}
            </div>
          </Section>
        )}

        {/* 7. Non-Goals */}
        {product?.non_goals && product.non_goals.length > 0 && (
          <Section icon={XCircle} title="Non-Goals" accent="#71717a">
            <div className="space-y-2">
              {product.non_goals.map((ng: any, i: number) => (
                <div key={i} className="rounded bg-zinc-900/50 px-3 py-2 space-y-0.5">
                  <p className="text-[12px] text-zinc-300">{ng.statement}</p>
                  {ng.reasoning && (
                    <p className="text-[11px] text-zinc-500 italic">{ng.reasoning}</p>
                  )}
                </div>
              ))}
            </div>
          </Section>
        )}

        {/* 8. MVP Boundary */}
        <MvpBoundarySection boundary={product?.mvp_boundary} />

        {/* 9. Constraints */}
        <ConstraintsSection constraints={product?.constraints} />

        {/* 10. Layers */}
        {product?.layers && product.layers.length > 0 && (
          <Section icon={Layers} title="Product Layers" accent="#38bdf8">
            <div className="space-y-3">
              {product.layers.map((layer: any, i: number) => {
                const LAYER_COLORS = ["#34d399", "#38bdf8", "#fbbf24", "#a78bfa"];
                const color = LAYER_COLORS[i % LAYER_COLORS.length];
                return (
                  <div key={layer.id || i}>
                    <div className="flex items-center gap-2 mb-1.5">
                      <span
                        className="flex h-5 w-5 items-center justify-center rounded text-[9px] font-bold"
                        style={{
                          backgroundColor: `${color}18`,
                          color,
                        }}
                      >
                        {layer.layer ?? i}
                      </span>
                      <span className="text-xs font-semibold text-zinc-300">{layer.name}</span>
                      {layer.theme && (
                        <span className="text-[10px] text-zinc-600 italic">{layer.theme}</span>
                      )}
                    </div>
                    <div className="ml-7 space-y-1">
                      {layer.user_can && (
                        <p className="text-[11px] text-zinc-400">
                          <span className="text-zinc-600 font-medium">User can:</span>{" "}
                          {layer.user_can}
                        </p>
                      )}
                      {layer.verification && (
                        <p className="text-[11px] text-zinc-500">
                          <span className="text-zinc-600 font-medium">Verify:</span>{" "}
                          {layer.verification}
                        </p>
                      )}
                      {layer.outcome_contract && (
                        <p className="text-[11px] text-zinc-500">
                          <span className="text-zinc-600 font-medium">Outcome:</span>{" "}
                          {typeof layer.outcome_contract === "object"
                            ? (layer.outcome_contract as any).hypothesis ||
                              JSON.stringify(layer.outcome_contract)
                            : String(layer.outcome_contract)}
                        </p>
                      )}
                      {layer.capabilities && (
                        <div className="flex flex-wrap gap-1 pt-0.5">
                          {layer.capabilities.map((cap: string, j: number) => (
                            <span
                              key={j}
                              className="rounded bg-zinc-900/60 border border-zinc-800/40 px-1.5 py-0.5 text-[10px] text-zinc-500"
                            >
                              {cap}
                            </span>
                          ))}
                        </div>
                      )}
                    </div>
                  </div>
                );
              })}
            </div>
          </Section>
        )}

        {/* 11. Success Criteria */}
        {product?.success_criteria &&
          (product.success_criteria.functional?.length ||
            product.success_criteria.non_functional?.length) && (
            <Section icon={Activity} title="Success Criteria" accent="#34d399">
              <div className="grid gap-3 sm:grid-cols-2">
                {product.success_criteria.functional?.length > 0 && (
                  <div className="space-y-1.5">
                    <SubLabel>Functional</SubLabel>
                    {product.success_criteria.functional.map((c: string, i: number) => (
                      <div key={i} className="flex items-start gap-2 text-[11px] text-zinc-400">
                        <span className="mt-1 inline-block h-1.5 w-1.5 shrink-0 rounded-full bg-emerald-400" />
                        <span>{c}</span>
                      </div>
                    ))}
                  </div>
                )}
                {product.success_criteria.non_functional?.length > 0 && (
                  <div className="space-y-1.5">
                    <SubLabel>Non-Functional</SubLabel>
                    {product.success_criteria.non_functional.map((c: string, i: number) => (
                      <div key={i} className="flex items-start gap-2 text-[11px] text-zinc-400">
                        <span className="mt-1 inline-block h-1.5 w-1.5 shrink-0 rounded-full bg-zinc-500" />
                        <span>{c}</span>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            </Section>
          )}

        {/* 12. Decisions */}
        <DecisionsSection decisions={product?.decisions} />

        {/* 13. Open Questions */}
        {product?.open_questions && product.open_questions.length > 0 && (
          <Section icon={HelpCircle} title="Open Questions" accent="#fbbf24">
            <div className="space-y-2">
              {product.open_questions.map((q: any, i: number) => (
                <div
                  key={i}
                  className="rounded-md border border-zinc-800/40 bg-zinc-900/50 px-3 py-2.5 space-y-1"
                >
                  <p className="text-[12px] font-medium text-zinc-300">{q.question}</p>
                  {q.default_assumption && (
                    <p className="text-[11px] text-zinc-500">
                      <span className="text-zinc-600 font-medium">Assumption:</span>{" "}
                      {q.default_assumption}
                    </p>
                  )}
                  {q.revisit_trigger && (
                    <p className="text-[11px] text-amber-400/60">
                      <span className="text-zinc-600 font-medium">Revisit:</span>{" "}
                      {q.revisit_trigger}
                    </p>
                  )}
                </div>
              ))}
            </div>
          </Section>
        )}

        {/* 14. Quality Dimensions */}
        {product?.quality_dimensions && product.quality_dimensions.length > 0 && (
          <Section icon={Gauge} title="Quality Dimensions" accent="#a78bfa">
            <div className="space-y-1.5">
              {product.quality_dimensions.map((qd: any, i: number) => (
                <div key={i} className="flex items-start gap-3 rounded bg-zinc-900/40 px-3 py-2">
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                      <span className="text-[12px] font-medium text-zinc-300">{qd.dimension}</span>
                      {qd.criticality && <CriticalityBadge criticality={qd.criticality} />}
                      {qd.first_calibration_layer != null && (
                        <span className="text-[9px] text-zinc-600">
                          layer {qd.first_calibration_layer}
                        </span>
                      )}
                    </div>
                    {qd.rationale && (
                      <p className="text-[11px] text-zinc-500 mt-0.5">{qd.rationale}</p>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </Section>
        )}

        {/* 15. Stress Test */}
        <StressTestSection stressTest={product?.stress_test} />

        {/* 16. Failure Model */}
        {product?.failure_model && <FailureModelSection model={product.failure_model} />}

        {/* 17. Security Model */}
        {product?.security_model && (
          <Section icon={Shield} title="Security Model" accent="#34d399">
            <div className="space-y-2">
              {product.security_model.trust_boundaries && (
                <div className="space-y-1">
                  <SubLabel>Trust Boundaries</SubLabel>
                  <p className="text-[11px] text-zinc-400">
                    {product.security_model.trust_boundaries}
                  </p>
                </div>
              )}
              {product.security_model.auth && (
                <div className="space-y-1">
                  <SubLabel>Authentication</SubLabel>
                  <p className="text-[11px] text-zinc-400">{product.security_model.auth}</p>
                </div>
              )}
              {product.security_model.secret_handling && (
                <div className="space-y-1">
                  <SubLabel>Secret Handling</SubLabel>
                  <p className="text-[11px] text-zinc-400">
                    {product.security_model.secret_handling}
                  </p>
                </div>
              )}
            </div>
          </Section>
        )}

        {/* 18. Risks (v1 compat) */}
        {opportunity?.risks && opportunity.risks.length > 0 && (
          <Section icon={AlertTriangle} title="Risks" accent="#f87171">
            <div className="space-y-1.5">
              {opportunity.risks.map((risk: any) => (
                <div
                  key={risk.id}
                  className="flex items-start gap-3 rounded bg-zinc-900/40 px-3 py-2"
                >
                  <SeverityDot severity={risk.severity} />
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                      <span className="font-mono text-[10px] text-zinc-500">{risk.id}</span>
                      <StatusPill status={risk.status} />
                    </div>
                    <p className="text-[11px] text-zinc-400 mt-0.5">{risk.description}</p>
                    {risk.mitigation && (
                      <p className="text-[10px] text-zinc-500 mt-1 italic">{risk.mitigation}</p>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </Section>
        )}

        {/* Jobs to be Done (v1 compat) */}
        {product?.jobs_to_be_done && product.jobs_to_be_done.length > 0 && (
          <Section icon={Briefcase} title="Jobs to be Done" accent="#a78bfa">
            <div className="space-y-2">
              {product.jobs_to_be_done.map((job: string, i: number) => (
                <div key={i} className="flex gap-2">
                  <span className="text-[10px] font-bold text-zinc-600 mt-0.5">{i + 1}</span>
                  <p className="text-[12px] leading-relaxed text-zinc-400">{job}</p>
                </div>
              ))}
            </div>
          </Section>
        )}
      </div>
    </div>
  );
}

/* ------------------------------------------------------------------ */
/*  Inline sub-components for simpler sections                        */
/* ------------------------------------------------------------------ */

function OpportunitySection({ opportunity }: { opportunity: any }) {
  const bet = opportunity.bet || opportunity.core_bet;
  if (!bet && !opportunity.problem_cluster?.length) return null;

  return (
    <Section icon={Target} title="Opportunity" accent="#34d399">
      <div className="space-y-3">
        {/* Bet + decision */}
        <div className="flex items-start justify-between gap-4">
          <p className="text-[13px] leading-relaxed text-zinc-300">{bet}</p>
          {opportunity.decision && (
            <div className="flex items-center gap-2 shrink-0">
              <StatusPill status={opportunity.decision} />
              {opportunity.decided_at && (
                <span className="text-[9px] text-zinc-600">{opportunity.decided_at}</span>
              )}
            </div>
          )}
        </div>

        {/* Why Now */}
        {opportunity.why_now && (
          <div className="rounded bg-emerald-900/20 border border-emerald-800/20 px-3 py-2">
            <SubLabel>Why Now</SubLabel>
            <p className="text-[11px] text-emerald-300/80 mt-0.5">{opportunity.why_now}</p>
          </div>
        )}

        {/* Scale of Impact */}
        {opportunity.scale_of_impact && (
          <div className="space-y-1">
            <SubLabel>Scale of Impact</SubLabel>
            <p className="text-[11px] text-zinc-400">{opportunity.scale_of_impact}</p>
          </div>
        )}

        {/* Counter Arguments */}
        {opportunity.counter_arguments && opportunity.counter_arguments.length > 0 && (
          <div className="space-y-1.5">
            <SubLabel>Counter Arguments</SubLabel>
            {opportunity.counter_arguments.map((arg: string, i: number) => (
              <div key={i} className="flex items-start gap-2 text-[11px] text-zinc-400">
                <span className="mt-1 inline-block h-1.5 w-1.5 shrink-0 rounded-full bg-amber-400" />
                <span>{arg}</span>
              </div>
            ))}
          </div>
        )}

        {/* Kill Criteria */}
        {opportunity.kill_criteria && opportunity.kill_criteria.length > 0 && (
          <div className="space-y-1.5">
            <SubLabel>Kill Criteria</SubLabel>
            {opportunity.kill_criteria.map((kc: string, i: number) => (
              <div key={i} className="flex items-start gap-2 text-[11px] text-red-400/70">
                <span className="mt-1 inline-block h-1.5 w-1.5 shrink-0 rounded-full bg-red-400" />
                <span>{kc}</span>
              </div>
            ))}
          </div>
        )}

        {/* v1 Target User */}
        {opportunity.target_user && typeof opportunity.target_user === "object" && (
          <div className="rounded bg-zinc-900/60 px-3 py-2">
            {Object.entries(opportunity.target_user).map(([k, v]) => (
              <div key={k} className="flex gap-2 text-[11px]">
                <span className="font-medium text-zinc-500 uppercase w-16 shrink-0">{k}</span>
                <span className="text-zinc-400">{String(v)}</span>
              </div>
            ))}
          </div>
        )}

        {/* v1 Problem Cluster */}
        {opportunity.problem_cluster && opportunity.problem_cluster.length > 0 && (
          <div className="space-y-1.5">
            <SubLabel>Problem Cluster</SubLabel>
            {opportunity.problem_cluster.map((p: any) => (
              <div key={p.id} className="flex items-start gap-2 rounded bg-zinc-900/40 px-3 py-2">
                <SeverityDot severity={p.severity} />
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <span className="font-mono text-[10px] text-zinc-500">{p.id}</span>
                    {p.wedge && (
                      <span className="rounded bg-amber-900/60 px-1 py-px text-[9px] font-bold text-amber-300">
                        WEDGE
                      </span>
                    )}
                    {p.deferred_to && (
                      <span className="text-[9px] text-zinc-600">
                        deferred {">"} {p.deferred_to}
                      </span>
                    )}
                  </div>
                  <p className="text-[11px] text-zinc-400 mt-0.5">{p.statement}</p>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </Section>
  );
}

function CriticalityBadge({ criticality }: { criticality: string }) {
  const styles: Record<string, string> = {
    critical: "bg-red-900/50 text-red-300 border-red-800/40",
    high: "bg-amber-900/50 text-amber-300 border-amber-800/40",
    medium: "bg-zinc-800/80 text-zinc-400 border-zinc-700/40",
    low: "bg-zinc-800/60 text-zinc-500 border-zinc-700/30",
  };
  return (
    <span
      className={`rounded border px-1.5 py-px text-[10px] font-medium ${styles[criticality] || "bg-zinc-800 text-zinc-400 border-zinc-700"}`}
    >
      {criticality}
    </span>
  );
}

function FailureModelSection({ model }: { model: any }) {
  const hasClasses = model.classes && model.classes.length > 0;
  if (!hasClasses && !model.degraded_operation) return null;

  return (
    <Section icon={ServerCrash} title="Failure Model" accent="#f87171">
      <div className="space-y-3">
        {hasClasses &&
          model.classes.map((cls: any, i: number) => (
            <div
              key={i}
              className="rounded-md border border-zinc-800/40 bg-zinc-900/50 px-3 py-2.5 space-y-1.5"
            >
              <p className="text-[12px] font-medium text-zinc-300">{cls.name}</p>
              {cls.examples && (
                <p className="text-[11px] text-zinc-500">
                  <span className="text-zinc-600 font-medium">Examples:</span>{" "}
                  {String(cls.examples)}
                </p>
              )}
              {cls.detection && (
                <p className="text-[11px] text-zinc-500">
                  <span className="text-zinc-600 font-medium">Detection:</span> {cls.detection}
                </p>
              )}
              {cls.recovery && (
                <p className="text-[11px] text-zinc-500">
                  <span className="text-zinc-600 font-medium">Recovery:</span> {cls.recovery}
                </p>
              )}
              {cls.escalation && (
                <p className="text-[11px] text-zinc-500">
                  <span className="text-zinc-600 font-medium">Escalation:</span> {cls.escalation}
                </p>
              )}
            </div>
          ))}
        {model.degraded_operation && (
          <div className="space-y-1">
            <SubLabel>Degraded Operation</SubLabel>
            <p className="text-[11px] text-zinc-400">{model.degraded_operation}</p>
          </div>
        )}
      </div>
    </Section>
  );
}
