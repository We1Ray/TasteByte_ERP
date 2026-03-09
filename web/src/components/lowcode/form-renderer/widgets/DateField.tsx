"use client";

import { type UseFormRegisterReturn } from "react-hook-form";
import { Input } from "@/components/ui/input";
import type { FieldDefinition } from "@/lib/types/lowcode";

interface DateFieldProps {
  field: FieldDefinition;
  register: UseFormRegisterReturn;
  error?: string;
  disabled?: boolean;
}

export function DateField({ field, register, error, disabled }: DateFieldProps) {
  return (
    <Input
      {...register}
      type="date"
      label={field.label}
      error={error}
      helperText={field.help_text}
      required={field.validation.required}
      disabled={disabled || field.is_readonly}
    />
  );
}
