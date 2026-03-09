"use client";

import { useEffect } from "react";
import { useRouter } from "next/navigation";
import { useAuthStore } from "@/lib/stores/auth-store";
import { useUiStore } from "@/lib/stores/ui-store";
import { ErpSidebar } from "@/components/layout/erp-sidebar";
import { ErpHeader } from "@/components/layout/erp-header";
import { cn } from "@/lib/utils";

export default function ErpLayout({ children }: { children: React.ReactNode }) {
  const router = useRouter();
  const { isAuthenticated, isLoading, hydrate } = useAuthStore();
  const { sidebarCollapsed } = useUiStore();

  useEffect(() => {
    hydrate();
  }, [hydrate]);

  useEffect(() => {
    if (!isLoading && !isAuthenticated) {
      router.replace("/login");
    }
  }, [isLoading, isAuthenticated, router]);

  if (isLoading) {
    return (
      <div className="flex min-h-screen items-center justify-center bg-gray-50">
        <div className="h-8 w-8 animate-spin rounded-full border-4 border-blue-600 border-t-transparent" />
      </div>
    );
  }

  if (!isAuthenticated) {
    return null;
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <ErpSidebar />
      <div
        className={cn(
          "transition-all duration-200",
          sidebarCollapsed ? "ml-16" : "ml-64"
        )}
      >
        <ErpHeader />
        <main className="p-6">{children}</main>
      </div>
    </div>
  );
}
