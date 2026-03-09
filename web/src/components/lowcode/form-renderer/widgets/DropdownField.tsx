"use client";

import { type UseFormRegisterReturn } from "react-hook-form";
import { useTranslations } from "next-intl";
import { Select } from "@/components/ui/select";
import { useDropdownOptions } from "@/lib/hooks/use-lookup";
import type { FieldDefinition } from "@/lib/types/lowcode";

interface DropdownFieldProps {
  field: FieldDefinition;
  register: UseFormRegisterReturn;
  error?: string;
  disabled?: boolean;
}

export function DropdownField({ field, register, error, disabled }: DropdownFieldProps) {
  const t = useTranslations("lowcode");
  const options = useDropdownOptions(field.data_source);

  return (
    <Select
      {...register}
      label={field.label}
      placeholder={field.placeholder || `${t("select")}...`}
      error={error}
      required={field.validation.required}
      disabled={disabled || field.is_readonly}
      options={options}
    />
  );
}
