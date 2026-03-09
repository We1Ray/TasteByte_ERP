"use client";

import { ModuleError } from "@/components/layout/module-error";

export default function PpError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  return (
    <ModuleError
      moduleName="Production Planning"
      moduleCode="PP"
      error={error}
      reset={reset}
      fallbackHref="/pp/production-orders"
    />
  );
}
