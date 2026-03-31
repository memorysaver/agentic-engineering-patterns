import { getStatusColor, STATUS_ORDER } from "./status-colors";
import { SlidersHorizontal, X } from "lucide-react";

type FilterToolbarProps = {
  statuses: string[];
  layers: Array<{ id: string; name: string }>;
  statusFilter: string[];
  layerFilter: number[];
  onStatusFilterChange: (statuses: string[]) => void;
  onLayerFilterChange: (layers: number[]) => void;
};

const VISIBLE_STATUSES = STATUS_ORDER.filter((s) => !["done", "review"].includes(s));

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
  const filterCount = statusFilter.length + layerFilter.length;

  return (
    <div className="flex flex-1 items-center gap-2">
      <SlidersHorizontal className="h-3 w-3 text-zinc-600" />

      {/* Status dots */}
      <div className="flex items-center gap-0.5 rounded-md bg-zinc-900/60 px-1.5 py-1">
        {VISIBLE_STATUSES.map((status) => {
          const colors = getStatusColor(status);
          const isActive = statusFilter.includes(status);
          const exists = statuses.includes(status);
          return (
            <button
              key={status}
              type="button"
              onClick={() => toggleStatus(status)}
              title={status.replace(/_/g, " ")}
              className={`group relative flex h-5 w-5 items-center justify-center rounded transition-all ${
                isActive ? "bg-zinc-700/80" : "hover:bg-zinc-800/60"
              }`}
            >
              <span
                className={`rounded-full transition-all ${colors.dot} ${
                  isActive
                    ? "h-2.5 w-2.5"
                    : exists
                      ? "h-[6px] w-[6px] group-hover:h-2 group-hover:w-2"
                      : "h-[4px] w-[4px] opacity-25"
                }`}
              />
              {/* Tooltip */}
              <span className="pointer-events-none absolute -bottom-6 left-1/2 -translate-x-1/2 whitespace-nowrap rounded bg-zinc-800 px-1.5 py-0.5 text-[9px] text-zinc-300 opacity-0 shadow-lg transition-opacity group-hover:opacity-100">
                {status.replace(/_/g, " ")}
              </span>
            </button>
          );
        })}
      </div>

      {/* Layer pills */}
      <div className="flex items-center gap-0.5 rounded-md bg-zinc-900/60 px-1 py-0.5">
        {layers.map((lane) => {
          const layerNum = Number(lane.id.match(/\d+/)?.[0] ?? 0);
          const isActive = layerFilter.includes(layerNum);
          return (
            <button
              key={lane.id}
              type="button"
              onClick={() => toggleLayer(layerNum)}
              className={`rounded px-1.5 py-0.5 text-[10px] font-medium transition-all ${
                isActive
                  ? "bg-zinc-700/80 text-zinc-200"
                  : hasFilters
                    ? "text-zinc-600 hover:text-zinc-400"
                    : "text-zinc-500 hover:bg-zinc-800/50 hover:text-zinc-300"
              }`}
            >
              L{layerNum}
            </button>
          );
        })}
      </div>

      {/* Active filter count + clear */}
      {hasFilters && (
        <button
          type="button"
          onClick={() => {
            onStatusFilterChange([]);
            onLayerFilterChange([]);
          }}
          className="flex items-center gap-1 rounded-md bg-zinc-800/80 px-1.5 py-0.5 text-[10px] text-zinc-400 transition-colors hover:bg-zinc-700/80 hover:text-zinc-200"
        >
          <span>{filterCount}</span>
          <X className="h-2.5 w-2.5" />
        </button>
      )}
    </div>
  );
}
