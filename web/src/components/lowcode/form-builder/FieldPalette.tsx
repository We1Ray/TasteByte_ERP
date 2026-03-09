"use client";

import React from "react";
import {
  Type,
  Hash,
  AlignLeft,
  ToggleLeft,
  ChevronDown,
  List,
  Search,
  Layers,
  Upload,
  Calendar,
  Clock,
  ToggleRight,
  Palette,
  DollarSign,
  CircleDot,
  Timer,
  FileText,
  CheckSquare,
  Table2,
  GitBranch,
  GitPullRequest,
} from "lucide-react";
import { useDraggable } from "@dnd-kit/core";
import { useTranslations } from "next-intl";
import type { FieldType } from "@/lib/types/lowcode";

interface PaletteItem {
  type: FieldType;
  labelKey: string;
  icon: React.ElementType;
  categoryKey: string;
}

const paletteItems: PaletteItem[] = [
  { type: "text", labelKey: "text", icon: Type, categoryKey: "basic" },
  { type: "number", labelKey: "number", icon: Hash, categoryKey: "basic" },
  { type: "textarea", labelKey: "textArea", icon: AlignLeft, categoryKey: "basic" },
  { type: "checkbox", labelKey: "checkbox", icon: ToggleLeft, categoryKey: "basic" },
  { type: "dropdown", labelKey: "dropdown", icon: ChevronDown, categoryKey: "selection" },
  { type: "multi_select", labelKey: "multiSelect", icon: List, categoryKey: "selection" },
  { type: "lookup", labelKey: "lookup", icon: Search, categoryKey: "advanced" },
  { type: "composite", labelKey: "composite", icon: Layers, categoryKey: "advanced" },
  { type: "file", labelKey: "fileUpload", icon: Upload, categoryKey: "advanced" },
  { type: "date", labelKey: "date", icon: Calendar, categoryKey: "dateTime" },
  { type: "datetime", labelKey: "dateTimeField", icon: Clock, categoryKey: "dateTime" },
  { type: "time_picker", labelKey: "timePicker", icon: Timer, categoryKey: "dateTime" },
  { type: "toggle", labelKey: "toggle", icon: ToggleRight, categoryKey: "basic" },
  { type: "radio_group", labelKey: "radioGroup", icon: CircleDot, categoryKey: "selection" },
  { type: "currency", labelKey: "currency", icon: DollarSign, categoryKey: "advanced" },
  { type: "color", labelKey: "colorPicker", icon: Palette, categoryKey: "advanced" },
  { type: "rich_text", labelKey: "richText", icon: FileText, categoryKey: "advanced" },
  { type: "tree_table", labelKey: "treeTable", icon: GitBranch, categoryKey: "advanced" },
  { type: "document_flow", labelKey: "documentFlow", icon: GitPullRequest, categoryKey: "advanced" },
  { type: "approval_buttons", labelKey: "approval", icon: CheckSquare, categoryKey: "advanced" },
  { type: "master_detail", labelKey: "masterDetail", icon: Table2, categoryKey: "advanced" },
];

const categoryKeys = ["basic", "selection", "advanced", "dateTime"] as const;

const DraggableItem = React.memo(function DraggableItem({ item, label }: { item: PaletteItem; label: string }) {
  const { attributes, listeners, setNodeRef, isDragging } = useDraggable({
    id: `palette-${item.type}`,
    data: { type: "palette", fieldType: item.type },
  });

  const Icon = item.icon;

  return (
    <div
      ref={setNodeRef}
      {...listeners}
      {...attributes}
      className={`flex cursor-grab items-center gap-2 rounded-md border border-gray-200 bg-white px-3 py-2 text-sm transition-colors hover:border-blue-300 hover:bg-blue-50 ${isDragging ? "opacity-50" : ""}`}
    >
      <Icon className="h-4 w-4 text-gray-500" />
      <span className="text-gray-700">{label}</span>
    </div>
  );
});

export function FieldPalette() {
  const t = useTranslations("lowcode");

  return (
    <div className="flex h-full w-60 flex-col border-r border-gray-200 bg-gray-50">
      <div className="border-b border-gray-200 px-4 py-3">
        <h3 className="text-sm font-semibold text-gray-900">{t("fieldTypes")}</h3>
        <p className="text-xs text-gray-500">{t("dragFieldsHint")}</p>
      </div>

      <div className="flex-1 overflow-y-auto px-3 py-3">
        {categoryKeys.map((catKey) => (
          <div key={catKey} className="mb-4">
            <h4 className="mb-2 text-xs font-semibold uppercase tracking-wider text-gray-400">
              {t(catKey)}
            </h4>
            <div className="space-y-1.5">
              {paletteItems
                .filter((item) => item.categoryKey === catKey)
                .map((item) => (
                  <DraggableItem key={item.type} item={item} label={t(item.labelKey)} />
                ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
