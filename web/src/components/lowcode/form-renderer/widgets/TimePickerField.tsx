"use client";

import { useState, useRef, useEffect, useMemo } from "react";
import { Clock } from "lucide-react";
import { useTranslations } from "next-intl";
import { cn } from "@/lib/utils";
import type { FieldDefinition, TimePickerFieldConfig } from "@/lib/types/lowcode";

interface TimePickerFieldProps {
  field: FieldDefinition;
  value: string;
  onChange: (value: string) => void;
  error?: string;
  disabled?: boolean;
}

function formatTimeSlot(hour: number, minute: number, format: "12h" | "24h"): string {
  if (format === "12h") {
    const period = hour >= 12 ? "PM" : "AM";
    const h = hour === 0 ? 12 : hour > 12 ? hour - 12 : hour;
    return `${h.toString().padStart(2, "0")}:${minute.toString().padStart(2, "0")} ${period}`;
  }
  return `${hour.toString().padStart(2, "0")}:${minute.toString().padStart(2, "0")}`;
}

function toStorageFormat(hour: number, minute: number): string {
  return `${hour.toString().padStart(2, "0")}:${minute.toString().padStart(2, "0")}`;
}

function displayFromStorage(val: string, format: "12h" | "24h"): string {
  if (!val || !/^\d{2}:\d{2}$/.test(val)) return "";
  const [h, m] = val.split(":").map(Number);
  return formatTimeSlot(h, m, format);
}

export function TimePickerField({ field, value, onChange, error, disabled }: TimePickerFieldProps) {
  const t = useTranslations("lowcode");
  const config = (field.field_config || {}) as TimePickerFieldConfig;
  const format = config.format || "24h";
  const minuteStep = config.minuteStep || 15;
  const isDisabled = disabled || field.is_readonly;

  const [open, setOpen] = useState(false);
  const [search, setSearch] = useState("");
  const ref = useRef<HTMLDivElement>(null);
  const listRef = useRef<HTMLDivElement>(null);

  // Generate all time slots
  const timeSlots = useMemo(() => {
    const slots: { display: string; value: string }[] = [];
    for (let h = 0; h < 24; h++) {
      for (let m = 0; m < 60; m += minuteStep) {
        slots.push({
          display: formatTimeSlot(h, m, format),
          value: toStorageFormat(h, m),
        });
      }
    }
    return slots;
  }, [format, minuteStep]);

  // Filter by search
  const filteredSlots = useMemo(() => {
    if (!search) return timeSlots;
    const lower = search.toLowerCase();
    return timeSlots.filter((slot) => slot.display.toLowerCase().includes(lower));
  }, [timeSlots, search]);

  // Close on outside click
  useEffect(() => {
    const handleClick = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) {
        setOpen(false);
        setSearch("");
      }
    };
    document.addEventListener("mousedown", handleClick);
    return () => document.removeEventListener("mousedown", handleClick);
  }, []);

  // Scroll to selected item when opening
  useEffect(() => {
    if (open && value && listRef.current) {
      const selectedEl = listRef.current.querySelector(`[data-value="${value}"]`);
      if (selectedEl) {
        selectedEl.scrollIntoView({ block: "center" });
      }
    }
  }, [open, value]);

  const handleSelect = (slot: string) => {
    onChange(slot);
    setOpen(false);
    setSearch("");
  };

  const displayValue = displayFromStorage(value, format);

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
            "flex w-full items-center rounded-md border px-3 py-2 text-sm shadow-sm",
            "focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500",
            isDisabled ? "cursor-not-allowed bg-gray-50" : "bg-white",
            error ? "border-red-300" : "border-gray-300"
          )}
        >
          <Clock className="mr-2 h-4 w-4 text-gray-400" />
          <span className={displayValue ? "text-gray-900" : "text-gray-400"}>
            {displayValue || field.placeholder || `${t("select")}...`}
          </span>
        </button>

        {open && !isDisabled && (
          <div className="absolute z-10 mt-1 w-full rounded-md border border-gray-200 bg-white shadow-lg">
            <div className="border-b border-gray-100 p-2">
              <input
                type="text"
                value={search}
                onChange={(e) => setSearch(e.target.value)}
                placeholder={t("searchPlaceholder")}
                autoFocus
                className="w-full rounded-md border border-gray-300 px-2 py-1.5 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
              />
            </div>
            <div ref={listRef} className="max-h-48 overflow-auto">
              {filteredSlots.map((slot) => (
                <button
                  key={slot.value}
                  type="button"
                  data-value={slot.value}
                  className={cn(
                    "w-full px-3 py-1.5 text-left text-sm hover:bg-gray-50",
                    value === slot.value && "bg-blue-50 font-medium text-blue-700"
                  )}
                  onClick={() => handleSelect(slot.value)}
                >
                  {slot.display}
                </button>
              ))}
              {filteredSlots.length === 0 && (
                <p className="px-3 py-2 text-sm text-gray-500">{t("noResults")}</p>
              )}
            </div>
          </div>
        )}
      </div>

      {error && <p className="mt-1 text-sm text-red-600">{error}</p>}
      {field.help_text && !error && <p className="mt-1 text-sm text-gray-500">{field.help_text}</p>}
    </div>
  );
}
