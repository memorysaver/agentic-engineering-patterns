import { Scale } from "lucide-react";
import { Section } from "./section";

interface Decision {
  decision?: string;
  reasoning?: string;
  alternatives?: string[];
}

export function DecisionsSection({ decisions }: { decisions?: Decision[] }) {
  if (!decisions || decisions.length === 0) return null;

  return (
    <Section icon={Scale} title="Decisions" accent="#a78bfa">
      <div className="space-y-2">
        {decisions.map((d, i) => (
          <div
            key={i}
            className="rounded-md border border-zinc-800/40 bg-zinc-900/50 px-3 py-2.5 space-y-1.5"
          >
            <p className="text-[12px] font-medium text-zinc-300">{d.decision}</p>
            {d.reasoning && <p className="text-[11px] text-zinc-500 italic">{d.reasoning}</p>}
            {d.alternatives && d.alternatives.length > 0 && (
              <div className="flex flex-wrap gap-1 pt-0.5">
                <span className="text-[9px] text-zinc-600 uppercase font-medium">
                  alternatives:
                </span>
                {d.alternatives.map((alt, j) => (
                  <span
                    key={j}
                    className="rounded bg-violet-900/30 border border-violet-800/30 px-1.5 py-px text-[10px] text-violet-300"
                  >
                    {alt}
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
