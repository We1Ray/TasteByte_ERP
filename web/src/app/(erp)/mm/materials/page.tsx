"use client";

import { useState, useMemo } from "react";
import { useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import { Plus } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { DataTable } from "@/components/ui/data-table";
import { Button } from "@/components/ui/button";
import { SearchBar } from "@/components/forms/search-bar";
import { ExportButton } from "@/components/shared/export-button";
import { PrintButton } from "@/components/shared/print-button";
import { BulkActionBar } from "@/components/ui/bulk-action-bar";
import { StatusFilter, useMaterialStatuses } from "@/components/shared/status-filter";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { usePagination } from "@/lib/hooks/use-pagination";
import { mmApi, type Material } from "@/lib/api/mm";
import { getMaterialColumns } from "@/modules/mm/material-columns";

export default function MaterialsPage() {
  const router = useRouter();
  const t = useTranslations("mm");
  const tCommon = useTranslations("common");
  const tShared = useTranslations("shared");
  const { page, pageSize, goToPage } = usePagination();
  const [search, setSearch] = useState("");
  const [typeFilter, setTypeFilter] = useState("");
  const [statusFilter, setStatusFilter] = useState("");
  const [selectedRows, setSelectedRows] = useState<Material[]>([]);
  const materialStatuses = useMaterialStatuses();

  const columns = useMemo(
    () => getMaterialColumns(t, tCommon, tShared),
    [t, tCommon, tShared]
  );

  const { data, isLoading } = useApiQuery(
    ["mm", "materials", String(page), search, typeFilter, statusFilter],
    () =>
      mmApi.getMaterials({
        page,
        page_size: pageSize,
        search: search || undefined,
        material_type: typeFilter || undefined,
      })
  );

  const filteredItems = statusFilter
    ? (data?.items || []).filter((item) =>
        statusFilter === "Active" ? item.is_active : !item.is_active
      )
    : data?.items || [];

  return (
    <div>
      <PageHeader
        title={t("materials")}
        description={t("manageMaterials")}
        actions={
          <>
            <PrintButton />
            <ExportButton
              data={filteredItems}
              filename="materials"
              sheetName="Materials"
            />
            <Button onClick={() => router.push("/mm/materials/new")}>
              <Plus className="h-4 w-4" />
              {t("createMaterial")}
            </Button>
          </>
        }
      />

      <div className="mb-4 flex flex-wrap items-center gap-3">
        <SearchBar
          placeholder={t("searchMaterials")}
          onSearch={setSearch}
          filters={[
            {
              key: "material_type",
              label: t("allTypes"),
              value: typeFilter,
              onChange: setTypeFilter,
              options: [
                { value: "RAW", label: t("rawMaterial") },
                { value: "SEMI", label: t("semiFinished") },
                { value: "FERT", label: t("finishedProduct") },
                { value: "HIBE", label: t("operatingSupplies") },
                { value: "TRADING", label: t("tradingGoods") },
              ],
            },
          ]}
        />
        <StatusFilter
          value={statusFilter}
          onChange={setStatusFilter}
          options={materialStatuses}
          allLabel={tShared("allStatuses")}
        />
      </div>

      <BulkActionBar
        selectedCount={selectedRows.length}
        onClearSelection={() => setSelectedRows([])}
      >
        <ExportButton
          data={selectedRows}
          filename="selected-materials"
          sheetName="Selected Materials"
        />
      </BulkActionBar>

      <DataTable
        columns={columns}
        data={filteredItems}
        enableSelection
        onSelectionChange={setSelectedRows}
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        getRowId={(row: any) => row.id}
        page={page}
        pageSize={pageSize}
        total={data?.total || 0}
        totalPages={data?.total_pages || 1}
        onPageChange={goToPage}
        onRowClick={(row) => router.push(`/mm/materials/${row.id}`)}
        isLoading={isLoading}
        emptyTitle={t("noMaterialsFound")}
        emptyDescription={t("createFirstMaterial")}
      />
    </div>
  );
}
