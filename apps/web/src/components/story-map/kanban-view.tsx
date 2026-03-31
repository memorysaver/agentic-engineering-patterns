import { StoryCard } from "./story-card";
import { getStatusColor } from "./status-colors";
import type { Card } from "./types";

type KanbanViewProps = {
  cards: Card[];
  allModules: string[];
  onCardClick: (storyId: string) => void;
  selectedStoryId: string | null;
};

const KANBAN_COLUMNS = [
  { status: ["pending"], label: "Pending", icon: "\u25CB" },
  { status: ["ready"], label: "Ready", icon: "\u25D4" },
  { status: ["in_progress"], label: "In Progress", icon: "\u25D1" },
  { status: ["in_review", "review"], label: "In Review", icon: "\u25D5" },
  { status: ["completed", "done"], label: "Completed", icon: "\u25CF" },
  { status: ["blocked", "failed"], label: "Blocked", icon: "\u2716" },
];

export function KanbanView({ cards, allModules, onCardClick, selectedStoryId }: KanbanViewProps) {
  const activeColumns = KANBAN_COLUMNS.filter((col) =>
    cards.some((c) => col.status.includes(c.status)),
  );

  return (
    <div className="flex h-full gap-3 overflow-x-auto p-4">
      {activeColumns.map((col) => {
        const columnCards = cards.filter((c) => col.status.includes(c.status));
        const colors = getStatusColor(col.status[0]);

        return (
          <div
            key={col.label}
            className="flex w-[240px] shrink-0 flex-col rounded-lg border border-zinc-800/50 bg-zinc-900/20"
          >
            {/* Column header */}
            <div className="flex items-center gap-2 border-b border-zinc-800/40 px-3 py-2.5">
              <span className={`text-xs ${colors.text}`}>{col.icon}</span>
              <span className="text-xs font-semibold text-zinc-300">{col.label}</span>
              <span className="ml-auto rounded-full bg-zinc-800 px-1.5 py-px text-[10px] font-medium text-zinc-400 tabular-nums">
                {columnCards.length}
              </span>
            </div>

            {/* Cards */}
            <div className="flex flex-1 flex-col gap-2 overflow-y-auto p-2">
              {columnCards.length === 0 ? (
                <p className="py-8 text-center text-[10px] text-zinc-700 italic">No stories</p>
              ) : (
                columnCards.map((card) => (
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

            {/* Column footer micro-progress */}
            <div className="border-t border-zinc-800/30 px-3 py-1.5">
              <div className="h-[3px] overflow-hidden rounded-full bg-zinc-800/60">
                <div
                  className={`h-full rounded-full ${colors.dot} transition-all duration-500`}
                  style={{
                    width: `${cards.length > 0 ? (columnCards.length / cards.length) * 100 : 0}%`,
                  }}
                />
              </div>
            </div>
          </div>
        );
      })}
    </div>
  );
}
