"use client";

import { type UseFormRegisterReturn } from "react-hook-form";
import type { FieldDefinition } from "@/lib/types/lowcode";

interface CheckboxFieldProps {
  field: FieldDefinition;
  register: UseFormRegisterReturn;
  error?: string;
  disabled?: boolean;
}

export function CheckboxField({ field, register, error, disabled }: CheckboxFieldProps) {
  return (
    <div className="w-full">
      <label className="flex items-center gap-2">
        <input
          {...register}
          type="checkbox"
          disabled={disabled || field.is_readonly}
          className="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500 disabled:cursor-not-allowed disabled:opacity-50"
        />
        <span className="text-sm font-medium text-gray-700">{field.label}</span>
      </label>
      {error && <p className="mt-1 text-sm text-red-600">{error}</p>}
      {field.help_text && !error && <p className="mt-1 text-sm text-gray-500">{field.help_text}</p>}
    </div>
  );
}
