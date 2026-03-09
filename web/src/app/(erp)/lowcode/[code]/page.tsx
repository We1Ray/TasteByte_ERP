"use client";

import { useState } from "react";
import { useRouter, useParams } from "next/navigation";
import { Plus } from "lucide-react";
import { type ColumnDef } from "@tanstack/react-table";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { DataTable } from "@/components/ui/data-table";
import { SearchBar } from "@/components/forms/search-bar";
import { DynamicForm } from "@/components/lowcode/form-renderer/DynamicForm";
import { ListRenderer } from "@/components/lowcode/list-renderer/ListRenderer";
import { DashboardRenderer } from "@/components/lowcode/dashboard-renderer/DashboardRenderer";
import { Modal } from "@/components/ui/modal";
import { usePagination } from "@/lib/hooks/use-pagination";
import { useDynamicForm, useFormRecords } from "@/lib/hooks/use-dynamic-form";
import { formatDateTime } from "@/lib/utils";
import type { FormRecord } from "@/lib/types/lowcode";

export default function LowCodeOperationPage() {
  const params = useParams();
  const code = params.code as string;

  const { operation } = useDynamicForm(code);
  const operationType = operation?.operation_type?.toLowerCase();

  // Branch on operation_type
  if (operationType === "list" && operation?.id) {
    return <ListRenderer operationId={operation.id} operationCode={code} />;
  }

  if (operationType === "dashboard" && operation?.id) {
    return <DashboardRenderer operationId={operation.id} />;
  }

  // Default: FORM operation
  return <FormOperationPage code={code} />;
}

function FormOperationPage({ code }: { code: string }) {
  const router = useRouter();
  const { page, pageSize, goToPage } = usePagination();
  const [search, setSearch] = useState("");
  const [showCreateModal, setShowCreateModal] = useState(false);

  const { operation, formDefinition } = useDynamicForm(code);
  const { data, isLoading } = useFormRecords(code, { page, page_size: pageSize, search: search || undefined });

  // Build columns from form definition
  const columns: ColumnDef<FormRecord, unknown>[] = [
    {
      accessorKey: "id",
      header: "ID",
      cell: ({ row }) => (
        <span className="font-mono text-xs text-gray-500">
          {row.original.id.slice(0, 8)}...
        </span>
      ),
    },
    ...(formDefinition?.sections ?? [])
      .flatMap((s) => s.fields)
      .slice(0, 4)
      .map((field) => ({
        id: field.field_key,
        header: field.label,
        accessorFn: (row: FormRecord) => {
          const val = row.data[field.field_key];
          if (val === null || val === undefined) return "-";
          return String(val);
        },
      })),
    {
      accessorKey: "created_at",
      header: "Created",
      cell: ({ row }) => formatDateTime(row.original.created_at),
    },
  ];

  return (
    <div>
      <PageHeader
        title={operation?.name || code}
        description={operation?.description || "Dynamic form operation"}
        actions={
          <Button onClick={() => setShowCreateModal(true)}>
            <Plus className="h-4 w-4" />
            Create New
          </Button>
        }
      />

      <div className="mb-4">
        <SearchBar placeholder="Search records..." onSearch={setSearch} />
      </div>

      <DataTable
        columns={columns}
        data={data?.items || []}
        page={page}
        pageSize={pageSize}
        total={data?.total || 0}
        totalPages={data?.total_pages || 1}
        onPageChange={goToPage}
        onRowClick={(row) => router.push(`/lowcode/${code}/${row.id}`)}
        isLoading={isLoading}
        emptyTitle="No records found"
        emptyDescription="Create your first record to get started"
      />

      <Modal
        open={showCreateModal}
        onClose={() => setShowCreateModal(false)}
        title={`Create ${operation?.name || "Record"}`}
        size="xl"
      >
        <DynamicForm
          operationCode={code}
          onSubmitSuccess={() => setShowCreateModal(false)}
        />
      </Modal>
    </div>
  );
}
