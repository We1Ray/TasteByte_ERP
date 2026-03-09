"use client";

import { useRouter } from "next/navigation";
import {
  FolderOpen,
  FileText,
  Rocket,
  MessageSquare,
  Plus,
  Settings,
  Shield,
  Navigation,
  ArrowRight,
} from "lucide-react";
import { useTranslations } from "next-intl";
import { PageHeader } from "@/components/layout/page-header";
import { KpiCard } from "@/components/charts/kpi-card";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { StatusBadge, Badge } from "@/components/ui/badge";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import {
  projectsApi,
  operationsApi,
  releasesApi,
  feedbackApi,
} from "@/lib/api/lowcode";
import { formatDateTime } from "@/lib/utils";
import type { Release, Feedback } from "@/lib/types/lowcode";

export default function AdminDashboardPage() {
  const router = useRouter();
  const t = useTranslations("admin");

  // KPI queries
  const { data: projects } = useApiQuery(
    ["lowcode", "projects", "count"],
    () => projectsApi.list({ page_size: 1 })
  );

  const { data: operations } = useApiQuery(
    ["lowcode", "operations", "count"],
    () => operationsApi.list({ page_size: 1 })
  );

  const { data: pendingReleases } = useApiQuery(
    ["lowcode", "releases", "pending-count"],
    () => releasesApi.list({ status: "submitted", page_size: 1 })
  );

  const { data: openFeedback } = useApiQuery(
    ["lowcode", "feedback", "open-count"],
    () => feedbackApi.list({ status: "open", page_size: 1 })
  );

  // Recent releases (5 latest)
  const { data: recentReleases } = useApiQuery(
    ["lowcode", "releases", "recent"],
    () => releasesApi.list({ page_size: 5 })
  );

  // Recent feedback (5 latest)
  const { data: recentFeedbackData } = useApiQuery(
    ["lowcode", "feedback", "recent"],
    () => feedbackApi.list({ page_size: 5 })
  );

  const quickActions = [
    {
      label: t("createProject"),
      icon: Plus,
      href: "/admin/projects",
      color: "text-blue-600 bg-blue-50",
    },
    {
      label: t("operations"),
      icon: FileText,
      href: "/developer/operations",
      color: "text-green-600 bg-green-50",
    },
    {
      label: t("roleManagement"),
      icon: Shield,
      href: "/admin/roles",
      color: "text-purple-600 bg-purple-50",
    },
    {
      label: t("navigation"),
      icon: Navigation,
      href: "/admin/navigation",
      color: "text-amber-600 bg-amber-50",
    },
  ];

  const priorityColors: Record<string, "red" | "amber" | "blue" | "gray"> = {
    critical: "red",
    high: "amber",
    medium: "blue",
    low: "gray",
  };

  return (
    <div>
      <PageHeader title={t("title")} description={t("description")} />

      {/* KPI Row - clickable cards */}
      <div className="mb-6 grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-4">
        <div
          className="cursor-pointer"
          onClick={() => router.push("/admin/projects")}
        >
          <KpiCard
            title={t("totalProjects")}
            value={String(projects?.total ?? 0)}
            icon={FolderOpen}
            iconColor="bg-blue-100 text-blue-600"
          />
        </div>
        <div
          className="cursor-pointer"
          onClick={() => router.push("/developer/operations")}
        >
          <KpiCard
            title={t("totalOperations")}
            value={String(operations?.total ?? 0)}
            icon={FileText}
            iconColor="bg-green-100 text-green-600"
          />
        </div>
        <div
          className="cursor-pointer"
          onClick={() => router.push("/admin/releases")}
        >
          <KpiCard
            title={t("pendingReleases")}
            value={String(pendingReleases?.total ?? 0)}
            icon={Rocket}
            iconColor="bg-amber-100 text-amber-600"
          />
        </div>
        <div
          className="cursor-pointer"
          onClick={() => router.push("/developer/feedback")}
        >
          <KpiCard
            title={t("openFeedback")}
            value={String(openFeedback?.total ?? 0)}
            icon={MessageSquare}
            iconColor="bg-purple-100 text-purple-600"
          />
        </div>
      </div>

      {/* Middle row: Recent Releases + Quick Actions */}
      <div className="mb-6 grid grid-cols-1 gap-6 lg:grid-cols-3">
        {/* Recent Releases (2/3 width) */}
        <Card className="lg:col-span-2">
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Rocket className="h-4 w-4" />
              {t("releaseApprovals")}
            </CardTitle>
            <button
              onClick={() => router.push("/admin/releases")}
              className="flex items-center gap-1 text-sm text-blue-600 hover:text-blue-700"
            >
              {t("viewAll")} <ArrowRight className="h-3 w-3" />
            </button>
          </CardHeader>
          <div className="space-y-3">
            {(recentReleases?.items ?? []).length === 0 ? (
              <p className="py-4 text-center text-sm text-gray-500">
                {t("noReleasesFound")}
              </p>
            ) : (
              (recentReleases?.items ?? []).map((release: Release) => (
                <div
                  key={release.id}
                  className="flex cursor-pointer items-center justify-between rounded border-b border-gray-100 px-2 py-1 pb-3 last:border-0 hover:bg-gray-50"
                  onClick={() => router.push("/admin/releases")}
                >
                  <div>
                    <p className="text-sm font-medium text-gray-900">
                      {release.title}
                    </p>
                    <p className="mt-0.5 text-xs text-gray-500">
                      v{release.version} &middot;{" "}
                      {formatDateTime(release.created_at)}
                    </p>
                  </div>
                  <StatusBadge status={release.status} />
                </div>
              ))
            )}
          </div>
        </Card>

        {/* Quick Actions (1/3 width) */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Settings className="h-4 w-4" />
              {t("quickActions")}
            </CardTitle>
          </CardHeader>
          <div className="space-y-2">
            {quickActions.map((action) => (
              <button
                key={action.href}
                onClick={() => router.push(action.href)}
                className="flex w-full items-center gap-3 rounded-lg px-3 py-2.5 text-left transition-colors hover:bg-gray-50"
              >
                <div className={`rounded-lg p-2 ${action.color}`}>
                  <action.icon className="h-4 w-4" />
                </div>
                <span className="text-sm font-medium text-gray-700">
                  {action.label}
                </span>
                <ArrowRight className="ml-auto h-3.5 w-3.5 text-gray-400" />
              </button>
            ))}
          </div>
        </Card>
      </div>

      {/* Bottom: Recent Feedback */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <MessageSquare className="h-4 w-4" />
            {t("recentFeedback")}
          </CardTitle>
          <button
            onClick={() => router.push("/developer/feedback")}
            className="flex items-center gap-1 text-sm text-blue-600 hover:text-blue-700"
          >
            {t("viewAll")} <ArrowRight className="h-3 w-3" />
          </button>
        </CardHeader>
        <div className="space-y-3">
          {(recentFeedbackData?.items ?? []).length === 0 ? (
            <p className="py-4 text-center text-sm text-gray-500">
              {t("noRecentFeedback")}
            </p>
          ) : (
            (recentFeedbackData?.items ?? []).map((fb: Feedback) => (
              <div
                key={fb.id}
                className="flex items-center justify-between border-b border-gray-100 pb-3 last:border-0"
              >
                <div>
                  <p className="text-sm font-medium text-gray-900">
                    {fb.title}
                  </p>
                  <p className="mt-0.5 text-xs text-gray-500">
                    {fb.feedback_type} &middot;{" "}
                    {formatDateTime(fb.created_at)}
                  </p>
                </div>
                <div className="flex items-center gap-2">
                  {fb.priority && (
                    <Badge color={priorityColors[fb.priority] || "gray"}>
                      {fb.priority}
                    </Badge>
                  )}
                  <StatusBadge status={fb.status} />
                </div>
              </div>
            ))
          )}
        </div>
      </Card>
    </div>
  );
}
