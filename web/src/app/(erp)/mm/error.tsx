"use client";

import { ModuleError } from "@/components/layout/module-error";

export default function MmError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  return (
    <ModuleError
      moduleName="Materials Management"
      moduleCode="MM"
      error={error}
      reset={reset}
      fallbackHref="/mm/materials"
    />
  );
}
