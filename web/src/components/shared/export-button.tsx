"use client";

import { Download } from "lucide-react";
import { useTranslations } from "next-intl";
import { Button } from "@/components/ui/button";
import { DropdownMenu, DropdownMenuItem } from "@/components/ui/dropdown-menu";
import { exportToExcel, exportToCsv } from "@/lib/utils/export";

interface ExportButtonProps {
  data: unknown[];
  filename: string;
  sheetName?: string;
}

export function ExportButton({ data: rawData, filename, sheetName }: ExportButtonProps) {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const data = rawData as Record<string, any>[];
  const t = useTranslations("shared");
  const tCommon = useTranslations("common");

  return (
    <DropdownMenu
      trigger={
        <Button variant="secondary" disabled={data.length === 0}>
          <Download className="h-4 w-4" />
          {tCommon("export")}
        </Button>
      }
      align="right"
    >
      <DropdownMenuItem onClick={() => exportToExcel(data, filename, sheetName)}>
        {t("exportExcel")}
      </DropdownMenuItem>
      <DropdownMenuItem onClick={() => exportToCsv(data, filename)}>
        {t("exportCsv")}
      </DropdownMenuItem>
    </DropdownMenu>
  );
}
