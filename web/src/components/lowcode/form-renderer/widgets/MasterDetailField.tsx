"use client";

import { useMemo, useCallback } from "react";
import {
  useReactTable,
  getCoreRowModel,
  flexRender,
  type ColumnDef,
} from "@tanstack/react-table";
import { Plus, Trash2 } from "lucide-react";
import { useTranslations } from "next-intl";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import type { FieldDefinition, MasterDetailFieldConfig } from "@/lib/types/lowcode";

interface MasterDetailFieldProps {
  field: FieldDefinition;
  value: { items: Record<string, unknown>[] };
  onChange: (value: { items: Record<string, unknown>[] }) => void;
  error?: string;
  disabled?: boolean;
}

function InlineCell({
  fieldType,
  value,
  onChange,
  disabled,
}: {
  fieldType: string;
  value: unknown;
  onChange: (val: unknown) => void;
  disabled?: boolean;
}) {
  const baseClass =
    "w-full rounded border border-gray-200 bg-white px-2 py-1 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 disabled:bg-gray-50 disabled:text-gray-500";

  switch (fieldType) {
    case "number":
      return (
        <input
          type="number"
          value={value != null ? String(value) : ""}
          onChange={(e) => onChange(e.target.value ? Number(e.target.value) : null)}
          disabled={disabled}
          className={baseClass}
        />
      );
    case "date":
      return (
        <input
          type="date"
          value={value != null ? String(value) : ""}
          onChange={(e) => onChange(e.target.value || null)}
          disabled={disabled}
          className={baseClass}
        />
      );
    case "dropdown":
      return (
        <select
          value={value != null ? String(value) : ""}
          onChange={(e) => onChange(e.target.value || null)}
          disabled={disabled}
          className={baseClass}
        >
          <option value="">--</option>
        </select>
      );
    default:
      // text input
      return (
        <input
          type="text"
          value={value != null ? String(value) : ""}
          onChange={(e) => onChange(e.target.value)}
          disabled={disabled}
          className={baseClass}
        />
      );
  }
}

export function MasterDetailField({
  field,
  value,
  onChange,
  error,
  disabled,
}: MasterDetailFieldProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const config = (field.field_config ?? {}) as MasterDetailFieldConfig;
  // eslint-disable-next-line react-hooks/exhaustive-deps
  const detailColumns = useMemo(() => config.detailColumns ?? [], [JSON.stringify(config.detailColumns)]);

  // eslint-disable-next-line react-hooks/exhaustive-deps
  const items = useMemo(() => value?.items ?? [], [JSON.stringify(value?.items)]);

  const updateCell = useCallback(
    (rowIndex: number, fieldKey: string, cellValue: unknown) => {
      const newItems = [...items];
      newItems[rowIndex] = { ...newItems[rowIndex], [fieldKey]: cellValue };
      onChange({ items: newItems });
    },
    [items, onChange]
  );

  const addRow = useCallback(() => {
    const newRow: Record<string, unknown> = {};
    for (const col of detailColumns) {
      newRow[col.field_key] = col.field_type === "number" ? null : "";
    }
    onChange({ items: [...items, newRow] });
  }, [items, onChange, detailColumns]);

  const deleteRow = useCallback(
    (index: number) => {
      const newItems = items.filter((_, i) => i !== index);
      onChange({ items: newItems });
    },
    [items, onChange]
  );

  const columns: ColumnDef<Record<string, unknown>>[] = useMemo(() => {
    const cols: ColumnDef<Record<string, unknown>>[] = [
      {
        id: "_row_number",
        header: "#",
        size: 40,
        cell: ({ row }) => (
          <span className="text-xs text-gray-400">{row.index + 1}</span>
        ),
      },
      ...detailColumns.map((col) => ({
        accessorKey: col.field_key,
        header: col.label,
        size: col.width ?? 150,
        cell: ({ row, getValue }: { row: { index: number }; getValue: () => unknown }) => (
          <InlineCell
            fieldType={col.field_type}
            value={getValue()}
            onChange={(v) => updateCell(row.index, col.field_key, v)}
            disabled={disabled || field.is_readonly}
          />
        ),
      })),
    ];

    // Add delete column when not disabled
    if (!disabled && !field.is_readonly) {
      cols.push({
        id: "_actions",
        header: "",
        size: 40,
        cell: ({ row }) => (
          <button
            type="button"
            onClick={() => deleteRow(row.index)}
            className="rounded p-1 text-gray-400 transition-colors hover:bg-red-50 hover:text-red-500"
            title={tCommon("delete")}
          >
            <Trash2 className="h-4 w-4" />
          </button>
        ),
      });
    }

    return cols;
  }, [detailColumns, updateCell, deleteRow, disabled, field.is_readonly]);

  const table = useReactTable({
    data: items,
    columns,
    getCoreRowModel: getCoreRowModel(),
  });

  return (
    <div className="w-full">
      {field.label && (
        <label className="mb-1 block text-sm font-medium text-gray-700">
          {field.label}
          {field.validation.required && <span className="ml-0.5 text-red-500">*</span>}
        </label>
      )}

      <div
        className={cn(
          "overflow-auto rounded-md border shadow-sm",
          error ? "border-red-300" : "border-gray-200"
        )}
      >
        <table className="w-full text-sm">
          <thead>
            {table.getHeaderGroups().map((headerGroup) => (
              <tr key={headerGroup.id} className="border-b border-gray-200 bg-gray-50">
                {headerGroup.headers.map((header) => (
                  <th
                    key={header.id}
                    className="px-2 py-2 text-left text-xs font-semibold uppercase tracking-wider text-gray-500"
                    style={{ width: header.getSize() }}
                  >
                    {header.isPlaceholder
                      ? null
                      : flexRender(header.column.columnDef.header, header.getContext())}
                  </th>
                ))}
              </tr>
            ))}
          </thead>
          <tbody>
            {table.getRowModel().rows.length === 0 ? (
              <tr>
                <td
                  colSpan={columns.length}
                  className="px-3 py-6 text-center text-sm text-gray-400"
                >
                  {tCommon("noData")}
                </td>
              </tr>
            ) : (
              table.getRowModel().rows.map((row) => (
                <tr
                  key={row.id}
                  className="border-b border-gray-100 transition-colors hover:bg-gray-50"
                >
                  {row.getVisibleCells().map((cell) => (
                    <td key={cell.id} className="px-2 py-1.5">
                      {flexRender(cell.column.columnDef.cell, cell.getContext())}
                    </td>
                  ))}
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {!disabled && !field.is_readonly && (
        <div className="mt-2">
          <Button type="button" variant="secondary" size="sm" onClick={addRow}>
            <Plus className="h-4 w-4" />
            {t("addColumn")}
          </Button>
        </div>
      )}

      {field.help_text && !error && (
        <p className="mt-1 text-sm text-gray-500">{field.help_text}</p>
      )}
      {error && <p className="mt-1 text-sm text-red-600">{error}</p>}
    </div>
  );
}
