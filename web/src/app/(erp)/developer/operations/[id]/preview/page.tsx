"use client";

import { useState } from "react";
import { useParams, useRouter } from "next/navigation";
import { ArrowLeft } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { PreviewPanel } from "@/components/lowcode/form-builder/PreviewPanel";
import { TestModePanel } from "@/components/lowcode/form-builder/TestModePanel";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { operationsApi } from "@/lib/api/lowcode";

export default function FormPreviewPage() {
  const params = useParams();
  const router = useRouter();
  const id = params.id as string;
  const [mode, setMode] = useState<"preview" | "test">("preview");

  const { data: operation } = useApiQuery(
    ["lowcode", "operations", id],
    () => operationsApi.get(id),
    { enabled: id !== "new" }
  );

  return (
    <div>
      <PageHeader
        title={`Preview: ${operation?.name || "Form"}`}
        description="This is how the form will appear to users"
        actions={
          <div className="flex items-center gap-2">
            <div className="inline-flex rounded-md border border-gray-300">
              <button
                onClick={() => setMode("preview")}
                className={`px-3 py-1.5 text-sm font-medium ${
                  mode === "preview"
                    ? "bg-gray-900 text-white"
                    : "bg-white text-gray-700 hover:bg-gray-50"
                } rounded-l-md`}
              >
                Preview
              </button>
              <button
                onClick={() => setMode("test")}
                className={`px-3 py-1.5 text-sm font-medium ${
                  mode === "test"
                    ? "bg-gray-900 text-white"
                    : "bg-white text-gray-700 hover:bg-gray-50"
                } rounded-r-md`}
              >
                Test
              </button>
            </div>
            <Button variant="secondary" onClick={() => router.push(`/developer/operations/${id}`)}>
              <ArrowLeft className="h-4 w-4" />
              Back to Builder
            </Button>
          </div>
        }
      />

      {mode === "preview" ? (
        <PreviewPanel operationCode={operation?.code || ""} />
      ) : (
        <TestModePanel operationCode={operation?.code || ""} />
      )}
    </div>
  );
}
