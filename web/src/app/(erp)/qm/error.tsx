"use client";

import { ModuleError } from "@/components/layout/module-error";

export default function QmError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  return (
    <ModuleError
      moduleName="Quality Management"
      moduleCode="QM"
      error={error}
      reset={reset}
      fallbackHref="/qm"
    />
  );
}
