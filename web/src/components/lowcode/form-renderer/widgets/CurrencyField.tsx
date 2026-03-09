"use client";

import { useState, useCallback } from "react";
import { cn } from "@/lib/utils";
import type { FieldDefinition, CurrencyFieldConfig } from "@/lib/types/lowcode";

interface CurrencyFieldProps {
  field: FieldDefinition;
  value: number | string;
  onChange: (value: number) => void;
  error?: string;
  disabled?: boolean;
}

const CURRENCY_SYMBOLS: Record<string, string> = {
  TWD: "NT$",
  USD: "$",
  EUR: "\u20AC",
  JPY: "\u00A5",
  GBP: "\u00A3",
  CNY: "\u00A5",
};

const CURRENCY_LOCALES: Record<string, string> = {
  TWD: "zh-TW",
  USD: "en-US",
  EUR: "de-DE",
  JPY: "ja-JP",
  GBP: "en-GB",
  CNY: "zh-CN",
};

export function CurrencyField({ field, value, onChange, error, disabled }: CurrencyFieldProps) {
  const config = (field.field_config || {}) as CurrencyFieldConfig;
  const currency = config.currency || "TWD";
  const decimals = config.decimals ?? 2;
  const showSymbol = config.showSymbol !== false;
  const isDisabled = disabled || field.is_readonly;

  const [focused, setFocused] = useState(false);
  const [rawInput, setRawInput] = useState("");

  const symbol = CURRENCY_SYMBOLS[currency] || currency;
  const locale = CURRENCY_LOCALES[currency] || "en-US";

  const formatValue = useCallback(
    (num: number | string): string => {
      const n = typeof num === "string" ? parseFloat(num) : num;
      if (isNaN(n)) return "";
      return new Intl.NumberFormat(locale, {
        minimumFractionDigits: decimals,
        maximumFractionDigits: decimals,
      }).format(n);
    },
    [locale, decimals]
  );

  const handleFocus = () => {
    setFocused(true);
    const num = typeof value === "string" ? parseFloat(value) : value;
    setRawInput(isNaN(num) ? "" : String(num));
  };

  const handleBlur = () => {
    setFocused(false);
    const num = parseFloat(rawInput);
    if (!isNaN(num)) {
      onChange(num);
    } else if (rawInput === "") {
      onChange(0);
    }
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const val = e.target.value;
    // Allow digits, minus sign, and decimal point only
    if (/^-?\d*\.?\d*$/.test(val) || val === "") {
      setRawInput(val);
    }
  };

  const numericValue = typeof value === "string" ? parseFloat(value) : value;
  const displayValue = focused ? rawInput : formatValue(numericValue);

  return (
    <div className="w-full">
      {field.label && (
        <label className="mb-1 block text-sm font-medium text-gray-700">
          {field.label}
          {field.validation.required && <span className="ml-0.5 text-red-500">*</span>}
        </label>
      )}
      <div
        className={cn(
          "flex items-center rounded-md border shadow-sm transition-colors",
          "focus-within:border-blue-500 focus-within:ring-1 focus-within:ring-blue-500",
          isDisabled ? "bg-gray-50" : "bg-white",
          error ? "border-red-300" : "border-gray-300"
        )}
      >
        {showSymbol && (
          <span className="shrink-0 border-r border-gray-300 bg-gray-50 px-3 py-2 text-sm text-gray-500">
            {symbol}
          </span>
        )}
        <input
          type="text"
          inputMode="decimal"
          value={displayValue}
          onFocus={handleFocus}
          onBlur={handleBlur}
          onChange={handleChange}
          placeholder={field.placeholder || "0.00"}
          disabled={isDisabled}
          className={cn(
            "w-full rounded-r-md border-0 bg-transparent px-3 py-2 text-sm text-right",
            "focus:outline-none focus:ring-0",
            "disabled:cursor-not-allowed disabled:opacity-50",
            !showSymbol && "rounded-l-md"
          )}
        />
      </div>
      {error && <p className="mt-1 text-sm text-red-600">{error}</p>}
      {field.help_text && !error && <p className="mt-1 text-sm text-gray-500">{field.help_text}</p>}
    </div>
  );
}
