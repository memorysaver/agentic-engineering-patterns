import { createFileRoute } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import { orpc } from "@/utils/orpc";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@agentic-engineering-patterns/ui/components/card";
import { Badge } from "@agentic-engineering-patterns/ui/components/badge";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
} from "@agentic-engineering-patterns/ui/components/sheet";
import { Separator } from "@agentic-engineering-patterns/ui/components/separator";
import { useState, useRef, useEffect, useCallback } from "react";

export const Route = createFileRoute("/architecture")({
  component: ArchitecturePage,
});

type ModuleNode = {
  id: string;
  name: string;
  package?: string;
  status?: string;
  responsibility?: string;
  does?: string | string[];
  doesNot?: string | string[];
};

function ArchitecturePage() {
  const { data, isLoading, error } = useQuery(orpc.productContext.getModuleGraph.queryOptions());
  const [selectedModule, setSelectedModule] = useState<ModuleNode | null>(null);

  if (isLoading) {
    return (
      <div className="flex h-full items-center justify-center">
        <p className="text-muted-foreground">Loading...</p>
      </div>
    );
  }

  if (error || !data) {
    return (
      <div className="flex h-full items-center justify-center">
        <p className="text-destructive">Failed to load architecture data</p>
      </div>
    );
  }

  return (
    <div className="flex h-full flex-col">
      <div className="border-b px-4 py-2">
        <h2 className="text-lg font-semibold">Architecture</h2>
        <p className="text-muted-foreground text-xs">
          {data.nodes.length} modules, {data.edges.length} dependencies
        </p>
      </div>
      <div className="flex-1 overflow-auto p-6">
        <ModuleGraph nodes={data.nodes} edges={data.edges} onNodeClick={setSelectedModule} />
      </div>

      {selectedModule && (
        <Sheet open={!!selectedModule} onOpenChange={(v) => !v && setSelectedModule(null)}>
          <SheetContent className="w-96 overflow-auto">
            <SheetHeader>
              <SheetTitle className="text-sm">{selectedModule.name}</SheetTitle>
            </SheetHeader>
            <div className="mt-4 space-y-3">
              <dl className="space-y-2 text-xs">
                {selectedModule.package && (
                  <>
                    <dt className="text-muted-foreground">Package</dt>
                    <dd className="font-mono">{selectedModule.package}</dd>
                  </>
                )}
                {selectedModule.status && (
                  <>
                    <dt className="text-muted-foreground">Status</dt>
                    <dd>
                      <Badge variant="outline">{selectedModule.status}</Badge>
                    </dd>
                  </>
                )}
                {selectedModule.responsibility && (
                  <>
                    <dt className="text-muted-foreground">Responsibility</dt>
                    <dd>{selectedModule.responsibility}</dd>
                  </>
                )}
              </dl>
              <Separator />
              {selectedModule.does && (
                <div>
                  <p className="text-muted-foreground text-xs font-medium">Does</p>
                  {Array.isArray(selectedModule.does) ? (
                    <ul className="mt-1 list-inside list-disc space-y-0.5">
                      {selectedModule.does.map((d, i) => (
                        <li key={i} className="text-xs">
                          {d}
                        </li>
                      ))}
                    </ul>
                  ) : (
                    <p className="text-xs">{selectedModule.does}</p>
                  )}
                </div>
              )}
              {selectedModule.doesNot && (
                <div>
                  <p className="text-muted-foreground text-xs font-medium">Does Not</p>
                  {Array.isArray(selectedModule.doesNot) ? (
                    <ul className="mt-1 list-inside list-disc space-y-0.5">
                      {selectedModule.doesNot.map((d, i) => (
                        <li key={i} className="text-xs">
                          {d}
                        </li>
                      ))}
                    </ul>
                  ) : (
                    <p className="text-xs">{selectedModule.doesNot}</p>
                  )}
                </div>
              )}
            </div>
          </SheetContent>
        </Sheet>
      )}
    </div>
  );
}

// Simple force-directed-ish layout for module graph
function ModuleGraph({
  nodes,
  edges,
  onNodeClick,
}: {
  nodes: ModuleNode[];
  edges: Array<{ from: string; to: string }>;
  onNodeClick: (node: ModuleNode) => void;
}) {
  const svgRef = useRef<SVGSVGElement>(null);
  const [positions, setPositions] = useState<Map<string, { x: number; y: number }>>(new Map());

  const layoutNodes = useCallback(() => {
    // Simple grid layout — arrange nodes in a circle
    const cx = 400;
    const cy = 300;
    const radius = 220;
    const newPositions = new Map<string, { x: number; y: number }>();

    nodes.forEach((node, i) => {
      const angle = (2 * Math.PI * i) / nodes.length - Math.PI / 2;
      newPositions.set(node.id, {
        x: cx + radius * Math.cos(angle),
        y: cy + radius * Math.sin(angle),
      });
    });

    setPositions(newPositions);
  }, [nodes]);

  useEffect(() => {
    layoutNodes();
  }, [layoutNodes]);

  const statusColors: Record<string, string> = {
    new: "#22c55e",
    extend: "#3b82f6",
    "no-change": "#71717a",
    "auto-update": "#a855f7",
  };

  return (
    <svg ref={svgRef} viewBox="0 0 800 600" className="h-full w-full min-h-[500px]">
      <defs>
        <marker id="graph-arrow" markerWidth="8" markerHeight="6" refX="8" refY="3" orient="auto">
          <polygon points="0 0, 8 3, 0 6" fill="#525252" />
        </marker>
      </defs>

      {/* Edges */}
      {edges.map((edge) => {
        const from = positions.get(edge.from);
        const to = positions.get(edge.to);
        if (!from || !to) return null;

        // Shorten line to not overlap with node circles
        const dx = to.x - from.x;
        const dy = to.y - from.y;
        const dist = Math.sqrt(dx * dx + dy * dy);
        const nodeRadius = 40;
        const x1 = from.x + (dx / dist) * nodeRadius;
        const y1 = from.y + (dy / dist) * nodeRadius;
        const x2 = to.x - (dx / dist) * nodeRadius;
        const y2 = to.y - (dy / dist) * nodeRadius;

        return (
          <line
            key={`${edge.from}-${edge.to}`}
            x1={x1}
            y1={y1}
            x2={x2}
            y2={y2}
            stroke="#525252"
            strokeWidth={1.5}
            markerEnd="url(#graph-arrow)"
          />
        );
      })}

      {/* Nodes */}
      {nodes.map((node) => {
        const pos = positions.get(node.id);
        if (!pos) return null;
        const color = statusColors[node.status || ""] || "#71717a";

        return (
          <g key={node.id} className="cursor-pointer" onClick={() => onNodeClick(node)}>
            <circle cx={pos.x} cy={pos.y} r={35} fill="#18181b" stroke={color} strokeWidth={2} />
            <text
              x={pos.x}
              y={pos.y - 5}
              textAnchor="middle"
              className="fill-foreground text-xs font-medium"
              style={{ fontSize: 11 }}
            >
              {node.id}
            </text>
            {node.package && (
              <text
                x={pos.x}
                y={pos.y + 10}
                textAnchor="middle"
                className="fill-muted-foreground"
                style={{ fontSize: 8 }}
              >
                {node.package}
              </text>
            )}
            {node.status && (
              <text
                x={pos.x}
                y={pos.y + 50}
                textAnchor="middle"
                fill={color}
                style={{ fontSize: 9 }}
              >
                {node.status}
              </text>
            )}
          </g>
        );
      })}
    </svg>
  );
}
