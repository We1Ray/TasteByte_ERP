"use client";

import { Plus, Pencil, Trash2, Rocket, RotateCcw } from "lucide-react";
import { useTranslations } from "next-intl";
import { Badge } from "@/components/ui/badge";
import { formatDateTime } from "@/lib/utils";
import type { JournalEntry } from "@/lib/types/lowcode";

interface JournalTimelineProps {
  entries: JournalEntry[];
  onViewDiff?: (entry: JournalEntry) => void;
  onRollback?: (entry: JournalEntry) => void;
}

const changeTypeConfig: Record<string, { icon: React.ElementType; color: string; badgeColor: string }> = {
  create: { icon: Plus, color: "text-green-500", badgeColor: "green" },
  update: { icon: Pencil, color: "text-blue-500", badgeColor: "blue" },
  delete: { icon: Trash2, color: "text-red-500", badgeColor: "red" },
  publish: { icon: Rocket, color: "text-purple-500", badgeColor: "purple" },
  rollback: { icon: RotateCcw, color: "text-amber-500", badgeColor: "amber" },
};

const changeTypeKeys: Record<string, string> = {
  create: "changeCreate",
  update: "changeUpdate",
  delete: "changeDelete",
  publish: "changePublish",
  rollback: "changeRollback",
};

export function JournalTimeline({ entries, onViewDiff, onRollback }: JournalTimelineProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");

  if (entries.length === 0) {
    return (
      <div className="py-12 text-center text-sm text-gray-500">
        {t("noJournalEntries")}
      </div>
    );
  }

  return (
    <div className="relative">
      <div className="absolute left-5 top-0 h-full w-0.5 bg-gray-200" />

      <div className="space-y-6">
        {entries.map((entry) => {
          const config = changeTypeConfig[entry.change_type.toLowerCase()] || changeTypeConfig.update;
          const Icon = config.icon;

          return (
            <div key={entry.id} className="relative flex gap-4">
              <div className={`relative z-10 flex h-10 w-10 shrink-0 items-center justify-center rounded-full border-2 border-white bg-gray-100 ${config.color}`}>
                <Icon className="h-4 w-4" />
              </div>

              <div className="flex-1 rounded-lg border border-gray-200 bg-white p-4">
                <div className="flex items-start justify-between">
                  <div>
                    <div className="flex items-center gap-2">
                      <Badge color={config.badgeColor as "green" | "blue" | "red" | "purple" | "amber"}>
                        {t(changeTypeKeys[entry.change_type.toLowerCase()] || entry.change_type)}
                      </Badge>
                      <span className="text-sm font-medium text-gray-900">
                        {entry.user_name ?? entry.changed_by}
                      </span>
                      {entry.version != null && (
                        <span className="text-xs text-gray-400">v{entry.version}</span>
                      )}
                    </div>
                    <p className="mt-1 text-sm text-gray-600">{entry.diff_summary ?? ""}</p>
                    <p className="mt-1 text-xs text-gray-400">{formatDateTime(entry.created_at)}</p>
                  </div>

                  <div className="flex items-center gap-2">
                    {entry.old_values !== undefined && entry.new_values !== undefined && onViewDiff && (
                      <button
                        onClick={() => onViewDiff(entry)}
                        className="text-xs text-blue-600 hover:text-blue-700"
                      >
                        {t("viewDiff")}
                      </button>
                    )}
                    {onRollback && entry.change_type !== "create" && (
                      <button
                        onClick={() => onRollback(entry)}
                        className="text-xs text-amber-600 hover:text-amber-700"
                      >
                        {tCommon("rollback")}
                      </button>
                    )}
                  </div>
                </div>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
