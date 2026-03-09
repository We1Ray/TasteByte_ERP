"use client";

import { useRouter } from "next/navigation";
import { useForm, useFieldArray } from "react-hook-form";
import { standardSchemaResolver } from "@hookform/resolvers/standard-schema";
import { z } from "zod/v4";
import { ArrowLeft, Plus, Trash2 } from "lucide-react";
import { useTranslations } from "next-intl";
import { PageHeader } from "@/components/layout/page-header";
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { useToastMutation } from "@/lib/hooks/use-toast-mutation";
import { fiApi, type CreateJournalEntryInput } from "@/lib/api/fi";
import { formatCurrency } from "@/lib/utils";

const journalEntrySchema = z.object({
  posting_date: z.string().min(1, "Posting date is required"),
  document_date: z.string().min(1, "Document date is required"),
  reference: z.string().optional().default(""),
  description: z.string().optional().default(""),
  line_items: z
    .array(
      z.object({
        account_id: z.string().min(1, "Account is required"),
        debit: z.coerce.number().nonnegative("Debit must be non-negative").default(0),
        credit: z.coerce.number().nonnegative("Credit must be non-negative").default(0),
        description: z.string().optional().default(""),
      })
    )
    .min(2, "At least two line items are required"),
});

type JournalEntryFormData = z.infer<typeof journalEntrySchema>;

export default function NewJournalEntryPage() {
  const t = useTranslations("fi");
  const tCommon = useTranslations("common");

  const router = useRouter();

  const { data: accountsData } = useApiQuery(
    ["fi", "accounts", "all"],
    () => fiApi.getAccounts({ page: 1, page_size: 200 })
  );

  const {
    register,
    handleSubmit,
    control,
    watch,
    formState: { errors },
  } = useForm<JournalEntryFormData>({
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    resolver: standardSchemaResolver(journalEntrySchema) as any,
    defaultValues: {
      posting_date: new Date().toISOString().split("T")[0],
      document_date: new Date().toISOString().split("T")[0],
      reference: "",
      description: "",
      line_items: [
        { account_id: "", debit: 0, credit: 0, description: "" },
        { account_id: "", debit: 0, credit: 0, description: "" },
      ],
    },
  });

  const { fields, append, remove } = useFieldArray({
    control,
    name: "line_items",
  });

  const watchItems = watch("line_items");

  const createMutation = useToastMutation(
    (data: JournalEntryFormData) => {
      const input: CreateJournalEntryInput = {
        posting_date: data.posting_date,
        document_date: data.document_date,
        reference: data.reference || undefined,
        description: data.description || undefined,
        line_items: data.line_items.map((item) => ({
          account_id: item.account_id,
          debit: item.debit || 0,
          credit: item.credit || 0,
          description: item.description || "",
        })),
      };
      return fiApi.createJournalEntry(input);
    },
    {
      successMessage: "Journal entry created successfully",
      invalidateKeys: ["fi", "journal"],
      onSuccess: () => {
        router.push("/fi/journal");
      },
    }
  );

  const accounts = accountsData?.items || [];

  const totalDebit = watchItems.reduce((sum, item) => sum + (Number(item.debit) || 0), 0);
  const totalCredit = watchItems.reduce((sum, item) => sum + (Number(item.credit) || 0), 0);
  const isBalanced = Math.abs(totalDebit - totalCredit) < 0.01;

  return (
    <div>
      <PageHeader
        title={t("newEntry")}
        description={t("manageJournal")}
        actions={
          <Button variant="secondary" onClick={() => router.push("/fi/journal")}>
            <ArrowLeft className="h-4 w-4" />
            {tCommon("back")}
          </Button>
        }
      />

      <form onSubmit={handleSubmit((data) => createMutation.mutate(data))} className="space-y-6">
        <Card>
          <h3 className="mb-4 text-sm font-semibold uppercase text-gray-500">{t("reference")}</h3>
          <div className="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-4">
            <Input
              label={t("postingDate")}
              type="date"
              required
              error={errors.posting_date?.message}
              {...register("posting_date")}
            />
            <Input
              label={t("documentNo")}
              type="date"
              required
              error={errors.document_date?.message}
              {...register("document_date")}
            />
            <Input
              label={t("reference")}
              error={errors.reference?.message}
              {...register("reference")}
              placeholder="e.g., INV-001"
            />
            <div className="lg:col-span-1">
              <Input
                label={tCommon("description")}
                error={errors.description?.message}
                {...register("description")}
                placeholder="Entry description..."
              />
            </div>
          </div>
        </Card>

        <Card>
          <div className="mb-4 flex items-center justify-between">
            <h3 className="text-sm font-semibold uppercase text-gray-500">{t("debit")} / {t("credit")}</h3>
            <Button
              type="button"
              variant="secondary"
              size="sm"
              onClick={() => append({ account_id: "", debit: 0, credit: 0, description: "" })}
            >
              <Plus className="h-4 w-4" />
              Add Line
            </Button>
          </div>

          {errors.line_items?.root?.message && (
            <p className="mb-4 text-sm text-red-600">{errors.line_items.root.message}</p>
          )}

          <div className="space-y-3">
            {fields.map((field, index) => (
              <div key={field.id} className="flex items-start gap-3 rounded-md border border-gray-200 bg-gray-50 p-3">
                <div className="flex-1">
                  <Select
                    label={t("accountName")}
                    required
                    error={errors.line_items?.[index]?.account_id?.message}
                    placeholder="Select account"
                    options={accounts.map((a) => ({
                      value: a.id,
                      label: `${a.account_number} - ${a.name}`,
                    }))}
                    {...register(`line_items.${index}.account_id`)}
                  />
                </div>
                <div className="w-36">
                  <Input
                    label={t("debit")}
                    type="number"
                    step="0.01"
                    min={0}
                    error={errors.line_items?.[index]?.debit?.message}
                    {...register(`line_items.${index}.debit`)}
                  />
                </div>
                <div className="w-36">
                  <Input
                    label={t("credit")}
                    type="number"
                    step="0.01"
                    min={0}
                    error={errors.line_items?.[index]?.credit?.message}
                    {...register(`line_items.${index}.credit`)}
                  />
                </div>
                <div className="flex-1">
                  <Input
                    label={tCommon("description")}
                    error={errors.line_items?.[index]?.description?.message}
                    {...register(`line_items.${index}.description`)}
                    placeholder="Line description"
                  />
                </div>
                <div className="pt-6">
                  <Button
                    type="button"
                    variant="ghost"
                    size="icon"
                    onClick={() => fields.length > 2 && remove(index)}
                    disabled={fields.length <= 2}
                  >
                    <Trash2 className="h-4 w-4 text-red-500" />
                  </Button>
                </div>
              </div>
            ))}
          </div>

          <div className="mt-4 border-t pt-4">
            <div className="flex justify-end gap-8">
              <div className="text-right">
                <p className="text-sm text-gray-500">{t("debit")}</p>
                <p className="text-lg font-bold text-gray-900">{formatCurrency(totalDebit)}</p>
              </div>
              <div className="text-right">
                <p className="text-sm text-gray-500">{t("credit")}</p>
                <p className="text-lg font-bold text-gray-900">{formatCurrency(totalCredit)}</p>
              </div>
              <div className="text-right">
                <p className="text-sm text-gray-500">{t("balance")}</p>
                <p className={`text-lg font-bold ${isBalanced ? "text-green-600" : "text-red-600"}`}>
                  {formatCurrency(Math.abs(totalDebit - totalCredit))}
                </p>
              </div>
            </div>
            {!isBalanced && (
              <p className="mt-2 text-right text-sm text-red-600">
                Debits and credits must balance before posting
              </p>
            )}
          </div>
        </Card>

        <div className="flex justify-end gap-3">
          <Button type="button" variant="secondary" onClick={() => router.push("/fi/journal")}>
            {tCommon("cancel")}
          </Button>
          <Button type="submit" loading={createMutation.isPending}>
            {t("newEntry")}
          </Button>
        </div>
      </form>
    </div>
  );
}
