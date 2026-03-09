"use client";

import { useState, useMemo, useCallback } from "react";
import { useRouter } from "next/navigation";
import {
  useReactTable,
  getCoreRowModel,
  flexRender,
  type ColumnDef,
  type SortingState,
} from "@tanstack/react-table";
import { useTranslations } from "next-intl";
import {
  Search,
  ChevronLeft,
  ChevronRight,
  ChevronsLeft,
  ChevronsRight,
  ArrowUpDown,
  ArrowUp,
  ArrowDown,
  Upload,
  Loader2,
} from "lucide-react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Badge, StatusBadge } from "@/components/ui/badge";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { listApi, listExecutorApi, executorApi } from "@/lib/api/lowcode";
import { formatDate, formatCurrency } from "@/lib/utils";
import { ExcelExportButton } from "./ExcelExportButton";
import { CsvImportModal } from "./CsvImportModal";
import { toast } from "sonner";
import type { ListColumn, ListAction } from "@/lib/types/lowcode";

interface ListRendererProps {
  operationId: string;
  operationCode: string;
}

function formatCellValue(
  value: unknown,
  column: ListColumn
): React.ReactNode {
  if (value === null || value === undefined) return "-";

  const renderer = column.cell_renderer || "default";

  switch (renderer) {
    case "badge":
      return <StatusBadge status={String(value)} />;
    case "currency":
      return formatCurrency(Number(value));
    case "date":
      return formatDate(String(value));
    case "boolean":
      return (
        <Badge color={value ? "green" : "gray"}>
          {value ? "Yes" : "No"}
        </Badge>
      );
    case "link":
      return (
        <span className="text-blue-600 hover:underline cursor-pointer">
          {String(value)}
        </span>
      );
    default:
      if (column.data_type === "DATE" && typeof value === "string") {
        return formatDate(value);
      }
      if (column.data_type === "NUMBER" && typeof value === "number") {
        if (column.format_pattern) {
          return new Intl.NumberFormat("en-US", {
            minimumFractionDigits: 2,
            maximumFractionDigits: 2,
          }).format(value);
        }
        return String(value);
      }
      if (column.data_type === "BOOLEAN") {
        return (
          <Badge color={value ? "green" : "gray"}>
            {value ? "Yes" : "No"}
          </Badge>
        );
      }
      return String(value);
  }
}

export function ListRenderer({
  operationId,
  operationCode,
}: ListRendererProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const router = useRouter();
  const [page, setPage] = useState(1);
  const [searchTerm, setSearchTerm] = useState("");
  const [sorting, setSorting] = useState<SortingState>([]);
  const [columnFilters, setColumnFilters] = useState<
    Record<string, string>
  >({});
  const [importOpen, setImportOpen] = useState(false);

  // Fetch list definition
  const { data: listDef, isLoading: defLoading } = useApiQuery(
    ["lowcode", "list", operationId],
    () => listApi.getDefinition(operationId),
    { enabled: !!operationId }
  );

  const pageSize = listDef?.default_page_size || 20;

  // Build query params
  const queryParams = useMemo(() => {
    const params: Record<string, unknown> = {
      page,
      page_size: pageSize,
    };
    if (searchTerm) params.search = searchTerm;
    if (sorting.length > 0) {
      params.sort_by = sorting[0].id;
      params.sort_order = sorting[0].desc ? "desc" : "asc";
    }
    Object.entries(columnFilters).forEach(([key, val]) => {
      if (val) params[`filter_${key}`] = val;
    });
    return params;
  }, [page, pageSize, searchTerm, sorting, columnFilters]);

  // Fetch data
  const {
    data: queryResult,
    isLoading: dataLoading,
    refetch,
  } = useApiQuery(
    ["lowcode", "list-data", operationCode, JSON.stringify(queryParams)],
    () => listExecutorApi.query(operationCode, queryParams),
    { enabled: !!operationCode && !!listDef }
  );

  const items = queryResult?.items ?? [];
  const totalPages = queryResult?.total_pages ?? 1;
  const total = queryResult?.total ?? 0;

  const sortedColumnDefs = useMemo(() => {
    if (!listDef?.columns) return [];
    return [...listDef.columns]
      .filter((c) => c.is_visible)
      .sort((a, b) => a.sort_order - b.sort_order);
  }, [listDef]);

  const handleAction = useCallback(
    async (action: ListAction, row: Record<string, unknown>) => {
      const rowId = String(row.id || row.ID || "");

      switch (action.action_type) {
        case "navigate": {
          const url = (action.target_url || "")
            .replace("{id}", rowId)
            .replace("{code}", operationCode);
          router.push(url);
          break;
        }
        case "delete": {
          const msg =
            action.confirm_message ||
            "Are you sure you want to delete this record?";
          if (window.confirm(msg)) {
            try {
              await executorApi.delete(operationCode, rowId);
              toast.success("Record deleted successfully");
              refetch();
            } catch {
              toast.error("Failed to delete record");
            }
          }
          break;
        }
        default:
          break;
      }
    },
    [operationCode, router, refetch]
  );

  // Build tanstack columns
  const tableColumns = useMemo<ColumnDef<Record<string, unknown>>[]>(() => {
    const cols: ColumnDef<Record<string, unknown>>[] = sortedColumnDefs.map(
      (colDef) => ({
        id: colDef.field_key,
        accessorKey: colDef.field_key,
        header: () => (
          <div className="flex items-center gap-1">
            <span>{colDef.label}</span>
            {colDef.is_sortable && (
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  setSorting((prev) => {
                    const existing = prev.find(
                      (s) => s.id === colDef.field_key
                    );
                    if (!existing) {
                      return [{ id: colDef.field_key, desc: false }];
                    }
                    if (!existing.desc) {
                      return [{ id: colDef.field_key, desc: true }];
                    }
                    return [];
                  });
                }}
                className="text-gray-400 hover:text-gray-600"
              >
                {sorting.find((s) => s.id === colDef.field_key)?.desc ===
                false ? (
                  <ArrowUp className="h-3 w-3" />
                ) : sorting.find((s) => s.id === colDef.field_key)?.desc ===
                  true ? (
                  <ArrowDown className="h-3 w-3" />
                ) : (
                  <ArrowUpDown className="h-3 w-3" />
                )}
              </button>
            )}
          </div>
        ),
        cell: ({ getValue }) => formatCellValue(getValue(), colDef),
        size: colDef.width || 150,
        minSize: colDef.min_width || 80,
      })
    );

    // Actions column
    if (listDef?.actions && listDef.actions.length > 0) {
      cols.push({
        id: "_actions",
        header: t("actionsColumn"),
        size: 120,
        cell: ({ row }) => (
          <div className="flex items-center gap-1">
            {listDef!.actions.map((action) => (
              <button
                key={action.id}
                onClick={() =>
                  handleAction(
                    action,
                    row.original as Record<string, unknown>
                  )
                }
                className="rounded px-2 py-1 text-xs text-blue-600 hover:bg-blue-50"
                title={action.label}
              >
                {action.label}
              </button>
            ))}
          </div>
        ),
      });
    }

    return cols;
  }, [sortedColumnDefs, listDef, sorting, handleAction]);

  const table = useReactTable({
    data: items,
    columns: tableColumns,
    getCoreRowModel: getCoreRowModel(),
    manualSorting: true,
    manualPagination: true,
    pageCount: totalPages,
    state: {
      sorting,
    },
    onSortingChange: setSorting,
  });

  if (defLoading) {
    return (
      <div className="flex h-64 items-center justify-center">
        <Loader2 className="h-8 w-8 animate-spin text-gray-400" />
      </div>
    );
  }

  if (!listDef) {
    return (
      <div className="flex h-64 items-center justify-center">
        <p className="text-sm text-gray-500">
          List definition not found. Configure it in the builder first.
        </p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {/* Toolbar */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          {listDef.enable_search && (
            <div className="relative w-72">
              <Search className="absolute left-3 top-2.5 h-4 w-4 text-gray-400" />
              <Input
                value={searchTerm}
                onChange={(e) => {
                  setSearchTerm(e.target.value);
                  setPage(1);
                }}
                placeholder={t("searchPlaceholder")}
                className="pl-9"
              />
            </div>
          )}
        </div>
        <div className="flex items-center gap-2">
          {listDef.enable_import && (
            <Button
              variant="secondary"
              size="sm"
              onClick={() => setImportOpen(true)}
            >
              <Upload className="h-4 w-4" />
              {tCommon("import")}
            </Button>
          )}
          {listDef.enable_export && (
            <ExcelExportButton
              operationCode={operationCode}
              columns={sortedColumnDefs}
              totalRecords={total}
            />
          )}
        </div>
      </div>

      {/* Filter row */}
      {sortedColumnDefs.some((c) => c.is_filterable) && (
        <div className="flex items-center gap-2 flex-wrap">
          {sortedColumnDefs
            .filter((c) => c.is_filterable)
            .map((col) => (
              <div key={col.id} className="w-40">
                <Input
                  value={columnFilters[col.field_key] ?? ""}
                  onChange={(e) => {
                    setColumnFilters((prev) => ({
                      ...prev,
                      [col.field_key]: e.target.value,
                    }));
                    setPage(1);
                  }}
                  placeholder={t("filterColumn", { column: col.label })}
                  className="text-xs"
                />
              </div>
            ))}
        </div>
      )}

      {/* Table */}
      <div className="overflow-hidden rounded-lg border border-gray-200 bg-white shadow-sm">
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              {table.getHeaderGroups().map((headerGroup) => (
                <tr key={headerGroup.id}>
                  {headerGroup.headers.map((header) => (
                    <th
                      key={header.id}
                      className="px-4 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-600"
                      style={{
                        width: header.getSize(),
                        minWidth: header.column.columnDef.minSize,
                      }}
                    >
                      {header.isPlaceholder
                        ? null
                        : flexRender(
                            header.column.columnDef.header,
                            header.getContext()
                          )}
                    </th>
                  ))}
                </tr>
              ))}
            </thead>
            <tbody className="divide-y divide-gray-100">
              {dataLoading ? (
                <tr>
                  <td
                    colSpan={tableColumns.length}
                    className="px-4 py-12 text-center"
                  >
                    <Loader2 className="mx-auto h-6 w-6 animate-spin text-gray-400" />
                    <p className="mt-2 text-sm text-gray-500">{t("loadingData")}</p>
                  </td>
                </tr>
              ) : items.length === 0 ? (
                <tr>
                  <td
                    colSpan={tableColumns.length}
                    className="px-4 py-12 text-center"
                  >
                    <p className="text-sm text-gray-500">{t("noRecordsFound")}</p>
                  </td>
                </tr>
              ) : (
                table.getRowModel().rows.map((row) => (
                  <tr
                    key={row.id}
                    className="transition-colors hover:bg-gray-50"
                  >
                    {row.getVisibleCells().map((cell) => (
                      <td
                        key={cell.id}
                        className="whitespace-nowrap px-4 py-3 text-sm text-gray-700"
                        style={{
                          width: cell.column.getSize(),
                          minWidth: cell.column.columnDef.minSize,
                        }}
                      >
                        {flexRender(
                          cell.column.columnDef.cell,
                          cell.getContext()
                        )}
                      </td>
                    ))}
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>

        {/* Pagination */}
        <div className="flex items-center justify-between border-t bg-gray-50 px-4 py-3">
          <p className="text-sm text-gray-500">
            {t("recordsCount", { count: items.length, total })}
          </p>
          <div className="flex items-center gap-1">
            <Button
              variant="ghost"
              size="icon"
              disabled={page <= 1}
              onClick={() => setPage(1)}
            >
              <ChevronsLeft className="h-4 w-4" />
            </Button>
            <Button
              variant="ghost"
              size="icon"
              disabled={page <= 1}
              onClick={() => setPage((p) => p - 1)}
            >
              <ChevronLeft className="h-4 w-4" />
            </Button>
            <span className="px-3 text-sm text-gray-700">
              {t("pageOfTotal", { page, totalPages })}
            </span>
            <Button
              variant="ghost"
              size="icon"
              disabled={page >= totalPages}
              onClick={() => setPage((p) => p + 1)}
            >
              <ChevronRight className="h-4 w-4" />
            </Button>
            <Button
              variant="ghost"
              size="icon"
              disabled={page >= totalPages}
              onClick={() => setPage(totalPages)}
            >
              <ChevronsRight className="h-4 w-4" />
            </Button>
          </div>
        </div>
      </div>

      {/* Import Modal */}
      {listDef.enable_import && (
        <CsvImportModal
          open={importOpen}
          onClose={() => setImportOpen(false)}
          operationCode={operationCode}
          columns={sortedColumnDefs}
          onImportComplete={() => refetch()}
        />
      )}
    </div>
  );
}
