"use client";

import { type UseFormRegisterReturn } from "react-hook-form";
import { Input } from "@/components/ui/input";
import type { FieldDefinition } from "@/lib/types/lowcode";

interface NumberFieldProps {
  field: FieldDefinition;
  register: UseFormRegisterReturn;
  error?: string;
  disabled?: boolean;
}

export function NumberField({ field, register, error, disabled }: NumberFieldProps) {
  return (
    <Input
      {...register}
      type="number"
      label={field.label}
      placeholder={field.placeholder}
      error={error}
      helperText={field.help_text}
      required={field.validation.required}
      disabled={disabled || field.is_readonly}
      min={field.validation.min_value}
      max={field.validation.max_value}
      step="any"
    />
  );
}
