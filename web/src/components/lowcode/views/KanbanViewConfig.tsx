"use client";

import { Plus, Trash2, ArrowUp, ArrowDown } from "lucide-react";
import { useTranslations } from "next-intl";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import type { KanbanViewConfig } from "@/lib/types/lowcode";

interface KanbanViewConfigProps {
  config: KanbanViewConfig;
  onChange: (config: KanbanViewConfig) => void;
  availableFields: { key: string; label: string }[];
}

export function KanbanViewConfigPanel({
  config,
  onChange,
  availableFields,
}: KanbanViewConfigProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const fieldOptions = availableFields.map((f) => ({
    value: f.key,
    label: f.label,
  }));

  const optionalFieldOptions = [
    { value: "", label: "-- None --" },
    ...fieldOptions,
  ];

  const addColumn = () => {
    onChange({
      ...config,
      columns: [
        ...config.columns,
        { value: "", label: "", color: "#6B7280" },
      ],
    });
  };

  const removeColumn = (index: number) => {
    const newColumns = config.columns.filter((_, i) => i !== index);
    onChange({ ...config, columns: newColumns });
  };

  const updateColumn = (
    index: number,
    field: "value" | "label" | "color",
    value: string
  ) => {
    const newColumns = config.columns.map((col, i) =>
      i === index ? { ...col, [field]: value } : col
    );
    onChange({ ...config, columns: newColumns });
  };

  const moveColumn = (index: number, direction: -1 | 1) => {
    const targetIndex = index + direction;
    if (targetIndex < 0 || targetIndex >= config.columns.length) return;
    const newColumns = [...config.columns];
    const [moved] = newColumns.splice(index, 1);
    newColumns.splice(targetIndex, 0, moved);
    onChange({ ...config, columns: newColumns });
  };

  return (
    <div className="space-y-4">
      <h4 className="text-sm font-semibold text-gray-900">{t("kanbanSettings")}</h4>

      <Select
        label={t("statusColumn")}
        required
        options={fieldOptions}
        placeholder="Select status field"
        value={config.statusField || ""}
        onChange={(e) =>
          onChange({ ...config, statusField: e.target.value })
        }
      />

      <Select
        label={t("titleField")}
        required
        options={fieldOptions}
        placeholder="Select title field"
        value={config.titleField || ""}
        onChange={(e) =>
          onChange({ ...config, titleField: e.target.value })
        }
      />

      <Select
        label={`${tCommon("description")} (${tCommon("optional")})`}
        options={optionalFieldOptions}
        value={config.descriptionField || ""}
        onChange={(e) =>
          onChange({
            ...config,
            descriptionField: e.target.value || undefined,
          })
        }
      />

      {/* Columns editor */}
      <div className="space-y-2">
        <div className="flex items-center justify-between">
          <label className="text-sm font-medium text-gray-700">{t("kanbanColumns")}</label>
          <Button type="button" variant="ghost" size="sm" onClick={addColumn}>
            <Plus className="h-3.5 w-3.5" />
            {t("addColumn")}
          </Button>
        </div>

        {config.columns.length === 0 && (
          <p className="text-xs text-gray-400 py-2">
            {t("noColumnsDefined")}
          </p>
        )}

        <div className="space-y-2">
          {config.columns.map((col, index) => (
            <div
              key={index}
              className="flex items-center gap-2 rounded-md border border-gray-200 bg-gray-50 p-2"
            >
              <input
                type="color"
                value={col.color}
                onChange={(e) => updateColumn(index, "color", e.target.value)}
                className="h-7 w-7 cursor-pointer rounded border border-gray-300 p-0.5"
                title={t("color")}
              />

              <Input
                placeholder="Value"
                value={col.value}
                onChange={(e) => updateColumn(index, "value", e.target.value)}
                className="flex-1 text-xs"
              />

              <Input
                placeholder={t("label")}
                value={col.label}
                onChange={(e) => updateColumn(index, "label", e.target.value)}
                className="flex-1 text-xs"
              />

              <div className="flex items-center gap-0.5">
                <button
                  type="button"
                  onClick={() => moveColumn(index, -1)}
                  disabled={index === 0}
                  className="rounded p-0.5 text-gray-400 hover:text-gray-600 disabled:opacity-30"
                  title={t("moveUp")}
                >
                  <ArrowUp className="h-3.5 w-3.5" />
                </button>
                <button
                  type="button"
                  onClick={() => moveColumn(index, 1)}
                  disabled={index === config.columns.length - 1}
                  className="rounded p-0.5 text-gray-400 hover:text-gray-600 disabled:opacity-30"
                  title={t("moveDown")}
                >
                  <ArrowDown className="h-3.5 w-3.5" />
                </button>
                <button
                  type="button"
                  onClick={() => removeColumn(index)}
                  className="rounded p-0.5 text-red-400 hover:text-red-600"
                  title={t("removeColumn")}
                >
                  <Trash2 className="h-3.5 w-3.5" />
                </button>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
