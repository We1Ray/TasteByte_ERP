"use client";

import { Plus, Trash2, GripVertical } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { Card } from "@/components/ui/card";

export interface ButtonItem {
  button_key: string;
  label: string;
  icon?: string;
  variant: string;
  action_type: string;
  action_config: Record<string, unknown>;
  confirm_message?: string;
  required_permission?: string;
  is_visible: boolean;
  sort_order: number;
}

interface Props {
  buttons: ButtonItem[];
  onChange: (buttons: ButtonItem[]) => void;
}

const VARIANT_OPTIONS = [
  { value: "primary", label: "Primary" },
  { value: "secondary", label: "Secondary" },
  { value: "danger", label: "Danger" },
  { value: "ghost", label: "Ghost" },
];

const ACTION_TYPE_OPTIONS = [
  { value: "NAVIGATE", label: "Navigate" },
  { value: "API_CALL", label: "API Call" },
  { value: "MODAL", label: "Modal" },
  { value: "CUSTOM_JS", label: "Custom JS" },
];

export function ButtonDesigner({ buttons, onChange }: Props) {
  const addButton = () => {
    const newBtn: ButtonItem = {
      button_key: `btn_${Date.now()}`,
      label: "New Button",
      variant: "secondary",
      action_type: "API_CALL",
      action_config: {},
      is_visible: true,
      sort_order: buttons.length,
    };
    onChange([...buttons, newBtn]);
  };

  const updateButton = (index: number, updates: Partial<ButtonItem>) => {
    const updated = buttons.map((btn, i) =>
      i === index ? { ...btn, ...updates } : btn
    );
    onChange(updated);
  };

  const removeButton = (index: number) => {
    onChange(buttons.filter((_, i) => i !== index));
  };

  return (
    <div className="space-y-3">
      {buttons.map((btn, idx) => (
        <Card key={btn.button_key} className="relative">
          <div className="flex items-start gap-3 p-4">
            <GripVertical className="mt-2 h-4 w-4 shrink-0 cursor-grab text-gray-400" />
            <div className="flex-1 space-y-3">
              <div className="grid grid-cols-2 gap-3">
                <Input
                  label="Button Key"
                  value={btn.button_key}
                  onChange={(e) =>
                    updateButton(idx, { button_key: e.target.value })
                  }
                  placeholder="e.g., approve"
                />
                <Input
                  label="Label"
                  value={btn.label}
                  onChange={(e) =>
                    updateButton(idx, { label: e.target.value })
                  }
                  placeholder="e.g., Approve"
                />
              </div>
              <div className="grid grid-cols-2 gap-3">
                <Select
                  label="Variant"
                  value={btn.variant}
                  onChange={(e) =>
                    updateButton(idx, { variant: e.target.value })
                  }
                  options={VARIANT_OPTIONS}
                />
                <Select
                  label="Action Type"
                  value={btn.action_type}
                  onChange={(e) =>
                    updateButton(idx, { action_type: e.target.value })
                  }
                  options={ACTION_TYPE_OPTIONS}
                />
              </div>
              {(btn.action_type === "NAVIGATE" || btn.action_type === "API_CALL") && (
                <Input
                  label={btn.action_type === "NAVIGATE" ? "URL" : "API URL"}
                  value={(btn.action_config.url as string) || ""}
                  onChange={(e) =>
                    updateButton(idx, {
                      action_config: { ...btn.action_config, url: e.target.value },
                    })
                  }
                  placeholder={btn.action_type === "NAVIGATE" ? "/path/to/page" : "/api/v1/..."}
                />
              )}
              {btn.action_type === "API_CALL" && (
                <Select
                  label="HTTP Method"
                  value={(btn.action_config.method as string) || "POST"}
                  onChange={(e) =>
                    updateButton(idx, {
                      action_config: { ...btn.action_config, method: e.target.value },
                    })
                  }
                  options={[
                    { value: "GET", label: "GET" },
                    { value: "POST", label: "POST" },
                    { value: "PUT", label: "PUT" },
                    { value: "DELETE", label: "DELETE" },
                  ]}
                />
              )}
              <Input
                label="Confirm Message (optional)"
                value={btn.confirm_message || ""}
                onChange={(e) =>
                  updateButton(idx, {
                    confirm_message: e.target.value || undefined,
                  })
                }
                placeholder="Are you sure you want to..."
              />
            </div>
            <button
              onClick={() => removeButton(idx)}
              className="mt-2 rounded p-1 text-red-400 hover:bg-red-50 hover:text-red-600"
            >
              <Trash2 className="h-4 w-4" />
            </button>
          </div>
        </Card>
      ))}

      <Button variant="secondary" onClick={addButton} className="w-full">
        <Plus className="h-4 w-4" />
        Add Button
      </Button>
    </div>
  );
}
