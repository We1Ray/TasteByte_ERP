"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import { FileText, List, LayoutDashboard, ChevronRight, ChevronLeft, Loader2 } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { useToastMutation } from "@/lib/hooks/use-toast-mutation";
import { operationsApi, projectsApi, datasourceApi } from "@/lib/api/lowcode";
import type { LowCodeOperation, TableInfo } from "@/lib/types/lowcode";

const operationTypes = [
  { value: "form", icon: FileText },
  { value: "list", icon: List },
  { value: "dashboard", icon: LayoutDashboard },
] as const;

export default function NewOperationPage() {
  const router = useRouter();
  const t = useTranslations("developer");
  const tCommon = useTranslations("common");
  const [step, setStep] = useState(0);
  const [form, setForm] = useState({
    name: "",
    code: "",
    description: "",
    operation_type: "" as string,
    table_name: "",
    project_id: "",
    module: "",
  });

  const { data: tables } = useApiQuery(
    ["lowcode", "datasource", "tables"],
    () => datasourceApi.tables()
  );

  const { data: projects } = useApiQuery(
    ["lowcode", "projects"],
    () => projectsApi.list({ page_size: 100 })
  );

  const createMutation = useToastMutation(
    () => operationsApi.create(form as Partial<LowCodeOperation>),
    {
      successMessage: tCommon("create"),
      invalidateKeys: ["lowcode", "operations"],
      onSuccess: (data) => {
        router.push(`/developer/operations/${data.id}`);
      },
    }
  );

  const typeDescMap: Record<string, string> = {
    form: t("formTypeDesc"),
    list: t("listTypeDesc"),
    dashboard: t("dashboardTypeDesc"),
  };

  const typeLabelMap: Record<string, string> = {
    form: t("formType"),
    list: t("listType"),
    dashboard: t("dashboardType"),
  };

  const steps = [t("stepBasicInfo"), t("stepDataSource"), t("stepReview")];

  const canNext = step === 0
    ? form.name && form.code && form.operation_type
    : step === 1
    ? form.project_id
    : true;

  return (
    <div>
      <PageHeader title={t("newOperationWizard")} description={t("wizardDesc")} />

      {/* Step indicator */}
      <div className="mb-8 flex items-center justify-center gap-2">
        {steps.map((label, i) => (
          <div key={i} className="flex items-center gap-2">
            <div
              className={`flex h-8 w-8 items-center justify-center rounded-full text-sm font-medium ${
                i <= step
                  ? "bg-blue-600 text-white"
                  : "bg-gray-200 text-gray-500"
              }`}
            >
              {i + 1}
            </div>
            <span className={`text-sm ${i <= step ? "text-gray-900 font-medium" : "text-gray-400"}`}>
              {label}
            </span>
            {i < steps.length - 1 && (
              <ChevronRight className="h-4 w-4 text-gray-300" />
            )}
          </div>
        ))}
      </div>

      <Card className="mx-auto max-w-2xl" padding={false}>
        {step === 0 && (
          <div className="space-y-4 p-6">
            <div>
              <label className="mb-1 block text-sm font-medium text-gray-700">
                {t("operationName")} <span className="text-red-500">*</span>
              </label>
              <input
                value={form.name}
                onChange={(e) => setForm({ ...form, name: e.target.value })}
                placeholder={t("operationNamePlaceholder")}
                className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
              />
            </div>
            <div>
              <label className="mb-1 block text-sm font-medium text-gray-700">
                {t("operationCode")} <span className="text-red-500">*</span>
              </label>
              <input
                value={form.code}
                onChange={(e) =>
                  setForm({
                    ...form,
                    code: e.target.value.toUpperCase().replace(/[^A-Z0-9_]/g, ""),
                  })
                }
                placeholder="e.g., CUST_REG"
                className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm font-mono focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
              />
            </div>
            <div>
              <label className="mb-1 block text-sm font-medium text-gray-700">
                {tCommon("description")}
              </label>
              <textarea
                value={form.description}
                onChange={(e) => setForm({ ...form, description: e.target.value })}
                rows={2}
                className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
              />
            </div>
            <div>
              <label className="mb-2 block text-sm font-medium text-gray-700">
                {t("operationType")} <span className="text-red-500">*</span>
              </label>
              <div className="grid grid-cols-3 gap-3">
                {operationTypes.map(({ value, icon: Icon }) => (
                  <button
                    key={value}
                    type="button"
                    onClick={() => setForm({ ...form, operation_type: value })}
                    className={`flex flex-col items-center gap-2 rounded-lg border-2 p-4 transition-colors ${
                      form.operation_type === value
                        ? "border-blue-500 bg-blue-50"
                        : "border-gray-200 hover:border-gray-300"
                    }`}
                  >
                    <Icon className={`h-8 w-8 ${form.operation_type === value ? "text-blue-600" : "text-gray-400"}`} />
                    <span className="text-sm font-medium">{typeLabelMap[value]}</span>
                    <span className="text-center text-xs text-gray-500">{typeDescMap[value]}</span>
                  </button>
                ))}
              </div>
            </div>
            <div>
              <label className="mb-1 block text-sm font-medium text-gray-700">
                {t("erpModule")} <span className="text-gray-400">({tCommon("optional")})</span>
              </label>
              <select
                value={form.module}
                onChange={(e) => setForm({ ...form, module: e.target.value })}
                className="w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm"
              >
                <option value="">{t("standalone")}</option>
                <option value="FI">FI - Financial</option>
                <option value="CO">CO - Controlling</option>
                <option value="MM">MM - Materials</option>
                <option value="SD">SD - Sales</option>
                <option value="PP">PP - Production</option>
                <option value="HR">HR - Human Resources</option>
                <option value="WM">WM - Warehouse</option>
                <option value="QM">QM - Quality</option>
              </select>
            </div>
          </div>
        )}

        {step === 1 && (
          <div className="space-y-4 p-6">
            <div>
              <label className="mb-1 block text-sm font-medium text-gray-700">
                {t("project")} <span className="text-red-500">*</span>
              </label>
              <select
                value={form.project_id}
                onChange={(e) => setForm({ ...form, project_id: e.target.value })}
                className="w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm"
              >
                <option value="">{t("selectProject")}</option>
                {(projects?.items ?? []).map((p) => (
                  <option key={p.id} value={p.id}>{p.name}</option>
                ))}
              </select>
            </div>
            <div>
              <label className="mb-1 block text-sm font-medium text-gray-700">
                {t("tableName")}
              </label>
              <select
                value={form.table_name}
                onChange={(e) => setForm({ ...form, table_name: e.target.value })}
                className="w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm"
              >
                <option value="">{t("selectTable")}</option>
                {(Array.isArray(tables) ? tables : []).map((tbl: TableInfo) => (
                  <option key={tbl.table_name} value={tbl.table_name}>{tbl.table_name}</option>
                ))}
              </select>
            </div>
          </div>
        )}

        {step === 2 && (
          <div className="space-y-3 p-6">
            <h3 className="text-lg font-medium text-gray-900">{t("stepReview")}</h3>
            <div className="rounded-md bg-gray-50 p-4 space-y-2 text-sm">
              <p><span className="font-medium">{t("operationName")}:</span> {form.name}</p>
              <p><span className="font-medium">{t("operationCode")}:</span> <code className="font-mono">{form.code}</code></p>
              <p><span className="font-medium">{t("operationType")}:</span> {typeLabelMap[form.operation_type] ?? form.operation_type}</p>
              <p><span className="font-medium">{t("project")}:</span> {(projects?.items ?? []).find(p => p.id === form.project_id)?.name ?? "-"}</p>
              {form.table_name && <p><span className="font-medium">{t("tableName")}:</span> <code className="font-mono">{form.table_name}</code></p>}
              {form.module && <p><span className="font-medium">{t("erpModule")}:</span> {form.module}</p>}
            </div>
          </div>
        )}

        <div className="flex items-center justify-between border-t px-6 py-4">
          <Button
            variant="secondary"
            onClick={() => step === 0 ? router.back() : setStep(step - 1)}
          >
            <ChevronLeft className="h-4 w-4" />
            {tCommon("back")}
          </Button>
          {step < 2 ? (
            <Button onClick={() => setStep(step + 1)} disabled={!canNext}>
              {steps[step + 1]}
              <ChevronRight className="h-4 w-4" />
            </Button>
          ) : (
            <Button
              onClick={() => createMutation.mutate(undefined)}
              disabled={createMutation.isPending}
            >
              {createMutation.isPending && <Loader2 className="h-4 w-4 animate-spin" />}
              {t("createAndOpen")}
            </Button>
          )}
        </div>
      </Card>
    </div>
  );
}
