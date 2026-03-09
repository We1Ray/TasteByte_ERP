"use client";

import { Plus, Trash2, GripVertical } from "lucide-react";
import { useTranslations } from "next-intl";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import type { FieldDefinition, MasterDetailFieldConfig } from "@/lib/types/lowcode";

interface MasterDetailConfigPanelProps {
  field: FieldDefinition;
  onUpdate: (updates: Partial<FieldDefinition>) => void;
}

export function MasterDetailConfigPanel({ field, onUpdate }: MasterDetailConfigPanelProps) {
  const t = useTranslations("lowcode");

  const FIELD_TYPE_OPTIONS = [
    { value: "text", label: t("text") },
    { value: "number", label: t("number") },
    { value: "date", label: t("date") },
    { value: "dropdown", label: t("dropdown") },
  ];

  const config = (field.field_config ?? {}) as MasterDetailFieldConfig;
  const detailColumns = config.detailColumns ?? [];

  const updateConfig = (updates: Partial<MasterDetailFieldConfig>) => {
    onUpdate({
      field_config: { ...field.field_config, ...updates },
    });
  };

  const addColumn = () => {
    const newColumn = {
      field_key: "",
      label: "",
      field_type: "text",
      width: 150,
    };
    updateConfig({ detailColumns: [...detailColumns, newColumn] });
  };

  const updateColumn = (
    index: number,
    key: keyof (typeof detailColumns)[0],
    value: string | number
  ) => {
    const updated = [...detailColumns];
    updated[index] = { ...updated[index], [key]: value };
    updateConfig({ detailColumns: updated });
  };

  const removeColumn = (index: number) => {
    updateConfig({ detailColumns: detailColumns.filter((_, i) => i !== index) });
  };

  return (
    <div className="space-y-4">
      <hr className="border-gray-200" />
      <h4 className="text-sm font-semibold text-gray-900">{t("masterDetailSettings")}</h4>

      <Input
        label={t("foreignKey")}
        value={config.foreignKey ?? ""}
        placeholder={t("foreignKeyExample")}
        onChange={(e) => updateConfig({ foreignKey: e.target.value || undefined })}
      />

      <div>
        <div className="mb-2 flex items-center justify-between">
          <label className="block text-sm font-medium text-gray-700">{t("detailColumns")}</label>
          <Button type="button" variant="ghost" size="sm" onClick={addColumn}>
            <Plus className="h-3.5 w-3.5" />
            {t("addColumn")}
          </Button>
        </div>

        {detailColumns.length === 0 ? (
          <p className="rounded-md border border-dashed border-gray-300 px-3 py-4 text-center text-xs text-gray-400">
            {t("noColumnsDefined")}
          </p>
        ) : (
          <div className="space-y-3">
            {detailColumns.map((col, index) => (
              <div
                key={index}
                className="rounded-md border border-gray-200 bg-white p-3"
              >
                <div className="mb-2 flex items-center justify-between">
                  <div className="flex items-center gap-1 text-xs text-gray-400">
                    <GripVertical className="h-3.5 w-3.5" />
                    <span>{t("columnIndex", { index: index + 1 })}</span>
                  </div>
                  <button
                    type="button"
                    onClick={() => removeColumn(index)}
                    className="rounded p-1 text-gray-400 transition-colors hover:bg-red-50 hover:text-red-500"
                    title={t("removeColumn")}
                  >
                    <Trash2 className="h-3.5 w-3.5" />
                  </button>
                </div>

                <div className="space-y-2">
                  <div className="flex gap-2">
                    <div className="flex-1">
                      <input
                        value={col.field_key}
                        onChange={(e) => updateColumn(index, "field_key", e.target.value)}
                        placeholder={t("fieldKeyPlaceholder")}
                        className="w-full rounded-md border border-gray-300 px-2 py-1 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                      />
                    </div>
                    <div className="flex-1">
                      <input
                        value={col.label}
                        onChange={(e) => updateColumn(index, "label", e.target.value)}
                        placeholder={t("labelPlaceholder")}
                        className="w-full rounded-md border border-gray-300 px-2 py-1 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                      />
                    </div>
                  </div>

                  <div className="flex gap-2">
                    <div className="flex-1">
                      <select
                        value={col.field_type}
                        onChange={(e) => updateColumn(index, "field_type", e.target.value)}
                        className="w-full rounded-md border border-gray-300 px-2 py-1 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                      >
                        {FIELD_TYPE_OPTIONS.map((opt) => (
                          <option key={opt.value} value={opt.value}>
                            {opt.label}
                          </option>
                        ))}
                      </select>
                    </div>
                    <div className="w-20">
                      <input
                        type="number"
                        value={col.width ?? 150}
                        onChange={(e) =>
                          updateColumn(index, "width", e.target.value ? Number(e.target.value) : 150)
                        }
                        placeholder={t("widthPlaceholder")}
                        title={t("widthPx")}
                        className="w-full rounded-md border border-gray-300 px-2 py-1 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                      />
                    </div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
