"use client";

import { use, useState } from "react";
import { useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import { Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { ConfirmDialog } from "@/components/ui/confirm-dialog";
import { DetailPageLayout } from "@/components/layout/detail-page-layout";
import { PrintButton } from "@/components/shared/print-button";
import { MaterialForm } from "@/modules/mm/material-form";
import { useApiQuery, useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { mmApi, type Material } from "@/lib/api/mm";

export default function MaterialDetailPage({ params }: { params: Promise<{ id: string }> }) {
  const { id } = use(params);
  const router = useRouter();
  const t = useTranslations("mm");
  const tCommon = useTranslations("common");
  const invalidate = useInvalidateQueries();
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);

  const { data: material, isLoading } = useApiQuery(
    ["mm", "materials", id],
    () => mmApi.getMaterial(id)
  );

  const updateMutation = useApiMutation(
    (data: Partial<Material>) => mmApi.updateMaterial(id, data),
    {
      onSuccess: () => {
        invalidate(["mm", "materials"]);
        router.push("/mm/materials");
      },
    }
  );

  const deleteMutation = useApiMutation(
    () => mmApi.deleteMaterial(id),
    {
      onSuccess: () => {
        invalidate(["mm", "materials"]);
        router.push("/mm/materials");
      },
    }
  );

  return (
    <DetailPageLayout
      title={material ? `${material.material_number} - ${material.name}` : t("materials")}
      subtitle={t("manageMaterials")}
      isLoading={isLoading}
      onBack={() => router.push("/mm/materials")}
      actions={
        <>
          <PrintButton />
          <Button
            variant="danger"
            onClick={() => setShowDeleteConfirm(true)}
            loading={deleteMutation.isPending}
          >
            <Trash2 className="h-4 w-4" />
            {tCommon("delete")}
          </Button>
        </>
      }
    >
      {material && (
        <MaterialForm
          defaultValues={material}
          onSubmit={(data) => updateMutation.mutate(data)}
          isLoading={updateMutation.isPending}
          submitLabel={tCommon("save")}
        />
      )}

      <ConfirmDialog
        open={showDeleteConfirm}
        onClose={() => setShowDeleteConfirm(false)}
        onConfirm={() => {
          setShowDeleteConfirm(false);
          deleteMutation.mutate(undefined);
        }}
        title={`${tCommon("delete")} ${t("materials")}`}
        message="Are you sure you want to delete this material? This action cannot be undone."
        confirmLabel={tCommon("delete")}
        variant="danger"
        loading={deleteMutation.isPending}
      />
    </DetailPageLayout>
  );
}
