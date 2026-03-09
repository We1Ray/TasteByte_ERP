"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import { useForm, useFieldArray, type Resolver } from "react-hook-form";
import { standardSchemaResolver } from "@hookform/resolvers/standard-schema";
import { z } from "zod/v4";
import { ArrowLeft, Plus, Trash2 } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { useToastMutation } from "@/lib/hooks/use-toast-mutation";
import { sdApi, type CreateSalesOrderInput } from "@/lib/api/sd";
import { mmApi } from "@/lib/api/mm";

const salesOrderSchema = z.object({
  customer_id: z.string().min(1, "Customer is required"),
  order_date: z.string().min(1, "Order date is required"),
  delivery_date: z.string().optional().default(""),
  notes: z.string().optional().default(""),
  items: z
    .array(
      z.object({
        material_id: z.string().min(1, "Material is required"),
        quantity: z.coerce.number().positive("Quantity must be greater than 0"),
        unit_price: z.coerce.number().nonnegative("Unit price must be non-negative"),
      })
    )
    .min(1, "At least one line item is required"),
});

type SalesOrderFormData = z.infer<typeof salesOrderSchema>;

export default function NewSalesOrderPage() {
  const router = useRouter();
  const t = useTranslations("sd");
  const tc = useTranslations("common");
  const tp = useTranslations("pp");
  const tShared = useTranslations("shared");
  const [customerSearch] = useState("");

  const { data: customersData } = useApiQuery(
    ["sd", "customers", "all", customerSearch],
    () => sdApi.getCustomers({ page: 1, page_size: 100, search: customerSearch || undefined })
  );

  const { data: materialsData } = useApiQuery(
    ["mm", "materials", "all"],
    () => mmApi.getMaterials({ page: 1, page_size: 200 })
  );

  const {
    register,
    handleSubmit,
    control,
    watch,
    formState: { errors },
  } = useForm<SalesOrderFormData>({
    resolver: standardSchemaResolver(salesOrderSchema) as Resolver<SalesOrderFormData>,
    defaultValues: {
      customer_id: "",
      order_date: new Date().toISOString().split("T")[0],
      delivery_date: "",
      notes: "",
      items: [{ material_id: "", quantity: 1, unit_price: 0 }],
    },
  });

  const { fields, append, remove } = useFieldArray({
    control,
    name: "items",
  });

  const watchItems = watch("items");

  const createMutation = useToastMutation(
    (data: SalesOrderFormData) => {
      const input: CreateSalesOrderInput = {
        customer_id: data.customer_id,
        order_date: data.order_date,
        delivery_date: data.delivery_date || undefined,
        items: data.items.map((item) => ({
          material_id: item.material_id,
          quantity: item.quantity || 0,
          unit_price: item.unit_price || 0,
        })),
      };
      return sdApi.createSalesOrder(input);
    },
    {
      successMessage: "Sales order created successfully",
      invalidateKeys: ["sd", "sales-orders"],
      onSuccess: () => {
        router.push("/sd/sales-orders");
      },
    }
  );

  const customers = customersData?.items || [];
  const materials = materialsData?.items || [];

  const totalAmount = watchItems.reduce(
    (sum, item) => sum + (item.quantity || 0) * (item.unit_price || 0),
    0
  );

  return (
    <div>
      <PageHeader
        title={t("createSalesOrder")}
        description={t("createSalesOrderDesc")}
        actions={
          <Button variant="secondary" onClick={() => router.push("/sd/sales-orders")}>
            <ArrowLeft className="h-4 w-4" />
            {tc("back")}
          </Button>
        }
      />

      <form onSubmit={handleSubmit((data) => createMutation.mutate(data))} className="space-y-6">
        <Card>
          <h3 className="mb-4 text-sm font-semibold uppercase text-gray-500">Order Information</h3>
          <div className="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
            <Select
              label={t("customer")}
              required
              error={errors.customer_id?.message}
              placeholder="Select a customer"
              options={customers.map((c) => ({
                value: c.id,
                label: `${c.customer_number} - ${c.name}`,
              }))}
              {...register("customer_id")}
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
              error={errors.delivery_date?.message}
              {...register("delivery_date")}
            />
            <div className="md:col-span-2 lg:col-span-3">
              <label className="mb-1 block text-sm font-medium text-gray-700">{tc("notes")}</label>
              <textarea
                className="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm placeholder:text-gray-400 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                rows={3}
                placeholder="Order notes..."
                {...register("notes")}
              />
            </div>
          </div>
        </Card>

        <Card>
          <div className="mb-4 flex items-center justify-between">
            <h3 className="text-sm font-semibold uppercase text-gray-500">{tShared("lineItems")}</h3>
            <Button
              type="button"
              variant="secondary"
              size="sm"
              onClick={() => append({ material_id: "", quantity: 1, unit_price: 0 })}
            >
              <Plus className="h-4 w-4" />
              {tc("create")}
            </Button>
          </div>

          {errors.items?.root?.message && (
            <p className="mb-4 text-sm text-red-600">{errors.items.root.message}</p>
          )}

          <div className="space-y-3">
            {fields.map((field, index) => (
              <div key={field.id} className="flex items-start gap-3 rounded-md border border-gray-200 bg-gray-50 p-3">
                <div className="flex-1">
                  <Select
                    label={tp("material")}
                    required
                    error={errors.items?.[index]?.material_id?.message}
                    placeholder={tp("selectMaterial")}
                    options={materials.map((m) => ({
                      value: m.id,
                      label: `${m.material_number} - ${m.name}`,
                    }))}
                    {...register(`items.${index}.material_id`)}
                  />
                </div>
                <div className="w-32">
                  <Input
                    label={tc("quantity")}
                    type="number"
                    min={1}
                    required
                    error={errors.items?.[index]?.quantity?.message}
                    {...register(`items.${index}.quantity`)}
                  />
                </div>
                <div className="w-36">
                  <Input
                    label={tc("price")}
                    type="number"
                    step="0.01"
                    min={0}
                    required
                    error={errors.items?.[index]?.unit_price?.message}
                    {...register(`items.${index}.unit_price`)}
                  />
                </div>
                <div className="w-32 pt-6">
                  <p className="text-sm font-medium text-gray-700">
                    ${((watchItems[index]?.quantity || 0) * (watchItems[index]?.unit_price || 0)).toFixed(2)}
                  </p>
                </div>
                <div className="pt-6">
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
              </div>
            ))}
          </div>

          <div className="mt-4 flex justify-end border-t pt-4">
            <div className="text-right">
              <p className="text-sm text-gray-500">{tc("amount")}</p>
              <p className="text-xl font-bold text-gray-900">${totalAmount.toFixed(2)}</p>
            </div>
          </div>
        </Card>

        <div className="flex justify-end gap-3">
          <Button type="button" variant="secondary" onClick={() => router.push("/sd/sales-orders")}>
            {tc("cancel")}
          </Button>
          <Button type="submit" loading={createMutation.isPending}>
            {t("createSalesOrder")}
          </Button>
        </div>
      </form>
    </div>
  );
}
