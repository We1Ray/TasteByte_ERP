"use client";

import { X } from "lucide-react";
import { cn } from "@/lib/utils";
import { Button } from "./button";

interface BulkActionBarProps {
  selectedCount: number;
  onClearSelection: () => void;
  children: React.ReactNode;
  className?: string;
}

export function BulkActionBar({ selectedCount, onClearSelection, children, className }: BulkActionBarProps) {
  if (selectedCount === 0) return null;

  return (
    <div className={cn(
      "flex items-center justify-between rounded-lg border border-blue-200 bg-blue-50 px-4 py-3 mb-3",
      className
    )}>
      <div className="flex items-center gap-3">
        <span className="text-sm font-medium text-blue-700">
          {selectedCount} selected
        </span>
        <Button variant="ghost" size="sm" onClick={onClearSelection} className="text-blue-600 hover:text-blue-800">
          <X className="h-3 w-3" />
          Clear
        </Button>
      </div>
      <div className="flex items-center gap-2">
        {children}
      </div>
    </div>
  );
}
