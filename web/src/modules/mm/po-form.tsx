"use client";

import { useForm, useFieldArray } from "react-hook-form";
import { z } from "zod/v4";
import { standardSchemaResolver } from "@hookform/resolvers/standard-schema";
import { useTranslations } from "next-intl";
import { Plus, Trash2 } from "lucide-react";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";

const poItemSchema = z.object({
  material_id: z.string().min(1, "Material is required"),
  material_number: z.string(),
  quantity: z.coerce.number().min(1, "Quantity must be at least 1"),
  unit: z.string().default("EA"),
  unit_price: z.coerce.number().min(0, "Price must be positive"),
});

const poSchema = z.object({
  vendor_id: z.string().min(1, "Vendor is required"),
  order_date: z.string().min(1, "Order date is required"),
  delivery_date: z.string().min(1, "Delivery date is required"),
  currency: z.string().default("USD"),
  items: z.array(poItemSchema).min(1, "At least one item is required"),
});

type PoFormData = z.infer<typeof poSchema>;

interface PoFormProps {
  onSubmit: (data: PoFormData) => void;
  isLoading?: boolean;
}

export function PoForm({ onSubmit, isLoading }: PoFormProps) {
  const t = useTranslations("mm");
  const tCommon = useTranslations("common");
  const tShared = useTranslations("shared");
  const {
    register,
    handleSubmit,
    control,
    watch,
    formState: { errors },
  } = useForm<PoFormData>({
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    resolver: standardSchemaResolver(poSchema) as any,
    defaultValues: {
      vendor_id: "",
      order_date: new Date().toISOString().split("T")[0],
      delivery_date: "",
      currency: "USD",
      items: [{ material_id: "", material_number: "", quantity: 1, unit: "EA", unit_price: 0 }],
    },
  });

  const { fields, append, remove } = useFieldArray({
    control,
    name: "items",
  });

  const items = watch("items");
  const totalAmount = items?.reduce((sum, item) => sum + (item.quantity || 0) * (item.unit_price || 0), 0) || 0;

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
      <Card>
        <h3 className="mb-4 text-sm font-semibold uppercase text-gray-500">{tShared("orderHeader")}</h3>
        <div className="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-4">
          <Input
            label={t("vendor")}
            required
            error={errors.vendor_id?.message}
            {...register("vendor_id")}
            placeholder="Select vendor..."
          />
          <Input
            label={t("orderDate")}
            type="date"
            required
            error={errors.order_date?.message}
            {...register("order_date")}
          />
          <Input
            label={t("deliveryDate")}
            type="date"
            required
            error={errors.delivery_date?.message}
            {...register("delivery_date")}
          />
          <Select
            label={tCommon("currency")}
            {...register("currency")}
            options={[
              { value: "USD", label: "USD" },
              { value: "EUR", label: "EUR" },
              { value: "TWD", label: "TWD" },
            ]}
          />
        </div>
      </Card>

      <Card>
        <div className="mb-4 flex items-center justify-between">
          <h3 className="text-sm font-semibold uppercase text-gray-500">{tShared("lineItems")}</h3>
          <Button
            type="button"
            variant="secondary"
            size="sm"
            onClick={() =>
              append({ material_id: "", material_number: "", quantity: 1, unit: "EA", unit_price: 0 })
            }
          >
            <Plus className="h-4 w-4" />
            {t("addItem")}
          </Button>
        </div>

        {errors.items?.root && (
          <p className="mb-3 text-sm text-red-600">{errors.items.root.message}</p>
        )}

        <div className="space-y-4">
          {fields.map((field, index) => (
            <div
              key={field.id}
              className="flex items-end gap-3 rounded-lg border border-gray-100 bg-gray-50 p-4"
            >
              <div className="flex-1">
                <Input
                  label={t("materials")}
                  required
                  error={errors.items?.[index]?.material_id?.message}
                  {...register(`items.${index}.material_id`)}
                  placeholder="Material ID"
                />
              </div>
              <div className="w-24">
                <Input
                  label={tCommon("quantity")}
                  type="number"
                  required
                  error={errors.items?.[index]?.quantity?.message}
                  {...register(`items.${index}.quantity`)}
                />
              </div>
              <div className="w-24">
                <Select
                  label={tCommon("unit")}
                  {...register(`items.${index}.unit`)}
                  options={[
                    { value: "EA", label: "EA" },
                    { value: "KG", label: "KG" },
                    { value: "L", label: "L" },
                    { value: "BOX", label: "BOX" },
                  ]}
                />
              </div>
              <div className="w-32">
                <Input
                  label={tCommon("price")}
                  type="number"
                  step="0.01"
                  required
                  error={errors.items?.[index]?.unit_price?.message}
                  {...register(`items.${index}.unit_price`)}
                />
              </div>
              <div className="w-32 text-right">
                <label className="mb-1 block text-sm font-medium text-gray-700">{tCommon("total")}</label>
                <p className="py-2 font-mono text-sm font-medium text-gray-900">
                  ${((items?.[index]?.quantity || 0) * (items?.[index]?.unit_price || 0)).toFixed(2)}
                </p>
              </div>
              <Button
                type="button"
                variant="ghost"
                size="icon"
                onClick={() => fields.length > 1 && remove(index)}
                disabled={fields.length <= 1}
              >
                <Trash2 className="h-4 w-4 text-red-500" />
              </Button>
            </div>
          ))}
        </div>

        <div className="mt-4 flex justify-end border-t pt-4">
          <div className="text-right">
            <p className="text-sm text-gray-500">{tCommon("total")} {tCommon("amount")}</p>
            <p className="text-2xl font-bold text-gray-900">${totalAmount.toFixed(2)}</p>
          </div>
        </div>
      </Card>

      <div className="flex justify-end gap-3">
        <Button type="button" variant="secondary" onClick={() => window.history.back()}>
          {tCommon("cancel")}
        </Button>
        <Button type="submit" loading={isLoading}>
          {t("createPo")}
        </Button>
      </div>
    </form>
  );
}
