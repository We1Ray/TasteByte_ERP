"use client";

import { useState, useMemo } from "react";
import { type ColumnDef } from "@tanstack/react-table";
import { useTranslations } from "next-intl";
import { PageHeader } from "@/components/layout/page-header";
import { DataTable } from "@/components/ui/data-table";
import { StatusBadge, Badge } from "@/components/ui/badge";
import { SearchBar } from "@/components/forms/search-bar";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { usePagination } from "@/lib/hooks/use-pagination";
import { qmApi, type InspectionLot, type QualityNotification } from "@/lib/api/qm";
import { formatDate } from "@/lib/utils";

export default function QmPage() {
  const t = useTranslations("qm");
  const tCommon = useTranslations("common");

  const inspectionColumns = useMemo<ColumnDef<InspectionLot, unknown>[]>(() => [
    {
      accessorKey: "lot_number",
      header: t("lotNo"),
      cell: ({ row }) => (
        <span className="font-medium text-blue-600">{row.original.lot_number}</span>
      ),
    },
    {
      accessorKey: "material_name",
      header: "Material",
      cell: ({ row }) => (
        <div>
          <p className="font-medium text-gray-900">{row.original.material_name}</p>
          <p className="text-xs text-gray-500">{row.original.material_number}</p>
        </div>
      ),
    },
    { accessorKey: "inspection_type", header: tCommon("type") },
    { accessorKey: "origin", header: t("origin") },
    {
      accessorKey: "quantity",
      header: tCommon("quantity"),
      cell: ({ row }) => (
        <span>{row.original.quantity} {row.original.unit}</span>
      ),
    },
    {
      accessorKey: "planned_date",
      header: t("plannedDate"),
      cell: ({ row }) => formatDate(row.original.planned_date),
    },
    {
      accessorKey: "status",
      header: tCommon("status"),
      cell: ({ row }) => <StatusBadge status={row.original.status} />,
    },
  ], [t, tCommon]);

  const notificationColumns = useMemo<ColumnDef<QualityNotification, unknown>[]>(() => [
    {
      accessorKey: "notification_number",
      header: t("notificationNo"),
      cell: ({ row }) => (
        <span className="font-medium text-blue-600">{row.original.notification_number}</span>
      ),
    },
    { accessorKey: "material_name", header: "Material" },
    { accessorKey: "type", header: tCommon("type") },
    { accessorKey: "description", header: tCommon("description") },
    {
      accessorKey: "priority",
      header: t("priority"),
      cell: ({ row }) => {
        const p = row.original.priority.toLowerCase();
        const color = p === "high" ? "red" : p === "medium" ? "amber" : "blue";
        return <Badge color={color}>{row.original.priority}</Badge>;
      },
    },
    {
      accessorKey: "status",
      header: tCommon("status"),
      cell: ({ row }) => <StatusBadge status={row.original.status} />,
    },
  ], [t, tCommon]);

  const [activeTab, setActiveTab] = useState<"inspections" | "notifications">("inspections");
  const inspPagination = usePagination();
  const notifPagination = usePagination();
  const [search, setSearch] = useState("");

  const { data: inspections, isLoading: inspLoading } = useApiQuery(
    ["qm", "inspections", String(inspPagination.page), search],
    () =>
      qmApi.getInspectionLots({
        page: inspPagination.page,
        page_size: inspPagination.pageSize,
        search: search || undefined,
      }),
    { enabled: activeTab === "inspections" }
  );

  const { data: notifications, isLoading: notifLoading } = useApiQuery(
    ["qm", "notifications", String(notifPagination.page), search],
    () =>
      qmApi.getQualityNotifications({
        page: notifPagination.page,
        page_size: notifPagination.pageSize,
        search: search || undefined,
      }),
    { enabled: activeTab === "notifications" }
  );

  return (
    <div>
      <PageHeader
        title={t("qualityManagement")}
        description={t("qualityManagementDesc")}
      />

      <div className="mb-4 flex gap-1 rounded-lg bg-gray-100 p-1">
        <button
          onClick={() => setActiveTab("inspections")}
          className={`flex-1 rounded-md px-4 py-2 text-sm font-medium transition-colors ${
            activeTab === "inspections"
              ? "bg-white text-gray-900 shadow-sm"
              : "text-gray-500 hover:text-gray-700"
          }`}
        >
          {t("inspectionLots")}
        </button>
        <button
          onClick={() => setActiveTab("notifications")}
          className={`flex-1 rounded-md px-4 py-2 text-sm font-medium transition-colors ${
            activeTab === "notifications"
              ? "bg-white text-gray-900 shadow-sm"
              : "text-gray-500 hover:text-gray-700"
          }`}
        >
          {t("qualityNotifications")}
        </button>
      </div>

      <div className="mb-4">
        <SearchBar
          placeholder={activeTab === "inspections" ? t("searchInspectionLots") : t("searchNotifications")}
          onSearch={setSearch}
        />
      </div>

      {activeTab === "inspections" ? (
        <DataTable
          columns={inspectionColumns}
          data={inspections?.items || []}
          page={inspPagination.page}
          pageSize={inspPagination.pageSize}
          total={inspections?.total || 0}
          totalPages={inspections?.total_pages || 1}
          onPageChange={inspPagination.goToPage}
          isLoading={inspLoading}
          emptyTitle={t("noInspectionLotsFound")}
          emptyDescription={t("inspectionLotsWillAppear")}
        />
      ) : (
        <DataTable
          columns={notificationColumns}
          data={notifications?.items || []}
          page={notifPagination.page}
          pageSize={notifPagination.pageSize}
          total={notifications?.total || 0}
          totalPages={notifications?.total_pages || 1}
          onPageChange={notifPagination.goToPage}
          isLoading={notifLoading}
          emptyTitle={t("noNotificationsFound")}
          emptyDescription={t("notificationsDesc")}
        />
      )}
    </div>
  );
}
