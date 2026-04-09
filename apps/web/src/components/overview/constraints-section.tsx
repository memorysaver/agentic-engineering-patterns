import { Lock } from "lucide-react";
import { Section, SubLabel } from "./section";

interface Constraints {
  required_stack?: Record<string, string> | string[];
  preferred_stack?: Record<string, string> | string[];
  infrastructure?: string;
  external_deps?: Array<string | { name?: string; provides?: string; failure_mode?: string }>;
}

export function ConstraintsSection({ constraints }: { constraints?: Constraints }) {
  if (!constraints) return null;

  const { required_stack, preferred_stack, infrastructure, external_deps } = constraints;

  const hasContent = required_stack || preferred_stack || infrastructure || external_deps?.length;
  if (!hasContent) return null;

  return (
    <Section icon={Lock} title="Constraints" accent="#fbbf24">
      <div className="space-y-3">
        {required_stack && <StackTable label="Required Stack" stack={required_stack} />}
        {preferred_stack && <StackTable label="Preferred Stack" stack={preferred_stack} />}
        {infrastructure && (
          <div className="space-y-1">
            <SubLabel>Infrastructure</SubLabel>
            <p className="text-[11px] text-zinc-400">{infrastructure}</p>
          </div>
        )}
        {external_deps && external_deps.length > 0 && (
          <div className="space-y-1.5">
            <SubLabel>External Dependencies</SubLabel>
            {external_deps.map((dep, i) => {
              if (typeof dep === "string") {
                return (
                  <span
                    key={i}
                    className="inline-block rounded bg-amber-900/30 border border-amber-800/30 px-1.5 py-px text-[10px] text-amber-300 mr-1.5"
                  >
                    {dep}
                  </span>
                );
              }
              return (
                <div
                  key={i}
                  className="rounded bg-zinc-900/50 border border-zinc-800/40 px-3 py-2 space-y-0.5"
                >
                  <span className="text-[11px] font-medium text-amber-300">{dep.name}</span>
                  {dep.provides && <p className="text-[10px] text-zinc-400">{dep.provides}</p>}
                  {dep.failure_mode && (
                    <p className="text-[10px] text-zinc-500 italic">Failure: {dep.failure_mode}</p>
                  )}
                </div>
              );
            })}
          </div>
        )}
      </div>
    </Section>
  );
}

function StackTable({ label, stack }: { label: string; stack: Record<string, string> | string[] }) {
  const entries = Array.isArray(stack)
    ? stack.map((s, i) => [String(i + 1), s])
    : Object.entries(stack);

  return (
    <div className="space-y-1">
      <SubLabel>{label}</SubLabel>
      <div className="rounded bg-zinc-900/60 border border-zinc-800/30 overflow-hidden">
        {entries.map(([key, val], i) => (
          <div
            key={i}
            className={`flex gap-3 px-3 py-1.5 text-[11px] ${i > 0 ? "border-t border-zinc-800/20" : ""}`}
          >
            <span className="w-24 shrink-0 font-medium text-zinc-500">{key}</span>
            <span className="text-zinc-400">
              {typeof val === "object" ? JSON.stringify(val) : String(val)}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
}
