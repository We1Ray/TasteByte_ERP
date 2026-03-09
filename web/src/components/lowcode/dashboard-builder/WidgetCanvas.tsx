"use client";

import { useRef } from "react";
import {
  BarChart3,
  LineChart,
  PieChart,
  Activity,
  Table2,
  Trash2,
  GripVertical,
  LayoutGrid,
} from "lucide-react";
import { useTranslations } from "next-intl";
import { useDraggable, useDroppable } from "@dnd-kit/core";
import { useDashboardBuilderStore } from "@/lib/stores/dashboard-builder-store";
import { cn } from "@/lib/utils";
import type { DashboardWidget } from "@/lib/types/lowcode";

export const CELL_HEIGHT = 60;

const widgetIcons: Record<string, React.ElementType> = {
  bar: BarChart3,
  line: LineChart,
  pie: PieChart,
  kpi: Activity,
  table: Table2,
};

const widgetLabelKeys: Record<string, string> = {
  bar: "barChart",
  line: "lineChart",
  pie: "pieChart",
  kpi: "kpiCard",
  table: "dataTable",
};

// ---- Resize Handle (native pointer events, not @dnd-kit) ----

function ResizeHandle({ widget }: { widget: DashboardWidget }) {
  const { resizeWidget, gridColumns } = useDashboardBuilderStore();
  const startRef = useRef<{
    x: number;
    y: number;
    w: number;
    h: number;
  } | null>(null);

  const handlePointerDown = (e: React.PointerEvent) => {
    e.stopPropagation();
    e.preventDefault();
    (e.target as HTMLElement).setPointerCapture(e.pointerId);
    startRef.current = {
      x: e.clientX,
      y: e.clientY,
      w: widget.grid_w,
      h: widget.grid_h,
    };
  };

  const handlePointerMove = (e: React.PointerEvent) => {
    if (!startRef.current) return;
    const gridEl = (e.currentTarget as HTMLElement).closest(
      "[data-grid-container]"
    ) as HTMLElement | null;
    const cellWidth = gridEl ? gridEl.offsetWidth / gridColumns : 80;
    const cellHeight = CELL_HEIGHT;
    const dw = Math.round((e.clientX - startRef.current.x) / cellWidth);
    const dh = Math.round((e.clientY - startRef.current.y) / cellHeight);
    const newW = Math.max(
      2,
      Math.min(gridColumns - widget.grid_x, startRef.current.w + dw)
    );
    const newH = Math.max(2, startRef.current.h + dh);
    resizeWidget(widget.id, newW, newH);
  };

  const handlePointerUp = () => {
    startRef.current = null;
  };

  return (
    <div
      className="absolute bottom-0 right-0 z-10 h-4 w-4 cursor-nwse-resize opacity-0 transition-opacity group-hover:opacity-100"
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
      onPointerUp={handlePointerUp}
    >
      <svg
        viewBox="0 0 12 12"
        className="h-full w-full text-gray-400"
      >
        <path
          d="M10 2L2 10M10 6L6 10M10 10L10 10"
          stroke="currentColor"
          strokeWidth="1.5"
          fill="none"
        />
      </svg>
    </div>
  );
}

// ---- Draggable Widget Placeholder ----

function WidgetPlaceholder({ widget }: { widget: DashboardWidget }) {
  const t = useTranslations("lowcode");
  const { selectWidget, deleteWidget, selectedWidgetId } =
    useDashboardBuilderStore();

  const {
    attributes,
    listeners,
    setNodeRef,
    isDragging: isDraggingThis,
  } = useDraggable({
    id: widget.id,
    data: { type: "widget", widget },
  });

  const Icon = widgetIcons[widget.widget_type] || BarChart3;
  const isSelected = selectedWidgetId === widget.id;

  return (
    <div
      ref={setNodeRef}
      className={cn(
        "group relative flex h-full flex-col rounded-lg border-2 bg-white transition-all",
        isSelected
          ? "border-blue-500 shadow-md ring-1 ring-blue-500"
          : "border-gray-200 shadow-sm hover:border-blue-300",
        isDraggingThis && "opacity-30"
      )}
      style={{
        gridColumn: `${widget.grid_x + 1} / span ${widget.grid_w}`,
        gridRow: `${widget.grid_y + 1} / span ${widget.grid_h}`,
        minHeight: `${widget.grid_h * CELL_HEIGHT}px`,
      }}
      onClick={() => selectWidget(widget.id)}
    >
      {/* Widget header */}
      <div className="flex items-center justify-between border-b border-gray-100 px-3 py-2">
        <div className="flex items-center gap-2">
          {/* Drag handle */}
          <button
            className="cursor-grab touch-none text-gray-400 hover:text-gray-600"
            {...attributes}
            {...listeners}
          >
            <GripVertical className="h-4 w-4" />
          </button>
          <Icon className="h-4 w-4 text-gray-400" />
          <span className="truncate text-xs font-medium text-gray-700">
            {widget.title}
          </span>
        </div>
        <div className="flex items-center gap-1 opacity-0 transition-opacity group-hover:opacity-100">
          <button
            onClick={(e) => {
              e.stopPropagation();
              deleteWidget(widget.id);
            }}
            className="rounded p-1 text-gray-400 hover:bg-red-50 hover:text-red-500"
          >
            <Trash2 className="h-3 w-3" />
          </button>
        </div>
      </div>

      {/* Widget body placeholder */}
      <div className="flex flex-1 items-center justify-center p-4">
        <div className="text-center">
          <Icon className="mx-auto h-8 w-8 text-gray-200" />
          <p className="mt-2 text-xs text-gray-400">
            {t(widgetLabelKeys[widget.widget_type] ?? "dataTable")}
          </p>
          {!widget.data_source_sql && (
            <p className="mt-1 text-xs text-amber-500">
              {t("noTargetTable")}
            </p>
          )}
        </div>
      </div>

      {/* Grid position indicator */}
      <div className="border-t border-gray-100 px-3 py-1.5">
        <p className="text-[10px] text-gray-400">
          Position: ({widget.grid_x}, {widget.grid_y}) | Size:{" "}
          {widget.grid_w}x{widget.grid_h}
        </p>
      </div>

      {/* Resize handle */}
      <ResizeHandle widget={widget} />
    </div>
  );
}

// ---- Widget Canvas ----

interface WidgetCanvasProps {
  gridContainerRef?: React.MutableRefObject<HTMLDivElement | null>;
}

export function WidgetCanvas({ gridContainerRef }: WidgetCanvasProps) {
  const t = useTranslations("lowcode");
  const { widgets, gridColumns, isDragging } = useDashboardBuilderStore();

  // Make the canvas a droppable area for palette items
  const { setNodeRef: setDroppableRef } = useDroppable({
    id: "dashboard-canvas",
  });

  // Merge refs: local + external gridContainerRef
  const localGridRef = useRef<HTMLDivElement | null>(null);
  const setGridRef = (el: HTMLDivElement | null) => {
    localGridRef.current = el;
    if (gridContainerRef) {
      gridContainerRef.current = el;
    }
  };

  if (widgets.length === 0) {
    return (
      <div
        ref={setDroppableRef}
        className="flex flex-1 items-center justify-center bg-gray-100"
      >
        <div className="text-center">
          <LayoutGrid className="mx-auto h-12 w-12 text-gray-300" />
          <h3 className="mt-3 text-sm font-semibold text-gray-600">
            {t("emptyDashboard")}
          </h3>
          <p className="mt-1 text-xs text-gray-400">
            {t("dragWidgetsHint")}
          </p>
        </div>
      </div>
    );
  }

  // Calculate required rows
  const maxRow = widgets.reduce(
    (max, w) => Math.max(max, w.grid_y + w.grid_h),
    0
  );

  // Dynamic grid lines background shown during drag
  const gridLinesStyle: React.CSSProperties = isDragging
    ? {
        backgroundImage: [
          `repeating-linear-gradient(90deg, transparent, transparent calc(100% / ${gridColumns} - 1px), #e5e7eb calc(100% / ${gridColumns} - 1px), #e5e7eb calc(100% / ${gridColumns}))`,
          `repeating-linear-gradient(0deg, transparent, transparent ${CELL_HEIGHT - 1}px, #e5e7eb ${CELL_HEIGHT - 1}px, #e5e7eb ${CELL_HEIGHT}px)`,
        ].join(", "),
        backgroundSize: "100% 100%",
      }
    : {};

  return (
    <div
      ref={setDroppableRef}
      className="flex-1 overflow-auto bg-gray-100 p-6"
    >
      <div className="mx-auto max-w-6xl">
        <div className="mb-3 flex items-center justify-between">
          <span className="text-xs font-medium uppercase tracking-wider text-gray-500">
            {t("dashboardBuilder")}
          </span>
          <span className="text-xs text-gray-400">
            {gridColumns}-column grid | {widgets.length} widget
            {widgets.length !== 1 && "s"}
            {isDragging && " | Dragging..."}
          </span>
        </div>

        <div
          ref={setGridRef}
          data-grid-container
          className={cn(
            "relative rounded-lg border border-dashed border-gray-300 bg-white/50 p-2 transition-colors",
            isDragging && "border-blue-300 bg-blue-50/30"
          )}
          style={{
            display: "grid",
            gridTemplateColumns: `repeat(${gridColumns}, 1fr)`,
            gridTemplateRows: `repeat(${maxRow}, ${CELL_HEIGHT}px)`,
            gap: "8px",
            ...gridLinesStyle,
          }}
        >
          {widgets.map((widget) => (
            <WidgetPlaceholder key={widget.id} widget={widget} />
          ))}
        </div>
      </div>
    </div>
  );
}
