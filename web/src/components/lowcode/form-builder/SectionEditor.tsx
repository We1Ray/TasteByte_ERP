"use client";

import { useTranslations } from "next-intl";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { useBuilderStore } from "@/lib/stores/builder-store";
import type { FormSection } from "@/lib/types/lowcode";

interface SectionEditorProps {
  section: FormSection;
}

export function SectionEditor({ section }: SectionEditorProps) {
  const { updateSection } = useBuilderStore();
  const t = useTranslations("lowcode");

  return (
    <div className="space-y-4">
      <h4 className="text-sm font-semibold text-gray-900">{t("sectionSettings")}</h4>

      <Input
        label={t("sectionTitle")}
        value={section.title}
        onChange={(e) => updateSection(section.id, { title: e.target.value })}
      />

      <Input
        label={t("sectionDescription")}
        value={section.description || ""}
        onChange={(e) => updateSection(section.id, { description: e.target.value })}
      />

      <Select
        label={t("columns")}
        value={String(section.columns)}
        onChange={(e) =>
          updateSection(section.id, { columns: Number(e.target.value) as 1 | 2 | 3 | 4 })
        }
        options={[
          { value: "1", label: t("column1") },
          { value: "2", label: t("column2") },
          { value: "3", label: t("column3") },
          { value: "4", label: t("column4") },
        ]}
      />

      <label className="flex items-center gap-2">
        <input
          type="checkbox"
          checked={section.collapsible}
          onChange={(e) => updateSection(section.id, { collapsible: e.target.checked })}
          className="h-4 w-4 rounded border-gray-300 text-blue-600"
        />
        <span className="text-sm text-gray-700">{t("collapsible")}</span>
      </label>

      {section.collapsible && (
        <label className="flex items-center gap-2">
          <input
            type="checkbox"
            checked={section.collapsed_default}
            onChange={(e) => updateSection(section.id, { collapsed_default: e.target.checked })}
            className="h-4 w-4 rounded border-gray-300 text-blue-600"
          />
          <span className="text-sm text-gray-700">{t("collapsedDefault")}</span>
        </label>
      )}
    </div>
  );
}
