"use client";

import { useState } from "react";
import { ChevronDown, ChevronRight } from "lucide-react";
import { Card } from "@/components/ui/card";
import { cn } from "@/lib/utils";
import { DynamicField } from "./DynamicField";
import type { FormSection } from "@/lib/types/lowcode";

interface DynamicSectionProps {
  section: FormSection;
  disabled?: boolean;
}

const columnClasses: Record<number, string> = {
  1: "grid-cols-1",
  2: "grid-cols-1 sm:grid-cols-2",
  3: "grid-cols-1 sm:grid-cols-2 lg:grid-cols-3",
  4: "grid-cols-1 sm:grid-cols-2 lg:grid-cols-4",
};

export function DynamicSection({ section, disabled }: DynamicSectionProps) {
  const [collapsed, setCollapsed] = useState(section.collapsed_default);

  const sortedFields = [...section.fields].sort((a, b) => a.sort_order - b.sort_order);

  return (
    <Card>
      <div
        className={cn(
          "flex items-center gap-2",
          section.collapsible && "cursor-pointer"
        )}
        onClick={() => section.collapsible && setCollapsed(!collapsed)}
      >
        {section.collapsible && (
          collapsed ? <ChevronRight className="h-4 w-4 text-gray-400" /> : <ChevronDown className="h-4 w-4 text-gray-400" />
        )}
        <div>
          <h3 className="text-base font-semibold text-gray-900">{section.title}</h3>
          {section.description && (
            <p className="mt-0.5 text-sm text-gray-500">{section.description}</p>
          )}
        </div>
      </div>

      {!collapsed && (
        <div className={cn("mt-4 grid gap-4", columnClasses[section.columns] || "grid-cols-1")}>
          {sortedFields.map((field) => (
            <div key={field.id} className={field.width === "full" ? "col-span-full" : undefined}>
              <DynamicField field={field} disabled={disabled} />
            </div>
          ))}
        </div>
      )}
    </Card>
  );
}
