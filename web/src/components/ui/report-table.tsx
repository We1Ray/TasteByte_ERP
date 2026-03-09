"use client";

import { cn } from "@/lib/utils";

interface ReportTableColumn<T> {
  header: string;
  accessor: keyof T | ((item: T, index: number) => React.ReactNode);
  align?: "left" | "right" | "center";
  className?: string;
}

interface FooterCell {
  label: string;
  value: React.ReactNode;
  colSpan: number;
}

interface ReportTableProps<T> {
  columns: ReportTableColumn<T>[];
  data: T[] | undefined;
  keyExtractor: (item: T, index: number) => string | number;
  footer?: FooterCell[];
  emptyMessage?: string;
  isLoading?: boolean;
}

function getCellValue<T>(item: T, accessor: ReportTableColumn<T>["accessor"], index: number): React.ReactNode {
  if (typeof accessor === "function") {
    return accessor(item, index);
  }
  const val = item[accessor];
  if (val === null || val === undefined) return "-";
  return String(val);
}

const alignClass = {
  left: "text-left",
  right: "text-right",
  center: "text-center",
} as const;

export function ReportTable<T>({
  columns,
  data,
  keyExtractor,
  footer,
  emptyMessage = "No data available",
  isLoading,
}: ReportTableProps<T>) {
  if (isLoading) {
    return (
      <div className="animate-pulse space-y-3">
        {Array.from({ length: 8 }).map((_, i) => (
          <div key={i} className="flex gap-4">
            <div className="h-4 flex-1 rounded bg-gray-200" />
            <div className="h-4 w-24 rounded bg-gray-200" />
            <div className="h-4 w-24 rounded bg-gray-200" />
          </div>
        ))}
      </div>
    );
  }

  return (
    <div className="-mx-6 -mb-6">
      <table className="w-full text-sm">
        <thead className="border-b border-t bg-gray-50">
          <tr>
            {columns.map((col, i) => (
              <th
                key={i}
                className={cn(
                  "px-6 py-3 text-xs font-semibold uppercase text-gray-500",
                  alignClass[col.align ?? "left"],
                  col.className
                )}
              >
                {col.header}
              </th>
            ))}
          </tr>
        </thead>
        <tbody className="divide-y">
          {data && data.length > 0 ? (
            data.map((item, rowIndex) => (
              <tr key={keyExtractor(item, rowIndex)} className="hover:bg-gray-50">
                {columns.map((col, i) => (
                  <td
                    key={i}
                    className={cn(
                      "px-6 py-3 text-gray-700",
                      alignClass[col.align ?? "left"],
                      col.className
                    )}
                  >
                    {getCellValue(item, col.accessor, rowIndex)}
                  </td>
                ))}
              </tr>
            ))
          ) : (
            <tr>
              <td
                colSpan={columns.length}
                className="px-6 py-8 text-center text-gray-500"
              >
                {emptyMessage}
              </td>
            </tr>
          )}
        </tbody>
        {footer && data && data.length > 0 && (
          <tfoot className="border-t-2 border-gray-300 bg-gray-50 font-semibold">
            <tr>
              {footer.map((cell, i) => (
                <td
                  key={i}
                  colSpan={cell.colSpan}
                  className={cn(
                    "px-6 py-3",
                    i === 0 ? "text-gray-900" : "text-right font-mono text-gray-900"
                  )}
                >
                  {i === 0 ? cell.label : cell.value}
                </td>
              ))}
            </tr>
          </tfoot>
        )}
      </table>
    </div>
  );
}
