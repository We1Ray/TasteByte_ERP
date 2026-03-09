"use client";

import { useState, useMemo } from "react";
import { Plus, Edit3, Trash2 } from "lucide-react";
import { type ColumnDef } from "@tanstack/react-table";
import { useTranslations } from "next-intl";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { DataTable } from "@/components/ui/data-table";
import { StatusBadge } from "@/components/ui/badge";
import { Modal } from "@/components/ui/modal";
import { Input } from "@/components/ui/input";
import { ConfirmDialog } from "@/components/ui/confirm-dialog";
import { SearchBar } from "@/components/forms/search-bar";
import { useApiQuery, useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { usePagination } from "@/lib/hooks/use-pagination";
import { projectsApi } from "@/lib/api/lowcode";
import { formatDateTime } from "@/lib/utils";
import type { LowCodeProject } from "@/lib/types/lowcode";

export default function AdminProjectsPage() {
  const t = useTranslations("admin");
  const tCommon = useTranslations("common");
  const { page, pageSize, goToPage } = usePagination();
  const [search, setSearch] = useState("");
  const [showCreate, setShowCreate] = useState(false);
  const [editingProject, setEditingProject] = useState<LowCodeProject | null>(null);
  const [deleteTarget, setDeleteTarget] = useState<LowCodeProject | null>(null);
  const [formData, setFormData] = useState({ code: "", name: "", description: "" });
  const invalidate = useInvalidateQueries();

  const columns: ColumnDef<LowCodeProject, unknown>[] = useMemo(() => [
    { accessorKey: "code", header: t("projectCode") },
    { accessorKey: "name", header: tCommon("name") },
    {
      accessorKey: "status",
      header: tCommon("status"),
      cell: ({ row }) => <StatusBadge status={row.original.status} />,
    },
    { accessorKey: "owner_name", header: t("owner") },
    {
      accessorKey: "created_at",
      header: tCommon("createdAt"),
      cell: ({ row }) => formatDateTime(row.original.created_at),
    },
    {
      id: "actions",
      header: tCommon("actions"),
      cell: ({ row }) => (
        <div className="flex items-center gap-1" onClick={(e) => e.stopPropagation()}>
          <button
            onClick={() => {
              const proj = row.original;
              setEditingProject(proj);
              setFormData({ code: proj.code, name: proj.name, description: proj.description || "" });
            }}
            className="rounded p-1 text-gray-400 hover:bg-gray-100 hover:text-blue-500"
            title={tCommon("edit")}
          >
            <Edit3 className="h-4 w-4" />
          </button>
          <button
            onClick={() => setDeleteTarget(row.original)}
            className="rounded p-1 text-gray-400 hover:bg-gray-100 hover:text-red-500"
            title={tCommon("delete")}
          >
            <Trash2 className="h-4 w-4" />
          </button>
        </div>
      ),
    },
  ], [t, tCommon]);

  const { data, isLoading } = useApiQuery(
    ["lowcode", "projects", String(page), search],
    () => projectsApi.list({ page, page_size: pageSize, search: search || undefined })
  );

  const createMutation = useApiMutation(
    (data: Partial<LowCodeProject>) => projectsApi.create(data),
    {
      onSuccess: () => {
        invalidate(["lowcode", "projects"]);
        setShowCreate(false);
        setFormData({ code: "", name: "", description: "" });
      },
    }
  );

  const updateMutation = useApiMutation(
    ({ id, data }: { id: string; data: Partial<LowCodeProject> }) => projectsApi.update(id, data),
    {
      onSuccess: () => {
        invalidate(["lowcode", "projects"]);
        setEditingProject(null);
      },
    }
  );

  const deleteMutation = useApiMutation(
    (id: string) => projectsApi.delete(id),
    {
      onSuccess: () => {
        invalidate(["lowcode", "projects"]);
        setDeleteTarget(null);
      },
    }
  );

  return (
    <div>
      <PageHeader
        title={t("projects")}
        description={t("manageProjects")}
        actions={
          <Button onClick={() => setShowCreate(true)}>
            <Plus className="h-4 w-4" />
            {t("createProject")}
          </Button>
        }
      />

      <div className="mb-4">
        <SearchBar placeholder={t("searchProjects")} onSearch={setSearch} />
      </div>

      <DataTable
        columns={columns}
        data={data?.items || []}
        page={page}
        pageSize={pageSize}
        total={data?.total || 0}
        totalPages={data?.total_pages || 1}
        onPageChange={goToPage}
        isLoading={isLoading}
        emptyTitle={t("noProjectsFound")}
      />

      <Modal
        open={showCreate}
        onClose={() => setShowCreate(false)}
        title={t("createProject")}
        footer={
          <>
            <Button variant="secondary" onClick={() => setShowCreate(false)}>{tCommon("cancel")}</Button>
            <Button
              loading={createMutation.isPending}
              onClick={() => createMutation.mutateAsync(formData)}
            >
              {tCommon("create")}
            </Button>
          </>
        }
      >
        <div className="space-y-4">
          <Input
            label={t("projectCode")}
            required
            value={formData.code}
            onChange={(e) => setFormData({ ...formData, code: e.target.value })}
            placeholder={t("projectCodePlaceholder")}
          />
          <Input
            label={t("projectName")}
            required
            value={formData.name}
            onChange={(e) => setFormData({ ...formData, name: e.target.value })}
            placeholder={t("projectNamePlaceholder")}
          />
          <div className="w-full">
            <label className="mb-1 block text-sm font-medium text-gray-700">{tCommon("description")}</label>
            <textarea
              value={formData.description}
              onChange={(e) => setFormData({ ...formData, description: e.target.value })}
              rows={3}
              className="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
              placeholder={t("projectDescPlaceholder")}
            />
          </div>
        </div>
      </Modal>

      <Modal
        open={!!editingProject}
        onClose={() => setEditingProject(null)}
        title={`${tCommon("edit")}: ${editingProject?.name ?? ""}`}
        footer={
          <>
            <Button variant="secondary" onClick={() => setEditingProject(null)}>{tCommon("cancel")}</Button>
            <Button
              loading={updateMutation.isPending}
              onClick={() => {
                if (editingProject) {
                  updateMutation.mutateAsync({
                    id: editingProject.id,
                    data: { name: formData.name, description: formData.description },
                  });
                }
              }}
            >
              {tCommon("save")}
            </Button>
          </>
        }
      >
        <div className="space-y-4">
          <Input
            label={t("projectCode")}
            value={formData.code}
            disabled
          />
          <Input
            label={t("projectName")}
            required
            value={formData.name}
            onChange={(e) => setFormData({ ...formData, name: e.target.value })}
          />
          <div className="w-full">
            <label className="mb-1 block text-sm font-medium text-gray-700">{tCommon("description")}</label>
            <textarea
              value={formData.description}
              onChange={(e) => setFormData({ ...formData, description: e.target.value })}
              rows={3}
              className="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
            />
          </div>
        </div>
      </Modal>

      <ConfirmDialog
        open={!!deleteTarget}
        onClose={() => setDeleteTarget(null)}
        onConfirm={() => {
          if (deleteTarget) {
            deleteMutation.mutateAsync(deleteTarget.id);
          }
        }}
        title={tCommon("delete")}
        message={`Are you sure you want to delete project "${deleteTarget?.name ?? ""}"? This action cannot be undone.`}
        confirmLabel={tCommon("delete")}
        variant="danger"
        loading={deleteMutation.isPending}
      />
    </div>
  );
}
