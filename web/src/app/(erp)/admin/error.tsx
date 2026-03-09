"use client";

import { ModuleError } from "@/components/layout/module-error";

export default function AdminError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  return (
    <ModuleError
      moduleName="Administration"
      moduleCode="Admin"
      error={error}
      reset={reset}
      fallbackHref="/admin"
    />
  );
}
