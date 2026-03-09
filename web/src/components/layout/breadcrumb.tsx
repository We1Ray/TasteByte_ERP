"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { ChevronRight, Home } from "lucide-react";
import { useTranslations } from "next-intl";

const segmentKeyMap: Record<string, string> = {
  dashboard: "dashboard",
  fi: "fi",
  co: "co",
  mm: "mm",
  sd: "sd",
  pp: "pp",
  hr: "hr",
  wm: "wm",
  qm: "qm",
  accounts: "accounts",
  journal: "journal",
  reports: "reports",
  materials: "materials",
  "purchase-orders": "purchaseOrders",
  stock: "stock",
  customers: "customers",
  "sales-orders": "salesOrders",
  invoices: "invoices",
  boms: "boms",
  "production-orders": "productionOrders",
  employees: "employees",
  attendance: "attendance",
  new: "new",
  lowcode: "lowcode",
  developer: "developer",
  admin: "admin",
  projects: "projects",
  releases: "releases",
  navigation: "navigation",
  permissions: "permissions",
  feedback: "feedback",
  operations: "operations",
  preview: "preview",
  settings: "settings",
};

export function Breadcrumb() {
  const pathname = usePathname();
  const t = useTranslations("layout");
  const segments = pathname.split("/").filter(Boolean);

  if (segments.length === 0) return null;

  const crumbs = segments.map((segment, index) => {
    const href = "/" + segments.slice(0, index + 1).join("/");
    const isLast = index === segments.length - 1;
    const key = segmentKeyMap[segment];
    const label = key ? t(key) : segment;

    return { href, label, isLast, segment };
  });

  return (
    <nav className="flex items-center gap-1.5 text-sm">
      <Link href="/dashboard" className="text-gray-400 hover:text-gray-600">
        <Home className="h-4 w-4" />
      </Link>
      {crumbs.map((crumb) => (
        <span key={crumb.href} className="flex items-center gap-1.5">
          <ChevronRight className="h-3.5 w-3.5 text-gray-300" />
          {crumb.isLast ? (
            <span className="font-medium text-gray-900">{crumb.label}</span>
          ) : (
            <Link href={crumb.href} className="text-gray-500 hover:text-gray-700">
              {crumb.label}
            </Link>
          )}
        </span>
      ))}
    </nav>
  );
}
