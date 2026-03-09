"use client";

import { useMemo } from "react";
import {
  CheckCircle,
  Truck,
  FileText,
  Rocket,
  Play,
  Package,
  XCircle,
} from "lucide-react";
import { useTranslations } from "next-intl";
import { Button } from "@/components/ui/button";

interface DocumentActionsProps {
  status: string;
  documentType:
    | "sales_order"
    | "purchase_order"
    | "production_order"
    | "invoice"
    | "journal_entry"
    | "delivery";
  onAction: (action: string) => void;
  isLoading?: boolean;
}

interface ActionDef {
  action: string;
  label: string;
  icon: React.ReactNode;
  variant?: "primary" | "secondary" | "danger";
}

function getActions(documentType: string, status: string, t: (key: string) => string): ActionDef[] {
  const s = status.toLowerCase().replace(/_/g, " ");

  switch (documentType) {
    case "sales_order":
      if (s === "draft")
        return [
          { action: "confirm", label: t("confirmOrder"), icon: <CheckCircle className="h-4 w-4" /> },
        ];
      if (s === "confirmed" || s === "open")
        return [
          { action: "create_delivery", label: t("createDelivery"), icon: <Truck className="h-4 w-4" /> },
          { action: "create_invoice", label: t("createInvoice"), icon: <FileText className="h-4 w-4" /> },
        ];
      if (s === "delivered")
        return [
          { action: "create_invoice", label: t("createInvoice"), icon: <FileText className="h-4 w-4" /> },
          { action: "complete", label: t("complete"), icon: <CheckCircle className="h-4 w-4" /> },
        ];
      break;

    case "purchase_order":
      if (s === "draft")
        return [
          { action: "confirm", label: t("confirmAction"), icon: <CheckCircle className="h-4 w-4" /> },
        ];
      if (s === "released" || s === "confirmed")
        return [
          { action: "goods_receipt", label: t("goodsReceipt"), icon: <Package className="h-4 w-4" /> },
        ];
      break;

    case "production_order":
      if (s === "created" || s === "planned")
        return [
          { action: "release", label: t("release"), icon: <Rocket className="h-4 w-4" /> },
        ];
      if (s === "released")
        return [
          { action: "start", label: t("startProduction"), icon: <Play className="h-4 w-4" /> },
        ];
      if (s === "in progress")
        return [
          { action: "complete", label: t("complete"), icon: <CheckCircle className="h-4 w-4" /> },
        ];
      break;

    case "invoice":
      if (s === "draft")
        return [
          { action: "post", label: t("postInvoice"), icon: <CheckCircle className="h-4 w-4" /> },
        ];
      if (s === "posted")
        return [
          { action: "record_payment", label: t("recordPayment"), icon: <FileText className="h-4 w-4" /> },
        ];
      break;

    case "journal_entry":
      if (s === "draft")
        return [
          { action: "post", label: t("postEntry"), icon: <CheckCircle className="h-4 w-4" /> },
        ];
      break;

    case "delivery":
      if (s === "created" || s === "draft")
        return [
          { action: "ship", label: t("ship"), icon: <Truck className="h-4 w-4" /> },
        ];
      if (s === "shipped")
        return [
          { action: "deliver", label: t("confirmDelivery"), icon: <CheckCircle className="h-4 w-4" /> },
        ];
      break;
  }

  // Cancel is available for draft/open statuses on most document types
  if (
    (s === "draft" || s === "open" || s === "created") &&
    documentType !== "journal_entry"
  ) {
    return [
      { action: "cancel", label: t("cancelAction"), icon: <XCircle className="h-4 w-4" />, variant: "danger" },
    ];
  }

  return [];
}

export function DocumentActions({
  status,
  documentType,
  onAction,
  isLoading,
}: DocumentActionsProps) {
  const t = useTranslations("shared");
  const actions = useMemo(() => getActions(documentType, status, t), [documentType, status, t]);

  if (actions.length === 0) return null;

  return (
    <>
      {actions.map((a) => (
        <Button
          key={a.action}
          variant={a.variant || "primary"}
          onClick={() => onAction(a.action)}
          loading={isLoading}
        >
          {a.icon}
          {a.label}
        </Button>
      ))}
    </>
  );
}
