export const STATUS_COLORS: Record<
  string,
  { bg: string; text: string; border: string; dot: string; accent: string; accentHex: string }
> = {
  pending: {
    bg: "bg-zinc-900/60",
    text: "text-zinc-400",
    border: "border-zinc-700/50",
    dot: "bg-zinc-500",
    accent: "border-l-zinc-500",
    accentHex: "#71717a",
  },
  ready: {
    bg: "bg-sky-950/40",
    text: "text-sky-400",
    border: "border-sky-800/40",
    dot: "bg-sky-400",
    accent: "border-l-sky-400",
    accentHex: "#38bdf8",
  },
  in_progress: {
    bg: "bg-amber-950/40",
    text: "text-amber-400",
    border: "border-amber-800/40",
    dot: "bg-amber-400",
    accent: "border-l-amber-400",
    accentHex: "#fbbf24",
  },
  in_review: {
    bg: "bg-violet-950/40",
    text: "text-violet-400",
    border: "border-violet-800/40",
    dot: "bg-violet-400",
    accent: "border-l-violet-400",
    accentHex: "#a78bfa",
  },
  review: {
    bg: "bg-violet-950/40",
    text: "text-violet-400",
    border: "border-violet-800/40",
    dot: "bg-violet-400",
    accent: "border-l-violet-400",
    accentHex: "#a78bfa",
  },
  completed: {
    bg: "bg-emerald-950/40",
    text: "text-emerald-400",
    border: "border-emerald-800/40",
    dot: "bg-emerald-400",
    accent: "border-l-emerald-400",
    accentHex: "#34d399",
  },
  done: {
    bg: "bg-emerald-950/40",
    text: "text-emerald-400",
    border: "border-emerald-800/40",
    dot: "bg-emerald-400",
    accent: "border-l-emerald-400",
    accentHex: "#34d399",
  },
  blocked: {
    bg: "bg-red-950/40",
    text: "text-red-400",
    border: "border-red-800/40",
    dot: "bg-red-400",
    accent: "border-l-red-400",
    accentHex: "#f87171",
  },
  failed: {
    bg: "bg-red-950/50",
    text: "text-red-300",
    border: "border-red-900/50",
    dot: "bg-red-500",
    accent: "border-l-red-500",
    accentHex: "#ef4444",
  },
  deferred: {
    bg: "bg-zinc-900/40",
    text: "text-zinc-500",
    border: "border-zinc-800/40",
    dot: "bg-zinc-600",
    accent: "border-l-zinc-600",
    accentHex: "#52525b",
  },
};

export const STATUS_ARROW_COLORS: Record<string, string> = {
  completed: "#34d399",
  done: "#34d399",
  in_progress: "#fbbf24",
  in_review: "#a78bfa",
  review: "#a78bfa",
  ready: "#38bdf8",
  pending: "#71717a",
  blocked: "#f87171",
  failed: "#ef4444",
  deferred: "#52525b",
};

export const COMPLEXITY_COLORS: Record<string, string> = {
  S: "bg-emerald-900/60 text-emerald-300",
  M: "bg-amber-900/60 text-amber-300",
  L: "bg-orange-900/60 text-orange-300",
};

export const MODULE_COLORS = [
  "#38bdf8", // sky
  "#a78bfa", // violet
  "#34d399", // emerald
  "#fbbf24", // amber
  "#f87171", // red
  "#fb923c", // orange
  "#2dd4bf", // teal
  "#e879f9", // fuchsia
  "#818cf8", // indigo
];

export function getStatusColor(status: string) {
  return STATUS_COLORS[status] || STATUS_COLORS.pending;
}

export function getModuleColor(moduleId: string, allModules: string[]): string {
  const idx = allModules.indexOf(moduleId);
  return MODULE_COLORS[idx >= 0 ? idx % MODULE_COLORS.length : 0];
}

export const STATUS_ORDER = [
  "pending",
  "ready",
  "in_progress",
  "in_review",
  "review",
  "completed",
  "done",
  "blocked",
  "failed",
  "deferred",
];
