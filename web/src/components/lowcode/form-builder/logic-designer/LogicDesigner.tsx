"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import {
  ReactFlow,
  Background,
  Controls,
  MiniMap,
  Panel,
  useNodesState,
  useEdgesState,
  MarkerType,
  type Connection,
  type Edge,
  type Node,
  type NodeTypes,
  type EdgeTypes,
  type OnConnect,
  type NodeMouseHandler,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import { useTranslations } from "next-intl";
import { useBuilderStore } from "@/lib/stores/builder-store";
import { FieldNode } from "./FieldNode";
import { SectionNode } from "./SectionNode";
import { ConditionEdge } from "./ConditionEdge";
import { ConnectionModal } from "./ConnectionModal";
import type { FormSection } from "@/lib/types/lowcode";

const nodeTypes: NodeTypes = {
  field: FieldNode,
  section: SectionNode,
};

const edgeTypes: EdgeTypes = {
  condition: ConditionEdge,
};

interface LogicDesignerProps {
  onFieldSelect: (fieldId: string) => void;
}

// Convert builder store sections into React Flow nodes and edges
function buildNodesAndEdges(sections: FormSection[]) {
  const nodes: Node[] = [];
  const edges: Edge[] = [];

  const SECTION_WIDTH = 280;
  const FIELD_HEIGHT = 64;
  const SECTION_HEADER = 48;
  const SECTION_GAP = 40;
  const FIELD_GAP = 8;

  let sectionX = 0;

  for (const section of sections) {
    const sectionHeight =
      SECTION_HEADER + section.fields.length * (FIELD_HEIGHT + FIELD_GAP) + 20;

    // Section node (group)
    nodes.push({
      id: `section-${section.id}`,
      type: "section",
      position: { x: sectionX, y: 0 },
      data: {
        label: section.title,
        fieldCount: section.fields.length,
        sectionId: section.id,
      },
      style: {
        width: SECTION_WIDTH,
        height: sectionHeight,
      },
    });

    // Field nodes within section
    section.fields.forEach((field, fieldIndex) => {
      const fieldY = SECTION_HEADER + fieldIndex * (FIELD_HEIGHT + FIELD_GAP);

      nodes.push({
        id: `field-${field.id}`,
        type: "field",
        position: { x: 16, y: fieldY },
        parentId: `section-${section.id}`,
        extent: "parent" as const,
        data: {
          field,
          sectionId: section.id,
        },
      });

      // Create edges for visibility rules
      if (field.visibility_rule) {
        const rule = field.visibility_rule as {
          dependent_field?: string;
          operator?: string;
          value?: string;
          action?: string;
        };
        if (rule.dependent_field) {
          const sourceField = sections
            .flatMap((s) => s.fields)
            .find((f) => f.field_key === rule.dependent_field);

          if (sourceField) {
            edges.push({
              id: `vis-${sourceField.id}-${field.id}`,
              source: `field-${sourceField.id}`,
              target: `field-${field.id}`,
              type: "condition",
              animated: true,
              data: {
                ruleType: "visibility",
                operator: rule.operator || "equals",
                value: rule.value || "",
                action: rule.action || "show",
              },
              markerEnd: {
                type: MarkerType.ArrowClosed,
                color: "#6366f1",
              },
              style: { stroke: "#6366f1", strokeWidth: 2 },
            });
          }
        }
      }

      // Create edges for lookup references
      if (field.field_config) {
        const config = field.field_config as Record<string, unknown>;
        if (config.operation_code) {
          // Lookup references are shown as dashed edges to indicate external dependency
          edges.push({
            id: `lookup-${field.id}`,
            source: `field-${field.id}`,
            target: `field-${field.id}`, // self-referencing (indicates external dep)
            type: "condition",
            animated: false,
            data: {
              ruleType: "lookup",
              operator: "lookup",
              value: String(config.operation_code),
              action: "ref",
            },
          });
        }
      }

      // Create edges for data source SQL dependencies
      if (field.data_source?.type === "sql" && field.data_source?.sql_query) {
        // Check if SQL references other fields by field_key
        const sqlQuery = field.data_source.sql_query;
        const allFields = sections.flatMap(s => s.fields);
        for (const otherField of allFields) {
          if (otherField.id !== field.id && sqlQuery.includes(otherField.field_key)) {
            edges.push({
              id: `sql-${otherField.id}-${field.id}`,
              source: `field-${otherField.id}`,
              target: `field-${field.id}`,
              type: "condition",
              animated: false,
              data: {
                ruleType: "datasource",
                operator: "sql_ref",
                value: "",
                action: "ref",
              },
              markerEnd: {
                type: MarkerType.ArrowClosed,
                color: "#059669",
              },
              style: { stroke: "#059669", strokeWidth: 2, strokeDasharray: "5 5" },
            });
          }
        }
      }
    });

    sectionX += SECTION_WIDTH + SECTION_GAP;
  }

  return { nodes, edges };
}

export function LogicDesigner({ onFieldSelect }: LogicDesignerProps) {
  const t = useTranslations("lowcode");
  const { sections, updateField } = useBuilderStore();
  const [connectionModal, setConnectionModal] = useState<{
    sourceFieldId: string;
    targetFieldId: string;
  } | null>(null);

  const { nodes: initialNodes, edges: initialEdges } = useMemo(
    () => buildNodesAndEdges(sections),
    [sections]
  );

  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

  // Sync when sections change
  useEffect(() => {
    const { nodes: newNodes, edges: newEdges } = buildNodesAndEdges(sections);
    setNodes(newNodes);
    setEdges(newEdges);
  }, [sections, setNodes, setEdges]);

  const onConnect: OnConnect = useCallback(
    (connection: Connection) => {
      if (
        !connection.source?.startsWith("field-") ||
        !connection.target?.startsWith("field-")
      ) {
        return;
      }
      const sourceFieldId = connection.source.replace("field-", "");
      const targetFieldId = connection.target.replace("field-", "");

      setConnectionModal({ sourceFieldId, targetFieldId });
    },
    []
  );

  const handleConnectionSave = useCallback(
    (rule: { operator: string; value: string; action: string }) => {
      if (!connectionModal) return;

      const { sourceFieldId, targetFieldId } = connectionModal;

      const sourceField = sections
        .flatMap((s) => s.fields)
        .find((f) => f.id === sourceFieldId);

      if (!sourceField) return;

      updateField(targetFieldId, {
        visibility_rule: {
          dependent_field: sourceField.field_key,
          operator: rule.operator,
          value: rule.value,
          action: rule.action,
        },
      });

      setConnectionModal(null);
    },
    [connectionModal, sections, updateField]
  );

  const onNodeClick: NodeMouseHandler = useCallback(
    (_event, node) => {
      if (node.id.startsWith("field-")) {
        const fieldId = node.id.replace("field-", "");
        onFieldSelect(fieldId);
      }
    },
    [onFieldSelect]
  );

  const onEdgeClick = useCallback(
    (_event: React.MouseEvent, edge: Edge) => {
      if (
        edge.source.startsWith("field-") &&
        edge.target.startsWith("field-")
      ) {
        const sourceFieldId = edge.source.replace("field-", "");
        const targetFieldId = edge.target.replace("field-", "");
        setConnectionModal({ sourceFieldId, targetFieldId });
      }
    },
    []
  );

  const onEdgesDelete = useCallback(
    (deletedEdges: Edge[]) => {
      for (const edge of deletedEdges) {
        if (edge.target.startsWith("field-")) {
          const targetFieldId = edge.target.replace("field-", "");
          updateField(targetFieldId, {
            visibility_rule: null as unknown as undefined,
          });
        }
      }
    },
    [updateField]
  );

  return (
    <div className="h-full w-full">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        onNodeClick={onNodeClick}
        onEdgeClick={onEdgeClick}
        onEdgesDelete={onEdgesDelete}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        fitView
        snapToGrid
        snapGrid={[16, 16]}
        minZoom={0.3}
        maxZoom={2}
        defaultEdgeOptions={{
          type: "condition",
          animated: true,
          markerEnd: { type: MarkerType.ArrowClosed, color: "#6366f1" },
          style: { stroke: "#6366f1", strokeWidth: 2 },
        }}
      >
        <Background gap={16} size={1} color="#e5e7eb" />
        <Controls />
        <MiniMap
          nodeStrokeColor="#6366f1"
          nodeColor={(node) =>
            node.type === "section" ? "#f1f5f9" : "#ffffff"
          }
          style={{ border: "1px solid #e5e7eb" }}
        />
        <Panel position="top-left">
          <div className="rounded-md bg-white/90 px-3 py-2 shadow-sm backdrop-blur text-xs text-gray-500">
            <p className="font-medium text-gray-700">{t("logicDesigner")}</p>
            <p>{t("logicDesignerHint")}</p>
          </div>
        </Panel>
        <Panel position="top-right">
          <div className="flex flex-wrap gap-2">
            <div className="flex items-center gap-1.5 rounded-md bg-white/90 px-2 py-1 text-xs shadow-sm backdrop-blur">
              <div className="h-2 w-2 rounded-full bg-indigo-500" />
              <span className="text-gray-600">{t("visibilityRule")}</span>
            </div>
            <div className="flex items-center gap-1.5 rounded-md bg-white/90 px-2 py-1 text-xs shadow-sm backdrop-blur">
              <div className="h-2 w-2 rounded-full bg-emerald-500" />
              <span className="text-gray-600">{t("dataDependency")}</span>
            </div>
            <div className="flex items-center gap-1.5 rounded-md bg-white/90 px-2 py-1 text-xs shadow-sm backdrop-blur">
              <div className="h-2 w-2 rounded-full bg-amber-500" />
              <span className="text-gray-600">{t("required")}</span>
            </div>
          </div>
        </Panel>
      </ReactFlow>

      {connectionModal && (
        <ConnectionModal
          open={true}
          onClose={() => setConnectionModal(null)}
          onSave={handleConnectionSave}
          sourceField={sections
            .flatMap((s) => s.fields)
            .find((f) => f.id === connectionModal.sourceFieldId)}
          targetField={sections
            .flatMap((s) => s.fields)
            .find((f) => f.id === connectionModal.targetFieldId)}
          existingRule={(() => {
            const target = sections
              .flatMap((s) => s.fields)
              .find((f) => f.id === connectionModal.targetFieldId);
            if (!target?.visibility_rule) return undefined;
            const rule = target.visibility_rule as {
              operator?: string;
              value?: string;
              action?: string;
            };
            return {
              operator: rule.operator || "equals",
              value: rule.value || "",
              action: rule.action || "show",
            };
          })()}
        />
      )}
    </div>
  );
}
