"use client";

import { useState } from "react";
import { useTranslations } from "next-intl";
import { Edit2, Check, X } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { Card } from "@/components/ui/card";
import { PageLoading } from "@/components/ui/loading";
import {
  useApiQuery,
  useApiMutation,
  useInvalidateQueries,
} from "@/lib/hooks/use-api-query";
import {
  numberRangeApi,
  type NumberRangeConfig,
} from "@/lib/api/system";

export default function NumberRangesPage() {
  const t = useTranslations("admin");
  const tCommon = useTranslations("common");
  const invalidate = useInvalidateQueries();
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editData, setEditData] = useState<Partial<NumberRangeConfig>>({});

  const { data: ranges, isLoading } = useApiQuery(
    ["system", "number-ranges"],
    () => numberRangeApi.list()
  );

  const updateMutation = useApiMutation(
    ({ id, data }: { id: string; data: Partial<NumberRangeConfig> }) =>
      numberRangeApi.update(id, data),
    {
      onSuccess: () => {
        invalidate(["system", "number-ranges"]);
        setEditingId(null);
      },
    }
  );

  if (isLoading) return <PageLoading />;

  const startEdit = (range: NumberRangeConfig) => {
    setEditingId(range.id);
    setEditData({
      description: range.description,
      padding: range.padding,
      separator: range.separator,
    });
  };

  return (
    <div>
      <PageHeader
        title={t("numberRanges")}
        description={t("numberRangesDesc")}
      />

      <Card>
        <div className="overflow-x-auto">
          <table className="min-w-full text-sm">
            <thead className="border-b bg-gray-50">
              <tr>
                <th className="px-4 py-3 text-left font-medium text-gray-600">
                  {t("prefix")}
                </th>
                <th className="px-4 py-3 text-left font-medium text-gray-600">
                  {tCommon("description")}
                </th>
                <th className="px-4 py-3 text-right font-medium text-gray-600">
                  {t("currentValue")}
                </th>
                <th className="px-4 py-3 text-right font-medium text-gray-600">
                  {t("range")}
                </th>
                <th className="px-4 py-3 text-center font-medium text-gray-600">
                  {t("padding")}
                </th>
                <th className="px-4 py-3 text-center font-medium text-gray-600">
                  {t("separator")}
                </th>
                <th className="px-4 py-3 text-center font-medium text-gray-600">
                  {t("preview")}
                </th>
                <th className="px-4 py-3 text-right font-medium text-gray-600"></th>
              </tr>
            </thead>
            <tbody className="divide-y">
              {(ranges || []).map((range: NumberRangeConfig) => {
                const isEditing = editingId === range.id;
                const preview = `${range.range_prefix}${range.separator}${String(range.current_value + 1).padStart(range.padding, "0")}`;
                return (
                  <tr key={range.id} className="hover:bg-gray-50">
                    <td className="px-4 py-3 font-mono font-bold text-blue-700">
                      {range.range_prefix}
                    </td>
                    <td className="px-4 py-3">
                      {isEditing ? (
                        <input
                          value={editData.description || ""}
                          onChange={(e) =>
                            setEditData({
                              ...editData,
                              description: e.target.value,
                            })
                          }
                          className="w-full rounded border border-gray-300 px-2 py-1 text-sm"
                        />
                      ) : (
                        range.description || "-"
                      )}
                    </td>
                    <td className="px-4 py-3 text-right font-mono">
                      {range.current_value}
                    </td>
                    <td className="px-4 py-3 text-right text-xs text-gray-400">
                      {range.start_value} - {range.end_value}
                    </td>
                    <td className="px-4 py-3 text-center">
                      {isEditing ? (
                        <input
                          type="number"
                          value={editData.padding || 3}
                          onChange={(e) =>
                            setEditData({
                              ...editData,
                              padding: Number(e.target.value),
                            })
                          }
                          className="w-16 rounded border border-gray-300 px-2 py-1 text-center text-sm"
                        />
                      ) : (
                        range.padding
                      )}
                    </td>
                    <td className="px-4 py-3 text-center font-mono">
                      {isEditing ? (
                        <input
                          value={editData.separator || "-"}
                          onChange={(e) =>
                            setEditData({
                              ...editData,
                              separator: e.target.value,
                            })
                          }
                          className="w-12 rounded border border-gray-300 px-2 py-1 text-center text-sm"
                        />
                      ) : (
                        range.separator
                      )}
                    </td>
                    <td className="px-4 py-3 text-center">
                      <span className="rounded bg-gray-100 px-2 py-1 font-mono text-xs text-gray-700">
                        {preview}
                      </span>
                    </td>
                    <td className="px-4 py-3 text-right">
                      {isEditing ? (
                        <div className="flex justify-end gap-1">
                          <button
                            onClick={() =>
                              updateMutation.mutateAsync({
                                id: range.id,
                                data: editData,
                              })
                            }
                            className="text-green-600 hover:text-green-700"
                          >
                            <Check className="h-4 w-4" />
                          </button>
                          <button
                            onClick={() => setEditingId(null)}
                            className="text-gray-400 hover:text-gray-600"
                          >
                            <X className="h-4 w-4" />
                          </button>
                        </div>
                      ) : (
                        <button
                          onClick={() => startEdit(range)}
                          className="text-gray-400 hover:text-blue-500"
                        >
                          <Edit2 className="h-4 w-4" />
                        </button>
                      )}
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      </Card>
    </div>
  );
}
