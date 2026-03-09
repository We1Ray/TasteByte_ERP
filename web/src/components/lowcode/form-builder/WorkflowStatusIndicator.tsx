"use client";

import { AlertTriangle } from "lucide-react";
import { useTranslations } from "next-intl";
import { StatusBadge } from "@/components/ui/badge";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { releasesApi } from "@/lib/api/lowcode";

interface WorkflowStatusIndicatorProps {
  operationId: string;
}

export function WorkflowStatusIndicator({ operationId }: WorkflowStatusIndicatorProps) {
  const t = useTranslations("lowcode");

  const { data: releases } = useApiQuery(
    ["lowcode", "releases", "latest", operationId],
    () => releasesApi.list({ page_size: 1, status: undefined } as never),
    { enabled: !!operationId }
  );

  const latestRelease = releases?.items?.[0];

  if (!latestRelease) {
    return (
      <span className="inline-flex items-center rounded-full bg-gray-100 px-2 py-0.5 text-xs text-gray-500">
        {t("noReleases")}
      </span>
    );
  }

  return (
    <div className="inline-flex items-center gap-1.5">
      <StatusBadge status={latestRelease.status} />
      {latestRelease.status === "rejected" && latestRelease.review_notes && (
        <div className="group relative">
          <AlertTriangle className="h-3.5 w-3.5 text-amber-500" />
          <div className="absolute bottom-full left-1/2 z-50 mb-2 hidden -translate-x-1/2 rounded-md bg-gray-900 px-3 py-2 text-xs text-white shadow-lg group-hover:block">
            <div className="max-w-xs">
              <p className="font-medium">{t("reviewNotesLabel")}</p>
              <p className="mt-1">{latestRelease.review_notes}</p>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
