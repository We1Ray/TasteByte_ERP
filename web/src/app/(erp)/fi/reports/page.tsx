"use client";

import { useState, useMemo } from "react";
import { useTranslations } from "next-intl";
import { PageHeader } from "@/components/layout/page-header";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Tabs } from "@/components/ui/tabs";
import { ReportTable } from "@/components/ui/report-table";
import { ReportSkeleton } from "@/components/ui/report-skeleton";
import { KPIGrid } from "@/components/ui/kpi-grid";
import { ReportActionBar } from "@/components/shared/report-action-bar";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { fiApi } from "@/lib/api/fi";
import { formatCurrency } from "@/lib/utils";

type ReportTab = "trial-balance" | "income-statement" | "balance-sheet" | "ar-aging" | "ap-aging";


export default function FiReportsPage() {
  const t = useTranslations("fi");
  const tCommon = useTranslations("common");

  const tabs = useMemo<{ key: ReportTab; label: string }[]>(() => [
    { key: "trial-balance", label: t("trialBalance") },
    { key: "income-statement", label: t("incomeStatement") },
    { key: "balance-sheet", label: t("balanceSheet") },
    { key: "ar-aging", label: t("arAging") },
    { key: "ap-aging", label: t("apAging") },
  ], [t]);

  const [activeTab, setActiveTab] = useState<ReportTab>("trial-balance");
  const { today, thirtyDaysAgo } = useMemo(() => {
    const now = new Date();
    const past = new Date(now);
    past.setDate(past.getDate() - 30);
    return {
      today: now.toISOString().split("T")[0],
      thirtyDaysAgo: past.toISOString().split("T")[0],
    };
  }, []);
  const [asOfDate, setAsOfDate] = useState(today);
  const [startDate, setStartDate] = useState(thirtyDaysAgo);
  const [endDate, setEndDate] = useState(today);

  const { data: trialBalance, isLoading: tbLoading } = useApiQuery(
    ["fi", "trial-balance", asOfDate],
    () => fiApi.getTrialBalance({ as_of_date: asOfDate }),
    { enabled: activeTab === "trial-balance" }
  );

  const { data: incomeStatement, isLoading: isLoading } = useApiQuery(
    ["fi", "income-statement", startDate, endDate],
    () => fiApi.getIncomeStatement({ start_date: startDate, end_date: endDate }),
    { enabled: activeTab === "income-statement" }
  );

  const { data: balanceSheet, isLoading: bsLoading } = useApiQuery(
    ["fi", "balance-sheet", asOfDate],
    () => fiApi.getBalanceSheet({ as_of_date: asOfDate }),
    { enabled: activeTab === "balance-sheet" }
  );

  const { data: arAging, isLoading: arLoading } = useApiQuery(
    ["fi", "ar-aging"],
    () => fiApi.getArAging(),
    { enabled: activeTab === "ar-aging" }
  );

  const { data: apAging, isLoading: apLoading } = useApiQuery(
    ["fi", "ap-aging"],
    () => fiApi.getApAging(),
    { enabled: activeTab === "ap-aging" }
  );

  const loading =
    (activeTab === "trial-balance" && tbLoading) ||
    (activeTab === "income-statement" && isLoading) ||
    (activeTab === "balance-sheet" && bsLoading) ||
    (activeTab === "ar-aging" && arLoading) ||
    (activeTab === "ap-aging" && apLoading);

  const exportData = useMemo(() => {
    if (activeTab === "trial-balance") return trialBalance?.accounts ?? [];
    if (activeTab === "income-statement") return [...(incomeStatement?.revenue ?? []), ...(incomeStatement?.expenses ?? [])];
    if (activeTab === "balance-sheet") return [...(balanceSheet?.assets ?? []), ...(balanceSheet?.liabilities ?? []), ...(balanceSheet?.equity ?? [])];
    if (activeTab === "ar-aging") return arAging?.entries ?? [];
    return apAging?.entries ?? [];
  }, [activeTab, trialBalance, incomeStatement, balanceSheet, arAging, apAging]);

  const sectionLabels: Record<string, string> = useMemo(() => ({
    assets: t("totalAssets"),
    liabilities: t("totalLiabilities"),
    equity: t("totalEquity"),
  }), [t]);

  const sectionTitles: Record<string, string> = useMemo(() => ({
    assets: t("totalAssets").replace(/^Total\s*/i, ""),
    liabilities: t("totalLiabilities").replace(/^Total\s*/i, ""),
    equity: t("totalEquity").replace(/^Total\s*/i, ""),
  }), [t]);

  const agingColumns = [
    { header: t("document"), accessor: "document_number" as const, className: "font-medium text-gray-900" },
    { header: activeTab === "ar-aging" ? t("customer") : t("vendor"), accessor: "party_name" as const },
    { header: t("dueDate"), accessor: "due_date" as const, className: "text-gray-500" },
    { header: tCommon("amount"), accessor: (entry: { amount: number }) => formatCurrency(entry.amount), align: "right" as const, className: "font-mono" },
    { header: t("daysOverdue"), accessor: "days_overdue" as const, align: "right" as const },
    {
      header: t("bucket"),
      accessor: (entry: { days_overdue: number; aging_bucket: string }) => (
        <Badge color={entry.days_overdue > 60 ? "red" : entry.days_overdue > 30 ? "amber" : "blue"}>
          {entry.aging_bucket}
        </Badge>
      ),
    },
  ];

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
        exportFilename={`fi-${activeTab}`}
        dateFilter={
          activeTab === "trial-balance" || activeTab === "balance-sheet"
            ? { type: "single" as const, label: t("asOfDate"), value: asOfDate, onChange: setAsOfDate }
            : activeTab === "income-statement"
            ? { type: "range" as const, startDate, endDate, onStartDateChange: setStartDate, onEndDateChange: setEndDate }
            : undefined
        }
      />

      {loading && <ReportSkeleton />}

      {/* Trial Balance */}
      {activeTab === "trial-balance" && !tbLoading && (
        <Card>
          <CardHeader>
            <CardTitle>{t("trialBalance")}</CardTitle>
          </CardHeader>
          <ReportTable
            columns={[
              { header: t("accountNo"), accessor: "account_number" as const, className: "font-medium text-gray-900" },
              { header: t("accountName"), accessor: "account_name" as const },
              { header: tCommon("type"), accessor: "account_type" as const, className: "text-gray-500" },
              {
                header: t("debit"),
                align: "right" as const,
                className: "font-mono",
                accessor: (entry) =>
                  entry.debit_balance > 0 ? formatCurrency(entry.debit_balance) : "-",
              },
              {
                header: t("credit"),
                align: "right" as const,
                className: "font-mono",
                accessor: (entry) =>
                  entry.credit_balance > 0 ? formatCurrency(entry.credit_balance) : "-",
              },
            ]}
            data={trialBalance?.accounts}
            keyExtractor={(entry) => entry.account_id}
            footer={trialBalance ? [
              { label: tCommon("total"), value: "", colSpan: 3 },
              { label: "", value: formatCurrency(trialBalance.total_debit), colSpan: 1 },
              { label: "", value: formatCurrency(trialBalance.total_credit), colSpan: 1 },
            ] : undefined}
          />
        </Card>
      )}

      {/* Income Statement */}
      {activeTab === "income-statement" && !isLoading && (
        <div className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle>{t("revenue")}</CardTitle>
            </CardHeader>
            <ReportTable
              columns={[
                { header: t("accountNo"), accessor: "account_number" as const, className: "font-medium text-gray-900" },
                { header: t("accountName"), accessor: "account_name" as const },
                { header: tCommon("amount"), accessor: (item) => formatCurrency(item.amount), align: "right" as const, className: "font-mono text-green-700" },
              ]}
              data={incomeStatement?.revenue}
              keyExtractor={(item) => item.account_number}
              footer={incomeStatement ? [
                { label: t("totalRevenue"), value: "", colSpan: 2 },
                { label: "", value: <span className="text-green-700">{formatCurrency(incomeStatement.total_revenue)}</span>, colSpan: 1 },
              ] : undefined}
            />
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>{t("expenses")}</CardTitle>
            </CardHeader>
            <ReportTable
              columns={[
                { header: t("accountNo"), accessor: "account_number" as const, className: "font-medium text-gray-900" },
                { header: t("accountName"), accessor: "account_name" as const },
                { header: tCommon("amount"), accessor: (item) => formatCurrency(item.amount), align: "right" as const, className: "font-mono text-red-700" },
              ]}
              data={incomeStatement?.expenses}
              keyExtractor={(item) => item.account_number}
              footer={incomeStatement ? [
                { label: t("totalExpenses"), value: "", colSpan: 2 },
                { label: "", value: <span className="text-red-700">{formatCurrency(incomeStatement.total_expenses)}</span>, colSpan: 1 },
              ] : undefined}
            />
          </Card>

          {incomeStatement && (
            <Card>
              <div className="flex items-center justify-between">
                <span className="text-lg font-semibold text-gray-900">{t("netIncome")}</span>
                <span className={`text-2xl font-bold font-mono ${incomeStatement.net_income >= 0 ? "text-green-700" : "text-red-700"}`}>
                  {formatCurrency(incomeStatement.net_income)}
                </span>
              </div>
            </Card>
          )}
        </div>
      )}

      {/* Balance Sheet */}
      {activeTab === "balance-sheet" && !bsLoading && (
        <div className="space-y-6">
          {(["assets", "liabilities", "equity"] as const).map((section) => (
            <Card key={section}>
              <CardHeader>
                <CardTitle>{sectionTitles[section]}</CardTitle>
              </CardHeader>
              <ReportTable
                columns={[
                  { header: t("accountNo"), accessor: "account_number" as const, className: "font-medium text-gray-900" },
                  { header: t("accountName"), accessor: "account_name" as const },
                  { header: tCommon("amount"), accessor: (item) => formatCurrency(item.amount), align: "right" as const, className: "font-mono" },
                ]}
                data={balanceSheet?.[section]}
                keyExtractor={(item) => item.account_number}
                footer={balanceSheet ? [
                  { label: sectionLabels[section], value: "", colSpan: 2 },
                  {
                    label: "",
                    value: formatCurrency(
                      section === "assets"
                        ? balanceSheet.total_assets
                        : section === "liabilities"
                        ? balanceSheet.total_liabilities
                        : balanceSheet.total_equity
                    ),
                    colSpan: 1,
                  },
                ] : undefined}
              />
            </Card>
          ))}
        </div>
      )}

      {/* AR Aging */}
      {activeTab === "ar-aging" && !arLoading && (
        <div className="space-y-6">
          {arAging?.buckets && arAging.buckets.length > 0 && (
            <KPIGrid
              items={arAging.buckets.map((b) => ({
                label: b.bucket,
                value: b.amount,
                format: "currency" as const,
              }))}
            />
          )}
          <Card>
            <CardHeader>
              <CardTitle>{t("arAgingTitle")}</CardTitle>
            </CardHeader>
            <ReportTable
              columns={agingColumns}
              data={arAging?.entries}
              keyExtractor={(entry) => entry.id}
              footer={arAging ? [
                { label: tCommon("total"), value: "", colSpan: 3 },
                { label: "", value: formatCurrency(arAging.total_amount), colSpan: 1 },
                { label: "", value: "", colSpan: 2 },
              ] : undefined}
            />
          </Card>
        </div>
      )}

      {/* AP Aging */}
      {activeTab === "ap-aging" && !apLoading && (
        <div className="space-y-6">
          {apAging?.buckets && apAging.buckets.length > 0 && (
            <KPIGrid
              items={apAging.buckets.map((b) => ({
                label: b.bucket,
                value: b.amount,
                format: "currency" as const,
              }))}
            />
          )}
          <Card>
            <CardHeader>
              <CardTitle>{t("apAgingTitle")}</CardTitle>
            </CardHeader>
            <ReportTable
              columns={agingColumns}
              data={apAging?.entries}
              keyExtractor={(entry) => entry.id}
              footer={apAging ? [
                { label: tCommon("total"), value: "", colSpan: 3 },
                { label: "", value: formatCurrency(apAging.total_amount), colSpan: 1 },
                { label: "", value: "", colSpan: 2 },
              ] : undefined}
            />
          </Card>
        </div>
      )}
    </div>
  );
}
