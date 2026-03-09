"use client";

import { useMemo, useCallback } from "react";
import {
  DragDropContext,
  type DropResult,
} from "@hello-pangea/dnd";
import { toast } from "sonner";
import { useTranslations } from "next-intl";
import { useApiQuery, useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { executorApi } from "@/lib/api/lowcode";
import { PageLoading } from "@/components/ui/loading";
import { KanbanColumn } from "./KanbanColumn";
import type { KanbanViewConfig, FormRecord } from "@/lib/types/lowcode";
import type { PaginatedResponse } from "@/lib/api/client";

interface KanbanViewProps {
  operationCode: string;
  config: KanbanViewConfig;
  onCardClick?: (record: FormRecord) => void;
}

export function KanbanView({
  operationCode,
  config,
  onCardClick,
}: KanbanViewProps) {
  const t = useTranslations("lowcode");
  const invalidate = useInvalidateQueries();

  const { data, isLoading } = useApiQuery<PaginatedResponse<FormRecord>>(
    ["lowcode", "data", operationCode, "kanban"],
    () => executorApi.list(operationCode, { page_size: 1000 })
  );

  const updateMutation = useApiMutation(
    ({ id, statusValue }: { id: string; statusValue: string }) =>
      executorApi.update(operationCode, id, {
        [config.statusField]: statusValue,
      }),
    {
      onSuccess: () => {
        invalidate(["lowcode", "data", operationCode, "kanban"]);
        toast.success(t("cardMoved"));
      },
      onError: () => {
        toast.error(t("cardMoveFailed"));
        invalidate(["lowcode", "data", operationCode, "kanban"]);
      },
    }
  );

  const groupedRecords = useMemo(() => {
    const records = data?.items ?? [];
    const grouped: Record<string, FormRecord[]> = {};
    for (const col of config.columns) {
      grouped[col.value] = [];
    }
    for (const record of records) {
      const status = String(record.data[config.statusField] || "");
      if (grouped[status]) {
        grouped[status].push(record);
      }
    }
    return grouped;
  }, [data, config.columns, config.statusField]);

  const handleDragEnd = useCallback(
    (result: DropResult) => {
      const { draggableId, destination, source } = result;

      if (!destination) return;
      if (
        destination.droppableId === source.droppableId &&
        destination.index === source.index
      ) {
        return;
      }

      const targetColumnValue = destination.droppableId;

      if (source.droppableId !== destination.droppableId) {
        updateMutation.mutate({
          id: draggableId,
          statusValue: targetColumnValue,
        });
      }
    },
    [updateMutation]
  );

  if (isLoading) {
    return <PageLoading />;
  }

  if (config.columns.length === 0) {
    return (
      <div className="py-12 text-center text-sm text-gray-500">
        {t("kanbanNoColumns")}
      </div>
    );
  }

  return (
    <DragDropContext onDragEnd={handleDragEnd}>
      <div className="flex gap-4 overflow-x-auto pb-4">
        {config.columns.map((column) => (
          <KanbanColumn
            key={column.value}
            column={column}
            cards={groupedRecords[column.value] || []}
            titleField={config.titleField}
            descriptionField={config.descriptionField}
            droppableId={column.value}
            onCardClick={onCardClick}
          />
        ))}
      </div>
    </DragDropContext>
  );
}
