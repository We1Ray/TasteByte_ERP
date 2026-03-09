"use client";

import { useState } from "react";
import { Database, Key, Search } from "lucide-react";
import { useTranslations } from "next-intl";
import { Modal } from "@/components/ui/modal";
import { Badge } from "@/components/ui/badge";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { datasourceApi } from "@/lib/api/lowcode";
import type { ColumnInfo, TableInfo } from "@/lib/types/lowcode";

interface TableSchemaBrowserProps {
  open: boolean;
  onClose: () => void;
  onSelect: (columnName: string) => void;
  currentTableName?: string;
}

export function TableSchemaBrowser({
  open,
  onClose,
  onSelect,
  currentTableName,
}: TableSchemaBrowserProps) {
  const [selectedTable, setSelectedTable] = useState<string>(currentTableName || "");
  const [search, setSearch] = useState("");
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");

  const { data: tables, isLoading: tablesLoading } = useApiQuery(
    ["lowcode", "datasource", "tables"],
    () => datasourceApi.tables(),
    { enabled: open }
  );

  const { data: columns, isLoading: columnsLoading } = useApiQuery(
    ["lowcode", "datasource", "columns", selectedTable],
    () => datasourceApi.columns(selectedTable),
    { enabled: open && !!selectedTable }
  );

  const filteredTables = (tables || []).filter((tbl: TableInfo) =>
    tbl.table_name.toLowerCase().includes(search.toLowerCase())
  );

  const handleSelect = (columnName: string) => {
    onSelect(columnName);
    onClose();
  };

  return (
    <Modal open={open} onClose={onClose} title={t("schemaBrowser")} size="xl">
      <div className="flex h-[28rem] gap-4">
        {/* Left: Table list */}
        <div className="flex w-1/3 flex-col border-r border-gray-200 pr-4">
          <div className="mb-3">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400" />
              <input
                type="text"
                placeholder={t("searchTablesPlaceholder")}
                value={search}
                onChange={(e) => setSearch(e.target.value)}
                className="w-full rounded-md border border-gray-300 py-2 pl-9 pr-3 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
              />
            </div>
          </div>
          <div className="flex-1 overflow-y-auto">
            {tablesLoading ? (
              <div className="py-8 text-center text-sm text-gray-500">{t("loadingTables")}</div>
            ) : (
              filteredTables.map((table: TableInfo) => (
                <button
                  key={table.table_name}
                  onClick={() => setSelectedTable(table.table_name)}
                  className={`flex w-full items-center gap-2 rounded-md px-3 py-2 text-left text-sm transition-colors ${
                    selectedTable === table.table_name
                      ? "bg-blue-50 font-medium text-blue-700"
                      : table.table_name === currentTableName
                        ? "bg-amber-50 text-amber-700"
                        : "text-gray-700 hover:bg-gray-100"
                  }`}
                >
                  <Database className="h-3.5 w-3.5 shrink-0" />
                  <span className="truncate">{table.table_name}</span>
                  {table.table_name === currentTableName &&
                    selectedTable !== table.table_name && (
                      <Badge color="amber" className="ml-auto text-[10px]">
                        {t("currentTable")}
                      </Badge>
                    )}
                </button>
              ))
            )}
          </div>
        </div>

        {/* Right: Column list */}
        <div className="flex flex-1 flex-col">
          {!selectedTable ? (
            <div className="flex flex-1 items-center justify-center text-sm text-gray-500">
              {t("selectTableHint")}
            </div>
          ) : columnsLoading ? (
            <div className="flex flex-1 items-center justify-center text-sm text-gray-500">
              {t("loadingColumns")}
            </div>
          ) : (
            <>
              <div className="mb-3 flex items-center gap-2">
                <Database className="h-4 w-4 text-gray-500" />
                <h4 className="text-sm font-semibold text-gray-900">{selectedTable}</h4>
                <Badge color="gray">{t("columnsCount", { count: columns?.length || 0 })}</Badge>
              </div>
              <div className="flex-1 overflow-y-auto">
                <table className="w-full text-sm">
                  <thead>
                    <tr className="border-b text-left text-xs font-medium uppercase text-gray-500">
                      <th className="pb-2 pr-4">{t("columnHeader")}</th>
                      <th className="pb-2 pr-4">{tCommon("type")}</th>
                      <th className="pb-2">{t("attributes")}</th>
                    </tr>
                  </thead>
                  <tbody>
                    {(columns || []).map((col: ColumnInfo) => (
                      <tr
                        key={col.column_name}
                        onClick={() => handleSelect(col.column_name)}
                        className="cursor-pointer border-b border-gray-100 transition-colors hover:bg-blue-50"
                      >
                        <td className="py-2 pr-4 font-medium text-gray-900">
                          <div className="flex items-center gap-1.5">
                            {col.is_primary_key && (
                              <Key className="h-3 w-3 text-amber-500" />
                            )}
                            {col.column_name}
                          </div>
                        </td>
                        <td className="py-2 pr-4 font-mono text-xs text-gray-600">
                          {col.data_type}
                        </td>
                        <td className="py-2">
                          <div className="flex gap-1">
                            {col.is_primary_key && <Badge color="amber">{t("primaryKey")}</Badge>}
                            {!col.is_nullable && <Badge color="red">{t("notNull")}</Badge>}
                            {col.is_nullable && <Badge color="gray">{t("nullable")}</Badge>}
                          </div>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </>
          )}
        </div>
      </div>
    </Modal>
  );
}
