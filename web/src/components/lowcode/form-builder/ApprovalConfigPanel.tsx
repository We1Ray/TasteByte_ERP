"use client";

import { Plus, Trash2, GripVertical } from "lucide-react";
import { useTranslations } from "next-intl";
import { Button } from "@/components/ui/button";
import { useBuilderStore } from "@/lib/stores/builder-store";
import type { FieldDefinition, ApprovalButtonsFieldConfig } from "@/lib/types/lowcode";

interface ApprovalConfigPanelProps {
  field: FieldDefinition;
  onUpdate: (updates: Partial<FieldDefinition>) => void;
}

const colorPreview: Record<string, string> = {
  blue: "bg-blue-500",
  green: "bg-green-500",
  red: "bg-red-500",
  yellow: "bg-yellow-500",
  gray: "bg-gray-500",
};

export function ApprovalConfigPanel({ field, onUpdate }: ApprovalConfigPanelProps) {
  const t = useTranslations("lowcode");

  const COLOR_OPTIONS = [
    { value: "blue", label: t("colorBlue") },
    { value: "green", label: t("colorGreen") },
    { value: "red", label: t("colorRed") },
    { value: "yellow", label: t("colorYellow") },
    { value: "gray", label: t("colorGray") },
  ];

  const config = (field.field_config ?? {}) as ApprovalButtonsFieldConfig;
  const actions = config.actions ?? [];
  const statusField = config.statusField ?? "";

  // Get all other fields from the builder store to populate the status field dropdown
  const { sections } = useBuilderStore();
  const allFields: FieldDefinition[] = [];
  for (const section of sections) {
    for (const f of section.fields) {
      if (f.id !== field.id) {
        allFields.push(f);
      }
    }
  }

  const updateConfig = (updates: Partial<ApprovalButtonsFieldConfig>) => {
    onUpdate({
      field_config: { ...field.field_config, ...updates },
    });
  };

  const addAction = () => {
    const newAction = {
      label: "",
      targetStatus: "",
      requireComment: false,
      color: "blue",
    };
    updateConfig({ actions: [...actions, newAction] });
  };

  const updateAction = (
    index: number,
    key: string,
    value: string | boolean
  ) => {
    const updated = [...actions];
    updated[index] = { ...updated[index], [key]: value };
    updateConfig({ actions: updated });
  };

  const removeAction = (index: number) => {
    updateConfig({ actions: actions.filter((_, i) => i !== index) });
  };

  return (
    <div className="space-y-4">
      <hr className="border-gray-200" />
      <h4 className="text-sm font-semibold text-gray-900">{t("approvalSettings")}</h4>

      {/* Status Field selector */}
      <div>
        <label className="mb-1 block text-sm font-medium text-gray-700">{t("statusField")}</label>
        <select
          value={statusField}
          onChange={(e) => updateConfig({ statusField: e.target.value || undefined })}
          className="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
        >
          <option value="">{t("selectFieldPlaceholder")}</option>
          {allFields.map((f) => (
            <option key={f.id} value={f.field_key}>
              {f.label} ({f.field_key})
            </option>
          ))}
        </select>
        <p className="mt-1 text-xs text-gray-400">
          {t("statusFieldHint")}
        </p>
      </div>

      {/* Actions list */}
      <div>
        <div className="mb-2 flex items-center justify-between">
          <label className="block text-sm font-medium text-gray-700">{t("actions")}</label>
          <Button type="button" variant="ghost" size="sm" onClick={addAction}>
            <Plus className="h-3.5 w-3.5" />
            {t("addAction")}
          </Button>
        </div>

        {actions.length === 0 ? (
          <p className="rounded-md border border-dashed border-gray-300 px-3 py-4 text-center text-xs text-gray-400">
            {t("noActionsDefined")}
          </p>
        ) : (
          <div className="space-y-3">
            {actions.map((action, index) => (
              <div
                key={index}
                className="rounded-md border border-gray-200 bg-white p-3"
              >
                <div className="mb-2 flex items-center justify-between">
                  <div className="flex items-center gap-1 text-xs text-gray-400">
                    <GripVertical className="h-3.5 w-3.5" />
                    <span>{t("actionIndex", { index: index + 1 })}</span>
                  </div>
                  <button
                    type="button"
                    onClick={() => removeAction(index)}
                    className="rounded p-1 text-gray-400 transition-colors hover:bg-red-50 hover:text-red-500"
                    title={t("removeAction")}
                  >
                    <Trash2 className="h-3.5 w-3.5" />
                  </button>
                </div>

                <div className="space-y-2">
                  {/* Label */}
                  <div>
                    <label className="mb-0.5 block text-xs text-gray-500">{t("buttonLabel")}</label>
                    <input
                      value={action.label}
                      onChange={(e) => updateAction(index, "label", e.target.value)}
                      placeholder={t("buttonLabelExample")}
                      className="w-full rounded-md border border-gray-300 px-2 py-1 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                    />
                  </div>

                  {/* Target Status */}
                  <div>
                    <label className="mb-0.5 block text-xs text-gray-500">{t("targetStatus")}</label>
                    <input
                      value={action.targetStatus}
                      onChange={(e) => updateAction(index, "targetStatus", e.target.value)}
                      placeholder={t("targetStatusExample")}
                      className="w-full rounded-md border border-gray-300 px-2 py-1 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                    />
                  </div>

                  {/* Color and Require Comment row */}
                  <div className="flex items-end gap-2">
                    <div className="flex-1">
                      <label className="mb-0.5 block text-xs text-gray-500">{t("color")}</label>
                      <div className="flex items-center gap-2">
                        <div
                          className={`h-4 w-4 rounded-full ${colorPreview[action.color ?? "blue"] ?? colorPreview.blue}`}
                        />
                        <select
                          value={action.color ?? "blue"}
                          onChange={(e) => updateAction(index, "color", e.target.value)}
                          className="flex-1 rounded-md border border-gray-300 px-2 py-1 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                        >
                          {COLOR_OPTIONS.map((opt) => (
                            <option key={opt.value} value={opt.value}>
                              {opt.label}
                            </option>
                          ))}
                        </select>
                      </div>
                    </div>
                  </div>

                  {/* Require Comment checkbox */}
                  <label className="flex items-center gap-2">
                    <input
                      type="checkbox"
                      checked={action.requireComment ?? false}
                      onChange={(e) => updateAction(index, "requireComment", e.target.checked)}
                      className="h-4 w-4 rounded border-gray-300 text-blue-600"
                    />
                    <span className="text-sm text-gray-700">{t("requireComment")}</span>
                  </label>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
