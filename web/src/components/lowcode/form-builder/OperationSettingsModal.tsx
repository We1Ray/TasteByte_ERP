"use client";

import { useState } from "react";
import { useTranslations } from "next-intl";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { Modal } from "@/components/ui/modal";
import {
  useApiQuery,
  useApiMutation,
  useInvalidateQueries,
} from "@/lib/hooks/use-api-query";
import { operationsApi, datasourceApi } from "@/lib/api/lowcode";
import type { LowCodeOperation, TableInfo } from "@/lib/types/lowcode";

interface OperationSettingsModalProps {
  open: boolean;
  onClose: () => void;
  operationId: string;
}

export function OperationSettingsModal({
  open,
  onClose,
  operationId,
}: OperationSettingsModalProps) {
  const invalidate = useInvalidateQueries();
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [tableName, setTableName] = useState("");
  const [operationType, setOperationType] = useState("form");
  const [syncedId, setSyncedId] = useState<string | null>(null);

  const { data: operation } = useApiQuery(
    ["lowcode", "operations", operationId],
    () => operationsApi.get(operationId),
    { enabled: open && !!operationId }
  );

  const { data: tables } = useApiQuery(
    ["lowcode", "datasource", "tables"],
    () => datasourceApi.tables(),
    { enabled: open }
  );

  // Sync operation data into local state using the state initializer pattern.
  // Calling setState during render is allowed by React when it's conditional
  // and used to derive state from props/query data (avoids an extra render
  // cycle compared to useEffect).
  if (operation && syncedId !== operation.id) {
    setSyncedId(operation.id);
    setName(operation.name || "");
    setDescription(operation.description || "");
    setTableName(operation.table_name || "");
    setOperationType(operation.operation_type || "form");
  }

  const updateMutation = useApiMutation(
    (data: Partial<LowCodeOperation>) => operationsApi.update(operationId, data),
    {
      onSuccess: () => {
        invalidate(["lowcode", "operations", operationId]);
        invalidate(["lowcode", "operations"]);
        onClose();
      },
    }
  );

  const handleSave = () => {
    updateMutation.mutate({
      name,
      description,
      table_name: tableName,
      operation_type: operationType as LowCodeOperation["operation_type"],
    } as Partial<LowCodeOperation>);
  };

  return (
    <Modal
      open={open}
      onClose={onClose}
      title={t("operationSettings")}
      size="lg"
      footer={
        <>
          <Button variant="secondary" onClick={onClose}>
            {tCommon("cancel")}
          </Button>
          <Button onClick={handleSave} loading={updateMutation.isPending}>
            {t("saveSettings")}
          </Button>
        </>
      }
    >
      <div className="space-y-4">
        <Input
          label={tCommon("name")}
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder={t("operationNamePlaceholder")}
        />

        <div>
          <label className="mb-1 block text-sm font-medium text-gray-700">
            {tCommon("description")}
          </label>
          <textarea
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            rows={3}
            placeholder={t("operationDescPlaceholder")}
            className="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
          />
        </div>

        <Select
          label={t("tableNameLabel")}
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

        <Select
          label={t("operationType")}
          value={operationType}
          onChange={(e) => setOperationType(e.target.value)}
          options={[
            { value: "form", label: t("formType") },
            { value: "list", label: t("listType") },
            { value: "dashboard", label: t("dashboardType") },
            { value: "report", label: t("reportType") },
            { value: "workflow", label: t("workflowType") },
          ]}
        />
      </div>
    </Modal>
  );
}
