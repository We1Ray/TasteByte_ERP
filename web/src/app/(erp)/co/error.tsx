"use client";

import { ModuleError } from "@/components/layout/module-error";

export default function CoError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  return (
    <ModuleError
      moduleName="Controlling"
      moduleCode="CO"
      error={error}
      reset={reset}
      fallbackHref="/co"
    />
  );
}
