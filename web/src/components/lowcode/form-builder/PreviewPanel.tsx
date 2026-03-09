"use client";

import { useTranslations } from "next-intl";
import { useBuilderStore } from "@/lib/stores/builder-store";
import { DynamicForm } from "../form-renderer/DynamicForm";

interface PreviewPanelProps {
  operationCode: string;
}

export function PreviewPanel({ operationCode }: PreviewPanelProps) {
  const { sections } = useBuilderStore();
  const t = useTranslations("lowcode");

  return (
    <div className="p-6">
      <h3 className="mb-4 text-lg font-semibold text-gray-900">{t("formPreview")}</h3>
      <DynamicForm operationCode={operationCode} preview sections={sections} />
    </div>
  );
}
