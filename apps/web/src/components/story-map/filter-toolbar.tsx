import { getStatusColor } from "./status-colors";

type FilterToolbarProps = {
  statuses: string[];
  layers: Array<{ id: string; name: string }>;
  statusFilter: string[];
  layerFilter: number[];
  onStatusFilterChange: (statuses: string[]) => void;
  onLayerFilterChange: (layers: number[]) => void;
};

export function FilterToolbar({
  statuses,
  layers,
  statusFilter,
  layerFilter,
  onStatusFilterChange,
  onLayerFilterChange,
}: FilterToolbarProps) {
  const toggleStatus = (status: string) => {
    if (statusFilter.includes(status)) {
      onStatusFilterChange(statusFilter.filter((s) => s !== status));
    } else {
      onStatusFilterChange([...statusFilter, status]);
    }
  };

  const toggleLayer = (layer: number) => {
    if (layerFilter.includes(layer)) {
      onLayerFilterChange(layerFilter.filter((l) => l !== layer));
    } else {
      onLayerFilterChange([...layerFilter, layer]);
    }
  };

  const hasFilters = statusFilter.length > 0 || layerFilter.length > 0;

  return (
    <div className="flex flex-wrap items-center gap-4 border-b px-4 py-2">
      <div className="flex items-center gap-1.5">
        <span className="text-muted-foreground text-xs">Status:</span>
        {statuses.sort().map((status) => {
          const colors = getStatusColor(status);
          const isActive = statusFilter.includes(status);
          return (
            <button
              key={status}
              type="button"
              onClick={() => toggleStatus(status)}
              className={`flex items-center gap-1 rounded-full border px-2 py-0.5 text-[10px] transition-opacity ${
                !hasFilters || isActive ? "opacity-100" : "opacity-40"
              } ${colors.border}`}
            >
              <span className={`h-1.5 w-1.5 rounded-full ${colors.dot}`} />
              {status.replace(/_/g, " ")}
            </button>
          );
        })}
      </div>
      <div className="flex items-center gap-1.5">
        <span className="text-muted-foreground text-xs">Layer:</span>
        {layers.map((lane) => {
          const layerNum = Number(lane.id.match(/\d+/)?.[0] ?? 0);
          const isActive = layerFilter.includes(layerNum);
          return (
            <button
              key={lane.id}
              type="button"
              onClick={() => toggleLayer(layerNum)}
              className={`rounded-full border px-2 py-0.5 text-[10px] transition-opacity ${
                !hasFilters || isActive ? "opacity-100" : "opacity-40"
              }`}
            >
              {lane.name}
            </button>
          );
        })}
      </div>
      {hasFilters && (
        <button
          type="button"
          onClick={() => {
            onStatusFilterChange([]);
            onLayerFilterChange([]);
          }}
          className="text-muted-foreground text-[10px] underline"
        >
          Clear filters
        </button>
      )}
    </div>
  );
}
