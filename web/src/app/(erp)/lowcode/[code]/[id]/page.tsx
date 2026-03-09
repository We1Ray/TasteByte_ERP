"use client";

import { useParams, useRouter } from "next/navigation";
import { ArrowLeft } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { PageLoading } from "@/components/ui/loading";
import { DynamicForm } from "@/components/lowcode/form-renderer/DynamicForm";
import { useDynamicForm, useFormRecord } from "@/lib/hooks/use-dynamic-form";

export default function LowCodeRecordPage() {
  const params = useParams();
  const router = useRouter();
  const code = params.code as string;
  const id = params.id as string;

  const { operation } = useDynamicForm(code);
  const { data: record, isLoading } = useFormRecord(code, id);

  if (isLoading) return <PageLoading />;

  return (
    <div>
      <PageHeader
        title={`Edit ${operation?.name || code}`}
        description={`Record ${id.slice(0, 8)}...`}
        actions={
          <Button variant="secondary" onClick={() => router.push(`/lowcode/${code}`)}>
            <ArrowLeft className="h-4 w-4" />
            Back to List
          </Button>
        }
      />

      <DynamicForm
        operationCode={code}
        recordId={id}
        initialData={record?.data}
        onSubmitSuccess={() => router.push(`/lowcode/${code}`)}
      />
    </div>
  );
}
