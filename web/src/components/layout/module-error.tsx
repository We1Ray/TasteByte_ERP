"use client";

import { useEffect } from "react";
import { AlertTriangle, RefreshCw, ArrowLeft } from "lucide-react";
import { useTranslations } from "next-intl";

interface ModuleErrorProps {
  moduleName: string;
  moduleCode: string;
  error: Error & { digest?: string };
  reset: () => void;
  fallbackHref: string;
}

export function ModuleError({
  moduleName,
  moduleCode,
  error,
  reset,
  fallbackHref,
}: ModuleErrorProps) {
  const t = useTranslations("layout");

  useEffect(() => {
    console.error(`[${moduleCode}] Error:`, error);
  }, [moduleCode, error]);

  const is403 = error.message?.includes("403") || error.message?.includes("Forbidden");
  const is404 = error.message?.includes("404") || error.message?.includes("Not found");

  return (
    <div className="flex min-h-[50vh] items-center justify-center">
      <div className="mx-auto max-w-md text-center">
        <div className="mx-auto mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-red-100">
          <AlertTriangle className="h-7 w-7 text-red-600" />
        </div>

        <p className="mb-1 text-xs font-semibold uppercase tracking-wider text-gray-400">
          {moduleName}
        </p>

        <h2 className="mb-2 text-lg font-semibold text-gray-900">
          {is403
            ? t("accessDenied")
            : is404
              ? t("pageNotFound")
              : t("somethingWentWrong")}
        </h2>

        <p className="mb-6 text-sm text-gray-500">
          {is403
            ? t("accessDeniedDesc")
            : is404
              ? t("pageNotFoundDesc")
              : error.message || t("unexpectedError")}
        </p>

        <div className="flex items-center justify-center gap-3">
          <button
            onClick={reset}
            className="inline-flex items-center gap-2 rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700"
          >
            <RefreshCw className="h-4 w-4" />
            {t("tryAgain")}
          </button>
          <a
            href={fallbackHref}
            className="inline-flex items-center gap-2 rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50"
          >
            <ArrowLeft className="h-4 w-4" />
            {t("backTo", { module: moduleCode })}
          </a>
        </div>

        {error.digest && (
          <p className="mt-4 text-xs text-gray-400">Error ID: {error.digest}</p>
        )}
      </div>
    </div>
  );
}
