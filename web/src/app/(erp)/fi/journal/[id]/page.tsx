"use client";

import { use } from "react";
import { useRouter } from "next/navigation";
import { ArrowLeft, CheckCircle } from "lucide-react";
import { useTranslations } from "next-intl";
import { PageHeader } from "@/components/layout/page-header";
import { StatusStepper, useStatusSteps } from "@/components/shared/status-stepper";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { DescriptionList } from "@/components/ui/description-list";
import { StatusBadge } from "@/components/ui/badge";
import { PageLoading } from "@/components/ui/loading";
import { useApiQuery, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { useToastMutation } from "@/lib/hooks/use-toast-mutation";
import { fiApi } from "@/lib/api/fi";
import { formatCurrency, formatDate } from "@/lib/utils";

export default function JournalEntryDetailPage({ params }: { params: Promise<{ id: string }> }) {
  const { id } = use(params);
  const router = useRouter();
  const invalidate = useInvalidateQueries();
  const { journalSteps } = useStatusSteps();
  const t = useTranslations("fi");
  const tCommon = useTranslations("common");
  const tShared = useTranslations("shared");

  const { data: entry, isLoading } = useApiQuery(
    ["fi", "journal", id],
    () => fiApi.getJournalEntry(id)
  );

  const postMutation = useToastMutation(
    () => fiApi.postJournalEntry(id),
    {
      successMessage: "Journal entry posted successfully",
      invalidateKeys: ["fi", "journal"],
      onSuccess: () => {
        invalidate(["fi", "journal", id]);
      },
    }
  );

  if (isLoading) {
    return <PageLoading />;
  }

  if (!entry) {
    return (
      <div className="py-12 text-center">
        <p className="text-gray-500">{t("noJournalFound")}</p>
        <Button variant="link" onClick={() => router.push("/fi/journal")} className="mt-2">
          {tCommon("back")}
        </Button>
      </div>
    );
  }

  return (
    <div>
      <PageHeader
        title={`${t("journalEntries")} ${entry.document_number}`}
        description={entry.description || t("journalEntries")}
        actions={
          <div className="flex gap-2">
            <Button variant="secondary" onClick={() => router.push("/fi/journal")}>
              <ArrowLeft className="h-4 w-4" />
              {tCommon("back")}
            </Button>
            {entry.status === "Draft" && (
              <Button
                onClick={() => postMutation.mutate(undefined)}
                loading={postMutation.isPending}
              >
                <CheckCircle className="h-4 w-4" />
                {tShared("postEntry")}
              </Button>
            )}
          </div>
        }
      />

      <div className="grid grid-cols-1 gap-6 lg:grid-cols-3">
        <Card className="lg:col-span-2">
          <CardHeader>
            <CardTitle>{t("debit")} / {t("credit")}</CardTitle>
          </CardHeader>
          <div className="-mx-6 -mb-6">
            <table className="w-full text-sm">
              <thead className="border-b border-t bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-semibold uppercase text-gray-500">{t("accountName")}</th>
                  <th className="px-6 py-3 text-left text-xs font-semibold uppercase text-gray-500">{tCommon("description")}</th>
                  <th className="px-6 py-3 text-right text-xs font-semibold uppercase text-gray-500">{t("debit")}</th>
                  <th className="px-6 py-3 text-right text-xs font-semibold uppercase text-gray-500">{t("credit")}</th>
                </tr>
              </thead>
              <tbody className="divide-y">
                {entry.line_items?.map((item) => (
                  <tr key={item.id} className="hover:bg-gray-50">
                    <td className="px-6 py-3">
                      <p className="font-medium text-gray-900">{item.account_name}</p>
                      <p className="text-xs text-gray-500">{item.account_number}</p>
                    </td>
                    <td className="px-6 py-3 text-gray-600">{item.description || "-"}</td>
                    <td className="px-6 py-3 text-right font-mono">
                      {item.debit > 0 ? formatCurrency(item.debit) : "-"}
                    </td>
                    <td className="px-6 py-3 text-right font-mono">
                      {item.credit > 0 ? formatCurrency(item.credit) : "-"}
                    </td>
                  </tr>
                ))}
              </tbody>
              <tfoot className="border-t-2 bg-gray-50 font-semibold">
                <tr>
                  <td colSpan={2} className="px-6 py-3 text-right text-gray-900">{tCommon("total")}</td>
                  <td className="px-6 py-3 text-right font-mono text-gray-900">
                    {formatCurrency(entry.total_debit)}
                  </td>
                  <td className="px-6 py-3 text-right font-mono text-gray-900">
                    {formatCurrency(entry.total_credit)}
                  </td>
                </tr>
              </tfoot>
            </table>
          </div>
        </Card>

        <div className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle>{t("journalEntries")}</CardTitle>
            </CardHeader>
            <div className="mb-6">
              <StatusStepper steps={journalSteps} currentStatus={entry.status} orientation="vertical" />
            </div>
            <DescriptionList
              items={[
                { label: tCommon("status"), value: <StatusBadge status={entry.status} /> },
                { label: t("postingDate"), value: formatDate(entry.posting_date) },
                { label: t("documentNo"), value: formatDate(entry.document_date) },
                { label: t("reference"), value: entry.reference, hidden: !entry.reference },
                { label: tCommon("description"), value: entry.description, hidden: !entry.description },
                { label: tCommon("createdAt"), value: formatDate(entry.created_at) },
              ]}
            />
          </Card>
        </div>
      </div>
    </div>
  );
}
