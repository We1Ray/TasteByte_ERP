"use client";

import { ModuleError } from "@/components/layout/module-error";

export default function SdError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  return (
    <ModuleError
      moduleName="Sales & Distribution"
      moduleCode="SD"
      error={error}
      reset={reset}
      fallbackHref="/sd/sales-orders"
    />
  );
}
