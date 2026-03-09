"use client";

import { useState } from "react";
import { useTranslations } from "next-intl";
import { Download } from "lucide-react";
import * as XLSX from "xlsx";
import { Button } from "@/components/ui/button";
import { listExecutorApi } from "@/lib/api/lowcode";
import { toast } from "sonner";
import type { ListColumn } from "@/lib/types/lowcode";

interface ExcelExportButtonProps {
  operationCode: string;
  columns: ListColumn[];
  totalRecords: number;
}

export function ExcelExportButton({
  operationCode,
  columns,
  totalRecords,
}: ExcelExportButtonProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const [exporting, setExporting] = useState(false);

  const handleExport = async () => {
    setExporting(true);
    try {
      // Fetch all data in pages
      const allRows: Record<string, unknown>[] = [];
      const pageSize = 500;
      const totalPages = Math.ceil(totalRecords / pageSize) || 1;

      for (let page = 1; page <= totalPages; page++) {
        const result = await listExecutorApi.query(operationCode, {
          page,
          page_size: pageSize,
        });
        allRows.push(...(result.items || []));
      }

      // Map rows to use column labels as headers
      const exportData = allRows.map((row) => {
        const mappedRow: Record<string, unknown> = {};
        columns.forEach((col) => {
          mappedRow[col.label] = row[col.field_key] ?? "";
        });
        return mappedRow;
      });

      // Create workbook
      const ws = XLSX.utils.json_to_sheet(exportData);

      // Set column widths based on header length
      ws["!cols"] = columns.map((col) => ({
        wch: Math.max(col.label.length + 2, 12),
      }));

      const wb = XLSX.utils.book_new();
      XLSX.utils.book_append_sheet(wb, ws, "Data");

      // Generate filename with timestamp
      const timestamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
      const filename = `${operationCode}_export_${timestamp}.xlsx`;

      XLSX.writeFile(wb, filename);
      toast.success(t("exportSuccess", { count: allRows.length, filename }));
    } catch {
      toast.error(t("exportFailed"));
    } finally {
      setExporting(false);
    }
  };

  return (
    <Button
      variant="secondary"
      size="sm"
      onClick={handleExport}
      loading={exporting}
      disabled={totalRecords === 0}
    >
      <Download className="h-4 w-4" />
      {tCommon("export")}
    </Button>
  );
}
