"use client";

import { ModuleError } from "@/components/layout/module-error";

export default function HrError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  return (
    <ModuleError
      moduleName="Human Resources"
      moduleCode="HR"
      error={error}
      reset={reset}
      fallbackHref="/hr/employees"
    />
  );
}
