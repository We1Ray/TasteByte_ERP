"use client";

import { useState } from "react";
import { ChevronRight, ChevronLeft } from "lucide-react";
import { useTranslations } from "next-intl";
import { Button } from "@/components/ui/button";
import { Modal } from "@/components/ui/modal";
import type { FieldDefinition } from "@/lib/types/lowcode";

interface CompositeFieldProps {
  field: FieldDefinition;
  value: unknown;
  onChange: (value: unknown) => void;
  error?: string;
  disabled?: boolean;
}

export function CompositeField({ field, value, onChange, error, disabled }: CompositeFieldProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const [open, setOpen] = useState(false);
  const [step, setStep] = useState(0);
  const [localData, setLocalData] = useState<Record<string, string>>({});

  const steps = field.composite_steps || [];
  const currentStep = steps[step];
  const isLast = step === steps.length - 1;

  const handleOpen = () => {
    setLocalData((value as Record<string, string>) || {});
    setStep(0);
    setOpen(true);
  };

  const handleSave = () => {
    onChange(localData);
    setOpen(false);
  };

  const displayValue = value
    ? Object.values(value as Record<string, string>).filter(Boolean).join(", ")
    : "";

  return (
    <div className="w-full">
      {field.label && (
        <label className="mb-1 block text-sm font-medium text-gray-700">
          {field.label}
          {field.validation.required && <span className="ml-0.5 text-red-500">*</span>}
        </label>
      )}
      <div
        className="flex min-h-[38px] w-full cursor-pointer items-center rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm hover:border-gray-400"
        onClick={() => !disabled && handleOpen()}
      >
        <span className={displayValue ? "text-gray-700" : "text-gray-400"}>
          {displayValue || field.placeholder || `${t("select")}...`}
        </span>
      </div>

      {error && <p className="mt-1 text-sm text-red-600">{error}</p>}

      <Modal
        open={open}
        onClose={() => setOpen(false)}
        title={`${field.label} - ${currentStep?.title || `Step ${step + 1}`}`}
        size="lg"
        footer={
          <>
            {step > 0 && (
              <Button variant="secondary" onClick={() => setStep(step - 1)}>
                <ChevronLeft className="h-4 w-4" />
                {tCommon("back")}
              </Button>
            )}
            {isLast ? (
              <Button onClick={handleSave}>{tCommon("save")}</Button>
            ) : (
              <Button onClick={() => setStep(step + 1)}>
                {t("nextStep")}
                <ChevronRight className="h-4 w-4" />
              </Button>
            )}
          </>
        }
      >
        {currentStep ? (
          <div className="space-y-4">
            {currentStep.fields.map((fieldKey) => (
              <div key={fieldKey}>
                <label className="mb-1 block text-sm font-medium text-gray-700">{fieldKey}</label>
                <input
                  value={localData[fieldKey] || ""}
                  onChange={(e) => setLocalData({ ...localData, [fieldKey]: e.target.value })}
                  className="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                />
              </div>
            ))}
          </div>
        ) : (
          <p className="text-sm text-gray-500">{t("noStepsConfigured")}</p>
        )}

        {steps.length > 1 && (
          <div className="mt-4 flex items-center justify-center gap-1">
            {steps.map((_, i) => (
              <div
                key={i}
                className={`h-2 w-2 rounded-full ${i === step ? "bg-blue-600" : "bg-gray-200"}`}
              />
            ))}
          </div>
        )}
      </Modal>
    </div>
  );
}
