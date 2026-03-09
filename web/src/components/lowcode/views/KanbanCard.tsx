"use client";

import { Draggable } from "@hello-pangea/dnd";
import { cn } from "@/lib/utils";
import type { FormRecord } from "@/lib/types/lowcode";

interface KanbanCardProps {
  record: FormRecord;
  titleField: string;
  descriptionField?: string;
  index: number;
  onClick?: (record: FormRecord) => void;
}

export function KanbanCard({
  record,
  titleField,
  descriptionField,
  index,
  onClick,
}: KanbanCardProps) {
  const title = String(record.data[titleField] || "Untitled");
  const description = descriptionField
    ? String(record.data[descriptionField] || "")
    : "";

  return (
    <Draggable draggableId={record.id} index={index}>
      {(provided, snapshot) => (
        <div
          ref={provided.innerRef}
          {...provided.draggableProps}
          {...provided.dragHandleProps}
          className={cn(
            "rounded-md border border-gray-200 bg-white p-3 shadow-sm transition-shadow",
            "cursor-grab active:cursor-grabbing",
            snapshot.isDragging && "shadow-lg ring-2 ring-blue-200",
            "hover:shadow-md"
          )}
          onClick={() => onClick?.(record)}
        >
          <p className="text-sm font-medium text-gray-900 leading-snug">
            {title}
          </p>
          {description && (
            <p className="mt-1 text-xs text-gray-500 line-clamp-2">
              {description}
            </p>
          )}
        </div>
      )}
    </Draggable>
  );
}
