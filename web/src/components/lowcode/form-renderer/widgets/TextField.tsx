"use client";

import { type UseFormRegisterReturn } from "react-hook-form";
import { Input } from "@/components/ui/input";
import type { FieldDefinition } from "@/lib/types/lowcode";

interface TextFieldProps {
  field: FieldDefinition;
  register: UseFormRegisterReturn;
  error?: string;
  disabled?: boolean;
}

export function TextField({ field, register, error, disabled }: TextFieldProps) {
  return (
    <Input
      {...register}
      label={field.label}
      placeholder={field.placeholder}
      error={error}
      helperText={field.help_text}
      required={field.validation.required}
      disabled={disabled || field.is_readonly}
      maxLength={field.validation.max_length}
    />
  );
}
