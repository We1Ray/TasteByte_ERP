"use client";

import { useState, useEffect, useMemo, useCallback } from "react";
import {
  useReactTable,
  getCoreRowModel,
  getExpandedRowModel,
  flexRender,
  type ColumnDef,
  type ExpandedState,
} from "@tanstack/react-table";
import { ChevronRight, ChevronDown, Loader2 } from "lucide-react";
import { useTranslations } from "next-intl";
import { cn } from "@/lib/utils";
import { datasourceApi } from "@/lib/api/lowcode";
import type { FieldDefinition, TreeTableFieldConfig } from "@/lib/types/lowcode";

interface TreeNode extends Record<string, unknown> {
  _subRows?: TreeNode[];
}

interface TreeTableFieldProps {
  field: FieldDefinition;
  value: unknown[];
  onChange: (value: unknown[]) => void;
  error?: string;
  disabled?: boolean;
}

function buildTree(
  flatData: Record<string, unknown>[],
  idField: string,
  parentField: string
): TreeNode[] {
  const map = new Map<unknown, TreeNode>();
  const roots: TreeNode[] = [];

  // First pass: index all nodes
  for (const row of flatData) {
    const node: TreeNode = { ...row, _subRows: [] };
    map.set(row[idField], node);
  }

  // Second pass: build parent-child relationships
  for (const row of flatData) {
    const node = map.get(row[idField])!;
    const parentId = row[parentField];
    if (parentId && map.has(parentId)) {
      map.get(parentId)!._subRows!.push(node);
    } else {
      roots.push(node);
    }
  }

  // Remove empty _subRows arrays for leaf nodes
  for (const node of map.values()) {
    if (node._subRows && node._subRows.length === 0) {
      delete node._subRows;
    }
  }

  return roots;
}

function getDefaultExpanded(treeData: TreeNode[], expandLevel: number, currentLevel = 0): Record<string, boolean> {
  const result: Record<string, boolean> = {};
  if (currentLevel >= expandLevel) return result;
  treeData.forEach((node, index) => {
    if (node._subRows && node._subRows.length > 0) {
      result[String(index)] = true;
      const childExpanded = getDefaultExpanded(node._subRows, expandLevel, currentLevel + 1);
      for (const [key, val] of Object.entries(childExpanded)) {
        result[`${index}.${key}`] = val;
      }
    }
  });
  return result;
}

export function TreeTableField({ field, value, onChange, error, disabled: _disabled }: TreeTableFieldProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const config = (field.field_config ?? {}) as TreeTableFieldConfig;
  const {
    parent_field = "parent_id",
    id_field = "id",
    expand_level = 1,
    columns: columnConfig = [],
    data_source,
  } = config;

  const [loading, setLoading] = useState(false);
  const [fetchedData, setFetchedData] = useState<Record<string, unknown>[] | null>(null);
  const [expanded, setExpanded] = useState<ExpandedState>({});

  // Fetch data from SQL data source if configured
  useEffect(() => {
    if (data_source?.type === "sql" && data_source.sql_query) {
      setLoading(true);
      datasourceApi
        .query(data_source.sql_query)
        .then((result) => {
          setFetchedData(result.rows);
          onChange(result.rows as unknown[]);
        })
        .catch(() => {
          setFetchedData([]);
        })
        .finally(() => setLoading(false));
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [data_source?.sql_query]);

  const rawData = useMemo(() => {
    const source = fetchedData ?? (value as Record<string, unknown>[]) ?? [];
    return source;
  }, [fetchedData, value]);

  const treeData = useMemo(() => {
    return buildTree(rawData, id_field, parent_field);
  }, [rawData, id_field, parent_field]);

  // Set default expanded state based on expand_level
  useEffect(() => {
    if (treeData.length > 0) {
      const defaultExpanded = getDefaultExpanded(treeData, expand_level);
      setExpanded(defaultExpanded);
    }
  }, [treeData, expand_level]);

  const columns: ColumnDef<TreeNode>[] = useMemo(() => {
    if (columnConfig.length === 0) {
      // Auto-detect columns from data
      const keys = rawData.length > 0 ? Object.keys(rawData[0]).filter((k) => k !== parent_field && k !== "_subRows") : [];
      return keys.map((key, i) => ({
        accessorKey: key,
        header: key,
        size: 150,
        cell: ({ row, getValue }) => {
          if (i === 0) {
            return (
              <div className="flex items-center" style={{ paddingLeft: `${row.depth * 20}px` }}>
                {row.getCanExpand() ? (
                  <button
                    type="button"
                    onClick={(e) => {
                      e.stopPropagation();
                      row.toggleExpanded();
                    }}
                    className="mr-1 rounded p-0.5 text-gray-500 hover:bg-gray-100"
                  >
                    {row.getIsExpanded() ? (
                      <ChevronDown className="h-4 w-4" />
                    ) : (
                      <ChevronRight className="h-4 w-4" />
                    )}
                  </button>
                ) : (
                  <span className="mr-1 inline-block w-5" />
                )}
                <span className="truncate">{String(getValue() ?? "")}</span>
              </div>
            );
          }
          return <span className="truncate">{String(getValue() ?? "")}</span>;
        },
      }));
    }

    return columnConfig.map((col, i) => ({
      accessorKey: col.field_key,
      header: col.label,
      size: col.width ?? 150,
      cell: ({ row, getValue }) => {
        if (i === 0) {
          return (
            <div className="flex items-center" style={{ paddingLeft: `${row.depth * 20}px` }}>
              {row.getCanExpand() ? (
                <button
                  type="button"
                  onClick={(e) => {
                    e.stopPropagation();
                    row.toggleExpanded();
                  }}
                  className="mr-1 rounded p-0.5 text-gray-500 hover:bg-gray-100"
                >
                  {row.getIsExpanded() ? (
                    <ChevronDown className="h-4 w-4" />
                  ) : (
                    <ChevronRight className="h-4 w-4" />
                  )}
                </button>
              ) : (
                <span className="mr-1 inline-block w-5" />
              )}
              <span className="truncate">{String(getValue() ?? "")}</span>
            </div>
          );
        }
        return <span className="truncate">{String(getValue() ?? "")}</span>;
      },
    }));
  }, [columnConfig, rawData, parent_field]);

  const getSubRows = useCallback((row: TreeNode) => row._subRows, []);

  const table = useReactTable({
    data: treeData,
    columns,
    state: { expanded },
    onExpandedChange: setExpanded,
    getSubRows,
    getCoreRowModel: getCoreRowModel(),
    getExpandedRowModel: getExpandedRowModel(),
  });

  if (loading) {
    return (
      <div className="w-full">
        {field.label && (
          <label className="mb-1 block text-sm font-medium text-gray-700">
            {field.label}
            {field.validation.required && <span className="ml-0.5 text-red-500">*</span>}
          </label>
        )}
        <div className="flex items-center justify-center rounded-md border border-gray-200 py-8">
          <Loader2 className="h-5 w-5 animate-spin text-gray-400" />
          <span className="ml-2 text-sm text-gray-500">{t("loadingData")}</span>
        </div>
      </div>
    );
  }

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
                    className="px-3 py-2 text-left text-xs font-semibold uppercase tracking-wider text-gray-500"
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
                  className="px-3 py-6 text-center text-sm text-gray-500"
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
                    <td key={cell.id} className="px-3 py-2 text-gray-700">
                      {flexRender(cell.column.columnDef.cell, cell.getContext())}
                    </td>
                  ))}
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {field.help_text && !error && (
        <p className="mt-1 text-sm text-gray-500">{field.help_text}</p>
      )}
      {error && <p className="mt-1 text-sm text-red-600">{error}</p>}
    </div>
  );
}
