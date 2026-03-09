"use client";

import { ModuleError } from "@/components/layout/module-error";

export default function DeveloperError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  return (
    <ModuleError
      moduleName="Developer Mode"
      moduleCode="Dev"
      error={error}
      reset={reset}
      fallbackHref="/developer"
    />
  );
}
