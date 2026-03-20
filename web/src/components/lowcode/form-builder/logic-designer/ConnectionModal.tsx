"use client";

import { useState } from "react";
import { useTranslations } from "next-intl";
import { ArrowRight } from "lucide-react";
import { Modal } from "@/components/ui/modal";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import type { FieldDefinition } from "@/lib/types/lowcode";

interface ConnectionModalProps {
  open: boolean;
  onClose: () => void;
  onSave: (rule: { operator: string; value: string; action: string }) => void;
  sourceField?: FieldDefinition;
  targetField?: FieldDefinition;
  existingRule?: { operator: string; value: string; action: string };
}

export function ConnectionModal({
  open,
  onClose,
  onSave,
  sourceField,
  targetField,
  existingRule,
}: ConnectionModalProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const [operator, setOperator] = useState(existingRule?.operator || "equals");
  const [value, setValue] = useState(existingRule?.value || "");
  const [action, setAction] = useState(existingRule?.action || "show");

  const operatorSymbol =
    operator === "equals"
      ? "="
      : operator === "not_equals"
        ? "\u2260"
        : operator === "contains"
          ? "contains"
          : operator === "gt"
            ? ">"
            : "<";

  return (
    <Modal
      open={open}
      onClose={onClose}
      title={t("configureLogic")}
      size="md"
      footer={
        <>
          <Button variant="secondary" onClick={onClose}>
            {tCommon("cancel")}
          </Button>
          <Button onClick={() => onSave({ operator, value, action })}>
            {tCommon("save")}
          </Button>
        </>
      }
    >
      <div className="space-y-4">
        {/* Visual connection summary */}
        <div className="flex items-center justify-center gap-3 rounded-lg bg-indigo-50 p-4">
          <div className="rounded-md bg-white px-3 py-2 text-sm font-medium text-gray-900 shadow-sm">
            {sourceField?.label || "Source"}
          </div>
          <ArrowRight className="h-5 w-5 text-indigo-500" />
          <div className="rounded-md bg-white px-3 py-2 text-sm font-medium text-gray-900 shadow-sm">
            {targetField?.label || "Target"}
          </div>
        </div>

        <p className="text-xs text-gray-500">{t("connectionDesc")}</p>

        <Select
          label={t("visibilityAction")}
          value={action}
          onChange={(e) => setAction(e.target.value)}
          options={[
            { value: "show", label: t("showWhen") },
            { value: "hide", label: t("hideWhen") },
          ]}
        />

        <Select
          label={t("operator")}
          value={operator}
          onChange={(e) => setOperator(e.target.value)}
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
          value={value}
          onChange={(e) => setValue(e.target.value)}
          placeholder={t("conditionValuePlaceholder")}
        />

        <div className="rounded-md bg-gray-50 p-3">
          <p className="text-xs text-gray-600">
            <span className="font-medium">{t("logicPreview")}:</span>{" "}
            {action === "show" ? t("showWhen") : t("hideWhen")}{" "}
            <span className="font-mono font-medium text-indigo-600">
              {sourceField?.label}
            </span>{" "}
            {operatorSymbol}{" "}
            <span className="font-mono font-medium text-indigo-600">
              &quot;{value}&quot;
            </span>
          </p>
        </div>
      </div>
    </Modal>
  );
}
