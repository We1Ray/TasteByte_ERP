"use client";

import { useState, useMemo } from "react";
import { useRouter } from "next/navigation";
import { type ColumnDef } from "@tanstack/react-table";
import { useTranslations } from "next-intl";
import { Plus, Trash2 } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { DataTable } from "@/components/ui/data-table";
import { Button } from "@/components/ui/button";
import { StatusBadge } from "@/components/ui/badge";
import { SearchBar } from "@/components/forms/search-bar";
import { Modal } from "@/components/ui/modal";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { useApiQuery, useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { usePagination } from "@/lib/hooks/use-pagination";
import { ppApi, type BillOfMaterial } from "@/lib/api/pp";
import { mmApi } from "@/lib/api/mm";
import { formatDate, formatNumber } from "@/lib/utils";

interface BomComponent {
  component_material_id: string;
  quantity: number;
}

export default function BomsPage() {
  const t = useTranslations("pp");
  const tc = useTranslations("common");

  const columns = useMemo<ColumnDef<BillOfMaterial, unknown>[]>(() => [
    {
      accessorKey: "bom_number",
      header: t("bomNumber"),
      cell: ({ row }) => (
        <span className="font-medium text-blue-600">{row.original.bom_number}</span>
      ),
    },
    {
      accessorKey: "material_name",
      header: t("material"),
      cell: ({ row }) => (
        <div>
          <p className="font-medium text-gray-900">{row.original.material_name ?? row.original.name ?? "-"}</p>
          <p className="text-xs text-gray-500">{row.original.material_number ?? ""}</p>
        </div>
      ),
    },
    {
      accessorKey: "quantity",
      header: t("baseQty"),
      cell: ({ row }) => (
        <span>
          {formatNumber(row.original.quantity ?? 0)} {row.original.unit ?? ""}
        </span>
      ),
    },
    {
      accessorKey: "items",
      header: t("components"),
      cell: ({ row }) => <span>{(row.original.items ?? []).length} {t("items")}</span>,
    },
    {
      accessorKey: "valid_from",
      header: t("validFrom"),
      cell: ({ row }) => formatDate(row.original.valid_from),
    },
    {
      accessorKey: "status",
      header: tc("status"),
      cell: ({ row }) => <StatusBadge status={row.original.status} />,
    },
  ], [t, tc]);

  const router = useRouter();
  const { page, pageSize, goToPage } = usePagination();
  const [search, setSearch] = useState("");
  const [showCreate, setShowCreate] = useState(false);
  const [newBom, setNewBom] = useState({
    material_id: "",
    quantity: 1,
    unit: "EA",
    valid_from: new Date().toISOString().split("T")[0],
  });
  const [components, setComponents] = useState<BomComponent[]>([
    { component_material_id: "", quantity: 1 },
  ]);
  const invalidate = useInvalidateQueries();

  const { data, isLoading } = useApiQuery(
    ["pp", "boms", String(page), search],
    () => ppApi.getBoms({ page, page_size: pageSize, search: search || undefined })
  );

  const { data: materialsData } = useApiQuery(
    ["mm", "materials", "all"],
    () => mmApi.getMaterials({ page: 1, page_size: 200 })
  );

  const materials = materialsData?.items || [];

  const createMutation = useApiMutation(
    (data: Partial<BillOfMaterial>) => ppApi.createBom(data),
    {
      onSuccess: () => {
        invalidate(["pp", "boms"]);
        setShowCreate(false);
        setNewBom({
          material_id: "",
          quantity: 1,
          unit: "EA",
          valid_from: new Date().toISOString().split("T")[0],
        });
        setComponents([{ component_material_id: "", quantity: 1 }]);
      },
    }
  );

  const addComponent = () => {
    setComponents([...components, { component_material_id: "", quantity: 1 }]);
  };

  const removeComponent = (index: number) => {
    if (components.length > 1) {
      setComponents(components.filter((_, i) => i !== index));
    }
  };

  const updateComponent = (index: number, field: keyof BomComponent, value: string | number) => {
    const updated = [...components];
    updated[index] = { ...updated[index], [field]: value };
    setComponents(updated);
  };

  const handleCreate = () => {
    createMutation.mutate({
      material_id: newBom.material_id,
      quantity: newBom.quantity,
      unit: newBom.unit,
      valid_from: newBom.valid_from,
      items: components.map((c, i) => ({
        component_material_id: c.component_material_id,
        quantity: c.quantity,
        position: i + 1,
      })),
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    } as any);
  };

  return (
    <div>
      <PageHeader
        title={t("billOfMaterials")}
        description={t("manageBoms")}
        actions={
          <Button onClick={() => setShowCreate(true)}>
            <Plus className="h-4 w-4" />
            {t("createBom")}
          </Button>
        }
      />

      <div className="mb-4">
        <SearchBar placeholder={t("searchBoms")} onSearch={setSearch} />
      </div>

      <DataTable
        columns={columns}
        data={data?.items || []}
        page={page}
        pageSize={pageSize}
        total={data?.total || 0}
        totalPages={data?.total_pages || 1}
        onPageChange={goToPage}
        onRowClick={(row) => router.push(`/pp/boms/${row.id}`)}
        isLoading={isLoading}
        emptyTitle={t("noBomsFound")}
        emptyDescription={t("createFirstBom")}
      />

      <Modal
        open={showCreate}
        onClose={() => setShowCreate(false)}
        title={t("createBomTitle")}
        size="xl"
        footer={
          <>
            <Button variant="secondary" onClick={() => setShowCreate(false)}>{tc("cancel")}</Button>
            <Button loading={createMutation.isPending} onClick={handleCreate}>
              {t("createBom")}
            </Button>
          </>
        }
      >
        <div className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <Select
              label={t("material")}
              required
              value={newBom.material_id}
              onChange={(e) => setNewBom({ ...newBom, material_id: e.target.value })}
              placeholder={t("selectMaterial")}
              options={materials.map((m) => ({
                value: m.id,
                label: `${m.material_number} - ${m.name}`,
              }))}
            />
            <Input
              label={t("baseQuantity")}
              type="number"
              min={1}
              required
              value={newBom.quantity}
              onChange={(e) => setNewBom({ ...newBom, quantity: Number(e.target.value) })}
            />
            <Select
              label={tc("unit")}
              value={newBom.unit}
              onChange={(e) => setNewBom({ ...newBom, unit: e.target.value })}
              options={[
                { value: "EA", label: t("eachEa") },
                { value: "KG", label: t("kilogramKg") },
                { value: "L", label: t("literL") },
                { value: "M", label: t("meterM") },
                { value: "BOX", label: t("boxBox") },
              ]}
            />
            <Input
              label={t("validFrom")}
              type="date"
              value={newBom.valid_from}
              onChange={(e) => setNewBom({ ...newBom, valid_from: e.target.value })}
            />
          </div>

          <div>
            <div className="mb-2 flex items-center justify-between">
              <h4 className="text-sm font-semibold text-gray-700">{t("components")}</h4>
              <Button type="button" variant="secondary" size="sm" onClick={addComponent}>
                <Plus className="h-4 w-4" />
                {t("addComponent")}
              </Button>
            </div>
            <div className="space-y-2">
              {components.map((comp, index) => (
                <div key={index} className="flex items-end gap-3 rounded-md border border-gray-200 bg-gray-50 p-3">
                  <div className="flex-1">
                    <Select
                      label={t("componentMaterial")}
                      required
                      value={comp.component_material_id}
                      onChange={(e) => updateComponent(index, "component_material_id", e.target.value)}
                      placeholder={t("selectMaterial")}
                      options={materials.map((m) => ({
                        value: m.id,
                        label: `${m.material_number} - ${m.name}`,
                      }))}
                    />
                  </div>
                  <div className="w-32">
                    <Input
                      label={tc("quantity")}
                      type="number"
                      min={1}
                      step="0.01"
                      required
                      value={comp.quantity}
                      onChange={(e) => updateComponent(index, "quantity", Number(e.target.value))}
                    />
                  </div>
                  <Button
                    type="button"
                    variant="ghost"
                    size="icon"
                    onClick={() => removeComponent(index)}
                    disabled={components.length <= 1}
                  >
                    <Trash2 className="h-4 w-4 text-red-500" />
                  </Button>
                </div>
              ))}
            </div>
          </div>
        </div>
      </Modal>
    </div>
  );
}
