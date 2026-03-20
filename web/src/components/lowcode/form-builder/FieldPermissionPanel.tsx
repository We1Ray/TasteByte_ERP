"use client";

import { useState } from "react";
import { useTranslations } from "next-intl";
import { Shield, Plus, Trash2 } from "lucide-react";
import { Modal } from "@/components/ui/modal";
import { Button } from "@/components/ui/button";
import { Select } from "@/components/ui/select";
import { useApiQuery, useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { permissionsApi } from "@/lib/api/lowcode";
import { roleManagementApi } from "@/lib/api/lowcode";
import type { FieldPermission } from "@/lib/types/lowcode";

interface FieldPermissionPanelProps {
  fieldId: string;
  onClose: () => void;
}

export function FieldPermissionPanel({ fieldId, onClose }: FieldPermissionPanelProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const invalidate = useInvalidateQueries();

  const { data: permissions, isLoading } = useApiQuery(
    ["lowcode", "field-permissions", fieldId],
    () => permissionsApi.getFieldPermissions(fieldId),
    { enabled: !!fieldId }
  );

  const { data: rolesData } = useApiQuery(
    ["lowcode", "roles"],
    () => roleManagementApi.listUsersWithRoles()
  );

  // Extract unique roles from users data
  const roles: { id: string; role_name: string }[] = [];
  const seenRoles = new Set<string>();
  if (Array.isArray(rolesData?.items || rolesData)) {
    const items = rolesData?.items || rolesData || [];
    for (const user of items as any[]) {
      for (const role of (user.platform_roles || [])) {
        if (!seenRoles.has(role)) {
          seenRoles.add(role);
          roles.push({ id: role, role_name: role });
        }
      }
    }
  }
  // Add default roles
  for (const r of ["PLATFORM_ADMIN", "DEVELOPER", "USER"]) {
    if (!seenRoles.has(r)) {
      roles.push({ id: r, role_name: r });
    }
  }

  const createMutation = useApiMutation(
    (data: { role_id?: string; visibility?: string; is_editable?: boolean }) =>
      permissionsApi.createFieldPermission(fieldId, data),
    { onSuccess: () => invalidate(["lowcode", "field-permissions", fieldId]) }
  );

  const deleteMutation = useApiMutation(
    (permId: string) => permissionsApi.deleteFieldPermission(fieldId, permId),
    { onSuccess: () => invalidate(["lowcode", "field-permissions", fieldId]) }
  );

  const [newRole, setNewRole] = useState("");
  const [newVisibility, setNewVisibility] = useState("VISIBLE");
  const [newEditable, setNewEditable] = useState(true);

  const handleAdd = () => {
    if (!newRole) return;
    createMutation.mutateAsync({
      role_id: newRole,
      visibility: newVisibility,
      is_editable: newEditable,
    });
    setNewRole("");
  };

  return (
    <Modal
      open={true}
      onClose={onClose}
      title={t("fieldPermissions")}
      size="lg"
    >
      <div className="space-y-4">
        {/* Existing permissions table */}
        <div className="overflow-x-auto rounded-md border border-gray-200">
          <table className="min-w-full text-sm">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-4 py-2 text-left font-medium text-gray-600">{t("role")}</th>
                <th className="px-4 py-2 text-left font-medium text-gray-600">{t("visibility")}</th>
                <th className="px-4 py-2 text-left font-medium text-gray-600">{t("editable")}</th>
                <th className="px-4 py-2 text-right font-medium text-gray-600"></th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-100">
              {isLoading ? (
                <tr><td colSpan={4} className="px-4 py-4 text-center text-gray-400">{tCommon("loading")}</td></tr>
              ) : (permissions || []).length === 0 ? (
                <tr><td colSpan={4} className="px-4 py-4 text-center text-gray-400">{t("noPermissions")}</td></tr>
              ) : (
                (permissions || []).map((perm: FieldPermission) => (
                  <tr key={perm.id}>
                    <td className="px-4 py-2 font-mono text-xs">{perm.role_id || perm.user_id || "-"}</td>
                    <td className="px-4 py-2">
                      <span className={`inline-flex rounded-full px-2 py-0.5 text-xs font-medium ${
                        perm.visibility === "HIDDEN" ? "bg-red-100 text-red-700" :
                        perm.visibility === "MASKED" ? "bg-yellow-100 text-yellow-700" :
                        "bg-green-100 text-green-700"
                      }`}>
                        {perm.visibility}
                      </span>
                    </td>
                    <td className="px-4 py-2">{perm.is_editable ? "Yes" : "No"}</td>
                    <td className="px-4 py-2 text-right">
                      <button
                        onClick={() => deleteMutation.mutateAsync(perm.id)}
                        className="text-red-400 hover:text-red-600"
                      >
                        <Trash2 className="h-4 w-4" />
                      </button>
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>

        {/* Add new permission */}
        <div className="flex items-end gap-2 rounded-md bg-gray-50 p-3">
          <div className="flex-1">
            <Select
              label={t("role")}
              value={newRole}
              onChange={(e) => setNewRole(e.target.value)}
              options={[
                { value: "", label: t("selectRole") },
                ...roles.map(r => ({ value: r.role_name, label: r.role_name })),
              ]}
            />
          </div>
          <div className="flex-1">
            <Select
              label={t("visibility")}
              value={newVisibility}
              onChange={(e) => setNewVisibility(e.target.value)}
              options={[
                { value: "VISIBLE", label: t("visibilityVisible") },
                { value: "HIDDEN", label: t("visibilityHidden") },
                { value: "MASKED", label: t("visibilityMasked") },
              ]}
            />
          </div>
          <label className="flex items-center gap-2 pb-1">
            <input
              type="checkbox"
              checked={newEditable}
              onChange={(e) => setNewEditable(e.target.checked)}
              className="h-4 w-4 rounded border-gray-300 text-blue-600"
            />
            <span className="text-sm">{t("editable")}</span>
          </label>
          <Button size="sm" onClick={handleAdd} disabled={!newRole}>
            <Plus className="h-4 w-4" />
          </Button>
        </div>
      </div>
    </Modal>
  );
}
