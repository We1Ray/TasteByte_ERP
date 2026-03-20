"use client";

import { useTranslations } from "next-intl";
import { BarChart3, Users, Activity } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { PageLoading } from "@/components/ui/loading";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { analyticsApi, type UsageSummary } from "@/lib/api/system";

export default function AnalyticsPage() {
  const t = useTranslations("admin");

  const { data: summary, isLoading } = useApiQuery(
    ["system", "analytics", "summary"],
    () => analyticsApi.getSummary(30),
    { refetchInterval: 60000 }
  );

  if (isLoading) return <PageLoading />;

  const totalEvents = (summary || []).reduce(
    (acc: number, s: UsageSummary) => acc + s.count,
    0
  );
  const totalUsers = (summary || []).reduce(
    (acc: number, s: UsageSummary) => acc + s.unique_users,
    0
  );

  return (
    <div>
      <PageHeader
        title={t("usageAnalytics")}
        description={t("usageAnalyticsDesc")}
      />

      <div className="mb-6 grid gap-4 sm:grid-cols-3">
        <Card>
          <div className="flex items-center gap-3 p-4">
            <Activity className="h-8 w-8 text-blue-500" />
            <div>
              <p className="text-2xl font-bold">{totalEvents}</p>
              <p className="text-xs text-gray-500">{t("totalEvents30d")}</p>
            </div>
          </div>
        </Card>
        <Card>
          <div className="flex items-center gap-3 p-4">
            <Users className="h-8 w-8 text-indigo-500" />
            <div>
              <p className="text-2xl font-bold">{totalUsers}</p>
              <p className="text-xs text-gray-500">{t("uniqueUsers30d")}</p>
            </div>
          </div>
        </Card>
        <Card>
          <div className="flex items-center gap-3 p-4">
            <BarChart3 className="h-8 w-8 text-emerald-500" />
            <div>
              <p className="text-2xl font-bold">
                {(summary || []).length}
              </p>
              <p className="text-xs text-gray-500">{t("eventTypes")}</p>
            </div>
          </div>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>{t("eventBreakdown")}</CardTitle>
        </CardHeader>
        <div className="overflow-x-auto">
          <table className="min-w-full text-sm">
            <thead className="border-b bg-gray-50">
              <tr>
                <th className="px-4 py-3 text-left font-medium text-gray-600">
                  {t("eventType")}
                </th>
                <th className="px-4 py-3 text-right font-medium text-gray-600">
                  {t("totalEvents")}
                </th>
                <th className="px-4 py-3 text-right font-medium text-gray-600">
                  {t("uniqueUsers")}
                </th>
                <th className="px-4 py-3 text-left font-medium text-gray-600">
                  {t("usageBar")}
                </th>
              </tr>
            </thead>
            <tbody className="divide-y">
              {(summary || []).map((s: UsageSummary) => {
                const maxCount = Math.max(
                  ...(summary || []).map((x: UsageSummary) => x.count),
                  1
                );
                const pct = Math.round((s.count / maxCount) * 100);
                return (
                  <tr key={s.event_type} className="hover:bg-gray-50">
                    <td className="px-4 py-3 font-medium text-gray-900">
                      {s.event_type}
                    </td>
                    <td className="px-4 py-3 text-right font-mono">
                      {s.count}
                    </td>
                    <td className="px-4 py-3 text-right font-mono">
                      {s.unique_users}
                    </td>
                    <td className="px-4 py-3">
                      <div className="h-2 w-full rounded-full bg-gray-100">
                        <div
                          className="h-2 rounded-full bg-blue-500"
                          style={{ width: `${pct}%` }}
                        />
                      </div>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      </Card>
    </div>
  );
}
