"use client";

import { memo } from "react";
import { type NodeProps } from "@xyflow/react";
import { Layers } from "lucide-react";

function SectionNodeComponent({ data }: NodeProps) {
  const label = data.label as string;
  const fieldCount = data.fieldCount as number;

  return (
    <div className="h-full w-full rounded-xl border-2 border-dashed border-gray-200 bg-slate-50/80">
      <div className="flex items-center gap-2 border-b border-dashed border-gray-200 px-4 py-2.5">
        <Layers className="h-4 w-4 text-gray-400" />
        <span className="text-sm font-semibold text-gray-700">{label}</span>
        <span className="ml-auto rounded-full bg-gray-200 px-2 py-0.5 text-xs text-gray-500">
          {fieldCount}
        </span>
      </div>
    </div>
  );
}

export const SectionNode = memo(SectionNodeComponent);
