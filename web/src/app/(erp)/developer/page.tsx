"use client";

import { useMemo } from "react";
import { useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import { FileText, MessageSquare, Plus, Clock, Rocket, History } from "lucide-react";
import { type ColumnDef } from "@tanstack/react-table";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { DataTable } from "@/components/ui/data-table";
import { StatusBadge } from "@/components/ui/badge";
import { KpiCard } from "@/components/charts/kpi-card";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { operationsApi, feedbackApi, releasesApi } from "@/lib/api/lowcode";
import { formatDateTime } from "@/lib/utils";
import type { LowCodeOperation, Release } from "@/lib/types/lowcode";

export default function DeveloperDashboardPage() {
  const router = useRouter();
  const t = useTranslations("developer");
  const tCommon = useTranslations("common");

  const operationColumns = useMemo<ColumnDef<LowCodeOperation, unknown>[]>(
    () => [
      { accessorKey: "code", header: t("code") },
      { accessorKey: "name", header: tCommon("name") },
      { accessorKey: "operation_type", header: tCommon("type") },
      {
        accessorKey: "status",
        header: tCommon("status"),
        cell: ({ row }) => <StatusBadge status={row.original.status} />,
      },
      {
        accessorKey: "updated_at",
        header: t("lastUpdated"),
        cell: ({ row }) => formatDateTime(row.original.updated_at),
      },
    ],
    [t, tCommon]
  );

  const { data: operations, isLoading } = useApiQuery(
    ["lowcode", "operations", "my"],
    () => operationsApi.list({ page_size: 10 })
  );

  const { data: openTickets } = useApiQuery(
    ["lowcode", "feedback", "open"],
    () => feedbackApi.list({ status: "open", page_size: 1 })
  );

  const { data: myReleases } = useApiQuery(
    ["lowcode", "releases", "my"],
    () => releasesApi.list({ page_size: 5 })
  );

  // Recently updated operations (sorted by updated_at from the operations list)
  const recentlyUpdated = [...(operations?.items ?? [])]
    .sort((a, b) => new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime())
    .slice(0, 5);

  return (
    <div>
      <PageHeader
        title={t("dashboard")}
        description={t("dashboardDesc")}
      />

      <div className="mb-6 grid grid-cols-1 gap-4 sm:grid-cols-3">
        <div className="cursor-pointer" onClick={() => router.push("/developer/operations")}>
          <KpiCard
            title={t("myOperations")}
            value={String(operations?.total ?? 0)}
            icon={FileText}
            iconColor="bg-blue-100 text-blue-600"
          />
        </div>
        <div className="cursor-pointer" onClick={() => router.push("/developer/feedback")}>
          <KpiCard
            title={t("openTickets")}
            value={String(openTickets?.total ?? 0)}
            icon={MessageSquare}
            iconColor="bg-amber-100 text-amber-600"
          />
        </div>
        <div className="cursor-pointer" onClick={() => router.push("/admin/releases")}>
          <KpiCard
            title={t("releases")}
            value={String(myReleases?.total ?? 0)}
            icon={Rocket}
            iconColor="bg-green-100 text-green-600"
          />
        </div>
      </div>

      <div className="mb-6 grid grid-cols-2 gap-3 sm:grid-cols-4">
        <button
          onClick={() => router.push("/developer/operations/new")}
          className="flex items-center gap-3 rounded-lg border border-gray-200 bg-white px-4 py-3 text-left transition-colors hover:bg-gray-50"
        >
          <div className="rounded-lg bg-blue-50 p-2 text-blue-600">
            <Plus className="h-4 w-4" />
          </div>
          <span className="text-sm font-medium text-gray-700">{t("createOperation")}</span>
        </button>
        <button
          onClick={() => router.push("/developer/operations")}
          className="flex items-center gap-3 rounded-lg border border-gray-200 bg-white px-4 py-3 text-left transition-colors hover:bg-gray-50"
        >
          <div className="rounded-lg bg-green-50 p-2 text-green-600">
            <FileText className="h-4 w-4" />
          </div>
          <span className="text-sm font-medium text-gray-700">{t("operationsList")}</span>
        </button>
        <button
          onClick={() => router.push("/developer/feedback")}
          className="flex items-center gap-3 rounded-lg border border-gray-200 bg-white px-4 py-3 text-left transition-colors hover:bg-gray-50"
        >
          <div className="rounded-lg bg-amber-50 p-2 text-amber-600">
            <MessageSquare className="h-4 w-4" />
          </div>
          <span className="text-sm font-medium text-gray-700">{t("feedback")}</span>
        </button>
        <button
          onClick={() => router.push("/developer/journal")}
          className="flex items-center gap-3 rounded-lg border border-gray-200 bg-white px-4 py-3 text-left transition-colors hover:bg-gray-50"
        >
          <div className="rounded-lg bg-purple-50 p-2 text-purple-600">
            <History className="h-4 w-4" />
          </div>
          <span className="text-sm font-medium text-gray-700">{t("journal")}</span>
        </button>
      </div>

      <div className="mb-6 grid grid-cols-1 gap-6 lg:grid-cols-3">
        {/* Operations table (2/3 width) */}
        <div className="lg:col-span-2">
          <Card padding={false}>
            <div className="p-4">
              <CardHeader>
                <CardTitle>{t("operationsTitle")}</CardTitle>
                <Button size="sm" onClick={() => router.push("/developer/operations/new")}>
                  <Plus className="h-4 w-4" />
                  {t("newOperation")}
                </Button>
              </CardHeader>
            </div>
            <DataTable
              columns={operationColumns}
              data={operations?.items || []}
              onRowClick={(row) => router.push(`/developer/operations/${row.id}`)}
              isLoading={isLoading}
              emptyTitle={t("noOperationsYet")}
              emptyDescription={t("createFirstOperation")}
            />
          </Card>
        </div>

        {/* Releases (1/3 width) */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Rocket className="h-4 w-4" />
              {t("myReleases")}
            </CardTitle>
          </CardHeader>
          <div className="space-y-3">
            {(myReleases?.items ?? []).length === 0 ? (
              <p className="py-4 text-center text-sm text-gray-500">{t("noReleasesYet")}</p>
            ) : (
              (myReleases?.items ?? []).map((release: Release) => (
                <div key={release.id} className="flex items-center justify-between border-b border-gray-100 pb-3 last:border-0">
                  <div>
                    <p className="text-sm font-medium text-gray-900">{release.title}</p>
                    <p className="mt-0.5 text-xs text-gray-500">
                      {release.release_number} - {formatDateTime(release.created_at)}
                    </p>
                  </div>
                  <StatusBadge status={release.status} />
                </div>
              ))
            )}
          </div>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>{t("recentlyUpdated")}</CardTitle>
        </CardHeader>
        <div className="space-y-3">
          {recentlyUpdated.length === 0 ? (
            <p className="py-4 text-center text-sm text-gray-500">{t("noRecentUpdates")}</p>
          ) : (
            recentlyUpdated.map((op) => (
              <div
                key={op.id}
                className="flex cursor-pointer items-start gap-3 border-b border-gray-100 pb-3 last:border-0 hover:bg-gray-50"
                onClick={() => router.push(`/developer/operations/${op.id}`)}
              >
                <Clock className="mt-0.5 h-4 w-4 shrink-0 text-gray-400" />
                <div className="flex-1">
                  <p className="text-sm text-gray-700">{op.code} - {op.name}</p>
                  <p className="mt-0.5 text-xs text-gray-400">
                    {formatDateTime(op.updated_at)}
                  </p>
                </div>
                <StatusBadge status={op.status} />
              </div>
            ))
          )}
        </div>
      </Card>
    </div>
  );
}
