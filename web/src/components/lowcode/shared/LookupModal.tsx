"use client";

import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { useTranslations } from "next-intl";
import { Modal } from "@/components/ui/modal";
import { Button } from "@/components/ui/button";
import { SearchBar } from "@/components/forms/search-bar";
import { datasourceApi } from "@/lib/api/lowcode";
import type { LookupConfig } from "@/lib/types/lowcode";

interface LookupModalProps {
  open: boolean;
  onClose: () => void;
  title: string;
  config: LookupConfig;
  onSelect: (value: string, label: string) => void;
}

export function LookupModal({ open, onClose, title, config, onSelect }: LookupModalProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const [search, setSearch] = useState("");
  const [selected, setSelected] = useState<{ value: string; label: string } | null>(null);

  const { data, isLoading } = useQuery({
    queryKey: ["lowcode", "lookup", config.operation_code, search],
    queryFn: () =>
      datasourceApi.query(
        `SELECT * FROM ${config.operation_code} WHERE 1=1 ${search ? `AND CAST(${config.label_column} AS TEXT) ILIKE '%${search}%'` : ""} LIMIT 50`
      ),
    enabled: open,
  });

  const handleConfirm = () => {
    if (selected) {
      onSelect(selected.value, selected.label);
    }
  };

  return (
    <Modal
      open={open}
      onClose={onClose}
      title={title}
      size="xl"
      footer={
        <>
          <Button variant="secondary" onClick={onClose}>{tCommon("cancel")}</Button>
          <Button onClick={handleConfirm} disabled={!selected}>{t("select")}</Button>
        </>
      }
    >
      <div className="space-y-4">
        <SearchBar placeholder={tCommon("searchPlaceholder")} onSearch={setSearch} />

        <div className="max-h-64 overflow-auto rounded-md border border-gray-200">
          <table className="w-full text-left text-sm">
            <thead className="sticky top-0 border-b bg-gray-50">
              <tr>
                {config.display_columns.map((col) => (
                  <th key={col} className="px-4 py-2 text-xs font-semibold uppercase text-gray-500">
                    {col}
                  </th>
                ))}
              </tr>
            </thead>
            <tbody className="divide-y">
              {isLoading ? (
                <tr>
                  <td colSpan={config.display_columns.length} className="px-4 py-8 text-center text-gray-500">
                    {tCommon("loading")}
                  </td>
                </tr>
              ) : (data?.rows ?? []).length === 0 ? (
                <tr>
                  <td colSpan={config.display_columns.length} className="px-4 py-8 text-center text-gray-500">
                    {t("noResults")}
                  </td>
                </tr>
              ) : (
                (data?.rows ?? []).map((row, idx) => {
                  const value = String(row[config.value_column] ?? "");
                  const label = String(row[config.label_column] ?? "");
                  const isSelected = selected?.value === value;

                  return (
                    <tr
                      key={idx}
                      className={`cursor-pointer transition-colors ${isSelected ? "bg-blue-50" : "hover:bg-gray-50"}`}
                      onClick={() => setSelected({ value, label })}
                      onDoubleClick={() => onSelect(value, label)}
                    >
                      {config.display_columns.map((col) => (
                        <td key={col} className="whitespace-nowrap px-4 py-2 text-gray-700">
                          {String(row[col] ?? "")}
                        </td>
                      ))}
                    </tr>
                  );
                })
              )}
            </tbody>
          </table>
        </div>
      </div>
    </Modal>
  );
}
