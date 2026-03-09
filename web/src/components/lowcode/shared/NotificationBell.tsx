"use client";

import { useState, useEffect, useRef } from "react";
import { Bell } from "lucide-react";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useTranslations } from "next-intl";
import { notificationsApi } from "@/lib/api/lowcode";
import { cn } from "@/lib/utils";
import { formatDateTime } from "@/lib/utils";

export function NotificationBell() {
  const t = useTranslations("lowcode");
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);
  const queryClient = useQueryClient();

  const { data } = useQuery({
    queryKey: ["lowcode", "notifications", "unread"],
    queryFn: () => notificationsApi.list({ page_size: 10, unread_only: true }),
    refetchInterval: 30000,
  });

  const unreadCount = data?.total ?? 0;

  useEffect(() => {
    const handleClick = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) {
        setOpen(false);
      }
    };
    document.addEventListener("mousedown", handleClick);
    return () => document.removeEventListener("mousedown", handleClick);
  }, []);

  const handleMarkRead = async (id: string) => {
    await notificationsApi.markRead(id);
    queryClient.invalidateQueries({ queryKey: ["lowcode", "notifications"] });
  };

  const handleMarkAllRead = async () => {
    await notificationsApi.markAllRead();
    queryClient.invalidateQueries({ queryKey: ["lowcode", "notifications"] });
  };

  const typeColors = {
    info: "bg-blue-500",
    warning: "bg-amber-500",
    error: "bg-red-500",
    success: "bg-green-500",
  };

  return (
    <div className="relative" ref={ref}>
      <button
        onClick={() => setOpen(!open)}
        className="relative rounded-md p-2 text-gray-500 hover:bg-gray-100"
      >
        <Bell className="h-5 w-5" />
        {unreadCount > 0 && (
          <span className="absolute -right-0.5 -top-0.5 flex h-4 w-4 items-center justify-center rounded-full bg-red-500 text-[10px] font-bold text-white">
            {unreadCount > 9 ? "9+" : unreadCount}
          </span>
        )}
      </button>

      {open && (
        <div className="absolute right-0 top-full z-50 mt-1 w-80 rounded-md border border-gray-200 bg-white shadow-lg">
          <div className="flex items-center justify-between border-b px-4 py-3">
            <h4 className="text-sm font-semibold text-gray-900">{t("notifications")}</h4>
            {unreadCount > 0 && (
              <button
                onClick={handleMarkAllRead}
                className="text-xs text-blue-600 hover:text-blue-700"
              >
                {t("markAllRead")}
              </button>
            )}
          </div>

          <div className="max-h-72 overflow-y-auto">
            {(data?.items ?? []).length === 0 ? (
              <p className="px-4 py-8 text-center text-sm text-gray-500">{t("noNotifications")}</p>
            ) : (
              data?.items.map((n) => (
                <div
                  key={n.id}
                  className={cn(
                    "cursor-pointer border-b px-4 py-3 transition-colors hover:bg-gray-50",
                    !n.is_read && "bg-blue-50/50"
                  )}
                  onClick={() => handleMarkRead(n.id)}
                >
                  <div className="flex items-start gap-2">
                    <span className={cn("mt-1.5 h-2 w-2 shrink-0 rounded-full", typeColors[n.notification_type] || "bg-gray-400")} />
                    <div className="flex-1">
                      <p className="text-sm font-medium text-gray-900">{n.title}</p>
                      <p className="text-xs text-gray-500">{n.message}</p>
                      <p className="mt-1 text-xs text-gray-400">{formatDateTime(n.created_at)}</p>
                    </div>
                  </div>
                </div>
              ))
            )}
          </div>
        </div>
      )}
    </div>
  );
}
