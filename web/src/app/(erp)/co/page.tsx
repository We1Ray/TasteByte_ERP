"use client";

import { useState, useMemo } from "react";
import { type ColumnDef } from "@tanstack/react-table";
import { useTranslations } from "next-intl";
import { PageHeader } from "@/components/layout/page-header";
import { DataTable } from "@/components/ui/data-table";
import { Badge } from "@/components/ui/badge";
import { SearchBar } from "@/components/forms/search-bar";
import { KpiCard } from "@/components/charts/kpi-card";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { usePagination } from "@/lib/hooks/use-pagination";
import { coApi, type CostCenter, type ProfitCenter } from "@/lib/api/co";
import { formatCurrency } from "@/lib/utils";
import { DollarSign, TrendingUp, PieChart } from "lucide-react";

export default function CoPage() {
  const t = useTranslations("co");
  const tFi = useTranslations("fi");
  const tCommon = useTranslations("common");
  const tShared = useTranslations("shared");

  const costCenterColumns = useMemo<ColumnDef<CostCenter, unknown>[]>(() => [
    {
      accessorKey: "cost_center_number",
      header: t("costCenterNo"),
      cell: ({ row }) => (
        <span className="font-medium text-blue-600">{row.original.cost_center_number}</span>
      ),
    },
    { accessorKey: "name", header: tCommon("name") },
    { accessorKey: "category", header: t("category") },
    { accessorKey: "responsible_person", header: t("responsible") },
    {
      accessorKey: "planned_costs",
      header: t("planned"),
      cell: ({ row }) => (
        <span className="font-mono">{formatCurrency(row.original.planned_costs)}</span>
      ),
    },
    {
      accessorKey: "actual_costs",
      header: t("actual"),
      cell: ({ row }) => (
        <span className="font-mono">{formatCurrency(row.original.actual_costs)}</span>
      ),
    },
    {
      accessorKey: "variance",
      header: t("variance"),
      cell: ({ row }) => {
        const variance = row.original.actual_costs - row.original.planned_costs;
        return (
          <span className={`font-mono ${variance > 0 ? "text-red-600" : "text-green-600"}`}>
            {variance > 0 ? "+" : ""}{formatCurrency(variance)}
          </span>
        );
      },
    },
    {
      accessorKey: "is_active",
      header: tCommon("status"),
      cell: ({ row }) => (
        <Badge color={row.original.is_active ? "green" : "gray"}>
          {row.original.is_active ? tCommon("active") : tShared("inactive")}
        </Badge>
      ),
    },
  ], [t, tCommon, tShared]);

  const profitCenterColumns = useMemo<ColumnDef<ProfitCenter, unknown>[]>(() => [
    {
      accessorKey: "profit_center_number",
      header: t("profitCenterNo"),
      cell: ({ row }) => (
        <span className="font-medium text-blue-600">{row.original.profit_center_number}</span>
      ),
    },
    { accessorKey: "name", header: tCommon("name") },
    { accessorKey: "responsible_person", header: t("responsible") },
    {
      accessorKey: "revenue",
      header: tFi("revenue"),
      cell: ({ row }) => (
        <span className="font-mono text-green-600">{formatCurrency(row.original.revenue)}</span>
      ),
    },
    {
      accessorKey: "costs",
      header: t("costs"),
      cell: ({ row }) => (
        <span className="font-mono text-red-600">{formatCurrency(row.original.costs)}</span>
      ),
    },
    {
      accessorKey: "profit",
      header: t("profit"),
      cell: ({ row }) => (
        <span className={`font-mono font-medium ${row.original.profit >= 0 ? "text-green-600" : "text-red-600"}`}>
          {formatCurrency(row.original.profit)}
        </span>
      ),
    },
  ], [t, tFi, tCommon]);

  const [activeTab, setActiveTab] = useState<"cost" | "profit">("cost");
  const costPagination = usePagination();
  const profitPagination = usePagination();
  const [search, setSearch] = useState("");

  const { data: costCenters, isLoading: costLoading } = useApiQuery(
    ["co", "cost-centers", String(costPagination.page), search],
    () =>
      coApi.getCostCenters({
        page: costPagination.page,
        page_size: costPagination.pageSize,
        search: search || undefined,
      }),
    { enabled: activeTab === "cost" }
  );

  const { data: profitCenters, isLoading: profitLoading } = useApiQuery(
    ["co", "profit-centers", String(profitPagination.page), search],
    () =>
      coApi.getProfitCenters({
        page: profitPagination.page,
        page_size: profitPagination.pageSize,
        search: search || undefined,
      }),
    { enabled: activeTab === "profit" }
  );

  // Fetch all data for KPI aggregation (not paginated by tab)
  const { data: allCostCenters } = useApiQuery(
    ["co", "cost-centers-kpi"],
    () => coApi.getCostCenters({ page: 1, page_size: 9999 }),
  );
  const { data: allProfitCenters } = useApiQuery(
    ["co", "profit-centers-kpi"],
    () => coApi.getProfitCenters({ page: 1, page_size: 9999 }),
  );

  const totalPlannedCosts = useMemo(
    () => allCostCenters?.items.reduce((sum, cc) => sum + cc.planned_costs, 0) ?? 0,
    [allCostCenters]
  );
  const totalActualCosts = useMemo(
    () => allCostCenters?.items.reduce((sum, cc) => sum + cc.actual_costs, 0) ?? 0,
    [allCostCenters]
  );
  const netProfit = useMemo(
    () => allProfitCenters?.items.reduce((sum, pc) => sum + pc.profit, 0) ?? 0,
    [allProfitCenters]
  );

  return (
    <div>
      <PageHeader
        title={t("controlling")}
        description={t("controllingDesc")}
      />

      <div className="mb-6 grid grid-cols-1 gap-4 sm:grid-cols-3">
        <KpiCard
          title={t("totalPlannedCosts")}
          value={formatCurrency(totalPlannedCosts)}
          icon={DollarSign}
          iconColor="bg-blue-100 text-blue-600"
        />
        <KpiCard
          title={t("totalActualCosts")}
          value={formatCurrency(totalActualCosts)}
          change={totalActualCosts <= totalPlannedCosts ? t("underBudget") : undefined}
          changeType={totalActualCosts <= totalPlannedCosts ? "positive" : "negative"}
          icon={PieChart}
          iconColor="bg-green-100 text-green-600"
        />
        <KpiCard
          title={t("netProfit")}
          value={formatCurrency(netProfit)}
          changeType={netProfit >= 0 ? "positive" : "negative"}
          icon={TrendingUp}
          iconColor="bg-purple-100 text-purple-600"
        />
      </div>

      <div className="mb-4 flex gap-1 rounded-lg bg-gray-100 p-1">
        <button
          onClick={() => setActiveTab("cost")}
          className={`flex-1 rounded-md px-4 py-2 text-sm font-medium transition-colors ${
            activeTab === "cost"
              ? "bg-white text-gray-900 shadow-sm"
              : "text-gray-500 hover:text-gray-700"
          }`}
        >
          {t("costCenters")}
        </button>
        <button
          onClick={() => setActiveTab("profit")}
          className={`flex-1 rounded-md px-4 py-2 text-sm font-medium transition-colors ${
            activeTab === "profit"
              ? "bg-white text-gray-900 shadow-sm"
              : "text-gray-500 hover:text-gray-700"
          }`}
        >
          {t("profitCenters")}
        </button>
      </div>

      <div className="mb-4">
        <SearchBar
          placeholder={activeTab === "cost" ? t("searchCostCenters") : t("searchProfitCenters")}
          onSearch={setSearch}
        />
      </div>

      {activeTab === "cost" ? (
        <DataTable
          columns={costCenterColumns}
          data={costCenters?.items || []}
          page={costPagination.page}
          pageSize={costPagination.pageSize}
          total={costCenters?.total || 0}
          totalPages={costCenters?.total_pages || 1}
          onPageChange={costPagination.goToPage}
          isLoading={costLoading}
          emptyTitle={t("noCostCentersFound")}
          emptyDescription={t("createCostCenters")}
        />
      ) : (
        <DataTable
          columns={profitCenterColumns}
          data={profitCenters?.items || []}
          page={profitPagination.page}
          pageSize={profitPagination.pageSize}
          total={profitCenters?.total || 0}
          totalPages={profitCenters?.total_pages || 1}
          onPageChange={profitPagination.goToPage}
          isLoading={profitLoading}
          emptyTitle={t("noProfitCentersFound")}
          emptyDescription={t("createProfitCenters")}
        />
      )}
    </div>
  );
}
