"use client";

import {
  flexRender,
  getCoreRowModel,
  getSortedRowModel,
  useReactTable,
  type ColumnDef,
  type SortingState,
  type RowSelectionState,
} from "@tanstack/react-table";
import { useState, useEffect, useMemo } from "react";
import { ArrowUpDown, ChevronLeft, ChevronRight } from "lucide-react";
import { useTranslations } from "next-intl";
import { cn } from "@/lib/utils";
import { Button } from "./button";
import { EmptyState } from "./empty-state";
import { PageSizeSelector } from "./page-size-selector";

interface DataTableProps<TData> {
  columns: ColumnDef<TData, unknown>[];
  data: TData[];
  page?: number;
  pageSize?: number;
  total?: number;
  totalPages?: number;
  onPageChange?: (page: number) => void;
  onPageSizeChange?: (size: number) => void;
  onRowClick?: (row: TData) => void;
  isLoading?: boolean;
  emptyTitle?: string;
  emptyDescription?: string;
  enableSelection?: boolean;
  onSelectionChange?: (rows: TData[]) => void;
  getRowId?: (row: TData) => string;
}

export function DataTable<TData>({
  columns,
  data,
  page = 1,
  pageSize = 20,
  total = 0,
  totalPages = 1,
  onPageChange,
  onPageSizeChange,
  onRowClick,
  isLoading,
  emptyTitle,
  emptyDescription,
  enableSelection,
  onSelectionChange,
  getRowId,
}: DataTableProps<TData>) {
  const [sorting, setSorting] = useState<SortingState>([]);
  const [rowSelection, setRowSelection] = useState<RowSelectionState>({});
  const t = useTranslations("common");
  const resolvedEmptyTitle = emptyTitle ?? t("noDataFound");

  const selectionColumn: ColumnDef<TData, unknown> = {
    id: "select",
    header: ({ table }) => (
      <input
        type="checkbox"
        className="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
        checked={table.getIsAllPageRowsSelected()}
        onChange={table.getToggleAllPageRowsSelectedHandler()}
      />
    ),
    cell: ({ row }) => (
      <input
        type="checkbox"
        className="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
        checked={row.getIsSelected()}
        onChange={row.getToggleSelectedHandler()}
        onClick={(e) => e.stopPropagation()}
      />
    ),
    enableSorting: false,
  };

  const allColumns = useMemo<ColumnDef<TData, unknown>[]>(
    () => (enableSelection ? [selectionColumn, ...columns] : columns),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [enableSelection, columns]
  );

  const table = useReactTable<TData>({
    data,
    columns: allColumns,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    onSortingChange: setSorting,
    state: enableSelection ? { sorting, rowSelection } : { sorting },
    onRowSelectionChange: enableSelection ? setRowSelection : undefined,
    enableRowSelection: enableSelection ?? false,
    getRowId: enableSelection
      ? getRowId || ((row) => (row as { id: string }).id)
      : undefined,
  });

  useEffect(() => {
    if (enableSelection && onSelectionChange) {
      const selectedRows = table.getSelectedRowModel().rows.map((row) => row.original);
      onSelectionChange(selectedRows);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [rowSelection, enableSelection]);

  useEffect(() => {
    setRowSelection({});
  }, [data]);

  if (isLoading) {
    return (
      <div className="overflow-hidden rounded-lg border border-gray-200 bg-white">
        <div className="animate-pulse">
          <div className="border-b bg-gray-50 px-6 py-3">
            <div className="flex gap-4">
              {Array.from({ length: enableSelection ? 5 : 4 }).map((_, i) => (
                <div key={i} className="h-4 flex-1 rounded bg-gray-200" />
              ))}
            </div>
          </div>
          {Array.from({ length: 5 }).map((_, i) => (
            <div key={i} className="border-b px-6 py-4">
              <div className="flex gap-4">
                {Array.from({ length: enableSelection ? 5 : 4 }).map((_, j) => (
                  <div key={j} className="h-4 flex-1 rounded bg-gray-100" />
                ))}
              </div>
            </div>
          ))}
        </div>
      </div>
    );
  }

  if (data.length === 0) {
    return (
      <div className="overflow-hidden rounded-lg border border-gray-200 bg-white">
        <EmptyState title={resolvedEmptyTitle} description={emptyDescription} />
      </div>
    );
  }

  const startRow = (page - 1) * pageSize + 1;
  const endRow = Math.min(page * pageSize, total);

  return (
    <div className="overflow-hidden rounded-lg border border-gray-200 bg-white">
      <div className="overflow-x-auto">
        <table className="w-full text-left text-sm">
          <thead className="border-b bg-gray-50">
            {table.getHeaderGroups().map((headerGroup) => (
              <tr key={headerGroup.id}>
                {headerGroup.headers.map((header) => (
                  <th
                    key={header.id}
                    className={cn(
                      "px-4 py-3 text-xs font-semibold uppercase tracking-wider text-gray-500",
                      header.column.getCanSort() && "cursor-pointer select-none hover:text-gray-700"
                    )}
                    onClick={header.column.getToggleSortingHandler()}
                  >
                    <div className="flex items-center gap-1">
                      {header.isPlaceholder
                        ? null
                        : flexRender(header.column.columnDef.header, header.getContext())}
                      {header.column.getCanSort() && (
                        <ArrowUpDown className="h-3 w-3 text-gray-400" />
                      )}
                    </div>
                  </th>
                ))}
              </tr>
            ))}
          </thead>
          <tbody className="divide-y divide-gray-100">
            {table.getRowModel().rows.map((row) => (
              <tr
                key={row.id}
                className={cn(
                  "transition-colors hover:bg-gray-50",
                  onRowClick && "cursor-pointer"
                )}
                onClick={() => onRowClick?.(row.original)}
              >
                {row.getVisibleCells().map((cell) => (
                  <td key={cell.id} className="whitespace-nowrap px-4 py-3 text-gray-700">
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      {total > 0 && onPageChange && (
        <div className="flex items-center justify-between border-t bg-white px-4 py-3">
          <div className="flex items-center gap-4">
            <p className="text-sm text-gray-500">
              {t("showing", { start: startRow, end: endRow, total })}
            </p>
            {onPageSizeChange && (
              <PageSizeSelector pageSize={pageSize} onPageSizeChange={onPageSizeChange} />
            )}
          </div>
          <div className="flex items-center gap-2">
            <Button
              variant="secondary"
              size="sm"
              onClick={() => onPageChange(page - 1)}
              disabled={page <= 1}
            >
              <ChevronLeft className="h-4 w-4" />
              {t("previous")}
            </Button>
            <span className="text-sm text-gray-700">
              {t("pageOf", { page, totalPages })}
            </span>
            <Button
              variant="secondary"
              size="sm"
              onClick={() => onPageChange(page + 1)}
              disabled={page >= totalPages}
            >
              {t("next")}
              <ChevronRight className="h-4 w-4" />
            </Button>
          </div>
        </div>
      )}
    </div>
  );
}
