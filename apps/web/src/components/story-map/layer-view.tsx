import { useState } from "react";
import { StoryCard } from "./story-card";
import { getStatusColor } from "./status-colors";
import type { Card, Lane } from "./types";

type LayerViewProps = {
  cards: Card[];
  lanes: Lane[];
  allModules: string[];
  onCardClick: (storyId: string) => void;
  selectedStoryId: string | null;
};

const LAYER_ACCENT = ["#34d399", "#38bdf8", "#fbbf24", "#a78bfa"];

export function LayerView({
  cards,
  lanes,
  allModules,
  onCardClick,
  selectedStoryId,
}: LayerViewProps) {
  const [expandedLayer, setExpandedLayer] = useState<string | null>(lanes[0]?.id || null);

  const uniqueLayers = [...new Set(cards.map((c) => c.layer))].sort((a, b) => a - b);

  return (
    <div className="space-y-2 p-4">
      {uniqueLayers.map((layerNum, i) => {
        const lane = lanes.find((l) => {
          const num = l.id.match(/\d+/)?.[0];
          return Number(num ?? -1) === layerNum;
        });
        const laneId = lane?.id || `layer-${layerNum}`;
        const isExpanded = expandedLayer === laneId;
        const layerCards = cards.filter((c) => c.layer === layerNum);
        const completed = layerCards.filter(
          (c) => c.status === "completed" || c.status === "done",
        ).length;
        const total = layerCards.length;
        const pct = total > 0 ? Math.round((completed / total) * 100) : 0;
        const accent = LAYER_ACCENT[i % LAYER_ACCENT.length];

        // Group by status for summary
        const statusGroups = layerCards.reduce<Record<string, number>>((acc, c) => {
          acc[c.status] = (acc[c.status] || 0) + 1;
          return acc;
        }, {});

        return (
          <div
            key={laneId}
            className={`overflow-hidden rounded-lg border transition-all duration-200 ${
              isExpanded
                ? "border-zinc-700/60 bg-zinc-900/40"
                : "border-zinc-800/40 bg-zinc-900/20 hover:border-zinc-700/40"
            }`}
          >
            {/* Layer header — always visible */}
            <button
              type="button"
              onClick={() => setExpandedLayer(isExpanded ? null : laneId)}
              className="flex w-full items-center gap-3 px-4 py-3 text-left"
            >
              {/* Layer number badge */}
              <div
                className="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg font-mono text-sm font-bold"
                style={{
                  backgroundColor: `${accent}12`,
                  color: accent,
                  border: `1px solid ${accent}30`,
                }}
              >
                L{layerNum}
              </div>

              <div className="min-w-0 flex-1">
                <div className="flex items-center gap-2">
                  <p className="text-sm font-semibold text-zinc-200">
                    {lane?.name || `Layer ${layerNum}`}
                  </p>
                  {lane?.theme && (
                    <span className="text-[10px] text-zinc-500 italic">— {lane.theme}</span>
                  )}
                </div>
                {/* Status dots row */}
                <div className="mt-1 flex items-center gap-2">
                  {Object.entries(statusGroups).map(([status, count]) => {
                    const colors = getStatusColor(status);
                    return (
                      <span key={status} className="flex items-center gap-0.5 text-[10px]">
                        <span className={`h-1.5 w-1.5 rounded-full ${colors.dot}`} />
                        <span className={colors.text}>{count}</span>
                      </span>
                    );
                  })}
                </div>
              </div>

              {/* Progress bar + percentage */}
              <div className="flex items-center gap-2">
                <div className="h-2 w-28 overflow-hidden rounded-full bg-zinc-800">
                  <div
                    className="h-full rounded-full transition-all duration-700"
                    style={{ width: `${pct}%`, backgroundColor: accent }}
                  />
                </div>
                <span
                  className="min-w-[2.5rem] text-right font-mono text-xs tabular-nums"
                  style={{ color: accent }}
                >
                  {completed}/{total}
                </span>
              </div>

              {/* Expand chevron */}
              <svg
                className={`h-4 w-4 shrink-0 text-zinc-500 transition-transform duration-200 ${isExpanded ? "rotate-180" : ""}`}
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                strokeWidth={2}
              >
                <path strokeLinecap="round" strokeLinejoin="round" d="M19 9l-7 7-7-7" />
              </svg>
            </button>

            {/* Expanded content */}
            {isExpanded && (
              <div className="border-t border-zinc-800/40">
                {/* Capabilities */}
                {lane?.capabilities && lane.capabilities.length > 0 && (
                  <div className="border-b border-zinc-800/30 px-4 py-2.5">
                    <p className="mb-1.5 text-[10px] font-medium uppercase tracking-wider text-zinc-500">
                      Capabilities
                    </p>
                    <div className="flex flex-wrap gap-1.5">
                      {lane.capabilities.map((cap, j) => (
                        <span
                          key={j}
                          className="rounded border border-zinc-800/50 bg-zinc-900/60 px-2 py-0.5 text-[10px] text-zinc-400"
                        >
                          {cap}
                        </span>
                      ))}
                    </div>
                  </div>
                )}

                {/* Stories */}
                <div className="space-y-1 p-3">
                  {layerCards.length === 0 ? (
                    <p className="py-4 text-center text-xs text-zinc-600 italic">
                      No stories match filters
                    </p>
                  ) : (
                    layerCards.map((card) => (
                      <StoryCard
                        key={card.storyId}
                        storyId={card.storyId}
                        title={card.title}
                        status={card.status}
                        complexity={card.complexity}
                        moduleId={card.moduleId}
                        allModules={allModules}
                        prUrl={card.prUrl}
                        isSelected={card.storyId === selectedStoryId}
                        compact
                        onClick={() => onCardClick(card.storyId)}
                      />
                    ))
                  )}
                </div>
              </div>
            )}
          </div>
        );
      })}
    </div>
  );
}
