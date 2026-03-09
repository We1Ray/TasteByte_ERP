"use client";

import { useEffect, useRef } from "react";
import { Loader2, LayoutGrid } from "lucide-react";
import { useTranslations } from "next-intl";
import { useApiQuery, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { dashboardApi } from "@/lib/api/lowcode";
import { WidgetRenderer } from "./WidgetRenderer";

interface DashboardRendererProps {
  operationId: string;
}

export function DashboardRenderer({ operationId }: DashboardRendererProps) {
  const t = useTranslations("lowcode");
  const invalidate = useInvalidateQueries();
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const { data: dashDef, isLoading } = useApiQuery(
    ["lowcode", "dashboard", operationId],
    () => dashboardApi.getDefinition(operationId),
    { enabled: !!operationId }
  );

  // Auto-refresh
  useEffect(() => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }

    if (dashDef?.refresh_interval && dashDef.refresh_interval >= 5) {
      intervalRef.current = setInterval(() => {
        // Invalidate all widget data queries to trigger refetch
        dashDef.widgets.forEach((widget) => {
          invalidate([
            "lowcode",
            "widget-data",
            widget.id,
            widget.data_source_sql,
          ]);
        });
      }, dashDef.refresh_interval * 1000);
    }

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, [dashDef, invalidate]);

  if (isLoading) {
    return (
      <div className="flex h-64 items-center justify-center">
        <Loader2 className="h-8 w-8 animate-spin text-gray-400" />
      </div>
    );
  }

  if (!dashDef) {
    return (
      <div className="flex h-64 items-center justify-center">
        <p className="text-sm text-gray-500">
          {t("dashboardNotFound")}
        </p>
      </div>
    );
  }

  const widgets = [...dashDef.widgets].sort(
    (a, b) => a.sort_order - b.sort_order
  );

  if (widgets.length === 0) {
    return (
      <div className="flex h-64 items-center justify-center">
        <div className="text-center">
          <LayoutGrid className="mx-auto h-12 w-12 text-gray-300" />
          <h3 className="mt-3 text-sm font-semibold text-gray-600">
            {t("emptyDashboard")}
          </h3>
          <p className="mt-1 text-xs text-gray-400">
            {t("noWidgetsConfigured")}
          </p>
        </div>
      </div>
    );
  }

  const gridColumns = dashDef.grid_columns || 12;
  const maxRow = widgets.reduce(
    (max, w) => Math.max(max, w.grid_y + w.grid_h),
    0
  );

  return (
    <div className="space-y-4">
      {dashDef.refresh_interval && (
        <p className="text-xs text-gray-400">
          {t("autoRefresh", { seconds: dashDef.refresh_interval })}
        </p>
      )}

      <div
        style={{
          display: "grid",
          gridTemplateColumns: `repeat(${gridColumns}, 1fr)`,
          gridTemplateRows: `repeat(${maxRow}, 80px)`,
          gap: "16px",
        }}
      >
        {widgets.map((widget) => (
          <div
            key={widget.id}
            style={{
              gridColumn: `${widget.grid_x + 1} / span ${widget.grid_w}`,
              gridRow: `${widget.grid_y + 1} / span ${widget.grid_h}`,
            }}
          >
            <WidgetRenderer widget={widget} />
          </div>
        ))}
      </div>
    </div>
  );
}
