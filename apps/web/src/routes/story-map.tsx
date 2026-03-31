import { createFileRoute } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import { orpc } from "@/utils/orpc";
import { StoryMapGrid } from "@/components/story-map/story-map-grid";
import { SliceView } from "@/components/story-map/slice-view";
import { KanbanView } from "@/components/story-map/kanban-view";
import { LayerView } from "@/components/story-map/layer-view";
import { FilterToolbar } from "@/components/story-map/filter-toolbar";
import { SummaryStats } from "@/components/story-map/summary-stats";
import { StoryDetailPanel } from "@/components/story-map/story-detail-panel";
import { JourneyView } from "@/components/story-map/journey-view";
import { useMemo, useState, type ComponentType } from "react";
import { Grid3x3, Layers, Columns3, ListCollapse, Route as RouteIcon } from "lucide-react";

export const Route = createFileRoute("/story-map")({
  component: StoryMapPage,
});

type ViewTab = "journey" | "grid" | "slice" | "kanban" | "layer";

type TabDef = { id: ViewTab; label: string; icon: ComponentType<{ className?: string }> };

const BASE_TABS: TabDef[] = [
  { id: "grid", label: "Architecture", icon: Grid3x3 },
  { id: "slice", label: "By Slice", icon: Layers },
  { id: "kanban", label: "By Status", icon: Columns3 },
  { id: "layer", label: "By Layer", icon: ListCollapse },
];

const JOURNEY_TAB: TabDef = { id: "journey", label: "User Journey", icon: RouteIcon };

function StoryMapPage() {
  const { data, isLoading, error } = useQuery(orpc.productContext.getStoryMap.queryOptions());
  const progressQuery = useQuery(orpc.productContext.getProgress.queryOptions());
  const [selectedStoryId, setSelectedStoryId] = useState<string | null>(null);
  const [statusFilter, setStatusFilter] = useState<string[]>([]);
  const [layerFilter, setLayerFilter] = useState<number[]>([]);

  const hasActivities = (data?.activities?.length ?? 0) > 0;
  const tabs = useMemo(
    () => (hasActivities ? [JOURNEY_TAB, ...BASE_TABS] : BASE_TABS),
    [hasActivities],
  );
  const [activeTab, setActiveTab] = useState<ViewTab>(hasActivities ? "journey" : "grid");

  if (isLoading) {
    return (
      <div className="flex h-full items-center justify-center">
        <div className="flex flex-col items-center gap-3">
          <div className="h-6 w-6 animate-spin rounded-full border-2 border-zinc-700 border-t-emerald-400" />
          <p className="text-xs text-zinc-500 tracking-wider uppercase">Loading story map</p>
        </div>
      </div>
    );
  }

  if (error || !data) {
    return (
      <div className="flex h-full items-center justify-center">
        <div className="text-center">
          <div className="mx-auto mb-3 flex h-10 w-10 items-center justify-center rounded-full bg-red-950/50 text-red-400">
            !
          </div>
          <p className="text-sm font-medium text-zinc-300">Failed to load product context</p>
          <p className="mt-1 text-xs text-zinc-500">
            {error?.message || "Check that PRODUCT_CONTEXT_PATH is set correctly"}
          </p>
        </div>
      </div>
    );
  }

  const selectedCard = data.cards.find((c) => c.storyId === selectedStoryId);
  const allModules = data.backbone.map((m) => m.id);

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
    <div className="flex h-full flex-col bg-[#0a0a0b]">
      {/* Summary stats bar */}
      <SummaryStats cards={data.cards} dispatchEpoch={progressQuery.data?.dispatchEpoch} />

      {/* Tab bar + filters */}
      <div className="flex items-center border-b border-zinc-800/60 bg-zinc-950/60">
        {/* Tabs */}
        <div className="flex">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              type="button"
              onClick={() => setActiveTab(tab.id)}
              className={`relative flex items-center gap-1.5 px-4 py-2.5 text-xs font-medium transition-colors ${
                activeTab === tab.id ? "text-zinc-100" : "text-zinc-500 hover:text-zinc-300"
              }`}
            >
              <tab.icon className="h-3.5 w-3.5" />
              {tab.label}
              {activeTab === tab.id && (
                <span className="absolute bottom-0 left-2 right-2 h-[2px] rounded-full bg-emerald-400" />
              )}
            </button>
          ))}
        </div>

        {/* Divider */}
        <div className="mx-2 h-4 w-px bg-zinc-800" />

        {/* Filters */}
        <FilterToolbar
          statuses={[...new Set(data.cards.map((c) => c.status))]}
          layers={data.lanes}
          statusFilter={statusFilter}
          layerFilter={layerFilter}
          onStatusFilterChange={setStatusFilter}
          onLayerFilterChange={setLayerFilter}
        />
      </div>

      {/* View content */}
      <div className="flex-1 overflow-auto">
        {activeTab === "journey" && hasActivities && (
          <JourneyView
            cards={filteredCards}
            activities={data.activities}
            lanes={data.lanes}
            allModules={allModules}
            onCardClick={(storyId) => setSelectedStoryId(storyId)}
            selectedStoryId={selectedStoryId}
          />
        )}

        {activeTab === "grid" && (
          <div className="p-4">
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
        )}

        {activeTab === "slice" && (
          <SliceView
            cards={filteredCards}
            slices={data.slices}
            allModules={allModules}
            onCardClick={(storyId) => setSelectedStoryId(storyId)}
            selectedStoryId={selectedStoryId}
          />
        )}

        {activeTab === "kanban" && (
          <KanbanView
            cards={filteredCards}
            allModules={allModules}
            onCardClick={(storyId) => setSelectedStoryId(storyId)}
            selectedStoryId={selectedStoryId}
          />
        )}

        {activeTab === "layer" && (
          <LayerView
            cards={filteredCards}
            lanes={data.lanes}
            allModules={allModules}
            onCardClick={(storyId) => setSelectedStoryId(storyId)}
            selectedStoryId={selectedStoryId}
          />
        )}
      </div>

      {/* Detail panel */}
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
