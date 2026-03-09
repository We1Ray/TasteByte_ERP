"use client";

import { LayoutList, Calendar, Columns3 } from "lucide-react";
import { useTranslations } from "next-intl";
import { cn } from "@/lib/utils";

interface ViewSelectorProps {
  views: string[];
  activeView: string;
  onViewChange: (view: string) => void;
}

const viewIcons: Record<string, React.ReactNode> = {
  list: <LayoutList className="h-4 w-4" />,
  calendar: <Calendar className="h-4 w-4" />,
  kanban: <Columns3 className="h-4 w-4" />,
};

const viewLabelKeys: Record<string, string> = {
  list: "viewList",
  calendar: "viewCalendar",
  kanban: "viewKanban",
};

export function ViewSelector({ views, activeView, onViewChange }: ViewSelectorProps) {
  const t = useTranslations("lowcode");

  return (
    <div className="flex items-center gap-1 border-b border-gray-200">
      {views.map((view) => (
        <button
          key={view}
          type="button"
          onClick={() => onViewChange(view)}
          className={cn(
            "inline-flex items-center gap-1.5 px-3 py-2 text-sm font-medium transition-colors",
            "border-b-2 -mb-px",
            activeView === view
              ? "border-blue-600 text-blue-600"
              : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300"
          )}
        >
          {viewIcons[view] || <LayoutList className="h-4 w-4" />}
          {viewLabelKeys[view] ? t(viewLabelKeys[view]) : view.charAt(0).toUpperCase() + view.slice(1)}
        </button>
      ))}
    </div>
  );
}
