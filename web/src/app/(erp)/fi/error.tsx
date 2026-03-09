"use client";

import { ModuleError } from "@/components/layout/module-error";

export default function FiError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  return (
    <ModuleError
      moduleName="Financial Accounting"
      moduleCode="FI"
      error={error}
      reset={reset}
      fallbackHref="/fi/accounts"
    />
  );
}
