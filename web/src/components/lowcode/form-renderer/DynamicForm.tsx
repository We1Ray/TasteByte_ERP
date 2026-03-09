"use client";

import { useForm, FormProvider } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { useTranslations } from "next-intl";
import { Button } from "@/components/ui/button";
import { PageLoading } from "@/components/ui/loading";
import { DynamicSection } from "./DynamicSection";
import { DynamicTabs } from "./DynamicTabs";
import { WizardForm } from "./WizardForm";
import { buildFormSchema, getDefaultValues } from "./schema-builder";
import { useDynamicForm } from "@/lib/hooks/use-dynamic-form";
import type { FormSection } from "@/lib/types/lowcode";

interface DynamicFormProps {
  operationCode: string;
  recordId?: string;
  initialData?: Record<string, unknown>;
  onSubmitSuccess?: () => void;
  onTestSubmit?: (data: Record<string, unknown>) => void;
  preview?: boolean;
  sections?: FormSection[];
}

export function DynamicForm({
  operationCode,
  recordId,
  initialData,
  onSubmitSuccess,
  onTestSubmit,
  preview,
  sections: previewSections,
}: DynamicFormProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const { formDefinition, isLoading, createRecord, updateRecord, isSubmitting } =
    useDynamicForm(operationCode);

  const sections = previewSections || formDefinition?.sections || [];
  const sortedSections = [...sections].sort((a, b) => a.sort_order - b.sort_order);

  // Extract layout settings
  const wizardConfig = formDefinition?.form_settings?.wizard;
  const tabGroups = formDefinition?.layout_config?.tabGroups;
  const hasWizard = wizardConfig && wizardConfig.steps && wizardConfig.steps.length > 0 && !preview;
  const hasTabs = tabGroups && tabGroups.length > 0;

  const schema = buildFormSchema(sections);
  const defaults = initialData || getDefaultValues(sections);

  const methods = useForm({
    resolver: zodResolver(schema),
    defaultValues: defaults,
    values: initialData,
  });

  const handleSubmit = methods.handleSubmit(async (data) => {
    if (onTestSubmit) {
      onTestSubmit(data);
      return;
    }
    if (preview) return;
    try {
      if (recordId) {
        await updateRecord({ id: recordId, data });
      } else {
        await createRecord(data);
      }
      onSubmitSuccess?.();
    } catch {
      // error handled by mutation
    }
  });

  if (isLoading && !previewSections) {
    return <PageLoading />;
  }

  if (sections.length === 0) {
    return (
      <div className="py-12 text-center text-sm text-gray-500">
        {t("noFormDefinition")}
      </div>
    );
  }

  // Wizard mode: render WizardForm instead of standard form
  if (hasWizard) {
    return (
      <WizardForm
        sections={sections}
        wizardConfig={wizardConfig}
        operationCode={operationCode}
        recordId={recordId}
        initialData={initialData}
        onSubmitSuccess={onSubmitSuccess}
      />
    );
  }

  return (
    <FormProvider {...methods}>
      <form onSubmit={handleSubmit} className="space-y-6">
        {hasTabs ? (
          <DynamicTabs
            tabGroups={tabGroups}
            sections={sortedSections}
            sectionRenderer={(section) => (
              <DynamicSection section={section} disabled={preview} />
            )}
          />
        ) : (
          sortedSections.map((section) => (
            <DynamicSection key={section.id} section={section} disabled={preview} />
          ))
        )}

        {!preview && (
          <div className="flex justify-end gap-3">
            <Button
              type="button"
              variant="secondary"
              onClick={() => methods.reset()}
            >
              {tCommon("reset")}
            </Button>
            <Button type="submit" loading={isSubmitting}>
              {recordId ? tCommon("update") : tCommon("create")}
            </Button>
          </div>
        )}
      </form>
    </FormProvider>
  );
}
