"use client";

import { type UseFormRegisterReturn } from "react-hook-form";
import { useTranslations } from "next-intl";
import type { FieldDefinition, ToggleFieldConfig } from "@/lib/types/lowcode";

interface ToggleFieldProps {
  field: FieldDefinition;
  register: UseFormRegisterReturn;
  error?: string;
  disabled?: boolean;
}

export function ToggleField({ field, register, error, disabled }: ToggleFieldProps) {
  const t = useTranslations("lowcode");
  const config = (field.field_config || {}) as ToggleFieldConfig;
  const onLabel = config.onLabel || t("onDefault");
  const offLabel = config.offLabel || t("offDefault");
  const isDisabled = disabled || field.is_readonly;

  return (
    <div className="w-full">
      {field.label && (
        <label className="mb-1 block text-sm font-medium text-gray-700">
          {field.label}
          {field.validation.required && <span className="ml-0.5 text-red-500">*</span>}
        </label>
      )}
      <label
        className={`inline-flex items-center gap-3 ${isDisabled ? "cursor-not-allowed opacity-50" : "cursor-pointer"}`}
      >
        <span className="text-sm text-gray-500">{offLabel}</span>
        <div className="relative">
          <input
            {...register}
            type="checkbox"
            disabled={isDisabled}
            className="peer sr-only"
          />
          <div className="h-6 w-11 rounded-full bg-gray-300 transition-colors peer-checked:bg-blue-600 peer-focus:ring-2 peer-focus:ring-blue-500 peer-focus:ring-offset-2" />
          <div className="absolute left-0.5 top-0.5 h-5 w-5 rounded-full bg-white shadow-sm transition-transform peer-checked:translate-x-5" />
        </div>
        <span className="text-sm text-gray-500">{onLabel}</span>
      </label>
      {error && <p className="mt-1 text-sm text-red-600">{error}</p>}
      {field.help_text && !error && <p className="mt-1 text-sm text-gray-500">{field.help_text}</p>}
    </div>
  );
}
