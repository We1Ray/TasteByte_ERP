"use client";

import { useState, useMemo } from "react";
import { useTranslations } from "next-intl";
import { Shield, Plus, Trash2, User } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { ConfirmDialog } from "@/components/ui/confirm-dialog";
import { Modal } from "@/components/ui/modal";
import { Select } from "@/components/ui/select";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { PageLoading } from "@/components/ui/loading";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { useToastMutation } from "@/lib/hooks/use-toast-mutation";
import { operationsApi, permissionsApi } from "@/lib/api/lowcode";
import { rbacApi } from "@/lib/api/rbac";
import type { OperationPermission } from "@/lib/types/lowcode";

export default function AdminPermissionsPage() {
  const t = useTranslations("admin");
  const tCommon = useTranslations("common");
  const [selectedOperationId, setSelectedOperationId] = useState("");
  const [deletePermissionId, setDeletePermissionId] = useState<string | null>(null);
  const [showAddModal, setShowAddModal] = useState(false);
  const [assignType, setAssignType] = useState<"role" | "user">("role");
  const [assignId, setAssignId] = useState("");
  const [newPermCrud, setNewPermCrud] = useState({
    can_create: false,
    can_read: true,
    can_update: false,
    can_delete: false,
  });
  const [operationSearch, setOperationSearch] = useState("");

  const { data: operations, isLoading: opsLoading } = useApiQuery(
    ["lowcode", "operations", "all"],
    () => operationsApi.list({ page_size: 100 })
  );

  const { data: permissions, isLoading: permsLoading } = useApiQuery(
    ["lowcode", "permissions", selectedOperationId],
    () => permissionsApi.getOperationPermissions(selectedOperationId),
    { enabled: !!selectedOperationId }
  );

  const { data: roles } = useApiQuery(
    ["auth", "roles"],
    () => rbacApi.listRoles()
  );

  const { data: users } = useApiQuery(
    ["auth", "users"],
    () => rbacApi.listUsersWithRoles({ per_page: 200 })
  );

  const createMutation = useToastMutation(
    (perm: {
      role_id?: string;
      user_id?: string;
      can_create: boolean;
      can_read: boolean;
      can_update: boolean;
      can_delete: boolean;
    }) => permissionsApi.createOperationPermission(selectedOperationId, perm),
    {
      successMessage: t("permissionCreated"),
      invalidateKeys: ["lowcode", "permissions"],
    }
  );

  const updateMutation = useToastMutation(
    ({ permissionId, data }: {
      permissionId: string;
      data: {
        can_create?: boolean;
        can_read?: boolean;
        can_update?: boolean;
        can_delete?: boolean;
      };
    }) => permissionsApi.updateOperationPermission(selectedOperationId, permissionId, data),
    {
      successMessage: t("permissionsUpdated"),
      invalidateKeys: ["lowcode", "permissions"],
    }
  );

  const deleteMutation = useToastMutation(
    (permissionId: string) =>
      permissionsApi.deleteOperationPermission(selectedOperationId, permissionId),
    {
      successMessage: t("permissionDeleted"),
      invalidateKeys: ["lowcode", "permissions"],
    }
  );

  if (opsLoading) return <PageLoading />;

  const operationOptions = (operations?.items ?? []).map((op) => ({
    value: op.id,
    label: `${op.code} - ${op.name}`,
  }));

  const filteredOperationOptions = useMemo(
    () =>
      operationSearch
        ? operationOptions.filter((op) =>
            op.label.toLowerCase().includes(operationSearch.toLowerCase())
          )
        : operationOptions,
    [operationOptions, operationSearch]
  );

  const permissionsList: OperationPermission[] = permissions ?? [];

  const togglePermission = (
    perm: OperationPermission,
    field: "can_create" | "can_read" | "can_update" | "can_delete",
    value: boolean
  ) => {
    updateMutation.mutate({
      permissionId: perm.id,
      data: { [field]: value },
    });
  };

  const handleDelete = (permissionId: string) => {
    setDeletePermissionId(permissionId);
  };

  return (
    <div>
      <PageHeader
        title={t("permissionsTitle")}
        description={t("managePermissions")}
      />

      <div className="mb-6 space-y-2">
        <Input
          placeholder={t("searchOperations")}
          value={operationSearch}
          onChange={(e) => setOperationSearch(e.target.value)}
        />
        <Select
          label={t("selectOperation")}
          value={selectedOperationId}
          onChange={(e) => setSelectedOperationId(e.target.value)}
          options={filteredOperationOptions}
          placeholder={t("chooseOperation")}
        />
      </div>

      {selectedOperationId && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Shield className="h-5 w-5" />
              {t("operationPermissions")}
            </CardTitle>
            <Button
              size="sm"
              onClick={() => {
                setShowAddModal(true);
                setAssignId("");
                setNewPermCrud({ can_create: false, can_read: true, can_update: false, can_delete: false });
              }}
              loading={createMutation.isPending}
            >
              <Plus className="h-4 w-4" />
              {t("addPermission")}
            </Button>
          </CardHeader>

          {permsLoading ? (
            <div className="py-8 text-center text-sm text-gray-500">{t("loadingPermissions")}</div>
          ) : permissionsList.length === 0 ? (
            <div className="py-8 text-center text-sm text-gray-500">
              {t("noPermissions")}
            </div>
          ) : (
            <div className="overflow-x-auto">
              <table className="w-full text-sm">
                <thead>
                  <tr className="border-b bg-gray-50">
                    <th className="px-4 py-2 text-left text-xs font-semibold uppercase text-gray-500">{t("roleUser")}</th>
                    <th className="px-4 py-2 text-center text-xs font-semibold uppercase text-gray-500">{t("permCreate")}</th>
                    <th className="px-4 py-2 text-center text-xs font-semibold uppercase text-gray-500">{t("permRead")}</th>
                    <th className="px-4 py-2 text-center text-xs font-semibold uppercase text-gray-500">{t("permUpdate")}</th>
                    <th className="px-4 py-2 text-center text-xs font-semibold uppercase text-gray-500">{t("permDelete")}</th>
                    <th className="px-4 py-2 text-center text-xs font-semibold uppercase text-gray-500">{tCommon("actions")}</th>
                  </tr>
                </thead>
                <tbody className="divide-y">
                  {permissionsList.map((perm) => (
                    <tr key={perm.id}>
                      <td className="px-4 py-3 font-medium text-gray-900">
                        {perm.role_id ? (
                          <span className="inline-flex items-center gap-1.5">
                            <Shield className="h-3.5 w-3.5 text-blue-500" />
                            <span className="text-sm">
                              {roles?.find((r) => r.id === perm.role_id)?.name ?? perm.role_id}
                            </span>
                          </span>
                        ) : perm.user_id ? (
                          <span className="inline-flex items-center gap-1.5">
                            <User className="h-3.5 w-3.5 text-green-500" />
                            <span className="text-sm">
                              {users?.items?.find((u) => u.id === perm.user_id)?.username ?? perm.user_id}
                            </span>
                          </span>
                        ) : (
                          <span className="text-xs text-gray-400">{t("noRoleUser")}</span>
                        )}
                      </td>
                      {(["can_create", "can_read", "can_update", "can_delete"] as const).map((field) => (
                        <td key={field} className="px-4 py-3 text-center">
                          <input
                            type="checkbox"
                            checked={perm[field] ?? false}
                            onChange={(e) => togglePermission(perm, field, e.target.checked)}
                            className="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                          />
                        </td>
                      ))}
                      <td className="px-4 py-3 text-center">
                        <button
                          onClick={() => handleDelete(perm.id)}
                          className="text-gray-400 hover:text-red-500"
                          title={t("deletePermission")}
                        >
                          <Trash2 className="h-4 w-4" />
                        </button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </Card>
      )}

      <ConfirmDialog
        open={!!deletePermissionId}
        onClose={() => setDeletePermissionId(null)}
        onConfirm={() => {
          if (deletePermissionId) {
            deleteMutation.mutate(deletePermissionId);
            setDeletePermissionId(null);
          }
        }}
        title={tCommon("delete")}
        message={t("deletePermissionConfirm")}
        confirmLabel={tCommon("delete")}
        variant="danger"
        loading={deleteMutation.isPending}
      />

      <Modal
        open={showAddModal}
        onClose={() => setShowAddModal(false)}
        title={t("addPermission")}
        footer={
          <>
            <Button variant="secondary" onClick={() => setShowAddModal(false)}>
              {tCommon("cancel")}
            </Button>
            <Button
              onClick={() => {
                createMutation.mutate({
                  ...(assignType === "role"
                    ? { role_id: assignId }
                    : { user_id: assignId }),
                  ...newPermCrud,
                });
                setShowAddModal(false);
              }}
              disabled={!assignId}
            >
              {tCommon("create")}
            </Button>
          </>
        }
      >
        <div className="space-y-4">
          <div>
            <label className="mb-1 block text-sm font-medium text-gray-700">
              {t("assignTo")}
            </label>
            <div className="flex gap-2">
              <button
                type="button"
                onClick={() => {
                  setAssignType("role");
                  setAssignId("");
                }}
                className={`rounded-md px-3 py-1.5 text-sm font-medium ${
                  assignType === "role"
                    ? "bg-blue-100 text-blue-700"
                    : "bg-gray-100 text-gray-600"
                }`}
              >
                {t("role")}
              </button>
              <button
                type="button"
                onClick={() => {
                  setAssignType("user");
                  setAssignId("");
                }}
                className={`rounded-md px-3 py-1.5 text-sm font-medium ${
                  assignType === "user"
                    ? "bg-blue-100 text-blue-700"
                    : "bg-gray-100 text-gray-600"
                }`}
              >
                {t("user")}
              </button>
            </div>
          </div>
          <Select
            label={assignType === "role" ? t("selectRole") : t("selectUser")}
            value={assignId}
            onChange={(e) => setAssignId(e.target.value)}
            options={
              assignType === "role"
                ? (roles ?? []).map((r) => ({ value: r.id, label: r.name }))
                : (users?.items ?? []).map((u) => ({
                    value: u.id,
                    label: `${u.username}${u.display_name ? ` (${u.display_name})` : ""}`,
                  }))
            }
            placeholder={
              assignType === "role"
                ? t("chooseRole")
                : t("chooseUser")
            }
          />
          <div>
            <label className="mb-1 block text-sm font-medium text-gray-700">
              {t("initialPermissions")}
            </label>
            <div className="grid grid-cols-2 gap-2">
              {(["can_create", "can_read", "can_update", "can_delete"] as const).map((field) => (
                <label key={field} className="flex items-center gap-2 text-sm text-gray-700">
                  <input
                    type="checkbox"
                    checked={newPermCrud[field]}
                    onChange={(e) =>
                      setNewPermCrud((prev) => ({ ...prev, [field]: e.target.checked }))
                    }
                    className="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                  />
                  {t(field === "can_create" ? "permCreate" : field === "can_read" ? "permRead" : field === "can_update" ? "permUpdate" : "permDelete")}
                </label>
              ))}
            </div>
          </div>
        </div>
      </Modal>
    </div>
  );
}
