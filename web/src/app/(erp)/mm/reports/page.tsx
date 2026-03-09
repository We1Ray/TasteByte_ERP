"use client";

import { useState, useMemo } from "react";
import { useTranslations } from "next-intl";
import { PageHeader } from "@/components/layout/page-header";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Badge } from "@/components/ui/badge";
import { Tabs } from "@/components/ui/tabs";
import { ReportTable } from "@/components/ui/report-table";
import { ReportSkeleton } from "@/components/ui/report-skeleton";
import { ReportActionBar } from "@/components/shared/report-action-bar";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { mmApi } from "@/lib/api/mm";
import { formatCurrency, formatNumber, formatDate } from "@/lib/utils";

type ReportTab = "stock-valuation" | "movement-summary" | "slow-moving";

export default function MmReportsPage() {
  const t = useTranslations("mm");
  const tCommon = useTranslations("common");
  const [activeTab, setActiveTab] = useState<ReportTab>("stock-valuation");
  const { today, thirtyDaysAgo } = useMemo(() => {
    const now = new Date();
    const past = new Date(now);
    past.setDate(past.getDate() - 30);
    return {
      today: now.toISOString().split("T")[0],
      thirtyDaysAgo: past.toISOString().split("T")[0],
    };
  }, []);
  const [startDate, setStartDate] = useState(thirtyDaysAgo);
  const [endDate, setEndDate] = useState(today);
  const [slowDays, setSlowDays] = useState(90);

  const tabs = useMemo<{ key: ReportTab; label: string }[]>(() => [
    { key: "stock-valuation", label: t("stockValuation") },
    { key: "movement-summary", label: t("movementSummary") },
    { key: "slow-moving", label: t("slowMovingItems") },
  ], [t]);

  const { data: stockValuation, isLoading: svLoading } = useApiQuery(
    ["mm", "stock-valuation"],
    () => mmApi.getStockValuation(),
    { enabled: activeTab === "stock-valuation" }
  );

  const { data: movementSummary, isLoading: msLoading } = useApiQuery(
    ["mm", "movement-summary", startDate, endDate],
    () => mmApi.getMovementSummary({ start_date: startDate, end_date: endDate }),
    { enabled: activeTab === "movement-summary" }
  );

  const { data: slowMoving, isLoading: smLoading } = useApiQuery(
    ["mm", "slow-moving", String(slowDays)],
    () => mmApi.getSlowMoving({ days: slowDays }),
    { enabled: activeTab === "slow-moving" }
  );

  const loading =
    (activeTab === "stock-valuation" && svLoading) ||
    (activeTab === "movement-summary" && msLoading) ||
    (activeTab === "slow-moving" && smLoading);

  const exportData = useMemo(() => {
    if (activeTab === "stock-valuation") return stockValuation?.items ?? [];
    if (activeTab === "movement-summary") return movementSummary?.movements ?? [];
    return slowMoving ?? [];
  }, [activeTab, stockValuation, movementSummary, slowMoving]);

  return (
    <div>
      <PageHeader
        title={t("reports")}
        description={t("reportsDesc")}
      />

      <Tabs
        tabs={tabs}
        activeTab={activeTab}
        onTabChange={(key) => setActiveTab(key as ReportTab)}
      />

      <ReportActionBar
        exportData={exportData}
        exportFilename={`mm-${activeTab}`}
        dateFilter={
          activeTab === "movement-summary"
            ? { type: "range" as const, startDate, endDate, onStartDateChange: setStartDate, onEndDateChange: setEndDate }
            : undefined
        }
      />

      {activeTab === "slow-moving" && (
        <div className="mb-6">
          <Input label={t("daysThreshold")} type="number" value={slowDays} onChange={(e) => setSlowDays(Number(e.target.value))} className="w-32" />
        </div>
      )}

      {loading && <ReportSkeleton />}

      {/* Stock Valuation */}
      {activeTab === "stock-valuation" && !svLoading && (
        <Card>
          <CardHeader>
            <CardTitle>{t("stockValuation")}</CardTitle>
            {stockValuation && (
              <span className="text-lg font-bold text-gray-900">
                {tCommon("total")}: {formatCurrency(stockValuation.total_value)}
              </span>
            )}
          </CardHeader>
          <ReportTable
            columns={[
              { header: t("materialNo"), accessor: "material_number", className: "font-medium text-gray-900" },
              { header: t("materialName"), accessor: "material_name" },
              { header: t("warehouse"), accessor: "warehouse_name", className: "text-gray-500" },
              { header: tCommon("quantity"), accessor: (item) => `${formatNumber(item.quantity)} ${item.unit}`, align: "right", className: "font-mono" },
              { header: tCommon("price"), accessor: (item) => formatCurrency(item.unit_price), align: "right", className: "font-mono" },
              { header: t("totalValue"), accessor: (item) => formatCurrency(item.total_value), align: "right", className: "font-mono font-medium" },
            ]}
            data={stockValuation?.items}
            keyExtractor={(_, i) => i}
            footer={stockValuation ? [
              { label: t("totalValue"), value: "", colSpan: 5 },
              { label: "", value: formatCurrency(stockValuation.total_value), colSpan: 1 },
            ] : undefined}
          />
        </Card>
      )}

      {/* Movement Summary */}
      {activeTab === "movement-summary" && !msLoading && (
        <Card>
          <CardHeader>
            <CardTitle>{t("movementSummary")}</CardTitle>
          </CardHeader>
          <ReportTable
            columns={[
              { header: t("materialNo"), accessor: "material_number", className: "font-medium text-gray-900" },
              { header: t("materialName"), accessor: "material_name" },
              {
                header: t("movementType"),
                accessor: (m) => (
                  <Badge color={m.movement_type === "RECEIPT" ? "green" : "amber"}>
                    {m.movement_type}
                  </Badge>
                ),
              },
              { header: tCommon("quantity"), accessor: (m) => formatNumber(m.total_quantity), align: "right", className: "font-mono" },
              { header: tCommon("unit"), accessor: "unit", className: "text-gray-500" },
            ]}
            data={movementSummary?.movements}
            keyExtractor={(_, i) => i}
          />
        </Card>
      )}

      {/* Slow Moving */}
      {activeTab === "slow-moving" && !smLoading && (
        <Card>
          <CardHeader>
            <CardTitle>{t("slowMovingItems")} ({slowDays}+)</CardTitle>
          </CardHeader>
          <ReportTable
            columns={[
              { header: t("materialNo"), accessor: "material_number", className: "font-medium text-gray-900" },
              { header: t("materialName"), accessor: "material_name" },
              { header: tCommon("quantity"), accessor: (item) => `${formatNumber(item.quantity)} ${item.unit}`, align: "right", className: "font-mono" },
              { header: t("value"), accessor: (item) => formatCurrency(item.value), align: "right", className: "font-mono" },
              {
                header: t("lastMovement"),
                accessor: (item) => item.last_movement_date ? formatDate(item.last_movement_date) : "-",
                className: "text-gray-500",
              },
              {
                header: t("daysIdle"),
                align: "right",
                accessor: (item) => (
                  <Badge color={item.days_since_movement > 180 ? "red" : "amber"}>
                    {item.days_since_movement}
                  </Badge>
                ),
              },
            ]}
            data={slowMoving}
            keyExtractor={(_, i) => i}
          />
        </Card>
      )}
    </div>
  );
}
