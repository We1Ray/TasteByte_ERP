"use client";

import { useState, useMemo } from "react";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { useTranslations } from "next-intl";
import {
  Shield,
  ShieldCheck,
  Plus,
  ChevronRight,
  ChevronDown,
  ChevronsDownUp,
  Trash2,
  Edit3,
  Lock,
} from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Modal } from "@/components/ui/modal";
import { ConfirmDialog } from "@/components/ui/confirm-dialog";
import { Loading } from "@/components/ui/loading";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { useToastMutation } from "@/lib/hooks/use-toast-mutation";
import { rbacApi, type Role } from "@/lib/api/rbac";

// Module label map
const moduleLabels: Record<string, string> = {
  fi: "FI",
  co: "CO",
  mm: "MM",
  sd: "SD",
  pp: "PP",
  hr: "HR",
  wm: "WM",
  qm: "QM",
};

const modules = ["fi", "co", "mm", "sd", "pp", "hr", "wm", "qm"];
const actions = ["read", "write"];

// Build a tree structure from flat role list
interface RoleNode extends Role {
  children: RoleNode[];
  depth: number;
}

function buildTree(roles: Role[]): RoleNode[] {
  const map = new Map<string, RoleNode>();
  const roots: RoleNode[] = [];

  roles.forEach((r) => map.set(r.id, { ...r, children: [], depth: 0 }));

  roles.forEach((r) => {
    const node = map.get(r.id)!;
    if (r.parent_id && map.has(r.parent_id)) {
      const parent = map.get(r.parent_id)!;
      node.depth = parent.depth + 1;
      parent.children.push(node);
    } else {
      roots.push(node);
    }
  });

  return roots;
}

function flattenTree(nodes: RoleNode[]): RoleNode[] {
  const result: RoleNode[] = [];
  function walk(list: RoleNode[]) {
    for (const node of list) {
      result.push(node);
      walk(node.children);
    }
  }
  walk(nodes);
  return result;
}

function getDescendantIds(roles: Role[], rootId: string): Set<string> {
  const ids = new Set<string>();
  function collect(parentId: string) {
    for (const r of roles) {
      if (r.parent_id === parentId && !ids.has(r.id)) {
        ids.add(r.id);
        collect(r.id);
      }
    }
  }
  collect(rootId);
  return ids;
}

export default function AdminRolesPage() {
  const t = useTranslations("admin");
  const tCommon = useTranslations("common");
  const [selectedRoleId, setSelectedRoleId] = useState<string | null>(null);
  const [expandedIds, setExpandedIds] = useState<Set<string>>(new Set());
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showEditModal, setShowEditModal] = useState(false);
  const [editingRole, setEditingRole] = useState<Role | null>(null);
  const [formName, setFormName] = useState("");
  const [formDescription, setFormDescription] = useState("");
  const [formParentId, setFormParentId] = useState<string>("");
  const [deleteTarget, setDeleteTarget] = useState<{ id: string; name: string } | null>(null);
  // Queries
  const { data: roles, isLoading: rolesLoading } = useApiQuery(
    ["auth", "roles"],
    () => rbacApi.listRoles()
  );

  const { data: allPermissions } = useApiQuery(
    ["auth", "permissions"],
    () => rbacApi.listPermissions()
  );

  const { data: roleDetail } = useApiQuery(
    ["auth", "roles", selectedRoleId ?? ""],
    () => rbacApi.getRole(selectedRoleId!),
    { enabled: !!selectedRoleId }
  );

  const { data: parentDetail } = useApiQuery(
    ["auth", "roles", roleDetail?.parent_id ?? ""],
    () => rbacApi.getRole(roleDetail!.parent_id!),
    { enabled: !!roleDetail?.parent_id }
  );

  // Mutations
  const createMutation = useToastMutation(
    (data: { name: string; description?: string; parent_id?: string | null }) =>
      rbacApi.createRole(data),
    {
      successMessage: t("roleCreated"),
      invalidateKeys: ["auth", "roles"],
      onSuccess: () => {
        setShowCreateModal(false);
        resetForm();
      },
    }
  );

  const updateMutation = useToastMutation(
    ({ id, data }: { id: string; data: { name?: string; description?: string | null; parent_id?: string | null } }) =>
      rbacApi.updateRole(id, data),
    {
      successMessage: t("roleUpdated"),
      invalidateKeys: ["auth", "roles"],
      onSuccess: () => {
        setShowEditModal(false);
        setEditingRole(null);
        resetForm();
      },
    }
  );

  const deleteMutation = useToastMutation(
    (id: string) => rbacApi.deleteRole(id),
    {
      successMessage: t("roleDeleted"),
      invalidateKeys: ["auth", "roles"],
      onSuccess: () => setSelectedRoleId(null),
    }
  );

  const setPermissionsMutation = useToastMutation(
    ({ roleId, permissionIds }: { roleId: string; permissionIds: string[] }) =>
      rbacApi.setRolePermissions(roleId, permissionIds),
    {
      successMessage: t("permissionsUpdated"),
      invalidateKeys: ["auth"],
    }
  );

  function resetForm() {
    setFormName("");
    setFormDescription("");
    setFormParentId("");
  }

  function openCreateModal() {
    resetForm();
    setShowCreateModal(true);
  }

  function openEditModal(role: Role) {
    setEditingRole(role);
    setFormName(role.name);
    setFormDescription(role.description ?? "");
    setFormParentId(role.parent_id ?? "");
    setShowEditModal(true);
  }

  function toggleExpand(id: string) {
    setExpandedIds((prev) => {
      const next = new Set(prev);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return next;
    });
  }

  // Permission toggle
  function handlePermissionToggle(module: string, action: string) {
    if (!selectedRoleId || !allPermissions || !roleDetail) return;

    const currentPermIds = roleDetail.permissions.map((p) => p.id);
    const targetPerm = allPermissions.find(
      (p) => p.module === module && p.action === action
    );
    if (!targetPerm) return;

    const hasIt = roleDetail.permissions.some(
      (p) => p.module === module && p.action === action
    );
    const newIds = hasIt
      ? currentPermIds.filter((id) => id !== targetPerm.id)
      : [...currentPermIds, targetPerm.id];

    setPermissionsMutation.mutate({
      roleId: selectedRoleId,
      permissionIds: newIds,
    });
  }

  const tree = roles ? buildTree(roles) : [];
  const flatRoles = flattenTree(tree);

  // For edit: exclude the editing role itself and its descendants to prevent cycles
  const editParentOptions = useMemo(() => {
    if (!editingRole) return roles ?? [];
    const descendants = getDescendantIds(roles ?? [], editingRole.id);
    return (roles ?? []).filter(
      (r) => r.id !== editingRole.id && !descendants.has(r.id)
    );
  }, [roles, editingRole]);

  // For create: all roles are available as parent
  const parentOptions = roles ?? [];

  if (rolesLoading) return <Loading />;

  return (
    <div>
      <PageHeader
        title={t("roleManagement")}
        description={t("roleManagementDesc")}
        actions={
          <Button onClick={openCreateModal}>
            <Plus className="mr-1.5 h-4 w-4" />
            {t("createRole")}
          </Button>
        }
      />

      <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
        {/* Left: Role Tree */}
        <div className="rounded-lg border border-gray-200 bg-white">
          <div className="flex items-center justify-between border-b px-4 py-3">
            <h3 className="text-sm font-semibold text-gray-700">{t("roleHierarchy")}</h3>
            <div className="flex items-center gap-1">
              <button
                onClick={() => {
                  const allIds = new Set((roles ?? []).filter(r => {
                    const node = flatRoles.find(n => n.id === r.id);
                    return node && node.children.length > 0;
                  }).map(r => r.id));
                  setExpandedIds(allIds);
                }}
                className="rounded px-2 py-1 text-xs text-gray-500 hover:bg-gray-100 hover:text-gray-700"
                title="Expand all"
              >
                <ChevronsDownUp className="h-3.5 w-3.5 inline mr-0.5 rotate-180" />
                {t("expand")}
              </button>
              <button
                onClick={() => setExpandedIds(new Set())}
                className="rounded px-2 py-1 text-xs text-gray-500 hover:bg-gray-100 hover:text-gray-700"
                title="Collapse all"
              >
                <ChevronsDownUp className="h-3.5 w-3.5 inline mr-0.5" />
                {t("collapse")}
              </button>
            </div>
          </div>
          <div className="divide-y">
            {flatRoles.map((role) => {
              const hasChildren = role.children.length > 0;
              const isExpanded = expandedIds.has(role.id);
              const isSelected = selectedRoleId === role.id;

              // Only show if all ancestors are expanded
              if (role.depth > 0) {
                let parent = roles?.find((r) => r.id === role.parent_id);
                let visible = true;
                while (parent) {
                  if (!expandedIds.has(parent.id)) {
                    visible = false;
                    break;
                  }
                  parent = roles?.find((r) => r.id === parent!.parent_id);
                }
                if (!visible) return null;
              }

              return (
                <div
                  key={role.id}
                  className={`flex items-center gap-2 px-4 py-2.5 cursor-pointer transition-colors ${
                    isSelected
                      ? "bg-blue-50 border-l-2 border-blue-500"
                      : "hover:bg-gray-50 border-l-2 border-transparent"
                  }`}
                  style={{ paddingLeft: `${role.depth * 24 + 16}px` }}
                  onClick={() => setSelectedRoleId(role.id)}
                >
                  {/* Expand/collapse */}
                  {hasChildren ? (
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        toggleExpand(role.id);
                      }}
                      className="rounded p-0.5 text-gray-400 hover:bg-gray-200"
                    >
                      {isExpanded ? (
                        <ChevronDown className="h-4 w-4" />
                      ) : (
                        <ChevronRight className="h-4 w-4" />
                      )}
                    </button>
                  ) : (
                    <span className="w-5" />
                  )}

                  {/* Icon */}
                  {role.name === "ADMIN" ? (
                    <ShieldCheck className="h-4 w-4 text-red-500" />
                  ) : role.is_system ? (
                    <Shield className="h-4 w-4 text-blue-500" />
                  ) : (
                    <Shield className="h-4 w-4 text-gray-400" />
                  )}

                  {/* Name + Description */}
                  <div className="min-w-0 flex-1">
                    <span className="text-sm font-medium text-gray-800">
                      {role.name}
                    </span>
                    {role.description && (
                      <p className="truncate text-xs text-gray-400">
                        {role.description}
                      </p>
                    )}
                  </div>

                  {/* System badge */}
                  {role.is_system && (
                    <Badge color="gray">
                      <Lock className="mr-0.5 h-3 w-3" />
                      {t("system")}
                    </Badge>
                  )}

                  {/* Actions */}
                  <div className="flex items-center gap-1">
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        openEditModal(role);
                      }}
                      className="rounded p-1 text-gray-400 hover:bg-gray-200 hover:text-gray-600"
                      title={t("editRole")}
                    >
                      <Edit3 className="h-3.5 w-3.5" />
                    </button>
                    {!role.is_system && (
                      <button
                        onClick={(e) => {
                          e.stopPropagation();
                          setDeleteTarget({ id: role.id, name: role.name });
                        }}
                        className="rounded p-1 text-gray-400 hover:bg-red-100 hover:text-red-600"
                        title={t("deleteRole")}
                      >
                        <Trash2 className="h-3.5 w-3.5" />
                      </button>
                    )}
                  </div>
                </div>
              );
            })}
          </div>
        </div>

        {/* Right: Permission Matrix */}
        <div className="rounded-lg border border-gray-200 bg-white">
          <div className="border-b px-4 py-3">
            <h3 className="text-sm font-semibold text-gray-700">
              {selectedRoleId && roleDetail
                ? t("permissionsFor", { name: roleDetail.name })
                : t("selectRoleToManage")}
            </h3>
          </div>

          {selectedRoleId && roleDetail ? (
            <div className="p-4">
              {roleDetail.description && (
                <p className="mb-4 text-sm text-gray-500">{roleDetail.description}</p>
              )}

              <div className="overflow-x-auto">
                <table className="w-full text-sm">
                  <thead>
                    <tr className="border-b">
                      <th className="px-3 py-2 text-left font-medium text-gray-600">
                        {t("module")}
                      </th>
                      {actions.map((a) => (
                        <th
                          key={a}
                          className="px-3 py-2 text-center font-medium text-gray-600 capitalize"
                        >
                          {a === "read" ? t("permRead") : t("permWrite")}
                        </th>
                      ))}
                    </tr>
                  </thead>
                  <tbody>
                    {modules.map((mod) => (
                      <tr key={mod} className="border-b last:border-0">
                        <td className="px-3 py-2.5 font-medium text-gray-800">
                          {moduleLabels[mod]}
                        </td>
                        {actions.map((action) => {
                          const hasPerm = roleDetail.permissions.some(
                            (p) => p.module === mod && p.action === action
                          );
                          const isInherited = !hasPerm && parentDetail?.permissions?.some(
                            (p) => p.module === mod && p.action === action
                          );
                          return (
                            <td key={action} className="px-3 py-2.5 text-center">
                              {isInherited ? (
                                <div className="flex items-center justify-center" title={t("inheritedFromParent")}>
                                  <input
                                    type="checkbox"
                                    checked
                                    disabled
                                    className="h-4 w-4 rounded border-gray-300 text-blue-300 opacity-50"
                                  />
                                </div>
                              ) : (
                                <input
                                  type="checkbox"
                                  checked={hasPerm}
                                  onChange={() => handlePermissionToggle(mod, action)}
                                  className="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                  disabled={roleDetail.name === "ADMIN"}
                                />
                              )}
                            </td>
                          );
                        })}
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>

              {roleDetail.name === "ADMIN" && (
                <p className="mt-3 text-xs text-gray-400">
                  {t("adminBypass")}
                </p>
              )}

              {roleDetail.parent_id && (
                <div className="mt-3 flex items-center gap-4 text-xs text-gray-400">
                  <span className="flex items-center gap-1">
                    <input type="checkbox" checked disabled className="h-3 w-3 rounded border-gray-300 text-blue-600" />
                    {t("directPermission")}
                  </span>
                  <span className="flex items-center gap-1">
                    <input type="checkbox" checked disabled className="h-3 w-3 rounded border-gray-300 text-blue-300 opacity-50" />
                    {t("inheritedPermission")}
                  </span>
                </div>
              )}
            </div>
          ) : (
            <div className="flex items-center justify-center p-12 text-gray-400">
              <p className="text-sm">{t("clickRoleToManage")}</p>
            </div>
          )}
        </div>
      </div>

      {/* Create Role Modal */}
      <Modal
        open={showCreateModal}
        onClose={() => setShowCreateModal(false)}
        title={t("createRole")}
        footer={
          <>
            <Button
              variant="secondary"
              onClick={() => setShowCreateModal(false)}
            >
              {tCommon("cancel")}
            </Button>
            <Button
              onClick={() =>
                createMutation.mutate({
                  name: formName,
                  description: formDescription || undefined,
                  parent_id: formParentId || null,
                })
              }
              disabled={!formName.trim() || createMutation.isPending}
            >
              {tCommon("create")}
            </Button>
          </>
        }
      >
        <div className="space-y-4">
          <Input
            label={t("roleName")}
            value={formName}
            onChange={(e) => setFormName(e.target.value)}
            placeholder={t("roleNamePlaceholder")}
          />
          <Input
            label={tCommon("description")}
            value={formDescription}
            onChange={(e) => setFormDescription(e.target.value)}
            placeholder={t("optionalDescription")}
          />
          <Select
            label={t("parentRole")}
            value={formParentId}
            onChange={(e) => setFormParentId(e.target.value)}
            options={[
              { value: "", label: t("noneRootLevel") },
              ...parentOptions.map((r) => ({ value: r.id, label: r.name })),
            ]}
          />
        </div>
      </Modal>

      {/* Delete Role Confirm */}
      <ConfirmDialog
        open={!!deleteTarget}
        onClose={() => setDeleteTarget(null)}
        onConfirm={() => {
          if (deleteTarget) {
            deleteMutation.mutate(deleteTarget.id);
            setDeleteTarget(null);
          }
        }}
        title={t("deleteRoleTitle")}
        message={t("deleteRoleConfirm", { name: deleteTarget?.name ?? "" })}
        confirmLabel={tCommon("delete")}
        variant="danger"
        loading={deleteMutation.isPending}
      />

      {/* Edit Role Modal */}
      <Modal
        open={showEditModal}
        onClose={() => {
          setShowEditModal(false);
          setEditingRole(null);
        }}
        title={t("editRoleTitle", { name: editingRole?.name ?? "" })}
        footer={
          <>
            <Button
              variant="secondary"
              onClick={() => {
                setShowEditModal(false);
                setEditingRole(null);
              }}
            >
              {tCommon("cancel")}
            </Button>
            <Button
              onClick={() => {
                if (!editingRole) return;
                updateMutation.mutate({
                  id: editingRole.id,
                  data: {
                    name: formName,
                    description: formDescription || null,
                    parent_id: formParentId || null,
                  },
                });
              }}
              disabled={!formName.trim() || updateMutation.isPending}
            >
              {tCommon("save")}
            </Button>
          </>
        }
      >
        <div className="space-y-4">
          <Input
            label={t("roleName")}
            value={formName}
            onChange={(e) => setFormName(e.target.value)}
          />
          <Input
            label={tCommon("description")}
            value={formDescription}
            onChange={(e) => setFormDescription(e.target.value)}
            placeholder={t("optionalDescription")}
          />
          <Select
            label={t("parentRole")}
            value={formParentId}
            onChange={(e) => setFormParentId(e.target.value)}
            options={[
              { value: "", label: t("noneRootLevel") },
              ...editParentOptions.map((r) => ({ value: r.id, label: r.name })),
            ]}
          />
        </div>
      </Modal>
    </div>
  );
}
