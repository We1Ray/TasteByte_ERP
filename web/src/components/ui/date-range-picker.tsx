"use client";

import { useMemo } from "react";
import { cn } from "@/lib/utils";

interface DateRangePickerProps {
  startDate: string;
  endDate: string;
  onStartDateChange: (date: string) => void;
  onEndDateChange: (date: string) => void;
  presets?: boolean;
  className?: string;
}

function toDateString(date: Date): string {
  return date.toISOString().split("T")[0];
}

export function DateRangePicker({
  startDate,
  endDate,
  onStartDateChange,
  onEndDateChange,
  presets = true,
  className,
}: DateRangePickerProps) {
  const presetButtons = useMemo(() => {
    const now = new Date();
    const today = toDateString(now);

    const daysAgo = (n: number) => {
      const d = new Date(now);
      d.setDate(d.getDate() - n);
      return toDateString(d);
    };

    const startOfMonth = toDateString(new Date(now.getFullYear(), now.getMonth(), 1));
    const quarter = Math.floor(now.getMonth() / 3);
    const startOfQuarter = toDateString(new Date(now.getFullYear(), quarter * 3, 1));
    const startOfYear = toDateString(new Date(now.getFullYear(), 0, 1));

    return [
      { label: "Today", start: today, end: today },
      { label: "7 Days", start: daysAgo(7), end: today },
      { label: "30 Days", start: daysAgo(30), end: today },
      { label: "This Month", start: startOfMonth, end: today },
      { label: "This Quarter", start: startOfQuarter, end: today },
      { label: "This Year", start: startOfYear, end: today },
    ];
  }, []);

  return (
    <div className={cn("flex flex-wrap items-end gap-3", className)}>
      {presets && (
        <div className="flex flex-wrap gap-1.5">
          {presetButtons.map((preset) => (
            <button
              key={preset.label}
              type="button"
              onClick={() => {
                onStartDateChange(preset.start);
                onEndDateChange(preset.end);
              }}
              className={cn(
                "rounded-md border px-2.5 py-1.5 text-xs font-medium transition-colors",
                startDate === preset.start && endDate === preset.end
                  ? "border-blue-600 bg-blue-50 text-blue-700"
                  : "border-gray-300 bg-white text-gray-700 hover:bg-gray-50"
              )}
            >
              {preset.label}
            </button>
          ))}
        </div>
      )}
      <div className="flex items-end gap-2">
        <div className="w-full">
          <label className="mb-1 block text-sm font-medium text-gray-700">From</label>
          <input
            type="date"
            value={startDate}
            onChange={(e) => onStartDateChange(e.target.value)}
            className="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm transition-colors focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
          />
        </div>
        <div className="w-full">
          <label className="mb-1 block text-sm font-medium text-gray-700">To</label>
          <input
            type="date"
            value={endDate}
            onChange={(e) => onEndDateChange(e.target.value)}
            className="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm transition-colors focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
          />
        </div>
      </div>
    </div>
  );
}
