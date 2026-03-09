"use client";

import React, { useMemo } from "react";
import { Check, XCircle } from "lucide-react";
import { useTranslations } from "next-intl";
import { cn } from "@/lib/utils";

interface StatusStep {
  key: string;
  label: string;
}

interface StatusStepperProps {
  steps: StatusStep[];
  currentStatus: string;
  orientation?: "horizontal" | "vertical";
  className?: string;
}

export function getSoSteps(t: (key: string) => string): StatusStep[] {
  return [
    { key: "DRAFT", label: t("draft") },
    { key: "CONFIRMED", label: t("confirmed") },
    { key: "DELIVERED", label: t("delivered") },
    { key: "INVOICED", label: t("invoiced") },
    { key: "COMPLETED", label: t("completed") },
  ];
}

export function getPoSteps(t: (key: string) => string): StatusStep[] {
  return [
    { key: "DRAFT", label: t("draft") },
    { key: "CONFIRMED", label: t("confirmed") },
    { key: "RECEIVED", label: t("received") },
    { key: "COMPLETED", label: t("completed") },
  ];
}

export function getProductionSteps(t: (key: string) => string): StatusStep[] {
  return [
    { key: "CREATED", label: t("created") },
    { key: "RELEASED", label: t("released") },
    { key: "IN_PROGRESS", label: t("inProgress") },
    { key: "COMPLETED", label: t("completed") },
  ];
}

export function getInvoiceSteps(t: (key: string) => string): StatusStep[] {
  return [
    { key: "DRAFT", label: t("draft") },
    { key: "POSTED", label: t("posted") },
    { key: "PAID", label: t("paid") },
  ];
}

export function getJournalSteps(t: (key: string) => string): StatusStep[] {
  return [
    { key: "DRAFT", label: t("draft") },
    { key: "POSTED", label: t("posted") },
  ];
}

export function useStatusSteps() {
  const t = useTranslations("shared");
  return useMemo(
    () => ({
      soSteps: getSoSteps(t),
      poSteps: getPoSteps(t),
      productionSteps: getProductionSteps(t),
      invoiceSteps: getInvoiceSteps(t),
      journalSteps: getJournalSteps(t),
    }),
    [t]
  );
}

export function StatusStepper({
  steps,
  currentStatus,
  orientation = "horizontal",
  className,
}: StatusStepperProps) {
  const t = useTranslations("shared");
  const normalizedStatus = currentStatus.toUpperCase().replace(/ /g, "_");
  const currentIdx = steps.findIndex((s) => s.key === normalizedStatus);
  const isCancelled = normalizedStatus === "CANCELLED";

  if (orientation === "vertical") {
    return (
      <div className={cn("space-y-0", className)}>
        {steps.map((step, i) => (
          <div key={step.key} className="flex items-start gap-3">
            <div className="flex flex-col items-center">
              <div
                className={cn(
                  "flex h-8 w-8 items-center justify-center rounded-full text-xs font-bold flex-shrink-0",
                  i < currentIdx &&
                    !isCancelled &&
                    "bg-blue-600 text-white",
                  i === currentIdx &&
                    !isCancelled &&
                    "bg-blue-600 text-white ring-4 ring-blue-100",
                  (i > currentIdx || isCancelled) &&
                    !(i < currentIdx && !isCancelled) &&
                    !(i === currentIdx && !isCancelled) &&
                    "border-2 border-gray-300 text-gray-400"
                )}
              >
                {i < currentIdx && !isCancelled ? (
                  <Check className="h-4 w-4" />
                ) : (
                  i + 1
                )}
              </div>
              {i < steps.length - 1 && (
                <div
                  className={cn(
                    "w-0.5 h-6",
                    i < currentIdx && !isCancelled
                      ? "bg-blue-600"
                      : "bg-gray-300"
                  )}
                />
              )}
            </div>
            <span
              className={cn(
                "pt-1.5 text-sm",
                i <= currentIdx && !isCancelled
                  ? "font-medium text-gray-900"
                  : "text-gray-400"
              )}
            >
              {step.label}
            </span>
          </div>
        ))}
        {isCancelled && (
          <div className="flex items-start gap-3">
            <div className="flex flex-col items-center">
              <div className="flex h-8 w-8 items-center justify-center rounded-full bg-red-100 text-red-600 flex-shrink-0">
                <XCircle className="h-4 w-4" />
              </div>
            </div>
            <span className="pt-1.5 text-sm font-medium text-red-600">
              {t("cancelled")}
            </span>
          </div>
        )}
      </div>
    );
  }

  return (
    <div className={cn("flex items-center", className)}>
      {steps.map((step, i) => (
        <React.Fragment key={step.key}>
          <div className="flex flex-col items-center">
            <div
              className={cn(
                "flex h-8 w-8 items-center justify-center rounded-full text-xs font-bold",
                i < currentIdx &&
                  !isCancelled &&
                  "bg-blue-600 text-white",
                i === currentIdx &&
                  !isCancelled &&
                  "bg-blue-600 text-white ring-4 ring-blue-100",
                (i > currentIdx || isCancelled) &&
                  !(i < currentIdx && !isCancelled) &&
                  !(i === currentIdx && !isCancelled) &&
                  "border-2 border-gray-300 text-gray-400"
              )}
            >
              {i < currentIdx && !isCancelled ? (
                <Check className="h-4 w-4" />
              ) : (
                i + 1
              )}
            </div>
            <span
              className={cn(
                "mt-2 text-xs",
                i <= currentIdx && !isCancelled
                  ? "font-medium text-gray-900"
                  : "text-gray-400"
              )}
            >
              {step.label}
            </span>
          </div>
          {i < steps.length - 1 && (
            <div
              className={cn(
                "h-0.5 flex-1 mx-2",
                i < currentIdx && !isCancelled
                  ? "bg-blue-600"
                  : "bg-gray-300"
              )}
            />
          )}
        </React.Fragment>
      ))}
      {isCancelled && (
        <>
          <div className="h-0.5 flex-1 mx-2 bg-red-300" />
          <div className="flex flex-col items-center">
            <div className="flex h-8 w-8 items-center justify-center rounded-full bg-red-100 text-red-600">
              <XCircle className="h-4 w-4" />
            </div>
            <span className="mt-2 text-xs font-medium text-red-600">
              {t("cancelled")}
            </span>
          </div>
        </>
      )}
    </div>
  );
}
