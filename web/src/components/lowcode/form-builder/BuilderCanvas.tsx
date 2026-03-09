"use client";

import React, { useMemo, useState, useCallback } from "react";
import {
  DndContext,
  DragOverlay,
  closestCenter,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
  useDroppable,
  type DragEndEvent,
  type DragStartEvent,
} from "@dnd-kit/core";
import {
  SortableContext,
  verticalListSortingStrategy,
  sortableKeyboardCoordinates,
  useSortable,
} from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { GripVertical, Trash2, Plus, Settings } from "lucide-react";
import { useTranslations } from "next-intl";
import { Button } from "@/components/ui/button";
import { useBuilderStore } from "@/lib/stores/builder-store";
import { cn } from "@/lib/utils";
import type { FormSection, FieldDefinition, FieldType } from "@/lib/types/lowcode";

export function generateId() {
  return `${Date.now()}-${Math.random().toString(36).slice(2, 9)}`;
}

export function createDefaultField(type: FieldType, sectionId: string, order: number): FieldDefinition {
  return {
    id: generateId(),
    section_id: sectionId,
    field_key: `field_${Date.now()}`,
    field_type: type,
    label: `New ${type.charAt(0).toUpperCase() + type.slice(1)} Field`,
    validation: {},
    sort_order: order,
  };
}

// Maps field types to translation keys in the "lowcode" namespace
const fieldTypeLabelKeys: Record<string, string> = {
  text: "text",
  number: "number",
  dropdown: "dropdown",
  multi_select: "multiSelect",
  textarea: "textArea",
  checkbox: "checkbox",
  file: "fileUpload",
  lookup: "lookup",
  composite: "composite",
  date: "date",
  datetime: "dateTimeField",
  time_picker: "timePicker",
  toggle: "toggle",
  radio_group: "radioGroup",
  currency: "currency",
  color: "colorPicker",
  rich_text: "richText",
  tree_table: "treeTable",
  document_flow: "documentFlow",
  approval_buttons: "approval",
  master_detail: "masterDetail",
};

const SortableField = React.memo(function SortableField({
  field,
  sectionId,
}: {
  field: FieldDefinition;
  sectionId: string;
}) {
  const { selectField, deleteField, selectedFieldId } = useBuilderStore();
  const t = useTranslations("lowcode");
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({
    id: field.id,
    data: { type: "field", sectionId },
  });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
  };

  const fieldTypeLabel = fieldTypeLabelKeys[field.field_type]
    ? t(fieldTypeLabelKeys[field.field_type])
    : field.field_type;

  return (
    <div
      ref={setNodeRef}
      style={style}
      className={cn(
        "flex items-center gap-2 rounded-md border bg-white px-3 py-2 transition-colors",
        selectedFieldId === field.id ? "border-blue-500 ring-1 ring-blue-500" : "border-gray-200",
        isDragging && "opacity-50"
      )}
      onClick={() => selectField(field.id)}
    >
      <button
        className="cursor-grab text-gray-400 hover:text-gray-600"
        {...attributes}
        {...listeners}
      >
        <GripVertical className="h-4 w-4" />
      </button>
      <div className="flex-1 min-w-0">
        <p className="truncate text-sm font-medium text-gray-700">{field.label}</p>
        <p className="text-xs text-gray-400">{fieldTypeLabel} - {field.field_key}</p>
      </div>
      <button
        onClick={(e) => {
          e.stopPropagation();
          deleteField(field.id);
        }}
        className="text-gray-400 hover:text-red-500"
      >
        <Trash2 className="h-4 w-4" />
      </button>
    </div>
  );
});

const SortableSection = React.memo(function SortableSection({
  section,
}: {
  section: FormSection;
}) {
  const { selectSection, deleteSection, addField, selectedSectionId } = useBuilderStore();
  const t = useTranslations("lowcode");
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({
    id: section.id,
    data: { type: "section" },
  });

  // Make section a drop target for palette items
  const { setNodeRef: setDropRef, isOver } = useDroppable({
    id: `droppable-${section.id}`,
    data: { type: "section-drop", sectionId: section.id },
  });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
  };

  const sortedFields = useMemo(
    () => [...section.fields].sort((a, b) => a.sort_order - b.sort_order),
    [section.fields]
  );

  const handleAddField = (type: FieldType) => {
    addField(section.id, createDefaultField(type, section.id, section.fields.length));
  };

  return (
    <div
      ref={(node) => {
        setNodeRef(node);
        setDropRef(node);
      }}
      style={style}
      className={cn(
        "rounded-lg border bg-white p-4 transition-colors",
        selectedSectionId === section.id ? "border-blue-500 ring-1 ring-blue-500" : "border-gray-200",
        isDragging && "opacity-50",
        isOver && "border-blue-400 bg-blue-50/50 ring-1 ring-blue-300"
      )}
    >
      <div className="mb-3 flex items-center gap-2">
        <button
          className="cursor-grab text-gray-400 hover:text-gray-600"
          {...attributes}
          {...listeners}
        >
          <GripVertical className="h-4 w-4" />
        </button>
        <div
          className="flex-1 cursor-pointer"
          onClick={() => selectSection(section.id)}
        >
          <h4 className="text-sm font-semibold text-gray-900">{section.title}</h4>
          {section.description && (
            <p className="text-xs text-gray-500">{section.description}</p>
          )}
        </div>
        <button
          onClick={() => selectSection(section.id)}
          className="text-gray-400 hover:text-blue-500"
        >
          <Settings className="h-4 w-4" />
        </button>
        <button
          onClick={() => deleteSection(section.id)}
          className="text-gray-400 hover:text-red-500"
        >
          <Trash2 className="h-4 w-4" />
        </button>
      </div>

      <SortableContext
        items={sortedFields.map((f) => f.id)}
        strategy={verticalListSortingStrategy}
      >
        <div className="space-y-2">
          {sortedFields.map((field) => (
            <SortableField key={field.id} field={field} sectionId={section.id} />
          ))}
        </div>
      </SortableContext>

      {sortedFields.length === 0 && (
        <div className={cn(
          "rounded-md border-2 border-dashed px-4 py-6 text-center text-sm",
          isOver ? "border-blue-400 bg-blue-50 text-blue-500" : "border-gray-200 text-gray-400"
        )}>
          {isOver ? t("dropHere") : t("dragOrClick")}
        </div>
      )}

      <button
        onClick={() => handleAddField("text")}
        className="mt-2 flex w-full items-center justify-center gap-1 rounded-md border border-dashed border-gray-300 px-3 py-2 text-sm text-gray-500 hover:border-blue-400 hover:text-blue-600"
      >
        <Plus className="h-4 w-4" />
        {t("addField")}
      </button>
    </div>
  );
});

/** Hook providing shared DnD sensors, handlers, and overlay for the form builder.
 *  The DndContext must wrap both FieldPalette and BuilderCanvas in the parent. */
export function useBuilderDnd() {
  const { sections, addField, addSection, reorderSections, reorderFields, moveField, selectField } = useBuilderStore();

  const [activeDragItem, setActiveDragItem] = useState<{
    type: "field" | "section" | "palette";
    id: string;
    fieldType?: FieldType;
  } | null>(null);

  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: { distance: 8 },
    }),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  );

  const handleDragStart = useCallback((event: DragStartEvent) => {
    const data = event.active.data.current;
    if (data?.type === "palette") {
      setActiveDragItem({ type: "palette", id: String(event.active.id), fieldType: data.fieldType });
    } else if (data?.type === "section") {
      setActiveDragItem({ type: "section", id: String(event.active.id) });
    } else if (data?.type === "field") {
      setActiveDragItem({ type: "field", id: String(event.active.id) });
    }
  }, []);

  const handleDragEnd = useCallback((event: DragEndEvent) => {
    setActiveDragItem(null);

    const { active, over } = event;
    if (!over) return;

    const activeData = active.data.current;
    const overData = over.data.current;

    // Palette item dropped onto a section or field
    if (activeData?.type === "palette") {
      const fieldType = activeData.fieldType as FieldType;
      let targetSectionId: string | null = null;

      if (overData?.type === "section-drop") {
        targetSectionId = overData.sectionId as string;
      } else if (overData?.type === "section") {
        targetSectionId = String(over.id);
      } else if (overData?.type === "field") {
        targetSectionId = overData.sectionId as string;
      }

      if (targetSectionId) {
        const section = sections.find((s) => s.id === targetSectionId);
        if (section) {
          const newField = createDefaultField(fieldType, targetSectionId, section.fields.length);
          addField(targetSectionId, newField);
          selectField(newField.id);
        }
      }
      return;
    }

    if (active.id === over.id) return;

    if (activeData?.type === "section" && overData?.type === "section") {
      reorderSections(String(active.id), String(over.id));
    } else if (activeData?.type === "field" && overData?.type === "field") {
      const fromSection = activeData.sectionId as string;
      const toSection = overData.sectionId as string;
      if (fromSection === toSection) {
        reorderFields(fromSection, String(active.id), String(over.id));
      } else {
        const targetSection = sections.find((s) => s.id === toSection);
        const overIndex = targetSection?.fields.findIndex((f) => f.id === over.id) ?? 0;
        moveField(String(active.id), fromSection, toSection, overIndex);
      }
    }
  }, [sections, addField, selectField, reorderSections, reorderFields, moveField]);

  return { sensors, handleDragStart, handleDragEnd, activeDragItem, sections };
}

/** Drag overlay rendered during drag operations */
export function BuilderDragOverlay({ activeDragItem }: {
  activeDragItem: { type: string; id: string; fieldType?: FieldType } | null;
}) {
  const { sections } = useBuilderStore();
  const t = useTranslations("lowcode");

  return (
    <DragOverlay dropAnimation={null}>
      {activeDragItem?.type === "palette" && activeDragItem.fieldType && (
        <div className="rounded-md border border-green-300 bg-green-50 px-3 py-2 shadow-lg">
          <p className="text-sm font-medium text-green-700">
            + {fieldTypeLabelKeys[activeDragItem.fieldType] ? t(fieldTypeLabelKeys[activeDragItem.fieldType]) : activeDragItem.fieldType}
          </p>
        </div>
      )}
      {activeDragItem?.type === "field" && (() => {
        const field = sections.flatMap((s) => s.fields).find((f) => f.id === activeDragItem.id);
        if (!field) return null;
        return (
          <div className="rounded-md border border-blue-300 bg-blue-50 px-3 py-2 shadow-lg">
            <p className="text-sm font-medium text-blue-700">{field.label}</p>
            <p className="text-xs text-blue-500">{fieldTypeLabelKeys[field.field_type] ? t(fieldTypeLabelKeys[field.field_type]) : field.field_type}</p>
          </div>
        );
      })()}
      {activeDragItem?.type === "section" && (() => {
        const section = sections.find((s) => s.id === activeDragItem.id);
        if (!section) return null;
        return (
          <div className="rounded-lg border border-blue-300 bg-blue-50 p-3 shadow-lg">
            <p className="text-sm font-semibold text-blue-700">{section.title}</p>
            <p className="text-xs text-blue-500">{t("fieldsCount", { count: section.fields.length })}</p>
          </div>
        );
      })()}
    </DragOverlay>
  );
}

export function BuilderCanvas() {
  const { sections, addSection } = useBuilderStore();
  const t = useTranslations("lowcode");

  const sortedSections = useMemo(
    () => [...sections].sort((a, b) => a.sort_order - b.sort_order),
    [sections]
  );

  const handleAddSection = () => {
    addSection({
      id: generateId(),
      title: `Section ${sections.length + 1}`,
      columns: 2,
      collapsible: false,
      collapsed_default: false,
      sort_order: sections.length,
      fields: [],
    });
  };

  return (
    <div className="flex-1 overflow-y-auto bg-gray-100 p-6">
      <SortableContext
        items={sortedSections.map((s) => s.id)}
        strategy={verticalListSortingStrategy}
      >
        <div className="mx-auto max-w-3xl space-y-4">
          {sortedSections.map((section) => (
            <SortableSection key={section.id} section={section} />
          ))}

          <Button
            variant="secondary"
            className="w-full"
            onClick={handleAddSection}
          >
            <Plus className="h-4 w-4" />
            {t("addSection")}
          </Button>
        </div>
      </SortableContext>
    </div>
  );
}
