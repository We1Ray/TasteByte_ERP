"use client";

import { useState, useMemo } from "react";
import { useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import { type ColumnDef } from "@tanstack/react-table";
import { FileText, List, LayoutDashboard, Plus, Copy, Edit3, Trash2 } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { DataTable } from "@/components/ui/data-table";
import { Button } from "@/components/ui/button";
import { Select } from "@/components/ui/select";
import { ConfirmDialog } from "@/components/ui/confirm-dialog";
import { SearchBar } from "@/components/forms/search-bar";
import { StatusBadge } from "@/components/ui/badge";
import { useApiQuery, useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { operationsApi, projectsApi } from "@/lib/api/lowcode";
import { formatDateTime } from "@/lib/utils";
import type { LowCodeOperation } from "@/lib/types/lowcode";

const typeIcons: Record<string, React.ElementType> = {
  form: FileText,
  list: List,
  dashboard: LayoutDashboard,
};

export default function OperationsListPage() {
  const router = useRouter();
  const t = useTranslations("developer");
  const tCommon = useTranslations("common");

  const [search, setSearch] = useState("");
  const [typeFilter, setTypeFilter] = useState<string>("");
  const [projectFilter, setProjectFilter] = useState<string>("");
  const [deleteTarget, setDeleteTarget] = useState<LowCodeOperation | null>(null);

  const invalidate = useInvalidateQueries();

  const cloneMutation = useApiMutation(
    async (op: LowCodeOperation) => {
      const newOp = await operationsApi.create({
        code: `${op.code}_COPY`,
        name: `${op.name} (Copy)`,
        operation_type: op.operation_type,
        project_id: op.project_id,
      });
      return newOp;
    },
    {
      onSuccess: () => invalidate(["lowcode", "operations"]),
    }
  );

  const deleteMutation = useApiMutation(
    (id: string) => operationsApi.delete(id),
    {
      onSuccess: () => {
        invalidate(["lowcode", "operations"]);
        setDeleteTarget(null);
      },
    }
  );

  const { data: operations, isLoading } = useApiQuery(
    ["lowcode", "operations", search, typeFilter, projectFilter],
    () =>
      operationsApi.list({
        search: search || undefined,
        status: undefined,
        project_id: projectFilter || undefined,
        page_size: 50,
      })
  );

  const { data: projects } = useApiQuery(
    ["lowcode", "projects"],
    () => projectsApi.list({ page_size: 100 })
  );

  const columns = useMemo<ColumnDef<LowCodeOperation, unknown>[]>(
    () => {
      const typeLabels: Record<string, string> = {
        form: t("formType"),
        list: t("listType"),
        dashboard: t("dashboardType"),
      };
      return [
      { accessorKey: "code", header: t("code") },
      { accessorKey: "name", header: tCommon("name") },
      {
        accessorKey: "operation_type",
        header: tCommon("type"),
        cell: ({ row }) => {
          const type = row.original.operation_type;
          const Icon = typeIcons[type] ?? FileText;
          return (
            <span className="inline-flex items-center gap-1.5 text-sm">
              <Icon className="h-3.5 w-3.5 text-gray-400" />
              {typeLabels[type] ?? type}
            </span>
          );
        },
      },
      {
        accessorKey: "project_id",
        header: t("project"),
        cell: ({ row }) => {
          const proj = (projects?.items ?? []).find(
            (p) => p.id === row.original.project_id
          );
          return (
            <span className="text-sm text-gray-500">{proj?.name ?? "-"}</span>
          );
        },
      },
      {
        accessorKey: "status",
        header: tCommon("status"),
        cell: ({ row }) => <StatusBadge status={row.original.status} />,
      },
      {
        accessorKey: "updated_at",
        header: t("lastUpdated"),
        cell: ({ row }) => (
          <span className="text-sm text-gray-500">
            {formatDateTime(row.original.updated_at)}
          </span>
        ),
      },
      {
        id: "actions",
        header: "",
        cell: ({ row }) => {
          const op = row.original;
          return (
            <div className="flex items-center gap-1" onClick={(e) => e.stopPropagation()}>
              <button
                onClick={() => router.push(`/developer/operations/${op.id}`)}
                className="rounded p-1 text-gray-400 hover:bg-gray-100 hover:text-blue-500"
                title={tCommon("edit")}
              >
                <Edit3 className="h-4 w-4" />
              </button>
              <button
                onClick={() => cloneMutation.mutateAsync(op)}
                className="rounded p-1 text-gray-400 hover:bg-gray-100 hover:text-green-500"
                title="Clone"
                disabled={cloneMutation.isPending}
              >
                <Copy className="h-4 w-4" />
              </button>
              <button
                onClick={() => setDeleteTarget(op)}
                className="rounded p-1 text-gray-400 hover:bg-gray-100 hover:text-red-500"
                title={tCommon("delete")}
              >
                <Trash2 className="h-4 w-4" />
              </button>
            </div>
          );
        },
      },
    ];
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [t, tCommon, projects, cloneMutation.isPending]
  );

  // Client-side type filter
  const filteredData = (operations?.items ?? []).filter((op) => {
    if (typeFilter && op.operation_type !== typeFilter) return false;
    return true;
  });

  return (
    <div>
      <PageHeader
        title={t("operationsList")}
        description={t("operationsDesc")}
        actions={
          <Button onClick={() => router.push("/developer/operations/new")}>
            <Plus className="h-4 w-4" />
            {t("createOperation")}
          </Button>
        }
      />

      <div className="mb-4 flex flex-wrap items-center gap-3">
        <div className="flex-1">
          <SearchBar
            placeholder={t("searchOperations")}
            onSearch={setSearch}
          />
        </div>
        <Select
          value={typeFilter}
          onChange={(e) => setTypeFilter(e.target.value)}
          options={[
            { value: "", label: t("allTypes") },
            { value: "form", label: t("formType") },
            { value: "list", label: t("listType") },
            { value: "dashboard", label: t("dashboardType") },
          ]}
          className="w-40"
        />
        <Select
          value={projectFilter}
          onChange={(e) => setProjectFilter(e.target.value)}
          options={[
            { value: "", label: t("allProjects") },
            ...(projects?.items ?? []).map((p) => ({ value: p.id, label: p.name })),
          ]}
          className="w-48"
        />
      </div>

      <DataTable
        columns={columns}
        data={filteredData}
        onRowClick={(row) => router.push(`/developer/operations/${row.id}`)}
        isLoading={isLoading}
        emptyTitle={t("noOperationsFound")}
        emptyDescription={search || typeFilter || projectFilter ? t("adjustSearchCriteria") : t("createFirstOperation")}
      />

      <ConfirmDialog
        open={!!deleteTarget}
        onClose={() => setDeleteTarget(null)}
        onConfirm={() => {
          if (deleteTarget) {
            deleteMutation.mutateAsync(deleteTarget.id);
          }
        }}
        title={tCommon("delete")}
        message={`Are you sure you want to delete operation "${deleteTarget?.name ?? ""}"? This action cannot be undone.`}
        confirmLabel={tCommon("delete")}
        variant="danger"
        loading={deleteMutation.isPending}
      />
    </div>
  );
}
