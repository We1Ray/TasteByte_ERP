"use client";

import { useState, useMemo, useCallback } from "react";
import { useForm, FormProvider } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { toast } from "sonner";
import { useTranslations } from "next-intl";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { DynamicSection } from "./DynamicSection";
import { WizardStepIndicator } from "./WizardStepIndicator";
import { buildFormSchema, getDefaultValues } from "./schema-builder";
import { useDynamicForm } from "@/lib/hooks/use-dynamic-form";
import type { FormSection, WizardConfig } from "@/lib/types/lowcode";

interface WizardFormProps {
  sections: FormSection[];
  wizardConfig: WizardConfig;
  operationCode: string;
  recordId?: string;
  initialData?: Record<string, unknown>;
  onSubmitSuccess?: () => void;
}

export function WizardForm({
  sections,
  wizardConfig,
  operationCode,
  recordId,
  initialData,
  onSubmitSuccess,
}: WizardFormProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const [currentStep, setCurrentStep] = useState(0);
  const [completedSteps, setCompletedSteps] = useState<number[]>([]);

  const { createRecord, updateRecord, isSubmitting } =
    useDynamicForm(operationCode);

  const steps = wizardConfig.steps;
  const isLastStep = currentStep === steps.length - 1;
  const isFirstStep = currentStep === 0;

  // Build a map of section ID -> section for quick lookup
  const sectionMap = useMemo(() => {
    const map = new Map<string, FormSection>();
    for (const s of sections) {
      map.set(s.id, s);
    }
    return map;
  }, [sections]);

  // Get sections for a specific step
  const getSectionsForStep = useCallback(
    (stepIndex: number): FormSection[] => {
      const step = steps[stepIndex];
      if (!step) return [];
      return step.sectionIds
        .map((id) => sectionMap.get(id))
        .filter((s): s is FormSection => s !== undefined)
        .sort((a, b) => a.sort_order - b.sort_order);
    },
    [steps, sectionMap]
  );

  // Current step sections
  const currentSections = useMemo(
    () => getSectionsForStep(currentStep),
    [currentStep, getSectionsForStep]
  );

  // Build the full form schema for all sections (used for final submit)
  const fullSchema = useMemo(() => buildFormSchema(sections), [sections]);

  const defaults = useMemo(
    () => initialData || getDefaultValues(sections),
    [initialData, sections]
  );

  const methods = useForm({
    resolver: zodResolver(fullSchema),
    defaultValues: defaults,
    values: initialData,
    mode: "onTouched",
  });

  const handleNext = useCallback(async () => {
    // Validate only the current step's fields
    const stepFields = currentSections.flatMap((s) =>
      s.fields.map((f) => f.field_key)
    );

    const isValid = await methods.trigger(stepFields);
    if (!isValid) {
      toast.error(t("fixErrors"));
      return;
    }

    setCompletedSteps((prev) =>
      prev.includes(currentStep) ? prev : [...prev, currentStep]
    );
    setCurrentStep((prev) => Math.min(prev + 1, steps.length - 1));
  }, [currentStep, currentSections, methods, steps.length, t]);

  const handleBack = useCallback(() => {
    setCurrentStep((prev) => Math.max(prev - 1, 0));
  }, []);

  const handleSubmit = methods.handleSubmit(async (data) => {
    try {
      if (recordId) {
        await updateRecord({ id: recordId, data });
      } else {
        await createRecord(data);
      }
      // Mark last step as completed
      setCompletedSteps((prev) =>
        prev.includes(currentStep) ? prev : [...prev, currentStep]
      );
      toast.success(recordId ? t("recordUpdated") : t("recordCreated"));
      onSubmitSuccess?.();
    } catch {
      toast.error(t("formSubmitFailed"));
    }
  });

  return (
    <FormProvider {...methods}>
      <form onSubmit={handleSubmit} className="space-y-6">
        {/* Step indicator */}
        <Card>
          <WizardStepIndicator
            steps={steps.map((s) => ({
              title: s.title,
              description: s.description,
            }))}
            currentStep={currentStep}
            completedSteps={completedSteps}
          />
        </Card>

        {/* Step title */}
        <div>
          <h2 className="text-lg font-semibold text-gray-900">
            {steps[currentStep]?.title}
          </h2>
          {steps[currentStep]?.description && (
            <p className="mt-1 text-sm text-gray-500">
              {steps[currentStep].description}
            </p>
          )}
        </div>

        {/* Current step sections */}
        {currentSections.length === 0 ? (
          <div className="py-8 text-center text-sm text-gray-400">
            {t("noSectionsAvailable")}
          </div>
        ) : (
          currentSections.map((section) => (
            <DynamicSection key={section.id} section={section} />
          ))
        )}

        {/* Navigation buttons */}
        <div className="flex items-center justify-between border-t border-gray-200 pt-4">
          <div>
            {!isFirstStep && (
              <Button
                type="button"
                variant="secondary"
                onClick={handleBack}
              >
                {tCommon("back")}
              </Button>
            )}
          </div>

          <div className="flex items-center gap-3">
            <span className="text-xs text-gray-400">
              {t("stepOf", { current: currentStep + 1, total: steps.length })}
            </span>

            {isLastStep ? (
              <Button type="submit" loading={isSubmitting}>
                {recordId ? tCommon("update") : tCommon("submit")}
              </Button>
            ) : (
              <Button type="button" onClick={handleNext}>
                {t("nextStep")}
              </Button>
            )}
          </div>
        </div>
      </form>
    </FormProvider>
  );
}
