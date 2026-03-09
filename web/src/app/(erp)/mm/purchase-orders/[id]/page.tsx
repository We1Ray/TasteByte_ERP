"use client";

import { use, useState } from "react";
import { useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import { Package } from "lucide-react";
import { DetailPageLayout } from "@/components/layout/detail-page-layout";
import { DocumentActions } from "@/components/shared/document-actions";
import { PrintButton } from "@/components/shared/print-button";
import { StatusStepper, useStatusSteps } from "@/components/shared/status-stepper";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { DescriptionList } from "@/components/ui/description-list";
import { StatusBadge, Badge } from "@/components/ui/badge";
import { WorkflowTimeline } from "@/components/shared/workflow-timeline";
import { useApiQuery, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { useToastMutation } from "@/lib/hooks/use-toast-mutation";
import { mmApi, type PurchaseOrderItem } from "@/lib/api/mm";
import { wmApi } from "@/lib/api/wm";
import { formatCurrency, formatDate } from "@/lib/utils";

export default function PurchaseOrderDetailPage({ params }: { params: Promise<{ id: string }> }) {
  const { id } = use(params);
  const router = useRouter();
  const t = useTranslations("mm");
  const tCommon = useTranslations("common");
  const tShared = useTranslations("shared");
  const invalidate = useInvalidateQueries();
  const { poSteps } = useStatusSteps();
  const [showReceive, setShowReceive] = useState(false);
  const [receiveItems, setReceiveItems] = useState<Record<string, { quantity: number; warehouse_id: string }>>({});

  const { data: po, isLoading } = useApiQuery(
    ["mm", "purchase-orders", id],
    () => mmApi.getPurchaseOrder(id)
  );

  const { data: warehouses } = useApiQuery(
    ["wm", "warehouses"],
    () => wmApi.getWarehouses({ page: 1, page_size: 100 }),
    { enabled: showReceive }
  );

  const receiveMutation = useToastMutation(
    (items: { po_item_id: string; quantity: number; warehouse_id: string }[]) =>
      mmApi.receivePurchaseOrder(id, items),
    {
      successMessage: tShared("goodsReceipt"),
      invalidateKeys: ["mm", "purchase-orders"],
      onSuccess: () => {
        invalidate(["mm", "purchase-orders", id]);
        setShowReceive(false);
        setReceiveItems({});
      },
    }
  );

  const initReceiveItems = (items: PurchaseOrderItem[]) => {
    const initial: Record<string, { quantity: number; warehouse_id: string }> = {};
    for (const item of items) {
      const remaining = item.quantity - item.received_quantity;
      if (remaining > 0) {
        initial[item.id] = { quantity: remaining, warehouse_id: "" };
      }
    }
    setReceiveItems(initial);
  };

  const handleReceive = () => {
    const items = Object.entries(receiveItems)
      .filter(([, v]) => v.quantity > 0 && v.warehouse_id)
      .map(([po_item_id, v]) => ({
        po_item_id,
        quantity: v.quantity,
        warehouse_id: v.warehouse_id,
      }));
    if (items.length > 0) {
      receiveMutation.mutate(items);
    }
  };

  const handleAction = (action: string) => {
    switch (action) {
      case "goods_receipt":
        if (po?.items) {
          setShowReceive(true);
          initReceiveItems(po.items);
        }
        break;
    }
  };

  if (!po && !isLoading) {
    return (
      <div className="py-12 text-center">
        <p className="text-gray-500">{t("noPurchaseOrdersFound")}</p>
      </div>
    );
  }

  return (
    <DetailPageLayout
      title={po ? `${t("purchaseOrders")} ${po.po_number}` : t("purchaseOrders")}
      subtitle={po ? `Vendor: ${po.vendor_name}` : undefined}
      isLoading={isLoading}
      onBack={() => router.push("/mm/purchase-orders")}
      actions={
        <>
          <PrintButton />
          {po && !showReceive && (
            <DocumentActions
              status={po.status}
              documentType="purchase_order"
              onAction={handleAction}
            />
          )}
        </>
      }
      sidebar={po ? [
        {
          title: tCommon("status"),
          content: (
            <StatusStepper steps={poSteps} currentStatus={po.status} orientation="vertical" />
          ),
        },
        {
          title: t("orderDate"),
          content: (
            <DescriptionList
              items={[
                { label: tCommon("status"), value: <StatusBadge status={po.status} /> },
                { label: t("orderDate"), value: formatDate(po.order_date) },
                { label: t("deliveryDate"), value: formatDate(po.delivery_date) },
                { label: "Currency", value: po.currency },
                { label: tCommon("createdAt"), value: formatDate(po.created_at) },
              ]}
            />
          ),
        },
        {
          title: tShared("transitionHistory"),
          content: <WorkflowTimeline documentType="PURCHASE_ORDER" documentId={id} />,
        },
      ] : undefined}
    >
      {/* Receive Form */}
      {showReceive && po && (
        <Card className="mb-6">
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Package className="h-5 w-5" />
              {tShared("goodsReceipt")}
            </CardTitle>
          </CardHeader>
          <div className="space-y-4">
            {po.items?.filter((item) => item.quantity - item.received_quantity > 0).map((item) => (
              <div key={item.id} className="flex items-center gap-4 rounded-lg border border-gray-200 p-4">
                <div className="flex-1">
                  <p className="font-medium text-gray-900">{item.material_name}</p>
                  <p className="text-xs text-gray-500">{item.material_number} - Remaining: {item.quantity - item.received_quantity} {item.unit}</p>
                </div>
                <Input
                  label={tCommon("quantity")}
                  type="number"
                  value={receiveItems[item.id]?.quantity ?? 0}
                  min={0}
                  max={item.quantity - item.received_quantity}
                  onChange={(e) =>
                    setReceiveItems((prev) => ({
                      ...prev,
                      [item.id]: { ...prev[item.id], quantity: Number(e.target.value) },
                    }))
                  }
                  className="w-24"
                />
                <div className="w-48">
                  <label className="mb-1 block text-sm font-medium text-gray-700">{t("warehouse")}</label>
                  <select
                    className="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                    value={receiveItems[item.id]?.warehouse_id ?? ""}
                    onChange={(e) =>
                      setReceiveItems((prev) => ({
                        ...prev,
                        [item.id]: { ...prev[item.id], warehouse_id: e.target.value },
                      }))
                    }
                  >
                    <option value="">Select...</option>
                    {warehouses?.items?.map((wh) => (
                      <option key={wh.id} value={wh.id}>{wh.name}</option>
                    ))}
                  </select>
                </div>
              </div>
            ))}
            <div className="flex gap-2">
              <Button
                onClick={handleReceive}
                loading={receiveMutation.isPending}
                disabled={Object.values(receiveItems).every((v) => !v.quantity || !v.warehouse_id)}
              >
                {tShared("confirmAction")}
              </Button>
              <Button variant="secondary" onClick={() => setShowReceive(false)}>
                {tCommon("cancel")}
              </Button>
            </div>
          </div>
        </Card>
      )}

      {po && (
        <Card>
          <CardHeader>
            <CardTitle>{tShared("lineItems")}</CardTitle>
          </CardHeader>
          <div className="-mx-6 -mb-6">
            <table className="w-full text-sm">
              <thead className="border-b border-t bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-semibold uppercase text-gray-500">{t("materials")}</th>
                  <th className="px-6 py-3 text-right text-xs font-semibold uppercase text-gray-500">{tCommon("quantity")}</th>
                  <th className="px-6 py-3 text-left text-xs font-semibold uppercase text-gray-500">{tCommon("unit")}</th>
                  <th className="px-6 py-3 text-right text-xs font-semibold uppercase text-gray-500">{tCommon("price")}</th>
                  <th className="px-6 py-3 text-right text-xs font-semibold uppercase text-gray-500">{tCommon("total")}</th>
                  <th className="px-6 py-3 text-right text-xs font-semibold uppercase text-gray-500">{tShared("received")}</th>
                </tr>
              </thead>
              <tbody className="divide-y">
                {po.items?.map((item) => (
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
                      <Badge color={item.received_quantity >= item.quantity ? "green" : "amber"}>
                        {item.received_quantity} / {item.quantity}
                      </Badge>
                    </td>
                  </tr>
                ))}
              </tbody>
              <tfoot className="border-t-2 bg-gray-50 font-semibold">
                <tr>
                  <td colSpan={4} className="px-6 py-3 text-right text-gray-900">{tCommon("total")} {tCommon("amount")}</td>
                  <td className="px-6 py-3 text-right font-mono text-gray-900">{formatCurrency(po.total_amount)}</td>
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
