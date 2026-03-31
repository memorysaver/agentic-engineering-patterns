import { STATUS_COLORS } from "./status-colors";

const LEGEND_ITEMS = [
  "pending",
  "ready",
  "in_progress",
  "in_review",
  "completed",
  "blocked",
  "failed",
  "deferred",
] as const;

export function StatusLegend() {
  return (
    <div className="flex flex-wrap gap-2">
      {LEGEND_ITEMS.map((status) => {
        const colors = STATUS_COLORS[status];
        return (
          <span key={status} className="flex items-center gap-1 text-[10px]">
            <span className={`h-2 w-2 rounded-full ${colors.dot}`} />
            <span className="text-muted-foreground">{status.replace(/_/g, " ")}</span>
          </span>
        );
      })}
    </div>
  );
}
