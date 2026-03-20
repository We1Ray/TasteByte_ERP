"use client";

import { useState } from "react";
import { useParams } from "next/navigation";
import { Database } from "lucide-react";
import { useTranslations } from "next-intl";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { useBuilderStore } from "@/lib/stores/builder-store";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { operationsApi } from "@/lib/api/lowcode";
import { SectionEditor } from "./SectionEditor";
import { TableSchemaBrowser } from "./TableSchemaBrowser";
import { MasterDetailConfigPanel } from "./MasterDetailConfigPanel";
import { ApprovalConfigPanel } from "./ApprovalConfigPanel";
import { FieldPermissionPanel } from "./FieldPermissionPanel";
import type { FieldDefinition } from "@/lib/types/lowcode";

function FieldPropertyEditor({ field }: { field: FieldDefinition }) {
  const { updateField } = useBuilderStore();
  const [schemaBrowserOpen, setSchemaBrowserOpen] = useState(false);
  const [permissionOpen, setPermissionOpen] = useState(false);
  const params = useParams();
  const operationId = params?.id as string;
  const { data: operation } = useApiQuery(
    ["lowcode", "operations", operationId],
    () => operationsApi.get(operationId),
    { enabled: !!operationId }
  );
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");

  const update = (updates: Partial<FieldDefinition>) => updateField(field.id, updates);

  return (
    <div className="space-y-4">
      <h4 className="text-sm font-semibold text-gray-900">{t("fieldSettings")}</h4>

      <Input
        label={t("label")}
        value={field.label}
        onChange={(e) => update({ label: e.target.value })}
      />

      <Input
        label={t("fieldKey")}
        value={field.field_key}
        onChange={(e) => update({ field_key: e.target.value })}
      />

      <Input
        label={t("placeholder")}
        value={field.placeholder || ""}
        onChange={(e) => update({ placeholder: e.target.value })}
      />

      <Input
        label={t("helpText")}
        value={field.help_text || ""}
        onChange={(e) => update({ help_text: e.target.value })}
      />

      <Input
        label={t("defaultValue")}
        value={field.default_value || ""}
        onChange={(e) => update({ default_value: e.target.value })}
      />

      <Select
        label={t("width")}
        value={field.width || "full"}
        onChange={(e) => update({ width: e.target.value as FieldDefinition["width"] })}
        options={[
          { value: "full", label: t("fullWidth") },
          { value: "half", label: t("halfWidth") },
          { value: "third", label: t("thirdWidth") },
          { value: "quarter", label: t("quarterWidth") },
        ]}
      />

      <div className="flex items-end gap-2">
        <div className="flex-1">
          <Input
            label={t("dbColumn")}
            value={field.db_column || ""}
            onChange={(e) => update({ db_column: e.target.value })}
          />
        </div>
        <button
          type="button"
          onClick={() => setSchemaBrowserOpen(true)}
          className="mb-[1px] rounded-md border border-gray-300 p-2 text-gray-500 hover:bg-gray-100 hover:text-gray-700"
          title={t("schemaBrowser")}
        >
          <Database className="h-4 w-4" />
        </button>
      </div>

      <TableSchemaBrowser
        open={schemaBrowserOpen}
        onClose={() => setSchemaBrowserOpen(false)}
        onSelect={(columnName) => update({ db_column: columnName })}
        currentTableName={operation?.table_name}
      />

      <label className="flex items-center gap-2">
        <input
          type="checkbox"
          checked={field.is_readonly}
          onChange={(e) => update({ is_readonly: e.target.checked })}
          className="h-4 w-4 rounded border-gray-300 text-blue-600"
        />
        <span className="text-sm text-gray-700">{t("readOnly")}</span>
      </label>

      <hr className="border-gray-200" />
      <h4 className="text-sm font-semibold text-gray-900">{t("validation")}</h4>

      <label className="flex items-center gap-2">
        <input
          type="checkbox"
          checked={field.validation.required}
          onChange={(e) => update({ validation: { ...field.validation, required: e.target.checked } })}
          className="h-4 w-4 rounded border-gray-300 text-blue-600"
        />
        <span className="text-sm text-gray-700">{t("required")}</span>
      </label>

      {(field.field_type === "text" || field.field_type === "textarea") && (
        <>
          <Input
            label={t("minLength")}
            type="number"
            value={field.validation.min_length ?? ""}
            onChange={(e) =>
              update({ validation: { ...field.validation, min_length: e.target.value ? Number(e.target.value) : undefined } })
            }
          />
          <Input
            label={t("maxLength")}
            type="number"
            value={field.validation.max_length ?? ""}
            onChange={(e) =>
              update({ validation: { ...field.validation, max_length: e.target.value ? Number(e.target.value) : undefined } })
            }
          />
          <Input
            label={t("regexPattern")}
            value={field.validation.regex_pattern || ""}
            onChange={(e) =>
              update({ validation: { ...field.validation, regex_pattern: e.target.value || undefined } })
            }
          />
          <Input
            label={t("regexError")}
            value={field.validation.regex_message || ""}
            onChange={(e) =>
              update({ validation: { ...field.validation, regex_message: e.target.value || undefined } })
            }
          />
        </>
      )}

      {field.field_type === "number" && (
        <>
          <Input
            label={t("minValue")}
            type="number"
            value={field.validation.min_value ?? ""}
            onChange={(e) =>
              update({ validation: { ...field.validation, min_value: e.target.value ? Number(e.target.value) : undefined } })
            }
          />
          <Input
            label={t("maxValue")}
            type="number"
            value={field.validation.max_value ?? ""}
            onChange={(e) =>
              update({ validation: { ...field.validation, max_value: e.target.value ? Number(e.target.value) : undefined } })
            }
          />
        </>
      )}

      {field.field_type === "file" && (
        <>
          <Input
            label={t("maxFileSizeMB")}
            type="number"
            value={field.validation.max_file_size ?? ""}
            onChange={(e) =>
              update({ validation: { ...field.validation, max_file_size: e.target.value ? Number(e.target.value) : undefined } })
            }
          />
          <Input
            label={t("allowedExtensions")}
            value={field.validation.allowed_extensions?.join(", ") ?? ""}
            placeholder={t("allowedExtensionsExample")}
            onChange={(e) =>
              update({
                validation: {
                  ...field.validation,
                  allowed_extensions: e.target.value
                    ? e.target.value.split(",").map((ext) => ext.trim()).filter(Boolean)
                    : undefined,
                },
              })
            }
          />
        </>
      )}

      {field.field_type === "toggle" && (
        <>
          <hr className="border-gray-200" />
          <h4 className="text-sm font-semibold text-gray-900">{t("toggleSettings")}</h4>
          <Input
            label={t("onLabel")}
            value={(field.field_config?.onLabel as string) || ""}
            placeholder={t("onDefault")}
            onChange={(e) =>
              update({ field_config: { ...field.field_config, onLabel: e.target.value || undefined } })
            }
          />
          <Input
            label={t("offLabel")}
            value={(field.field_config?.offLabel as string) || ""}
            placeholder={t("offDefault")}
            onChange={(e) =>
              update({ field_config: { ...field.field_config, offLabel: e.target.value || undefined } })
            }
          />
        </>
      )}

      {field.field_type === "color" && (
        <>
          <hr className="border-gray-200" />
          <h4 className="text-sm font-semibold text-gray-900">{t("colorSettings")}</h4>
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={field.field_config?.allowCustom !== false}
              onChange={(e) =>
                update({ field_config: { ...field.field_config, allowCustom: e.target.checked } })
              }
              className="h-4 w-4 rounded border-gray-300 text-blue-600"
            />
            <span className="text-sm text-gray-700">{t("allowCustomHex")}</span>
          </label>
          <div>
            <label className="mb-1 block text-sm font-medium text-gray-700">{t("colorPresets")}</label>
            <Input
              value={((field.field_config?.presets as string[]) || []).join(", ")}
              placeholder={t("colorPresetsExample")}
              onChange={(e) =>
                update({
                  field_config: {
                    ...field.field_config,
                    presets: e.target.value
                      ? e.target.value.split(",").map((c) => c.trim()).filter(Boolean)
                      : undefined,
                  },
                })
              }
            />
            <p className="mt-1 text-xs text-gray-400">{t("colorPresetsHint")}</p>
          </div>
        </>
      )}

      {field.field_type === "currency" && (
        <>
          <hr className="border-gray-200" />
          <h4 className="text-sm font-semibold text-gray-900">{t("currencySettings")}</h4>
          <Select
            label={tCommon("currency")}
            value={(field.field_config?.currency as string) || "TWD"}
            onChange={(e) =>
              update({ field_config: { ...field.field_config, currency: e.target.value } })
            }
            options={[
              { value: "TWD", label: t("currencyTWD") },
              { value: "USD", label: t("currencyUSD") },
              { value: "EUR", label: t("currencyEUR") },
              { value: "JPY", label: t("currencyJPY") },
              { value: "GBP", label: t("currencyGBP") },
              { value: "CNY", label: t("currencyCNY") },
            ]}
          />
          <Input
            label={t("decimalPlaces")}
            type="number"
            value={(field.field_config?.decimals as number) ?? 2}
            onChange={(e) =>
              update({
                field_config: {
                  ...field.field_config,
                  decimals: e.target.value ? Number(e.target.value) : 2,
                },
              })
            }
          />
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={field.field_config?.showSymbol !== false}
              onChange={(e) =>
                update({ field_config: { ...field.field_config, showSymbol: e.target.checked } })
              }
              className="h-4 w-4 rounded border-gray-300 text-blue-600"
            />
            <span className="text-sm text-gray-700">{t("showCurrencySymbol")}</span>
          </label>
        </>
      )}

      {field.field_type === "radio_group" && (
        <>
          <hr className="border-gray-200" />
          <h4 className="text-sm font-semibold text-gray-900">{t("radioGroupSettings")}</h4>
          <Select
            label={t("layout")}
            value={(field.field_config?.layout as string) || "vertical"}
            onChange={(e) =>
              update({ field_config: { ...field.field_config, layout: e.target.value } })
            }
            options={[
              { value: "horizontal", label: t("horizontal") },
              { value: "vertical", label: t("vertical") },
              { value: "button", label: t("segmentedButton") },
            ]}
          />
          <hr className="border-gray-200" />
          <h4 className="text-sm font-semibold text-gray-900">{t("dataSource")}</h4>
          <Select
            label={t("sourceType")}
            value={field.data_source?.type || "static"}
            onChange={(e) => {
              const type = e.target.value as "static" | "sql";
              update({
                data_source: {
                  type,
                  static_options: type === "static" ? field.data_source?.static_options || [] : undefined,
                  sql_query: type === "sql" ? field.data_source?.sql_query : undefined,
                  value_column: type === "sql" ? field.data_source?.value_column : undefined,
                  label_column: type === "sql" ? field.data_source?.label_column : undefined,
                },
              });
            }}
            options={[
              { value: "static", label: t("staticOptions") },
              { value: "sql", label: t("sqlQuery") },
            ]}
          />
          {field.data_source?.type === "sql" && (
            <>
              <div className="w-full">
                <label className="mb-1 block text-sm font-medium text-gray-700">{t("sqlQuery")}</label>
                <textarea
                  value={field.data_source.sql_query || ""}
                  onChange={(e) =>
                    update({ data_source: { ...field.data_source!, sql_query: e.target.value } })
                  }
                  rows={3}
                  className="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                />
              </div>
              <Input
                label={t("valueColumn")}
                value={field.data_source.value_column || ""}
                onChange={(e) =>
                  update({ data_source: { ...field.data_source!, value_column: e.target.value } })
                }
              />
              <Input
                label={t("labelColumn")}
                value={field.data_source.label_column || ""}
                onChange={(e) =>
                  update({ data_source: { ...field.data_source!, label_column: e.target.value } })
                }
              />
            </>
          )}
          {(!field.data_source || field.data_source.type === "static") && (
            <StaticOptionsEditor
              options={field.data_source?.static_options || []}
              onChange={(options) =>
                update({ data_source: { type: "static", static_options: options } })
              }
            />
          )}
        </>
      )}

      {field.field_type === "time_picker" && (
        <>
          <hr className="border-gray-200" />
          <h4 className="text-sm font-semibold text-gray-900">{t("timePickerSettings")}</h4>
          <Select
            label={t("timeFormat")}
            value={(field.field_config?.format as string) || "24h"}
            onChange={(e) =>
              update({ field_config: { ...field.field_config, format: e.target.value } })
            }
            options={[
              { value: "24h", label: t("timeFormat24h") },
              { value: "12h", label: t("timeFormat12h") },
            ]}
          />
          <Input
            label={t("minuteStep")}
            type="number"
            value={(field.field_config?.minuteStep as number) ?? 15}
            onChange={(e) =>
              update({
                field_config: {
                  ...field.field_config,
                  minuteStep: e.target.value ? Number(e.target.value) : 15,
                },
              })
            }
          />
        </>
      )}

      {field.field_type === "rich_text" && (
        <>
          <hr className="border-gray-200" />
          <h4 className="text-sm font-semibold text-gray-900">{t("richTextSettings")}</h4>
          <div>
            <label className="mb-1 block text-sm font-medium text-gray-700">{t("toolbarItems")}</label>
            <div className="space-y-1">
              {["bold", "italic", "strike", "link", "bulletList", "orderedList", "heading", "blockquote", "code"].map((item) => {
                const toolbar = ((field.field_config?.toolbar as string[]) || ["bold", "italic", "link", "bulletList", "orderedList"]);
                return (
                  <label key={item} className="flex items-center gap-2">
                    <input
                      type="checkbox"
                      checked={toolbar.includes(item)}
                      onChange={(e) => {
                        const newToolbar = e.target.checked
                          ? [...toolbar, item]
                          : toolbar.filter((t) => t !== item);
                        update({ field_config: { ...field.field_config, toolbar: newToolbar } });
                      }}
                      className="h-4 w-4 rounded border-gray-300 text-blue-600"
                    />
                    <span className="text-sm text-gray-700">{item}</span>
                  </label>
                );
              })}
            </div>
          </div>
          <Input
            label={t("maxLength")}
            type="number"
            value={(field.field_config?.maxLength as number) ?? ""}
            onChange={(e) =>
              update({
                field_config: {
                  ...field.field_config,
                  maxLength: e.target.value ? Number(e.target.value) : undefined,
                },
              })
            }
          />
        </>
      )}

      {field.field_type === "tree_table" && (
        <>
          <hr className="border-gray-200" />
          <h4 className="text-sm font-semibold text-gray-900">{t("treeTableSettings")}</h4>
          <Input
            label={t("parentField")}
            value={(field.field_config?.parent_field as string) || ""}
            placeholder={t("parentFieldExample")}
            onChange={(e) =>
              update({ field_config: { ...field.field_config, parent_field: e.target.value || undefined } })
            }
          />
          <Input
            label={t("idField")}
            value={(field.field_config?.id_field as string) || ""}
            placeholder={t("idFieldExample")}
            onChange={(e) =>
              update({ field_config: { ...field.field_config, id_field: e.target.value || undefined } })
            }
          />
          <Input
            label={t("expandLevel")}
            type="number"
            value={(field.field_config?.expand_level as number) ?? 1}
            onChange={(e) =>
              update({
                field_config: {
                  ...field.field_config,
                  expand_level: e.target.value ? Number(e.target.value) : 1,
                },
              })
            }
          />
        </>
      )}

      {field.field_type === "master_detail" && (
        <MasterDetailConfigPanel field={field} onUpdate={update} />
      )}

      {field.field_type === "approval_buttons" && (
        <ApprovalConfigPanel field={field} onUpdate={update} />
      )}

      {(field.field_type === "dropdown" || field.field_type === "multi_select") && (
        <>
          <hr className="border-gray-200" />
          <h4 className="text-sm font-semibold text-gray-900">{t("dataSource")}</h4>
          <Select
            label={t("sourceType")}
            value={field.data_source?.type || "static"}
            onChange={(e) => {
              const type = e.target.value as "static" | "sql";
              update({
                data_source: {
                  type,
                  static_options: type === "static" ? field.data_source?.static_options || [] : undefined,
                  sql_query: type === "sql" ? field.data_source?.sql_query : undefined,
                  value_column: type === "sql" ? field.data_source?.value_column : undefined,
                  label_column: type === "sql" ? field.data_source?.label_column : undefined,
                },
              });
            }}
            options={[
              { value: "static", label: t("staticOptions") },
              { value: "sql", label: t("sqlQuery") },
            ]}
          />

          {field.data_source?.type === "sql" && (
            <>
              <div className="w-full">
                <label className="mb-1 block text-sm font-medium text-gray-700">{t("sqlQuery")}</label>
                <textarea
                  value={field.data_source.sql_query || ""}
                  onChange={(e) =>
                    update({ data_source: { ...field.data_source!, sql_query: e.target.value } })
                  }
                  rows={3}
                  className="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                />
              </div>
              <Input
                label={t("valueColumn")}
                value={field.data_source.value_column || ""}
                onChange={(e) =>
                  update({ data_source: { ...field.data_source!, value_column: e.target.value } })
                }
              />
              <Input
                label={t("labelColumn")}
                value={field.data_source.label_column || ""}
                onChange={(e) =>
                  update({ data_source: { ...field.data_source!, label_column: e.target.value } })
                }
              />
            </>
          )}

          {(!field.data_source || field.data_source.type === "static") && (
            <StaticOptionsEditor
              options={field.data_source?.static_options || []}
              onChange={(options) =>
                update({ data_source: { type: "static", static_options: options } })
              }
            />
          )}
        </>
      )}

      {/* ── Visibility Rules ── */}
      <hr className="border-gray-200" />
      <details className="group" open={!!field.visibility_rule}>
        <summary className="flex cursor-pointer items-center justify-between text-sm font-semibold text-gray-900">
          <span className="flex items-center gap-2">
            {t("visibilityRules")}
            {field.visibility_rule && (field.visibility_rule as any).dependent_field && (
              <span className="inline-flex h-5 items-center rounded-full bg-indigo-100 px-2 text-xs font-medium text-indigo-700">
                {t("active")}
              </span>
            )}
          </span>
          <span className="text-gray-400 group-open:rotate-180 transition-transform">▼</span>
        </summary>
        <div className="mt-3 space-y-3">
          <VisibilityRulesEditor field={field} onUpdate={update} />
        </div>
      </details>

      {field.field_type === "lookup" && (
        <>
          <hr className="border-gray-200" />
          <h4 className="text-sm font-semibold text-gray-900">{t("lookupConfig")}</h4>
          <Input
            label={t("lookupOperationCode")}
            value={(field.field_config?.operation_code as string) || ""}
            placeholder={t("lookupOperationCodePlaceholder")}
            onChange={(e) => update({ field_config: { ...field.field_config, operation_code: e.target.value || undefined } })}
          />
          <Input
            label={t("valueColumn")}
            value={(field.field_config?.value_column as string) || ""}
            onChange={(e) => update({ field_config: { ...field.field_config, value_column: e.target.value || undefined } })}
          />
          <Input
            label={t("labelColumn")}
            value={(field.field_config?.label_column as string) || ""}
            onChange={(e) => update({ field_config: { ...field.field_config, label_column: e.target.value || undefined } })}
          />
          <Input
            label={t("displayColumns")}
            value={((field.field_config?.display_columns as string[]) || []).join(", ")}
            placeholder={t("displayColumnsPlaceholder")}
            onChange={(e) => update({
              field_config: {
                ...field.field_config,
                display_columns: e.target.value ? e.target.value.split(",").map(c => c.trim()).filter(Boolean) : undefined,
              },
            })}
          />
        </>
      )}

      <hr className="border-gray-200" />
      <button
        type="button"
        onClick={() => setPermissionOpen(true)}
        className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm text-gray-700 hover:bg-gray-50"
      >
        {t("fieldPermissions")}
      </button>
      {permissionOpen && (
        <FieldPermissionPanel
          fieldId={field.id}
          onClose={() => setPermissionOpen(false)}
        />
      )}
    </div>
  );
}

function VisibilityRulesEditor({ field, onUpdate }: { field: FieldDefinition; onUpdate: (updates: Partial<FieldDefinition>) => void }) {
  const t = useTranslations("lowcode");
  const { sections } = useBuilderStore();

  // Get all fields except current one (for dependent field selection)
  const otherFields = sections.flatMap(s => s.fields).filter(f => f.id !== field.id);

  const rule = (field.visibility_rule as { dependent_field?: string; operator?: string; value?: string; action?: string }) || {};

  const updateRule = (updates: Record<string, string>) => {
    onUpdate({ visibility_rule: { ...rule, ...updates } as any });
  };

  return (
    <div className="space-y-3">
      <Select
        label={t("visibilityAction")}
        value={rule.action || "show"}
        onChange={(e) => updateRule({ action: e.target.value })}
        options={[
          { value: "show", label: t("showWhen") },
          { value: "hide", label: t("hideWhen") },
        ]}
      />
      <Select
        label={t("dependentField")}
        value={rule.dependent_field || ""}
        onChange={(e) => updateRule({ dependent_field: e.target.value })}
        options={[
          { value: "", label: t("selectField") },
          ...otherFields.map(f => ({ value: f.field_key, label: f.label })),
        ]}
      />
      <Select
        label={t("operator")}
        value={rule.operator || "equals"}
        onChange={(e) => updateRule({ operator: e.target.value })}
        options={[
          { value: "equals", label: t("opEquals") },
          { value: "not_equals", label: t("opNotEquals") },
          { value: "contains", label: t("opContains") },
          { value: "gt", label: t("opGreaterThan") },
          { value: "lt", label: t("opLessThan") },
        ]}
      />
      <Input
        label={t("conditionValue")}
        value={String(rule.value ?? "")}
        onChange={(e) => updateRule({ value: e.target.value })}
      />
      {rule.dependent_field && (
        <>
          {/* Rule preview */}
          <div className="rounded-md bg-indigo-50 p-2">
            <p className="text-xs text-indigo-700">
              <span className="font-medium">{t("logicPreview")}:</span>{" "}
              {rule.action === "show" ? t("showWhen") : t("hideWhen")}{" "}
              <span className="font-mono font-semibold">{otherFields.find(f => f.field_key === rule.dependent_field)?.label || rule.dependent_field}</span>{" "}
              {rule.operator === "equals" ? "=" : rule.operator === "not_equals" ? "≠" : rule.operator === "contains" ? "∋" : rule.operator === "gt" ? ">" : "<"}{" "}
              &quot;{rule.value}&quot;
            </p>
          </div>
          <div className="flex items-center justify-between">
            <button
              type="button"
              className="text-xs text-red-500 hover:text-red-700"
              onClick={() => onUpdate({ visibility_rule: null as any })}
            >
              {t("clearRule")}
            </button>
            <span className="text-xs text-gray-400">{t("logicDesignerHint2")}</span>
          </div>
        </>
      )}
    </div>
  );
}

function StaticOptionsEditor({
  options,
  onChange,
}: {
  options: { label: string; value: string }[];
  onChange: (options: { label: string; value: string }[]) => void;
}) {
  const t = useTranslations("lowcode");

  const addOption = () => {
    onChange([...options, { label: "", value: "" }]);
  };

  const updateOption = (index: number, key: "label" | "value", val: string) => {
    const newOptions = [...options];
    newOptions[index] = { ...newOptions[index], [key]: val };
    onChange(newOptions);
  };

  const removeOption = (index: number) => {
    onChange(options.filter((_, i) => i !== index));
  };

  return (
    <div>
      <label className="mb-1 block text-sm font-medium text-gray-700">{t("staticOptions")}</label>
      <div className="space-y-2">
        {options.map((opt, i) => (
          <div key={i} className="flex gap-2">
            <input
              value={opt.label}
              onChange={(e) => updateOption(i, "label", e.target.value)}
              placeholder={t("label")}
              className="flex-1 rounded-md border border-gray-300 px-2 py-1 text-sm"
            />
            <input
              value={opt.value}
              onChange={(e) => updateOption(i, "value", e.target.value)}
              placeholder={t("valueColumn")}
              className="flex-1 rounded-md border border-gray-300 px-2 py-1 text-sm"
            />
            <button
              type="button"
              onClick={() => removeOption(i)}
              className="text-red-500 hover:text-red-700"
            >
              x
            </button>
          </div>
        ))}
      </div>
      <button
        type="button"
        onClick={addOption}
        className="mt-2 text-sm text-blue-600 hover:text-blue-700"
      >
        + {t("addField")}
      </button>
    </div>
  );
}

export function PropertyPanel() {
  const { sections, selectedFieldId, selectedSectionId } = useBuilderStore();
  const t = useTranslations("lowcode");

  const selectedSection = selectedSectionId
    ? sections.find((s) => s.id === selectedSectionId)
    : null;

  let selectedField: FieldDefinition | null = null;
  if (selectedFieldId) {
    for (const section of sections) {
      const found = section.fields.find((f) => f.id === selectedFieldId);
      if (found) {
        selectedField = found;
        break;
      }
    }
  }

  if (!selectedField && !selectedSection) {
    return (
      <div className="flex h-full w-72 flex-col border-l border-gray-200 bg-gray-50">
        <div className="border-b border-gray-200 px-4 py-3">
          <h3 className="text-sm font-semibold text-gray-900">{t("properties")}</h3>
        </div>
        <div className="flex flex-1 items-center justify-center px-4">
          <p className="text-center text-sm text-gray-500">
            {t("selectFieldOrSection")}
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex h-full w-72 flex-col border-l border-gray-200 bg-gray-50">
      <div className="border-b border-gray-200 px-4 py-3">
        <h3 className="text-sm font-semibold text-gray-900">{t("properties")}</h3>
      </div>
      <div className="flex-1 overflow-y-auto px-4 py-4">
        {selectedField && <FieldPropertyEditor field={selectedField} />}
        {selectedSection && !selectedField && <SectionEditor section={selectedSection} />}
      </div>
    </div>
  );
}
