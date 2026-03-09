"use client";

import { type UseFormRegisterReturn } from "react-hook-form";
import { Input } from "@/components/ui/input";
import type { FieldDefinition } from "@/lib/types/lowcode";

interface DateTimeFieldProps {
  field: FieldDefinition;
  register: UseFormRegisterReturn;
  error?: string;
  disabled?: boolean;
}

export function DateTimeField({ field, register, error, disabled }: DateTimeFieldProps) {
  return (
    <Input
      {...register}
      type="datetime-local"
      label={field.label}
      error={error}
      helperText={field.help_text}
      required={field.validation.required}
      disabled={disabled || field.is_readonly}
    />
  );
}
