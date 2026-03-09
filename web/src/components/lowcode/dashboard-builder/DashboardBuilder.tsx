"use client";

import { useState, useEffect, useCallback, useRef } from "react";
import { useParams, useRouter } from "next/navigation";
import { Save, ArrowLeft, Settings, Rocket } from "lucide-react";
import { toast } from "sonner";
import {
  DndContext,
  DragOverlay,
  PointerSensor,
  useSensor,
  useSensors,
  type DragEndEvent,
  type DragStartEvent,
} from "@dnd-kit/core";
import { useTranslations } from "next-intl";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Modal } from "@/components/ui/modal";
import { PageLoading } from "@/components/ui/loading";
import { WidgetPalette, widgetTypeOptions } from "./WidgetPalette";
import { WidgetCanvas, CELL_HEIGHT } from "./WidgetCanvas";
import { WidgetPropertyPanel } from "./WidgetPropertyPanel";
import { WorkflowStatusIndicator } from "@/components/lowcode/form-builder/WorkflowStatusIndicator";
import { ReleaseSubmitModal } from "@/components/lowcode/form-builder/ReleaseSubmitModal";
import {
  useApiQuery,
  useApiMutation,
  useInvalidateQueries,
} from "@/lib/hooks/use-api-query";
import { useDashboardBuilderStore } from "@/lib/stores/dashboard-builder-store";
import { operationsApi, dashboardApi } from "@/lib/api/lowcode";
import type { DashboardWidget } from "@/lib/types/lowcode";

// Icons map for drag overlay rendering
const widgetIcons: Record<string, React.ElementType> = {};
for (const opt of widgetTypeOptions) {
  widgetIcons[opt.type] = opt.icon;
}

interface ActiveDragItem {
  type: "palette" | "widget";
  widgetType?: DashboardWidget["widget_type"];
  label?: string;
  widget?: DashboardWidget;
}

export function DashboardBuilder() {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const params = useParams();
  const router = useRouter();
  const id = params?.id as string;
  const invalidate = useInvalidateQueries();

  const {
    widgets,
    gridColumns,
    refreshInterval,
    isDirty,
    loadFromDefinition,
    markClean,
    reset,
    addWidget,
    moveWidget,
    setDragging,
  } = useDashboardBuilderStore();

  const [settingsOpen, setSettingsOpen] = useState(false);
  const [releaseOpen, setReleaseOpen] = useState(false);
  const [activeDrag, setActiveDrag] = useState<ActiveDragItem | null>(null);

  // Ref to the grid container in WidgetCanvas for dimension calculations
  const gridContainerRef = useRef<HTMLDivElement | null>(null);

  const { data: operation, isLoading: opLoading } = useApiQuery(
    ["lowcode", "operations", id],
    () => operationsApi.get(id),
    { enabled: !!id }
  );

  const { data: dashDef, isLoading: dashLoading } = useApiQuery(
    ["lowcode", "dashboard", id],
    () => dashboardApi.getDefinition(id),
    { enabled: !!id }
  );

  useEffect(() => {
    if (dashDef) {
      loadFromDefinition(dashDef);
    }
  }, [dashDef, loadFromDefinition]);

  useEffect(() => {
    return () => {
      reset();
    };
  }, [reset]);

  const saveMutation = useApiMutation(
    () =>
      dashboardApi.saveDefinition(id, {
        widgets: widgets.map((w, i) => ({ ...w, sort_order: i })),
        grid_columns: gridColumns,
        refresh_interval: refreshInterval,
      }),
    {
      onSuccess: () => {
        invalidate(["lowcode", "dashboard", id]);
        markClean();
        toast.success(t("savedSuccessfully"));
      },
      onError: () => {
        toast.error(t("saveFailed"));
      },
    }
  );

  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: { distance: 8 },
    })
  );

  const handleDragStart = useCallback(
    (event: DragStartEvent) => {
      setDragging(true);
      const data = event.active.data.current;
      if (data?.type === "palette") {
        setActiveDrag({
          type: "palette",
          widgetType: data.widgetType as DashboardWidget["widget_type"],
          label: data.label as string,
        });
      } else if (data?.type === "widget") {
        setActiveDrag({
          type: "widget",
          widget: data.widget as DashboardWidget,
        });
      }
    },
    [setDragging]
  );

  const handleDragEnd = useCallback(
    (event: DragEndEvent) => {
      setDragging(false);
      setActiveDrag(null);

      const { active, delta } = event;
      const data = active.data.current;

      if (data?.type === "palette") {
        // Palette item dropped -- add widget at next available position
        const paletteOption = widgetTypeOptions.find(
          (o) => o.type === data.widgetType
        );
        if (!paletteOption) return;

        const maxY = widgets.reduce(
          (max, w) => Math.max(max, w.grid_y + w.grid_h),
          0
        );

        addWidget({
          title: `New ${t(paletteOption.labelKey)}`,
          widget_type: paletteOption.type,
          data_source_sql: "",
          x_axis_key: "",
          y_axis_key: "",
          series_config: paletteOption.defaultConfig.series_config || [],
          colors: paletteOption.defaultConfig.colors || [],
          grid_x: 0,
          grid_y: maxY,
          grid_w: paletteOption.defaultConfig.grid_w || 6,
          grid_h: paletteOption.defaultConfig.grid_h || 4,
          widget_config: {},
          sort_order: widgets.length,
        });
      } else if (data?.type === "widget") {
        // Widget repositioned -- calculate grid delta from pixel delta
        const widget = data.widget as DashboardWidget;
        const gridEl = gridContainerRef.current;
        if (!gridEl || !widget) return;

        // Ignore very small drags (less than half a cell in either direction)
        if (Math.abs(delta.x) < 5 && Math.abs(delta.y) < 5) return;

        const cellWidth = gridEl.offsetWidth / gridColumns;
        const cellHeight = CELL_HEIGHT;

        const dx = Math.round(delta.x / cellWidth);
        const dy = Math.round(delta.y / cellHeight);

        if (dx === 0 && dy === 0) return;

        const newX = Math.max(
          0,
          Math.min(gridColumns - widget.grid_w, widget.grid_x + dx)
        );
        const newY = Math.max(0, widget.grid_y + dy);

        moveWidget(widget.id, newX, newY);
      }
    },
    [widgets, gridColumns, addWidget, moveWidget, setDragging, t]
  );

  const handleDragCancel = useCallback(() => {
    setDragging(false);
    setActiveDrag(null);
  }, [setDragging]);

  if (opLoading || dashLoading) {
    return <PageLoading />;
  }

  // Render drag overlay content based on active drag type
  const renderDragOverlay = () => {
    if (!activeDrag) return null;

    if (activeDrag.type === "palette") {
      const Icon = widgetIcons[activeDrag.widgetType || "bar"];
      return (
        <div className="flex items-center gap-2 rounded-lg border-2 border-blue-400 bg-white px-4 py-3 shadow-lg">
          {Icon && <Icon className="h-5 w-5 text-blue-500" />}
          <span className="text-sm font-medium text-gray-700">
            {activeDrag.label}
          </span>
        </div>
      );
    }

    if (activeDrag.type === "widget" && activeDrag.widget) {
      const w = activeDrag.widget;
      const Icon = widgetIcons[w.widget_type];
      return (
        <div className="flex items-center gap-2 rounded-lg border-2 border-blue-500 bg-white/90 px-4 py-3 shadow-xl">
          {Icon && <Icon className="h-4 w-4 text-blue-500" />}
          <span className="truncate text-sm font-medium text-gray-700">
            {w.title}
          </span>
          <span className="text-xs text-gray-400">
            ({w.grid_w}x{w.grid_h})
          </span>
        </div>
      );
    }

    return null;
  };

  return (
    <div className="-m-6 flex h-[calc(100vh-3.5rem)] flex-col">
      {/* Toolbar */}
      <div className="flex items-center justify-between border-b bg-white px-6 py-3">
        <div className="flex items-center gap-3">
          <Button
            variant="ghost"
            size="icon"
            onClick={() => router.push("/developer")}
          >
            <ArrowLeft className="h-4 w-4" />
          </Button>
          <div>
            <div className="flex items-center gap-2">
              <h1 className="text-lg font-semibold text-gray-900">
                {operation?.name || t("dashboardBuilder")}
              </h1>
              {id && <WorkflowStatusIndicator operationId={id} />}
            </div>
            <p className="text-xs text-gray-500">
              {operation?.code || "DASHBOARD"} - {t("dashboardBuilder")}
              {isDirty && ` ${t("unsavedChanges")}`}
            </p>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <Button
            variant="ghost"
            size="icon"
            onClick={() => setSettingsOpen(true)}
            title={t("operationSettings")}
          >
            <Settings className="h-4 w-4" />
          </Button>
          <Button
            variant="secondary"
            onClick={() => setReleaseOpen(true)}
            disabled={isDirty}
            title={
              isDirty
                ? t("saveBeforeRelease")
                : t("createRelease")
            }
          >
            <Rocket className="h-4 w-4" />
            {t("createRelease")}
          </Button>
          <Button
            onClick={() => saveMutation.mutateAsync(undefined)}
            loading={saveMutation.isPending}
            disabled={!isDirty}
          >
            <Save className="h-4 w-4" />
            {tCommon("save")}
          </Button>
        </div>
      </div>

      {/* 3-panel layout wrapped in DndContext */}
      <DndContext
        sensors={sensors}
        onDragStart={handleDragStart}
        onDragEnd={handleDragEnd}
        onDragCancel={handleDragCancel}
      >
        <div className="flex flex-1 overflow-hidden">
          <WidgetPalette />
          <WidgetCanvas gridContainerRef={gridContainerRef} />
          <WidgetPropertyPanel />
        </div>

        <DragOverlay dropAnimation={null}>
          {renderDragOverlay()}
        </DragOverlay>
      </DndContext>

      {/* Modals */}
      <DashboardSettingsModal
        open={settingsOpen}
        onClose={() => setSettingsOpen(false)}
      />

      {id && (
        <ReleaseSubmitModal
          open={releaseOpen}
          onClose={() => setReleaseOpen(false)}
          operationId={id}
        />
      )}
    </div>
  );
}

// ---- Dashboard Settings Modal ----

function DashboardSettingsModal({
  open,
  onClose,
}: {
  open: boolean;
  onClose: () => void;
}) {
  const t = useTranslations("lowcode");
  const {
    gridColumns,
    refreshInterval,
    setGridColumns,
    setRefreshInterval,
  } = useDashboardBuilderStore();

  return (
    <Modal
      open={open}
      onClose={onClose}
      title={t("operationSettings")}
      size="md"
      footer={<Button onClick={onClose}>{t("done")}</Button>}
    >
      <div className="space-y-4">
        <Input
          label={t("columns")}
          type="number"
          value={gridColumns}
          onChange={(e) =>
            setGridColumns(
              Math.max(1, Math.min(24, Number(e.target.value) || 12))
            )
          }
        />

        <Input
          label={t("autoRefresh", { seconds: "" })}
          type="number"
          value={refreshInterval ?? ""}
          onChange={(e) => {
            const val = e.target.value ? Number(e.target.value) : null;
            setRefreshInterval(val && val >= 5 ? val : null);
          }}
        />
      </div>
    </Modal>
  );
}
