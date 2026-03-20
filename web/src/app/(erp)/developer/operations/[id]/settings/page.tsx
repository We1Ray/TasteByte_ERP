"use client";

import { useState } from "react";
import { useParams, useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import { ArrowLeft, Trash2, Plus } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { ConfirmDialog } from "@/components/ui/confirm-dialog";
import { PageLoading } from "@/components/ui/loading";
import { ButtonDesigner, type ButtonItem } from "@/components/lowcode/ButtonDesigner";
import { RecordPolicyPanel } from "@/components/lowcode/form-builder/RecordPolicyPanel";
import { useApiQuery, useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { useToastMutation } from "@/lib/hooks/use-toast-mutation";
import { operationsApi, datasourceApi, buttonsApi } from "@/lib/api/lowcode";
import {
  rulesApi,
  formVariantsApi,
  type CrossFieldRule,
  type CalculationFormula,
  type FormVariant,
} from "@/lib/api/system";
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

      {/* Cross-field Validation Rules */}
      <Card className="mb-6">
        <CardHeader>
          <CardTitle>{t("crossFieldRules")}</CardTitle>
        </CardHeader>
        <CrossFieldRulesPanel operationId={operationId} />
      </Card>

      {/* Calculation Formulas */}
      <Card className="mb-6">
        <CardHeader>
          <CardTitle>{t("calculationFormulas")}</CardTitle>
        </CardHeader>
        <CalculationFormulasPanel operationId={operationId} />
      </Card>

      {/* Form Variants */}
      <Card className="mb-6">
        <CardHeader>
          <CardTitle>{t("formVariants")}</CardTitle>
        </CardHeader>
        <FormVariantsPanel operationId={operationId} />
      </Card>

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

/* ---------- Cross-field Rules Panel ---------- */
function CrossFieldRulesPanel({ operationId }: { operationId: string }) {
  const t = useTranslations("developer");
  const tCommon = useTranslations("common");
  const invalidate = useInvalidateQueries();

  const [showForm, setShowForm] = useState(false);
  const [ruleName, setRuleName] = useState("");
  const [sourceField, setSourceField] = useState("");
  const [operator, setOperator] = useState("equals");
  const [targetField, setTargetField] = useState("");
  const [targetValue, setTargetValue] = useState("");
  const [errorMessage, setErrorMessage] = useState("");

  const { data: rules, isLoading } = useApiQuery(
    ["system", "rules", operationId],
    () => rulesApi.listRules(operationId),
    { enabled: !!operationId }
  );

  const createMutation = useApiMutation(
    (data: Partial<CrossFieldRule>) => rulesApi.createRule(operationId, data),
    { onSuccess: () => invalidate(["system", "rules", operationId]) }
  );

  const deleteMutation = useApiMutation(
    (id: string) => rulesApi.deleteRule(operationId, id),
    { onSuccess: () => invalidate(["system", "rules", operationId]) }
  );

  const handleCreate = () => {
    if (!ruleName || !sourceField || !errorMessage) return;
    createMutation.mutateAsync({
      rule_name: ruleName,
      source_field: sourceField,
      operator,
      target_field: targetField || undefined,
      target_value: targetValue || undefined,
      error_message: errorMessage,
      is_active: true,
    }).then(() => {
      setRuleName("");
      setSourceField("");
      setOperator("equals");
      setTargetField("");
      setTargetValue("");
      setErrorMessage("");
      setShowForm(false);
    });
  };

  const operatorOptions = [
    "gt", "lt", "gte", "lte", "equals", "not_equals", "before", "after", "not_empty",
  ];

  return (
    <div className="space-y-3">
      <div className="flex justify-end">
        <Button size="sm" variant="secondary" onClick={() => setShowForm(!showForm)}>
          <Plus className="h-4 w-4" />
          {t("addRule")}
        </Button>
      </div>

      {showForm && (
        <div className="rounded-md border border-blue-200 bg-blue-50 p-4 space-y-3">
          <Input label={t("ruleName")} value={ruleName} onChange={(e) => setRuleName(e.target.value)} required />
          <div className="grid grid-cols-2 gap-3">
            <Input label={t("sourceField")} value={sourceField} onChange={(e) => setSourceField(e.target.value)} required />
            <Select
              label={tCommon("type")}
              value={operator}
              onChange={(e) => setOperator(e.target.value)}
              options={operatorOptions.map((op) => ({ value: op, label: op }))}
            />
          </div>
          <div className="grid grid-cols-2 gap-3">
            <Input label={t("targetField")} value={targetField} onChange={(e) => setTargetField(e.target.value)} />
            <Input label={t("targetValue")} value={targetValue} onChange={(e) => setTargetValue(e.target.value)} />
          </div>
          <Input label={t("errorMessage")} value={errorMessage} onChange={(e) => setErrorMessage(e.target.value)} required />
          <div className="flex gap-2">
            <Button size="sm" onClick={handleCreate} disabled={!ruleName || !sourceField || !errorMessage} loading={createMutation.isPending}>
              {tCommon("save")}
            </Button>
            <Button size="sm" variant="secondary" onClick={() => setShowForm(false)}>
              {tCommon("cancel")}
            </Button>
          </div>
        </div>
      )}

      {isLoading ? (
        <p className="py-4 text-center text-sm text-gray-400">{tCommon("loading")}</p>
      ) : (rules || []).length === 0 ? (
        <p className="py-4 text-center text-sm text-gray-400">{tCommon("noData")}</p>
      ) : (
        <div className="space-y-2">
          {(rules || []).map((rule: CrossFieldRule) => (
            <div key={rule.id} className="flex items-center justify-between rounded-md border border-gray-200 bg-white p-3">
              <div className="flex-1 min-w-0">
                <p className="text-sm font-medium text-gray-900">{rule.rule_name}</p>
                <p className="mt-0.5 truncate font-mono text-xs text-gray-500">
                  {rule.source_field} {rule.operator} {rule.target_field || rule.target_value || ""}
                </p>
                <p className="mt-0.5 text-xs text-red-500">{rule.error_message}</p>
              </div>
              <button onClick={() => deleteMutation.mutateAsync(rule.id)} className="ml-2 text-red-400 hover:text-red-600">
                <Trash2 className="h-4 w-4" />
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

/* ---------- Calculation Formulas Panel ---------- */
function CalculationFormulasPanel({ operationId }: { operationId: string }) {
  const t = useTranslations("developer");
  const tCommon = useTranslations("common");
  const invalidate = useInvalidateQueries();

  const [showForm, setShowForm] = useState(false);
  const [targetField, setTargetField] = useState("");
  const [formula, setFormula] = useState("");
  const [triggerFields, setTriggerFields] = useState("");

  const { data: formulas, isLoading } = useApiQuery(
    ["system", "formulas", operationId],
    () => rulesApi.listFormulas(operationId),
    { enabled: !!operationId }
  );

  const createMutation = useApiMutation(
    (data: Partial<CalculationFormula>) => rulesApi.createFormula(operationId, data),
    { onSuccess: () => invalidate(["system", "formulas", operationId]) }
  );

  const deleteMutation = useApiMutation(
    (id: string) => rulesApi.deleteFormula(operationId, id),
    { onSuccess: () => invalidate(["system", "formulas", operationId]) }
  );

  const handleCreate = () => {
    if (!targetField || !formula) return;
    createMutation.mutateAsync({
      target_field: targetField,
      formula,
      trigger_fields: triggerFields.split(",").map((s) => s.trim()).filter(Boolean),
      is_active: true,
    }).then(() => {
      setTargetField("");
      setFormula("");
      setTriggerFields("");
      setShowForm(false);
    });
  };

  return (
    <div className="space-y-3">
      <div className="flex justify-end">
        <Button size="sm" variant="secondary" onClick={() => setShowForm(!showForm)}>
          <Plus className="h-4 w-4" />
          {t("addFormula")}
        </Button>
      </div>

      {showForm && (
        <div className="rounded-md border border-blue-200 bg-blue-50 p-4 space-y-3">
          <Input label={t("targetField")} value={targetField} onChange={(e) => setTargetField(e.target.value)} required />
          <Input label={t("formula")} value={formula} onChange={(e) => setFormula(e.target.value)} placeholder="e.g., quantity * unit_price" required />
          <Input label={t("triggerFields")} value={triggerFields} onChange={(e) => setTriggerFields(e.target.value)} placeholder="e.g., quantity, unit_price" />
          <div className="flex gap-2">
            <Button size="sm" onClick={handleCreate} disabled={!targetField || !formula} loading={createMutation.isPending}>
              {tCommon("save")}
            </Button>
            <Button size="sm" variant="secondary" onClick={() => setShowForm(false)}>
              {tCommon("cancel")}
            </Button>
          </div>
        </div>
      )}

      {isLoading ? (
        <p className="py-4 text-center text-sm text-gray-400">{tCommon("loading")}</p>
      ) : (formulas || []).length === 0 ? (
        <p className="py-4 text-center text-sm text-gray-400">{tCommon("noData")}</p>
      ) : (
        <div className="space-y-2">
          {(formulas || []).map((f: CalculationFormula) => (
            <div key={f.id} className="flex items-center justify-between rounded-md border border-gray-200 bg-white p-3">
              <div className="flex-1 min-w-0">
                <p className="text-sm font-medium text-gray-900">
                  {f.target_field} = <span className="font-mono text-blue-600">{f.formula}</span>
                </p>
                <p className="mt-0.5 text-xs text-gray-500">
                  Triggers: {f.trigger_fields.join(", ")}
                </p>
              </div>
              <button onClick={() => deleteMutation.mutateAsync(f.id)} className="ml-2 text-red-400 hover:text-red-600">
                <Trash2 className="h-4 w-4" />
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

/* ---------- Form Variants Panel ---------- */
function FormVariantsPanel({ operationId }: { operationId: string }) {
  const t = useTranslations("developer");
  const tCommon = useTranslations("common");
  const invalidate = useInvalidateQueries();

  const [showForm, setShowForm] = useState(false);
  const [variantName, setVariantName] = useState("");
  const [conditionField, setConditionField] = useState("");
  const [conditionValue, setConditionValue] = useState("");
  const [hiddenFields, setHiddenFields] = useState("");
  const [readonlyFields, setReadonlyFields] = useState("");

  const { data: variants, isLoading } = useApiQuery(
    ["system", "form-variants", operationId],
    () => formVariantsApi.list(operationId),
    { enabled: !!operationId }
  );

  const createMutation = useApiMutation(
    (data: Partial<FormVariant>) => formVariantsApi.create(operationId, data),
    { onSuccess: () => invalidate(["system", "form-variants", operationId]) }
  );

  const deleteMutation = useApiMutation(
    (id: string) => formVariantsApi.delete(operationId, id),
    { onSuccess: () => invalidate(["system", "form-variants", operationId]) }
  );

  const handleCreate = () => {
    if (!variantName) return;
    createMutation.mutateAsync({
      variant_name: variantName,
      condition_field: conditionField || undefined,
      condition_value: conditionValue || undefined,
      hidden_fields: hiddenFields.split(",").map((s) => s.trim()).filter(Boolean),
      readonly_fields: readonlyFields.split(",").map((s) => s.trim()).filter(Boolean),
      required_fields: [],
      default_values: {},
      is_default: false,
    }).then(() => {
      setVariantName("");
      setConditionField("");
      setConditionValue("");
      setHiddenFields("");
      setReadonlyFields("");
      setShowForm(false);
    });
  };

  return (
    <div className="space-y-3">
      <div className="flex justify-end">
        <Button size="sm" variant="secondary" onClick={() => setShowForm(!showForm)}>
          <Plus className="h-4 w-4" />
          {t("addVariant")}
        </Button>
      </div>

      {showForm && (
        <div className="rounded-md border border-blue-200 bg-blue-50 p-4 space-y-3">
          <Input label={t("variantName")} value={variantName} onChange={(e) => setVariantName(e.target.value)} required />
          <div className="grid grid-cols-2 gap-3">
            <Input label={t("conditionField")} value={conditionField} onChange={(e) => setConditionField(e.target.value)} />
            <Input label={t("conditionValue")} value={conditionValue} onChange={(e) => setConditionValue(e.target.value)} />
          </div>
          <Input label={t("hiddenFields")} value={hiddenFields} onChange={(e) => setHiddenFields(e.target.value)} placeholder="field1, field2" />
          <Input label={t("readonlyFields")} value={readonlyFields} onChange={(e) => setReadonlyFields(e.target.value)} placeholder="field1, field2" />
          <div className="flex gap-2">
            <Button size="sm" onClick={handleCreate} disabled={!variantName} loading={createMutation.isPending}>
              {tCommon("save")}
            </Button>
            <Button size="sm" variant="secondary" onClick={() => setShowForm(false)}>
              {tCommon("cancel")}
            </Button>
          </div>
        </div>
      )}

      {isLoading ? (
        <p className="py-4 text-center text-sm text-gray-400">{tCommon("loading")}</p>
      ) : (variants || []).length === 0 ? (
        <p className="py-4 text-center text-sm text-gray-400">{tCommon("noData")}</p>
      ) : (
        <div className="space-y-2">
          {(variants || []).map((v: FormVariant) => (
            <div key={v.id} className="flex items-center justify-between rounded-md border border-gray-200 bg-white p-3">
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2">
                  <p className="text-sm font-medium text-gray-900">{v.variant_name}</p>
                  {v.is_default && (
                    <span className="inline-flex rounded-full bg-blue-100 px-2 py-0.5 text-xs font-medium text-blue-700">Default</span>
                  )}
                </div>
                {v.condition_field && (
                  <p className="mt-0.5 text-xs text-gray-500">
                    {v.condition_field} = {v.condition_value}
                  </p>
                )}
              </div>
              <button onClick={() => deleteMutation.mutateAsync(v.id)} className="ml-2 text-red-400 hover:text-red-600">
                <Trash2 className="h-4 w-4" />
              </button>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
