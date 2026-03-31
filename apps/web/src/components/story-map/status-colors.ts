export const STATUS_COLORS: Record<
  string,
  { bg: string; text: string; border: string; dot: string }
> = {
  pending: {
    bg: "bg-zinc-800",
    text: "text-zinc-400",
    border: "border-zinc-700",
    dot: "bg-zinc-500",
  },
  ready: {
    bg: "bg-blue-950",
    text: "text-blue-400",
    border: "border-blue-800",
    dot: "bg-blue-500",
  },
  in_progress: {
    bg: "bg-amber-950",
    text: "text-amber-400",
    border: "border-amber-800",
    dot: "bg-amber-500",
  },
  in_review: {
    bg: "bg-purple-950",
    text: "text-purple-400",
    border: "border-purple-800",
    dot: "bg-purple-500",
  },
  review: {
    bg: "bg-purple-950",
    text: "text-purple-400",
    border: "border-purple-800",
    dot: "bg-purple-500",
  },
  completed: {
    bg: "bg-green-950",
    text: "text-green-400",
    border: "border-green-800",
    dot: "bg-green-500",
  },
  done: {
    bg: "bg-green-950",
    text: "text-green-400",
    border: "border-green-800",
    dot: "bg-green-500",
  },
  blocked: { bg: "bg-red-950", text: "text-red-400", border: "border-red-800", dot: "bg-red-500" },
  failed: { bg: "bg-red-950", text: "text-red-300", border: "border-red-900", dot: "bg-red-600" },
  deferred: {
    bg: "bg-zinc-900",
    text: "text-zinc-500",
    border: "border-zinc-800",
    dot: "bg-zinc-600",
  },
};

export const STATUS_ARROW_COLORS: Record<string, string> = {
  completed: "#22c55e",
  done: "#22c55e",
  in_progress: "#f59e0b",
  in_review: "#a855f7",
  review: "#a855f7",
  ready: "#3b82f6",
  pending: "#71717a",
  blocked: "#ef4444",
  failed: "#dc2626",
  deferred: "#52525b",
};

export const COMPLEXITY_COLORS: Record<string, string> = {
  S: "bg-emerald-800 text-emerald-200",
  M: "bg-yellow-800 text-yellow-200",
  L: "bg-orange-800 text-orange-200",
};

export function getStatusColor(status: string) {
  return STATUS_COLORS[status] || STATUS_COLORS.pending;
}
