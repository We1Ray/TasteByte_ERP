"use client";

import { useState } from "react";
import { useTranslations } from "next-intl";
import {
  Hash,
  Type,
  Calendar,
  ToggleLeft,
  Plus,
  Search,
  Loader2,
} from "lucide-react";
import { Input } from "@/components/ui/input";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { datasourceApi } from "@/lib/api/lowcode";
import { useListBuilderStore } from "@/lib/stores/list-builder-store";
import type { ColumnInfo } from "@/lib/types/lowcode";

function getColumnIcon(dataType: string) {
  const t = dataType.toLowerCase();
  if (t.includes("int") || t.includes("numeric") || t.includes("float") || t.includes("decimal") || t.includes("double")) {
    return Hash;
  }
  if (t.includes("bool")) return ToggleLeft;
  if (t.includes("date") || t.includes("time") || t.includes("timestamp")) return Calendar;
  return Type;
}

function mapDataType(pgType: string): string {
  const t = pgType.toLowerCase();
  if (t.includes("int") || t.includes("numeric") || t.includes("float") || t.includes("decimal") || t.includes("double")) {
    return "NUMBER";
  }
  if (t.includes("bool")) return "BOOLEAN";
  if (t.includes("date") || t.includes("time") || t.includes("timestamp")) return "DATE";
  return "TEXT";
}

interface ColumnPaletteProps {
  tableName?: string;
}

export function ColumnPalette({ tableName }: ColumnPaletteProps) {
  const t = useTranslations("lowcode");
  const [search, setSearch] = useState("");
  const { addColumn, columns } = useListBuilderStore();

  const { data: tableColumns, isLoading } = useApiQuery(
    ["lowcode", "datasource", "columns", tableName || ""],
    () => datasourceApi.columns(tableName!),
    { enabled: !!tableName }
  );

  const filteredColumns = (tableColumns || []).filter((col: ColumnInfo) =>
    col.column_name.toLowerCase().includes(search.toLowerCase())
  );

  const existingKeys = new Set(columns.map((c) => c.field_key));

  const handleAddColumn = (col: ColumnInfo) => {
    if (existingKeys.has(col.column_name)) return;
    addColumn({
      field_key: col.column_name,
      label: col.column_name
        .replace(/_/g, " ")
        .replace(/\b\w/g, (c) => c.toUpperCase()),
      data_type: mapDataType(col.data_type),
      is_sortable: true,
      is_filterable: false,
      is_visible: true,
      sort_order: columns.length,
    });
  };

  return (
    <div className="flex h-full w-60 flex-col border-r border-gray-200 bg-gray-50">
      <div className="border-b border-gray-200 px-4 py-3">
        <h3 className="text-sm font-semibold text-gray-900">{t("tableColumns")}</h3>
        <p className="text-xs text-gray-500">
          {tableName
            ? t("fromTable", { table: tableName })
            : t("noTargetTable")}
        </p>
      </div>

      <div className="px-3 pt-3">
        <div className="relative">
          <Search className="absolute left-2.5 top-2.5 h-3.5 w-3.5 text-gray-400" />
          <Input
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            placeholder={t("searchColumnsPlaceholder")}
            className="pl-8 text-xs"
          />
        </div>
      </div>

      <div className="flex-1 overflow-y-auto px-3 py-3">
        {isLoading && (
          <div className="flex items-center justify-center py-8">
            <Loader2 className="h-5 w-5 animate-spin text-gray-400" />
          </div>
        )}

        {!tableName && (
          <p className="py-8 text-center text-xs text-gray-400">
            {t("configureTargetTable")}
          </p>
        )}

        {tableName && !isLoading && filteredColumns.length === 0 && (
          <p className="py-8 text-center text-xs text-gray-400">
            No columns found
          </p>
        )}

        <div className="space-y-1.5">
          {filteredColumns.map((col: ColumnInfo) => {
            const Icon = getColumnIcon(col.data_type);
            const isAdded = existingKeys.has(col.column_name);

            return (
              <button
                key={col.column_name}
                onClick={() => handleAddColumn(col)}
                disabled={isAdded}
                className={`flex w-full items-center gap-2 rounded-md border px-3 py-2 text-left text-sm transition-colors ${
                  isAdded
                    ? "border-green-200 bg-green-50 text-green-700 opacity-60"
                    : "border-gray-200 bg-white text-gray-700 hover:border-blue-300 hover:bg-blue-50"
                }`}
              >
                <Icon className="h-4 w-4 shrink-0 text-gray-400" />
                <div className="min-w-0 flex-1">
                  <p className="truncate text-sm">{col.column_name}</p>
                  <p className="text-xs text-gray-400">
                    {col.data_type}
                    {col.is_primary_key && " (PK)"}
                    {!col.is_nullable && " NOT NULL"}
                  </p>
                </div>
                {!isAdded && <Plus className="h-3.5 w-3.5 shrink-0 text-gray-400" />}
              </button>
            );
          })}
        </div>
      </div>
    </div>
  );
}
