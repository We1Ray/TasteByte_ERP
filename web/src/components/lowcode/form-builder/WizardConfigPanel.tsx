"use client";

import { Plus, Trash2, ArrowUp, ArrowDown } from "lucide-react";
import { useTranslations } from "next-intl";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import type { FormSection, WizardConfig } from "@/lib/types/lowcode";

interface WizardConfigPanelProps {
  wizardConfig: WizardConfig;
  onChange: (config: WizardConfig) => void;
  sections: FormSection[];
}

export function WizardConfigPanel({
  wizardConfig,
  onChange,
  sections,
}: WizardConfigPanelProps) {
  const t = useTranslations("lowcode");
  const steps = wizardConfig.steps;

  // Find which sections are already assigned to a step
  const assignedSectionIds = new Set(steps.flatMap((s) => s.sectionIds));

  const addStep = () => {
    onChange({
      ...wizardConfig,
      steps: [
        ...steps,
        {
          title: `Step ${steps.length + 1}`,
          sectionIds: [],
          description: "",
        },
      ],
    });
  };

  const removeStep = (index: number) => {
    const newSteps = steps.filter((_, i) => i !== index);
    onChange({ ...wizardConfig, steps: newSteps });
  };

  const updateStep = (
    index: number,
    updates: Partial<WizardConfig["steps"][number]>
  ) => {
    const newSteps = steps.map((step, i) =>
      i === index ? { ...step, ...updates } : step
    );
    onChange({ ...wizardConfig, steps: newSteps });
  };

  const moveStep = (index: number, direction: -1 | 1) => {
    const targetIndex = index + direction;
    if (targetIndex < 0 || targetIndex >= steps.length) return;
    const newSteps = [...steps];
    const [moved] = newSteps.splice(index, 1);
    newSteps.splice(targetIndex, 0, moved);
    onChange({ ...wizardConfig, steps: newSteps });
  };

  const toggleSectionInStep = (stepIndex: number, sectionId: string) => {
    const step = steps[stepIndex];
    const hasSectionId = step.sectionIds.includes(sectionId);
    const newSectionIds = hasSectionId
      ? step.sectionIds.filter((id) => id !== sectionId)
      : [...step.sectionIds, sectionId];

    updateStep(stepIndex, { sectionIds: newSectionIds });
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h4 className="text-sm font-semibold text-gray-900">{t("wizardSteps")}</h4>
        <Button type="button" variant="ghost" size="sm" onClick={addStep}>
          <Plus className="h-3.5 w-3.5" />
          {t("addStep")}
        </Button>
      </div>

      {steps.length === 0 && (
        <p className="text-xs text-gray-400 py-2">
          {t("noStepsConfigured")}
        </p>
      )}

      <div className="space-y-3">
        {steps.map((step, index) => (
          <div
            key={index}
            className="rounded-lg border border-gray-200 bg-white overflow-hidden"
          >
            {/* Step header */}
            <div className="flex items-center gap-2 px-3 py-2 bg-gray-50 border-b border-gray-200">
              <span className="flex h-5 w-5 items-center justify-center rounded-full bg-blue-100 text-xs font-semibold text-blue-700">
                {index + 1}
              </span>

              <span className="flex-1 text-sm font-medium text-gray-700 truncate">
                {step.title || `Step ${index + 1}`}
              </span>

              <div className="flex items-center gap-0.5 shrink-0">
                <button
                  type="button"
                  onClick={() => moveStep(index, -1)}
                  disabled={index === 0}
                  className="rounded p-0.5 text-gray-400 hover:text-gray-600 disabled:opacity-30"
                  title={t("moveUp")}
                >
                  <ArrowUp className="h-3 w-3" />
                </button>
                <button
                  type="button"
                  onClick={() => moveStep(index, 1)}
                  disabled={index === steps.length - 1}
                  className="rounded p-0.5 text-gray-400 hover:text-gray-600 disabled:opacity-30"
                  title={t("moveDown")}
                >
                  <ArrowDown className="h-3 w-3" />
                </button>
                <button
                  type="button"
                  onClick={() => removeStep(index)}
                  className="rounded p-0.5 text-red-400 hover:text-red-600"
                  title={t("removeStep")}
                >
                  <Trash2 className="h-3 w-3" />
                </button>
              </div>
            </div>

            {/* Step details */}
            <div className="px-3 py-2.5 space-y-3">
              <Input
                label={t("sectionTitle")}
                value={step.title}
                onChange={(e) => updateStep(index, { title: e.target.value })}
                placeholder={t("stepTitlePlaceholder")}
              />

              <Input
                label={t("stepDescriptionLabel")}
                value={step.description || ""}
                onChange={(e) =>
                  updateStep(index, { description: e.target.value })
                }
                placeholder={t("stepDescriptionPlaceholder")}
              />

              {/* Section assignment */}
              <div>
                <label className="mb-1.5 block text-sm font-medium text-gray-700">
                  {t("assignedSections")}
                </label>

                {sections.length === 0 ? (
                  <p className="text-xs text-gray-400">
                    {t("noSectionsAvailable")}
                  </p>
                ) : (
                  <div className="space-y-1 max-h-40 overflow-y-auto">
                    {sections.map((section) => {
                      const isAssigned = step.sectionIds.includes(section.id);
                      const isAssignedElsewhere =
                        !isAssigned && assignedSectionIds.has(section.id);

                      return (
                        <label
                          key={section.id}
                          className={`flex items-center gap-2 rounded px-2 py-1.5 text-sm transition-colors cursor-pointer ${
                            isAssigned
                              ? "bg-blue-50 text-blue-700"
                              : isAssignedElsewhere
                              ? "text-gray-400"
                              : "text-gray-600 hover:bg-gray-50"
                          }`}
                        >
                          <input
                            type="checkbox"
                            checked={isAssigned}
                            onChange={() =>
                              toggleSectionInStep(index, section.id)
                            }
                            disabled={isAssignedElsewhere}
                            className="h-3.5 w-3.5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                          />
                          <span className="truncate">
                            {section.title || "Untitled Section"}
                          </span>
                          {isAssignedElsewhere && (
                            <span className="ml-auto text-[10px] text-gray-400 shrink-0">
                              {t("sectionUsedElsewhere")}
                            </span>
                          )}
                        </label>
                      );
                    })}
                  </div>
                )}
              </div>
            </div>
          </div>
        ))}
      </div>

      {steps.length > 0 && (
        <p className="text-xs text-gray-500">
          {t("stepAssignmentHint")}
        </p>
      )}
    </div>
  );
}
