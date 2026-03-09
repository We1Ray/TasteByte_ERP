"use client";

import { useState, useRef, useEffect } from "react";
import { useTranslations } from "next-intl";
import { cn } from "@/lib/utils";
import type { FieldDefinition, ColorFieldConfig } from "@/lib/types/lowcode";

interface ColorPickerFieldProps {
  field: FieldDefinition;
  value: string;
  onChange: (value: string) => void;
  error?: string;
  disabled?: boolean;
}

const DEFAULT_PRESETS = [
  "#EF4444",
  "#F59E0B",
  "#10B981",
  "#3B82F6",
  "#6366F1",
  "#8B5CF6",
  "#EC4899",
  "#6B7280",
];

export function ColorPickerField({ field, value, onChange, error, disabled }: ColorPickerFieldProps) {
  const t = useTranslations("lowcode");
  const config = (field.field_config || {}) as ColorFieldConfig;
  const presets = config.presets || DEFAULT_PRESETS;
  const allowCustom = config.allowCustom !== false;
  const isDisabled = disabled || field.is_readonly;

  const [hexInput, setHexInput] = useState(value || "");
  const [trackedValue, setTrackedValue] = useState(value);
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);

  // Sync hexInput when value prop changes
  if (trackedValue !== value) {
    setTrackedValue(value);
    setHexInput(value || "");
  }

  useEffect(() => {
    const handleClick = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) {
        setOpen(false);
      }
    };
    document.addEventListener("mousedown", handleClick);
    return () => document.removeEventListener("mousedown", handleClick);
  }, []);

  const handlePresetClick = (color: string) => {
    onChange(color);
    setHexInput(color);
  };

  const handleHexChange = (val: string) => {
    let sanitized = val;
    if (!sanitized.startsWith("#")) {
      sanitized = "#" + sanitized;
    }
    setHexInput(sanitized);
    if (/^#[0-9a-fA-F]{6}$/.test(sanitized)) {
      onChange(sanitized);
    }
  };

  return (
    <div className="w-full" ref={ref}>
      {field.label && (
        <label className="mb-1 block text-sm font-medium text-gray-700">
          {field.label}
          {field.validation.required && <span className="ml-0.5 text-red-500">*</span>}
        </label>
      )}

      <div className="relative">
        <button
          type="button"
          disabled={isDisabled}
          onClick={() => !isDisabled && setOpen(!open)}
          className={cn(
            "flex w-full items-center gap-3 rounded-md border px-3 py-2 text-sm shadow-sm",
            "focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500",
            isDisabled ? "cursor-not-allowed bg-gray-50" : "bg-white",
            error ? "border-red-300" : "border-gray-300"
          )}
        >
          <div
            className="h-6 w-6 shrink-0 rounded-full border border-gray-200"
            style={{ backgroundColor: value || "#FFFFFF" }}
          />
          <span className={value ? "text-gray-900" : "text-gray-400"}>
            {value || field.placeholder || `${t("select")}...`}
          </span>
        </button>

        {open && !isDisabled && (
          <div className="absolute z-10 mt-1 w-full rounded-md border border-gray-200 bg-white p-3 shadow-lg">
            <div className="mb-2 grid grid-cols-8 gap-2">
              {presets.map((color) => (
                <button
                  key={color}
                  type="button"
                  className={cn(
                    "h-7 w-7 rounded-full border-2 transition-transform hover:scale-110",
                    value === color ? "border-gray-900 ring-2 ring-gray-300" : "border-transparent"
                  )}
                  style={{ backgroundColor: color }}
                  onClick={() => handlePresetClick(color)}
                  title={color}
                />
              ))}
            </div>
            {allowCustom && (
              <div className="mt-2 flex items-center gap-2 border-t border-gray-100 pt-2">
                <div
                  className="h-8 w-8 shrink-0 rounded-md border border-gray-200"
                  style={{ backgroundColor: /^#[0-9a-fA-F]{6}$/.test(hexInput) ? hexInput : "#FFFFFF" }}
                />
                <input
                  type="text"
                  value={hexInput}
                  onChange={(e) => handleHexChange(e.target.value)}
                  placeholder="#000000"
                  maxLength={7}
                  className="flex-1 rounded-md border border-gray-300 px-2 py-1 text-sm font-mono focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                />
              </div>
            )}
          </div>
        )}
      </div>

      {error && <p className="mt-1 text-sm text-red-600">{error}</p>}
      {field.help_text && !error && <p className="mt-1 text-sm text-gray-500">{field.help_text}</p>}
    </div>
  );
}
