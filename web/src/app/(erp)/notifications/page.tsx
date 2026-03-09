"use client";

import { useState } from "react";
import { Bell, Check, CheckCheck, Trash2 } from "lucide-react";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { usePagination } from "@/lib/hooks/use-pagination";
import {
  notificationsApi,
  type Notification,
  type NotificationListParams,
} from "@/lib/api/notifications";

const typeColors: Record<string, string> = {
  info: "bg-blue-100 text-blue-700",
  warning: "bg-yellow-100 text-yellow-700",
  success: "bg-green-100 text-green-700",
  error: "bg-red-100 text-red-700",
};

const moduleLabels: Record<string, string> = {
  SD: "Sales & Distribution",
  MM: "Materials Management",
  FI: "Financial Accounting",
  CO: "Controlling",
  PP: "Production Planning",
  HR: "Human Resources",
  WM: "Warehouse Management",
  QM: "Quality Management",
};

function formatDate(dateStr: string): string {
  const date = new Date(dateStr);
  return date.toLocaleString();
}

export default function NotificationsPage() {
  const { page, pageSize, goToPage } = usePagination();
  const [readFilter, setReadFilter] = useState<string>("");
  const [typeFilter, setTypeFilter] = useState<string>("");
  const [moduleFilter, setModuleFilter] = useState<string>("");
  const queryClient = useQueryClient();

  const params: NotificationListParams = {
    page,
    per_page: pageSize,
    ...(readFilter === "unread" ? { is_read: false } : {}),
    ...(readFilter === "read" ? { is_read: true } : {}),
    ...(typeFilter ? { notification_type: typeFilter } : {}),
    ...(moduleFilter ? { module: moduleFilter } : {}),
  };

  const { data, isLoading } = useQuery({
    queryKey: ["notifications", "list", page, readFilter, typeFilter, moduleFilter],
    queryFn: () => notificationsApi.getNotifications(params),
  });

  const markAsRead = useMutation({
    mutationFn: notificationsApi.markAsRead,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["notifications"] });
    },
  });

  const markAllAsRead = useMutation({
    mutationFn: notificationsApi.markAllAsRead,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["notifications"] });
    },
  });

  const deleteNotification = useMutation({
    mutationFn: notificationsApi.deleteNotification,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["notifications"] });
    },
  });

  const notifications: Notification[] = data?.items ?? [];
  const totalPages = data?.total_pages ?? 1;
  const total = data?.total ?? 0;

  return (
    <div>
      <PageHeader
        title="Notifications"
        description="View and manage your notifications"
        actions={
          <Button
            variant="secondary"
            onClick={() => markAllAsRead.mutate()}
            disabled={markAllAsRead.isPending}
          >
            <CheckCheck className="mr-2 h-4 w-4" />
            Mark all as read
          </Button>
        }
      />

      {/* Filters */}
      <div className="mb-4 flex flex-wrap gap-3">
        <select
          value={readFilter}
          onChange={(e) => { setReadFilter(e.target.value); goToPage(1); }}
          className="rounded-md border border-gray-300 bg-white px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
        >
          <option value="">All</option>
          <option value="unread">Unread</option>
          <option value="read">Read</option>
        </select>

        <select
          value={typeFilter}
          onChange={(e) => { setTypeFilter(e.target.value); goToPage(1); }}
          className="rounded-md border border-gray-300 bg-white px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
        >
          <option value="">All Types</option>
          <option value="info">Info</option>
          <option value="warning">Warning</option>
          <option value="success">Success</option>
          <option value="error">Error</option>
        </select>

        <select
          value={moduleFilter}
          onChange={(e) => { setModuleFilter(e.target.value); goToPage(1); }}
          className="rounded-md border border-gray-300 bg-white px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
        >
          <option value="">All Modules</option>
          <option value="SD">SD - Sales</option>
          <option value="MM">MM - Materials</option>
          <option value="FI">FI - Finance</option>
          <option value="CO">CO - Controlling</option>
          <option value="PP">PP - Production</option>
          <option value="HR">HR - Human Resources</option>
          <option value="WM">WM - Warehouse</option>
          <option value="QM">QM - Quality</option>
        </select>
      </div>

      {/* Notification list */}
      {isLoading ? (
        <div className="flex items-center justify-center py-12">
          <div className="h-8 w-8 animate-spin rounded-full border-4 border-blue-600 border-t-transparent" />
        </div>
      ) : notifications.length === 0 ? (
        <div className="rounded-lg border border-gray-200 bg-white py-16 text-center">
          <Bell className="mx-auto h-12 w-12 text-gray-300" />
          <h3 className="mt-4 text-sm font-semibold text-gray-900">
            No notifications
          </h3>
          <p className="mt-1 text-sm text-gray-500">
            You&apos;re all caught up!
          </p>
        </div>
      ) : (
        <div className="space-y-2">
          {notifications.map((n) => (
            <div
              key={n.id}
              className={`flex items-start gap-4 rounded-lg border bg-white p-4 ${
                !n.is_read ? "border-blue-200 bg-blue-50/30" : "border-gray-200"
              }`}
            >
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2">
                  <span
                    className={`inline-block rounded px-2 py-0.5 text-xs font-medium ${
                      typeColors[n.notification_type] ?? typeColors.info
                    }`}
                  >
                    {n.notification_type}
                  </span>
                  {n.module && (
                    <span className="text-xs text-gray-500">
                      {moduleLabels[n.module] ?? n.module}
                    </span>
                  )}
                  {!n.is_read && (
                    <span className="h-2 w-2 rounded-full bg-blue-500" />
                  )}
                  <span className="ml-auto text-xs text-gray-400">
                    {formatDate(n.created_at)}
                  </span>
                </div>
                <h4 className="mt-1 text-sm font-semibold text-gray-900">
                  {n.title}
                </h4>
                <p className="mt-0.5 text-sm text-gray-600">{n.message}</p>
              </div>
              <div className="flex shrink-0 gap-1">
                {!n.is_read && (
                  <button
                    onClick={() => markAsRead.mutate(n.id)}
                    className="rounded p-1.5 text-gray-400 hover:bg-gray-100 hover:text-blue-600"
                    title="Mark as read"
                  >
                    <Check className="h-4 w-4" />
                  </button>
                )}
                <button
                  onClick={() => deleteNotification.mutate(n.id)}
                  className="rounded p-1.5 text-gray-400 hover:bg-gray-100 hover:text-red-600"
                  title="Delete"
                >
                  <Trash2 className="h-4 w-4" />
                </button>
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Pagination */}
      {totalPages > 1 && (
        <div className="mt-4 flex items-center justify-between">
          <p className="text-sm text-gray-500">
            Showing {notifications.length} of {total} notifications
          </p>
          <div className="flex gap-2">
            <Button
              variant="secondary"
              size="sm"
              onClick={() => goToPage(page - 1)}
              disabled={page <= 1}
            >
              Previous
            </Button>
            <span className="flex items-center px-3 text-sm text-gray-700">
              Page {page} of {totalPages}
            </span>
            <Button
              variant="secondary"
              size="sm"
              onClick={() => goToPage(page + 1)}
              disabled={page >= totalPages}
            >
              Next
            </Button>
          </div>
        </div>
      )}
    </div>
  );
}
