"use client";

import { useState } from "react";
import { Play, Code, AlertCircle, CheckCircle } from "lucide-react";
import { useTranslations } from "next-intl";
import { Button } from "@/components/ui/button";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { useBuilderStore } from "@/lib/stores/builder-store";
import { DynamicForm } from "../form-renderer/DynamicForm";
import { datasourceApi } from "@/lib/api/lowcode";
import type { FieldDefinition } from "@/lib/types/lowcode";

interface TestModePanelProps {
  operationCode: string;
}

interface SqlTestResult {
  fieldKey: string;
  sqlQuery: string;
  status: "pending" | "success" | "error";
  data?: { columns: string[]; rows: Record<string, unknown>[]; total: number };
  error?: string;
}

export function TestModePanel({ operationCode }: TestModePanelProps) {
  const { sections } = useBuilderStore();
  const t = useTranslations("lowcode");
  const [submittedData, setSubmittedData] = useState<Record<string, unknown> | null>(null);
  const [sqlResults, setSqlResults] = useState<SqlTestResult[]>([]);
  const [testingDataSources, setTestingDataSources] = useState(false);

  // Collect all fields with SQL data sources
  const sqlFields: { field: FieldDefinition; sectionTitle: string }[] = [];
  for (const section of sections) {
    for (const field of section.fields) {
      if (field.data_source?.type === "sql" && field.data_source.sql_query) {
        sqlFields.push({ field, sectionTitle: section.title });
      }
    }
  }

  const handleTestSubmit = (data: Record<string, unknown>) => {
    setSubmittedData(data);
  };

  const handleTestDataSources = async () => {
    setTestingDataSources(true);
    const results: SqlTestResult[] = sqlFields.map(({ field }) => ({
      fieldKey: field.field_key,
      sqlQuery: field.data_source!.sql_query!,
      status: "pending" as const,
    }));
    setSqlResults(results);

    for (let i = 0; i < results.length; i++) {
      try {
        const data = await datasourceApi.query(results[i].sqlQuery);
        results[i] = { ...results[i], status: "success", data };
      } catch (err: unknown) {
        const message = err instanceof Error ? err.message : t("queryExecutionFailed");
        results[i] = { ...results[i], status: "error", error: message };
      }
      setSqlResults([...results]);
    }
    setTestingDataSources(false);
  };

  return (
    <div className="space-y-6 p-6">
      <div>
        <h3 className="mb-4 text-lg font-semibold text-gray-900">{t("testMode")}</h3>
        <p className="mb-4 text-sm text-gray-500">
          {t("testModeDescription")}
        </p>
        <DynamicForm
          operationCode={operationCode}
          preview={false}
          sections={sections}
          onTestSubmit={handleTestSubmit}
        />
      </div>

      {submittedData && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Code className="h-4 w-4" />
              {t("submittedFormData")}
            </CardTitle>
          </CardHeader>
          <pre className="max-h-64 overflow-auto rounded-md bg-gray-900 p-4 text-sm text-green-400">
            {JSON.stringify(submittedData, null, 2)}
          </pre>
        </Card>
      )}

      {sqlFields.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Play className="h-4 w-4" />
              {t("dataSourceTests")}
            </CardTitle>
            <Button
              size="sm"
              onClick={handleTestDataSources}
              loading={testingDataSources}
            >
              {t("testDataSources")}
            </Button>
          </CardHeader>
          <div className="space-y-3">
            {sqlFields.map(({ field, sectionTitle }) => {
              const result = sqlResults.find((r) => r.fieldKey === field.field_key);
              return (
                <div key={field.id} className="rounded-md border border-gray-200 p-3">
                  <div className="mb-2 flex items-center justify-between">
                    <div>
                      <span className="text-sm font-medium text-gray-900">{field.label}</span>
                      <span className="ml-2 text-xs text-gray-500">({sectionTitle} / {field.field_key})</span>
                    </div>
                    {result?.status === "success" && (
                      <Badge color="green">
                        <CheckCircle className="mr-1 h-3 w-3" />
                        {result.data?.rows.length || 0} {t("rows")}
                      </Badge>
                    )}
                    {result?.status === "error" && (
                      <Badge color="red">
                        <AlertCircle className="mr-1 h-3 w-3" />
                        {t("queryExecutionFailed")}
                      </Badge>
                    )}
                    {result?.status === "pending" && (
                      <Badge color="amber">{t("running")}</Badge>
                    )}
                  </div>
                  <pre className="rounded bg-gray-50 p-2 text-xs text-gray-600">{field.data_source?.sql_query}</pre>
                  {result?.status === "error" && (
                    <p className="mt-2 text-xs text-red-600">{result.error}</p>
                  )}
                  {result?.status === "success" && result.data && result.data.rows.length > 0 && (
                    <div className="mt-2 max-h-32 overflow-auto">
                      <table className="w-full text-xs">
                        <thead>
                          <tr className="border-b">
                            {result.data.columns.map((col) => (
                              <th key={col} className="p-1 text-left font-medium text-gray-500">{col}</th>
                            ))}
                          </tr>
                        </thead>
                        <tbody>
                          {result.data.rows.slice(0, 5).map((row, i) => (
                            <tr key={i} className="border-b border-gray-100">
                              {result.data!.columns.map((col) => (
                                <td key={col} className="p-1 text-gray-700">{String(row[col] ?? "")}</td>
                              ))}
                            </tr>
                          ))}
                        </tbody>
                      </table>
                      {result.data.rows.length > 5 && (
                        <p className="mt-1 text-xs text-gray-500">{t("moreRows", { count: result.data.rows.length - 5 })}</p>
                      )}
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        </Card>
      )}
    </div>
  );
}
