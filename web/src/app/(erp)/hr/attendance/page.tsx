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
import { hrApi, type AttendanceRecord } from "@/lib/api/hr";
import { formatDate, formatNumber } from "@/lib/utils";

export default function AttendancePage() {
  const t = useTranslations("hr");
  const tc = useTranslations("common");

  const columns = useMemo<ColumnDef<AttendanceRecord, unknown>[]>(() => [
    { accessorKey: "employee_name", header: t("employee") },
    {
      accessorKey: "date",
      header: tc("date"),
      cell: ({ row }) => formatDate(row.original.date),
    },
    {
      accessorKey: "check_in",
      header: t("checkIn"),
      cell: ({ row }) => row.original.check_in || "-",
    },
    {
      accessorKey: "check_out",
      header: t("checkOut"),
      cell: ({ row }) => row.original.check_out || "-",
    },
    {
      accessorKey: "hours_worked",
      header: t("hoursWorked"),
      cell: ({ row }) => (
        <span className="font-mono">{formatNumber(row.original.hours_worked, 1)}h</span>
      ),
    },
    {
      accessorKey: "overtime_hours",
      header: t("overtime"),
      cell: ({ row }) =>
        row.original.overtime_hours > 0 ? (
          <Badge color="amber">{formatNumber(row.original.overtime_hours, 1)}h</Badge>
        ) : (
          <span className="text-gray-400">-</span>
        ),
    },
    {
      accessorKey: "status",
      header: tc("status"),
      cell: ({ row }) => <StatusBadge status={row.original.status} />,
    },
  ], [t, tc]);
  const { page, pageSize, goToPage } = usePagination();
  const [search, setSearch] = useState("");

  const { data, isLoading } = useApiQuery(
    ["hr", "attendance", String(page), search],
    () =>
      hrApi.getAttendance({
        page,
        page_size: pageSize,
      })
  );

  return (
    <div>
      <PageHeader
        title={t("attendance")}
        description={t("trackAttendance")}
      />

      <div className="mb-4">
        <SearchBar
          placeholder={t("searchAttendance")}
          onSearch={setSearch}
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
        isLoading={isLoading}
        emptyTitle={t("noAttendanceFound")}
        emptyDescription={t("attendanceWillAppear")}
      />
    </div>
  );
}
