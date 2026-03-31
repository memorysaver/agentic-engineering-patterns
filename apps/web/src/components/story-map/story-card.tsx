import { getStatusColor, getModuleColor, COMPLEXITY_COLORS } from "./status-colors";

type StoryCardProps = {
  storyId: string;
  title: string;
  status: string;
  complexity?: string;
  moduleId?: string;
  allModules?: string[];
  slice?: number;
  prUrl?: string | null;
  isSelected?: boolean;
  compact?: boolean;
  onClick: () => void;
};

export function StoryCard({
  storyId,
  title,
  status,
  complexity,
  moduleId,
  allModules,
  slice,
  prUrl,
  isSelected,
  compact,
  onClick,
}: StoryCardProps) {
  const colors = getStatusColor(status);
  const moduleColor = moduleId && allModules ? getModuleColor(moduleId, allModules) : undefined;

  if (compact) {
    return (
      <button
        type="button"
        data-story-id={storyId}
        onClick={onClick}
        className={`group flex w-full items-center gap-2 rounded border-l-2 px-2.5 py-1.5 text-left transition-all hover:bg-white/[0.03] ${colors.accent} ${colors.border} ${isSelected ? "bg-white/[0.06] ring-1 ring-white/10" : ""}`}
      >
        <span className={`h-1.5 w-1.5 shrink-0 rounded-full ${colors.dot}`} />
        <span className="min-w-0 flex-1 truncate text-[11px] text-zinc-300">{title}</span>
        {complexity && (
          <span
            className={`shrink-0 rounded px-1 py-px text-[9px] font-semibold ${COMPLEXITY_COLORS[complexity] || ""}`}
          >
            {complexity}
          </span>
        )}
      </button>
    );
  }

  return (
    <button
      type="button"
      data-story-id={storyId}
      onClick={onClick}
      className={`group w-[200px] rounded-md border border-l-[3px] p-2.5 text-left transition-all duration-150 hover:translate-y-[-1px] hover:shadow-lg hover:shadow-black/20 ${colors.bg} ${colors.border} ${colors.accent} ${isSelected ? "ring-1 ring-white/20 shadow-lg shadow-black/30" : ""}`}
    >
      <div className="flex items-start justify-between gap-1.5">
        <p className="text-[11px] font-medium leading-snug text-zinc-200 line-clamp-2 group-hover:text-white">
          {title}
        </p>
        {complexity && (
          <span
            className={`shrink-0 rounded px-1 py-px text-[9px] font-bold ${COMPLEXITY_COLORS[complexity] || ""}`}
          >
            {complexity}
          </span>
        )}
      </div>
      <div className="mt-2 flex items-center gap-1.5">
        <span className={`h-[5px] w-[5px] rounded-full ${colors.dot}`} />
        <span className={`text-[10px] font-medium tracking-wide uppercase ${colors.text}`}>
          {status.replace(/_/g, " ")}
        </span>
        {prUrl && (
          <span className="ml-auto rounded bg-sky-900/50 px-1 py-px text-[9px] font-medium text-sky-300">
            PR
          </span>
        )}
      </div>
      <div className="mt-1.5 flex items-center gap-1.5">
        <span className="font-mono text-[9px] text-zinc-500">{storyId}</span>
        {moduleId && moduleColor && (
          <span
            className="ml-auto rounded-sm px-1 py-px text-[9px] font-medium"
            style={{ backgroundColor: `${moduleColor}15`, color: moduleColor }}
          >
            {moduleId.replace("packages/", "").replace("apps/", "")}
          </span>
        )}
        {slice !== undefined && (
          <span className="rounded-sm bg-zinc-800 px-1 py-px text-[9px] text-zinc-500">
            S{slice}
          </span>
        )}
      </div>
    </button>
  );
}
