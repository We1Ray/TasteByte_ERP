"use client";

import { type UseFormRegisterReturn } from "react-hook-form";
import { cn } from "@/lib/utils";
import type { FieldDefinition } from "@/lib/types/lowcode";

interface TextAreaFieldProps {
  field: FieldDefinition;
  register: UseFormRegisterReturn;
  error?: string;
  disabled?: boolean;
}

export function TextAreaField({ field, register, error, disabled }: TextAreaFieldProps) {
  return (
    <div className="w-full">
      {field.label && (
        <label className="mb-1 block text-sm font-medium text-gray-700">
          {field.label}
          {field.validation.required && <span className="ml-0.5 text-red-500">*</span>}
        </label>
      )}
      <textarea
        {...register}
        placeholder={field.placeholder}
        disabled={disabled || field.is_readonly}
        rows={4}
        maxLength={field.validation.max_length}
        className={cn(
          "block w-full rounded-md border px-3 py-2 text-sm shadow-sm transition-colors",
          "placeholder:text-gray-400",
          "focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500",
          "disabled:cursor-not-allowed disabled:bg-gray-50 disabled:text-gray-500",
          error ? "border-red-300 focus:border-red-500 focus:ring-red-500" : "border-gray-300"
        )}
      />
      {error && <p className="mt-1 text-sm text-red-600">{error}</p>}
      {field.help_text && !error && <p className="mt-1 text-sm text-gray-500">{field.help_text}</p>}
    </div>
  );
}
