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
import { KPIGrid } from "@/components/ui/kpi-grid";
import { ReportActionBar } from "@/components/shared/report-action-bar";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { sdApi } from "@/lib/api/sd";
import { formatCurrency, formatNumber } from "@/lib/utils";

type ReportTab = "sales-summary" | "order-fulfillment" | "top-customers";

export default function SdReportsPage() {
  const t = useTranslations("sd");
  const tc = useTranslations("common");
  const ts = useTranslations("shared");

  const tabs: { key: ReportTab; label: string }[] = useMemo(() => [
    { key: "sales-summary", label: "Sales Summary" },
    { key: "order-fulfillment", label: "Order Fulfillment" },
    { key: "top-customers", label: t("customers") },
  ], [t]);
  const [activeTab, setActiveTab] = useState<ReportTab>("sales-summary");
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
  const [topLimit, setTopLimit] = useState(10);

  const { data: salesSummary, isLoading: ssLoading } = useApiQuery(
    ["sd", "sales-summary", startDate, endDate],
    () => sdApi.getSalesSummary({ start_date: startDate, end_date: endDate }),
    { enabled: activeTab === "sales-summary" }
  );

  const { data: fulfillment, isLoading: ofLoading } = useApiQuery(
    ["sd", "order-fulfillment"],
    () => sdApi.getOrderFulfillment(),
    { enabled: activeTab === "order-fulfillment" }
  );

  const { data: topCustomers, isLoading: tcLoading } = useApiQuery(
    ["sd", "top-customers", String(topLimit)],
    () => sdApi.getTopCustomers({ limit: topLimit }),
    { enabled: activeTab === "top-customers" }
  );

  const loading =
    (activeTab === "sales-summary" && ssLoading) ||
    (activeTab === "order-fulfillment" && ofLoading) ||
    (activeTab === "top-customers" && tcLoading);

  const exportData = useMemo(() => {
    if (activeTab === "sales-summary") return (salesSummary?.items ?? []) as unknown as Record<string, unknown>[];
    if (activeTab === "order-fulfillment") return (fulfillment?.orders ?? []) as unknown as Record<string, unknown>[];
    return (topCustomers ?? []) as unknown as Record<string, unknown>[];
  }, [activeTab, salesSummary, fulfillment, topCustomers]);

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
        exportFilename={`sd-${activeTab}`}
        dateFilter={
          activeTab === "sales-summary"
            ? { type: "range" as const, startDate, endDate, onStartDateChange: setStartDate, onEndDateChange: setEndDate }
            : undefined
        }
      />

      {activeTab === "top-customers" && (
        <div className="mb-6">
          <Input label="Top N" type="number" value={topLimit} onChange={(e) => setTopLimit(Number(e.target.value))} className="w-24" />
        </div>
      )}

      {loading && <ReportSkeleton />}

      {/* Sales Summary */}
      {activeTab === "sales-summary" && !ssLoading && (
        <div className="space-y-6">
          {salesSummary && (
            <KPIGrid
              columns={3}
              items={[
                { label: "Total Revenue", value: salesSummary.total_revenue ?? 0, format: "currency" },
                { label: "Total Orders", value: salesSummary.total_orders ?? 0, format: "number" },
                { label: "Average Order Value", value: salesSummary.average_order_value ?? 0, format: "currency" },
              ]}
            />
          )}
          <Card>
            <CardHeader>
              <CardTitle>Daily Breakdown</CardTitle>
            </CardHeader>
            <ReportTable
              columns={[
                { header: tc("date"), accessor: "date", className: "font-medium text-gray-900" },
                { header: "Revenue", accessor: (item) => formatCurrency(item.revenue), align: "right", className: "font-mono" },
                { header: t("salesOrders"), accessor: "order_count", align: "right", className: "font-mono" },
              ]}
              data={salesSummary?.items}
              keyExtractor={(_, i) => i}
            />
          </Card>
        </div>
      )}

      {/* Order Fulfillment */}
      {activeTab === "order-fulfillment" && !ofLoading && (
        <div className="space-y-6">
          {fulfillment && (
            <KPIGrid
              items={[
                { label: "Total Orders", value: fulfillment.total_orders ?? 0, format: "number" },
                { label: "Fully Delivered", value: fulfillment.fully_delivered ?? 0, format: "number", color: "green" },
                { label: "Partially Delivered", value: fulfillment.partially_delivered ?? 0, format: "number", color: "amber" },
                { label: "Fulfillment Rate", value: fulfillment.fulfillment_rate ?? 0, format: "percentage", color: "blue" },
              ]}
            />
          )}
          <Card>
            <CardHeader>
              <CardTitle>Order Details</CardTitle>
            </CardHeader>
            <ReportTable
              columns={[
                { header: t("orderNo"), accessor: (o) => <span className="font-medium text-blue-600">{o.order_number}</span> },
                { header: t("customer"), accessor: "customer_name" },
                { header: "Items", accessor: "total_items", align: "right", className: "font-mono" },
                {
                  header: ts("delivered"),
                  align: "right",
                  accessor: (o) => (
                    <Badge color={o.delivered_items >= o.total_items ? "green" : "amber"}>
                      {o.delivered_items} / {o.total_items}
                    </Badge>
                  ),
                },
                { header: tc("status"), accessor: "status" },
              ]}
              data={fulfillment?.orders}
              keyExtractor={(_, i) => i}
            />
          </Card>
        </div>
      )}

      {/* Top Customers */}
      {activeTab === "top-customers" && !tcLoading && (
        <Card>
          <CardHeader>
            <CardTitle>Top {topLimit} Customers by Revenue</CardTitle>
          </CardHeader>
          <ReportTable
            columns={[
              { header: "#", accessor: (_, i) => i + 1, className: "font-medium text-gray-500" },
              { header: t("customerNo"), accessor: "customer_number", className: "font-medium text-gray-900" },
              { header: tc("name"), accessor: "customer_name" },
              { header: "Revenue", accessor: (c) => formatCurrency(c.total_revenue), align: "right", className: "font-mono font-medium text-gray-900" },
              { header: t("salesOrders"), accessor: "order_count", align: "right", className: "font-mono" },
            ]}
            data={topCustomers}
            keyExtractor={(c) => c.customer_id}
          />
        </Card>
      )}
    </div>
  );
}
