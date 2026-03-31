import { createFileRoute } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import { orpc } from "@/utils/orpc";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
} from "@agentic-engineering-patterns/ui/components/sheet";
import { useState, useRef, useEffect, useCallback } from "react";
import { GitBranch } from "lucide-react";

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

const STATUS_COLORS: Record<string, { stroke: string; fill: string; label: string }> = {
  new: { stroke: "#34d399", fill: "#34d39910", label: "New" },
  extend: { stroke: "#38bdf8", fill: "#38bdf810", label: "Extend" },
  "no-change": { stroke: "#52525b", fill: "#52525b10", label: "Stable" },
  "auto-update": { stroke: "#a78bfa", fill: "#a78bfa10", label: "Auto" },
};

function ArchitecturePage() {
  const { data, isLoading, error } = useQuery(orpc.productContext.getModuleGraph.queryOptions());
  const [selectedModule, setSelectedModule] = useState<ModuleNode | null>(null);

  if (isLoading) {
    return (
      <div className="flex h-full items-center justify-center">
        <div className="h-5 w-5 animate-spin rounded-full border-2 border-zinc-700 border-t-emerald-400" />
      </div>
    );
  }

  if (error || !data) {
    return (
      <div className="flex h-full items-center justify-center">
        <p className="text-sm text-red-400">Failed to load architecture data</p>
      </div>
    );
  }

  return (
    <div className="flex h-full flex-col bg-[#0a0a0b]">
      <div className="flex items-center gap-2 border-b border-zinc-800/50 px-4 py-2.5">
        <GitBranch className="h-3.5 w-3.5 text-zinc-500" />
        <span className="text-xs font-semibold text-zinc-300">{data.nodes.length} modules</span>
        <span className="text-[10px] text-zinc-600">{data.edges.length} dependencies</span>
        <div className="flex-1" />
        {/* Legend */}
        <div className="flex items-center gap-3">
          {Object.entries(STATUS_COLORS).map(([key, val]) => (
            <span key={key} className="flex items-center gap-1">
              <span className="h-2 w-2 rounded-full" style={{ backgroundColor: val.stroke }} />
              <span className="text-[9px] text-zinc-600">{val.label}</span>
            </span>
          ))}
        </div>
      </div>
      <div className="flex-1 overflow-auto">
        <ModuleGraph nodes={data.nodes} edges={data.edges} onNodeClick={setSelectedModule} />
      </div>

      {selectedModule && (
        <Sheet open={!!selectedModule} onOpenChange={(v) => !v && setSelectedModule(null)}>
          <SheetContent className="w-[360px] overflow-auto bg-zinc-950 border-zinc-800">
            <SheetHeader>
              <SheetTitle className="flex items-center gap-2 text-sm text-zinc-200">
                <span
                  className="h-2 w-2 rounded-full"
                  style={{
                    backgroundColor:
                      STATUS_COLORS[selectedModule.status || ""]?.stroke || "#52525b",
                  }}
                />
                {selectedModule.name}
              </SheetTitle>
            </SheetHeader>
            <div className="mt-4 space-y-4">
              {selectedModule.package && (
                <div>
                  <p className="text-[10px] text-zinc-600 uppercase tracking-wider mb-0.5">
                    Package
                  </p>
                  <p className="font-mono text-xs text-zinc-400">{selectedModule.package}</p>
                </div>
              )}
              {selectedModule.status && (
                <div>
                  <p className="text-[10px] text-zinc-600 uppercase tracking-wider mb-0.5">
                    Status
                  </p>
                  <span
                    className="inline-block rounded border px-1.5 py-0.5 text-[10px] font-medium"
                    style={{
                      borderColor: `${STATUS_COLORS[selectedModule.status]?.stroke || "#52525b"}40`,
                      color: STATUS_COLORS[selectedModule.status]?.stroke || "#52525b",
                    }}
                  >
                    {selectedModule.status}
                  </span>
                </div>
              )}
              {selectedModule.does && (
                <div>
                  <p className="text-[10px] text-zinc-600 uppercase tracking-wider mb-1">Does</p>
                  <p className="text-[11px] text-zinc-400 leading-relaxed">
                    {Array.isArray(selectedModule.does)
                      ? selectedModule.does.join(". ")
                      : selectedModule.does}
                  </p>
                </div>
              )}
              {selectedModule.doesNot && (
                <div>
                  <p className="text-[10px] text-zinc-600 uppercase tracking-wider mb-1">
                    Does Not
                  </p>
                  <p className="text-[11px] text-zinc-500 leading-relaxed">
                    {Array.isArray(selectedModule.doesNot)
                      ? selectedModule.doesNot.join(". ")
                      : selectedModule.doesNot}
                  </p>
                </div>
              )}
            </div>
          </SheetContent>
        </Sheet>
      )}
    </div>
  );
}

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
  const [hoveredNode, setHoveredNode] = useState<string | null>(null);

  const layoutNodes = useCallback(() => {
    const cx = 450;
    const cy = 320;
    const radius = 240;
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

  const connectedTo = new Set<string>();
  if (hoveredNode) {
    for (const e of edges) {
      if (e.from === hoveredNode) connectedTo.add(e.to);
      if (e.to === hoveredNode) connectedTo.add(e.from);
    }
  }

  return (
    <svg ref={svgRef} viewBox="0 0 900 640" className="h-full w-full min-h-[500px]">
      <defs>
        <marker id="g-arrow" markerWidth="6" markerHeight="5" refX="6" refY="2.5" orient="auto">
          <polygon points="0 0, 6 2.5, 0 5" fill="#3f3f46" />
        </marker>
        <marker id="g-arrow-lit" markerWidth="6" markerHeight="5" refX="6" refY="2.5" orient="auto">
          <polygon points="0 0, 6 2.5, 0 5" fill="#71717a" />
        </marker>
      </defs>

      {/* Edges */}
      {edges.map((edge) => {
        const from = positions.get(edge.from);
        const to = positions.get(edge.to);
        if (!from || !to) return null;
        const dx = to.x - from.x;
        const dy = to.y - from.y;
        const dist = Math.sqrt(dx * dx + dy * dy);
        const r = 42;
        const x1 = from.x + (dx / dist) * r;
        const y1 = from.y + (dy / dist) * r;
        const x2 = to.x - (dx / dist) * r;
        const y2 = to.y - (dy / dist) * r;

        const isLit = hoveredNode && (edge.from === hoveredNode || edge.to === hoveredNode);
        const isDim = hoveredNode && !isLit;

        return (
          <line
            key={`${edge.from}-${edge.to}`}
            x1={x1}
            y1={y1}
            x2={x2}
            y2={y2}
            stroke={isLit ? "#71717a" : "#27272a"}
            strokeWidth={isLit ? 2 : 1}
            strokeOpacity={isDim ? 0.15 : 1}
            markerEnd={isLit ? "url(#g-arrow-lit)" : "url(#g-arrow)"}
            className="transition-all duration-200"
          />
        );
      })}

      {/* Nodes */}
      {nodes.map((node) => {
        const pos = positions.get(node.id);
        if (!pos) return null;
        const sc = STATUS_COLORS[node.status || ""] || STATUS_COLORS["no-change"];
        const isHovered = hoveredNode === node.id;
        const isConnected = connectedTo.has(node.id);
        const isDim = hoveredNode && !isHovered && !isConnected;

        return (
          <g
            key={node.id}
            className="cursor-pointer"
            onClick={() => onNodeClick(node)}
            onMouseEnter={() => setHoveredNode(node.id)}
            onMouseLeave={() => setHoveredNode(null)}
            style={{ opacity: isDim ? 0.2 : 1, transition: "opacity 200ms" }}
          >
            {/* Glow ring on hover */}
            {isHovered && (
              <circle
                cx={pos.x}
                cy={pos.y}
                r={44}
                fill="none"
                stroke={sc.stroke}
                strokeWidth={1}
                strokeOpacity={0.3}
              />
            )}
            <circle
              cx={pos.x}
              cy={pos.y}
              r={38}
              fill="#0f0f11"
              stroke={sc.stroke}
              strokeWidth={isHovered ? 2.5 : 1.5}
              className="transition-all duration-150"
            />
            <text
              x={pos.x}
              y={pos.y - 4}
              textAnchor="middle"
              fill="#e4e4e7"
              style={{ fontSize: 12, fontWeight: 600 }}
            >
              {node.id}
            </text>
            {node.package && (
              <text
                x={pos.x}
                y={pos.y + 11}
                textAnchor="middle"
                fill="#52525b"
                style={{ fontSize: 8, fontFamily: "monospace" }}
              >
                {node.package}
              </text>
            )}
            {node.status && (
              <text
                x={pos.x}
                y={pos.y + 56}
                textAnchor="middle"
                fill={sc.stroke}
                style={{ fontSize: 9, fontWeight: 500 }}
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
