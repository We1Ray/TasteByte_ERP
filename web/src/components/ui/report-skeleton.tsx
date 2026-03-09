"use client";

import { Card } from "./card";

interface ReportSkeletonProps {
  rows?: number;
}

export function ReportSkeleton({ rows = 8 }: ReportSkeletonProps) {
  return (
    <Card>
      <div className="animate-pulse space-y-3">
        {Array.from({ length: rows }).map((_, i) => (
          <div key={i} className="flex gap-4">
            <div className="h-4 flex-1 rounded bg-gray-200" />
            <div className="h-4 w-24 rounded bg-gray-200" />
            <div className="h-4 w-24 rounded bg-gray-200" />
          </div>
        ))}
      </div>
    </Card>
  );
}
