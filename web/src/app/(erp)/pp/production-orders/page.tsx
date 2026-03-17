"use client";

import { useState, useMemo } from "react";
import { useRouter } from "next/navigation";
import { type ColumnDef } from "@tanstack/react-table";
import { useTranslations } from "next-intl";
import { Plus } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { DataTable } from "@/components/ui/data-table";
import { Button } from "@/components/ui/button";
import { StatusBadge, Badge } from "@/components/ui/badge";
import { SearchBar } from "@/components/forms/search-bar";
import { ExportButton } from "@/components/shared/export-button";
import { PrintButton } from "@/components/shared/print-button";
import { StatusFilter, useProductionStatuses } from "@/components/shared/status-filter";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { usePagination } from "@/lib/hooks/use-pagination";
import { ppApi, type ProductionOrder } from "@/lib/api/pp";
import { formatDate, formatNumber } from "@/lib/utils";

export default function ProductionOrdersPage() {
  const t = useTranslations("pp");
  const tc = useTranslations("common");

  const columns = useMemo<ColumnDef<ProductionOrder, unknown>[]>(() => [
    {
      accessorKey: "order_number",
      header: t("orderNo"),
      cell: ({ row }) => (
        <span className="font-medium text-blue-600">{row.original.order_number}</span>
      ),
    },
    {
      accessorKey: "material_name",
      header: t("material"),
      cell: ({ row }) => (
        <div>
          <p className="font-medium text-gray-900">{row.original.material_name ?? "-"}</p>
          <p className="text-xs text-gray-500">{row.original.material_number ?? ""}</p>
        </div>
      ),
    },
    {
      accessorKey: "quantity",
      header: tc("quantity"),
      cell: ({ row }) => {
        const qty = row.original.quantity ?? row.original.planned_quantity ?? 0;
        return <span>{formatNumber(qty)} {row.original.unit ?? ""}</span>;
      },
    },
    {
      accessorKey: "completed_quantity",
      header: t("completed"),
      cell: ({ row }) => {
        const completed = row.original.completed_quantity ?? row.original.actual_quantity ?? 0;
        const total = row.original.quantity ?? row.original.planned_quantity ?? 0;
        return (
          <Badge color={completed >= total ? "green" : "amber"}>
            {formatNumber(completed)} / {formatNumber(total)}
          </Badge>
        );
      },
    },
    {
      accessorKey: "planned_start",
      header: t("plannedStart"),
      cell: ({ row }) => formatDate(row.original.planned_start),
    },
    {
      accessorKey: "planned_end",
      header: t("plannedEnd"),
      cell: ({ row }) => formatDate(row.original.planned_end),
    },
    {
      accessorKey: "status",
      header: tc("status"),
      cell: ({ row }) => <StatusBadge status={row.original.status} />,
    },
  ], [t, tc]);
  const { page, pageSize, goToPage } = usePagination();
  const [search, setSearch] = useState("");
  const [statusFilter, setStatusFilter] = useState("");
  const router = useRouter();
  const productionStatuses = useProductionStatuses();

  const { data, isLoading } = useApiQuery(
    ["pp", "production-orders", String(page), search, statusFilter],
    () =>
      ppApi.getProductionOrders({
        page,
        page_size: pageSize,
        search: search || undefined,
        status: statusFilter || undefined,
      })
  );

  return (
    <div>
      <PageHeader
        title={t("productionOrders")}
        description={t("manageProduction")}
        actions={
          <>
            <PrintButton />
            <ExportButton
              data={data?.items || []}
              filename="production-orders"
              sheetName="Production Orders"
            />
            <Button>
              <Plus className="h-4 w-4" />
              {t("createProductionOrder")}
            </Button>
          </>
        }
      />

      <div className="mb-4 flex flex-wrap items-center gap-3">
        <SearchBar
          placeholder={t("searchProductionOrders")}
          onSearch={setSearch}
        />
        <StatusFilter
          value={statusFilter}
          onChange={setStatusFilter}
          options={productionStatuses}
        />
      </div>

      <DataTable
        columns={columns}
        data={data?.items || []}
        page={page}
        pageSize={pageSize}
        total={data?.total || 0}
        totalPages={data?.total_pages || 1}
        onPageChange={goToPage}
        onRowClick={(row) => router.push(`/pp/production-orders/${row.id}`)}
        isLoading={isLoading}
        emptyTitle={t("noProductionOrdersFound")}
        emptyDescription={t("createFirstProductionOrder")}
      />
    </div>
  );
}
