"use client";

import { useState } from "react";
import { useTranslations } from "next-intl";
import { ConfirmDialog } from "@/components/ui/confirm-dialog";
import { PageHeader } from "@/components/layout/page-header";
import { SearchBar } from "@/components/forms/search-bar";
import { Select } from "@/components/ui/select";
import { PageLoading } from "@/components/ui/loading";
import { JournalTimeline } from "@/components/lowcode/journal/JournalTimeline";
import { DiffViewer } from "@/components/lowcode/journal/DiffViewer";
import { useApiQuery, useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { usePagination } from "@/lib/hooks/use-pagination";
import { journalApi, operationsApi } from "@/lib/api/lowcode";
import type { JournalEntry } from "@/lib/types/lowcode";

export default function JournalPage() {
  const t = useTranslations("developer");
  const tCommon = useTranslations("common");
  const { page, pageSize, goToPage } = usePagination();
  const invalidate = useInvalidateQueries();
  const [selectedOperationId, setSelectedOperationId] = useState("");
  const [typeFilter, setTypeFilter] = useState("");
  const [diffEntry, setDiffEntry] = useState<JournalEntry | null>(null);
  const [rollbackTarget, setRollbackTarget] = useState<JournalEntry | null>(null);

  const { data: operations, isLoading: opsLoading } = useApiQuery(
    ["lowcode", "operations", "all-for-journal"],
    () => operationsApi.list({ page_size: 100 })
  );

  const { data, isLoading } = useApiQuery(
    ["lowcode", "journal", selectedOperationId, String(page), typeFilter],
    () => journalApi.list(selectedOperationId, {
      page,
      page_size: pageSize,
      change_type: typeFilter || undefined,
    }),
    { enabled: !!selectedOperationId }
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

  if (opsLoading) return <PageLoading />;

  const operationOptions = (operations?.items ?? []).map((op) => ({
    value: op.id,
    label: `${op.code} - ${op.name}`,
  }));

  return (
    <div>
      <PageHeader
        title={t("changeJournal")}
        description={t("journalDesc")}
      />

      <div className="mb-6 space-y-4">
        <Select
          label={t("selectOperation")}
          value={selectedOperationId}
          onChange={(e) => {
            setSelectedOperationId(e.target.value);
            goToPage(1);
          }}
          options={operationOptions}
          placeholder={t("chooseOperation")}
        />

        {selectedOperationId && (
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
        )}
      </div>

      {!selectedOperationId ? (
        <div className="py-12 text-center text-sm text-gray-500">
          {t("selectOperationPrompt")}
        </div>
      ) : isLoading ? (
        <PageLoading />
      ) : (
        <>
          <JournalTimeline
            entries={data?.items ?? []}
            onViewDiff={setDiffEntry}
            onRollback={(entry) => {
              if (entry.version == null) {
                alert("This journal entry has no version snapshot to rollback to.");
                return;
              }
              setRollbackTarget(entry);
            }}
          />

          {data && data.total_pages > 1 && (
            <div className="mt-6 flex items-center justify-center gap-2">
              <button
                onClick={() => goToPage(page - 1)}
                disabled={page <= 1}
                className="rounded-md px-3 py-1 text-sm text-gray-600 hover:bg-gray-100 disabled:opacity-50"
              >
                {tCommon("previous")}
              </button>
              <span className="text-sm text-gray-500">
                {tCommon("pageOf", { page, totalPages: data.total_pages })}
              </span>
              <button
                onClick={() => goToPage(page + 1)}
                disabled={page >= data.total_pages}
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
