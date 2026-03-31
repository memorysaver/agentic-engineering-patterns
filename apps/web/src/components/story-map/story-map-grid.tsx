import { useRef, useEffect, useState, useCallback, Fragment } from "react";
import { StoryCard } from "./story-card";

type BackboneItem = { id: string; name: string; package?: string; status?: string };
type Lane = { id: string; name: string; theme?: string; capabilities?: string[] };
type Card = {
  storyId: string;
  title: string;
  moduleId: string;
  layer: number;
  slice: number;
  status: string;
  complexity?: string;
  priority?: string;
  businessValue?: string;
  dependencies: string[];
  prUrl?: string | null;
  completedAt?: string | null;
  attemptCount: number;
};
type Edge = { from: string; to: string; fromStatus: string };
type Slice = { slice: number; theme: string; storyIds: string[] };

type StoryMapGridProps = {
  backbone: BackboneItem[];
  lanes: Lane[];
  cards: Card[];
  edges: Edge[];
  slices: Slice[];
  onCardClick: (storyId: string) => void;
  selectedStoryId: string | null;
};

const SLICE_COLORS = [
  "rgba(59, 130, 246, 0.06)",
  "rgba(168, 85, 247, 0.06)",
  "rgba(34, 197, 94, 0.06)",
  "rgba(245, 158, 11, 0.06)",
  "rgba(239, 68, 68, 0.06)",
  "rgba(20, 184, 166, 0.06)",
];

const STATUS_ARROW_COLORS: Record<string, string> = {
  completed: "#22c55e",
  done: "#22c55e",
  in_progress: "#f59e0b",
  in_review: "#a855f7",
  review: "#a855f7",
  ready: "#3b82f6",
  pending: "#71717a",
  blocked: "#ef4444",
  failed: "#dc2626",
  deferred: "#52525b",
};

export function StoryMapGrid({
  backbone,
  lanes,
  cards,
  edges,
  slices,
  onCardClick,
  selectedStoryId,
}: StoryMapGridProps) {
  const gridRef = useRef<HTMLDivElement>(null);
  const svgRef = useRef<SVGSVGElement>(null);
  const [arrowPaths, setArrowPaths] = useState<
    Array<{ d: string; color: string; from: string; to: string }>
  >([]);
  const [hoveredEdge, setHoveredEdge] = useState<string | null>(null);

  // Unique layers from cards, sorted
  const uniqueLayers = [...new Set(cards.map((c) => c.layer))].sort((a, b) => a - b);

  // Map layer number to lane info
  const layerToLane = new Map(
    lanes.map((l) => {
      const layerNum = l.id.match(/\d+/)?.[0];
      return [Number(layerNum ?? 0), l];
    }),
  );

  // Get module columns — filter to only modules that have stories
  const modulesWithStories = backbone.filter((m) =>
    cards.some((c) => c.moduleId === m.id || c.moduleId === m.package),
  );

  const updateArrows = useCallback(() => {
    if (!gridRef.current || !svgRef.current) return;
    const gridRect = gridRef.current.getBoundingClientRect();
    const paths: Array<{ d: string; color: string; from: string; to: string }> = [];

    for (const edge of edges) {
      const fromEl = gridRef.current.querySelector(`[data-story-id="${edge.from}"]`);
      const toEl = gridRef.current.querySelector(`[data-story-id="${edge.to}"]`);
      if (!fromEl || !toEl) continue;

      const fromRect = fromEl.getBoundingClientRect();
      const toRect = toEl.getBoundingClientRect();

      const x1 = fromRect.left + fromRect.width / 2 - gridRect.left;
      const y1 = fromRect.bottom - gridRect.top;
      const x2 = toRect.left + toRect.width / 2 - gridRect.left;
      const y2 = toRect.top - gridRect.top;

      const midY = (y1 + y2) / 2;
      const d = `M ${x1} ${y1} C ${x1} ${midY}, ${x2} ${midY}, ${x2} ${y2}`;
      const color = STATUS_ARROW_COLORS[edge.fromStatus] || "#71717a";

      paths.push({ d, color, from: edge.from, to: edge.to });
    }
    setArrowPaths(paths);
  }, [edges]);

  useEffect(() => {
    const timer = setTimeout(updateArrows, 100);
    return () => clearTimeout(timer);
  }, [updateArrows, cards]);

  useEffect(() => {
    const observer = new ResizeObserver(updateArrows);
    if (gridRef.current) observer.observe(gridRef.current);
    return () => observer.disconnect();
  }, [updateArrows]);

  // Determine which story IDs are highlighted by hover
  const highlightedStories = new Set<string>();
  if (hoveredEdge) {
    const [from, to] = hoveredEdge.split("|");
    highlightedStories.add(from);
    highlightedStories.add(to);
  }

  return (
    <div className="relative" ref={gridRef}>
      {/* SVG overlay for dependency arrows */}
      <svg
        ref={svgRef}
        className="pointer-events-none absolute inset-0 z-10"
        style={{ width: "100%", height: "100%" }}
      >
        <defs>
          <marker id="arrowhead" markerWidth="8" markerHeight="6" refX="8" refY="3" orient="auto">
            <polygon points="0 0, 8 3, 0 6" fill="currentColor" />
          </marker>
        </defs>
        {arrowPaths.map((arrow) => {
          const edgeKey = `${arrow.from}|${arrow.to}`;
          const isHovered = hoveredEdge === edgeKey;
          return (
            <path
              key={edgeKey}
              d={arrow.d}
              fill="none"
              stroke={arrow.color}
              strokeWidth={isHovered ? 2.5 : 1.5}
              strokeOpacity={isHovered ? 1 : 0.4}
              markerEnd="url(#arrowhead)"
              style={{ color: arrow.color, pointerEvents: "stroke", cursor: "pointer" }}
              onMouseEnter={() => setHoveredEdge(edgeKey)}
              onMouseLeave={() => setHoveredEdge(null)}
            />
          );
        })}
      </svg>

      {/* Grid layout */}
      <div
        className="grid gap-0"
        style={{
          gridTemplateColumns: `120px repeat(${modulesWithStories.length}, minmax(200px, 1fr))`,
        }}
      >
        {/* Header row — empty corner + module names */}
        <div className="border-b border-r p-2" />
        {modulesWithStories.map((m) => (
          <div key={m.id} className="border-b border-r p-2 text-center">
            <p className="text-xs font-semibold">{m.id}</p>
            {m.package && (
              <p className="text-muted-foreground text-[10px] font-mono">{m.package}</p>
            )}
          </div>
        ))}

        {/* Layer rows */}
        {uniqueLayers.map((layerNum) => {
          const lane = layerToLane.get(layerNum);
          const layerCards = cards.filter((c) => c.layer === layerNum);

          // Find which slice(s) this layer belongs to
          const layerSlice = slices.find((s) =>
            s.storyIds.some((sid) => layerCards.some((c) => c.storyId === sid)),
          );
          const sliceBg = layerSlice
            ? SLICE_COLORS[(layerSlice.slice - 1) % SLICE_COLORS.length]
            : "transparent";

          return (
            <Fragment key={layerNum}>
              {/* Layer label */}
              <div className="border-b border-r p-2" style={{ backgroundColor: sliceBg }}>
                <p className="text-xs font-bold">Layer {layerNum}</p>
                <p className="text-muted-foreground text-[10px]">{lane?.name || ""}</p>
                {lane?.theme && (
                  <p className="text-muted-foreground mt-0.5 text-[10px] italic">{lane.theme}</p>
                )}
              </div>

              {/* Module cells for this layer */}
              {modulesWithStories.map((m) => {
                const cellCards = layerCards.filter(
                  (c) => c.moduleId === m.id || c.moduleId === m.package,
                );
                return (
                  <div
                    key={`${layerNum}-${m.id}`}
                    className="border-b border-r p-2"
                    style={{
                      backgroundColor: sliceBg,
                      minHeight: 80,
                    }}
                  >
                    <div className="flex flex-wrap gap-2">
                      {cellCards.map((card) => (
                        <StoryCard
                          key={card.storyId}
                          storyId={card.storyId}
                          title={card.title}
                          status={card.status}
                          complexity={card.complexity}
                          slice={card.slice}
                          prUrl={card.prUrl}
                          isSelected={
                            card.storyId === selectedStoryId || highlightedStories.has(card.storyId)
                          }
                          onClick={() => onCardClick(card.storyId)}
                        />
                      ))}
                    </div>
                  </div>
                );
              })}
            </Fragment>
          );
        })}
      </div>

      {/* Slice legend */}
      {slices.length > 0 && (
        <div className="mt-4 flex flex-wrap gap-3 border-t pt-3">
          <span className="text-muted-foreground text-xs font-medium">Execution Slices:</span>
          {slices.map((s, i) => (
            <span key={s.slice} className="flex items-center gap-1 text-xs">
              <span
                className="inline-block h-3 w-3 rounded border"
                style={{ backgroundColor: SLICE_COLORS[i % SLICE_COLORS.length] }}
              />
              Slice {s.slice}: {s.theme} ({s.storyIds.length})
            </span>
          ))}
        </div>
      )}
    </div>
  );
}
