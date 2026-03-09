"use client";

import { cn } from "@/lib/utils";

interface TabItem {
  key: string;
  label: string;
}

interface TabsProps {
  tabs: TabItem[];
  activeTab: string;
  onTabChange: (key: string) => void;
  className?: string;
}

export function Tabs({ tabs, activeTab, onTabChange, className }: TabsProps) {
  return (
    <div className={cn("mb-6 border-b border-gray-200", className)}>
      <nav className="-mb-px flex gap-4">
        {tabs.map((tab) => (
          <button
            key={tab.key}
            onClick={() => onTabChange(tab.key)}
            className={cn(
              "whitespace-nowrap border-b-2 px-1 py-3 text-sm font-medium transition-colors",
              activeTab === tab.key
                ? "border-blue-600 text-blue-600"
                : "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700"
            )}
          >
            {tab.label}
          </button>
        ))}
      </nav>
    </div>
  );
}
