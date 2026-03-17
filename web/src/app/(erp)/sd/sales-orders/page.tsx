"use client";

import { useState, useMemo } from "react";
import { useRouter } from "next/navigation";
import { type ColumnDef } from "@tanstack/react-table";
import { useTranslations } from "next-intl";
import { Plus } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { DataTable } from "@/components/ui/data-table";
import { Button } from "@/components/ui/button";
import { StatusBadge } from "@/components/ui/badge";
import { SearchBar } from "@/components/forms/search-bar";
import { ExportButton } from "@/components/shared/export-button";
import { PrintButton } from "@/components/shared/print-button";
import { BulkActionBar } from "@/components/ui/bulk-action-bar";
import { StatusFilter, useSoStatuses } from "@/components/shared/status-filter";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { usePagination } from "@/lib/hooks/use-pagination";
import { sdApi, type SalesOrder } from "@/lib/api/sd";
import { formatCurrency, formatDate } from "@/lib/utils";

export default function SalesOrdersPage() {
  const t = useTranslations("sd");
  const tc = useTranslations("common");

  const columns = useMemo<ColumnDef<SalesOrder, unknown>[]>(() => [
    {
      accessorKey: "order_number",
      header: t("orderNo"),
      cell: ({ row }) => (
        <span className="font-medium text-blue-600">{row.original.order_number}</span>
      ),
    },
    {
      accessorKey: "customer_name",
      header: t("customer"),
      cell: ({ row }) => row.original.customer_name ?? "-",
    },
    {
      accessorKey: "order_date",
      header: t("orderDate"),
      cell: ({ row }) => formatDate(row.original.order_date),
    },
    {
      accessorKey: "delivery_date",
      header: t("deliveryDate"),
      cell: ({ row }) => formatDate(row.original.delivery_date ?? row.original.requested_delivery_date),
    },
    {
      accessorKey: "total_amount",
      header: tc("total"),
      cell: ({ row }) => (
        <span className="font-mono">{formatCurrency(row.original.total_amount, row.original.currency)}</span>
      ),
    },
    {
      accessorKey: "status",
      header: tc("status"),
      cell: ({ row }) => <StatusBadge status={row.original.status} />,
    },
  ], [t, tc]);
  const router = useRouter();
  const { page, pageSize, goToPage } = usePagination();
  const [search, setSearch] = useState("");
  const [statusFilter, setStatusFilter] = useState("");
  const [selectedRows, setSelectedRows] = useState<SalesOrder[]>([]);
  const soStatuses = useSoStatuses();

  const { data, isLoading } = useApiQuery(
    ["sd", "sales-orders", String(page), search, statusFilter],
    () =>
      sdApi.getSalesOrders({
        page,
        page_size: pageSize,
        search: search || undefined,
        status: statusFilter || undefined,
      })
  );

  return (
    <div>
      <PageHeader
        title={t("salesOrders")}
        description={t("manageSalesOrders")}
        actions={
          <>
            <PrintButton />
            <ExportButton
              data={data?.items || []}
              filename="sales-orders"
              sheetName="Sales Orders"
            />
            <Button onClick={() => router.push("/sd/sales-orders/new")}>
              <Plus className="h-4 w-4" />
              {t("createOrder")}
            </Button>
          </>
        }
      />

      <div className="mb-4 flex flex-wrap items-center gap-3">
        <SearchBar
          placeholder={t("searchSalesOrders")}
          onSearch={setSearch}
        />
        <StatusFilter
          value={statusFilter}
          onChange={setStatusFilter}
          options={soStatuses}
        />
      </div>

      <BulkActionBar
        selectedCount={selectedRows.length}
        onClearSelection={() => setSelectedRows([])}
      >
        <ExportButton
          data={selectedRows}
          filename="selected-sales-orders"
          sheetName="Selected Orders"
        />
      </BulkActionBar>

      <DataTable
        columns={columns}
        data={data?.items || []}
        enableSelection
        onSelectionChange={setSelectedRows}
        getRowId={(row) => row.id}
        page={page}
        pageSize={pageSize}
        total={data?.total || 0}
        totalPages={data?.total_pages || 1}
        onPageChange={goToPage}
        onRowClick={(row) => router.push(`/sd/sales-orders/${row.id}`)}
        isLoading={isLoading}
        emptyTitle={t("noSalesOrdersFound")}
        emptyDescription={t("createFirstOrder")}
      />
    </div>
  );
}
