"use client";

import Link from "next/link";
import {
  DollarSign,
  ShoppingCart,
  Package,
  FileText,
  TrendingUp,
  Factory,
} from "lucide-react";
import { useTranslations } from "next-intl";
import { PageHeader } from "@/components/layout/page-header";
import { KpiCard } from "@/components/charts/kpi-card";
import { BarChartCard } from "@/components/charts/bar-chart";
import { LineChartCard } from "@/components/charts/line-chart";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { StatusBadge } from "@/components/ui/badge";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { dashboardApi } from "@/lib/api/dashboard";
import { sdApi } from "@/lib/api/sd";
import { formatCurrency, formatNumber } from "@/lib/utils";

export default function DashboardPage() {
  const t = useTranslations("dashboard");
  const tNav = useTranslations("nav");
  const tCommon = useTranslations("common");

  const { data: kpis } = useApiQuery(
    ["dashboard", "kpis"],
    () => dashboardApi.getKpis()
  );

  const { data: charts } = useApiQuery(
    ["dashboard", "charts"],
    () => dashboardApi.getCharts()
  );

  const { data: recentSO } = useApiQuery(
    ["sd", "sales-orders", "recent"],
    () => sdApi.getSalesOrders({ page: 1, page_size: 5 })
  );

  const revenueData = charts?.monthly_revenue ?? [];
  const orderData = charts?.monthly_orders ?? [];

  return (
    <div>
      <PageHeader
        title={t("title")}
        description={t("description")}
      />

      {/* KPI Cards */}
      <div className="mb-6 grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-4">
        <Link href="/sd/sales-orders">
          <KpiCard
            title={t("totalRevenue")}
            value={formatCurrency(kpis?.total_revenue ?? 0)}
            change={`${formatNumber(kpis?.total_order_count ?? 0)} ${t("orders")}`}
            changeType="positive"
            icon={DollarSign}
            iconColor="bg-green-100 text-green-600"
          />
        </Link>
        <Link href="/sd/sales-orders">
          <KpiCard
            title={t("salesOrders")}
            value={formatNumber(kpis?.total_order_count ?? 0)}
            icon={ShoppingCart}
            iconColor="bg-blue-100 text-blue-600"
          />
        </Link>
        <Link href="/mm/stock">
          <KpiCard
            title={t("inventoryQty")}
            value={formatNumber(kpis?.total_inventory_quantity ?? 0)}
            icon={Package}
            iconColor="bg-amber-100 text-amber-600"
          />
        </Link>
        <Link href="/pp/production-orders">
          <KpiCard
            title={t("pendingProduction")}
            value={formatNumber(kpis?.pending_production_orders ?? 0)}
            icon={Factory}
            iconColor="bg-purple-100 text-purple-600"
          />
        </Link>
      </div>

      {/* AR / AP summary */}
      <div className="mb-6 grid grid-cols-1 gap-4 sm:grid-cols-2">
        <Link href="/fi/reports">
          <KpiCard
            title={t("openAR")}
            value={formatCurrency(kpis?.open_ar_amount ?? 0)}
            icon={TrendingUp}
            iconColor="bg-blue-100 text-blue-600"
          />
        </Link>
        <Link href="/fi/reports">
          <KpiCard
            title={t("openAP")}
            value={formatCurrency(kpis?.open_ap_amount ?? 0)}
            icon={FileText}
            iconColor="bg-red-100 text-red-600"
          />
        </Link>
      </div>

      {/* Charts - real data */}
      <div className="mb-6 grid grid-cols-1 gap-6 lg:grid-cols-2">
        <BarChartCard
          title={t("revenueVsCosts")}
          data={revenueData}
          dataKey="revenue"
          secondaryDataKey="costs"
          xAxisKey="month"
          color="#2563eb"
          secondaryColor="#e5e7eb"
        />
        <LineChartCard
          title={t("orderTrends")}
          data={orderData}
          lines={[
            { dataKey: "orders", color: "#2563eb", name: "Orders" },
            { dataKey: "delivered", color: "#16a34a", name: "Delivered" },
          ]}
          xAxisKey="month"
        />
      </div>

      {/* Recent Activity */}
      <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle>{t("recentSalesOrders")}</CardTitle>
          </CardHeader>
          <div className="-mx-6 -mb-6">
            <table className="w-full text-sm">
              <thead className="border-b border-t bg-gray-50">
                <tr>
                  <th className="px-6 py-2 text-left text-xs font-semibold uppercase text-gray-500">{t("order")}</th>
                  <th className="px-6 py-2 text-left text-xs font-semibold uppercase text-gray-500">{t("customer")}</th>
                  <th className="px-6 py-2 text-right text-xs font-semibold uppercase text-gray-500">{t("amount")}</th>
                  <th className="px-6 py-2 text-left text-xs font-semibold uppercase text-gray-500">{tCommon("status")}</th>
                </tr>
              </thead>
              <tbody className="divide-y">
                {(recentSO?.items ?? []).map((order) => (
                  <tr key={order.id} className="hover:bg-gray-50">
                    <td className="px-6 py-3">
                      <Link href={`/sd/sales-orders/${order.id}`} className="font-medium text-blue-600 hover:underline">
                        {order.order_number}
                      </Link>
                    </td>
                    <td className="px-6 py-3 text-gray-700">{order.customer_name}</td>
                    <td className="px-6 py-3 text-right text-gray-700">{formatCurrency(order.total_amount)}</td>
                    <td className="px-6 py-3">
                      <StatusBadge status={order.status} />
                    </td>
                  </tr>
                ))}
                {(!recentSO?.items || recentSO.items.length === 0) && (
                  <tr>
                    <td colSpan={4} className="px-6 py-8 text-center text-gray-500">{t("noRecentOrders")}</td>
                  </tr>
                )}
              </tbody>
            </table>
          </div>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>{t("quickActions")}</CardTitle>
          </CardHeader>
          <div className="grid grid-cols-2 gap-3">
            {[
              { label: t("createSalesOrder"), href: "/sd/sales-orders/new", icon: ShoppingCart, color: "bg-blue-50 text-blue-700 hover:bg-blue-100" },
              { label: t("createPurchaseOrder"), href: "/mm/purchase-orders", icon: FileText, color: "bg-green-50 text-green-700 hover:bg-green-100" },
              { label: t("addMaterial"), href: "/mm/materials/new", icon: Package, color: "bg-amber-50 text-amber-700 hover:bg-amber-100" },
              { label: t("newJournalEntry"), href: "/fi/journal/new", icon: TrendingUp, color: "bg-purple-50 text-purple-700 hover:bg-purple-100" },
            ].map((action) => {
              const Icon = action.icon;
              return (
                <Link
                  key={action.href}
                  href={action.href}
                  className={`flex items-center gap-3 rounded-lg p-4 transition-colors ${action.color}`}
                >
                  <Icon className="h-5 w-5" />
                  <span className="text-sm font-medium">{action.label}</span>
                </Link>
              );
            })}
          </div>

          <div className="mt-6">
            <h4 className="mb-3 text-sm font-semibold text-gray-700">{t("moduleOverview")}</h4>
            <div className="space-y-2">
              {[
                { labelKey: "fi" as const, href: "/fi" },
                { labelKey: "mm" as const, href: "/mm" },
                { labelKey: "sd" as const, href: "/sd" },
                { labelKey: "pp" as const, href: "/pp" },
                { labelKey: "hr" as const, href: "/hr" },
              ].map((item) => (
                <Link key={item.labelKey} href={item.href} className="flex items-center justify-between rounded-md p-2 text-sm hover:bg-gray-50">
                  <span className="text-gray-600">{tNav(item.labelKey)}</span>
                  <div className="flex items-center gap-2">
                    <span className="h-2 w-2 rounded-full bg-green-500" />
                    <span className="text-gray-500">{tCommon("active")}</span>
                  </div>
                </Link>
              ))}
            </div>
          </div>
        </Card>
      </div>
    </div>
  );
}
