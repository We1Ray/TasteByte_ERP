"use client";

import { useState } from "react";
import { Search } from "lucide-react";
import { useTranslations } from "next-intl";
import { cn } from "@/lib/utils";
import { LookupModal } from "../../shared/LookupModal";
import type { FieldDefinition } from "@/lib/types/lowcode";

interface LookupWindowFieldProps {
  field: FieldDefinition;
  value: string;
  onChange: (value: string) => void;
  error?: string;
  disabled?: boolean;
}

export function LookupWindowField({ field, value, onChange, error, disabled }: LookupWindowFieldProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const [open, setOpen] = useState(false);
  const [displayLabel, setDisplayLabel] = useState("");

  const handleSelect = (selectedValue: string, label: string) => {
    onChange(selectedValue);
    setDisplayLabel(label);
    setOpen(false);
  };

  return (
    <div className="w-full">
      {field.label && (
        <label className="mb-1 block text-sm font-medium text-gray-700">
          {field.label}
          {field.validation.required && <span className="ml-0.5 text-red-500">*</span>}
        </label>
      )}
      <div className="flex gap-2">
        <input
          value={displayLabel || value}
          readOnly
          placeholder={field.placeholder || `${tCommon("search")}...`}
          disabled={disabled || field.is_readonly}
          className={cn(
            "block w-full rounded-md border px-3 py-2 text-sm shadow-sm",
            "focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500",
            "disabled:cursor-not-allowed disabled:bg-gray-50",
            error ? "border-red-300" : "border-gray-300"
          )}
        />
        <button
          type="button"
          onClick={() => setOpen(true)}
          disabled={disabled || field.is_readonly}
          className="rounded-md border border-gray-300 px-3 py-2 text-gray-500 hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-50"
        >
          <Search className="h-4 w-4" />
        </button>
      </div>

      {error && <p className="mt-1 text-sm text-red-600">{error}</p>}
      {field.help_text && !error && <p className="mt-1 text-sm text-gray-500">{field.help_text}</p>}

      {field.lookup_config && (
        <LookupModal
          open={open}
          onClose={() => setOpen(false)}
          title={`${t("select")} ${field.label}`}
          config={field.lookup_config}
          onSelect={handleSelect}
        />
      )}
    </div>
  );
}
