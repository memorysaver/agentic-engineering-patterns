import { createFileRoute } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import { orpc } from "@/utils/orpc";
import { Shield, Target, AlertTriangle, Layers, Briefcase } from "lucide-react";

export const Route = createFileRoute("/overview")({
  component: OverviewPage,
});

function Section({
  icon: Icon,
  title,
  accent = "#71717a",
  children,
}: {
  icon: React.ComponentType<{ className?: string }>;
  title: string;
  accent?: string;
  children: React.ReactNode;
}) {
  return (
    <div
      className="rounded-lg border border-zinc-800/50 bg-zinc-900/30"
      style={{ borderLeftWidth: 3, borderLeftColor: accent }}
    >
      <div className="flex items-center gap-2 border-b border-zinc-800/30 px-4 py-2.5">
        <Icon className="h-3.5 w-3.5" style={{ color: accent }} />
        <span className="text-xs font-semibold tracking-wide text-zinc-300 uppercase">{title}</span>
      </div>
      <div className="px-4 py-3">{children}</div>
    </div>
  );
}

function SeverityDot({ severity }: { severity: string }) {
  const color =
    severity === "high" ? "bg-red-400" : severity === "medium" ? "bg-amber-400" : "bg-zinc-400";
  return <span className={`inline-block h-1.5 w-1.5 rounded-full ${color}`} />;
}

function StatusPill({ status }: { status: string }) {
  const styles: Record<string, string> = {
    mitigated: "bg-emerald-900/50 text-emerald-300 border-emerald-800/40",
    open: "bg-red-900/50 text-red-300 border-red-800/40",
    acknowledged: "bg-zinc-800/80 text-zinc-400 border-zinc-700/40",
    proceed: "bg-emerald-900/50 text-emerald-300 border-emerald-800/40",
    kill: "bg-red-900/50 text-red-300 border-red-800/40",
    defer: "bg-amber-900/50 text-amber-300 border-amber-800/40",
  };
  return (
    <span
      className={`rounded border px-1.5 py-px text-[10px] font-medium ${styles[status] || "bg-zinc-800 text-zinc-400 border-zinc-700"}`}
    >
      {status}
    </span>
  );
}

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

  const { opportunity, product } = data;

  return (
    <div className="h-full overflow-auto bg-[#0a0a0b]">
      <div className="mx-auto max-w-4xl space-y-4 p-6">
        {/* Header */}
        <div className="mb-6">
          <h2 className="text-base font-bold text-zinc-200">
            {product?.name || data.project || "Project"}
          </h2>
          <p className="text-xs text-zinc-500">{product?.tagline || ""}</p>
        </div>

        {/* Core Bet */}
        {opportunity && (
          <Section icon={Target} title="Opportunity" accent="#34d399">
            <div className="space-y-3">
              <div className="flex items-start justify-between gap-4">
                <p className="text-[13px] leading-relaxed text-zinc-300">
                  {opportunity.core_bet || opportunity.bet || ""}
                </p>
                <StatusPill status={opportunity.decision} />
              </div>

              {/* Target User */}
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

              {/* Problems */}
              {opportunity.problem_cluster && opportunity.problem_cluster.length > 0 && (
                <div className="space-y-1.5">
                  <p className="text-[10px] font-medium uppercase tracking-wider text-zinc-600">
                    Problem Cluster
                  </p>
                  {opportunity.problem_cluster.map((p: any) => (
                    <div
                      key={p.id}
                      className="flex items-start gap-2 rounded bg-zinc-900/40 px-3 py-2"
                    >
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
                              deferred → {p.deferred_to}
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
        )}

        {/* Risks */}
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

        {/* Product Layers */}
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
                        style={{ backgroundColor: `${color}18`, color }}
                      >
                        {layer.layer ?? i}
                      </span>
                      <span className="text-xs font-semibold text-zinc-300">{layer.name}</span>
                      {layer.theme && (
                        <span className="text-[10px] text-zinc-600 italic">{layer.theme}</span>
                      )}
                    </div>
                    {layer.capabilities && (
                      <div className="ml-7 flex flex-wrap gap-1">
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
                );
              })}
            </div>
          </Section>
        )}

        {/* Jobs to be Done */}
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
