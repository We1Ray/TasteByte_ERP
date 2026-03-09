"use client";

import { useState, useMemo } from "react";
import { type ColumnDef } from "@tanstack/react-table";
import { Shield, ShieldCheck, UserCircle, Plus, X } from "lucide-react";
import { useTranslations } from "next-intl";
import { PageHeader } from "@/components/layout/page-header";
import { DataTable } from "@/components/ui/data-table";
import { Button } from "@/components/ui/button";
import { SearchBar } from "@/components/forms/search-bar";
import { useApiQuery, useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { roleManagementApi } from "@/lib/api/lowcode";
import { rbacApi, type UserWithRoles as RbacUserWithRoles } from "@/lib/api/rbac";
import { useToastMutation } from "@/lib/hooks/use-toast-mutation";
import type { UserWithRoles } from "@/lib/types/lowcode";

// --- Platform role styling ---
const platformRoleBadgeStyles: Record<string, string> = {
  PLATFORM_ADMIN: "bg-red-100 text-red-700 border-red-200",
  DEVELOPER: "bg-blue-100 text-blue-700 border-blue-200",
  USER: "bg-green-100 text-green-700 border-green-200",
};

const allPlatformRoles = ["PLATFORM_ADMIN", "DEVELOPER", "USER"];

// --- ERP role styling ---
const erpRoleBadgeColors: Record<string, string> = {
  ADMIN: "bg-red-100 text-red-700 border-red-200",
  FI_MANAGER: "bg-blue-100 text-blue-700 border-blue-200",
  CO_MANAGER: "bg-blue-100 text-blue-700 border-blue-200",
  MM_MANAGER: "bg-green-100 text-green-700 border-green-200",
  SD_MANAGER: "bg-purple-100 text-purple-700 border-purple-200",
  PP_MANAGER: "bg-amber-100 text-amber-700 border-amber-200",
  HR_MANAGER: "bg-pink-100 text-pink-700 border-pink-200",
  WM_MANAGER: "bg-teal-100 text-teal-700 border-teal-200",
  QM_MANAGER: "bg-indigo-100 text-indigo-700 border-indigo-200",
  OPERATOR: "bg-gray-100 text-gray-700 border-gray-200",
};

type ActiveTab = "erp" | "platform";

export default function AdminUsersPage() {
  const t = useTranslations("admin");
  const tCommon = useTranslations("common");
  const [search, setSearch] = useState("");
  const [activeTab, setActiveTab] = useState<ActiveTab>("erp");
  const [assigningUserId, setAssigningUserId] = useState<string | null>(null);
  const invalidate = useInvalidateQueries();

  const platformRoleLabels: Record<string, string> = useMemo(() => ({
    PLATFORM_ADMIN: t("platformAdmin"),
    DEVELOPER: t("platformDeveloper"),
    USER: t("platformUser"),
  }), [t]);

  // --- ERP Roles data ---
  const { data: erpUsersData, isLoading: erpLoading } = useApiQuery(
    ["auth", "users", search],
    () => rbacApi.listUsersWithRoles({ search: search || undefined, per_page: 100 }),
    { enabled: activeTab === "erp" }
  );
  const { data: allErpRoles } = useApiQuery(
    ["auth", "roles"],
    () => rbacApi.listRoles(),
    { enabled: activeTab === "erp" }
  );
  const erpAssignMutation = useToastMutation(
    ({ userId, roleId }: { userId: string; roleId: string }) =>
      rbacApi.assignUserRole(userId, roleId),
    {
      successMessage: t("erpRoleAssigned"),
      invalidateKeys: ["auth", "users"],
      onSuccess: () => setAssigningUserId(null),
    }
  );
  const erpRemoveMutation = useToastMutation(
    ({ userId, roleId }: { userId: string; roleId: string }) =>
      rbacApi.removeUserRole(userId, roleId),
    {
      successMessage: t("erpRoleRemoved"),
      invalidateKeys: ["auth", "users"],
    }
  );

  // --- Platform Roles data ---
  const { data: platformUsersData, isLoading: platformLoading } = useApiQuery(
    ["lowcode", "users", search],
    () => roleManagementApi.listUsersWithRoles({ search: search || undefined, per_page: 100 }),
    { enabled: activeTab === "platform" }
  );
  const platformAssignMutation = useApiMutation(
    ({ userId, roleName }: { userId: string; roleName: string }) =>
      roleManagementApi.assignRole(userId, roleName),
    {
      onSuccess: () => {
        invalidate(["lowcode", "users"]);
        setAssigningUserId(null);
      },
    }
  );
  const platformRevokeMutation = useApiMutation(
    ({ userId, roleName }: { userId: string; roleName: string }) =>
      roleManagementApi.revokeRole(userId, roleName),
    {
      onSuccess: () => invalidate(["lowcode", "users"]),
    }
  );

  // --- ERP columns ---
  const erpUsers: RbacUserWithRoles[] = erpUsersData?.items ?? [];
  const erpColumns: ColumnDef<RbacUserWithRoles, unknown>[] = useMemo(() => [
    {
      accessorKey: "username",
      header: tCommon("name"),
      cell: ({ row }) => (
        <div className="flex items-center gap-2">
          <UserCircle className="h-5 w-5 text-gray-400" />
          <div>
            <span className="font-medium">{row.original.username}</span>
            {row.original.display_name && (
              <span className="ml-2 text-xs text-gray-400">
                ({row.original.display_name})
              </span>
            )}
          </div>
        </div>
      ),
    },
    {
      accessorKey: "email",
      header: tCommon("email"),
      cell: ({ row }) => (
        <span className="text-gray-500">{row.original.email}</span>
      ),
    },
    {
      id: "roles",
      header: t("erpRoles"),
      cell: ({ row }) => {
        const userRoles = row.original.roles;
        return (
          <div className="flex flex-wrap gap-1">
            {userRoles.length === 0 ? (
              <span className="text-xs text-gray-400">{t("noRoles")}</span>
            ) : (
              userRoles.map((role) => (
                <span
                  key={role.id}
                  className={`inline-flex items-center gap-1 rounded-full border px-2 py-0.5 text-xs font-medium ${
                    erpRoleBadgeColors[role.name] ?? "bg-gray-100 text-gray-700 border-gray-200"
                  }`}
                >
                  <Shield className="h-3 w-3" />
                  {role.name}
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      erpRemoveMutation.mutate({ userId: row.original.id, roleId: role.id });
                    }}
                    className="ml-0.5 rounded-full p-0.5 hover:bg-black/10"
                    title={t("removeRole", { name: role.name })}
                  >
                    <X className="h-2.5 w-2.5" />
                  </button>
                </span>
              ))
            )}
          </div>
        );
      },
    },
    {
      id: "actions",
      header: tCommon("actions"),
      cell: ({ row }) => {
        const userRoleIds = new Set(row.original.roles.map((r) => r.id));
        const availableRoles = (allErpRoles ?? []).filter((r) => !userRoleIds.has(r.id));
        const isAssigning = assigningUserId === row.original.id;

        if (availableRoles.length === 0) {
          return <span className="text-xs text-gray-400">{t("allRolesAssigned")}</span>;
        }
        return (
          <div className="relative">
            <Button size="sm" variant="secondary" onClick={() => setAssigningUserId(isAssigning ? null : row.original.id)}>
              <Plus className="h-3.5 w-3.5" /> {t("addRole")}
            </Button>
            {isAssigning && (
              <>
                <div className="fixed inset-0 z-40" onClick={() => setAssigningUserId(null)} />
                <div className="absolute right-0 top-full z-50 mt-1 max-h-60 w-52 overflow-y-auto rounded-md border border-gray-200 bg-white py-1 shadow-lg">
                  {availableRoles.map((role) => (
                    <button
                      key={role.id}
                      onClick={() => erpAssignMutation.mutate({ userId: row.original.id, roleId: role.id })}
                      className="flex w-full items-center gap-2 px-3 py-2 text-sm text-gray-700 hover:bg-gray-100"
                    >
                      <Shield className="h-3.5 w-3.5 text-gray-400" />
                      {role.name}
                    </button>
                  ))}
                </div>
              </>
            )}
          </div>
        );
      },
    },
  ], [t, tCommon, allErpRoles, assigningUserId, erpAssignMutation, erpRemoveMutation, setAssigningUserId]);

  // --- Platform columns ---
  const platformUsers: UserWithRoles[] = platformUsersData?.items ?? [];
  const platformColumns: ColumnDef<UserWithRoles, unknown>[] = useMemo(() => [
    {
      accessorKey: "username",
      header: tCommon("name"),
      cell: ({ row }) => (
        <div className="flex items-center gap-2">
          <UserCircle className="h-5 w-5 text-gray-400" />
          <span className="font-medium">{row.original.username}</span>
        </div>
      ),
    },
    {
      accessorKey: "email",
      header: tCommon("email"),
      cell: ({ row }) => (
        <span className="text-gray-500">{row.original.email || "-"}</span>
      ),
    },
    {
      accessorKey: "platform_roles",
      header: t("platformRoles"),
      cell: ({ row }) => {
        const userRoles = row.original.platform_roles || [];
        return (
          <div className="flex flex-wrap gap-1">
            {userRoles.length === 0 ? (
              <span className="text-xs text-gray-400">{t("noRoles")}</span>
            ) : (
              userRoles.map((role) => (
                <span
                  key={role}
                  className={`inline-flex items-center gap-1 rounded-full border px-2 py-0.5 text-xs font-medium ${
                    platformRoleBadgeStyles[role] || "bg-gray-100 text-gray-700"
                  }`}
                >
                  {role === "PLATFORM_ADMIN" ? (
                    <ShieldCheck className="h-3 w-3" />
                  ) : role === "DEVELOPER" ? (
                    <Shield className="h-3 w-3" />
                  ) : null}
                  {platformRoleLabels[role] || role}
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      platformRevokeMutation.mutate({ userId: row.original.id, roleName: role });
                    }}
                    className="ml-0.5 rounded-full p-0.5 hover:bg-black/10"
                    title={t("removeRoleType", { name: platformRoleLabels[role] || role })}
                  >
                    <X className="h-2.5 w-2.5" />
                  </button>
                </span>
              ))
            )}
          </div>
        );
      },
    },
    {
      id: "actions",
      header: tCommon("actions"),
      cell: ({ row }) => {
        const userRoles = row.original.platform_roles || [];
        const availableRoles = allPlatformRoles.filter((r) => !userRoles.includes(r));
        const isAssigning = assigningUserId === row.original.id;

        if (availableRoles.length === 0) {
          return <span className="text-xs text-gray-400">{t("allRolesAssigned")}</span>;
        }
        return (
          <div className="relative">
            <Button size="sm" variant="secondary" onClick={() => setAssigningUserId(isAssigning ? null : row.original.id)}>
              <Plus className="h-3.5 w-3.5" /> {t("addRole")}
            </Button>
            {isAssigning && (
              <>
                <div className="fixed inset-0 z-40" onClick={() => setAssigningUserId(null)} />
                <div className="absolute right-0 top-full z-50 mt-1 w-44 rounded-md border border-gray-200 bg-white py-1 shadow-lg">
                  {availableRoles.map((role) => (
                    <button
                      key={role}
                      onClick={() => platformAssignMutation.mutate({ userId: row.original.id, roleName: role })}
                      className="flex w-full items-center gap-2 px-3 py-2 text-sm text-gray-700 hover:bg-gray-100"
                    >
                      <span className={`h-2 w-2 rounded-full ${
                        role === "PLATFORM_ADMIN" ? "bg-red-500" : role === "DEVELOPER" ? "bg-blue-500" : "bg-green-500"
                      }`} />
                      {platformRoleLabels[role]}
                    </button>
                  ))}
                </div>
              </>
            )}
          </div>
        );
      },
    },
  ], [t, tCommon, platformRoleLabels, assigningUserId, platformAssignMutation, platformRevokeMutation, setAssigningUserId]);

  return (
    <div>
      <PageHeader
        title={t("userManagement")}
        description={t("userManagementDesc")}
      />

      <div className="mb-4 flex items-center gap-4">
        <div className="flex rounded-lg border border-gray-200 bg-gray-50 p-1">
          <button
            onClick={() => { setActiveTab("erp"); setAssigningUserId(null); }}
            className={`rounded-md px-4 py-1.5 text-sm font-medium transition-colors ${
              activeTab === "erp"
                ? "bg-white text-gray-900 shadow-sm"
                : "text-gray-500 hover:text-gray-700"
            }`}
          >
            {t("erpRoles")}
          </button>
          <button
            onClick={() => { setActiveTab("platform"); setAssigningUserId(null); }}
            className={`rounded-md px-4 py-1.5 text-sm font-medium transition-colors ${
              activeTab === "platform"
                ? "bg-white text-gray-900 shadow-sm"
                : "text-gray-500 hover:text-gray-700"
            }`}
          >
            {t("platformRoles")}
          </button>
        </div>
        <div className="flex-1">
          <SearchBar placeholder={t("searchUsers")} onSearch={setSearch} />
        </div>
      </div>

      {activeTab === "erp" ? (
        <DataTable
          columns={erpColumns}
          data={erpUsers}
          isLoading={erpLoading}
          emptyTitle={t("noUsersFound")}
          emptyDescription={t("adjustSearch")}
        />
      ) : (
        <DataTable
          columns={platformColumns}
          data={platformUsers}
          isLoading={platformLoading}
          emptyTitle={t("noUsersFound")}
          emptyDescription={t("adjustSearch")}
        />
      )}
    </div>
  );
}
