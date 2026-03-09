"use client";

import { useTranslations } from "next-intl";
import { Select } from "@/components/ui/select";
import { cn } from "@/lib/utils";

export interface StatusOption {
  value: string;
  label: string;
}

interface StatusFilterProps {
  value: string;
  onChange: (value: string) => void;
  options: StatusOption[];
  allLabel?: string;
  className?: string;
}

export function StatusFilter({
  value,
  onChange,
  options,
  allLabel,
  className,
}: StatusFilterProps) {
  const t = useTranslations("shared");
  const resolvedAllLabel = allLabel ?? t("allStatuses");

  return (
    <Select
      value={value}
      onChange={(e) => onChange(e.target.value)}
      options={[{ value: "", label: resolvedAllLabel }, ...options]}
      className={cn(className)}
    />
  );
}

export function useSoStatuses(): StatusOption[] {
  const t = useTranslations("shared");
  return [
    { value: "Draft", label: t("draft") },
    { value: "Confirmed", label: t("confirmed") },
    { value: "Delivered", label: t("delivered") },
    { value: "Invoiced", label: t("invoiced") },
    { value: "Completed", label: t("completed") },
    { value: "Cancelled", label: t("cancelled") },
  ];
}

export function usePoStatuses(): StatusOption[] {
  const t = useTranslations("shared");
  return [
    { value: "Draft", label: t("draft") },
    { value: "Confirmed", label: t("confirmed") },
    { value: "Received", label: t("received") },
    { value: "Completed", label: t("completed") },
    { value: "Cancelled", label: t("cancelled") },
  ];
}

export function useMaterialStatuses(): StatusOption[] {
  const t = useTranslations("common");
  const tShared = useTranslations("shared");
  return [
    { value: "Active", label: t("active") },
    { value: "Inactive", label: tShared("inactive") },
  ];
}

export function useProductionStatuses(): StatusOption[] {
  const t = useTranslations("shared");
  return [
    { value: "Planned", label: t("planned") },
    { value: "Released", label: t("released") },
    { value: "In Progress", label: t("inProgress") },
    { value: "Completed", label: t("completed") },
    { value: "Cancelled", label: t("cancelled") },
  ];
}

export function useInvoiceStatuses(): StatusOption[] {
  const t = useTranslations("shared");
  return [
    { value: "Draft", label: t("draft") },
    { value: "Posted", label: t("posted") },
    { value: "Paid", label: t("paid") },
    { value: "Cancelled", label: t("cancelled") },
  ];
}
