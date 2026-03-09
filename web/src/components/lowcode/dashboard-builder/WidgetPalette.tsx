"use client";

import {
  BarChart3,
  LineChart,
  PieChart,
  Activity,
  Table2,
} from "lucide-react";
import { useTranslations } from "next-intl";
import { useDraggable } from "@dnd-kit/core";
import { useDashboardBuilderStore } from "@/lib/stores/dashboard-builder-store";
import type { DashboardWidget } from "@/lib/types/lowcode";

interface WidgetTypeOption {
  type: DashboardWidget["widget_type"];
  labelKey: string;
  descKey: string;
  icon: React.ElementType;
  defaultConfig: Partial<DashboardWidget>;
}

export const widgetTypeOptions: WidgetTypeOption[] = [
  {
    type: "bar",
    labelKey: "barChart",
    descKey: "barChartDesc",
    icon: BarChart3,
    defaultConfig: {
      grid_w: 6,
      grid_h: 4,
      colors: ["#2563eb", "#7c3aed"],
      series_config: [{ dataKey: "value", color: "#2563eb", name: "Value" }],
    },
  },
  {
    type: "line",
    labelKey: "lineChart",
    descKey: "lineChartDesc",
    icon: LineChart,
    defaultConfig: {
      grid_w: 6,
      grid_h: 4,
      colors: ["#2563eb", "#10b981"],
      series_config: [{ dataKey: "value", color: "#2563eb", name: "Value" }],
    },
  },
  {
    type: "pie",
    labelKey: "pieChart",
    descKey: "pieChartDesc",
    icon: PieChart,
    defaultConfig: {
      grid_w: 4,
      grid_h: 4,
      colors: ["#2563eb", "#7c3aed", "#10b981", "#f59e0b", "#ef4444"],
      series_config: [],
    },
  },
  {
    type: "kpi",
    labelKey: "kpiCard",
    descKey: "kpiCardDesc",
    icon: Activity,
    defaultConfig: {
      grid_w: 3,
      grid_h: 2,
      colors: ["#2563eb"],
      series_config: [],
    },
  },
  {
    type: "table",
    labelKey: "dataTable",
    descKey: "dataTableDesc",
    icon: Table2,
    defaultConfig: {
      grid_w: 6,
      grid_h: 4,
      colors: [],
      series_config: [],
    },
  },
];

// ---- Draggable Palette Item ----

function DraggablePaletteItem({ option }: { option: WidgetTypeOption }) {
  const t = useTranslations("lowcode");
  const { addWidget, widgets } = useDashboardBuilderStore();

  const label = t(option.labelKey);

  const {
    attributes,
    listeners,
    setNodeRef,
    isDragging,
  } = useDraggable({
    id: `palette-${option.type}`,
    data: {
      type: "palette",
      widgetType: option.type,
      label,
      defaultConfig: option.defaultConfig,
    },
  });

  const Icon = option.icon;

  // Click-to-add fallback
  const handleClick = () => {
    const maxY = widgets.reduce(
      (max, w) => Math.max(max, w.grid_y + w.grid_h),
      0
    );

    addWidget({
      title: `New ${label}`,
      widget_type: option.type,
      data_source_sql: "",
      x_axis_key: "",
      y_axis_key: "",
      series_config: option.defaultConfig.series_config || [],
      colors: option.defaultConfig.colors || [],
      grid_x: 0,
      grid_y: maxY,
      grid_w: option.defaultConfig.grid_w || 6,
      grid_h: option.defaultConfig.grid_h || 4,
      widget_config: {},
      sort_order: widgets.length,
    });
  };

  return (
    <button
      ref={setNodeRef}
      onClick={handleClick}
      className={`flex w-full cursor-grab items-start gap-3 rounded-md border border-gray-200 bg-white p-3 text-left transition-colors hover:border-blue-300 hover:bg-blue-50 ${isDragging ? "opacity-50" : ""}`}
      {...attributes}
      {...listeners}
    >
      <div className="rounded-md bg-gray-100 p-2">
        <Icon className="h-5 w-5 text-gray-600" />
      </div>
      <div className="min-w-0">
        <p className="text-sm font-medium text-gray-700">{label}</p>
        <p className="text-xs text-gray-400">{t(option.descKey)}</p>
      </div>
    </button>
  );
}

// ---- Widget Palette ----

export function WidgetPalette() {
  const t = useTranslations("lowcode");
  return (
    <div className="flex h-full w-60 flex-col border-r border-gray-200 bg-gray-50">
      <div className="border-b border-gray-200 px-4 py-3">
        <h3 className="text-sm font-semibold text-gray-900">{t("widgetTypes")}</h3>
        <p className="text-xs text-gray-500">{t("dragWidgetsHint")}</p>
      </div>

      <div className="flex-1 overflow-y-auto px-3 py-3">
        <div className="space-y-2">
          {widgetTypeOptions.map((option) => (
            <DraggablePaletteItem key={option.type} option={option} />
          ))}
        </div>
      </div>
    </div>
  );
}
