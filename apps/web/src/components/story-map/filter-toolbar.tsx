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
    <div className="flex flex-1 flex-wrap items-center gap-3 px-2 py-1">
      <div className="flex items-center gap-1">
        <span className="text-[10px] text-zinc-600 uppercase tracking-wider mr-1">Status</span>
        {statuses.sort().map((status) => {
          const colors = getStatusColor(status);
          const isActive = statusFilter.includes(status);
          return (
            <button
              key={status}
              type="button"
              onClick={() => toggleStatus(status)}
              className={`flex items-center gap-1 rounded-full border px-1.5 py-0.5 text-[10px] transition-all ${
                isActive
                  ? `${colors.border} ${colors.bg}`
                  : hasFilters
                    ? "border-transparent opacity-30 hover:opacity-60"
                    : "border-transparent opacity-70 hover:opacity-100"
              }`}
            >
              <span className={`h-1.5 w-1.5 rounded-full ${colors.dot}`} />
              <span className="text-zinc-400">{status.replace(/_/g, " ")}</span>
            </button>
          );
        })}
      </div>
      <div className="h-3 w-px bg-zinc-800" />
      <div className="flex items-center gap-1">
        <span className="text-[10px] text-zinc-600 uppercase tracking-wider mr-1">Layer</span>
        {layers.map((lane) => {
          const layerNum = Number(lane.id.match(/\d+/)?.[0] ?? 0);
          const isActive = layerFilter.includes(layerNum);
          return (
            <button
              key={lane.id}
              type="button"
              onClick={() => toggleLayer(layerNum)}
              className={`rounded border px-1.5 py-0.5 text-[10px] transition-all ${
                isActive
                  ? "border-zinc-600 bg-zinc-800 text-zinc-200"
                  : hasFilters
                    ? "border-transparent text-zinc-500 opacity-30 hover:opacity-60"
                    : "border-transparent text-zinc-500 opacity-70 hover:opacity-100"
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
          className="text-[10px] text-zinc-500 underline decoration-zinc-700 hover:text-zinc-300"
        >
          Clear
        </button>
      )}
    </div>
  );
}
