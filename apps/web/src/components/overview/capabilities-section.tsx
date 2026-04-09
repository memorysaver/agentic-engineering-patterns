import { Puzzle } from "lucide-react";
import { Section, StatusPill } from "./section";

interface Capability {
  id?: string;
  name?: string;
  description?: string;
  map_path?: string;
  status?: string;
  depends_on?: string[];
}

export function CapabilitiesSection({ capabilities }: { capabilities?: Capability[] }) {
  if (!capabilities || capabilities.length === 0) return null;

  return (
    <Section icon={Puzzle} title="Capabilities" accent="#818cf8">
      <div className="grid gap-2 sm:grid-cols-2">
        {capabilities.map((cap, i) => (
          <div
            key={cap.id || i}
            className="rounded-md border border-zinc-800/40 bg-zinc-900/50 px-3 py-2.5 space-y-1.5"
          >
            <div className="flex items-center justify-between gap-2">
              <span className="text-[12px] font-semibold text-zinc-300">{cap.name || cap.id}</span>
              {cap.status && <StatusPill status={cap.status} />}
            </div>
            {cap.description && (
              <p className="text-[11px] leading-relaxed text-zinc-400">{cap.description}</p>
            )}
            {cap.depends_on && cap.depends_on.length > 0 && (
              <div className="flex flex-wrap gap-1 pt-0.5">
                <span className="text-[9px] text-zinc-600 uppercase font-medium">depends:</span>
                {cap.depends_on.map((dep) => (
                  <span
                    key={dep}
                    className="rounded bg-zinc-800/60 px-1.5 py-px text-[9px] text-zinc-500"
                  >
                    {dep}
                  </span>
                ))}
              </div>
            )}
          </div>
        ))}
      </div>
    </Section>
  );
}
