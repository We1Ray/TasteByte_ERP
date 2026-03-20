"use client";

import { useMemo } from "react";
import { useTranslations } from "next-intl";
import { Plus, Minus, RefreshCw } from "lucide-react";

interface FormField {
  field_name?: string;
  field_key?: string;
  field_label?: string;
  label?: string;
  field_type?: string;
  is_required?: boolean;
  [key: string]: unknown;
}

interface FormSection {
  title: string;
  fields: FormField[] | { field: FormField }[];
}

interface FormSnapshot {
  sections?: FormSection[];
  form?: { layout_config?: unknown; form_settings?: unknown };
}

interface FormDefinitionDiffProps {
  before?: FormSnapshot | null;
  after?: FormSnapshot | null;
}

interface DiffItem {
  type: "added" | "removed" | "modified";
  fieldName: string;
  fieldLabel: string;
  fieldType: string;
  sectionTitle: string;
  changes?: { key: string; before: unknown; after: unknown }[];
}

function getFieldName(field: FormField): string {
  return field.field_name || field.field_key || "";
}

function getFieldLabel(field: FormField): string {
  return field.field_label || field.label || "";
}

function getFieldType(field: FormField): string {
  return field.field_type || "";
}

function extractFields(
  snapshot: FormSnapshot | null | undefined
): Map<string, { field: FormField; sectionTitle: string }> {
  const map = new Map<string, { field: FormField; sectionTitle: string }>();
  if (!snapshot?.sections) return map;
  for (const section of snapshot.sections) {
    const fields = section.fields || [];
    for (const f of fields) {
      const field =
        "field" in f ? (f as { field: FormField }).field : (f as FormField);
      const name = getFieldName(field);
      if (name) {
        map.set(name, { field, sectionTitle: section.title });
      }
    }
  }
  return map;
}

export function FormDefinitionDiff({ before, after }: FormDefinitionDiffProps) {
  const t = useTranslations("lowcode");

  const diffs = useMemo(() => {
    const beforeFields = extractFields(before);
    const afterFields = extractFields(after);
    const items: DiffItem[] = [];

    // Find added and modified fields
    for (const [name, { field, sectionTitle }] of afterFields) {
      const beforeEntry = beforeFields.get(name);
      if (!beforeEntry) {
        items.push({
          type: "added",
          fieldName: name,
          fieldLabel: getFieldLabel(field),
          fieldType: getFieldType(field),
          sectionTitle,
        });
      } else {
        // Check for modifications
        const changes: { key: string; before: unknown; after: unknown }[] = [];
        const compareKeys = [
          "field_label",
          "label",
          "field_type",
          "is_required",
          "is_unique",
          "default_value",
          "validation_regex",
          "min_value",
          "max_value",
          "min_length",
          "max_length",
        ];
        for (const key of compareKeys) {
          const bVal = (beforeEntry.field as Record<string, unknown>)[key];
          const aVal = (field as Record<string, unknown>)[key];
          if (JSON.stringify(bVal) !== JSON.stringify(aVal)) {
            changes.push({ key, before: bVal, after: aVal });
          }
        }
        if (changes.length > 0) {
          items.push({
            type: "modified",
            fieldName: name,
            fieldLabel: getFieldLabel(field),
            fieldType: getFieldType(field),
            sectionTitle,
            changes,
          });
        }
      }
    }

    // Find removed fields
    for (const [name, { field, sectionTitle }] of beforeFields) {
      if (!afterFields.has(name)) {
        items.push({
          type: "removed",
          fieldName: name,
          fieldLabel: getFieldLabel(field),
          fieldType: getFieldType(field),
          sectionTitle,
        });
      }
    }

    return items;
  }, [before, after]);

  if (diffs.length === 0) {
    return (
      <p className="py-4 text-center text-sm text-gray-400">
        {t("noChanges")}
      </p>
    );
  }

  const added = diffs.filter((d) => d.type === "added");
  const removed = diffs.filter((d) => d.type === "removed");
  const modified = diffs.filter((d) => d.type === "modified");

  return (
    <div className="space-y-3">
      <div className="flex gap-4 text-xs text-gray-500">
        <span className="flex items-center gap-1">
          <Plus className="h-3 w-3 text-green-600" /> {t("diffAdded")}:{" "}
          {added.length}
        </span>
        <span className="flex items-center gap-1">
          <Minus className="h-3 w-3 text-red-600" /> {t("diffRemoved")}:{" "}
          {removed.length}
        </span>
        <span className="flex items-center gap-1">
          <RefreshCw className="h-3 w-3 text-yellow-600" /> {t("diffModified")}
          : {modified.length}
        </span>
      </div>

      {added.map((d) => (
        <div
          key={d.fieldName}
          className="flex items-start gap-2 rounded-md border border-green-200 bg-green-50 p-3"
        >
          <Plus className="mt-0.5 h-4 w-4 shrink-0 text-green-600" />
          <div>
            <p className="text-sm font-medium text-green-800">
              {d.fieldLabel}{" "}
              <span className="font-mono text-xs text-green-600">
                ({d.fieldName})
              </span>
            </p>
            <p className="text-xs text-green-600">
              {d.fieldType} &middot; {d.sectionTitle}
            </p>
          </div>
        </div>
      ))}

      {removed.map((d) => (
        <div
          key={d.fieldName}
          className="flex items-start gap-2 rounded-md border border-red-200 bg-red-50 p-3"
        >
          <Minus className="mt-0.5 h-4 w-4 shrink-0 text-red-600" />
          <div>
            <p className="text-sm font-medium text-red-800 line-through">
              {d.fieldLabel}{" "}
              <span className="font-mono text-xs text-red-600">
                ({d.fieldName})
              </span>
            </p>
            <p className="text-xs text-red-600">
              {d.fieldType} &middot; {d.sectionTitle}
            </p>
          </div>
        </div>
      ))}

      {modified.map((d) => (
        <div
          key={d.fieldName}
          className="rounded-md border border-yellow-200 bg-yellow-50 p-3"
        >
          <div className="flex items-start gap-2">
            <RefreshCw className="mt-0.5 h-4 w-4 shrink-0 text-yellow-600" />
            <div className="flex-1">
              <p className="text-sm font-medium text-yellow-800">
                {d.fieldLabel}{" "}
                <span className="font-mono text-xs text-yellow-600">
                  ({d.fieldName})
                </span>
              </p>
              <div className="mt-2 space-y-1">
                {d.changes?.map((c) => (
                  <div key={c.key} className="flex gap-2 text-xs">
                    <span className="font-medium text-gray-600">{c.key}:</span>
                    <span className="text-red-600 line-through">
                      {String(c.before ?? "\u2014")}
                    </span>
                    <span className="text-gray-400">&rarr;</span>
                    <span className="text-green-600">
                      {String(c.after ?? "\u2014")}
                    </span>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>
      ))}
    </div>
  );
}
