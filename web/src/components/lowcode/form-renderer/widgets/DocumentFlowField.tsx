"use client";

import { useEffect, useState } from "react";
import { useParams } from "next/navigation";
import { useFormContext } from "react-hook-form";
import { ArrowRight, Loader2, FileText } from "lucide-react";
import { useTranslations } from "next-intl";
import { cn } from "@/lib/utils";
import { datasourceApi } from "@/lib/api/lowcode";
import type { FieldDefinition, DocumentFlowConfig, DocumentFlowNode } from "@/lib/types/lowcode";

interface DocumentFlowFieldProps {
  field: FieldDefinition;
  value: unknown;
  onChange: (value: unknown) => void;
  error?: string;
  disabled?: boolean;
}

const statusColors: Record<string, string> = {
  draft: "bg-gray-100 text-gray-700",
  open: "bg-blue-100 text-blue-700",
  active: "bg-blue-100 text-blue-700",
  pending: "bg-amber-100 text-amber-700",
  in_progress: "bg-amber-100 text-amber-700",
  approved: "bg-green-100 text-green-700",
  completed: "bg-green-100 text-green-700",
  released: "bg-green-100 text-green-700",
  cancelled: "bg-red-100 text-red-700",
  rejected: "bg-red-100 text-red-700",
  closed: "bg-gray-100 text-gray-600",
};

function getStatusColor(status?: string): string {
  if (!status) return "bg-gray-100 text-gray-500";
  return statusColors[status.toLowerCase()] ?? "bg-gray-100 text-gray-500";
}

function _flattenFlow(node: DocumentFlowNode): DocumentFlowNode[] {
  const result: DocumentFlowNode[] = [node];
  if (node.children && node.children.length > 0) {
    // For a linear flow, follow the first child chain
    for (const child of node.children) {
      result.push(..._flattenFlow(child));
    }
  }
  return result;
}

export function DocumentFlowField({
  field,
  value: _value,
  onChange,
  error,
  disabled: _disabled,
}: DocumentFlowFieldProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const config = (field.field_config ?? {}) as DocumentFlowConfig;
  const { document_type_field, document_id_field } = config;

  const params = useParams();
  const recordId = params?.id as string | undefined;
  const isCreateMode = !recordId || recordId === "new";

  // Try to read document type and id from form context
  let documentType: string | undefined;
  let documentId: string | undefined;
  try {
    const formContext = useFormContext();
    if (formContext) {
      if (document_type_field) {
        documentType = formContext.watch(document_type_field) as string | undefined;
      }
      if (document_id_field) {
        documentId = formContext.watch(document_id_field) as string | undefined;
      }
    }
  } catch {
    // Form context not available
  }

  const [loading, setLoading] = useState(false);
  const [flowNodes, setFlowNodes] = useState<DocumentFlowNode[]>([]);

  useEffect(() => {
    if (isCreateMode || !documentType || !documentId) {
      setFlowNodes([]);
      return;
    }

    setLoading(true);

    // Query the document flow using the datasource API
    // The flow data is typically fetched via a SQL query that returns document relationships
    const sql = `SELECT id, document_type, document_id, document_number, status, parent_id, created_at
                 FROM lc_document_flow
                 WHERE root_document_type = '${documentType}'
                   AND root_document_id = '${documentId}'
                 ORDER BY created_at ASC`;

    datasourceApi
      .query(sql)
      .then((result) => {
        if (result.rows.length > 0) {
          // Build tree from flat result
          const nodes = result.rows.map((row) => ({
            id: String(row.id ?? ""),
            document_type: String(row.document_type ?? ""),
            document_id: String(row.document_id ?? ""),
            document_number: row.document_number ? String(row.document_number) : undefined,
            status: row.status ? String(row.status) : undefined,
            created_at: row.created_at ? String(row.created_at) : undefined,
            children: [],
          }));
          setFlowNodes(nodes);
          onChange(nodes);
        } else {
          // If no flow table exists, create a single-node flow for the current document
          const singleNode: DocumentFlowNode = {
            id: documentId!,
            document_type: documentType!,
            document_id: documentId!,
            children: [],
          };
          setFlowNodes([singleNode]);
        }
      })
      .catch(() => {
        // If the query fails (e.g., table doesn't exist), show current document only
        const singleNode: DocumentFlowNode = {
          id: documentId!,
          document_type: documentType!,
          document_id: documentId!,
          children: [],
        };
        setFlowNodes([singleNode]);
      })
      .finally(() => setLoading(false));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [documentType, documentId, isCreateMode]);

  if (isCreateMode) {
    return (
      <div className="w-full">
        {field.label && (
          <label className="mb-1 block text-sm font-medium text-gray-700">
            {field.label}
          </label>
        )}
        <div className="flex items-center gap-2 rounded-md border border-dashed border-gray-300 px-4 py-6 text-center">
          <FileText className="mx-auto h-8 w-8 text-gray-300" />
        </div>
        <p className="mt-2 text-center text-sm text-gray-400">
          {t("noFormDefinition")}
        </p>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="w-full">
        {field.label && (
          <label className="mb-1 block text-sm font-medium text-gray-700">
            {field.label}
          </label>
        )}
        <div className="flex items-center justify-center rounded-md border border-gray-200 py-8">
          <Loader2 className="h-5 w-5 animate-spin text-gray-400" />
          <span className="ml-2 text-sm text-gray-500">{tCommon("loading")}</span>
        </div>
      </div>
    );
  }

  return (
    <div className="w-full">
      {field.label && (
        <label className="mb-1 block text-sm font-medium text-gray-700">
          {field.label}
        </label>
      )}

      {flowNodes.length === 0 ? (
        <div className="rounded-md border border-dashed border-gray-300 px-4 py-6 text-center">
          <p className="text-sm text-gray-400">{tCommon("noData")}</p>
        </div>
      ) : (
        <div className="overflow-x-auto rounded-md border border-gray-200 bg-gray-50 p-4">
          <div className="flex items-center gap-2">
            {flowNodes.map((node, index) => {
              const isCurrent = node.document_id === documentId;
              return (
                <div key={node.id || index} className="flex items-center gap-2">
                  {/* Arrow connector (not for first node) */}
                  {index > 0 && (
                    <ArrowRight className="h-5 w-5 flex-shrink-0 text-gray-400" />
                  )}

                  {/* Document node card */}
                  <div
                    className={cn(
                      "flex min-w-[140px] flex-col rounded-lg border p-3 shadow-sm transition-shadow",
                      isCurrent
                        ? "border-blue-400 bg-blue-50 ring-2 ring-blue-200"
                        : "border-gray-200 bg-white"
                    )}
                  >
                    <span className="text-xs font-semibold uppercase tracking-wider text-gray-500">
                      {node.document_type}
                    </span>
                    <span
                      className="mt-0.5 truncate text-sm font-medium text-gray-900"
                      title={node.document_number || node.document_id}
                    >
                      {node.document_number || node.document_id.slice(0, 8) + "..."}
                    </span>
                    {node.status && (
                      <span
                        className={cn(
                          "mt-1.5 inline-block self-start rounded-full px-2 py-0.5 text-xs font-medium",
                          getStatusColor(node.status)
                        )}
                      >
                        {node.status}
                      </span>
                    )}
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      )}

      {field.help_text && !error && (
        <p className="mt-1 text-sm text-gray-500">{field.help_text}</p>
      )}
      {error && <p className="mt-1 text-sm text-red-600">{error}</p>}
    </div>
  );
}
