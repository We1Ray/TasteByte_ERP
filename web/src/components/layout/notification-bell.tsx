"use client";

import { Bell, Check, CheckCheck, Trash2 } from "lucide-react";
import { useState } from "react";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { notificationsApi, type Notification } from "@/lib/api/notifications";
import Link from "next/link";

const typeColors: Record<string, string> = {
  info: "bg-blue-100 text-blue-700",
  warning: "bg-yellow-100 text-yellow-700",
  success: "bg-green-100 text-green-700",
  error: "bg-red-100 text-red-700",
};

function timeAgo(dateStr: string): string {
  const date = new Date(dateStr);
  const now = new Date();
  const seconds = Math.floor((now.getTime() - date.getTime()) / 1000);

  if (seconds < 60) return "just now";
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m ago`;
  if (seconds < 86400) return `${Math.floor(seconds / 3600)}h ago`;
  return `${Math.floor(seconds / 86400)}d ago`;
}

export function NotificationBell() {
  const [open, setOpen] = useState(false);
  const queryClient = useQueryClient();

  const { data: unreadData } = useQuery({
    queryKey: ["notifications", "unread-count"],
    queryFn: notificationsApi.getUnreadCount,
    refetchInterval: 30000,
  });

  const { data: notificationsData } = useQuery({
    queryKey: ["notifications", "recent"],
    queryFn: () => notificationsApi.getNotifications({ per_page: 10 }),
    enabled: open,
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

  const unreadCount = unreadData?.count ?? 0;
  const notifications: Notification[] = notificationsData?.items ?? [];

  return (
    <div className="relative">
      <button
        onClick={() => setOpen(!open)}
        className="relative rounded-md p-2 text-gray-500 hover:bg-gray-100"
      >
        <Bell className="h-5 w-5" />
        {unreadCount > 0 && (
          <span className="absolute -right-0.5 -top-0.5 flex h-4 min-w-4 items-center justify-center rounded-full bg-red-500 px-1 text-[10px] font-medium text-white">
            {unreadCount > 99 ? "99+" : unreadCount}
          </span>
        )}
      </button>

      {open && (
        <>
          <div className="fixed inset-0 z-40" onClick={() => setOpen(false)} />
          <div className="absolute right-0 top-full z-50 mt-1 w-80 rounded-md border border-gray-200 bg-white shadow-lg">
            <div className="flex items-center justify-between border-b px-4 py-2.5">
              <h3 className="text-sm font-semibold text-gray-900">
                Notifications
              </h3>
              {unreadCount > 0 && (
                <button
                  onClick={() => markAllAsRead.mutate()}
                  className="flex items-center gap-1 text-xs text-blue-600 hover:text-blue-800"
                >
                  <CheckCheck className="h-3.5 w-3.5" />
                  Mark all read
                </button>
              )}
            </div>

            <div className="max-h-80 overflow-y-auto">
              {notifications.length === 0 ? (
                <div className="px-4 py-8 text-center text-sm text-gray-500">
                  No notifications
                </div>
              ) : (
                notifications.map((n) => (
                  <div
                    key={n.id}
                    className={`flex items-start gap-3 border-b border-gray-100 px-4 py-3 last:border-0 ${
                      !n.is_read ? "bg-blue-50/50" : ""
                    }`}
                  >
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-2">
                        <span
                          className={`inline-block rounded px-1.5 py-0.5 text-[10px] font-medium ${
                            typeColors[n.notification_type] ?? typeColors.info
                          }`}
                        >
                          {n.notification_type}
                        </span>
                        {n.module && (
                          <span className="text-[10px] font-medium text-gray-400">
                            {n.module}
                          </span>
                        )}
                        <span className="ml-auto text-[10px] text-gray-400">
                          {timeAgo(n.created_at)}
                        </span>
                      </div>
                      <p className="mt-0.5 text-sm font-medium text-gray-900 truncate">
                        {n.title}
                      </p>
                      <p className="text-xs text-gray-500 truncate">
                        {n.message}
                      </p>
                    </div>
                    {!n.is_read && (
                      <button
                        onClick={() => markAsRead.mutate(n.id)}
                        className="mt-1 shrink-0 rounded p-1 text-gray-400 hover:bg-gray-100 hover:text-gray-600"
                        title="Mark as read"
                      >
                        <Check className="h-3.5 w-3.5" />
                      </button>
                    )}
                  </div>
                ))
              )}
            </div>

            <div className="border-t px-4 py-2">
              <Link
                href="/notifications"
                onClick={() => setOpen(false)}
                className="block text-center text-xs font-medium text-blue-600 hover:text-blue-800"
              >
                View all notifications
              </Link>
            </div>
          </div>
        </>
      )}
    </div>
  );
}
