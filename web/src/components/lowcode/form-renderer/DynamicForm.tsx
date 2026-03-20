"use client";

import { useEffect, useRef, useCallback } from "react";
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
import { useFormCalculations } from "@/lib/hooks/use-form-calculations";
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
  const { formDefinition, formulas, isLoading, createRecord, updateRecord, isSubmitting } =
    useDynamicForm(operationCode);
  const { calculate } = useFormCalculations(formulas);

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

  // Track the previous form values to detect which field changed
  const prevValuesRef = useRef<Record<string, unknown>>({});

  const applyCalculations = useCallback(
    (currentValues: Record<string, unknown>) => {
      if (formulas.length === 0) return;
      const prev = prevValuesRef.current;
      // Find which fields changed
      const changedFields: string[] = [];
      for (const key of Object.keys(currentValues)) {
        if (currentValues[key] !== prev[key]) {
          changedFields.push(key);
        }
      }
      prevValuesRef.current = { ...currentValues };
      if (changedFields.length === 0) return;

      // Accumulate all calculation updates
      const allUpdates: Record<string, unknown> = {};
      for (const fieldName of changedFields) {
        const updates = calculate(fieldName, currentValues);
        Object.assign(allUpdates, updates);
      }
      // Apply updates via setValue (does not trigger re-subscription for these fields if value is same)
      for (const [key, value] of Object.entries(allUpdates)) {
        const current = currentValues[key];
        if (current !== value) {
          methods.setValue(key, value, { shouldDirty: true, shouldValidate: true });
        }
      }
    },
    [formulas, calculate, methods]
  );

  // Subscribe to form value changes for real-time calculation
  useEffect(() => {
    if (formulas.length === 0) return;
    // Initialize prevValues with current form values
    prevValuesRef.current = { ...methods.getValues() };
    const subscription = methods.watch((values) => {
      applyCalculations(values as Record<string, unknown>);
    });
    return () => subscription.unsubscribe();
  }, [formulas, methods, applyCalculations]);

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
