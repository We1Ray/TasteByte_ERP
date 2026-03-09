"use client";

import { Droppable } from "@hello-pangea/dnd";
import { cn } from "@/lib/utils";
import { KanbanCard } from "./KanbanCard";
import type { FormRecord } from "@/lib/types/lowcode";

interface KanbanColumnProps {
  column: { value: string; label: string; color: string };
  cards: FormRecord[];
  titleField: string;
  descriptionField?: string;
  droppableId: string;
  onCardClick?: (record: FormRecord) => void;
}

export function KanbanColumn({
  column,
  cards,
  titleField,
  descriptionField,
  droppableId,
  onCardClick,
}: KanbanColumnProps) {
  return (
    <div className="flex w-72 shrink-0 flex-col rounded-lg bg-gray-50 border border-gray-200">
      {/* Column header */}
      <div className="flex items-center gap-2 px-3 py-2.5 border-b border-gray-200">
        <div
          className="h-3 w-3 rounded-full shrink-0"
          style={{ backgroundColor: column.color || "#6B7280" }}
        />
        <span className="text-sm font-semibold text-gray-700 truncate">
          {column.label}
        </span>
        <span
          className={cn(
            "ml-auto inline-flex h-5 min-w-[20px] items-center justify-center rounded-full px-1.5",
            "bg-gray-200 text-xs font-medium text-gray-600"
          )}
        >
          {cards.length}
        </span>
      </div>

      {/* Card list */}
      <Droppable droppableId={droppableId}>
        {(provided, snapshot) => (
          <div
            ref={provided.innerRef}
            {...provided.droppableProps}
            className={cn(
              "flex-1 space-y-2 overflow-y-auto p-2 transition-colors",
              "min-h-[120px]",
              snapshot.isDraggingOver && "bg-blue-50"
            )}
          >
            {cards.map((card, index) => (
              <KanbanCard
                key={card.id}
                record={card}
                titleField={titleField}
                descriptionField={descriptionField}
                index={index}
                onClick={onCardClick}
              />
            ))}
            {provided.placeholder}
          </div>
        )}
      </Droppable>
    </div>
  );
}
