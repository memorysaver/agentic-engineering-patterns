import { createFileRoute } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import { orpc } from "@/utils/orpc";
import { Badge } from "@agentic-engineering-patterns/ui/components/badge";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@agentic-engineering-patterns/ui/components/card";
import { Separator } from "@agentic-engineering-patterns/ui/components/separator";

export const Route = createFileRoute("/overview")({
  component: OverviewPage,
});

function OverviewPage() {
  const { data, isLoading, error } = useQuery(orpc.productContext.getAll.queryOptions());

  if (isLoading) {
    return (
      <div className="flex h-full items-center justify-center">
        <p className="text-muted-foreground">Loading...</p>
      </div>
    );
  }

  if (error || !data) {
    return (
      <div className="flex h-full items-center justify-center">
        <p className="text-destructive">Failed to load product context</p>
      </div>
    );
  }

  const { opportunity, product } = data;

  return (
    <div className="space-y-6 p-6">
      <div>
        <h2 className="text-lg font-semibold">Product Overview</h2>
        <p className="text-muted-foreground text-sm">
          {product?.name || data.project || "Unnamed project"} — {product?.tagline || ""}
        </p>
      </div>

      {/* Opportunity */}
      {opportunity && (
        <Card>
          <CardHeader>
            <div className="flex items-center justify-between">
              <CardTitle className="text-sm">Opportunity</CardTitle>
              <Badge variant={opportunity.decision === "proceed" ? "default" : "destructive"}>
                {opportunity.decision}
              </Badge>
            </div>
          </CardHeader>
          <CardContent className="space-y-3">
            {(opportunity.core_bet || opportunity.bet) && (
              <div>
                <p className="text-muted-foreground text-xs font-medium">Core Bet</p>
                <p className="text-sm">{opportunity.core_bet || opportunity.bet}</p>
              </div>
            )}

            {opportunity.target_user && (
              <div>
                <p className="text-muted-foreground text-xs font-medium">Target User</p>
                {typeof opportunity.target_user === "object" ? (
                  <dl className="mt-1 space-y-1">
                    {Object.entries(opportunity.target_user).map(([k, v]) => (
                      <div key={k} className="flex gap-2 text-xs">
                        <dt className="text-muted-foreground font-medium capitalize">{k}:</dt>
                        <dd>{String(v)}</dd>
                      </div>
                    ))}
                  </dl>
                ) : (
                  <p className="text-sm">{String(opportunity.target_user)}</p>
                )}
              </div>
            )}

            {opportunity.problem_cluster && opportunity.problem_cluster.length > 0 && (
              <div>
                <p className="text-muted-foreground text-xs font-medium">Problem Cluster</p>
                <div className="mt-1 space-y-2">
                  {opportunity.problem_cluster.map((p: any) => (
                    <div key={p.id} className="rounded border p-2">
                      <div className="flex items-center gap-2">
                        <span className="font-mono text-xs">{p.id}</span>
                        <Badge variant="outline" className="text-[10px]">
                          {p.severity}
                        </Badge>
                        {p.wedge && (
                          <Badge className="bg-amber-800 text-amber-200 text-[10px]">WEDGE</Badge>
                        )}
                        {p.deferred_to && (
                          <Badge variant="secondary" className="text-[10px]">
                            deferred → {p.deferred_to}
                          </Badge>
                        )}
                      </div>
                      <p className="text-muted-foreground mt-1 text-xs">{p.statement}</p>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* Risks */}
      {opportunity?.risks && opportunity.risks.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="text-sm">Risks</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-2">
              {opportunity.risks.map((risk: any) => (
                <div key={risk.id} className="flex items-start gap-3 rounded border p-2">
                  <div className="flex-1">
                    <div className="flex items-center gap-2">
                      <span className="font-mono text-xs">{risk.id}</span>
                      <Badge
                        variant="outline"
                        className={
                          risk.severity === "high"
                            ? "border-red-800 text-red-400"
                            : "border-yellow-800 text-yellow-400"
                        }
                      >
                        {risk.severity}
                      </Badge>
                      <Badge
                        variant={
                          risk.status === "mitigated"
                            ? "default"
                            : risk.status === "open"
                              ? "destructive"
                              : "secondary"
                        }
                        className="text-[10px]"
                      >
                        {risk.status}
                      </Badge>
                    </div>
                    <p className="text-muted-foreground mt-1 text-xs">{risk.description}</p>
                    {risk.mitigation && <p className="mt-1 text-xs italic">{risk.mitigation}</p>}
                  </div>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}

      <Separator />

      {/* Product Layers */}
      {product?.layers && product.layers.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="text-sm">Product Layers</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-3">
              {product.layers.map((layer: any, i: number) => (
                <div key={layer.id || i} className="rounded border p-3">
                  <div className="flex items-center gap-2">
                    <Badge variant="outline" className="font-mono">
                      Layer {layer.layer ?? i}
                    </Badge>
                    <span className="text-sm font-medium">{layer.name}</span>
                    {layer.theme && (
                      <span className="text-muted-foreground text-xs italic">— {layer.theme}</span>
                    )}
                  </div>
                  {layer.capabilities && (
                    <ul className="mt-2 list-inside list-disc space-y-0.5">
                      {layer.capabilities.map((cap: string, j: number) => (
                        <li key={j} className="text-muted-foreground text-xs">
                          {cap}
                        </li>
                      ))}
                    </ul>
                  )}
                  {layer.user_can && (
                    <p className="text-muted-foreground mt-1 text-xs">{layer.user_can}</p>
                  )}
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}

      {/* Jobs to be Done */}
      {product?.jobs_to_be_done && product.jobs_to_be_done.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="text-sm">Jobs to be Done</CardTitle>
          </CardHeader>
          <CardContent>
            <ul className="list-inside list-disc space-y-1">
              {product.jobs_to_be_done.map((job: string, i: number) => (
                <li key={i} className="text-sm">
                  {job}
                </li>
              ))}
            </ul>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
