"use client";

import { useTranslations } from "next-intl";
import {
  Activity,
  Database,
  Users,
  FileText,
  Clock,
  CheckCircle,
  AlertTriangle,
} from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { systemApi } from "@/lib/api/system";

function formatUptime(seconds: number): string {
  const d = Math.floor(seconds / 86400);
  const h = Math.floor((seconds % 86400) / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  if (d > 0) return `${d}d ${h}h ${m}m`;
  if (h > 0) return `${h}h ${m}m`;
  return `${m}m`;
}

const statItems = [
  { key: "total_users" as const, icon: Users, label: "totalUsers", color: "blue" },
  { key: "total_operations" as const, icon: FileText, label: "totalOperations", color: "indigo" },
  { key: "total_records" as const, icon: Database, label: "totalRecords", color: "purple" },
  { key: "pending_releases" as const, icon: Clock, label: "pendingReleases", color: "amber" },
  { key: "active_jobs" as const, icon: Activity, label: "activeJobs", color: "emerald" },
] as const;

const colorMap: Record<string, { bg: string; text: string }> = {
  blue: { bg: "bg-blue-100", text: "text-blue-600" },
  indigo: { bg: "bg-indigo-100", text: "text-indigo-600" },
  purple: { bg: "bg-purple-100", text: "text-purple-600" },
  amber: { bg: "bg-amber-100", text: "text-amber-600" },
  emerald: { bg: "bg-emerald-100", text: "text-emerald-600" },
};

export default function SystemMonitorPage() {
  const t = useTranslations("admin");

  const { data: health } = useApiQuery(
    ["system", "health"],
    () => systemApi.health(),
    { refetchInterval: 30000 }
  );

  return (
    <div>
      <PageHeader
        title={t("systemMonitor")}
        description={t("systemMonitorDesc")}
      />

      {/* Status Banner */}
      <div
        className={`mb-6 flex items-center gap-3 rounded-lg p-4 ${
          health?.status === "healthy"
            ? "bg-green-50 text-green-800"
            : "bg-yellow-50 text-yellow-800"
        }`}
      >
        {health?.status === "healthy" ? (
          <CheckCircle className="h-5 w-5 text-green-600" />
        ) : (
          <AlertTriangle className="h-5 w-5 text-yellow-600" />
        )}
        <div>
          <p className="font-medium">
            {t("systemStatus")}: {health?.status || "..."}
          </p>
          <p className="text-sm opacity-75">
            v{health?.version || "..."} · {t("uptime")}:{" "}
            {health ? formatUptime(health.uptime_seconds) : "..."}
          </p>
        </div>
      </div>

      {/* Stats Grid */}
      <div className="mb-6 grid gap-4 sm:grid-cols-2 lg:grid-cols-5">
        {statItems.map(({ key, icon: Icon, label, color }) => {
          const colors = colorMap[color];
          return (
            <Card key={key}>
              <div className="flex items-center gap-3 p-4">
                <div
                  className={`flex h-10 w-10 items-center justify-center rounded-lg ${colors.bg}`}
                >
                  <Icon className={`h-5 w-5 ${colors.text}`} />
                </div>
                <div>
                  <p className="text-2xl font-bold text-gray-900">
                    {health?.stats[key] ?? "\u2014"}
                  </p>
                  <p className="text-xs text-gray-500">{t(label)}</p>
                </div>
              </div>
            </Card>
          );
        })}
      </div>

      {/* Database Info */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Database className="h-4 w-4" />
            {t("databaseInfo")}
          </CardTitle>
        </CardHeader>
        <div className="grid gap-4 sm:grid-cols-3">
          <div className="rounded-md bg-gray-50 p-3">
            <p className="text-xs font-medium text-gray-500">
              {t("connectionStatus")}
            </p>
            <p
              className={`mt-1 text-sm font-medium ${
                health?.database.connected
                  ? "text-green-600"
                  : "text-red-600"
              }`}
            >
              {health?.database.connected
                ? t("connected")
                : t("disconnected")}
            </p>
          </div>
          <div className="rounded-md bg-gray-50 p-3">
            <p className="text-xs font-medium text-gray-500">
              {t("poolSize")}
            </p>
            <p className="mt-1 text-sm font-medium text-gray-900">
              {health?.database.pool_size ?? "\u2014"}
            </p>
          </div>
          <div className="rounded-md bg-gray-50 p-3">
            <p className="text-xs font-medium text-gray-500">
              {t("idleConnections")}
            </p>
            <p className="mt-1 text-sm font-medium text-gray-900">
              {health?.database.idle_connections ?? "\u2014"}
            </p>
          </div>
        </div>
      </Card>
    </div>
  );
}
