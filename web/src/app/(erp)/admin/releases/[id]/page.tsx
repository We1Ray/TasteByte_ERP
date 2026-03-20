"use client";

import { useState } from "react";
import { useParams, useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import {
  ArrowLeft,
  CheckCircle,
  XCircle,
  Rocket,
  RotateCcw,
} from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { StatusBadge } from "@/components/ui/badge";
import { PageLoading } from "@/components/ui/loading";
import { ConfirmDialog } from "@/components/ui/confirm-dialog";
import { FormDefinitionDiff } from "@/components/lowcode/FormDefinitionDiff";
import {
  useApiQuery,
  useApiMutation,
  useInvalidateQueries,
} from "@/lib/hooks/use-api-query";
import { releasesApi, formApi } from "@/lib/api/lowcode";
import { formatDateTime } from "@/lib/utils";
import type { Release } from "@/lib/types/lowcode";
import { toast } from "sonner";

export default function ReleaseDetailPage() {
  const params = useParams();
  const router = useRouter();
  const releaseId = params.id as string;
  const t = useTranslations("admin");
  const tCommon = useTranslations("common");
  const invalidate = useInvalidateQueries();
  const [reviewComment, setReviewComment] = useState("");
  const [showRollbackConfirm, setShowRollbackConfirm] = useState(false);

  const { data: release, isLoading } = useApiQuery(
    ["lowcode", "releases", releaseId],
    () => releasesApi.get(releaseId),
    { enabled: !!releaseId }
  );

  // Get current form definition for diff comparison
  const { data: currentForm } = useApiQuery(
    ["lowcode", "form", release?.operation_id ?? ""],
    () => formApi.getDefinition(release!.operation_id),
    { enabled: !!release?.operation_id }
  );

  const approveMutation = useApiMutation(
    ({ id, comment }: { id: string; comment?: string }) =>
      releasesApi.approve(id, comment),
    {
      onSuccess: () => {
        invalidate(["lowcode", "releases"]);
        toast.success(t("releaseApproved"));
      },
    }
  );

  const rejectMutation = useApiMutation(
    ({ id, comment }: { id: string; comment: string }) =>
      releasesApi.reject(id, comment),
    {
      onSuccess: () => {
        invalidate(["lowcode", "releases"]);
        toast.success(t("releaseRejected"));
      },
    }
  );

  const publishMutation = useApiMutation(
    (id: string) => releasesApi.publish(id),
    {
      onSuccess: () => {
        invalidate(["lowcode", "releases"]);
        toast.success(t("releasePublished"));
      },
    }
  );

  const rollbackMutation = useApiMutation(
    (id: string) => releasesApi.rollback(id),
    {
      onSuccess: () => {
        invalidate(["lowcode", "releases"]);
        toast.success(t("releaseRolledBack"));
        setShowRollbackConfirm(false);
      },
    }
  );

  if (isLoading) return <PageLoading />;
  if (!release)
    return (
      <div className="p-8 text-center text-gray-500">
        {t("releaseNotFound")}
      </div>
    );

  const statusSteps = ["draft", "submitted", "approved", "released"] as const;
  const currentStepIndex = statusSteps.indexOf(
    release.status as (typeof statusSteps)[number]
  );

  const stepLabels: Record<string, string> = {
    draft: t("statusDraft"),
    submitted: t("statusSubmitted"),
    approved: t("statusApproved"),
    released: t("statusPublished"),
  };

  return (
    <div>
      <PageHeader
        title={t("releaseVersion", { version: release.version })}
        description={release.title}
        actions={
          <Button
            variant="secondary"
            onClick={() => router.push("/admin/releases")}
          >
            <ArrowLeft className="h-4 w-4" />
            {tCommon("back")}
          </Button>
        }
      />

      {/* Status & Workflow */}
      <Card className="mb-6">
        <CardHeader>
          <CardTitle>{t("releaseInfo")}</CardTitle>
        </CardHeader>
        <div className="space-y-4">
          <div className="grid grid-cols-2 gap-4 sm:grid-cols-4">
            <div>
              <p className="text-xs font-medium text-gray-500">
                {t("releaseNumber")}
              </p>
              <p className="mt-1 font-mono text-sm">
                {release.release_number || "-"}
              </p>
            </div>
            <div>
              <p className="text-xs font-medium text-gray-500">
                {tCommon("status")}
              </p>
              <div className="mt-1">
                <StatusBadge status={release.status} />
              </div>
            </div>
            <div>
              <p className="text-xs font-medium text-gray-500">
                {t("version")}
              </p>
              <p className="mt-1 text-sm">v{release.version}</p>
            </div>
            <div>
              <p className="text-xs font-medium text-gray-500">
                {tCommon("createdAt")}
              </p>
              <p className="mt-1 text-sm">
                {formatDateTime(release.created_at)}
              </p>
            </div>
          </div>

          {release.description && (
            <div>
              <p className="text-xs font-medium text-gray-500">
                {tCommon("description")}
              </p>
              <p className="mt-1 text-sm text-gray-700">
                {release.description}
              </p>
            </div>
          )}

          {/* Workflow Steps */}
          <div>
            <p className="mb-2 text-xs font-medium text-gray-500">
              {t("workflow")}
            </p>
            <div className="flex items-center gap-1">
              {statusSteps.map((step, i) => {
                const isCurrent = release.status === step;
                const isRejected =
                  release.status === "rejected" && step === "submitted";
                const isPast = currentStepIndex > i;
                return (
                  <div key={step} className="flex items-center gap-1">
                    {i > 0 && (
                      <div
                        className={`h-0.5 w-6 ${isPast ? "bg-blue-400" : "bg-gray-200"}`}
                      />
                    )}
                    <div
                      className={`flex items-center rounded-full px-3 py-1 text-xs font-medium ${
                        isCurrent
                          ? "bg-blue-100 text-blue-700 ring-2 ring-blue-400"
                          : isRejected
                            ? "bg-red-100 text-red-700"
                            : isPast
                              ? "bg-green-100 text-green-700"
                              : "bg-gray-100 text-gray-400"
                      }`}
                    >
                      {isPast && <CheckCircle className="mr-1 h-3 w-3" />}
                      {stepLabels[step] || step}
                    </div>
                  </div>
                );
              })}
              {release.status === "rejected" && (
                <div className="flex items-center gap-1">
                  <div className="h-0.5 w-6 bg-red-200" />
                  <div className="flex items-center rounded-full bg-red-100 px-3 py-1 text-xs font-medium text-red-700 ring-2 ring-red-400">
                    <XCircle className="mr-1 h-3 w-3" />
                    {t("statusRejected")}
                  </div>
                </div>
              )}
            </div>
          </div>

          {/* Review info */}
          {release.reviewed_at && (
            <div className="grid grid-cols-2 gap-4">
              <div>
                <p className="text-xs font-medium text-gray-500">
                  {t("reviewDate")}
                </p>
                <p className="mt-1 text-sm">
                  {formatDateTime(release.reviewed_at)}
                </p>
              </div>
              {release.released_at && (
                <div>
                  <p className="text-xs font-medium text-gray-500">
                    {t("publishedDate")}
                  </p>
                  <p className="mt-1 text-sm">
                    {formatDateTime(release.released_at)}
                  </p>
                </div>
              )}
            </div>
          )}

          {release.review_notes && (
            <div>
              <p className="text-xs font-medium text-gray-500">
                {t("reviewNotes")}
              </p>
              <p className="mt-1 whitespace-pre-wrap rounded-md bg-gray-50 p-3 text-sm text-gray-700">
                {release.review_notes}
              </p>
            </div>
          )}
        </div>
      </Card>

      {/* Form Definition Diff */}
      {!!release.form_snapshot && (
        <Card className="mb-6">
          <CardHeader>
            <CardTitle>{t("formChanges")}</CardTitle>
          </CardHeader>
          <FormDefinitionDiff
            before={currentForm as unknown as Record<string, unknown> | null}
            after={release.form_snapshot as unknown as Record<string, unknown>}
          />
        </Card>
      )}

      {/* Actions - Review */}
      {release.status === "submitted" && (
        <Card className="mb-6">
          <CardHeader>
            <CardTitle>{t("reviewActions")}</CardTitle>
          </CardHeader>
          <div className="space-y-4">
            <div>
              <label className="mb-1 block text-sm font-medium text-gray-700">
                {t("reviewComment")}
                <span className="ml-1 text-xs font-normal text-gray-400">
                  {t("requiredForRejection")}
                </span>
              </label>
              <textarea
                value={reviewComment}
                onChange={(e) => setReviewComment(e.target.value)}
                rows={3}
                className="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                placeholder={t("reviewCommentPlaceholder")}
              />
            </div>
            <div className="flex gap-3">
              <Button
                variant="danger"
                loading={rejectMutation.isPending}
                onClick={() =>
                  rejectMutation.mutateAsync({
                    id: release.id,
                    comment: reviewComment,
                  })
                }
                disabled={!reviewComment}
              >
                <XCircle className="h-4 w-4" />
                {t("reject")}
              </Button>
              <Button
                loading={approveMutation.isPending}
                onClick={() =>
                  approveMutation.mutateAsync({
                    id: release.id,
                    comment: reviewComment || undefined,
                  })
                }
              >
                <CheckCircle className="h-4 w-4" />
                {t("approve")}
              </Button>
            </div>
          </div>
        </Card>
      )}

      {/* Actions - Publish */}
      {release.status === "approved" && (
        <Card className="mb-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-gray-900">
                {t("readyToPublish")}
              </p>
              <p className="mt-0.5 text-sm text-gray-500">
                {t("publishDesc")}
              </p>
            </div>
            <Button
              loading={publishMutation.isPending}
              onClick={() => publishMutation.mutateAsync(release.id)}
            >
              <Rocket className="h-4 w-4" />
              {t("publish")}
            </Button>
          </div>
        </Card>
      )}

      {/* Actions - Rollback */}
      {release.status === "released" && (
        <Card className="mb-6 border-orange-200">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-orange-700">
                {t("rollbackTitle")}
              </p>
              <p className="mt-0.5 text-sm text-gray-500">
                {t("rollbackDesc")}
              </p>
            </div>
            <Button
              variant="danger"
              onClick={() => setShowRollbackConfirm(true)}
            >
              <RotateCcw className="h-4 w-4" />
              {t("rollbackTitle")}
            </Button>
          </div>
        </Card>
      )}

      <ConfirmDialog
        open={showRollbackConfirm}
        onClose={() => setShowRollbackConfirm(false)}
        onConfirm={() => rollbackMutation.mutateAsync(release.id)}
        title={t("rollbackTitle")}
        message={t("rollbackConfirmMsg")}
        confirmLabel={t("rollbackTitle")}
        variant="danger"
        loading={rollbackMutation.isPending}
      />
    </div>
  );
}
