"use client";

import { useState, useMemo } from "react";
import { useTranslations } from "next-intl";
import { cn } from "@/lib/utils";
import type { FormSection, TabGroupConfig } from "@/lib/types/lowcode";

interface DynamicTabsProps {
  tabGroups: TabGroupConfig[];
  sections: FormSection[];
  sectionRenderer: (section: FormSection) => React.ReactNode;
}

const DEFAULT_TAB_ID = "__general__";

export function DynamicTabs({
  tabGroups,
  sections,
  sectionRenderer,
}: DynamicTabsProps) {
  const t = useTranslations("lowcode");
  // Build the tab list: sorted tab groups + an optional general tab for unassigned sections
  const tabs = useMemo(() => {
    const sorted = [...tabGroups].sort((a, b) => a.sort_order - b.sort_order);

    // Check if any sections are unassigned to a tab group
    const sectionTabIds = new Set(tabGroups.map((g) => g.id));
    const unassigned = sections.filter((s) => {
      const tabId = (s as FormSection & { tab_group_id?: string }).tab_group_id;
      return !tabId || !sectionTabIds.has(tabId);
    });

    const result: { id: string; label: string; icon?: string }[] = [];

    if (unassigned.length > 0) {
      result.push({ id: DEFAULT_TAB_ID, label: t("basic") });
    }

    for (const g of sorted) {
      result.push({ id: g.id, label: g.label, icon: g.icon });
    }

    return result;
  }, [tabGroups, sections, t]);

  const [activeTabId, setActiveTabId] = useState<string>(
    tabs[0]?.id || DEFAULT_TAB_ID
  );

  // Ensure active tab is valid
  const currentTabId = tabs.find((t) => t.id === activeTabId)
    ? activeTabId
    : tabs[0]?.id || DEFAULT_TAB_ID;

  // Get sections for the active tab
  const tabSections = useMemo(() => {
    const tabGroupIds = new Set(tabGroups.map((g) => g.id));

    return sections
      .filter((section) => {
        const tabId = (section as FormSection & { tab_group_id?: string })
          .tab_group_id;

        if (currentTabId === DEFAULT_TAB_ID) {
          // Show sections with no tab_group_id or an invalid one
          return !tabId || !tabGroupIds.has(tabId);
        }

        return tabId === currentTabId;
      })
      .sort((a, b) => a.sort_order - b.sort_order);
  }, [sections, currentTabId, tabGroups]);

  if (tabs.length === 0) {
    // No tabs at all - render all sections
    return (
      <div className="space-y-6">
        {[...sections]
          .sort((a, b) => a.sort_order - b.sort_order)
          .map((section) => (
            <div key={section.id}>{sectionRenderer(section)}</div>
          ))}
      </div>
    );
  }

  return (
    <div>
      {/* Tab bar */}
      <div className="flex items-center gap-0.5 border-b border-gray-200 mb-6">
        {tabs.map((tab) => (
          <button
            key={tab.id}
            type="button"
            onClick={() => setActiveTabId(tab.id)}
            className={cn(
              "px-4 py-2.5 text-sm font-medium transition-colors",
              "border-b-2 -mb-px whitespace-nowrap",
              currentTabId === tab.id
                ? "border-blue-600 text-blue-600"
                : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300"
            )}
          >
            {tab.label}
          </button>
        ))}
      </div>

      {/* Tab content */}
      <div className="space-y-6">
        {tabSections.length === 0 ? (
          <div className="py-8 text-center text-sm text-gray-400">
            {t("noSectionsAvailable")}
          </div>
        ) : (
          tabSections.map((section) => (
            <div key={section.id}>{sectionRenderer(section)}</div>
          ))
        )}
      </div>
    </div>
  );
}
