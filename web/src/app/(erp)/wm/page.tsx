"use client";

import { useState, useMemo } from "react";
import { type ColumnDef } from "@tanstack/react-table";
import { Plus } from "lucide-react";
import { useTranslations } from "next-intl";
import { PageHeader } from "@/components/layout/page-header";
import { DataTable } from "@/components/ui/data-table";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Modal } from "@/components/ui/modal";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { SearchBar } from "@/components/forms/search-bar";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { useToastMutation } from "@/lib/hooks/use-toast-mutation";
import { usePagination } from "@/lib/hooks/use-pagination";
import { wmApi, type Warehouse } from "@/lib/api/wm";
import { formatNumber } from "@/lib/utils";

export default function WarehousePage() {
  const t = useTranslations("wm");
  const tCommon = useTranslations("common");
  const tShared = useTranslations("shared");

  const warehouseColumns = useMemo<ColumnDef<Warehouse, unknown>[]>(() => [
    {
      accessorKey: "warehouse_number",
      header: t("warehouseNo"),
      cell: ({ row }) => (
        <span className="font-medium text-blue-600">{row.original.warehouse_number}</span>
      ),
    },
    { accessorKey: "name", header: tCommon("name") },
    { accessorKey: "city", header: "City" },
    { accessorKey: "country", header: "Country" },
    {
      accessorKey: "storage_bin_count",
      header: t("storageBins"),
      cell: ({ row }) => formatNumber(row.original.storage_bin_count),
    },
    {
      accessorKey: "capacity",
      header: t("utilization"),
      cell: ({ row }) => {
        const utilization = row.original.total_capacity > 0
          ? (row.original.used_capacity / row.original.total_capacity) * 100
          : 0;
        const color = utilization > 90 ? "red" : utilization > 70 ? "amber" : "green";
        return (
          <div className="flex items-center gap-2">
            <div className="h-2 w-20 overflow-hidden rounded-full bg-gray-200">
              <div
                className={`h-full rounded-full ${
                  color === "red" ? "bg-red-500" : color === "amber" ? "bg-amber-500" : "bg-green-500"
                }`}
                style={{ width: `${Math.min(utilization, 100)}%` }}
              />
            </div>
            <span className="text-xs text-gray-500">{utilization.toFixed(0)}%</span>
          </div>
        );
      },
    },
    {
      accessorKey: "is_active",
      header: tCommon("status"),
      cell: ({ row }) => (
        <Badge color={row.original.is_active ? "green" : "gray"}>
          {row.original.is_active ? tCommon("active") : tShared("inactive")}
        </Badge>
      ),
    },
  ], [t, tCommon, tShared]);

  const { page, pageSize, goToPage } = usePagination();
  const [search, setSearch] = useState("");
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [formCode, setFormCode] = useState("");
  const [formName, setFormName] = useState("");
  const [formAddress, setFormAddress] = useState("");
  const [formType, setFormType] = useState("STANDARD");

  const { data, isLoading } = useApiQuery(
    ["wm", "warehouses", String(page), search],
    () =>
      wmApi.getWarehouses({
        page,
        page_size: pageSize,
        search: search || undefined,
      })
  );

  const createMutation = useToastMutation(
    (input: { code: string; name: string; address?: string; warehouse_type?: string }) =>
      wmApi.createWarehouse(input),
    {
      successMessage: t("warehouseCreated"),
      invalidateKeys: ["wm", "warehouses"],
      onSuccess: () => {
        setShowCreateModal(false);
        resetForm();
      },
    }
  );

  function resetForm() {
    setFormCode("");
    setFormName("");
    setFormAddress("");
    setFormType("STANDARD");
  }

  function openCreateModal() {
    resetForm();
    setShowCreateModal(true);
  }

  return (
    <div>
      <PageHeader
        title={t("warehouseManagement")}
        description={t("warehouseManagementDesc")}
        actions={
          <Button onClick={openCreateModal}>
            <Plus className="h-4 w-4" />
            {t("addWarehouse")}
          </Button>
        }
      />

      <div className="mb-4">
        <SearchBar placeholder={t("searchWarehouses")} onSearch={setSearch} />
      </div>

      <DataTable
        columns={warehouseColumns}
        data={data?.items || []}
        page={page}
        pageSize={pageSize}
        total={data?.total || 0}
        totalPages={data?.total_pages || 1}
        onPageChange={goToPage}
        isLoading={isLoading}
        emptyTitle={t("noWarehousesFound")}
        emptyDescription={t("addFirstWarehouse")}
      />

      <Modal
        open={showCreateModal}
        onClose={() => setShowCreateModal(false)}
        title={t("addWarehouse")}
        footer={
          <>
            <Button variant="secondary" onClick={() => setShowCreateModal(false)}>
              {tCommon("cancel")}
            </Button>
            <Button
              onClick={() =>
                createMutation.mutate({
                  code: formCode,
                  name: formName,
                  address: formAddress || undefined,
                  warehouse_type: formType || undefined,
                })
              }
              disabled={!formCode.trim() || !formName.trim() || createMutation.isPending}
              loading={createMutation.isPending}
            >
              {tCommon("create")}
            </Button>
          </>
        }
      >
        <div className="space-y-4">
          <Input
            label={t("warehouseCode")}
            required
            value={formCode}
            onChange={(e) => setFormCode(e.target.value)}
            placeholder={t("warehouseCodePlaceholder")}
          />
          <Input
            label={tCommon("name")}
            required
            value={formName}
            onChange={(e) => setFormName(e.target.value)}
            placeholder={t("warehouseNamePlaceholder")}
          />
          <Input
            label="Address"
            value={formAddress}
            onChange={(e) => setFormAddress(e.target.value)}
            placeholder={t("addressPlaceholder")}
          />
          <Select
            label={t("warehouseType")}
            value={formType}
            onChange={(e) => setFormType(e.target.value)}
            options={[
              { value: "STANDARD", label: t("standard") },
              { value: "COLD_STORAGE", label: t("coldStorage") },
              { value: "BONDED", label: t("bonded") },
              { value: "DISTRIBUTION", label: t("distributionCenter") },
            ]}
          />
        </div>
      </Modal>
    </div>
  );
}
