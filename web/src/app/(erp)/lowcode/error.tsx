"use client";

import { ModuleError } from "@/components/layout/module-error";

export default function LowcodeError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  return (
    <ModuleError
      moduleName="Low-Code"
      moduleCode="LC"
      error={error}
      reset={reset}
      fallbackHref="/lowcode"
    />
  );
}
