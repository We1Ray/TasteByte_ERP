"use client";

import { useState, useEffect } from "react";
import { useParams, useRouter } from "next/navigation";
import {
  Save,
  ArrowLeft,
  Settings,
  Database,
  Zap,
  Rocket,
} from "lucide-react";
import { useTranslations } from "next-intl";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Modal } from "@/components/ui/modal";
import { PageLoading } from "@/components/ui/loading";
import { ColumnPalette } from "./ColumnPalette";
import { GridPreview } from "./GridPreview";
import { ColumnPropertyPanel } from "./ColumnPropertyPanel";
import { DataSourceEditor } from "./DataSourceEditor";
import { ActionEditor } from "./ActionEditor";
import { WorkflowStatusIndicator } from "@/components/lowcode/form-builder/WorkflowStatusIndicator";
import { ReleaseSubmitModal } from "@/components/lowcode/form-builder/ReleaseSubmitModal";
import {
  useApiQuery,
  useApiMutation,
  useInvalidateQueries,
} from "@/lib/hooks/use-api-query";
import { useListBuilderStore } from "@/lib/stores/list-builder-store";
import { operationsApi, listApi } from "@/lib/api/lowcode";

export function ListBuilder() {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const params = useParams();
  const router = useRouter();
  const id = params?.id as string;
  const invalidate = useInvalidateQueries();

  const {
    columns,
    actions,
    dataSourceSql,
    settings,
    isDirty,
    loadFromDefinition,
    updateSettings,
    markClean,
    reset,
  } = useListBuilderStore();

  const [dataSourceOpen, setDataSourceOpen] = useState(false);
  const [actionsOpen, setActionsOpen] = useState(false);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [releaseOpen, setReleaseOpen] = useState(false);

  const { data: operation, isLoading: opLoading } = useApiQuery(
    ["lowcode", "operations", id],
    () => operationsApi.get(id),
    { enabled: !!id }
  );

  const { data: listDef, isLoading: listLoading } = useApiQuery(
    ["lowcode", "list", id],
    () => listApi.getDefinition(id),
    { enabled: !!id }
  );

  useEffect(() => {
    if (listDef) {
      loadFromDefinition(listDef);
    }
  }, [listDef, loadFromDefinition]);

  useEffect(() => {
    return () => {
      reset();
    };
  }, [reset]);

  const saveMutation = useApiMutation(
    () =>
      listApi.saveDefinition(id, {
        columns: columns.map((c, i) => ({ ...c, sort_order: i })),
        actions,
        data_source_sql: dataSourceSql,
        default_page_size: settings.pageSize,
        enable_search: settings.enableSearch,
        enable_export: settings.enableExport,
        enable_import: settings.enableImport,
      }),
    {
      onSuccess: () => {
        invalidate(["lowcode", "list", id]);
        markClean();
        toast.success(t("savedSuccessfully"));
      },
      onError: () => {
        toast.error(t("saveFailed"));
      },
    }
  );

  if (opLoading || listLoading) {
    return <PageLoading />;
  }

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
                {operation?.name || t("listBuilder")}
              </h1>
              {id && <WorkflowStatusIndicator operationId={id} />}
            </div>
            <p className="text-xs text-gray-500">
              {operation?.code || "LIST"} - {t("listBuilder")}
              {isDirty && ` ${t("unsavedChanges")}`}
            </p>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <Button
            variant="ghost"
            size="sm"
            onClick={() => setDataSourceOpen(true)}
            title={t("dataSourceSQL")}
          >
            <Database className="h-4 w-4" />
            SQL
          </Button>
          <Button
            variant="ghost"
            size="sm"
            onClick={() => setActionsOpen(true)}
            title={t("rowActions")}
          >
            <Zap className="h-4 w-4" />
            {tCommon("actions")}
          </Button>
          <Button
            variant="ghost"
            size="icon"
            onClick={() => setSettingsOpen(true)}
            title={t("listSettings")}
          >
            <Settings className="h-4 w-4" />
          </Button>
          <Button
            variant="secondary"
            onClick={() => setReleaseOpen(true)}
            disabled={isDirty}
            title={isDirty ? t("saveBeforeRelease") : t("createRelease")}
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

      {/* 3-panel layout */}
      <div className="flex flex-1 overflow-hidden">
        <ColumnPalette tableName={operation?.table_name} />
        <GridPreview />
        <ColumnPropertyPanel />
      </div>

      {/* Modals */}
      <DataSourceEditor
        open={dataSourceOpen}
        onClose={() => setDataSourceOpen(false)}
      />
      <ActionEditor
        open={actionsOpen}
        onClose={() => setActionsOpen(false)}
      />

      <ListSettingsModal
        open={settingsOpen}
        onClose={() => setSettingsOpen(false)}
        settings={settings}
        onUpdate={updateSettings}
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

// ---- List Settings Modal ----

function ListSettingsModal({
  open,
  onClose,
  settings,
  onUpdate,
}: {
  open: boolean;
  onClose: () => void;
  settings: {
    pageSize: number;
    enableSearch: boolean;
    enableExport: boolean;
    enableImport: boolean;
  };
  onUpdate: (updates: Partial<typeof settings>) => void;
}) {
  const t = useTranslations("lowcode");
  return (
    <Modal
      open={open}
      onClose={onClose}
      title={t("listSettings")}
      size="md"
      footer={<Button onClick={onClose}>{t("done")}</Button>}
    >
      <div className="space-y-4">
        <Input
          label={t("defaultPageSize")}
          type="number"
          value={settings.pageSize}
          onChange={(e) => onUpdate({ pageSize: Number(e.target.value) || 20 })}
          helperText={t("pageSizeHint")}
        />

        <label className="flex items-center gap-2">
          <input
            type="checkbox"
            checked={settings.enableSearch}
            onChange={(e) => onUpdate({ enableSearch: e.target.checked })}
            className="h-4 w-4 rounded border-gray-300 text-blue-600"
          />
          <span className="text-sm text-gray-700">{t("enableGlobalSearch")}</span>
        </label>

        <label className="flex items-center gap-2">
          <input
            type="checkbox"
            checked={settings.enableExport}
            onChange={(e) => onUpdate({ enableExport: e.target.checked })}
            className="h-4 w-4 rounded border-gray-300 text-blue-600"
          />
          <span className="text-sm text-gray-700">{t("enableExcelExport")}</span>
        </label>

        <label className="flex items-center gap-2">
          <input
            type="checkbox"
            checked={settings.enableImport}
            onChange={(e) => onUpdate({ enableImport: e.target.checked })}
            className="h-4 w-4 rounded border-gray-300 text-blue-600"
          />
          <span className="text-sm text-gray-700">{t("enableCsvImport")}</span>
        </label>
      </div>
    </Modal>
  );
}
