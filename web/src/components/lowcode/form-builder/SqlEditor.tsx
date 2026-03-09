"use client";

import { useState } from "react";
import { CheckCircle, XCircle, Play } from "lucide-react";
import { useTranslations } from "next-intl";
import { Button } from "@/components/ui/button";
import { datasourceApi } from "@/lib/api/lowcode";
import type { SqlValidationResult } from "@/lib/types/lowcode";

interface SqlEditorProps {
  value: string;
  onChange: (value: string) => void;
  label?: string;
}

export function SqlEditor({ value, onChange, label }: SqlEditorProps) {
  const [validationResult, setValidationResult] = useState<SqlValidationResult | null>(null);
  const [validating, setValidating] = useState(false);
  const t = useTranslations("lowcode");

  const handleValidate = async () => {
    setValidating(true);
    try {
      const result = await datasourceApi.validateSql(value);
      setValidationResult(result);
    } catch {
      setValidationResult({ valid: false, error: t("saveFailed") });
    } finally {
      setValidating(false);
    }
  };

  return (
    <div className="w-full">
      {label && (
        <label className="mb-1 block text-sm font-medium text-gray-700">{label}</label>
      )}
      <div className="overflow-hidden rounded-md border border-gray-300">
        <textarea
          value={value}
          onChange={(e) => {
            onChange(e.target.value);
            setValidationResult(null);
          }}
          rows={6}
          className="block w-full border-0 px-3 py-2 font-mono text-sm focus:outline-none focus:ring-0"
          placeholder={t("sqlQueryPlaceholder")}
          spellCheck={false}
        />
        <div className="flex items-center justify-between border-t bg-gray-50 px-3 py-2">
          <div className="flex items-center gap-2">
            {validationResult && (
              validationResult.valid ? (
                <span className="flex items-center gap-1 text-xs text-green-600">
                  <CheckCircle className="h-3.5 w-3.5" />
                  {t("validSql")}
                  {validationResult.columns && (
                    <span className="text-gray-400">
                      ({validationResult.columns.join(", ")})
                    </span>
                  )}
                </span>
              ) : (
                <span className="flex items-center gap-1 text-xs text-red-600">
                  <XCircle className="h-3.5 w-3.5" />
                  {validationResult.error}
                </span>
              )
            )}
          </div>
          <Button
            variant="secondary"
            size="sm"
            onClick={handleValidate}
            loading={validating}
            disabled={!value.trim()}
          >
            <Play className="h-3 w-3" />
            {t("validate")}
          </Button>
        </div>
      </div>
    </div>
  );
}
