"use client";

import { useTranslations } from "next-intl";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import apiClient from "@/lib/api/client";
import { Clock, ArrowRight } from "lucide-react";

interface StatusHistoryEntry {
  id: string;
  document_type: string;
  document_id: string;
  from_status: string | null;
  to_status: string;
  changed_by: string;
  change_reason: string | null;
  created_at: string;
}

function formatDate(dateStr: string) {
  return new Date(dateStr).toLocaleString("en-US", {
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}

function statusColor(status: string): string {
  const s = status.toUpperCase();
  if (s.includes("CANCEL")) return "bg-red-100 text-red-700";
  if (
    s.includes("COMPLETE") ||
    s.includes("DELIVER") ||
    s.includes("PAID") ||
    s.includes("POSTED")
  )
    return "bg-green-100 text-green-700";
  if (
    s.includes("PROGRESS") ||
    s.includes("SHIP") ||
    s.includes("RELEASED")
  )
    return "bg-blue-100 text-blue-700";
  if (s.includes("DRAFT") || s.includes("CREATED") || s.includes("OPEN"))
    return "bg-gray-100 text-gray-700";
  return "bg-yellow-100 text-yellow-700";
}

export function WorkflowTimeline({
  documentType,
  documentId,
}: {
  documentType: string;
  documentId: string;
}) {
  const t = useTranslations("shared");
  const { data, isLoading } = useApiQuery<StatusHistoryEntry[]>(
    ["workflow", "history", documentType, documentId],
    () =>
      apiClient
        .get(`/workflow/history/${documentType}/${documentId}`)
        .then((res) => res.data ?? [])
  );

  if (isLoading) {
    return (
      <div className="flex items-center gap-2 text-sm text-gray-400 py-4">
        <Clock className="h-4 w-4 animate-pulse" />
        {t("processing")}
      </div>
    );
  }

  const entries = data ?? [];

  if (entries.length === 0) {
    return (
      <p className="text-sm text-gray-400 py-2">{t("noStatusHistory")}</p>
    );
  }

  return (
    <div className="space-y-0">
      <h4 className="text-sm font-medium text-gray-700 mb-3">
        Status History
      </h4>
      <div className="relative pl-6">
        {/* vertical line */}
        <div className="absolute left-2 top-1 bottom-1 w-px bg-gray-200" />

        {entries.map((entry, idx) => (
          <div key={entry.id} className="relative pb-4 last:pb-0">
            {/* dot */}
            <div
              className={`absolute -left-4 top-1 h-3 w-3 rounded-full border-2 border-white ${
                idx === entries.length - 1 ? "bg-blue-500" : "bg-gray-300"
              }`}
            />

            <div className="flex flex-col gap-0.5">
              <div className="flex items-center gap-2 flex-wrap">
                {entry.from_status && (
                  <>
                    <span
                      className={`inline-block text-xs font-medium px-2 py-0.5 rounded ${statusColor(entry.from_status)}`}
                    >
                      {entry.from_status}
                    </span>
                    <ArrowRight className="h-3 w-3 text-gray-400" />
                  </>
                )}
                <span
                  className={`inline-block text-xs font-medium px-2 py-0.5 rounded ${statusColor(entry.to_status)}`}
                >
                  {entry.to_status}
                </span>
              </div>
              <span className="text-xs text-gray-400">
                {formatDate(entry.created_at)}
              </span>
              {entry.change_reason && (
                <span className="text-xs text-gray-500 italic">
                  {entry.change_reason}
                </span>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
