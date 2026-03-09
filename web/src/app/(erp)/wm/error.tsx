"use client";

import { ModuleError } from "@/components/layout/module-error";

export default function WmError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  return (
    <ModuleError
      moduleName="Warehouse Management"
      moduleCode="WM"
      error={error}
      reset={reset}
      fallbackHref="/wm"
    />
  );
}
