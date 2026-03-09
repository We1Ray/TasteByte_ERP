"use client";

import { Plus, Trash2 } from "lucide-react";
import { useTranslations } from "next-intl";
import { Input } from "@/components/ui/input";
import { SqlEditor } from "@/components/lowcode/form-builder/SqlEditor";
import { useDashboardBuilderStore } from "@/lib/stores/dashboard-builder-store";
import type { DashboardWidget } from "@/lib/types/lowcode";

const widgetTypeLabelKeys: Record<string, string> = {
  bar: "barChart",
  line: "lineChart",
  pie: "pieChart",
  kpi: "kpiCard",
  table: "dataTable",
};

export function WidgetPropertyPanel() {
  const t = useTranslations("lowcode");
  const { widgets, selectedWidgetId, updateWidget } =
    useDashboardBuilderStore();

  const selectedWidget = selectedWidgetId
    ? widgets.find((w) => w.id === selectedWidgetId)
    : null;

  if (!selectedWidget) {
    return (
      <div className="flex h-full w-72 flex-col border-l border-gray-200 bg-gray-50">
        <div className="border-b border-gray-200 px-4 py-3">
          <h3 className="text-sm font-semibold text-gray-900">
            {t("widgetProperties")}
          </h3>
        </div>
        <div className="flex flex-1 items-center justify-center px-4">
          <p className="text-center text-sm text-gray-500">
            {t("selectWidgetHint")}
          </p>
        </div>
      </div>
    );
  }

  const update = (updates: Partial<DashboardWidget>) =>
    updateWidget(selectedWidget.id, updates);

  const showAxisKeys =
    selectedWidget.widget_type === "bar" ||
    selectedWidget.widget_type === "line";
  const showSeries =
    selectedWidget.widget_type === "bar" ||
    selectedWidget.widget_type === "line";
  const showColors =
    selectedWidget.widget_type === "pie" ||
    selectedWidget.widget_type === "kpi";

  return (
    <div className="flex h-full w-72 flex-col border-l border-gray-200 bg-gray-50">
      <div className="border-b border-gray-200 px-4 py-3">
        <h3 className="text-sm font-semibold text-gray-900">
          {t("widgetProperties")}
        </h3>
        <p className="text-xs text-gray-500">
          {t(widgetTypeLabelKeys[selectedWidget.widget_type] ?? "dataTable")}
        </p>
      </div>
      <div className="flex-1 overflow-y-auto px-4 py-4">
        <div className="space-y-4">
          <Input
            label={t("widgetTitle")}
            value={selectedWidget.title}
            onChange={(e) => update({ title: e.target.value })}
          />

          <div className="rounded-md bg-gray-100 p-3">
            <p className="text-xs font-medium text-gray-500">{t("widgetType")}</p>
            <p className="text-sm text-gray-700">
              {t(widgetTypeLabelKeys[selectedWidget.widget_type] ?? "dataTable")}
            </p>
          </div>

          <hr className="border-gray-200" />
          <h4 className="text-sm font-semibold text-gray-900">{t("dataSource")}</h4>

          <SqlEditor
            value={selectedWidget.data_source_sql}
            onChange={(sql) => update({ data_source_sql: sql })}
            label={t("widgetSqlQuery")}
          />

          {showAxisKeys && (
            <>
              <hr className="border-gray-200" />
              <h4 className="text-sm font-semibold text-gray-900">
                {t("xAxisColumn")} / {t("yAxisColumn")}
              </h4>
              <Input
                label={t("xAxisColumn")}
                value={selectedWidget.x_axis_key ?? ""}
                onChange={(e) => update({ x_axis_key: e.target.value })}
              />
              <Input
                label={t("yAxisColumn")}
                value={selectedWidget.y_axis_key ?? ""}
                onChange={(e) => update({ y_axis_key: e.target.value })}
              />
            </>
          )}

          {showSeries && (
            <>
              <hr className="border-gray-200" />
              <h4 className="text-sm font-semibold text-gray-900">
                {t("dataSource")}
              </h4>
              <SeriesEditor
                series={selectedWidget.series_config}
                onChange={(series) => update({ series_config: series })}
              />
            </>
          )}

          {showColors && (
            <>
              <hr className="border-gray-200" />
              <h4 className="text-sm font-semibold text-gray-900">{t("color")}</h4>
              <ColorsEditor
                colors={selectedWidget.colors}
                onChange={(colors) => update({ colors })}
              />
            </>
          )}

          <hr className="border-gray-200" />
          <h4 className="text-sm font-semibold text-gray-900">
            {t("layout")}
          </h4>

          <div className="grid grid-cols-2 gap-3">
            <Input
              label="X"
              type="number"
              value={selectedWidget.grid_x}
              onChange={(e) =>
                update({ grid_x: Math.max(0, Number(e.target.value)) })
              }
            />
            <Input
              label="Y"
              type="number"
              value={selectedWidget.grid_y}
              onChange={(e) =>
                update({ grid_y: Math.max(0, Number(e.target.value)) })
              }
            />
            <Input
              label={t("width")}
              type="number"
              value={selectedWidget.grid_w}
              onChange={(e) =>
                update({ grid_w: Math.max(1, Number(e.target.value)) })
              }
            />
            <Input
              label={t("columns")}
              type="number"
              value={selectedWidget.grid_h}
              onChange={(e) =>
                update({ grid_h: Math.max(1, Number(e.target.value)) })
              }
            />
          </div>
        </div>
      </div>
    </div>
  );
}

// ---- Series Editor ----

function SeriesEditor({
  series,
  onChange,
}: {
  series: { dataKey: string; color: string; name?: string }[];
  onChange: (series: { dataKey: string; color: string; name?: string }[]) => void;
}) {
  const t = useTranslations("lowcode");
  const defaultColors = [
    "#2563eb",
    "#7c3aed",
    "#10b981",
    "#f59e0b",
    "#ef4444",
    "#06b6d4",
  ];

  const addSeries = () => {
    onChange([
      ...series,
      {
        dataKey: "",
        color: defaultColors[series.length % defaultColors.length],
        name: "",
      },
    ]);
  };

  const updateSeries = (
    index: number,
    updates: Partial<{ dataKey: string; color: string; name: string }>
  ) => {
    const newSeries = [...series];
    newSeries[index] = { ...newSeries[index], ...updates };
    onChange(newSeries);
  };

  const removeSeries = (index: number) => {
    onChange(series.filter((_, i) => i !== index));
  };

  return (
    <div className="space-y-2">
      {series.map((s, i) => (
        <div
          key={i}
          className="rounded-md border border-gray-200 bg-white p-3 space-y-2"
        >
          <div className="flex items-center justify-between">
            <span className="text-xs font-medium text-gray-500">
              Series {i + 1}
            </span>
            <button
              onClick={() => removeSeries(i)}
              className="text-gray-400 hover:text-red-500"
            >
              <Trash2 className="h-3 w-3" />
            </button>
          </div>
          <Input
            value={s.dataKey}
            onChange={(e) => updateSeries(i, { dataKey: e.target.value })}
            placeholder="Data key (column name)"
            className="text-xs"
          />
          <Input
            value={s.name ?? ""}
            onChange={(e) => updateSeries(i, { name: e.target.value })}
            placeholder="Display name"
            className="text-xs"
          />
          <div className="flex items-center gap-2">
            <input
              type="color"
              value={s.color}
              onChange={(e) => updateSeries(i, { color: e.target.value })}
              className="h-7 w-7 cursor-pointer rounded border border-gray-200"
            />
            <Input
              value={s.color}
              onChange={(e) => updateSeries(i, { color: e.target.value })}
              className="text-xs"
            />
          </div>
        </div>
      ))}
      <button
        onClick={addSeries}
        className="flex w-full items-center justify-center gap-1 rounded-md border border-dashed border-gray-300 px-3 py-2 text-xs text-gray-500 hover:border-blue-400 hover:text-blue-600"
      >
        <Plus className="h-3 w-3" />
        {t("addColumn")}
      </button>
    </div>
  );
}

// ---- Colors Editor ----

function ColorsEditor({
  colors,
  onChange,
}: {
  colors: string[];
  onChange: (colors: string[]) => void;
}) {
  const t = useTranslations("lowcode");
  const addColor = () => {
    const defaults = [
      "#2563eb",
      "#7c3aed",
      "#10b981",
      "#f59e0b",
      "#ef4444",
      "#06b6d4",
      "#8b5cf6",
      "#ec4899",
    ];
    onChange([...colors, defaults[colors.length % defaults.length]]);
  };

  const updateColor = (index: number, color: string) => {
    const newColors = [...colors];
    newColors[index] = color;
    onChange(newColors);
  };

  const removeColor = (index: number) => {
    onChange(colors.filter((_, i) => i !== index));
  };

  return (
    <div className="space-y-2">
      {colors.map((color, i) => (
        <div key={i} className="flex items-center gap-2">
          <input
            type="color"
            value={color}
            onChange={(e) => updateColor(i, e.target.value)}
            className="h-7 w-7 cursor-pointer rounded border border-gray-200"
          />
          <Input
            value={color}
            onChange={(e) => updateColor(i, e.target.value)}
            className="text-xs flex-1"
          />
          <button
            onClick={() => removeColor(i)}
            className="text-gray-400 hover:text-red-500"
          >
            <Trash2 className="h-3 w-3" />
          </button>
        </div>
      ))}
      <button
        onClick={addColor}
        className="flex w-full items-center justify-center gap-1 rounded-md border border-dashed border-gray-300 px-3 py-2 text-xs text-gray-500 hover:border-blue-400 hover:text-blue-600"
      >
        <Plus className="h-3 w-3" />
        {t("addColumn")}
      </button>
    </div>
  );
}
