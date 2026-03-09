"use client";

import { useTranslations } from "next-intl";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { useListBuilderStore } from "@/lib/stores/list-builder-store";

export function ColumnPropertyPanel() {
  const t = useTranslations("lowcode");
  const { columns, selectedColumnId, updateColumn } = useListBuilderStore();

  const selectedColumn = selectedColumnId
    ? columns.find((c) => c.id === selectedColumnId)
    : null;

  if (!selectedColumn) {
    return (
      <div className="flex h-full w-72 flex-col border-l border-gray-200 bg-gray-50">
        <div className="border-b border-gray-200 px-4 py-3">
          <h3 className="text-sm font-semibold text-gray-900">
            {t("columnProperties")}
          </h3>
        </div>
        <div className="flex flex-1 items-center justify-center px-4">
          <p className="text-center text-sm text-gray-500">
            {t("selectColumnHint")}
          </p>
        </div>
      </div>
    );
  }

  const update = (updates: Partial<typeof selectedColumn>) =>
    updateColumn(selectedColumn.id, updates);

  return (
    <div className="flex h-full w-72 flex-col border-l border-gray-200 bg-gray-50">
      <div className="border-b border-gray-200 px-4 py-3">
        <h3 className="text-sm font-semibold text-gray-900">
          {t("columnProperties")}
        </h3>
        <p className="text-xs text-gray-500 truncate">
          {selectedColumn.field_key}
        </p>
      </div>
      <div className="flex-1 overflow-y-auto px-4 py-4">
        <div className="space-y-4">
          <Input
            label={t("label")}
            value={selectedColumn.label}
            onChange={(e) => update({ label: e.target.value })}
          />

          <Select
            label={t("dataType")}
            value={selectedColumn.data_type}
            onChange={(e) => update({ data_type: e.target.value })}
            options={[
              { value: "TEXT", label: t("text") },
              { value: "NUMBER", label: t("number") },
              { value: "DATE", label: t("date") },
              { value: "BOOLEAN", label: t("boolean") },
            ]}
          />

          <Input
            label={t("widthPx")}
            type="number"
            value={selectedColumn.width ?? ""}
            placeholder={t("auto")}
            onChange={(e) =>
              update({
                width: e.target.value ? Number(e.target.value) : undefined,
              })
            }
          />

          <Input
            label={t("minWidthPx")}
            type="number"
            value={selectedColumn.min_width ?? ""}
            placeholder="80"
            onChange={(e) =>
              update({
                min_width: e.target.value
                  ? Number(e.target.value)
                  : undefined,
              })
            }
          />

          <hr className="border-gray-200" />
          <h4 className="text-sm font-semibold text-gray-900">{t("behavior")}</h4>

          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={selectedColumn.is_sortable}
              onChange={(e) => update({ is_sortable: e.target.checked })}
              className="h-4 w-4 rounded border-gray-300 text-blue-600"
            />
            <span className="text-sm text-gray-700">{t("sortable")}</span>
          </label>

          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={selectedColumn.is_filterable}
              onChange={(e) => update({ is_filterable: e.target.checked })}
              className="h-4 w-4 rounded border-gray-300 text-blue-600"
            />
            <span className="text-sm text-gray-700">{t("filterable")}</span>
          </label>

          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={selectedColumn.is_visible}
              onChange={(e) => update({ is_visible: e.target.checked })}
              className="h-4 w-4 rounded border-gray-300 text-blue-600"
            />
            <span className="text-sm text-gray-700">{t("visible")}</span>
          </label>

          <hr className="border-gray-200" />
          <h4 className="text-sm font-semibold text-gray-900">{t("formatting")}</h4>

          <Input
            label={t("formatPattern")}
            value={selectedColumn.format_pattern ?? ""}
            placeholder={t("formatPatternExample")}
            onChange={(e) =>
              update({ format_pattern: e.target.value || undefined })
            }
            helperText={t("formatPatternHint")}
          />

          <Select
            label={t("cellRenderer")}
            value={selectedColumn.cell_renderer ?? "default"}
            onChange={(e) =>
              update({
                cell_renderer:
                  e.target.value === "default" ? undefined : e.target.value,
              })
            }
            options={[
              { value: "default", label: t("rendererDefault") },
              { value: "badge", label: t("rendererBadge") },
              { value: "link", label: t("rendererLink") },
              { value: "currency", label: t("rendererCurrency") },
              { value: "boolean", label: t("rendererBoolean") },
              { value: "date", label: t("rendererDate") },
            ]}
          />
        </div>
      </div>
    </div>
  );
}
