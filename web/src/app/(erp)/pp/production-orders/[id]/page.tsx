"use client";

import { use } from "react";
import { useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import { DetailPageLayout } from "@/components/layout/detail-page-layout";
import { DocumentActions } from "@/components/shared/document-actions";
import { PrintButton } from "@/components/shared/print-button";
import { StatusStepper, useStatusSteps } from "@/components/shared/status-stepper";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { DescriptionList } from "@/components/ui/description-list";
import { StatusBadge, Badge } from "@/components/ui/badge";
import { WorkflowTimeline } from "@/components/shared/workflow-timeline";
import { useApiQuery, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { useToastMutation } from "@/lib/hooks/use-toast-mutation";
import { ppApi } from "@/lib/api/pp";
import { formatDate, formatNumber } from "@/lib/utils";

export default function ProductionOrderDetailPage({ params }: { params: Promise<{ id: string }> }) {
  const { id } = use(params);
  const router = useRouter();
  const t = useTranslations("pp");
  const tc = useTranslations("common");
  const invalidate = useInvalidateQueries();
  const { productionSteps } = useStatusSteps();

  const { data: order, isLoading } = useApiQuery(
    ["pp", "production-orders", id],
    () => ppApi.getProductionOrder(id)
  );

  const statusMutation = useToastMutation(
    (status: string) => ppApi.updateProductionOrderStatus(id, { status }),
    {
      successMessage: "Production order status updated",
      invalidateKeys: ["pp", "production-orders"],
      onSuccess: () => {
        invalidate(["pp", "production-orders", id]);
      },
    }
  );

  const handleAction = (action: string) => {
    switch (action) {
      case "release":
        statusMutation.mutate("RELEASED");
        break;
      case "start":
        statusMutation.mutate("IN_PROGRESS");
        break;
      case "complete":
        statusMutation.mutate("COMPLETED");
        break;
    }
  };

  if (!order && !isLoading) {
    return (
      <div className="py-12 text-center">
        <p className="text-gray-500">{t("noProductionOrdersFound")}</p>
      </div>
    );
  }

  return (
    <DetailPageLayout
      title={order ? `${t("productionOrders")} ${order.order_number}` : t("productionOrders")}
      subtitle={order ? `${t("material")}: ${order.material_name}` : undefined}
      isLoading={isLoading}
      onBack={() => router.push("/pp/production-orders")}
      actions={
        <>
          <PrintButton />
          {order && (
            <DocumentActions
              status={order.status}
              documentType="production_order"
              onAction={handleAction}
              isLoading={statusMutation.isPending}
            />
          )}
        </>
      }
      sidebar={order ? [
        {
          title: tc("status"),
          content: (
            <StatusStepper steps={productionSteps} currentStatus={order.status} orientation="vertical" />
          ),
        },
        {
          title: t("billOfMaterials"),
          content: (
            <DescriptionList
              items={[
                { label: "BOM ID", value: <span className="font-mono text-xs">{order.bom_id}</span> },
                { label: tc("createdAt"), value: formatDate(order.created_at) },
              ]}
            />
          ),
        },
        {
          title: "Workflow",
          content: <WorkflowTimeline documentType="PRODUCTION_ORDER" documentId={id} />,
        },
      ] : undefined}
    >
      {order && (
        <Card>
          <CardHeader>
            <CardTitle>{t("productionOrders")}</CardTitle>
          </CardHeader>
          <DescriptionList
            layout="grid"
            columns={2}
            items={[
              { label: t("material"), value: <><span className="font-medium">{order.material_name ?? "-"}</span><br /><span className="text-xs text-gray-500">{order.material_number ?? ""}</span></> },
              { label: tc("quantity"), value: `${formatNumber(order.quantity ?? order.planned_quantity ?? 0)} ${order.unit ?? ""}` },
              { label: t("completed"), value: <Badge color={(order.completed_quantity ?? order.actual_quantity ?? 0) >= (order.quantity ?? order.planned_quantity ?? 0) ? "green" : "amber"}>{formatNumber(order.completed_quantity ?? order.actual_quantity ?? 0)} / {formatNumber(order.quantity ?? order.planned_quantity ?? 0)}</Badge> },
              { label: tc("status"), value: <StatusBadge status={order.status} /> },
              { label: t("plannedStart"), value: formatDate(order.planned_start) },
              { label: t("plannedEnd"), value: formatDate(order.planned_end) },
              { label: "Actual Start", value: order.actual_start ? formatDate(order.actual_start) : "-", hidden: !order.actual_start },
              { label: "Actual End", value: order.actual_end ? formatDate(order.actual_end) : "-", hidden: !order.actual_end },
            ]}
          />
        </Card>
      )}
    </DetailPageLayout>
  );
}
