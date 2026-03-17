"use client";

import { cn } from "@/lib/utils";
import { formatCurrency, formatNumber } from "@/lib/utils";
import { Card } from "./card";

interface KPIItem {
  label: string;
  value: string | number | null | undefined;
  format?: "currency" | "number" | "percentage" | "plain";
  color?: "blue" | "green" | "amber" | "red" | "purple";
  prefix?: string;
  suffix?: string;
}

interface KPIGridProps {
  items: KPIItem[];
  columns?: 2 | 3 | 4;
  isLoading?: boolean;
}

const colorClasses: Record<string, string> = {
  blue: "text-blue-700",
  green: "text-green-700",
  amber: "text-amber-700",
  red: "text-red-700",
  purple: "text-purple-700",
};

const gridCols = {
  2: "sm:grid-cols-2",
  3: "sm:grid-cols-3",
  4: "sm:grid-cols-2 lg:grid-cols-4",
} as const;

function formatKPIValue(item: KPIItem): string {
  const { value, format, prefix = "", suffix = "" } = item;
  if (value === undefined || value === null) {
    return `${prefix}0${suffix}`;
  }
  const num = typeof value === "number" ? value : parseFloat(String(value));

  if (format === "plain" || isNaN(num)) {
    return `${prefix}${value}${suffix}`;
  }

  let formatted: string;
  switch (format) {
    case "currency":
      formatted = formatCurrency(num);
      break;
    case "percentage":
      formatted = `${(num * 100).toFixed(1)}%`;
      break;
    case "number":
      formatted = formatNumber(num);
      break;
    default:
      formatted = formatNumber(num);
      break;
  }

  return `${prefix}${formatted}${suffix}`;
}

export function KPIGrid({ items, columns = 4, isLoading }: KPIGridProps) {
  if (isLoading) {
    return (
      <div className={cn("grid grid-cols-1 gap-4", gridCols[columns])}>
        {items.map((_, i) => (
          <Card key={i}>
            <div className="animate-pulse">
              <div className="h-4 w-24 rounded bg-gray-200" />
              <div className="mt-2 h-8 w-32 rounded bg-gray-200" />
            </div>
          </Card>
        ))}
      </div>
    );
  }

  return (
    <div className={cn("grid grid-cols-1 gap-4", gridCols[columns])}>
      {items.map((item) => (
        <Card key={item.label}>
          <p className="text-sm text-gray-500">{item.label}</p>
          <p
            className={cn(
              "mt-1 text-2xl font-bold",
              item.color ? colorClasses[item.color] : "text-gray-900"
            )}
          >
            {formatKPIValue(item)}
          </p>
        </Card>
      ))}
    </div>
  );
}
