import { Box } from "lucide-react";
import { Section, SubLabel } from "./section";

interface MvpBoundary {
  in_scope?: string[];
  out_of_scope?: string[];
  deferred?: string[];
}

export function MvpBoundarySection({ boundary }: { boundary?: MvpBoundary }) {
  if (!boundary) return null;
  const { in_scope, out_of_scope, deferred } = boundary;
  if (!in_scope?.length && !out_of_scope?.length && !deferred?.length) return null;

  return (
    <Section icon={Box} title="MVP Boundary" accent="#38bdf8">
      <div className="grid gap-3 sm:grid-cols-3">
        <Column label="In Scope" items={in_scope} dotColor="bg-emerald-400" />
        <Column label="Out of Scope" items={out_of_scope} dotColor="bg-red-400" />
        <Column label="Deferred" items={deferred} dotColor="bg-amber-400" />
      </div>
    </Section>
  );
}

function Column({ label, items, dotColor }: { label: string; items?: string[]; dotColor: string }) {
  if (!items || items.length === 0) return <div />;
  return (
    <div className="space-y-1.5">
      <SubLabel>{label}</SubLabel>
      {items.map((item, i) => (
        <div key={i} className="flex items-start gap-2 text-[11px] text-zinc-400">
          <span className={`mt-1.5 inline-block h-1.5 w-1.5 shrink-0 rounded-full ${dotColor}`} />
          <span>{item}</span>
        </div>
      ))}
    </div>
  );
}
