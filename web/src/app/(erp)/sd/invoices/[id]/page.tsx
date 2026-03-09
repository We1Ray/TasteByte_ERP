"use client";

import { use } from "react";
import { useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import { ArrowLeft } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { StatusStepper, useStatusSteps } from "@/components/shared/status-stepper";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { DescriptionList } from "@/components/ui/description-list";
import { StatusBadge } from "@/components/ui/badge";
import { PageLoading } from "@/components/ui/loading";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { sdApi } from "@/lib/api/sd";
import { formatCurrency, formatDate } from "@/lib/utils";

export default function InvoiceDetailPage({ params }: { params: Promise<{ id: string }> }) {
  const { id } = use(params);
  const router = useRouter();
  const t = useTranslations("sd");
  const tc = useTranslations("common");
  const { invoiceSteps } = useStatusSteps();

  const { data: invoice, isLoading } = useApiQuery(
    ["sd", "invoices", id],
    () => sdApi.getInvoice(id)
  );

  if (isLoading) {
    return <PageLoading />;
  }

  if (!invoice) {
    return (
      <div className="py-12 text-center">
        <p className="text-gray-500">{t("noInvoicesFound")}</p>
        <Button variant="link" onClick={() => router.push("/sd/invoices")} className="mt-2">
          {tc("back")}
        </Button>
      </div>
    );
  }

  const outstanding = invoice.total_amount - invoice.paid_amount;

  return (
    <div>
      <PageHeader
        title={`${t("invoices")} ${invoice.invoice_number}`}
        description={`${t("customer")}: ${invoice.customer_name}`}
        actions={
          <Button variant="secondary" onClick={() => router.push("/sd/invoices")}>
            <ArrowLeft className="h-4 w-4" />
            {tc("back")}
          </Button>
        }
      />

      <div className="grid grid-cols-1 gap-6 lg:grid-cols-3">
        <Card className="lg:col-span-2">
          <CardHeader>
            <CardTitle>{t("invoices")}</CardTitle>
          </CardHeader>
          <div className="grid grid-cols-2 gap-6 md:grid-cols-4">
            <div>
              <p className="text-sm text-gray-500">{tc("amount")}</p>
              <p className="text-xl font-bold text-gray-900">
                {formatCurrency(invoice.total_amount, invoice.currency)}
              </p>
            </div>
            <div>
              <p className="text-sm text-gray-500">{t("paid")}</p>
              <p className="text-xl font-bold text-green-600">
                {formatCurrency(invoice.paid_amount, invoice.currency)}
              </p>
            </div>
            <div>
              <p className="text-sm text-gray-500">Outstanding</p>
              <p className={`text-xl font-bold ${outstanding > 0 ? "text-amber-600" : "text-green-600"}`}>
                {formatCurrency(outstanding, invoice.currency)}
              </p>
            </div>
            <div>
              <p className="text-sm text-gray-500">{tc("status")}</p>
              <div className="mt-1">
                <StatusBadge status={invoice.status} />
              </div>
            </div>
          </div>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>{t("invoices")}</CardTitle>
          </CardHeader>
          <div className="mb-6">
            <StatusStepper steps={invoiceSteps} currentStatus={invoice.status} orientation="vertical" />
          </div>
          <DescriptionList
            items={[
              { label: tc("status"), value: <StatusBadge status={invoice.status} /> },
              { label: t("invoiceDate"), value: formatDate(invoice.invoice_date) },
              { label: t("dueDate"), value: formatDate(invoice.due_date) },
              { label: t("customer"), value: invoice.customer_name },
              { label: "Currency", value: invoice.currency },
              {
                label: t("salesOrders"),
                value: (
                  <Button
                    variant="link"
                    size="sm"
                    onClick={() => router.push(`/sd/sales-orders/${invoice.sales_order_id}`)}
                  >
                    View Order
                  </Button>
                ),
                hidden: !invoice.sales_order_id,
              },
              { label: tc("createdAt"), value: formatDate(invoice.created_at) },
            ]}
          />
        </Card>
      </div>
    </div>
  );
}
