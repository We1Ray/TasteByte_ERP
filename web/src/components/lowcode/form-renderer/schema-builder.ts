import { z } from "zod";
import type { FieldDefinition, FormSection } from "@/lib/types/lowcode";

function buildFieldSchema(field: FieldDefinition): z.ZodTypeAny {
  const v = field.validation;

  switch (field.field_type) {
    case "text":
    case "textarea": {
      let schema = z.string();
      if (v.min_length) schema = schema.min(v.min_length, `Minimum ${v.min_length} characters`);
      if (v.max_length) schema = schema.max(v.max_length, `Maximum ${v.max_length} characters`);
      if (v.regex_pattern) schema = schema.regex(new RegExp(v.regex_pattern), v.regex_message || "Invalid format");
      if (!v.required) return schema.optional().or(z.literal(""));
      return schema.min(1, "This field is required");
    }

    case "number": {
      let schema = z.coerce.number();
      if (v.min_value !== undefined) schema = schema.min(v.min_value, `Minimum value is ${v.min_value}`);
      if (v.max_value !== undefined) schema = schema.max(v.max_value, `Maximum value is ${v.max_value}`);
      if (!v.required) return schema.optional().or(z.literal("").transform(() => undefined));
      return schema;
    }

    case "dropdown":
    case "lookup": {
      const schema = z.string();
      if (!v.required) return schema.optional().or(z.literal(""));
      return schema.min(1, "Please select an option");
    }

    case "multi_select": {
      const schema = z.array(z.string());
      if (!v.required) return schema.optional();
      return schema.min(1, "Please select at least one option");
    }

    case "checkbox": {
      return z.boolean().optional();
    }

    case "date":
    case "datetime": {
      const schema = z.string();
      if (!v.required) return schema.optional().or(z.literal(""));
      return schema.min(1, "This field is required");
    }

    case "file": {
      if (!v.required) return z.any().optional();
      return z.any().refine((val) => val !== undefined && val !== null && val !== "", "File is required");
    }

    case "composite": {
      return z.any().optional();
    }

    case "toggle": {
      return z.boolean().optional();
    }

    case "color": {
      const colorSchema = z.string().regex(/^#[0-9a-fA-F]{6}$/, "Invalid color format");
      if (!v.required) return colorSchema.optional().or(z.literal(""));
      return colorSchema;
    }

    case "currency": {
      let currencySchema = z.coerce.number();
      if (v.min_value !== undefined) currencySchema = currencySchema.min(v.min_value, `Minimum value is ${v.min_value}`);
      if (v.max_value !== undefined) currencySchema = currencySchema.max(v.max_value, `Maximum value is ${v.max_value}`);
      if (!v.required) return currencySchema.optional().or(z.literal("").transform(() => undefined));
      return currencySchema;
    }

    case "radio_group": {
      const radioSchema = z.string();
      if (!v.required) return radioSchema.optional().or(z.literal(""));
      return radioSchema.min(1, "Please select an option");
    }

    case "time_picker": {
      const timeSchema = z.string().regex(/^\d{2}:\d{2}$/, "Invalid time format");
      if (!v.required) return timeSchema.optional().or(z.literal(""));
      return timeSchema;
    }

    case "tree_table":
    case "document_flow":
    case "approval_buttons":
      return z.any().optional();

    case "master_detail":
      return z.object({ items: z.array(z.record(z.string(), z.any())) }).optional();

    case "rich_text": {
      const richSchema = z.string();
      if (!v.required) return richSchema.optional().or(z.literal(""));
      return richSchema.min(1, "This field is required");
    }

    default:
      return z.any().optional();
  }
}

export function buildFormSchema(sections: FormSection[]): z.ZodObject<Record<string, z.ZodTypeAny>> {
  const shape: Record<string, z.ZodTypeAny> = {};

  for (const section of sections) {
    for (const field of section.fields) {
      shape[field.field_key] = buildFieldSchema(field);
    }
  }

  return z.object(shape);
}

export function getDefaultValues(sections: FormSection[]): Record<string, unknown> {
  const defaults: Record<string, unknown> = {};

  for (const section of sections) {
    for (const field of section.fields) {
      if (field.default_value !== undefined && field.default_value !== "") {
        if (field.field_type === "number" || field.field_type === "currency") {
          defaults[field.field_key] = Number(field.default_value);
        } else if (field.field_type === "checkbox" || field.field_type === "toggle") {
          defaults[field.field_key] = field.default_value === "true";
        } else if (field.field_type === "multi_select") {
          try {
            defaults[field.field_key] = JSON.parse(field.default_value);
          } catch {
            defaults[field.field_key] = [];
          }
        } else {
          defaults[field.field_key] = field.default_value;
        }
      } else {
        if (field.field_type === "checkbox" || field.field_type === "toggle") defaults[field.field_key] = false;
        else if (field.field_type === "multi_select") defaults[field.field_key] = [];
        else if (field.field_type === "number" || field.field_type === "currency") defaults[field.field_key] = "";
        else if (field.field_type === "master_detail") defaults[field.field_key] = { items: [] };
        else if (field.field_type === "tree_table") defaults[field.field_key] = [];
        else defaults[field.field_key] = "";
      }
    }
  }

  return defaults;
}
