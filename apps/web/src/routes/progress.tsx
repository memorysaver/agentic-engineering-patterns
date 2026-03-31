import { createFileRoute } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import { orpc } from "@/utils/orpc";
import { getStatusColor } from "@/components/story-map/status-colors";
import { Activity, TrendingUp, Layers } from "lucide-react";

export const Route = createFileRoute("/progress")({
  component: ProgressPage,
});

function Bar({
  value,
  color = "#34d399",
  height = "h-1.5",
}: {
  value: number;
  color?: string;
  height?: string;
}) {
  return (
    <div className={`${height} w-full overflow-hidden rounded-full bg-zinc-800/60`}>
      <div
        className="h-full rounded-full transition-all duration-700"
        style={{ width: `${value}%`, backgroundColor: color }}
      />
    </div>
  );
}

function ProgressPage() {
  const { data, isLoading, error } = useQuery(orpc.productContext.getProgress.queryOptions());

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
        <p className="text-sm text-red-400">Failed to load progress data</p>
      </div>
    );
  }

  const pct =
    data.totalStories > 0 ? Math.round((data.completedStories / data.totalStories) * 100) : 0;

  return (
    <div className="h-full overflow-auto bg-[#0a0a0b]">
      <div className="mx-auto max-w-4xl space-y-4 p-6">
        {/* Hero stat */}
        <div className="flex items-center gap-6 rounded-lg border border-zinc-800/50 bg-zinc-900/30 p-5">
          {/* Big ring */}
          <div className="relative h-20 w-20 shrink-0">
            <svg viewBox="0 0 36 36" className="h-20 w-20 -rotate-90">
              <circle cx="18" cy="18" r="15.5" fill="none" stroke="#1c1c1e" strokeWidth="2.5" />
              <circle
                cx="18"
                cy="18"
                r="15.5"
                fill="none"
                stroke="#34d399"
                strokeWidth="2.5"
                strokeDasharray={`${pct * 0.975} 100`}
                strokeLinecap="round"
                className="transition-all duration-1000"
              />
            </svg>
            <span className="absolute inset-0 flex items-center justify-center text-lg font-bold text-zinc-200">
              {pct}%
            </span>
          </div>
          <div>
            <p className="text-sm font-semibold text-zinc-200">
              {data.completedStories} of {data.totalStories} stories completed
            </p>
            <p className="text-xs text-zinc-500 mt-0.5">Dispatch Epoch {data.dispatchEpoch}</p>
            <div className="mt-2 flex gap-3">
              {Object.entries(data.statusCounts)
                .sort(([, a], [, b]) => (b as number) - (a as number))
                .map(([status, count]) => {
                  const colors = getStatusColor(status);
                  return (
                    <span key={status} className="flex items-center gap-1">
                      <span className={`h-[5px] w-[5px] rounded-full ${colors.dot}`} />
                      <span className="text-[10px] text-zinc-500">
                        {count as number} {status.replace(/_/g, " ")}
                      </span>
                    </span>
                  );
                })}
            </div>
          </div>
        </div>

        {/* Execution Slices */}
        <div
          className="rounded-lg border border-zinc-800/50 bg-zinc-900/30"
          style={{ borderLeftWidth: 3, borderLeftColor: "#38bdf8" }}
        >
          <div className="flex items-center gap-2 border-b border-zinc-800/30 px-4 py-2.5">
            <Activity className="h-3.5 w-3.5 text-sky-400" />
            <span className="text-xs font-semibold tracking-wide text-zinc-300 uppercase">
              Execution Slices
            </span>
          </div>
          <div className="divide-y divide-zinc-800/30">
            {data.sliceProgress.map((slice) => {
              const slicePct =
                slice.total > 0 ? Math.round((slice.completed / slice.total) * 100) : 0;
              const allDone = slice.completed === slice.total && slice.total > 0;
              return (
                <div key={slice.slice} className="flex items-center gap-4 px-4 py-3">
                  <span className="flex h-6 w-6 shrink-0 items-center justify-center rounded bg-zinc-800/80 font-mono text-[10px] font-bold text-zinc-400">
                    {slice.slice}
                  </span>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center justify-between mb-1">
                      <span className="text-[11px] font-medium text-zinc-300 truncate">
                        {slice.theme}
                      </span>
                      <span className="text-[10px] text-zinc-500 tabular-nums ml-2">
                        {slice.completed}/{slice.total}
                      </span>
                    </div>
                    <Bar value={slicePct} color={allDone ? "#34d399" : "#38bdf8"} />
                  </div>
                  <div className="flex gap-1 shrink-0">
                    {Object.entries(slice.statuses).map(([status, count]) => {
                      const colors = getStatusColor(status);
                      return (
                        <span key={status} className="flex items-center gap-0.5">
                          <span className={`h-1 w-1 rounded-full ${colors.dot}`} />
                          <span className={`text-[9px] ${colors.text}`}>{count}</span>
                        </span>
                      );
                    })}
                  </div>
                </div>
              );
            })}
          </div>
        </div>

        {/* Layer Progress */}
        <div
          className="rounded-lg border border-zinc-800/50 bg-zinc-900/30"
          style={{ borderLeftWidth: 3, borderLeftColor: "#a78bfa" }}
        >
          <div className="flex items-center gap-2 border-b border-zinc-800/30 px-4 py-2.5">
            <Layers className="h-3.5 w-3.5 text-violet-400" />
            <span className="text-xs font-semibold tracking-wide text-zinc-300 uppercase">
              Layer Progress
            </span>
          </div>
          <div className="divide-y divide-zinc-800/30">
            {data.layerProgress.map((layer) => {
              const layerPct =
                layer.total > 0 ? Math.round((layer.completed / layer.total) * 100) : 0;
              const LAYER_COLORS = ["#34d399", "#38bdf8", "#fbbf24", "#a78bfa"];
              const color = LAYER_COLORS[layer.layer % LAYER_COLORS.length];
              return (
                <div key={layer.layer} className="flex items-center gap-4 px-4 py-3">
                  <span
                    className="flex h-6 w-6 shrink-0 items-center justify-center rounded font-mono text-[10px] font-bold"
                    style={{ backgroundColor: `${color}18`, color }}
                  >
                    L{layer.layer}
                  </span>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center justify-between mb-1">
                      <div className="flex items-center gap-2">
                        <span className="text-[11px] font-medium text-zinc-300">{layer.name}</span>
                        {layer.gateStatus && (
                          <span
                            className={`rounded px-1 py-px text-[9px] font-medium ${
                              layer.gateStatus === "passed"
                                ? "bg-emerald-900/50 text-emerald-300"
                                : "bg-zinc-800 text-zinc-500"
                            }`}
                          >
                            gate: {layer.gateStatus}
                          </span>
                        )}
                      </div>
                      <span className="text-[10px] text-zinc-500 tabular-nums">
                        {layer.completed}/{layer.total}
                      </span>
                    </div>
                    <Bar value={layerPct} color={color} />
                  </div>
                </div>
              );
            })}
          </div>
        </div>

        {/* Cost (only if > 0) */}
        {data.cost.totalUsd > 0 && (
          <div
            className="rounded-lg border border-zinc-800/50 bg-zinc-900/30"
            style={{ borderLeftWidth: 3, borderLeftColor: "#fbbf24" }}
          >
            <div className="flex items-center gap-2 border-b border-zinc-800/30 px-4 py-2.5">
              <TrendingUp className="h-3.5 w-3.5 text-amber-400" />
              <span className="text-xs font-semibold tracking-wide text-zinc-300 uppercase">
                Cost
              </span>
            </div>
            <div className="px-4 py-3">
              <span className="text-xl font-bold text-zinc-200 tabular-nums">
                ${data.cost.totalUsd.toFixed(2)}
              </span>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
