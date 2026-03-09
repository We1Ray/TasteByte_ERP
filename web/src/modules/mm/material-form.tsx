"use client";

import { useForm, type Resolver } from "react-hook-form";
import { standardSchemaResolver } from "@hookform/resolvers/standard-schema";
import { z } from "zod/v4";
import { useTranslations } from "next-intl";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { type Material } from "@/lib/api/mm";

const materialSchema = z.object({
  material_number: z.string().min(1, "Material number is required"),
  name: z.string().min(1, "Name is required"),
  description: z.string().optional().default(""),
  material_type: z.string().min(1, "Material type is required"),
  material_group: z.string().min(1, "Material group is required"),
  base_unit: z.string().min(1, "Base unit is required"),
  weight: z.coerce.number().optional(),
  weight_unit: z.string().optional(),
  price: z.coerce.number().min(0, "Price must be positive"),
  currency: z.string().default("USD"),
  min_stock: z.coerce.number().min(0).default(0),
  max_stock: z.coerce.number().min(0).default(0),
  reorder_point: z.coerce.number().min(0).default(0),
});

type MaterialFormData = z.infer<typeof materialSchema>;

interface MaterialFormProps {
  defaultValues?: Partial<Material>;
  onSubmit: (data: MaterialFormData) => void;
  isLoading?: boolean;
  submitLabel?: string;
}

export function MaterialForm({
  defaultValues,
  onSubmit,
  isLoading,
  submitLabel,
}: MaterialFormProps) {
  const t = useTranslations("mm");
  const tCommon = useTranslations("common");
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<MaterialFormData>({
    resolver: standardSchemaResolver(materialSchema) as Resolver<MaterialFormData>,
    defaultValues: {
      material_number: defaultValues?.material_number || "",
      name: defaultValues?.name || "",
      description: defaultValues?.description || "",
      material_type: defaultValues?.material_type || "RAW",
      material_group: defaultValues?.material_group || "",
      base_unit: defaultValues?.base_unit || "EA",
      weight: defaultValues?.weight || undefined,
      weight_unit: defaultValues?.weight_unit || "KG",
      price: defaultValues?.price || 0,
      currency: defaultValues?.currency || "USD",
      min_stock: defaultValues?.min_stock || 0,
      max_stock: defaultValues?.max_stock || 0,
      reorder_point: defaultValues?.reorder_point || 0,
    },
  });

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
      <Card>
        <h3 className="mb-4 text-sm font-semibold uppercase text-gray-500">{t("basicData")}</h3>
        <div className="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
          <Input
            label={t("materialNo")}
            required
            error={errors.material_number?.message}
            {...register("material_number")}
            placeholder="e.g., MAT-001"
          />
          <Input
            label={tCommon("name")}
            required
            error={errors.name?.message}
            {...register("name")}
            placeholder="e.g., Steel Plate A4"
          />
          <Select
            label={t("materialType")}
            required
            error={errors.material_type?.message}
            {...register("material_type")}
            options={[
              { value: "RAW", label: t("rawMaterial") },
              { value: "SEMI", label: t("semiFinished") },
              { value: "FERT", label: t("finishedProduct") },
              { value: "HIBE", label: t("operatingSupplies") },
              { value: "TRADING", label: t("tradingGoods") },
            ]}
          />
          <Input
            label={t("materialGroup")}
            required
            error={errors.material_group?.message}
            {...register("material_group")}
            placeholder="e.g., Electronics"
          />
          <Select
            label={t("baseUnit")}
            required
            error={errors.base_unit?.message}
            {...register("base_unit")}
            options={[
              { value: "EA", label: "Each (EA)" },
              { value: "KG", label: "Kilogram (KG)" },
              { value: "L", label: "Liter (L)" },
              { value: "M", label: "Meter (M)" },
              { value: "BOX", label: "Box (BOX)" },
            ]}
          />
          <div className="md:col-span-2 lg:col-span-3">
            <Input
              label={tCommon("description")}
              error={errors.description?.message}
              {...register("description")}
              placeholder="Material description..."
            />
          </div>
        </div>
      </Card>

      <Card>
        <h3 className="mb-4 text-sm font-semibold uppercase text-gray-500">{t("pricing")}</h3>
        <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
          <Input
            label={tCommon("price")}
            type="number"
            step="0.01"
            required
            error={errors.price?.message}
            {...register("price")}
          />
          <Select
            label={tCommon("currency")}
            {...register("currency")}
            options={[
              { value: "USD", label: "USD" },
              { value: "EUR", label: "EUR" },
              { value: "GBP", label: "GBP" },
              { value: "TWD", label: "TWD" },
            ]}
          />
        </div>
      </Card>

      <Card>
        <h3 className="mb-4 text-sm font-semibold uppercase text-gray-500">{t("weightDimensions")}</h3>
        <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
          <Input
            label={t("weight")}
            type="number"
            step="0.01"
            error={errors.weight?.message}
            {...register("weight")}
          />
          <Select
            label={t("weightUnit")}
            {...register("weight_unit")}
            options={[
              { value: "KG", label: "Kilogram" },
              { value: "G", label: "Gram" },
              { value: "LB", label: "Pound" },
            ]}
          />
        </div>
      </Card>

      <Card>
        <h3 className="mb-4 text-sm font-semibold uppercase text-gray-500">{t("inventoryPlanning")}</h3>
        <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
          <Input
            label={t("minimumStock")}
            type="number"
            error={errors.min_stock?.message}
            {...register("min_stock")}
          />
          <Input
            label={t("maximumStock")}
            type="number"
            error={errors.max_stock?.message}
            {...register("max_stock")}
          />
          <Input
            label={t("reorderPoint")}
            type="number"
            error={errors.reorder_point?.message}
            {...register("reorder_point")}
          />
        </div>
      </Card>

      <div className="flex justify-end gap-3">
        <Button type="button" variant="secondary" onClick={() => window.history.back()}>
          {tCommon("cancel")}
        </Button>
        <Button type="submit" loading={isLoading}>
          {submitLabel ?? t("createMaterial")}
        </Button>
      </div>
    </form>
  );
}
