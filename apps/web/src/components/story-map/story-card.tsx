import { getStatusColor, COMPLEXITY_COLORS } from "./status-colors";

type StoryCardProps = {
  storyId: string;
  title: string;
  status: string;
  complexity?: string;
  priority?: string;
  prUrl?: string | null;
  isSelected?: boolean;
  onClick: () => void;
};

export function StoryCard({
  storyId,
  title,
  status,
  complexity,
  priority,
  prUrl,
  isSelected,
  onClick,
}: StoryCardProps) {
  const colors = getStatusColor(status);

  return (
    <button
      type="button"
      data-story-id={storyId}
      onClick={onClick}
      className={`w-48 rounded-md border p-2 text-left transition-all hover:ring-1 hover:ring-accent ${
        colors.bg
      } ${colors.border} ${isSelected ? "ring-2 ring-primary" : ""}`}
    >
      <div className="flex items-start justify-between gap-1">
        <p className="text-xs font-medium leading-tight text-foreground line-clamp-2">{title}</p>
        {complexity && (
          <span
            className={`shrink-0 rounded px-1 text-[10px] font-bold ${
              COMPLEXITY_COLORS[complexity] || ""
            }`}
          >
            {complexity}
          </span>
        )}
      </div>
      <div className="mt-1.5 flex items-center gap-1.5">
        <span className={`h-1.5 w-1.5 rounded-full ${colors.dot}`} />
        <span className={`text-[10px] ${colors.text}`}>{status.replace(/_/g, " ")}</span>
        {prUrl && <span className="text-[10px] text-blue-400">PR</span>}
      </div>
      <p className="text-muted-foreground mt-0.5 text-[10px] font-mono">{storyId}</p>
    </button>
  );
}
