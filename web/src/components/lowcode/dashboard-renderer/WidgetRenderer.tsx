"use client";

import { useMemo } from "react";
import {
  useReactTable,
  getCoreRowModel,
  flexRender,
  type ColumnDef,
} from "@tanstack/react-table";
import { Activity, Loader2, AlertCircle } from "lucide-react";
import { useTranslations } from "next-intl";
import { BarChartCard } from "@/components/charts/bar-chart";
import { LineChartCard } from "@/components/charts/line-chart";
import { PieChartCard } from "@/components/charts/pie-chart";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { datasourceApi } from "@/lib/api/lowcode";
import type { DashboardWidget } from "@/lib/types/lowcode";

interface WidgetRendererProps {
  widget: DashboardWidget;
}

function KpiWidgetCard({
  widget,
  data,
}: {
  widget: DashboardWidget;
  data: Record<string, unknown>[];
}) {
  const tCommon = useTranslations("common");
  const row = data[0];
  if (!row) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>{widget.title}</CardTitle>
        </CardHeader>
        <p className="text-sm text-gray-500">{tCommon("noData")}</p>
      </Card>
    );
  }

  // Use the first numeric column as the value
  const keys = Object.keys(row);
  const valueKey = widget.y_axis_key || keys.find((k) => typeof row[k] === "number") || keys[0];
  const labelKey = widget.x_axis_key || keys.find((k) => k !== valueKey) || "";

  const value = row[valueKey];
  const label = labelKey && row[labelKey] ? String(row[labelKey]) : "";

  const color = widget.colors[0] || "#2563eb";

  return (
    <Card>
      <CardHeader>
        <CardTitle className="text-sm">{widget.title}</CardTitle>
      </CardHeader>
      <div className="flex items-start justify-between">
        <div>
          <p className="text-2xl font-bold text-gray-900">
            {value !== null && value !== undefined ? String(value) : "-"}
          </p>
          {label && (
            <p className="mt-1 text-sm text-gray-500">{label}</p>
          )}
        </div>
        <div
          className="rounded-lg p-3"
          style={{ backgroundColor: `${color}20`, color }}
        >
          <Activity className="h-6 w-6" />
        </div>
      </div>
    </Card>
  );
}

function TableWidget({
  widget,
  data,
}: {
  widget: DashboardWidget;
  data: Record<string, unknown>[];
}) {
  const columns = useMemo<ColumnDef<Record<string, unknown>>[]>(() => {
    if (data.length === 0) return [];
    return Object.keys(data[0]).map((key) => ({
      id: key,
      accessorKey: key,
      header: key.replace(/_/g, " ").replace(/\b\w/g, (c) => c.toUpperCase()),
      size: 150,
    }));
  }, [data]);

  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
  });

  return (
    <Card padding={false}>
      <div className="px-4 py-3 border-b border-gray-200">
        <h3 className="text-sm font-semibold text-gray-900">{widget.title}</h3>
      </div>
      <div className="overflow-auto max-h-64">
        <table className="min-w-full divide-y divide-gray-200 text-xs">
          <thead className="bg-gray-50">
            {table.getHeaderGroups().map((headerGroup) => (
              <tr key={headerGroup.id}>
                {headerGroup.headers.map((header) => (
                  <th
                    key={header.id}
                    className="px-3 py-2 text-left font-semibold text-gray-600"
                  >
                    {header.isPlaceholder
                      ? null
                      : flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                  </th>
                ))}
              </tr>
            ))}
          </thead>
          <tbody className="divide-y divide-gray-100">
            {table.getRowModel().rows.map((row) => (
              <tr key={row.id}>
                {row.getVisibleCells().map((cell) => (
                  <td key={cell.id} className="px-3 py-2 text-gray-700">
                    {flexRender(
                      cell.column.columnDef.cell,
                      cell.getContext()
                    )}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </Card>
  );
}

export function WidgetRenderer({ widget }: WidgetRendererProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const { data: queryResult, isLoading, error } = useApiQuery(
    ["lowcode", "widget-data", widget.id, widget.data_source_sql],
    () => datasourceApi.query(widget.data_source_sql),
    { enabled: !!widget.data_source_sql }
  );

  const rows = (queryResult?.rows || []) as Record<string, unknown>[];

  if (!widget.data_source_sql) {
    return (
      <Card className="flex h-full items-center justify-center">
        <div className="text-center">
          <AlertCircle className="mx-auto h-8 w-8 text-amber-400" />
          <p className="mt-2 text-sm text-gray-500">{widget.title}</p>
          <p className="text-xs text-gray-400">{t("noTargetTable")}</p>
        </div>
      </Card>
    );
  }

  if (isLoading) {
    return (
      <Card className="flex h-full items-center justify-center">
        <Loader2 className="h-6 w-6 animate-spin text-gray-400" />
      </Card>
    );
  }

  if (error) {
    return (
      <Card className="flex h-full items-center justify-center">
        <div className="text-center">
          <AlertCircle className="mx-auto h-8 w-8 text-red-400" />
          <p className="mt-2 text-sm text-gray-500">{widget.title}</p>
          <p className="text-xs text-red-500">{tCommon("error")}</p>
        </div>
      </Card>
    );
  }

  switch (widget.widget_type) {
    case "bar":
      return (
        <BarChartCard
          title={widget.title}
          data={rows}
          xAxisKey={widget.x_axis_key || ""}
          dataKey={
            widget.series_config[0]?.dataKey || widget.y_axis_key || ""
          }
          color={widget.series_config[0]?.color || widget.colors[0]}
          secondaryDataKey={widget.series_config[1]?.dataKey}
          secondaryColor={widget.series_config[1]?.color}
        />
      );

    case "line":
      return (
        <LineChartCard
          title={widget.title}
          data={rows}
          xAxisKey={widget.x_axis_key || ""}
          lines={
            widget.series_config.length > 0
              ? widget.series_config
              : [
                  {
                    dataKey: widget.y_axis_key || "",
                    color: widget.colors[0] || "#2563eb",
                  },
                ]
          }
        />
      );

    case "pie":
      return (
        <PieChartCard
          title={widget.title}
          data={rows}
          dataKey={widget.y_axis_key || "value"}
          nameKey={widget.x_axis_key || "name"}
          colors={widget.colors}
        />
      );

    case "kpi":
      return <KpiWidgetCard widget={widget} data={rows} />;

    case "table":
      return <TableWidget widget={widget} data={rows} />;

    default:
      return (
        <Card>
          <p className="text-sm text-gray-500">
            Unknown widget type: {widget.widget_type}
          </p>
        </Card>
      );
  }
}
