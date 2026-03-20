"use client";

import { useTranslations } from "next-intl";
import { Info, Server, Database, Globe, Shield } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { systemApi } from "@/lib/api/system";

const techStack = [
  { label: "Backend", value: "Rust / Axum", icon: Server },
  { label: "Frontend", value: "Next.js 15 / React 19", icon: Globe },
  { label: "Database", value: "PostgreSQL 17", icon: Database },
  { label: "Auth", value: "JWT + RBAC", icon: Shield },
  { label: "Mobile", value: "iOS (Swift) / Android (Flutter)", icon: Globe },
  { label: "AI", value: "Claude API Integration", icon: Info },
];

const erpModules = [
  { code: "FI", name: "Financial Accounting" },
  { code: "CO", name: "Controlling" },
  { code: "MM", name: "Materials Management" },
  { code: "SD", name: "Sales & Distribution" },
  { code: "PP", name: "Production Planning" },
  { code: "HR", name: "Human Resources" },
  { code: "WM", name: "Warehouse Management" },
  { code: "QM", name: "Quality Management" },
];

export default function AboutPage() {
  const t = useTranslations("admin");

  const { data: health } = useApiQuery(["system", "health"], () =>
    systemApi.health()
  );

  return (
    <div>
      <PageHeader
        title={t("aboutSystem")}
        description={t("aboutSystemDesc")}
      />

      <Card className="mb-6">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Info className="h-4 w-4" />
            TasteByte ERP
          </CardTitle>
        </CardHeader>
        <div className="space-y-3">
          <div className="grid gap-4 sm:grid-cols-2">
            <div className="rounded-md bg-gray-50 p-3">
              <p className="text-xs font-medium text-gray-500">
                {t("version")}
              </p>
              <p className="mt-1 font-mono text-sm font-medium text-gray-900">
                v{health?.version || "0.1.0"}
              </p>
            </div>
            <div className="rounded-md bg-gray-50 p-3">
              <p className="text-xs font-medium text-gray-500">
                {t("systemStatus")}
              </p>
              <p
                className={`mt-1 text-sm font-medium ${
                  health?.status === "healthy"
                    ? "text-green-600"
                    : "text-yellow-600"
                }`}
              >
                {health?.status || "..."}
              </p>
            </div>
          </div>
        </div>
      </Card>

      <Card className="mb-6">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Server className="h-4 w-4" />
            {t("techStack")}
          </CardTitle>
        </CardHeader>
        <div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
          {techStack.map(({ label, value, icon: Icon }) => (
            <div
              key={label}
              className="flex items-center gap-3 rounded-md border border-gray-100 p-3"
            >
              <Icon className="h-4 w-4 text-gray-400" />
              <div>
                <p className="text-xs text-gray-500">{label}</p>
                <p className="text-sm font-medium text-gray-900">{value}</p>
              </div>
            </div>
          ))}
        </div>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>{t("modules")}</CardTitle>
        </CardHeader>
        <div className="grid gap-2 sm:grid-cols-2 lg:grid-cols-4">
          {erpModules.map(({ code, name }) => (
            <div
              key={code}
              className="flex items-center gap-2 rounded-md bg-gray-50 px-3 py-2"
            >
              <span className="rounded bg-blue-100 px-1.5 py-0.5 text-xs font-bold text-blue-700">
                {code}
              </span>
              <span className="text-sm text-gray-700">{name}</span>
            </div>
          ))}
        </div>
      </Card>
    </div>
  );
}
