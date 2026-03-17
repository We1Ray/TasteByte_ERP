"use client";

import { useState, useMemo } from "react";
import { useTranslations } from "next-intl";
import { type ColumnDef } from "@tanstack/react-table";
import { PageHeader } from "@/components/layout/page-header";
import { DataTable } from "@/components/ui/data-table";
import { SearchBar } from "@/components/forms/search-bar";
import { KpiCard } from "@/components/charts/kpi-card";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { usePagination } from "@/lib/hooks/use-pagination";
import { mmApi, type StockOverview } from "@/lib/api/mm";
import { formatCurrency, formatNumber } from "@/lib/utils";
import { Package, Warehouse, AlertTriangle } from "lucide-react";

export default function StockPage() {
  const t = useTranslations("mm");
  const tCommon = useTranslations("common");
  const { page, pageSize, goToPage } = usePagination();
  const [search, setSearch] = useState("");

  const columns = useMemo<ColumnDef<StockOverview, unknown>[]>(() => [
    {
      accessorKey: "material_number",
      header: t("materialNo"),
      cell: ({ row }) => (
        <span className="font-medium text-blue-600">{row.original.material_number}</span>
      ),
    },
    { accessorKey: "material_name", header: t("materials") },
    { accessorKey: "warehouse_name", header: t("warehouse") },
    { accessorKey: "storage_bin", header: t("storageBin") },
    {
      accessorKey: "quantity",
      header: tCommon("quantity"),
      cell: ({ row }) => (
        <span className="font-mono">
          {formatNumber(row.original.quantity)} {row.original.unit}
        </span>
      ),
    },
    {
      accessorKey: "value",
      header: t("value"),
      cell: ({ row }) => (
        <span className="font-mono">{formatCurrency(row.original.value)}</span>
      ),
    },
  ], [t, tCommon]);

  const { data, isLoading } = useApiQuery(
    ["mm", "stock", String(page), search],
    () => mmApi.getStock({ page, page_size: pageSize })
  );

  const { data: stockValuation } = useApiQuery(
    ["mm", "stock-valuation"],
    () => mmApi.getStockValuation()
  );

  return (
    <div>
      <PageHeader
        title={t("stockOverview")}
        description={t("viewStockLevels")}
      />

      <div className="mb-6 grid grid-cols-1 gap-4 sm:grid-cols-3">
        <KpiCard
          title={t("totalMaterials")}
          value={formatNumber(data?.total ?? 0)}
          icon={Package}
          iconColor="bg-blue-100 text-blue-600"
        />
        <KpiCard
          title={t("totalValue")}
          value={formatCurrency(stockValuation?.total_value ?? 0)}
          icon={Warehouse}
          iconColor="bg-green-100 text-green-600"
        />
        <KpiCard
          title={t("belowReorderPoint")}
          value={formatNumber((stockValuation?.items ?? []).filter(i => i.quantity <= 0).length)}
          change={t("requiresAttention")}
          changeType="negative"
          icon={AlertTriangle}
          iconColor="bg-amber-100 text-amber-600"
        />
      </div>

      <div className="mb-4">
        <SearchBar
          placeholder={t("searchStock")}
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
        emptyTitle={t("noStockFound")}
        emptyDescription={t("stockWillAppear")}
      />
    </div>
  );
}
