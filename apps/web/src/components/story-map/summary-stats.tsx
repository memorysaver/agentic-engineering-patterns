import { getStatusColor } from "./status-colors";
import type { Card } from "./types";

type SummaryStatsProps = {
  cards: Card[];
  dispatchEpoch?: number;
};

export function SummaryStats({ cards, dispatchEpoch }: SummaryStatsProps) {
  const total = cards.length;
  const completed = cards.filter((c) => c.status === "completed" || c.status === "done").length;
  const inProgress = cards.filter((c) => c.status === "in_progress").length;
  const pending = cards.filter((c) => c.status === "pending" || c.status === "ready").length;
  const blocked = cards.filter((c) => c.status === "blocked" || c.status === "failed").length;
  const pct = total > 0 ? Math.round((completed / total) * 100) : 0;

  const stats = [
    { label: "Total", value: total, color: "text-zinc-300" },
    { label: "Done", value: completed, color: "text-emerald-400" },
    { label: "Active", value: inProgress, color: "text-amber-400" },
    { label: "Queued", value: pending, color: "text-sky-400" },
    ...(blocked > 0 ? [{ label: "Blocked", value: blocked, color: "text-red-400" }] : []),
  ];

  return (
    <div className="flex items-center gap-4 border-b border-zinc-800/80 bg-zinc-950/80 px-4 py-2.5 backdrop-blur-sm">
      {/* Progress ring */}
      <div className="relative h-9 w-9 shrink-0">
        <svg viewBox="0 0 36 36" className="h-9 w-9 -rotate-90">
          <circle cx="18" cy="18" r="15" fill="none" stroke="#27272a" strokeWidth="3" />
          <circle
            cx="18"
            cy="18"
            r="15"
            fill="none"
            stroke="#34d399"
            strokeWidth="3"
            strokeDasharray={`${pct * 0.94} 100`}
            strokeLinecap="round"
            className="transition-all duration-700"
          />
        </svg>
        <span className="absolute inset-0 flex items-center justify-center text-[9px] font-bold text-zinc-300">
          {pct}%
        </span>
      </div>

      {/* Stat pills */}
      <div className="flex items-center gap-3">
        {stats.map((s) => (
          <div key={s.label} className="flex items-baseline gap-1">
            <span className={`text-sm font-semibold tabular-nums ${s.color}`}>{s.value}</span>
            <span className="text-[10px] text-zinc-500 uppercase tracking-wider">{s.label}</span>
          </div>
        ))}
      </div>

      {/* Spacer */}
      <div className="flex-1" />

      {/* Status micro-bar */}
      <div className="flex h-1.5 w-32 overflow-hidden rounded-full bg-zinc-800">
        {[
          "completed",
          "done",
          "in_progress",
          "in_review",
          "ready",
          "pending",
          "blocked",
          "failed",
        ].map((status) => {
          const count = cards.filter((c) => c.status === status).length;
          if (count === 0) return null;
          const colors = getStatusColor(status);
          return (
            <div
              key={status}
              className={`h-full ${colors.dot} transition-all duration-500`}
              style={{ width: `${(count / total) * 100}%` }}
            />
          );
        })}
      </div>

      {/* Epoch badge */}
      {dispatchEpoch !== undefined && (
        <span className="rounded border border-zinc-700/50 bg-zinc-900/80 px-1.5 py-0.5 font-mono text-[10px] text-zinc-400">
          epoch {dispatchEpoch}
        </span>
      )}
    </div>
  );
}
