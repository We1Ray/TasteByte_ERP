"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import {
  LayoutDashboard,
  DollarSign,
  PieChart,
  Package,
  ShoppingCart,
  Factory,
  Users,
  Warehouse,
  ClipboardCheck,
  ChevronLeft,
  ChevronRight,
  Blocks,
  Code2,
  Settings,
} from "lucide-react";
import { useTranslations } from "next-intl";
import { useQueries } from "@tanstack/react-query";
import { cn } from "@/lib/utils";
import { useUiStore } from "@/lib/stores/ui-store";
import { usePlatformRole } from "@/lib/hooks/use-platform-role";
import { useAuthStore } from "@/lib/stores/auth-store";
import { moduleOpsApi } from "@/lib/api/lowcode";

interface SidebarChild {
  labelKey?: string;
  rawLabel?: string;
  href: string;
  isDynamic?: boolean;
}

interface ModuleItem {
  key: string;
  labelKey: string;
  shortLabel?: string;
  icon: React.ElementType;
  href: string;
  requiredRole?: "USER" | "DEVELOPER" | "PLATFORM_ADMIN";
  requiredPermission?: string;
  children?: SidebarChild[];
}

const modules: ModuleItem[] = [
  {
    key: "dashboard",
    labelKey: "dashboard",
    icon: LayoutDashboard,
    href: "/dashboard",
  },
  {
    key: "fi",
    labelKey: "fi",
    shortLabel: "FI",
    icon: DollarSign,
    href: "/fi",
    requiredPermission: "fi:read",
    children: [
      { labelKey: "chartOfAccounts", href: "/fi/accounts" },
      { labelKey: "journalEntries", href: "/fi/journal" },
      { labelKey: "reports", href: "/fi/reports" },
    ],
  },
  {
    key: "co",
    labelKey: "co",
    shortLabel: "CO",
    icon: PieChart,
    href: "/co",
    requiredPermission: "co:read",
  },
  {
    key: "mm",
    labelKey: "mm",
    shortLabel: "MM",
    icon: Package,
    href: "/mm",
    requiredPermission: "mm:read",
    children: [
      { labelKey: "materials", href: "/mm/materials" },
      { labelKey: "purchaseOrders", href: "/mm/purchase-orders" },
      { labelKey: "stockOverview", href: "/mm/stock" },
      { labelKey: "reports", href: "/mm/reports" },
    ],
  },
  {
    key: "sd",
    labelKey: "sd",
    shortLabel: "SD",
    icon: ShoppingCart,
    href: "/sd",
    requiredPermission: "sd:read",
    children: [
      { labelKey: "customers", href: "/sd/customers" },
      { labelKey: "salesOrders", href: "/sd/sales-orders" },
      { labelKey: "invoices", href: "/sd/invoices" },
      { labelKey: "reports", href: "/sd/reports" },
    ],
  },
  {
    key: "pp",
    labelKey: "pp",
    shortLabel: "PP",
    icon: Factory,
    href: "/pp",
    requiredPermission: "pp:read",
    children: [
      { labelKey: "billOfMaterials", href: "/pp/boms" },
      { labelKey: "productionOrders", href: "/pp/production-orders" },
    ],
  },
  {
    key: "hr",
    labelKey: "hr",
    shortLabel: "HR",
    icon: Users,
    href: "/hr",
    requiredPermission: "hr:read",
    children: [
      { labelKey: "employees", href: "/hr/employees" },
      { labelKey: "attendance", href: "/hr/attendance" },
    ],
  },
  {
    key: "wm",
    labelKey: "wm",
    shortLabel: "WM",
    icon: Warehouse,
    href: "/wm",
    requiredPermission: "wm:read",
  },
  {
    key: "qm",
    labelKey: "qm",
    shortLabel: "QM",
    icon: ClipboardCheck,
    href: "/qm",
    requiredPermission: "qm:read",
  },
  {
    key: "lowcode",
    labelKey: "lowcode",
    icon: Blocks,
    href: "/lowcode",
    requiredRole: "USER",
  },
  {
    key: "developer",
    labelKey: "developer",
    icon: Code2,
    href: "/developer",
    requiredRole: "DEVELOPER",
    children: [
      { labelKey: "developerOverview", href: "/developer" },
      { labelKey: "operations", href: "/developer/operations" },
      { labelKey: "feedback", href: "/developer/feedback" },
    ],
  },
  {
    key: "admin",
    labelKey: "admin",
    icon: Settings,
    href: "/admin",
    requiredRole: "PLATFORM_ADMIN",
    children: [
      { labelKey: "dashboard", href: "/admin" },
      { labelKey: "users", href: "/admin/users" },
      { labelKey: "roles", href: "/admin/roles" },
      { labelKey: "projects", href: "/admin/projects" },
      { labelKey: "releases", href: "/admin/releases" },
      { labelKey: "navigation", href: "/admin/navigation" },
      { labelKey: "permissions", href: "/admin/permissions" },
    ],
  },
];

export function ErpSidebar() {
  const pathname = usePathname();
  const { sidebarCollapsed, toggleSidebarCollapsed } = useUiStore();
  const { isAdmin, isDeveloper, isUser } = usePlatformRole();
  const user = useAuthStore((s) => s.user);
  const t = useTranslations("nav");

  const isErpAdmin = user?.roles?.includes("ADMIN") ?? false;
  const userPermissions = user?.permissions ?? [];

  const visibleModules = modules.filter((mod) => {
    // Platform role check
    if (mod.requiredRole) {
      switch (mod.requiredRole) {
        case "USER":
          if (!isUser) return false;
          break;
        case "DEVELOPER":
          if (!isDeveloper) return false;
          break;
        case "PLATFORM_ADMIN":
          if (!isAdmin) return false;
          break;
      }
    }
    // ERP permission check - ADMIN bypasses
    if (mod.requiredPermission) {
      if (!isErpAdmin && !userPermissions.includes(mod.requiredPermission)) {
        return false;
      }
    }
    return true;
  });

  const erpModuleKeys = visibleModules
    .filter((m) => m.requiredPermission)
    .map((m) => m.key.toUpperCase());

  const moduleOpsQueries = useQueries({
    queries: erpModuleKeys.map((mod) => ({
      queryKey: ["lowcode", "module-ops", mod],
      queryFn: () => moduleOpsApi.listByModule(mod),
      staleTime: 5 * 60 * 1000,
      enabled: erpModuleKeys.length > 0,
    })),
  });

  const mergedModules = visibleModules.map((mod) => {
    if (!mod.requiredPermission) return mod;
    const modKey = mod.key.toUpperCase();
    const queryIdx = erpModuleKeys.indexOf(modKey);
    if (queryIdx === -1) return mod;
    const opsData = moduleOpsQueries[queryIdx]?.data;
    if (!opsData || opsData.length === 0) return mod;

    const dynamicChildren: SidebarChild[] = opsData.map((op) => ({
      rawLabel: op.name,
      href: `/${mod.key}/ops/${op.operation_code}`,
      isDynamic: true,
    }));

    return {
      ...mod,
      children: [...(mod.children || []), ...dynamicChildren],
    };
  });

  return (
    <aside
      className={cn(
        "fixed left-0 top-0 z-40 flex h-screen flex-col border-r border-slate-700 bg-slate-900 text-white transition-all duration-200",
        sidebarCollapsed ? "w-16" : "w-64"
      )}
    >
      <div className="flex h-14 items-center justify-between border-b border-slate-700 px-4">
        {!sidebarCollapsed && (
          <Link href="/dashboard" className="flex items-center gap-2">
            <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-blue-600 text-sm font-bold">
              TB
            </div>
            <span className="text-lg font-semibold">TasteByte</span>
          </Link>
        )}
        <button
          onClick={toggleSidebarCollapsed}
          className={cn(
            "rounded-md p-1.5 text-slate-400 hover:bg-slate-800 hover:text-white",
            sidebarCollapsed && "mx-auto"
          )}
        >
          {sidebarCollapsed ? (
            <ChevronRight className="h-4 w-4" />
          ) : (
            <ChevronLeft className="h-4 w-4" />
          )}
        </button>
      </div>

      <nav className="flex-1 overflow-y-auto py-4">
        <ul className="space-y-1 px-2">
          {mergedModules.map((mod, index) => {
            const isActive =
              pathname === mod.href || pathname.startsWith(mod.href + "/");
            const Icon = mod.icon;

            // Add divider before platform section
            const prevMod = mergedModules[index - 1];
            const showDivider = mod.requiredRole && prevMod && !prevMod.requiredRole;
            const label = t(mod.labelKey);

            return (
              <li key={mod.key}>
                {showDivider && !sidebarCollapsed && (
                  <div className="my-2 border-t border-slate-700">
                    <p className="px-3 pb-1 pt-3 text-[10px] font-semibold uppercase tracking-wider text-slate-500">
                      {t("platform")}
                    </p>
                  </div>
                )}
                {showDivider && sidebarCollapsed && (
                  <div className="my-2 border-t border-slate-700" />
                )}
                <Link
                  href={mod.children ? mod.children[0].href : mod.href}
                  className={cn(
                    "flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium transition-colors",
                    isActive
                      ? "bg-blue-600/20 text-blue-400"
                      : "text-slate-300 hover:bg-slate-800 hover:text-white"
                  )}
                  title={sidebarCollapsed ? label : undefined}
                >
                  <Icon className="h-5 w-5 shrink-0" />
                  {!sidebarCollapsed && (
                    <span>{mod.shortLabel ? `${mod.shortLabel} - ${label}` : label}</span>
                  )}
                </Link>
                {!sidebarCollapsed && mod.children && (
                  <div
                    className={cn(
                      "overflow-hidden transition-all duration-200",
                      isActive ? "max-h-96 opacity-100" : "max-h-0 opacity-0"
                    )}
                  >
                    <ul className="ml-8 mt-1 space-y-1">
                      {mod.children.map((child) => {
                        const childActive = pathname === child.href;
                        const label = child.isDynamic
                          ? child.rawLabel
                          : t(child.labelKey!);
                        return (
                          <li key={child.href}>
                            <Link
                              href={child.href}
                              className={cn(
                                "block rounded-md px-3 py-1.5 text-sm transition-colors",
                                childActive
                                  ? "text-blue-400"
                                  : "text-slate-400 hover:text-white"
                              )}
                            >
                              {label}
                            </Link>
                          </li>
                        );
                      })}
                    </ul>
                  </div>
                )}
              </li>
            );
          })}
        </ul>
      </nav>

      <div className="border-t border-slate-700 p-4">
        {!sidebarCollapsed && (
          <p className="text-xs text-slate-500">TasteByte ERP v1.0</p>
        )}
      </div>
    </aside>
  );
}
