"use client";

import { useState } from "react";
import { useParams, useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import { ArrowLeft } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { SearchBar } from "@/components/forms/search-bar";
import { PageLoading } from "@/components/ui/loading";
import { ConfirmDialog } from "@/components/ui/confirm-dialog";
import { JournalTimeline } from "@/components/lowcode/journal/JournalTimeline";
import { DiffViewer } from "@/components/lowcode/journal/DiffViewer";
import { useApiQuery, useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { usePagination } from "@/lib/hooks/use-pagination";
import { operationsApi, journalApi } from "@/lib/api/lowcode";
import type { JournalEntry } from "@/lib/types/lowcode";

export default function OperationJournalPage() {
  const params = useParams();
  const router = useRouter();
  const operationId = params.id as string;
  const t = useTranslations("developer");
  const tCommon = useTranslations("common");
  const { page, pageSize, goToPage } = usePagination();
  const invalidate = useInvalidateQueries();

  const [typeFilter, setTypeFilter] = useState("");
  const [diffEntry, setDiffEntry] = useState<JournalEntry | null>(null);
  const [rollbackTarget, setRollbackTarget] = useState<JournalEntry | null>(null);

  const { data: operation, isLoading: opLoading } = useApiQuery(
    ["lowcode", "operations", operationId],
    () => operationsApi.get(operationId)
  );

  const { data: journalEntries, isLoading } = useApiQuery(
    ["lowcode", "journal", operationId, String(page), typeFilter],
    () =>
      journalApi.list(operationId, {
        page,
        page_size: pageSize,
        change_type: typeFilter || undefined,
      })
  );

  const rollbackMutation = useApiMutation(
    (entry: JournalEntry) => {
      if (!entry.operation_id || entry.version == null) {
        return Promise.reject(new Error("Missing operation_id or version for rollback"));
      }
      return journalApi.rollback(entry.operation_id, entry.version);
    },
    { onSuccess: () => invalidate(["lowcode", "journal"]) }
  );

  if (opLoading) return <PageLoading />;

  const title = operation
    ? `${operation.code} - ${t("changeJournal")}`
    : t("changeJournal");

  return (
    <div>
      <PageHeader
        title={title}
        description={t("journalDesc")}
        actions={
          <Button
            variant="secondary"
            onClick={() => router.push(`/developer/operations/${operationId}`)}
          >
            <ArrowLeft className="h-4 w-4" />
            {tCommon("back")}
          </Button>
        }
      />

      <div className="mb-6">
        <SearchBar
          placeholder={t("filterJournal")}
          onSearch={() => {}}
          filters={[
            {
              key: "change_type",
              label: tCommon("allTypes"),
              value: typeFilter,
              onChange: setTypeFilter,
              options: [
                { value: "create", label: tCommon("create") },
                { value: "update", label: tCommon("update") },
                { value: "delete", label: tCommon("delete") },
                { value: "publish", label: tCommon("publish") },
                { value: "rollback", label: tCommon("rollback") },
              ],
            },
          ]}
        />
      </div>

      {isLoading ? (
        <PageLoading />
      ) : (
        <>
          <JournalTimeline
            entries={journalEntries?.items ?? []}
            onViewDiff={setDiffEntry}
            onRollback={(entry) => {
              if (entry.version == null) {
                alert(t("noSnapshotToRollback"));
                return;
              }
              setRollbackTarget(entry);
            }}
          />

          {journalEntries && journalEntries.total_pages > 1 && (
            <div className="mt-6 flex items-center justify-center gap-2">
              <button
                onClick={() => goToPage(page - 1)}
                disabled={page <= 1}
                className="rounded-md px-3 py-1 text-sm text-gray-600 hover:bg-gray-100 disabled:opacity-50"
              >
                {tCommon("previous")}
              </button>
              <span className="text-sm text-gray-500">
                {tCommon("pageOf", { page, totalPages: journalEntries.total_pages })}
              </span>
              <button
                onClick={() => goToPage(page + 1)}
                disabled={page >= journalEntries.total_pages}
                className="rounded-md px-3 py-1 text-sm text-gray-600 hover:bg-gray-100 disabled:opacity-50"
              >
                {tCommon("next")}
              </button>
            </div>
          )}
        </>
      )}

      <DiffViewer
        open={!!diffEntry}
        onClose={() => setDiffEntry(null)}
        entry={diffEntry}
      />

      <ConfirmDialog
        open={!!rollbackTarget}
        onClose={() => setRollbackTarget(null)}
        onConfirm={() => {
          if (rollbackTarget) {
            rollbackMutation.mutateAsync(rollbackTarget);
            setRollbackTarget(null);
          }
        }}
        title={t("rollbackVersion")}
        message={t("rollbackConfirm")}
        confirmLabel={tCommon("rollback")}
        variant="warning"
        loading={rollbackMutation.isPending}
      />
    </div>
  );
}
