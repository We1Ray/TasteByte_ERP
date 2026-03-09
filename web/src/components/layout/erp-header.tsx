"use client";

import { LogOut, Menu, User } from "lucide-react";
import { useState } from "react";
import { useTranslations } from "next-intl";
import { useUiStore } from "@/lib/stores/ui-store";
import { useAuthStore } from "@/lib/stores/auth-store";
import { usePlatformRole } from "@/lib/hooks/use-platform-role";
import { Breadcrumb } from "./breadcrumb";
import { LocaleSwitcher } from "./locale-switcher";
import { NotificationBell } from "./notification-bell";

const roleBadgeStyles: Record<string, string> = {
  PLATFORM_ADMIN: "bg-red-100 text-red-700",
  DEVELOPER: "bg-blue-100 text-blue-700",
  USER: "bg-green-100 text-green-700",
  ADMIN: "bg-purple-100 text-purple-700",
};

const roleKeyMap: Record<string, string> = {
  PLATFORM_ADMIN: "rolePlatformAdmin",
  DEVELOPER: "roleDeveloper",
  USER: "roleUser",
  ADMIN: "roleAdmin",
};

function getHighestRole(roles: string[], isTraditionalAdmin: boolean): string | null {
  if (isTraditionalAdmin) return "ADMIN";
  if (roles.includes("PLATFORM_ADMIN")) return "PLATFORM_ADMIN";
  if (roles.includes("DEVELOPER")) return "DEVELOPER";
  if (roles.includes("USER")) return "USER";
  return null;
}

export function ErpHeader() {
  const { toggleSidebar } = useUiStore();
  const { user, logout } = useAuthStore();
  const { roles, isTraditionalAdmin } = usePlatformRole();
  const [showUserMenu, setShowUserMenu] = useState(false);
  const t = useTranslations("auth");
  const tLayout = useTranslations("layout");

  const highestRole = getHighestRole(roles, isTraditionalAdmin);

  return (
    <header className="sticky top-0 z-30 flex h-14 items-center justify-between border-b border-gray-200 bg-white px-4">
      <div className="flex items-center gap-4">
        <button
          onClick={toggleSidebar}
          className="rounded-md p-1.5 text-gray-500 hover:bg-gray-100 lg:hidden"
        >
          <Menu className="h-5 w-5" />
        </button>
        <Breadcrumb />
      </div>

      <div className="flex items-center gap-2">
        <LocaleSwitcher />
        <NotificationBell />

        <div className="relative">
          <button
            onClick={() => setShowUserMenu(!showUserMenu)}
            className="flex items-center gap-2 rounded-md px-3 py-1.5 text-sm text-gray-700 hover:bg-gray-100"
          >
            <div className="flex h-7 w-7 items-center justify-center rounded-full bg-blue-100 text-blue-700">
              <User className="h-4 w-4" />
            </div>
            <span className="hidden sm:inline">{user?.display_name || user?.username || "User"}</span>
            {highestRole && (
              <span
                className={`hidden text-[10px] font-medium px-1.5 py-0.5 rounded-full sm:inline ${
                  roleBadgeStyles[highestRole] || "bg-gray-100 text-gray-700"
                }`}
              >
                {roleKeyMap[highestRole] ? tLayout(roleKeyMap[highestRole]) : highestRole}
              </span>
            )}
          </button>

          {showUserMenu && (
            <>
              <div
                className="fixed inset-0 z-40"
                onClick={() => setShowUserMenu(false)}
              />
              <div className="absolute right-0 top-full z-50 mt-1 w-56 rounded-md border border-gray-200 bg-white py-1 shadow-lg">
                <div className="border-b px-4 py-2">
                  <p className="text-sm font-medium text-gray-900">
                    {user?.display_name || user?.username || "User"}
                  </p>
                  <p className="text-xs text-gray-500">{user?.email || ""}</p>
                  {roles.length > 0 && (
                    <div className="mt-1.5 flex flex-wrap gap-1">
                      {roles.map((role) => (
                        <span
                          key={role}
                          className={`text-[10px] font-medium px-1.5 py-0.5 rounded-full ${
                            roleBadgeStyles[role] || "bg-gray-100 text-gray-700"
                          }`}
                        >
                          {roleKeyMap[role] ? tLayout(roleKeyMap[role]) : role}
                        </span>
                      ))}
                    </div>
                  )}
                </div>
                <button
                  onClick={() => {
                    setShowUserMenu(false);
                    logout();
                    window.location.href = "/login";
                  }}
                  className="flex w-full items-center gap-2 px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
                >
                  <LogOut className="h-4 w-4" />
                  {t("signOut")}
                </button>
              </div>
            </>
          )}
        </div>
      </div>
    </header>
  );
}
