"use client";

import { useState, useMemo } from "react";
import { useRouter } from "next/navigation";
import { type ColumnDef } from "@tanstack/react-table";
import { useTranslations } from "next-intl";
import { PageHeader } from "@/components/layout/page-header";
import { DataTable } from "@/components/ui/data-table";
import { StatusBadge } from "@/components/ui/badge";
import { SearchBar } from "@/components/forms/search-bar";
import { ExportButton } from "@/components/shared/export-button";
import { PrintButton } from "@/components/shared/print-button";
import { StatusFilter, useInvoiceStatuses } from "@/components/shared/status-filter";
import { DateRangePicker } from "@/components/ui/date-range-picker";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { usePagination } from "@/lib/hooks/use-pagination";
import { sdApi, type Invoice } from "@/lib/api/sd";
import { formatCurrency, formatDate } from "@/lib/utils";

export default function InvoicesPage() {
  const t = useTranslations("sd");
  const tc = useTranslations("common");

  const columns = useMemo<ColumnDef<Invoice, unknown>[]>(() => [
    {
      accessorKey: "invoice_number",
      header: t("invoiceNo"),
      cell: ({ row }) => (
        <span className="font-medium text-blue-600">{row.original.invoice_number}</span>
      ),
    },
    { accessorKey: "customer_name", header: t("customer") },
    {
      accessorKey: "invoice_date",
      header: t("invoiceDate"),
      cell: ({ row }) => formatDate(row.original.invoice_date),
    },
    {
      accessorKey: "due_date",
      header: t("dueDate"),
      cell: ({ row }) => formatDate(row.original.due_date),
    },
    {
      accessorKey: "total_amount",
      header: tc("total"),
      cell: ({ row }) => (
        <span className="font-mono">{formatCurrency(row.original.total_amount, row.original.currency)}</span>
      ),
    },
    {
      accessorKey: "paid_amount",
      header: t("paid"),
      cell: ({ row }) => (
        <span className="font-mono">{formatCurrency(row.original.paid_amount, row.original.currency)}</span>
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
  const invoiceStatuses = useInvoiceStatuses();
  const [startDate, setStartDate] = useState("");
  const [endDate, setEndDate] = useState("");

  const { data, isLoading } = useApiQuery(
    ["sd", "invoices", String(page), search, statusFilter, startDate, endDate],
    () =>
      sdApi.getInvoices({
        page,
        page_size: pageSize,
        search: search || undefined,
        status: statusFilter || undefined,
      })
  );

  return (
    <div>
      <PageHeader
        title={t("invoices")}
        description={t("manageInvoices")}
        actions={
          <>
            <PrintButton />
            <ExportButton
              data={data?.items || []}
              filename="invoices"
              sheetName="Invoices"
            />
          </>
        }
      />

      <div className="mb-4 flex flex-wrap items-center gap-3">
        <SearchBar
          placeholder={t("searchInvoices")}
          onSearch={setSearch}
        />
        <StatusFilter
          value={statusFilter}
          onChange={setStatusFilter}
          options={invoiceStatuses}
        />
      </div>

      <div className="mb-4">
        <DateRangePicker
          startDate={startDate}
          endDate={endDate}
          onStartDateChange={setStartDate}
          onEndDateChange={setEndDate}
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
        onRowClick={(row) => router.push(`/sd/invoices/${row.id}`)}
        isLoading={isLoading}
        emptyTitle={t("noInvoicesFound")}
        emptyDescription={t("invoicesWillAppear")}
      />
    </div>
  );
}
