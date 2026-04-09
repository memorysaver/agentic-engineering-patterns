import { Flame } from "lucide-react";
import { Section } from "./section";

interface StressTestEntry {
  challenge?: string;
  angle?: string;
  resolution?: string;
}

export function StressTestSection({ stressTest }: { stressTest?: StressTestEntry[] }) {
  if (!stressTest || stressTest.length === 0) return null;

  return (
    <Section icon={Flame} title="Stress Test" accent="#fb923c">
      <div className="space-y-2">
        {stressTest.map((entry, i) => (
          <div
            key={i}
            className="rounded-md border border-zinc-800/40 bg-zinc-900/50 px-3 py-2.5 space-y-1.5"
          >
            <div className="flex items-center gap-2">
              <p className="text-[12px] font-medium text-zinc-300 flex-1">{entry.challenge}</p>
              {entry.angle && (
                <span className="shrink-0 rounded bg-orange-900/40 border border-orange-800/30 px-1.5 py-px text-[10px] font-medium text-orange-300">
                  {entry.angle}
                </span>
              )}
            </div>
            {entry.resolution && <p className="text-[11px] text-zinc-500">{entry.resolution}</p>}
          </div>
        ))}
      </div>
    </Section>
  );
}
