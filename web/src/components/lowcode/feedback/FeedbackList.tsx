"use client";

import { useState } from "react";
import { MessageSquare, AlertCircle, Lightbulb, ArrowUpCircle } from "lucide-react";
import { useTranslations } from "next-intl";
import { Badge, StatusBadge } from "@/components/ui/badge";
import { SearchBar } from "@/components/forms/search-bar";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { usePagination } from "@/lib/hooks/use-pagination";
import { feedbackApi } from "@/lib/api/lowcode";
import { formatDateTime } from "@/lib/utils";
import type { Feedback } from "@/lib/types/lowcode";

interface FeedbackListProps {
  projectId?: string;
  onSelect?: (feedback: Feedback) => void;
}

const typeIcons: Record<string, React.ElementType> = {
  bug: AlertCircle,
  feature: Lightbulb,
  improvement: ArrowUpCircle,
};

const priorityColors: Record<string, string> = {
  low: "gray",
  medium: "blue",
  high: "amber",
  critical: "red",
};

export function FeedbackList({ projectId, onSelect }: FeedbackListProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const tShared = useTranslations("shared");
  const { page, pageSize, goToPage } = usePagination();
  const [statusFilter, setStatusFilter] = useState("");
  const [typeFilter, setTypeFilter] = useState("");

  const { data, isLoading } = useApiQuery(
    ["lowcode", "feedback", String(page), statusFilter, typeFilter, projectId || ""],
    () =>
      feedbackApi.list({
        page,
        page_size: pageSize,
        project_id: projectId,
        status: statusFilter || undefined,
        feedback_type: typeFilter || undefined,
      })
  );

  return (
    <div>
      <div className="mb-4">
        <SearchBar
          placeholder="Filter feedback..."
          onSearch={() => {}}
          filters={[
            {
              key: "status",
              label: tShared("allStatuses"),
              value: statusFilter,
              onChange: setStatusFilter,
              options: [
                { value: "open", label: tShared("open") },
                { value: "in_progress", label: tShared("inProgress") },
                { value: "resolved", label: "Resolved" },
                { value: "closed", label: tShared("closed") },
              ],
            },
            {
              key: "type",
              label: tCommon("allTypes"),
              value: typeFilter,
              onChange: setTypeFilter,
              options: [
                { value: "bug", label: t("feedbackTypeBug") },
                { value: "feature", label: t("feedbackTypeFeature") },
                { value: "improvement", label: t("feedbackTypeImprovement") },
              ],
            },
          ]}
        />
      </div>

      {isLoading ? (
        <div className="space-y-3">
          {[1, 2, 3].map((i) => (
            <div key={i} className="h-20 animate-pulse rounded-lg border bg-white" />
          ))}
        </div>
      ) : (data?.items ?? []).length === 0 ? (
        <div className="py-12 text-center text-sm text-gray-500">{t("noResults")}</div>
      ) : (
        <div className="space-y-3">
          {data?.items.map((item) => {
            const TypeIcon = typeIcons[item.feedback_type] || MessageSquare;
            return (
              <div
                key={item.id}
                className="cursor-pointer rounded-lg border border-gray-200 bg-white p-6 shadow-sm transition-colors hover:bg-gray-50"
                onClick={() => onSelect?.(item)}
              >
                <div className="flex items-start gap-3">
                  <TypeIcon className="mt-0.5 h-5 w-5 shrink-0 text-gray-400" />
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                      <h4 className="truncate text-sm font-medium text-gray-900">{item.title}</h4>
                      <StatusBadge status={item.status} />
                      <Badge color={priorityColors[item.priority] as "gray" | "blue" | "amber" | "red"}>
                        {item.priority}
                      </Badge>
                    </div>
                    <p className="mt-1 line-clamp-2 text-sm text-gray-500">{item.description}</p>
                    <div className="mt-2 flex items-center gap-3 text-xs text-gray-400">
                      <span>{item.reporter_name || "Unknown"}</span>
                      <span>{formatDateTime(item.created_at)}</span>
                      {item.comments && item.comments.length > 0 && (
                        <span className="flex items-center gap-1">
                          <MessageSquare className="h-3 w-3" />
                          {item.comments.length}
                        </span>
                      )}
                    </div>
                  </div>
                </div>
              </div>
            );
          })}
        </div>
      )}

      {data && data.total_pages > 1 && (
        <div className="mt-4 flex items-center justify-center gap-2">
          <button
            onClick={() => goToPage(page - 1)}
            disabled={page <= 1}
            className="rounded-md px-3 py-1 text-sm text-gray-600 hover:bg-gray-100 disabled:opacity-50"
          >
            {tCommon("previous")}
          </button>
          <span className="text-sm text-gray-500">
            {tCommon("pageOf", { page, totalPages: data.total_pages })}
          </span>
          <button
            onClick={() => goToPage(page + 1)}
            disabled={page >= data.total_pages}
            className="rounded-md px-3 py-1 text-sm text-gray-600 hover:bg-gray-100 disabled:opacity-50"
          >
            {tCommon("next")}
          </button>
        </div>
      )}
    </div>
  );
}
