import { type ClassValue, clsx } from "clsx";

export function cn(...inputs: ClassValue[]) {
  return clsx(inputs);
}

export function formatCurrency(amount: number | null | undefined, currency = "USD"): string {
  const safeAmount = amount ?? 0;
  if (isNaN(safeAmount)) return "$0.00";
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency,
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(safeAmount);
}

export function formatDate(date: string | Date | null | undefined, options?: Intl.DateTimeFormatOptions): string {
  if (!date) return "-";
  const d = typeof date === "string" ? new Date(date) : date;
  if (isNaN(d.getTime())) return "-";
  return new Intl.DateTimeFormat("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
    ...options,
  }).format(d);
}

export function formatNumber(num: number | null | undefined, decimals = 0): string {
  const safeNum = num ?? 0;
  if (isNaN(safeNum)) return "0";
  return new Intl.NumberFormat("en-US", {
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals,
  }).format(safeNum);
}

export function formatDateTime(date: string | Date | null | undefined): string {
  if (!date) return "-";
  const d = typeof date === "string" ? new Date(date) : date;
  if (isNaN(d.getTime())) return "-";
  return new Intl.DateTimeFormat("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  }).format(d);
}

export function statusColor(status: string): string {
  const s = status.toLowerCase();
  if (s === "draft") return "gray";
  if (s === "released" || s === "open" || s === "active") return "blue";
  if (s === "in_progress" || s === "in progress" || s === "pending") return "amber";
  if (s === "completed" || s === "done" || s === "approved") return "green";
  if (s === "closed" || s === "cancelled" || s === "rejected") return "red";
  return "gray";
}
