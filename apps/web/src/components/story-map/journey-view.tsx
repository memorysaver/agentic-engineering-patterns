import { Fragment } from "react";
import { StoryCard } from "./story-card";
import type { Card } from "./types";

type Activity = {
  id: string;
  name: string;
  description?: string;
  order: number;
  layerIntroduced: number;
};

type JourneyViewProps = {
  cards: Card[];
  activities: Activity[];
  lanes: Array<{ id: string; name: string; theme?: string }>;
  allModules: string[];
  onCardClick: (storyId: string) => void;
  selectedStoryId: string | null;
};

const ACTIVITY_COLORS = [
  "#38bdf8",
  "#a78bfa",
  "#34d399",
  "#fbbf24",
  "#f87171",
  "#2dd4bf",
  "#e879f9",
  "#818cf8",
];

export function JourneyView({
  cards,
  activities,
  lanes,
  allModules,
  onCardClick,
  selectedStoryId,
}: JourneyViewProps) {
  // Only show cards that have an activity (user-facing stories)
  const journeyCards = cards.filter((c) => c.activity);
  const infraCards = cards.filter((c) => !c.activity);

  // Get unique layers from journey cards
  const uniqueLayers = [...new Set(journeyCards.map((c) => c.layer))].sort((a, b) => a - b);

  const layerToLane = new Map(
    lanes.map((l) => {
      const num = l.id.match(/\d+/)?.[0];
      return [Number(num ?? 0), l];
    }),
  );

  return (
    <div className="p-4">
      {/* Narrative sentence */}
      <div className="mb-4 rounded-lg border border-zinc-800/40 bg-zinc-900/30 px-4 py-2.5">
        <p className="text-[10px] uppercase tracking-wider text-zinc-500 mb-1">
          User Journey Narrative
        </p>
        <p className="text-sm text-zinc-300">
          {activities.map((a, i) => (
            <span key={a.id}>
              {i > 0 && <span className="text-zinc-600"> → </span>}
              <span
                className="font-medium"
                style={{ color: ACTIVITY_COLORS[i % ACTIVITY_COLORS.length] }}
              >
                {a.name}
              </span>
            </span>
          ))}
        </p>
      </div>

      {/* Story map grid: activities × layers */}
      <div
        className="grid gap-0"
        style={{
          gridTemplateColumns: `100px repeat(${activities.length}, minmax(180px, 1fr))`,
        }}
      >
        {/* Header row — empty corner + activity names */}
        <div className="border-b border-r border-zinc-800/40 p-2" />
        {activities.map((a, i) => {
          const color = ACTIVITY_COLORS[i % ACTIVITY_COLORS.length];
          const activityCards = journeyCards.filter((c) => c.activity === a.id);
          const completed = activityCards.filter(
            (c) => c.status === "completed" || c.status === "done",
          ).length;
          return (
            <div
              key={a.id}
              className="border-b border-r border-zinc-800/40 p-2 text-center"
              style={{ borderTopWidth: 3, borderTopColor: color }}
            >
              <p className="text-xs font-semibold" style={{ color }}>
                {a.name}
              </p>
              {a.description && (
                <p className="text-[9px] text-zinc-500 mt-0.5 line-clamp-1">{a.description}</p>
              )}
              <p className="text-[9px] text-zinc-600 mt-0.5">
                {completed}/{activityCards.length}
              </p>
            </div>
          );
        })}

        {/* Layer rows */}
        {uniqueLayers.map((layerNum) => {
          const lane = layerToLane.get(layerNum);
          return (
            <Fragment key={layerNum}>
              {/* Layer label */}
              <div className="border-b border-r border-zinc-800/40 p-2">
                <p className="text-[10px] font-bold text-zinc-400">L{layerNum}</p>
                <p className="text-[9px] text-zinc-600">{lane?.name || ""}</p>
              </div>

              {/* Activity cells for this layer */}
              {activities.map((a) => {
                const cellCards = journeyCards.filter(
                  (c) => c.activity === a.id && c.layer === layerNum,
                );
                return (
                  <div
                    key={`${layerNum}-${a.id}`}
                    className="border-b border-r border-zinc-800/40 p-2"
                    style={{ minHeight: 72 }}
                  >
                    {cellCards.length > 0 ? (
                      <div className="flex flex-wrap gap-2">
                        {cellCards.map((card) => (
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
                        ))}
                      </div>
                    ) : // Check if this activity has ANY stories across all layers
                    journeyCards.filter((c) => c.activity === a.id).length === 0 &&
                      layerNum === uniqueLayers[0] ? (
                      <p className="text-[10px] text-zinc-600 italic py-2">No stories yet</p>
                    ) : null}
                  </div>
                );
              })}
            </Fragment>
          );
        })}
      </div>

      {/* Infrastructure stories (not on the journey map) */}
      {infraCards.length > 0 && (
        <div className="mt-6">
          <div className="mb-2 flex items-center gap-2">
            <div className="h-px flex-1 bg-zinc-800/40" />
            <span className="text-[10px] text-zinc-600 uppercase tracking-wider">
              Infrastructure ({infraCards.length} stories — not user-facing)
            </span>
            <div className="h-px flex-1 bg-zinc-800/40" />
          </div>
          <div className="flex flex-wrap gap-2">
            {infraCards.map((card) => (
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
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
