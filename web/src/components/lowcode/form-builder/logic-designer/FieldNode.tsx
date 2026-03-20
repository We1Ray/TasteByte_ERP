"use client";

import { memo } from "react";
import { Handle, Position, type NodeProps } from "@xyflow/react";
import {
  Type, Hash, AlignLeft, ChevronDown, CheckSquare, Search, Upload,
  Calendar, Clock, ToggleLeft, Palette, DollarSign, Radio, FileText,
  ListTree, GitBranch, ThumbsUp, Table2, List,
} from "lucide-react";
import type { FieldDefinition } from "@/lib/types/lowcode";

const fieldTypeIcons: Record<string, React.ElementType> = {
  text: Type,
  number: Hash,
  textarea: AlignLeft,
  dropdown: ChevronDown,
  multi_select: List,
  checkbox: CheckSquare,
  lookup: Search,
  file: Upload,
  date: Calendar,
  datetime: Calendar,
  time_picker: Clock,
  toggle: ToggleLeft,
  color: Palette,
  currency: DollarSign,
  radio_group: Radio,
  rich_text: FileText,
  tree_table: ListTree,
  document_flow: GitBranch,
  approval_buttons: ThumbsUp,
  master_detail: Table2,
};

function FieldNodeComponent({ data, selected }: NodeProps) {
  const field = data.field as FieldDefinition;
  const Icon = fieldTypeIcons[field.field_type] || Type;

  const hasVisibilityRule = !!field.visibility_rule;
  const isRequired = field.validation?.required;
  const hasValidation =
    field.validation?.min_length ||
    field.validation?.max_length ||
    field.validation?.regex_pattern ||
    field.validation?.min_value ||
    field.validation?.max_value;

  return (
    <div
      className={`group relative rounded-lg border-2 bg-white px-3 py-2 shadow-sm transition-all ${
        selected
          ? "border-blue-500 shadow-md ring-2 ring-blue-200"
          : "border-gray-200 hover:border-blue-300 hover:shadow-md"
      }`}
      style={{ width: 248 }}
    >
      {/* Source handle (right side) */}
      <Handle
        type="source"
        position={Position.Right}
        className="!h-3 !w-3 !border-2 !border-indigo-400 !bg-white transition-all group-hover:!bg-indigo-400"
      />
      {/* Target handle (left side) */}
      <Handle
        type="target"
        position={Position.Left}
        className="!h-3 !w-3 !border-2 !border-indigo-400 !bg-white transition-all group-hover:!bg-indigo-400"
      />

      <div className="flex items-start gap-2">
        <div
          className={`mt-0.5 flex h-7 w-7 shrink-0 items-center justify-center rounded-md ${
            selected ? "bg-blue-100 text-blue-600" : "bg-gray-100 text-gray-500"
          }`}
        >
          <Icon className="h-3.5 w-3.5" />
        </div>
        <div className="min-w-0 flex-1">
          <p className="truncate text-sm font-medium text-gray-900">
            {field.label}
          </p>
          <div className="flex items-center gap-1.5">
            <span className="truncate text-xs font-mono text-gray-400">
              {field.field_key}
            </span>
            <span className="text-xs text-gray-300">&middot;</span>
            <span className="text-xs text-gray-400">{field.field_type}</span>
          </div>
          {/* Metadata indicators */}
          <div className="flex flex-wrap items-center gap-1 mt-0.5">
            {field.width && field.width !== "full" && (
              <span className="rounded bg-gray-100 px-1 text-[9px] font-medium text-gray-500">
                {field.width === "half" ? "50%" : field.width === "third" ? "33%" : "25%"}
              </span>
            )}
            {field.default_value && (
              <span className="rounded bg-sky-50 px-1 text-[9px] font-medium text-sky-600" title={`Default: ${field.default_value}`}>
                D:{field.default_value.length > 8 ? field.default_value.slice(0, 8) + "\u2026" : field.default_value}
              </span>
            )}
            {field.data_source?.type === "sql" && (
              <span className="rounded bg-emerald-50 px-1 text-[9px] font-medium text-emerald-600">SQL</span>
            )}
            {!!(field.field_config as Record<string, unknown>)?.operation_code && (
              <span className="rounded bg-purple-50 px-1 text-[9px] font-medium text-purple-600">
                {"\u2197"} {String((field.field_config as Record<string, unknown>).operation_code)}
              </span>
            )}
            {field.is_readonly && (
              <span className="rounded bg-gray-100 px-1 text-[9px] font-medium text-gray-500">RO</span>
            )}
          </div>
        </div>
      </div>

      {/* Indicator badges */}
      <div className="absolute -top-1.5 -right-1.5 flex gap-0.5">
        {isRequired && (
          <span
            className="flex h-4 w-4 items-center justify-center rounded-full bg-amber-500 text-[8px] font-bold text-white"
            title="Required"
          >
            !
          </span>
        )}
        {hasValidation && (
          <span
            className="flex h-4 w-4 items-center justify-center rounded-full bg-emerald-500 text-[8px] font-bold text-white"
            title="Has validation"
          >
            V
          </span>
        )}
        {hasVisibilityRule && (
          <span
            className="flex h-4 w-4 items-center justify-center rounded-full bg-indigo-500 text-[8px] font-bold text-white"
            title="Has visibility rule"
          >
            R
          </span>
        )}
      </div>
    </div>
  );
}

export const FieldNode = memo(FieldNodeComponent);
