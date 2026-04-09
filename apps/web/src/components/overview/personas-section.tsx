import { Users } from "lucide-react";
import { Section } from "./section";

interface Persona {
  id?: string;
  description?: string;
  jtbd?: string;
}

export function PersonasSection({
  personas,
  legacyPersona,
}: {
  personas?: Persona[];
  legacyPersona?: { description?: string; jtbd?: string };
}) {
  const items: Persona[] =
    personas && personas.length > 0
      ? personas
      : legacyPersona
        ? [{ id: "default", ...legacyPersona }]
        : [];

  if (items.length === 0) return null;

  return (
    <Section icon={Users} title="Personas" accent="#06b6d4">
      <div className="grid gap-2 sm:grid-cols-2">
        {items.map((p, i) => (
          <div
            key={p.id || i}
            className="rounded-md border border-zinc-800/40 bg-zinc-900/50 px-3 py-2.5 space-y-1.5"
          >
            {p.id && (
              <span className="inline-block rounded bg-cyan-900/40 border border-cyan-800/30 px-1.5 py-px text-[10px] font-medium text-cyan-300">
                {p.id}
              </span>
            )}
            {p.description && (
              <p className="text-[12px] leading-relaxed text-zinc-400">{p.description}</p>
            )}
            {p.jtbd && (
              <p className="text-[11px] italic text-zinc-500 border-l-2 border-cyan-800/40 pl-2">
                "{p.jtbd}"
              </p>
            )}
          </div>
        ))}
      </div>
    </Section>
  );
}
