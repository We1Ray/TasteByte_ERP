"use client";

import { use } from "react";
import { useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import { ArrowLeft } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { StatusBadge } from "@/components/ui/badge";
import { PageLoading } from "@/components/ui/loading";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { ppApi } from "@/lib/api/pp";
import { DescriptionList } from "@/components/ui/description-list";
import { formatDate, formatNumber } from "@/lib/utils";

export default function BomDetailPage({ params }: { params: Promise<{ id: string }> }) {
  const { id } = use(params);
  const router = useRouter();
  const t = useTranslations("pp");
  const tc = useTranslations("common");

  const { data: bom, isLoading } = useApiQuery(
    ["pp", "boms", id],
    () => ppApi.getBom(id)
  );

  if (isLoading) {
    return <PageLoading />;
  }

  if (!bom) {
    return (
      <div className="py-12 text-center">
        <p className="text-gray-500">{t("noBomsFound")}</p>
        <Button variant="link" onClick={() => router.push("/pp/boms")} className="mt-2">
          {tc("back")}
        </Button>
      </div>
    );
  }

  return (
    <div>
      <PageHeader
        title={`${t("billOfMaterials")} ${bom.bom_number}`}
        description={`${t("material")}: ${bom.material_name} (${bom.material_number})`}
        actions={
          <Button variant="secondary" onClick={() => router.push("/pp/boms")}>
            <ArrowLeft className="h-4 w-4" />
            {tc("back")}
          </Button>
        }
      />

      <div className="grid grid-cols-1 gap-6 lg:grid-cols-3">
        <Card className="lg:col-span-2">
          <CardHeader>
            <CardTitle>{t("components")}</CardTitle>
          </CardHeader>
          <div className="-mx-6 -mb-6">
            <table className="w-full text-sm">
              <thead className="border-b border-t bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-semibold uppercase text-gray-500">Pos.</th>
                  <th className="px-6 py-3 text-left text-xs font-semibold uppercase text-gray-500">{t("material")}</th>
                  <th className="px-6 py-3 text-right text-xs font-semibold uppercase text-gray-500">{tc("quantity")}</th>
                  <th className="px-6 py-3 text-left text-xs font-semibold uppercase text-gray-500">{tc("unit")}</th>
                </tr>
              </thead>
              <tbody className="divide-y">
                {bom.items?.map((item) => (
                  <tr key={item.id} className="hover:bg-gray-50">
                    <td className="px-6 py-3 text-gray-500">{item.position}</td>
                    <td className="px-6 py-3">
                      <p className="font-medium text-gray-900">{item.component_material_name}</p>
                      <p className="text-xs text-gray-500">{item.component_material_number}</p>
                    </td>
                    <td className="px-6 py-3 text-right font-mono">{formatNumber(item.quantity, 2)}</td>
                    <td className="px-6 py-3 text-gray-500">{item.unit}</td>
                  </tr>
                ))}
                {(!bom.items || bom.items.length === 0) && (
                  <tr>
                    <td colSpan={4} className="px-6 py-8 text-center text-gray-500">
                      {t("noBomsFound")}
                    </td>
                  </tr>
                )}
              </tbody>
            </table>
          </div>
        </Card>

        <div className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle>{t("billOfMaterials")}</CardTitle>
            </CardHeader>
            <DescriptionList items={[
              { label: tc("status"), value: <StatusBadge status={bom.status} /> },
              { label: t("baseQuantity"), value: `${formatNumber(bom.quantity)} ${bom.unit}` },
              { label: t("components"), value: `${bom.items?.length || 0} ${t("items")}` },
              { label: t("validFrom"), value: formatDate(bom.valid_from) },
              { label: "Valid To", value: bom.valid_to ? formatDate(bom.valid_to) : "-", hidden: !bom.valid_to },
              { label: tc("createdAt"), value: formatDate(bom.created_at) },
            ]} />
          </Card>
        </div>
      </div>
    </div>
  );
}
