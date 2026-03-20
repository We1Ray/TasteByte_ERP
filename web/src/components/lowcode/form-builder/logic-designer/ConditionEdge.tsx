"use client";

import { memo } from "react";
import {
  BaseEdge,
  EdgeLabelRenderer,
  getSmoothStepPath,
  type EdgeProps,
} from "@xyflow/react";

function ConditionEdgeComponent({
  id,
  sourceX,
  sourceY,
  targetX,
  targetY,
  sourcePosition,
  targetPosition,
  data,
  markerEnd,
  style,
  selected,
}: EdgeProps) {
  const [edgePath, labelX, labelY] = getSmoothStepPath({
    sourceX,
    sourceY,
    targetX,
    targetY,
    sourcePosition,
    targetPosition,
    borderRadius: 12,
  });

  const operator = (data?.operator as string) || "equals";
  const value = (data?.value as string) || "";
  const action = (data?.action as string) || "show";
  const ruleType = (data?.ruleType as string) || "visibility";

  const operatorLabels: Record<string, string> = {
    equals: "=",
    not_equals: "\u2260",
    contains: "\u220B",
    gt: ">",
    lt: "<",
  };

  let label: string;
  if (ruleType === "datasource") {
    label = "\uD83D\uDCCA SQL ref";
  } else if (ruleType === "lookup") {
    label = `\u2197 ${value}`;
  } else {
    label = `${action === "show" ? "\u{1F441}" : "\u{1F6AB}"} ${operatorLabels[operator] || operator} "${value}"`;
  }

  const edgeColor = ruleType === "datasource" ? "#059669" : ruleType === "lookup" ? "#7c3aed" : "#6366f1";

  return (
    <>
      <BaseEdge
        id={id}
        path={edgePath}
        markerEnd={markerEnd}
        style={{
          ...style,
          strokeWidth: selected ? 3 : 2,
          stroke: edgeColor,
          strokeDasharray: ruleType === "datasource" ? "5 5" : undefined,
        }}
      />
      <EdgeLabelRenderer>
        <div
          className={`nodrag nopan absolute cursor-pointer rounded-md border px-2 py-1 text-xs font-medium shadow-sm transition-all ${
            selected
              ? "border-current bg-opacity-20"
              : "border-opacity-50 bg-white hover:bg-opacity-10"
          }`}
          style={{
            transform: `translate(-50%, -50%) translate(${labelX}px, ${labelY}px)`,
            pointerEvents: "all",
            color: edgeColor,
            borderColor: edgeColor,
          }}
        >
          {label}
        </div>
      </EdgeLabelRenderer>
    </>
  );
}

export const ConditionEdge = memo(ConditionEdgeComponent);
