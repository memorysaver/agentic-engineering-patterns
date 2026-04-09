import type React from "react";

export function Section({
  icon: Icon,
  title,
  accent = "#71717a",
  children,
}: {
  icon: React.ComponentType<{ className?: string; style?: React.CSSProperties }>;
  title: string;
  accent?: string;
  children: React.ReactNode;
}) {
  return (
    <div
      className="rounded-lg border border-zinc-800/50 bg-zinc-900/30"
      style={{ borderLeftWidth: 3, borderLeftColor: accent }}
    >
      <div className="flex items-center gap-2 border-b border-zinc-800/30 px-4 py-2.5">
        <Icon className="h-3.5 w-3.5" style={{ color: accent }} />
        <span className="text-xs font-semibold tracking-wide text-zinc-300 uppercase">{title}</span>
      </div>
      <div className="px-4 py-3">{children}</div>
    </div>
  );
}

export function SeverityDot({ severity }: { severity: string }) {
  const color =
    severity === "high" ? "bg-red-400" : severity === "medium" ? "bg-amber-400" : "bg-zinc-400";
  return <span className={`inline-block h-1.5 w-1.5 rounded-full ${color}`} />;
}

export function StatusPill({ status }: { status: string }) {
  const styles: Record<string, string> = {
    mitigated: "bg-emerald-900/50 text-emerald-300 border-emerald-800/40",
    open: "bg-red-900/50 text-red-300 border-red-800/40",
    acknowledged: "bg-zinc-800/80 text-zinc-400 border-zinc-700/40",
    proceed: "bg-emerald-900/50 text-emerald-300 border-emerald-800/40",
    kill: "bg-red-900/50 text-red-300 border-red-800/40",
    defer: "bg-amber-900/50 text-amber-300 border-amber-800/40",
    planned: "bg-sky-900/50 text-sky-300 border-sky-800/40",
    "in-progress": "bg-indigo-900/50 text-indigo-300 border-indigo-800/40",
    done: "bg-emerald-900/50 text-emerald-300 border-emerald-800/40",
  };
  return (
    <span
      className={`rounded border px-1.5 py-px text-[10px] font-medium ${styles[status] || "bg-zinc-800 text-zinc-400 border-zinc-700"}`}
    >
      {status}
    </span>
  );
}

export function SubLabel({ children }: { children: React.ReactNode }) {
  return (
    <p className="text-[10px] font-medium uppercase tracking-wider text-zinc-600">{children}</p>
  );
}
