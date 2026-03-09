"use client";

import { useState } from "react";
import { Plus, Trash2, ArrowUp, ArrowDown, GripVertical } from "lucide-react";
import { useTranslations } from "next-intl";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import type { TabGroupConfig } from "@/lib/types/lowcode";

interface TabGroupEditorProps {
  tabGroups: TabGroupConfig[];
  onUpdate: (groups: TabGroupConfig[]) => void;
}

function generateId() {
  return `tab_${Date.now()}_${Math.random().toString(36).substring(2, 7)}`;
}

export function TabGroupEditor({ tabGroups, onUpdate }: TabGroupEditorProps) {
  const [expandedId, setExpandedId] = useState<string | null>(null);
  const t = useTranslations("lowcode");

  const iconOptions = [
    { value: "", label: t("iconNone") },
    { value: "file-text", label: t("iconFileText") },
    { value: "settings", label: t("iconSettings") },
    { value: "user", label: t("iconUser") },
    { value: "package", label: t("iconPackage") },
    { value: "dollar-sign", label: t("iconDollarSign") },
    { value: "calendar", label: t("iconCalendar") },
    { value: "clipboard", label: t("iconClipboard") },
    { value: "truck", label: t("iconTruck") },
    { value: "shield", label: t("iconShield") },
    { value: "heart", label: t("iconHeart") },
    { value: "star", label: t("iconStar") },
    { value: "tag", label: t("iconTag") },
    { value: "map-pin", label: t("iconMapPin") },
    { value: "info", label: t("iconInfo") },
  ];

  const addTabGroup = () => {
    const newGroup: TabGroupConfig = {
      id: generateId(),
      label: `Tab ${tabGroups.length + 1}`,
      sort_order: tabGroups.length,
    };
    onUpdate([...tabGroups, newGroup]);
    setExpandedId(newGroup.id);
  };

  const removeTabGroup = (id: string) => {
    const filtered = tabGroups
      .filter((g) => g.id !== id)
      .map((g, i) => ({ ...g, sort_order: i }));
    onUpdate(filtered);
    if (expandedId === id) setExpandedId(null);
  };

  const updateTabGroup = (id: string, updates: Partial<TabGroupConfig>) => {
    onUpdate(
      tabGroups.map((g) => (g.id === id ? { ...g, ...updates } : g))
    );
  };

  const moveTabGroup = (index: number, direction: -1 | 1) => {
    const targetIndex = index + direction;
    if (targetIndex < 0 || targetIndex >= tabGroups.length) return;
    const newGroups = [...tabGroups];
    const [moved] = newGroups.splice(index, 1);
    newGroups.splice(targetIndex, 0, moved);
    onUpdate(newGroups.map((g, i) => ({ ...g, sort_order: i })));
  };

  return (
    <div className="space-y-2">
      <div className="flex items-center justify-between">
        <h4 className="text-sm font-semibold text-gray-900">{t("tabGroups")}</h4>
        <Button type="button" variant="ghost" size="sm" onClick={addTabGroup}>
          <Plus className="h-3.5 w-3.5" />
          {t("addTab")}
        </Button>
      </div>

      {tabGroups.length === 0 && (
        <p className="text-xs text-gray-400 py-2">
          {t("noTabsDefined")}
        </p>
      )}

      <div className="space-y-1">
        {tabGroups.map((group, index) => (
          <div
            key={group.id}
            className="rounded-md border border-gray-200 bg-white"
          >
            {/* Tab header row */}
            <div className="flex items-center gap-2 px-2 py-1.5">
              <GripVertical className="h-3.5 w-3.5 text-gray-300 shrink-0" />

              <button
                type="button"
                className="flex-1 text-left text-sm font-medium text-gray-700 truncate"
                onClick={() =>
                  setExpandedId(expandedId === group.id ? null : group.id)
                }
              >
                {group.label || t("untitledTab")}
              </button>

              <div className="flex items-center gap-0.5 shrink-0">
                <button
                  type="button"
                  onClick={() => moveTabGroup(index, -1)}
                  disabled={index === 0}
                  className="rounded p-0.5 text-gray-400 hover:text-gray-600 disabled:opacity-30"
                  title={t("moveUp")}
                >
                  <ArrowUp className="h-3 w-3" />
                </button>
                <button
                  type="button"
                  onClick={() => moveTabGroup(index, 1)}
                  disabled={index === tabGroups.length - 1}
                  className="rounded p-0.5 text-gray-400 hover:text-gray-600 disabled:opacity-30"
                  title={t("moveDown")}
                >
                  <ArrowDown className="h-3 w-3" />
                </button>
                <button
                  type="button"
                  onClick={() => removeTabGroup(group.id)}
                  className="rounded p-0.5 text-red-400 hover:text-red-600"
                  title={t("removeTab")}
                >
                  <Trash2 className="h-3 w-3" />
                </button>
              </div>
            </div>

            {/* Expanded edit section */}
            {expandedId === group.id && (
              <div className="border-t border-gray-100 px-3 py-2 space-y-2 bg-gray-50">
                <Input
                  label={t("label")}
                  value={group.label}
                  onChange={(e) =>
                    updateTabGroup(group.id, { label: e.target.value })
                  }
                  placeholder={t("tabLabelPlaceholder")}
                />
                <div>
                  <label className="mb-1 block text-sm font-medium text-gray-700">
                    {t("icon")}
                  </label>
                  <select
                    value={group.icon || ""}
                    onChange={(e) =>
                      updateTabGroup(group.id, {
                        icon: e.target.value || undefined,
                      })
                    }
                    className="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                  >
                    {iconOptions.map((opt) => (
                      <option key={opt.value} value={opt.value}>
                        {opt.label}
                      </option>
                    ))}
                  </select>
                </div>
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
