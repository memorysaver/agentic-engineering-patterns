import { StoryCard } from "./story-card";
import { getStatusColor } from "./status-colors";
import type { Card, Slice } from "./types";

type SliceViewProps = {
  cards: Card[];
  slices: Slice[];
  allModules: string[];
  onCardClick: (storyId: string) => void;
  selectedStoryId: string | null;
};

const SLICE_ACCENT_COLORS = ["#38bdf8", "#a78bfa", "#34d399", "#fbbf24", "#f87171", "#2dd4bf"];

export function SliceView({
  cards,
  slices,
  allModules,
  onCardClick,
  selectedStoryId,
}: SliceViewProps) {
  return (
    <div className="space-y-4 p-4">
      {slices.map((slice, i) => {
        const sliceCards = cards.filter((c) => slice.storyIds.includes(c.storyId));
        const completed = sliceCards.filter(
          (c) => c.status === "completed" || c.status === "done",
        ).length;
        const total = sliceCards.length;
        const pct = total > 0 ? Math.round((completed / total) * 100) : 0;
        const accentColor = SLICE_ACCENT_COLORS[i % SLICE_ACCENT_COLORS.length];
        const allDone = completed === total && total > 0;

        return (
          <div
            key={slice.slice}
            className="overflow-hidden rounded-lg border border-zinc-800/60 bg-zinc-900/30"
          >
            {/* Slice header */}
            <div
              className="flex items-center gap-3 border-b border-zinc-800/40 px-4 py-3"
              style={{ borderLeftWidth: 3, borderLeftColor: accentColor }}
            >
              <div
                className="flex h-7 w-7 items-center justify-center rounded-md font-mono text-xs font-bold"
                style={{ backgroundColor: `${accentColor}18`, color: accentColor }}
              >
                {slice.slice}
              </div>
              <div className="flex-1">
                <p className="text-sm font-medium text-zinc-200">{slice.theme}</p>
                <p className="text-[10px] text-zinc-500">
                  {total} {total === 1 ? "story" : "stories"} —{" "}
                  {allDone ? "complete" : `${completed}/${total}`}
                </p>
              </div>
              {/* Progress bar */}
              <div className="flex items-center gap-2">
                <div className="h-1.5 w-24 overflow-hidden rounded-full bg-zinc-800">
                  <div
                    className="h-full rounded-full transition-all duration-700"
                    style={{
                      width: `${pct}%`,
                      backgroundColor: allDone ? "#34d399" : accentColor,
                    }}
                  />
                </div>
                <span className="min-w-[2rem] text-right font-mono text-[10px] text-zinc-500">
                  {pct}%
                </span>
              </div>
            </div>

            {/* Cards in a horizontal flow */}
            <div className="flex flex-wrap gap-3 p-4">
              {sliceCards.length === 0 ? (
                <p className="text-xs text-zinc-600 italic">No stories match filters</p>
              ) : (
                sliceCards.map((card) => (
                  <StoryCard
                    key={card.storyId}
                    storyId={card.storyId}
                    title={card.title}
                    status={card.status}
                    complexity={card.complexity}
                    moduleId={card.moduleId}
                    allModules={allModules}
                    slice={card.slice}
                    prUrl={card.prUrl}
                    isSelected={card.storyId === selectedStoryId}
                    onClick={() => onCardClick(card.storyId)}
                  />
                ))
              )}
            </div>

            {/* Status summary micro-dots */}
            {sliceCards.length > 0 && (
              <div className="flex items-center gap-2 border-t border-zinc-800/30 px-4 py-2">
                {Object.entries(
                  sliceCards.reduce<Record<string, number>>((acc, c) => {
                    acc[c.status] = (acc[c.status] || 0) + 1;
                    return acc;
                  }, {}),
                ).map(([status, count]) => {
                  const colors = getStatusColor(status);
                  return (
                    <span key={status} className="flex items-center gap-1 text-[10px]">
                      <span className={`h-1.5 w-1.5 rounded-full ${colors.dot}`} />
                      <span className="text-zinc-500">{count}</span>
                    </span>
                  );
                })}
              </div>
            )}
          </div>
        );
      })}
    </div>
  );
}
