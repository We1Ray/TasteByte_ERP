"use client";

import { Check } from "lucide-react";
import { useTranslations } from "next-intl";
import { cn } from "@/lib/utils";

interface WizardStepIndicatorProps {
  steps: { title: string; description?: string }[];
  currentStep: number;
  completedSteps: number[];
}

export function WizardStepIndicator({
  steps,
  currentStep,
  completedSteps,
}: WizardStepIndicatorProps) {
  const t = useTranslations("lowcode");
  return (
    <div className="w-full">
      {/* Desktop: full step indicator */}
      <nav aria-label="Progress" className="hidden sm:block">
        <ol className="flex items-center">
          {steps.map((step, index) => {
            const isCompleted = completedSteps.includes(index);
            const isCurrent = index === currentStep;
            const isLast = index === steps.length - 1;

            return (
              <li
                key={index}
                className={cn("flex items-center", !isLast && "flex-1")}
              >
                <div className="flex flex-col items-center">
                  {/* Circle */}
                  <div
                    className={cn(
                      "flex h-8 w-8 items-center justify-center rounded-full text-sm font-semibold transition-colors",
                      isCompleted &&
                        "bg-green-600 text-white",
                      isCurrent &&
                        !isCompleted &&
                        "bg-blue-600 text-white ring-4 ring-blue-100",
                      !isCurrent &&
                        !isCompleted &&
                        "border-2 border-gray-300 text-gray-400 bg-white"
                    )}
                  >
                    {isCompleted ? (
                      <Check className="h-4 w-4" />
                    ) : (
                      index + 1
                    )}
                  </div>

                  {/* Label */}
                  <div className="mt-2 text-center">
                    <p
                      className={cn(
                        "text-xs font-medium",
                        isCurrent
                          ? "text-blue-600"
                          : isCompleted
                          ? "text-green-600"
                          : "text-gray-500"
                      )}
                    >
                      {step.title}
                    </p>
                    {step.description && (
                      <p className="mt-0.5 text-[10px] text-gray-400 max-w-[100px] truncate">
                        {step.description}
                      </p>
                    )}
                  </div>
                </div>

                {/* Connector line */}
                {!isLast && (
                  <div
                    className={cn(
                      "mx-2 h-0.5 flex-1 transition-colors",
                      isCompleted ? "bg-green-600" : "bg-gray-200"
                    )}
                  />
                )}
              </li>
            );
          })}
        </ol>
      </nav>

      {/* Mobile: show current step only */}
      <div className="sm:hidden">
        <p className="text-sm text-gray-500">
          {t("stepOf", { current: currentStep + 1, total: steps.length })}
        </p>
        <p className="text-sm font-semibold text-blue-600">
          {steps[currentStep]?.title}
        </p>
        {steps[currentStep]?.description && (
          <p className="text-xs text-gray-400 mt-0.5">
            {steps[currentStep].description}
          </p>
        )}
        {/* Progress bar */}
        <div className="mt-2 h-1.5 w-full rounded-full bg-gray-200">
          <div
            className="h-full rounded-full bg-blue-600 transition-all duration-300"
            style={{
              width: `${((currentStep + 1) / steps.length) * 100}%`,
            }}
          />
        </div>
      </div>
    </div>
  );
}
