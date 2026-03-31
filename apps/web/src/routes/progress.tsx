import { createFileRoute } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import { orpc } from "@/utils/orpc";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@agentic-engineering-patterns/ui/components/card";
import { Badge } from "@agentic-engineering-patterns/ui/components/badge";
import { Progress } from "@agentic-engineering-patterns/ui/components/progress";
import { getStatusColor } from "@/components/story-map/status-colors";

export const Route = createFileRoute("/progress")({
  component: ProgressPage,
});

function ProgressPage() {
  const { data, isLoading, error } = useQuery(orpc.productContext.getProgress.queryOptions());

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
        <p className="text-destructive">Failed to load progress data</p>
      </div>
    );
  }

  const completionPct =
    data.totalStories > 0 ? Math.round((data.completedStories / data.totalStories) * 100) : 0;

  return (
    <div className="space-y-6 p-6">
      <div>
        <h2 className="text-lg font-semibold">Execution Progress</h2>
        <p className="text-muted-foreground text-sm">
          Dispatch Epoch {data.dispatchEpoch} — {data.completedStories}/{data.totalStories} stories
          completed
        </p>
      </div>

      {/* Overall Progress */}
      <Card>
        <CardHeader>
          <CardTitle className="text-sm">Overall Completion</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-2">
            <div className="flex items-center justify-between text-sm">
              <span>{completionPct}%</span>
              <span className="text-muted-foreground">
                {data.completedStories} of {data.totalStories}
              </span>
            </div>
            <Progress value={completionPct} className="h-3" />
          </div>

          {/* Status breakdown */}
          <div className="mt-4 flex flex-wrap gap-3">
            {Object.entries(data.statusCounts)
              .sort(([, a], [, b]) => (b as number) - (a as number))
              .map(([status, count]) => {
                const colors = getStatusColor(status);
                return (
                  <div key={status} className="flex items-center gap-1.5">
                    <span className={`h-2 w-2 rounded-full ${colors.dot}`} />
                    <span className="text-xs">
                      {status.replace(/_/g, " ")}: {count as number}
                    </span>
                  </div>
                );
              })}
          </div>
        </CardContent>
      </Card>

      {/* Slice Progress */}
      <Card>
        <CardHeader>
          <CardTitle className="text-sm">Execution Slices</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            {data.sliceProgress.map((slice) => {
              const pct = slice.total > 0 ? Math.round((slice.completed / slice.total) * 100) : 0;
              return (
                <div key={slice.slice} className="space-y-1">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <Badge variant="outline" className="font-mono text-[10px]">
                        Slice {slice.slice}
                      </Badge>
                      <span className="text-xs">{slice.theme}</span>
                    </div>
                    <span className="text-muted-foreground text-xs">
                      {slice.completed}/{slice.total}
                    </span>
                  </div>
                  <Progress value={pct} className="h-2" />
                  <div className="flex gap-2">
                    {Object.entries(slice.statuses).map(([status, count]) => {
                      const colors = getStatusColor(status);
                      return (
                        <span
                          key={status}
                          className={`flex items-center gap-1 text-[10px] ${colors.text}`}
                        >
                          <span className={`h-1 w-1 rounded-full ${colors.dot}`} />
                          {count}
                        </span>
                      );
                    })}
                  </div>
                </div>
              );
            })}
          </div>
        </CardContent>
      </Card>

      {/* Layer Progress */}
      <Card>
        <CardHeader>
          <CardTitle className="text-sm">Layer Progress</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            {data.layerProgress.map((layer) => {
              const pct = layer.total > 0 ? Math.round((layer.completed / layer.total) * 100) : 0;
              return (
                <div key={layer.layer} className="space-y-1">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <Badge variant="outline" className="font-mono text-[10px]">
                        Layer {layer.layer}
                      </Badge>
                      <span className="text-xs">{layer.name}</span>
                      {layer.gateStatus && (
                        <Badge
                          variant={layer.gateStatus === "passed" ? "default" : "secondary"}
                          className="text-[10px]"
                        >
                          Gate: {layer.gateStatus}
                        </Badge>
                      )}
                    </div>
                    <span className="text-muted-foreground text-xs">
                      {layer.completed}/{layer.total} ({pct}%)
                    </span>
                  </div>
                  <Progress value={pct} className="h-2" />
                </div>
              );
            })}
          </div>
        </CardContent>
      </Card>

      {/* Cost */}
      {data.cost.totalUsd > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="text-sm">Cost Tracking</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-2xl font-bold">${data.cost.totalUsd.toFixed(2)}</p>
            {Object.keys(data.cost.byLayer).length > 0 && (
              <div className="mt-2">
                <p className="text-muted-foreground text-xs font-medium">By Layer</p>
                {Object.entries(data.cost.byLayer).map(([layer, cost]) => (
                  <div key={layer} className="flex justify-between text-xs">
                    <span>Layer {layer}</span>
                    <span>${(cost as number).toFixed(2)}</span>
                  </div>
                ))}
              </div>
            )}
          </CardContent>
        </Card>
      )}
    </div>
  );
}
