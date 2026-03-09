"use client";

import { useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import { PageHeader } from "@/components/layout/page-header";
import { MaterialForm } from "@/modules/mm/material-form";
import { useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { mmApi, type Material } from "@/lib/api/mm";

export default function NewMaterialPage() {
  const router = useRouter();
  const t = useTranslations("mm");
  const invalidate = useInvalidateQueries();

  const createMutation = useApiMutation(
    (data: Partial<Material>) => mmApi.createMaterial(data),
    {
      onSuccess: () => {
        invalidate(["mm", "materials"]);
        router.push("/mm/materials");
      },
    }
  );

  return (
    <div>
      <PageHeader
        title={t("createMaterial")}
        description={t("manageMaterials")}
      />
      <MaterialForm
        onSubmit={(data) => createMutation.mutate(data)}
        isLoading={createMutation.isPending}
        submitLabel={t("createMaterial")}
      />
    </div>
  );
}
