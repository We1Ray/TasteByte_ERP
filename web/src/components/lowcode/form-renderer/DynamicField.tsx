"use client";

import { useFormContext, Controller } from "react-hook-form";
import type { FieldDefinition } from "@/lib/types/lowcode";
import { TextField } from "./widgets/TextField";
import { NumberField } from "./widgets/NumberField";
import { DropdownField } from "./widgets/DropdownField";
import { MultiSelectField } from "./widgets/MultiSelectField";
import { TextAreaField } from "./widgets/TextAreaField";
import { CheckboxField } from "./widgets/CheckboxField";
import { FileUploadField } from "./widgets/FileUploadField";
import { LookupWindowField } from "./widgets/LookupWindowField";
import { CompositeField } from "./widgets/CompositeField";
import { DateField } from "./widgets/DateField";
import { DateTimeField } from "./widgets/DateTimeField";
import { ToggleField } from "./widgets/ToggleField";
import { ColorPickerField } from "./widgets/ColorPickerField";
import { CurrencyField } from "./widgets/CurrencyField";
import { RadioGroupField } from "./widgets/RadioGroupField";
import { TimePickerField } from "./widgets/TimePickerField";
import { TreeTableField } from "./widgets/TreeTableField";
import { DocumentFlowField } from "./widgets/DocumentFlowField";
import { ApprovalButtonsField } from "./widgets/ApprovalButtonsField";
import { MasterDetailField } from "./widgets/MasterDetailField";
import { RichTextField } from "./widgets/RichTextField";

interface DynamicFieldProps {
  field: FieldDefinition;
  disabled?: boolean;
}

export function DynamicField({ field, disabled }: DynamicFieldProps) {
  const { register, control, formState: { errors }, watch } = useFormContext();
  const error = errors[field.field_key]?.message as string | undefined;

  // Visibility rules
  if (field.visibility_rules?.length) {
    const allVisible = field.visibility_rules.every((rule) => {
      const depValue = watch(rule.field_id);
      switch (rule.operator) {
        case "equals": return String(depValue) === rule.value;
        case "not_equals": return String(depValue) !== rule.value;
        case "contains": return String(depValue).includes(rule.value);
        case "gt": return Number(depValue) > Number(rule.value);
        case "lt": return Number(depValue) < Number(rule.value);
        default: return true;
      }
    });
    if (!allVisible) return null;
  }

  const reg = register(field.field_key);

  switch (field.field_type) {
    case "text":
      return <TextField field={field} register={reg} error={error} disabled={disabled} />;
    case "number":
      return <NumberField field={field} register={reg} error={error} disabled={disabled} />;
    case "dropdown":
      return <DropdownField field={field} register={reg} error={error} disabled={disabled} />;
    case "textarea":
      return <TextAreaField field={field} register={reg} error={error} disabled={disabled} />;
    case "checkbox":
      return <CheckboxField field={field} register={reg} error={error} disabled={disabled} />;
    case "date":
      return <DateField field={field} register={reg} error={error} disabled={disabled} />;
    case "datetime":
      return <DateTimeField field={field} register={reg} error={error} disabled={disabled} />;
    case "multi_select":
      return (
        <Controller
          name={field.field_key}
          control={control}
          render={({ field: formField }) => (
            <MultiSelectField
              field={field}
              value={formField.value || []}
              onChange={formField.onChange}
              error={error}
              disabled={disabled}
            />
          )}
        />
      );
    case "file":
      return (
        <Controller
          name={field.field_key}
          control={control}
          render={({ field: formField }) => (
            <FileUploadField
              field={field}
              value={formField.value || ""}
              onChange={formField.onChange}
              error={error}
              disabled={disabled}
            />
          )}
        />
      );
    case "lookup":
      return (
        <Controller
          name={field.field_key}
          control={control}
          render={({ field: formField }) => (
            <LookupWindowField
              field={field}
              value={formField.value || ""}
              onChange={formField.onChange}
              error={error}
              disabled={disabled}
            />
          )}
        />
      );
    case "composite":
      return (
        <Controller
          name={field.field_key}
          control={control}
          render={({ field: formField }) => (
            <CompositeField
              field={field}
              value={formField.value}
              onChange={formField.onChange}
              error={error}
              disabled={disabled}
            />
          )}
        />
      );
    case "toggle":
      return <ToggleField field={field} register={reg} error={error} disabled={disabled} />;
    case "color":
      return (
        <Controller
          name={field.field_key}
          control={control}
          render={({ field: formField }) => (
            <ColorPickerField
              field={field}
              value={formField.value || ""}
              onChange={formField.onChange}
              error={error}
              disabled={disabled}
            />
          )}
        />
      );
    case "currency":
      return (
        <Controller
          name={field.field_key}
          control={control}
          render={({ field: formField }) => (
            <CurrencyField
              field={field}
              value={formField.value ?? ""}
              onChange={formField.onChange}
              error={error}
              disabled={disabled}
            />
          )}
        />
      );
    case "radio_group":
      return <RadioGroupField field={field} register={reg} error={error} disabled={disabled} />;
    case "time_picker":
      return (
        <Controller
          name={field.field_key}
          control={control}
          render={({ field: formField }) => (
            <TimePickerField
              field={field}
              value={formField.value || ""}
              onChange={formField.onChange}
              error={error}
              disabled={disabled}
            />
          )}
        />
      );
    case "tree_table":
      return (
        <Controller
          name={field.field_key}
          control={control}
          render={({ field: formField }) => (
            <TreeTableField
              field={field}
              value={formField.value || []}
              onChange={formField.onChange}
              error={error}
              disabled={disabled}
            />
          )}
        />
      );
    case "document_flow":
      return (
        <Controller
          name={field.field_key}
          control={control}
          render={({ field: formField }) => (
            <DocumentFlowField
              field={field}
              value={formField.value}
              onChange={formField.onChange}
              error={error}
              disabled={disabled}
            />
          )}
        />
      );
    case "approval_buttons":
      return (
        <Controller
          name={field.field_key}
          control={control}
          render={({ field: formField }) => (
            <ApprovalButtonsField
              field={field}
              value={formField.value}
              onChange={formField.onChange}
              error={error}
              disabled={disabled}
            />
          )}
        />
      );
    case "master_detail":
      return (
        <Controller
          name={field.field_key}
          control={control}
          render={({ field: formField }) => (
            <MasterDetailField
              field={field}
              value={formField.value || { items: [] }}
              onChange={formField.onChange}
              error={error}
              disabled={disabled}
            />
          )}
        />
      );
    case "rich_text":
      return (
        <Controller
          name={field.field_key}
          control={control}
          render={({ field: formField }) => (
            <RichTextField
              field={field}
              value={formField.value || ""}
              onChange={formField.onChange}
              error={error}
              disabled={disabled}
            />
          )}
        />
      );
    default:
      return <TextField field={field} register={reg} error={error} disabled={disabled} />;
  }
}
