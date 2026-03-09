"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { FileText, BarChart3, Workflow } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { SearchBar } from "@/components/forms/search-bar";
import { Badge } from "@/components/ui/badge";
import { PageLoading } from "@/components/ui/loading";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { operationsApi } from "@/lib/api/lowcode";

const typeIcons: Record<string, React.ElementType> = {
  form: FileText,
  report: BarChart3,
  workflow: Workflow,
};

const typeColors: Record<string, string> = {
  form: "blue",
  report: "green",
  workflow: "purple",
};

export default function LowCodeDirectoryPage() {
  const router = useRouter();
  const [search, setSearch] = useState("");

  const { data, isLoading } = useApiQuery(
    ["lowcode", "operations", "published", search],
    () => operationsApi.list({ status: "published", search: search || undefined, page_size: 100 })
  );

  if (isLoading) return <PageLoading />;

  const operations = data?.items ?? [];

  return (
    <div>
      <PageHeader
        title="Low-Code Operations"
        description="Browse and use available operations"
      />

      <div className="mb-6">
        <SearchBar placeholder="Search operations..." onSearch={setSearch} />
      </div>

      {operations.length === 0 ? (
        <div className="py-12 text-center text-sm text-gray-500">
          No operations available
        </div>
      ) : (
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
          {operations.map((op) => {
            const Icon = typeIcons[op.operation_type] || FileText;
            return (
              <div
                key={op.id}
                className="cursor-pointer rounded-lg border border-gray-200 bg-white p-6 shadow-sm transition-colors hover:border-blue-300 hover:shadow-md"
                onClick={() => router.push(`/lowcode/${op.code}`)}
              >
                <div className="flex items-start gap-3">
                  <div className="rounded-lg bg-blue-50 p-2">
                    <Icon className="h-5 w-5 text-blue-600" />
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                      <h3 className="truncate text-sm font-semibold text-gray-900">{op.name}</h3>
                      <Badge color={typeColors[op.operation_type] as "blue" | "green" | "purple"}>
                        {op.operation_type}
                      </Badge>
                    </div>
                    <p className="mt-1 line-clamp-2 text-sm text-gray-500">{op.description}</p>
                    <p className="mt-2 text-xs text-gray-400">Code: {op.code}</p>
                  </div>
                </div>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}
