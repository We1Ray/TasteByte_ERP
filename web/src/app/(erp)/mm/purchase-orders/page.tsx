"use client";

import { useState, useMemo } from "react";
import { useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import { type ColumnDef } from "@tanstack/react-table";
import { Plus } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { DataTable } from "@/components/ui/data-table";
import { Button } from "@/components/ui/button";
import { StatusBadge } from "@/components/ui/badge";
import { SearchBar } from "@/components/forms/search-bar";
import { ExportButton } from "@/components/shared/export-button";
import { PrintButton } from "@/components/shared/print-button";
import { StatusFilter, usePoStatuses } from "@/components/shared/status-filter";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { usePagination } from "@/lib/hooks/use-pagination";
import { mmApi, type PurchaseOrder } from "@/lib/api/mm";
import { formatCurrency, formatDate } from "@/lib/utils";

export default function PurchaseOrdersPage() {
  const router = useRouter();
  const t = useTranslations("mm");
  const tCommon = useTranslations("common");
  const tShared = useTranslations("shared");
  const { page, pageSize, goToPage } = usePagination();
  const [search, setSearch] = useState("");
  const [statusFilter, setStatusFilter] = useState("");
  const poStatuses = usePoStatuses();

  const columns = useMemo<ColumnDef<PurchaseOrder, unknown>[]>(() => [
    {
      accessorKey: "po_number",
      header: t("poNumber"),
      cell: ({ row }) => (
        <span className="font-medium text-blue-600">{row.original.po_number}</span>
      ),
    },
    {
      accessorKey: "vendor_name",
      header: "Vendor",
      cell: ({ row }) => row.original.vendor_name ?? "-",
    },
    {
      accessorKey: "order_date",
      header: t("orderDate"),
      cell: ({ row }) => formatDate(row.original.order_date),
    },
    {
      accessorKey: "delivery_date",
      header: t("deliveryDate"),
      cell: ({ row }) => formatDate(row.original.delivery_date),
    },
    {
      accessorKey: "total_amount",
      header: tCommon("total"),
      cell: ({ row }) => (
        <span className="font-mono">{formatCurrency(row.original.total_amount, row.original.currency)}</span>
      ),
    },
    {
      accessorKey: "status",
      header: tCommon("status"),
      cell: ({ row }) => <StatusBadge status={row.original.status} />,
    },
  ], [t, tCommon]);

  const { data, isLoading } = useApiQuery(
    ["mm", "purchase-orders", String(page), search, statusFilter],
    () =>
      mmApi.getPurchaseOrders({
        page,
        page_size: pageSize,
        search: search || undefined,
        status: statusFilter || undefined,
      })
  );

  return (
    <div>
      <PageHeader
        title={t("purchaseOrders")}
        description={t("managePurchaseOrders")}
        actions={
          <>
            <PrintButton />
            <ExportButton
              data={data?.items || []}
              filename="purchase-orders"
              sheetName="Purchase Orders"
            />
            <Button onClick={() => router.push("/mm/purchase-orders/new")}>
              <Plus className="h-4 w-4" />
              {t("createPo")}
            </Button>
          </>
        }
      />

      <div className="mb-4 flex flex-wrap items-center gap-3">
        <SearchBar
          placeholder={t("searchPurchaseOrders")}
          onSearch={setSearch}
        />
        <StatusFilter
          value={statusFilter}
          onChange={setStatusFilter}
          options={poStatuses}
          allLabel={tShared("allStatuses")}
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
        onRowClick={(row) => router.push(`/mm/purchase-orders/${row.id}`)}
        isLoading={isLoading}
        emptyTitle={t("noPurchaseOrdersFound")}
        emptyDescription={t("createFirstPo")}
      />
    </div>
  );
}
