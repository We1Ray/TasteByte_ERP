"use client";

import { useState, useMemo } from "react";
import { CheckCircle, XCircle, Rocket } from "lucide-react";
import { type ColumnDef } from "@tanstack/react-table";
import { useTranslations } from "next-intl";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { DataTable } from "@/components/ui/data-table";
import { StatusBadge } from "@/components/ui/badge";
import { Modal } from "@/components/ui/modal";
import { SearchBar } from "@/components/forms/search-bar";
import { useApiQuery, useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { usePagination } from "@/lib/hooks/use-pagination";
import { releasesApi } from "@/lib/api/lowcode";
import { formatDateTime } from "@/lib/utils";
import type { Release } from "@/lib/types/lowcode";

export default function AdminReleasesPage() {
  const t = useTranslations("admin");
  const tCommon = useTranslations("common");
  const { page, pageSize, goToPage } = usePagination();
  const [search, setSearch] = useState("");
  const [statusFilter, setStatusFilter] = useState("");
  const invalidate = useInvalidateQueries();
  const [selectedRelease, setSelectedRelease] = useState<Release | null>(null);
  const [reviewComment, setReviewComment] = useState("");

  const columns: ColumnDef<Release, unknown>[] = useMemo(() => [
    { accessorKey: "version", header: t("version") },
    { accessorKey: "title", header: tCommon("name") },
    {
      accessorKey: "status",
      header: tCommon("status"),
      cell: ({ row }) => <StatusBadge status={row.original.status} />,
    },
    {
      accessorKey: "submitted_at",
      header: t("submitted"),
      cell: ({ row }) => row.original.submitted_at ? formatDateTime(row.original.submitted_at) : "-",
    },
    {
      accessorKey: "created_at",
      header: tCommon("createdAt"),
      cell: ({ row }) => formatDateTime(row.original.created_at),
    },
  ], [t, tCommon]);

  const { data, isLoading } = useApiQuery(
    ["lowcode", "releases", String(page), statusFilter],
    () => releasesApi.list({ page, page_size: pageSize, status: statusFilter || undefined })
  );

  const filteredItems = useMemo(() => {
    const items = data?.items || [];
    if (!search) return items;
    const lower = search.toLowerCase();
    return items.filter(
      (r) => String(r.version ?? "").toLowerCase().includes(lower) || r.title?.toLowerCase().includes(lower)
    );
  }, [data?.items, search]);

  const approveMutation = useApiMutation(
    ({ id, comment }: { id: string; comment?: string }) => releasesApi.approve(id, comment),
    {
      onSuccess: () => {
        invalidate(["lowcode", "releases"]);
        setSelectedRelease(null);
      },
    }
  );

  const rejectMutation = useApiMutation(
    ({ id, comment }: { id: string; comment: string }) => releasesApi.reject(id, comment),
    {
      onSuccess: () => {
        invalidate(["lowcode", "releases"]);
        setSelectedRelease(null);
      },
    }
  );

  const publishMutation = useApiMutation(
    (id: string) => releasesApi.publish(id),
    {
      onSuccess: () => {
        invalidate(["lowcode", "releases"]);
        setSelectedRelease(null);
      },
    }
  );

  return (
    <div>
      <PageHeader
        title={t("releaseApprovals")}
        description={t("reviewReleases")}
      />

      <div className="mb-4">
        <SearchBar
          placeholder={t("filterReleases")}
          onSearch={setSearch}
          filters={[
            {
              key: "status",
              label: t("allStatus"),
              value: statusFilter,
              onChange: setStatusFilter,
              options: [
                { value: "draft", label: t("statusDraft") },
                { value: "submitted", label: t("statusSubmitted") },
                { value: "approved", label: t("statusApproved") },
                { value: "rejected", label: t("statusRejected") },
                { value: "released", label: t("statusPublished") },
              ],
            },
          ]}
        />
      </div>

      <DataTable
        columns={columns}
        data={filteredItems}
        page={page}
        pageSize={pageSize}
        total={data?.total || 0}
        totalPages={data?.total_pages || 1}
        onPageChange={goToPage}
        onRowClick={setSelectedRelease}
        isLoading={isLoading}
        emptyTitle={t("noReleasesFound")}
      />

      <Modal
        open={!!selectedRelease}
        onClose={() => setSelectedRelease(null)}
        title={t("releaseVersion", { version: selectedRelease?.version || "" })}
        size="lg"
        footer={
          selectedRelease?.status === "submitted" ? (
            <>
              <Button
                variant="danger"
                loading={rejectMutation.isPending}
                onClick={() =>
                  rejectMutation.mutateAsync({ id: selectedRelease.id, comment: reviewComment })
                }
                disabled={!reviewComment}
              >
                <XCircle className="h-4 w-4" />
                {t("reject")}
              </Button>
              <Button
                loading={approveMutation.isPending}
                onClick={() =>
                  approveMutation.mutateAsync({ id: selectedRelease.id, comment: reviewComment || undefined })
                }
              >
                <CheckCircle className="h-4 w-4" />
                {t("approve")}
              </Button>
            </>
          ) : selectedRelease?.status === "approved" ? (
            <Button
              loading={publishMutation.isPending}
              onClick={() => publishMutation.mutateAsync(selectedRelease.id)}
            >
              <Rocket className="h-4 w-4" />
              {t("publish")}
            </Button>
          ) : null
        }
      >
        {selectedRelease && (
          <div className="space-y-4">
            <div>
              <h4 className="text-sm font-medium text-gray-700">{tCommon("name")}</h4>
              <p className="text-sm text-gray-900">{selectedRelease.title}</p>
            </div>
            <div>
              <h4 className="text-sm font-medium text-gray-700">{tCommon("description")}</h4>
              <p className="text-sm text-gray-900">{selectedRelease.description}</p>
            </div>

            {selectedRelease.release_number && (
              <div>
                <h4 className="text-sm font-medium text-gray-700">{t("releaseNumber")}</h4>
                <p className="text-sm font-mono text-gray-900">{selectedRelease.release_number}</p>
              </div>
            )}

            <div>
              <h4 className="text-sm font-medium text-gray-700">{tCommon("status")}</h4>
              <StatusBadge status={selectedRelease.status} />
            </div>

            <div>
              <h4 className="text-sm font-medium text-gray-700">{t("workflow")}</h4>
              <div className="mt-1 flex items-center gap-2">
                {(["draft", "submitted", "approved", "released"] as const).map((step, i) => {
                  const statusOrder = ["draft", "submitted", "approved", "released"];
                  const stepLabels: Record<string, string> = {
                    draft: t("statusDraft"),
                    submitted: t("statusSubmitted"),
                    approved: t("statusApproved"),
                    released: t("statusPublished"),
                  };
                  const isCurrent = selectedRelease.status === step;
                  const isRejected = selectedRelease.status === "rejected" && step === "submitted";
                  const isPast = statusOrder.indexOf(selectedRelease.status) > i;
                  return (
                    <div key={step} className="flex items-center gap-2">
                      {i > 0 && <div className={`h-px w-4 ${isPast ? "bg-blue-400" : "bg-gray-200"}`} />}
                      <div className={`flex h-6 items-center rounded-full px-2 text-xs font-medium ${
                        isCurrent ? "bg-blue-100 text-blue-700" :
                        isRejected ? "bg-red-100 text-red-700" :
                        isPast ? "bg-green-100 text-green-700" :
                        "bg-gray-100 text-gray-500"
                      }`}>
                        {stepLabels[step] ?? step}
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>

            {selectedRelease.reviewed_at && (
              <div>
                <h4 className="text-sm font-medium text-gray-700">{t("reviewDate")}</h4>
                <p className="text-sm text-gray-900">
                  {formatDateTime(selectedRelease.reviewed_at)}
                </p>
              </div>
            )}

            {selectedRelease.released_at && (
              <div>
                <h4 className="text-sm font-medium text-gray-700">{t("publishedDate")}</h4>
                <p className="text-sm text-gray-900">{formatDateTime(selectedRelease.released_at)}</p>
              </div>
            )}

            {selectedRelease.review_notes && (
              <div>
                <h4 className="text-sm font-medium text-gray-700">{t("reviewNotes")}</h4>
                <p className="text-sm text-gray-900 whitespace-pre-wrap rounded-md bg-gray-50 p-3">{selectedRelease.review_notes}</p>
              </div>
            )}

            {selectedRelease.status === "submitted" && (
              <div className="w-full">
                <label className="mb-1 block text-sm font-medium text-gray-700">
                  {t("reviewComment")}
                  <span className="ml-1 text-xs font-normal text-gray-400">{t("requiredForRejection")}</span>
                </label>
                <textarea
                  value={reviewComment}
                  onChange={(e) => setReviewComment(e.target.value)}
                  rows={3}
                  className="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                  placeholder={t("reviewCommentPlaceholder")}
                />
              </div>
            )}
          </div>
        )}
      </Modal>
    </div>
  );
}
