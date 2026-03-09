"use client";

import { use } from "react";
import { useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import { useApiQuery, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { useToastMutation } from "@/lib/hooks/use-toast-mutation";
import { sdApi } from "@/lib/api/sd";
import { DetailPageLayout } from "@/components/layout/detail-page-layout";
import { DocumentActions } from "@/components/shared/document-actions";
import { PrintButton } from "@/components/shared/print-button";
import { StatusStepper, useStatusSteps } from "@/components/shared/status-stepper";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { DescriptionList } from "@/components/ui/description-list";
import { StatusBadge, Badge } from "@/components/ui/badge";
import { WorkflowTimeline } from "@/components/shared/workflow-timeline";
import { formatCurrency, formatDate } from "@/lib/utils";

export default function SalesOrderDetailPage({ params }: { params: Promise<{ id: string }> }) {
  const { id } = use(params);
  const router = useRouter();
  const t = useTranslations("sd");
  const tc = useTranslations("common");
  const ts = useTranslations("shared");
  const invalidate = useInvalidateQueries();
  const { soSteps } = useStatusSteps();

  const { data: order, isLoading } = useApiQuery(
    ["sd", "sales-orders", id],
    () => sdApi.getSalesOrder(id)
  );

  const confirmMutation = useToastMutation(
    () => sdApi.confirmSalesOrder(id),
    {
      successMessage: "Sales order confirmed",
      invalidateKeys: ["sd", "sales-orders"],
      onSuccess: () => {
        invalidate(["sd", "sales-orders", id]);
      },
    }
  );

  const handleAction = (action: string) => {
    switch (action) {
      case "confirm":
        confirmMutation.mutate(undefined);
        break;
      case "create_delivery":
        router.push(`/sd/sales-orders/${id}/delivery`);
        break;
      case "create_invoice":
        router.push(`/sd/sales-orders/${id}/invoice`);
        break;
      case "complete":
        break;
    }
  };

  if (!order && !isLoading) {
    return (
      <div className="py-12 text-center">
        <p className="text-gray-500">{t("noSalesOrdersFound")}</p>
      </div>
    );
  }

  return (
    <DetailPageLayout
      title={order ? `${t("salesOrders")} ${order.order_number}` : t("salesOrders")}
      subtitle={order ? `${t("customer")}: ${order.customer_name}` : undefined}
      isLoading={isLoading}
      onBack={() => router.push("/sd/sales-orders")}
      actions={
        <>
          <PrintButton />
          {order && (
            <DocumentActions
              status={order.status}
              documentType="sales_order"
              onAction={handleAction}
              isLoading={confirmMutation.isPending}
            />
          )}
        </>
      }
      sidebar={order ? [
        {
          title: tc("status"),
          content: (
            <StatusStepper steps={soSteps} currentStatus={order.status} orientation="vertical" />
          ),
        },
        {
          title: t("salesOrders"),
          content: (
            <DescriptionList
              items={[
                { label: tc("status"), value: <StatusBadge status={order.status} /> },
                { label: t("orderDate"), value: formatDate(order.order_date) },
                { label: t("deliveryDate"), value: formatDate(order.delivery_date) },
                { label: "Currency", value: order.currency },
                { label: tc("createdAt"), value: formatDate(order.created_at) },
              ]}
            />
          ),
        },
        {
          title: "Workflow",
          content: <WorkflowTimeline documentType="SALES_ORDER" documentId={id} />,
        },
      ] : undefined}
    >
      {order && (
        <Card>
          <CardHeader>
            <CardTitle>{ts("lineItems")}</CardTitle>
          </CardHeader>
          <div className="-mx-6 -mb-6">
            <table className="w-full text-sm">
              <thead className="border-b border-t bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-semibold uppercase text-gray-500">{tc("name")}</th>
                  <th className="px-6 py-3 text-right text-xs font-semibold uppercase text-gray-500">{tc("quantity")}</th>
                  <th className="px-6 py-3 text-left text-xs font-semibold uppercase text-gray-500">{tc("unit")}</th>
                  <th className="px-6 py-3 text-right text-xs font-semibold uppercase text-gray-500">{tc("price")}</th>
                  <th className="px-6 py-3 text-right text-xs font-semibold uppercase text-gray-500">{tc("total")}</th>
                  <th className="px-6 py-3 text-right text-xs font-semibold uppercase text-gray-500">{ts("delivered")}</th>
                </tr>
              </thead>
              <tbody className="divide-y">
                {order.items?.map((item) => (
                  <tr key={item.id} className="hover:bg-gray-50">
                    <td className="px-6 py-3">
                      <p className="font-medium text-gray-900">{item.material_name}</p>
                      <p className="text-xs text-gray-500">{item.material_number}</p>
                    </td>
                    <td className="px-6 py-3 text-right font-mono">{item.quantity}</td>
                    <td className="px-6 py-3 text-gray-500">{item.unit}</td>
                    <td className="px-6 py-3 text-right font-mono">{formatCurrency(item.unit_price)}</td>
                    <td className="px-6 py-3 text-right font-mono font-medium">{formatCurrency(item.total_price)}</td>
                    <td className="px-6 py-3 text-right">
                      <Badge color={item.delivered_quantity >= item.quantity ? "green" : "amber"}>
                        {item.delivered_quantity} / {item.quantity}
                      </Badge>
                    </td>
                  </tr>
                ))}
              </tbody>
              <tfoot className="border-t-2 bg-gray-50 font-semibold">
                <tr>
                  <td colSpan={4} className="px-6 py-3 text-right text-gray-900">{tc("amount")}</td>
                  <td className="px-6 py-3 text-right font-mono text-gray-900">{formatCurrency(order.total_amount)}</td>
                  <td />
                </tr>
              </tfoot>
            </table>
          </div>
        </Card>
      )}
    </DetailPageLayout>
  );
}
