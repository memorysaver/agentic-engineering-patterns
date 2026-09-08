import { createFileRoute } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import { useState } from "react";
import { orpc } from "@/utils/orpc";

export const Route = createFileRoute("/ledger")({ component: LedgerPage });

function LedgerPage() {
  const { data, error, isLoading } = useQuery(orpc.productContext.getLedger.queryOptions());
  const [kind, setKind] = useState("story");
  const [search, setSearch] = useState("");
  if (isLoading) return <p className="p-6 text-muted-foreground">Loading project ledger…</p>;
  if (error || !data)
    return (
      <p role="alert" className="p-6 text-red-400">
        {error?.message || "Project ledger is unavailable."}
      </p>
    );
  const selected = data.records.filter(
    (r) =>
      (kind === "all" || r.kind === kind) &&
      `${r.id} ${r.title} ${r.description}`.toLowerCase().includes(search.toLowerCase()),
  );
  const readiness = new Map(data.readiness.map((r) => [r.story, r]));
  return (
    <main className="h-full overflow-auto p-6">
      <div className="mx-auto max-w-5xl space-y-5">
        <div>
          <h1 className="text-xl font-semibold">Project ledger</h1>
          <p className="text-sm text-muted-foreground">Work, decisions, evidence, and lessons</p>
        </div>
        <div className="flex flex-wrap gap-3">
          <label className="text-sm">
            Records{" "}
            <select
              aria-label="Record kind"
              className="ml-2 rounded border bg-background p-2"
              value={kind}
              onChange={(e) => setKind(e.target.value)}
            >
              {[
                "all",
                "story",
                "layer",
                "wave",
                "release",
                "gate",
                "change",
                "decision",
                "roadmap",
                "lesson",
                "rule",
                "attempt",
                "evidence",
                "review",
                "delivery",
                "event",
                "import",
              ].map((k) => (
                <option key={k}>{k}</option>
              ))}
            </select>
          </label>
          <input
            aria-label="Search records"
            placeholder="Search IDs, titles, and descriptions"
            className="min-w-64 flex-1 rounded border bg-background px-3 py-2 text-sm"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>
        {data.diagnostics.length > 0 && (
          <div role="alert" className="rounded border border-amber-700 p-3 text-sm">
            {data.diagnostics.map((d, i) => (
              <p key={i}>
                {d.code}: {d.message}
              </p>
            ))}
          </div>
        )}
        <p className="text-xs text-muted-foreground">
          {selected.length} records · Revision {data.revision.slice(0, 12)}
        </p>
        {selected.length === 0 && (
          <p className="rounded border p-6 text-sm text-muted-foreground">No matching records.</p>
        )}
        {selected.map((r) => {
          const ready = readiness.get(r.id);
          return (
            <article key={r.id} className="rounded-lg border bg-card p-4 space-y-3">
              <div className="flex flex-wrap items-baseline justify-between gap-2">
                <h2 className="font-semibold">
                  <span className="mr-2 font-mono text-xs text-muted-foreground">{r.id}</span>
                  {r.title}
                </h2>
                <span className="rounded bg-muted px-2 py-1 text-xs">
                  {r.status.replaceAll("_", " ")}
                </span>
              </div>
              <dl className="flex flex-wrap gap-x-5 gap-y-1 text-xs text-muted-foreground">
                {(
                  [
                    ["Layer", r.layer],
                    ["Wave", r.wave],
                    ["Release", r.release],
                    ["Change", r.change],
                    ["Owner", r.owner],
                  ] as const
                )
                  .filter(([, value]) => value)
                  .map(([label, value]) => (
                    <div key={label}>
                      <dt className="inline">{label}: </dt>
                      <dd className="inline">{value}</dd>
                    </div>
                  ))}
              </dl>
              {r.depends_on.length > 0 && (
                <p className="text-xs">Dependencies: {r.depends_on.join(", ")}</p>
              )}
              {r.required_gates.length > 0 && (
                <p className="text-xs">Required gates: {r.required_gates.join(", ")}</p>
              )}
              {ready && (
                <div className="text-sm">
                  {ready.ready ? (
                    <p className="text-emerald-500">Ready for dispatch</p>
                  ) : (
                    <ul className="list-inside list-disc text-muted-foreground">
                      {ready.reasons.map((reason) => (
                        <li key={reason}>{reason}</li>
                      ))}
                    </ul>
                  )}
                </div>
              )}
              {r.description && <p className="whitespace-pre-wrap text-sm">{r.description}</p>}
              <details className="text-xs">
                <summary className="cursor-pointer text-muted-foreground">
                  Record details and references
                </summary>
                <pre className="mt-2 overflow-auto rounded bg-muted p-3">
                  {JSON.stringify(r, null, 2)}
                </pre>
              </details>
            </article>
          );
        })}
      </div>
    </main>
  );
}
