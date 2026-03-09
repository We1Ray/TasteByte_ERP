"use client";

import { useState, useRef, useEffect } from "react";
import { X, ChevronDown } from "lucide-react";
import { useTranslations } from "next-intl";
import { useDropdownOptions } from "@/lib/hooks/use-lookup";
import { cn } from "@/lib/utils";
import type { FieldDefinition } from "@/lib/types/lowcode";

interface MultiSelectFieldProps {
  field: FieldDefinition;
  value: string[];
  onChange: (value: string[]) => void;
  error?: string;
  disabled?: boolean;
}

export function MultiSelectField({ field, value = [], onChange, error, disabled }: MultiSelectFieldProps) {
  const t = useTranslations("lowcode");
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);
  const options = useDropdownOptions(field.data_source);

  useEffect(() => {
    const handleClick = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) {
        setOpen(false);
      }
    };
    document.addEventListener("mousedown", handleClick);
    return () => document.removeEventListener("mousedown", handleClick);
  }, []);

  const toggleOption = (optValue: string) => {
    if (value.includes(optValue)) {
      onChange(value.filter((v) => v !== optValue));
    } else {
      onChange([...value, optValue]);
    }
  };

  const removeOption = (optValue: string) => {
    onChange(value.filter((v) => v !== optValue));
  };

  return (
    <div className="w-full" ref={ref}>
      {field.label && (
        <label className="mb-1 block text-sm font-medium text-gray-700">
          {field.label}
          {field.validation.required && <span className="ml-0.5 text-red-500">*</span>}
        </label>
      )}
      <div
        className={cn(
          "relative min-h-[38px] w-full rounded-md border px-3 py-1.5 text-sm shadow-sm",
          "focus-within:border-blue-500 focus-within:ring-1 focus-within:ring-blue-500",
          disabled ? "cursor-not-allowed bg-gray-50" : "cursor-pointer bg-white",
          error ? "border-red-300" : "border-gray-300"
        )}
        onClick={() => !disabled && setOpen(!open)}
      >
        <div className="flex flex-wrap items-center gap-1 pr-6">
          {value.length === 0 && (
            <span className="text-gray-400">{field.placeholder || `${t("select")}...`}</span>
          )}
          {value.map((v) => {
            const label = options.find((o) => o.value === v)?.label || v;
            return (
              <span
                key={v}
                className="inline-flex items-center gap-1 rounded-full bg-blue-100 px-2 py-0.5 text-xs font-medium text-blue-700"
              >
                {label}
                {!disabled && (
                  <button
                    type="button"
                    onClick={(e) => {
                      e.stopPropagation();
                      removeOption(v);
                    }}
                    className="hover:text-blue-900"
                  >
                    <X className="h-3 w-3" />
                  </button>
                )}
              </span>
            );
          })}
        </div>
        <ChevronDown className="absolute right-2 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400" />
      </div>

      {open && !disabled && (
        <div className="absolute z-10 mt-1 max-h-48 w-full overflow-auto rounded-md border border-gray-200 bg-white shadow-lg">
          {options.map((opt) => (
            <button
              key={opt.value}
              type="button"
              className={cn(
                "flex w-full items-center gap-2 px-3 py-2 text-left text-sm hover:bg-gray-50",
                value.includes(opt.value) && "bg-blue-50 text-blue-700"
              )}
              onClick={() => toggleOption(opt.value)}
            >
              <input
                type="checkbox"
                checked={value.includes(opt.value)}
                readOnly
                className="h-4 w-4 rounded border-gray-300 text-blue-600"
              />
              {opt.label}
            </button>
          ))}
          {options.length === 0 && (
            <p className="px-3 py-2 text-sm text-gray-500">{t("noResults")}</p>
          )}
        </div>
      )}

      {error && <p className="mt-1 text-sm text-red-600">{error}</p>}
      {field.help_text && !error && <p className="mt-1 text-sm text-gray-500">{field.help_text}</p>}
    </div>
  );
}
