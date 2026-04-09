import { Fragment, useState } from "react";
import { StoryCard } from "./story-card";
import type { Card } from "./types";

type Activity = {
  id: string;
  name: string;
  description?: string;
  order: number;
  layerIntroduced: number;
};

type CapabilityJourney = {
  capabilityId: string;
  capabilityName: string;
  activities: Activity[];
  lanes: Array<{ id: string; name: string; theme?: string }>;
};

type JourneyViewProps = {
  cards: Card[];
  activities: Activity[];
  lanes: Array<{ id: string; name: string; theme?: string }>;
  allModules: string[];
  onCardClick: (storyId: string) => void;
  selectedStoryId: string | null;
  capabilityJourneys?: CapabilityJourney[];
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
  capabilityJourneys,
}: JourneyViewProps) {
  const [selectedCapability, setSelectedCapability] = useState<string | null>(null);

  const showTabs = capabilityJourneys && capabilityJourneys.length >= 2;

  // Determine effective activities/lanes based on selected capability
  const activeJourney =
    showTabs && selectedCapability
      ? capabilityJourneys.find((cj) => cj.capabilityId === selectedCapability)
      : null;

  const effectiveActivities = activeJourney ? activeJourney.activities : activities;
  const effectiveLanes = activeJourney ? activeJourney.lanes : lanes;

  // Activity ids for filtering cards when a capability is selected
  const activeActivityIds = activeJourney
    ? new Set(activeJourney.activities.map((a) => a.id))
    : null;

  // Only show cards that have an activity (user-facing stories)
  const journeyCards = cards.filter((c) => {
    if (!c.activity) return false;
    if (activeActivityIds && !activeActivityIds.has(c.activity)) return false;
    return true;
  });
  const infraCards = cards.filter((c) => !c.activity);

  // Get unique layers from journey cards
  const uniqueLayers = [...new Set(journeyCards.map((c) => c.layer))].sort((a, b) => a - b);

  const layerToLane = new Map(
    effectiveLanes.map((l) => {
      const num = l.id.match(/\d+/)?.[0];
      return [Number(num ?? 0), l];
    }),
  );

  return (
    <div className="p-4">
      {/* Capability tabs */}
      {showTabs && (
        <div className="flex rounded-lg bg-zinc-900/80 p-0.5 mb-3">
          <button
            type="button"
            onClick={() => setSelectedCapability(null)}
            className={`rounded-md px-3 py-1.5 text-[11px] font-medium transition-all ${
              selectedCapability === null
                ? "bg-zinc-800 text-zinc-100 shadow-sm shadow-black/20"
                : "text-zinc-500 hover:text-zinc-300"
            }`}
          >
            All
          </button>
          {capabilityJourneys.map((cj) => (
            <button
              key={cj.capabilityId}
              type="button"
              onClick={() => setSelectedCapability(cj.capabilityId)}
              className={`rounded-md px-3 py-1.5 text-[11px] font-medium transition-all ${
                selectedCapability === cj.capabilityId
                  ? "bg-zinc-800 text-zinc-100 shadow-sm shadow-black/20"
                  : "text-zinc-500 hover:text-zinc-300"
              }`}
            >
              {cj.capabilityName}
            </button>
          ))}
        </div>
      )}

      {/* Story map grid: activities × layers */}
      <div
        className="grid gap-0"
        style={{
          gridTemplateColumns: `100px repeat(${effectiveActivities.length}, minmax(180px, 1fr))`,
        }}
      >
        {/* Header row — empty corner + activity names */}
        <div className="border-b border-r border-zinc-800/40 p-2" />
        {effectiveActivities.map((a, i) => {
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
              <div className="flex items-center justify-center gap-1.5">
                <span
                  className="flex h-4 w-4 items-center justify-center rounded-full text-[9px] font-bold"
                  style={{ backgroundColor: `${color}20`, color }}
                >
                  {a.order}
                </span>
                <span className="text-xs font-semibold" style={{ color }}>
                  {a.name}
                </span>
              </div>
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
              {effectiveActivities.map((a) => {
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
