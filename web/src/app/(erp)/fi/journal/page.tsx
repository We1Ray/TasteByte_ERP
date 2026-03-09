"use client";

import { useState, useMemo } from "react";
import { useRouter } from "next/navigation";
import { type ColumnDef } from "@tanstack/react-table";
import { Plus } from "lucide-react";
import { useTranslations } from "next-intl";
import { PageHeader } from "@/components/layout/page-header";
import { DataTable } from "@/components/ui/data-table";
import { Button } from "@/components/ui/button";
import { StatusBadge } from "@/components/ui/badge";
import { SearchBar } from "@/components/forms/search-bar";
import { ExportButton } from "@/components/shared/export-button";
import { PrintButton } from "@/components/shared/print-button";
import { StatusFilter } from "@/components/shared/status-filter";
import { DateRangePicker } from "@/components/ui/date-range-picker";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { usePagination } from "@/lib/hooks/use-pagination";
import { fiApi, type JournalEntry } from "@/lib/api/fi";
import { formatCurrency, formatDate } from "@/lib/utils";

export default function JournalPage() {
  const t = useTranslations("fi");
  const tCommon = useTranslations("common");
  const tShared = useTranslations("shared");

  const columns = useMemo<ColumnDef<JournalEntry, unknown>[]>(() => [
    {
      accessorKey: "document_number",
      header: t("documentNo"),
      cell: ({ row }) => (
        <span className="font-medium text-blue-600">{row.original.document_number}</span>
      ),
    },
    {
      accessorKey: "posting_date",
      header: t("postingDate"),
      cell: ({ row }) => formatDate(row.original.posting_date),
    },
    { accessorKey: "description", header: tCommon("description") },
    { accessorKey: "reference", header: t("reference") },
    {
      accessorKey: "total_debit",
      header: t("debit"),
      cell: ({ row }) => (
        <span className="font-mono">{formatCurrency(row.original.total_debit)}</span>
      ),
    },
    {
      accessorKey: "total_credit",
      header: t("credit"),
      cell: ({ row }) => (
        <span className="font-mono">{formatCurrency(row.original.total_credit)}</span>
      ),
    },
    {
      accessorKey: "status",
      header: tCommon("status"),
      cell: ({ row }) => <StatusBadge status={row.original.status} />,
    },
  ], [t, tCommon]);

  const journalStatuses = useMemo(() => [
    { value: "Draft", label: tShared("draft") },
    { value: "Posted", label: tShared("posted") },
    { value: "Reversed", label: tShared("reversed") },
  ], [tShared]);

  const router = useRouter();
  const { page, pageSize, goToPage } = usePagination();
  const [search, setSearch] = useState("");
  const [statusFilter, setStatusFilter] = useState("");
  const [startDate, setStartDate] = useState("");
  const [endDate, setEndDate] = useState("");

  const { data, isLoading } = useApiQuery(
    ["fi", "journal", String(page), search, statusFilter, startDate, endDate],
    () =>
      fiApi.getJournalEntries({
        page,
        page_size: pageSize,
        search: search || undefined,
        status: statusFilter || undefined,
        date_from: startDate || undefined,
        date_to: endDate || undefined,
      })
  );

  return (
    <div>
      <PageHeader
        title={t("journalEntries")}
        description={t("manageJournal")}
        actions={
          <>
            <PrintButton />
            <ExportButton
              data={data?.items || []}
              filename="journal-entries"
              sheetName="Journal Entries"
            />
            <Button onClick={() => router.push("/fi/journal/new")}>
              <Plus className="h-4 w-4" />
              {t("newEntry")}
            </Button>
          </>
        }
      />

      <div className="mb-4 flex flex-wrap items-center gap-3">
        <SearchBar
          placeholder={t("searchJournal")}
          onSearch={setSearch}
        />
        <StatusFilter
          value={statusFilter}
          onChange={setStatusFilter}
          options={journalStatuses}
          allLabel={tShared("allStatuses")}
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
        onRowClick={(row) => router.push(`/fi/journal/${row.id}`)}
        isLoading={isLoading}
        emptyTitle={t("noJournalFound")}
        emptyDescription={t("createFirstEntry")}
      />
    </div>
  );
}
