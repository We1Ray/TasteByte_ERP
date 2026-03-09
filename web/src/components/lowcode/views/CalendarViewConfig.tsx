"use client";

import { useTranslations } from "next-intl";
import { Select } from "@/components/ui/select";
import type { CalendarViewConfig } from "@/lib/types/lowcode";

interface CalendarViewConfigProps {
  config: CalendarViewConfig;
  onChange: (config: CalendarViewConfig) => void;
  availableFields: { key: string; label: string }[];
}

export function CalendarViewConfigPanel({
  config,
  onChange,
  availableFields,
}: CalendarViewConfigProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const fieldOptions = availableFields.map((f) => ({
    value: f.key,
    label: f.label,
  }));

  const optionalFieldOptions = [
    { value: "", label: "-- None --" },
    ...fieldOptions,
  ];

  return (
    <div className="space-y-4">
      <h4 className="text-sm font-semibold text-gray-900">{t("calendarSettings")}</h4>

      <Select
        label={t("startDateField")}
        required
        options={fieldOptions}
        placeholder="Select date field"
        value={config.dateField || ""}
        onChange={(e) =>
          onChange({ ...config, dateField: e.target.value })
        }
      />

      <Select
        label={t("titleField")}
        required
        options={fieldOptions}
        placeholder="Select title field"
        value={config.titleField || ""}
        onChange={(e) =>
          onChange({ ...config, titleField: e.target.value })
        }
      />

      <Select
        label={`${t("color")} (${tCommon("optional")})`}
        options={optionalFieldOptions}
        value={config.colorField || ""}
        onChange={(e) =>
          onChange({
            ...config,
            colorField: e.target.value || undefined,
          })
        }
      />
    </div>
  );
}
