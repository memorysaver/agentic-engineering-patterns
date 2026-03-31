import { createFileRoute } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import { orpc } from "@/utils/orpc";
import { StoryMapGrid } from "@/components/story-map/story-map-grid";
import { FilterToolbar } from "@/components/story-map/filter-toolbar";
import { StatusLegend } from "@/components/story-map/status-legend";
import { StoryDetailPanel } from "@/components/story-map/story-detail-panel";
import { useState } from "react";

export const Route = createFileRoute("/story-map")({
  component: StoryMapPage,
});

function StoryMapPage() {
  const { data, isLoading, error } = useQuery(orpc.productContext.getStoryMap.queryOptions());
  const [selectedStoryId, setSelectedStoryId] = useState<string | null>(null);
  const [statusFilter, setStatusFilter] = useState<string[]>([]);
  const [layerFilter, setLayerFilter] = useState<number[]>([]);

  if (isLoading) {
    return (
      <div className="flex h-full items-center justify-center">
        <p className="text-muted-foreground">Loading story map...</p>
      </div>
    );
  }

  if (error || !data) {
    return (
      <div className="flex h-full items-center justify-center">
        <div className="text-center">
          <p className="text-destructive font-medium">Failed to load product context</p>
          <p className="text-muted-foreground text-sm mt-1">
            {error?.message || "Check that PRODUCT_CONTEXT_PATH is set correctly"}
          </p>
        </div>
      </div>
    );
  }

  const selectedCard = data.cards.find((c) => c.storyId === selectedStoryId);

  const filteredCards = data.cards.filter((card) => {
    if (statusFilter.length > 0 && !statusFilter.includes(card.status)) return false;
    if (layerFilter.length > 0 && !layerFilter.includes(card.layer)) return false;
    return true;
  });

  const filteredEdges = data.edges.filter(
    (edge) =>
      filteredCards.some((c) => c.storyId === edge.from) &&
      filteredCards.some((c) => c.storyId === edge.to),
  );

  return (
    <div className="flex h-full flex-col">
      <div className="flex items-center justify-between border-b px-4 py-2">
        <div>
          <h2 className="text-lg font-semibold">User Story Map</h2>
          <p className="text-muted-foreground text-xs">
            {data.cards.length} stories across {data.backbone.length} modules and{" "}
            {data.lanes.length} layers
          </p>
        </div>
        <StatusLegend />
      </div>
      <FilterToolbar
        statuses={[...new Set(data.cards.map((c) => c.status))]}
        layers={data.lanes}
        statusFilter={statusFilter}
        layerFilter={layerFilter}
        onStatusFilterChange={setStatusFilter}
        onLayerFilterChange={setLayerFilter}
      />
      <div className="flex-1 overflow-auto p-4">
        <StoryMapGrid
          backbone={data.backbone}
          lanes={data.lanes}
          cards={filteredCards}
          edges={filteredEdges}
          slices={data.slices}
          onCardClick={(storyId) => setSelectedStoryId(storyId)}
          selectedStoryId={selectedStoryId}
        />
      </div>
      {selectedCard && (
        <StoryDetailPanel
          card={selectedCard}
          allCards={data.cards}
          open={!!selectedStoryId}
          onClose={() => setSelectedStoryId(null)}
        />
      )}
    </div>
  );
}
