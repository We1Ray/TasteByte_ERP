"use client";

import { RefreshCw } from "lucide-react";
import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import { PrintButton } from "./print-button";
import { ExportButton } from "./export-button";
import { DateRangePicker } from "@/components/ui/date-range-picker";
import { Input } from "@/components/ui/input";

interface DateFilterRange {
  type: "range";
  startDate: string;
  endDate: string;
  onStartDateChange: (date: string) => void;
  onEndDateChange: (date: string) => void;
}

interface DateFilterSingle {
  type: "single";
  label?: string;
  value: string;
  onChange: (date: string) => void;
}

interface ReportActionBarProps {
  exportData: unknown[];
  exportFilename: string;
  exportSheetName?: string;
  showPrint?: boolean;
  dateFilter?: DateFilterRange | DateFilterSingle;
  onRefresh?: () => void;
  isRefreshing?: boolean;
  children?: React.ReactNode;
  className?: string;
}

export function ReportActionBar({
  exportData,
  exportFilename,
  exportSheetName,
  showPrint = true,
  dateFilter,
  onRefresh,
  isRefreshing,
  children,
  className,
}: ReportActionBarProps) {
  return (
    <div className={cn(
      "mb-4 flex items-center justify-between gap-4 rounded-lg border border-gray-200 bg-white px-4 py-3",
      className
    )}>
      {/* Left: Date filters */}
      <div className="flex items-center gap-3">
        {dateFilter?.type === "range" && (
          <DateRangePicker
            startDate={dateFilter.startDate}
            endDate={dateFilter.endDate}
            onStartDateChange={dateFilter.onStartDateChange}
            onEndDateChange={dateFilter.onEndDateChange}
          />
        )}
        {dateFilter?.type === "single" && (
          <Input
            label={dateFilter.label || "As of Date"}
            type="date"
            value={dateFilter.value}
            onChange={(e) => dateFilter.onChange(e.target.value)}
            className="w-48"
          />
        )}
        {!dateFilter && <div />}
      </div>

      {/* Right: Actions */}
      <div className="flex items-center gap-2">
        {children}
        {onRefresh && (
          <Button
            variant="secondary"
            size="icon"
            onClick={onRefresh}
            disabled={isRefreshing}
          >
            <RefreshCw className={cn("h-4 w-4", isRefreshing && "animate-spin")} />
          </Button>
        )}
        {showPrint && <PrintButton />}
        <ExportButton data={exportData} filename={exportFilename} sheetName={exportSheetName} />
      </div>
    </div>
  );
}
