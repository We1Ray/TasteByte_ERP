"use client";

import { useState } from "react";
import { useParams, useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import { ArrowLeft, Trash2 } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { ConfirmDialog } from "@/components/ui/confirm-dialog";
import { PageLoading } from "@/components/ui/loading";
import { ButtonDesigner, type ButtonItem } from "@/components/lowcode/ButtonDesigner";
import { RecordPolicyPanel } from "@/components/lowcode/form-builder/RecordPolicyPanel";
import { useApiQuery, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { useToastMutation } from "@/lib/hooks/use-toast-mutation";
import { operationsApi, datasourceApi, buttonsApi } from "@/lib/api/lowcode";
import type { LowCodeOperation, TableInfo, OperationButton } from "@/lib/types/lowcode";

export default function OperationSettingsPage() {
  const params = useParams();
  const router = useRouter();
  const operationId = params.id as string;
  const t = useTranslations("developer");
  const tCommon = useTranslations("common");
  const invalidate = useInvalidateQueries();

  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [tableName, setTableName] = useState("");
  const [module, setModule] = useState<string>("");
  const [sidebarIcon, setSidebarIcon] = useState("");
  const [sidebarSortOrder, setSidebarSortOrder] = useState(100);
  const [buttons, setButtons] = useState<ButtonItem[]>([]);
  const [syncedId, setSyncedId] = useState<string | null>(null);
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);

  const { data: operation, isLoading: opLoading } = useApiQuery(
    ["lowcode", "operations", operationId],
    () => operationsApi.get(operationId)
  );

  const { data: tables } = useApiQuery(
    ["lowcode", "datasource", "tables"],
    () => datasourceApi.tables()
  );

  const { data: buttonsData } = useApiQuery(
    ["lowcode", "buttons", operationId],
    () => buttonsApi.get(operationId),
    { enabled: !!operationId }
  );

  // Sync operation data into local state
  if (operation && syncedId !== operation.id) {
    setSyncedId(operation.id);
    setName(operation.name || "");
    setDescription(operation.description || "");
    setTableName(operation.table_name || "");
    setModule(operation.module || "");
    setSidebarIcon(operation.sidebar_icon || "");
    setSidebarSortOrder(operation.sidebar_sort_order ?? 100);
  }

  const updateMutation = useToastMutation(
    (data: Partial<LowCodeOperation>) => operationsApi.update(operationId, data),
    {
      successMessage: tCommon("saveSuccess"),
      invalidateKeys: ["lowcode", "operations"],
      onSuccess: () => {
        invalidate(["lowcode", "operations", operationId]);
      },
    }
  );

  const saveButtonsMutation = useToastMutation(
    (btns: ButtonItem[]) => buttonsApi.save(operationId, btns as Partial<OperationButton>[]),
    {
      successMessage: tCommon("saveSuccess"),
      invalidateKeys: ["lowcode", "buttons"],
    }
  );

  const deleteMutation = useToastMutation(
    () => operationsApi.delete(operationId),
    {
      successMessage: tCommon("deleteSuccess"),
      invalidateKeys: ["lowcode", "operations"],
      onSuccess: () => {
        router.push("/developer/operations");
      },
    }
  );

  const handleSave = () => {
    updateMutation.mutate({
      name,
      description,
      table_name: tableName,
      module: module || null,
      sidebar_icon: sidebarIcon || null,
      sidebar_sort_order: sidebarSortOrder,
    } as Partial<LowCodeOperation>);
    if (buttons.length > 0 || (buttonsData && buttonsData.length > 0)) {
      saveButtonsMutation.mutate(buttons);
    }
  };

  if (opLoading) return <PageLoading />;

  const title = operation
    ? `${operation.code} - ${t("settings")}`
    : t("settings");

  return (
    <div>
      <PageHeader
        title={title}
        description={t("settingsDesc")}
        actions={
          <Button
            variant="secondary"
            onClick={() => router.push(`/developer/operations/${operationId}`)}
          >
            <ArrowLeft className="h-4 w-4" />
            {tCommon("back")}
          </Button>
        }
      />

      {/* Basic Info Section */}
      <Card className="mb-6">
        <CardHeader>
          <CardTitle>{t("basicInfo")}</CardTitle>
        </CardHeader>
        <div className="space-y-4">
          <Input
            label={t("operationName")}
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder={t("operationNamePlaceholder")}
            required
          />

          <div>
            <label className="mb-1 block text-sm font-medium text-gray-700">
              {t("operationCode")}
            </label>
            <input
              value={operation?.code ?? ""}
              disabled
              className="block w-full rounded-md border border-gray-300 bg-gray-50 px-3 py-2 text-sm text-gray-500"
            />
            <p className="mt-1 text-xs text-gray-400">{t("codeReadonly")}</p>
          </div>

          <div>
            <label className="mb-1 block text-sm font-medium text-gray-700">
              {tCommon("description")}
            </label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              rows={3}
              className="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
            />
          </div>

          <div>
            <label className="mb-1 block text-sm font-medium text-gray-700">
              {t("operationType")}
            </label>
            <input
              value={operation?.operation_type ?? ""}
              disabled
              className="block w-full rounded-md border border-gray-300 bg-gray-50 px-3 py-2 text-sm text-gray-500"
            />
            <p className="mt-1 text-xs text-gray-400">{t("typeReadonly")}</p>
          </div>
        </div>
      </Card>

      {/* Data Source Section */}
      <Card className="mb-6">
        <CardHeader>
          <CardTitle>{t("dataSource")}</CardTitle>
        </CardHeader>
        <div className="space-y-4">
          <Select
            label={t("tableName")}
            value={tableName}
            onChange={(e) => setTableName(e.target.value)}
            options={[
              { value: "", label: t("selectTable") },
              ...(tables || []).map((tbl: TableInfo) => ({
                value: tbl.table_name,
                label: tbl.table_name,
              })),
            ]}
          />
        </div>
      </Card>

      {/* Module Binding Section */}
      <Card className="mb-6">
        <CardHeader>
          <CardTitle>{t("moduleBinding")}</CardTitle>
        </CardHeader>
        <div className="space-y-4">
          <Select
            label={t("erpModule")}
            value={module}
            onChange={(e) => setModule(e.target.value)}
            options={[
              { value: "", label: t("standalone") },
              { value: "FI", label: "FI - Financial" },
              { value: "CO", label: "CO - Controlling" },
              { value: "MM", label: "MM - Materials" },
              { value: "SD", label: "SD - Sales" },
              { value: "PP", label: "PP - Production" },
              { value: "HR", label: "HR - Human Resources" },
              { value: "WM", label: "WM - Warehouse" },
              { value: "QM", label: "QM - Quality" },
            ]}
          />
          <Input
            label={t("sidebarIcon")}
            value={sidebarIcon}
            onChange={(e) => setSidebarIcon(e.target.value)}
            placeholder="e.g., FileText"
          />
          <Input
            label={t("sidebarSortOrder")}
            type="number"
            value={String(sidebarSortOrder)}
            onChange={(e) => setSidebarSortOrder(Number(e.target.value) || 100)}
          />
        </div>
      </Card>

      {/* Action Buttons Section */}
      <Card className="mb-6">
        <CardHeader>
          <CardTitle>{t("actionButtons")}</CardTitle>
        </CardHeader>
        <div>
          <ButtonDesigner
            buttons={buttons.length > 0 ? buttons : (buttonsData || []).map((b: OperationButton) => ({
              button_key: b.button_key,
              label: b.label,
              icon: b.icon,
              variant: b.variant,
              action_type: b.action_type,
              action_config: b.action_config,
              confirm_message: b.confirm_message,
              required_permission: b.required_permission,
              is_visible: b.is_visible,
              sort_order: b.sort_order,
            }))}
            onChange={setButtons}
          />
        </div>
      </Card>

      {/* Record Policies */}
      <div className="mb-6">
        <RecordPolicyPanel operationId={operationId} />
      </div>

      {/* Save Button */}
      <div className="mb-8 flex justify-end">
        <Button onClick={handleSave} loading={updateMutation.isPending}>
          {tCommon("save")}
        </Button>
      </div>

      {/* Danger Zone */}
      <Card className="border-red-200">
        <CardHeader>
          <CardTitle className="text-red-600">{t("dangerZone")}</CardTitle>
        </CardHeader>
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm font-medium text-gray-900">{t("deleteOperation")}</p>
            <p className="mt-0.5 text-sm text-gray-500">{t("deleteOperationDesc")}</p>
          </div>
          <Button
            variant="danger"
            onClick={() => setShowDeleteConfirm(true)}
          >
            <Trash2 className="h-4 w-4" />
            {tCommon("delete")}
          </Button>
        </div>
      </Card>

      <ConfirmDialog
        open={showDeleteConfirm}
        onClose={() => setShowDeleteConfirm(false)}
        onConfirm={() => {
          deleteMutation.mutate(undefined);
          setShowDeleteConfirm(false);
        }}
        title={t("deleteOperation")}
        message={t("deleteOperationConfirm")}
        confirmLabel={tCommon("delete")}
        variant="danger"
        loading={deleteMutation.isPending}
      />
    </div>
  );
}
