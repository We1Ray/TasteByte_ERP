"use client";

import { type UseFormRegisterReturn } from "react-hook-form";
import { useTranslations } from "next-intl";
import { useDropdownOptions } from "@/lib/hooks/use-lookup";
import { cn } from "@/lib/utils";
import type { FieldDefinition, RadioGroupFieldConfig } from "@/lib/types/lowcode";

interface RadioGroupFieldProps {
  field: FieldDefinition;
  register: UseFormRegisterReturn;
  error?: string;
  disabled?: boolean;
}

export function RadioGroupField({ field, register, error, disabled }: RadioGroupFieldProps) {
  const t = useTranslations("lowcode");
  const config = (field.field_config || {}) as RadioGroupFieldConfig;
  const layout = config.layout || "vertical";
  const isDisabled = disabled || field.is_readonly;

  // Get options from data_source (static/sql) or fallback to config.options
  const dataSourceOptions = useDropdownOptions(field.data_source);
  const options =
    dataSourceOptions.length > 0
      ? dataSourceOptions
      : (config.options || []).map((o) => ({ label: o.label, value: o.value }));

  if (layout === "button") {
    return (
      <div className="w-full">
        {field.label && (
          <label className="mb-1 block text-sm font-medium text-gray-700">
            {field.label}
            {field.validation.required && <span className="ml-0.5 text-red-500">*</span>}
          </label>
        )}
        <div className="inline-flex overflow-hidden rounded-md border border-gray-300 shadow-sm">
          {options.map((opt, idx) => (
            <label
              key={opt.value}
              className={cn(
                "relative cursor-pointer px-4 py-2 text-sm font-medium transition-colors",
                "has-[:checked]:bg-blue-600 has-[:checked]:text-white has-[:checked]:border-blue-600",
                "has-[:not(:checked)]:bg-white has-[:not(:checked)]:text-gray-700 has-[:not(:checked)]:hover:bg-gray-50",
                idx > 0 && "border-l border-gray-300",
                isDisabled && "cursor-not-allowed opacity-50"
              )}
            >
              <input
                {...register}
                type="radio"
                value={opt.value}
                disabled={isDisabled}
                className="sr-only"
              />
              {opt.label}
            </label>
          ))}
        </div>
        {error && <p className="mt-1 text-sm text-red-600">{error}</p>}
        {field.help_text && !error && <p className="mt-1 text-sm text-gray-500">{field.help_text}</p>}
      </div>
    );
  }

  return (
    <div className="w-full">
      {field.label && (
        <label className="mb-1 block text-sm font-medium text-gray-700">
          {field.label}
          {field.validation.required && <span className="ml-0.5 text-red-500">*</span>}
        </label>
      )}
      <div
        className={cn(
          "flex gap-4",
          layout === "vertical" ? "flex-col gap-2" : "flex-row flex-wrap"
        )}
      >
        {options.map((opt) => (
          <label
            key={opt.value}
            className={cn(
              "flex items-center gap-2",
              isDisabled ? "cursor-not-allowed opacity-50" : "cursor-pointer"
            )}
          >
            <input
              {...register}
              type="radio"
              value={opt.value}
              disabled={isDisabled}
              className="h-4 w-4 border-gray-300 text-blue-600 focus:ring-blue-500"
            />
            <span className="text-sm text-gray-700">{opt.label}</span>
          </label>
        ))}
      </div>
      {options.length === 0 && (
        <p className="text-sm text-gray-400">{t("noResults")}</p>
      )}
      {error && <p className="mt-1 text-sm text-red-600">{error}</p>}
      {field.help_text && !error && <p className="mt-1 text-sm text-gray-500">{field.help_text}</p>}
    </div>
  );
}
