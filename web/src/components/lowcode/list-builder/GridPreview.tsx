"use client";

import {
  useReactTable,
  getCoreRowModel,
  flexRender,
  type ColumnDef,
} from "@tanstack/react-table";
import React, { memo, useMemo, useState } from "react";
import { useTranslations } from "next-intl";
import {
  GripVertical,
  ArrowUpDown,
  Filter,
  Eye,
  EyeOff,
} from "lucide-react";
import {
  DndContext,
  DragOverlay,
  closestCenter,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
  type DragEndEvent,
  type DragStartEvent,
} from "@dnd-kit/core";
import {
  SortableContext,
  horizontalListSortingStrategy,
  sortableKeyboardCoordinates,
  useSortable,
} from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { useListBuilderStore } from "@/lib/stores/list-builder-store";
import { cn } from "@/lib/utils";
import type { ListColumn } from "@/lib/types/lowcode";

const MOCK_ROWS = 5;

function generateMockValue(dataType: string, index: number): string {
  switch (dataType) {
    case "NUMBER":
      return String((index + 1) * 100 + Math.floor(Math.random() * 50));
    case "DATE":
      return `2026-0${(index % 9) + 1}-${String((index % 28) + 1).padStart(2, "0")}`;
    case "BOOLEAN":
      return index % 2 === 0 ? "true" : "false";
    default:
      return `Sample ${index + 1}`;
  }
}

const SortableColumnHeader = memo(function SortableColumnHeader({
  column,
  isSelected,
  onSelect,
}: {
  column: ListColumn;
  isSelected: boolean;
  onSelect: () => void;
}) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id: column.id });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    width: column.width || 150,
    minWidth: column.min_width || 80,
  };

  return (
    <th
      ref={setNodeRef}
      style={style}
      className={cn(
        "relative cursor-pointer border-b border-r border-gray-200 bg-gray-50 px-3 py-2 text-left text-xs font-semibold text-gray-700 transition-colors",
        isSelected && "bg-blue-50 ring-2 ring-inset ring-blue-500",
        isDragging && "opacity-50",
        !column.is_visible && "opacity-40"
      )}
      onClick={onSelect}
    >
      <div className="flex items-center gap-1.5">
        <button
          className="cursor-grab text-gray-300 hover:text-gray-500"
          {...attributes}
          {...listeners}
        >
          <GripVertical className="h-3 w-3" />
        </button>
        <span className="truncate">{column.label}</span>
        <div className="ml-auto flex items-center gap-0.5">
          {column.is_sortable && (
            <ArrowUpDown className="h-3 w-3 text-gray-300" />
          )}
          {column.is_filterable && (
            <Filter className="h-3 w-3 text-gray-300" />
          )}
          {!column.is_visible && (
            <EyeOff className="h-3 w-3 text-gray-400" />
          )}
        </div>
      </div>
    </th>
  );
});

export function GridPreview() {
  const t = useTranslations("lowcode");
  const { columns, selectedColumnId, selectColumn, reorderColumns } =
    useListBuilderStore();

  const [activeDragId, setActiveDragId] = useState<string | null>(null);

  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: { distance: 8 },
    }),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  );

  const sortedColumns = useMemo(
    () => [...columns].sort((a, b) => a.sort_order - b.sort_order),
    [columns]
  );

  const tableColumns = useMemo<ColumnDef<Record<string, string>>[]>(
    () =>
      sortedColumns
        .filter((c) => c.is_visible)
        .map((col) => ({
          id: col.id,
          accessorKey: col.field_key,
          header: col.label,
          size: col.width || 150,
          minSize: col.min_width || 80,
        })),
    [sortedColumns]
  );

  const mockData = useMemo(() => {
    return Array.from({ length: MOCK_ROWS }, (_, rowIdx) => {
      const row: Record<string, string> = {};
      sortedColumns.forEach((col) => {
        row[col.field_key] = generateMockValue(col.data_type, rowIdx);
      });
      return row;
    });
  }, [sortedColumns]);

  const table = useReactTable({
    data: mockData,
    columns: tableColumns,
    getCoreRowModel: getCoreRowModel(),
  });

  const handleDragStart = (event: DragStartEvent) => {
    setActiveDragId(String(event.active.id));
  };

  const handleDragEnd = (event: DragEndEvent) => {
    setActiveDragId(null);
    const { active, over } = event;
    if (!over || active.id === over.id) return;

    const fromIndex = sortedColumns.findIndex((c) => c.id === active.id);
    const toIndex = sortedColumns.findIndex((c) => c.id === over.id);
    if (fromIndex !== -1 && toIndex !== -1) {
      reorderColumns(fromIndex, toIndex);
    }
  };

  if (sortedColumns.length === 0) {
    return (
      <div className="flex flex-1 items-center justify-center bg-gray-100">
        <div className="text-center">
          <Eye className="mx-auto h-12 w-12 text-gray-300" />
          <h3 className="mt-3 text-sm font-semibold text-gray-600">
            {t("noColumnsConfigured")}
          </h3>
          <p className="mt-1 text-xs text-gray-400">
            {t("addColumnsHint")}
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex-1 overflow-auto bg-gray-100 p-6">
      <div className="mx-auto max-w-5xl">
        <div className="mb-3 text-xs font-medium text-gray-500 uppercase tracking-wider">
          {t("gridPreview")}
        </div>
        <div className="overflow-hidden rounded-lg border border-gray-200 bg-white shadow-sm">
          <div className="overflow-x-auto">
            <DndContext
              sensors={sensors}
              collisionDetection={closestCenter}
              onDragStart={handleDragStart}
              onDragEnd={handleDragEnd}
            >
              <table className="min-w-full">
                <thead>
                  <tr>
                    <SortableContext
                      items={sortedColumns.map((c) => c.id)}
                      strategy={horizontalListSortingStrategy}
                    >
                      {sortedColumns.map((col) => (
                        <SortableColumnHeader
                          key={col.id}
                          column={col}
                          isSelected={selectedColumnId === col.id}
                          onSelect={() => selectColumn(col.id)}
                        />
                      ))}
                    </SortableContext>
                  </tr>
                </thead>
                <tbody>
                  {table.getRowModel().rows.map((row) => (
                    <tr
                      key={row.id}
                      className="border-b border-gray-100 last:border-b-0"
                    >
                      {row.getVisibleCells().map((cell) => (
                        <td
                          key={cell.id}
                          className="px-3 py-2 text-xs text-gray-500"
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
                  ))}
                </tbody>
              </table>
              <DragOverlay dropAnimation={null}>
                {activeDragId && (() => {
                  const col = sortedColumns.find((c) => c.id === activeDragId);
                  if (!col) return null;
                  return (
                    <div className="rounded border border-blue-300 bg-blue-50 px-3 py-2 shadow-lg">
                      <span className="text-xs font-semibold text-blue-700">
                        {col.label}
                      </span>
                    </div>
                  );
                })()}
              </DragOverlay>
            </DndContext>
          </div>
          <div className="flex items-center justify-between border-t bg-gray-50 px-4 py-2">
            <p className="text-xs text-gray-400">
              {t("mockRows", { count: MOCK_ROWS })}
            </p>
            <p className="text-xs text-gray-400">
              {t("columnsConfigured", { count: sortedColumns.length })}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
