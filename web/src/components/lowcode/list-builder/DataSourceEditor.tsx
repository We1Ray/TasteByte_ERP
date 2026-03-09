"use client";

import { useTranslations } from "next-intl";
import { Modal } from "@/components/ui/modal";
import { Button } from "@/components/ui/button";
import { SqlEditor } from "@/components/lowcode/form-builder/SqlEditor";
import { useListBuilderStore } from "@/lib/stores/list-builder-store";

interface DataSourceEditorProps {
  open: boolean;
  onClose: () => void;
}

export function DataSourceEditor({ open, onClose }: DataSourceEditorProps) {
  const t = useTranslations("lowcode");
  const { dataSourceSql, setDataSourceSql } = useListBuilderStore();

  return (
    <Modal
      open={open}
      onClose={onClose}
      title={t("dataSourceSQL")}
      size="xl"
      footer={
        <Button onClick={onClose}>{t("done")}</Button>
      }
    >
      <div className="space-y-4">
        <p className="text-sm text-gray-500">
          {t("dataSourceSQLDesc")}
        </p>
        <SqlEditor
          value={dataSourceSql}
          onChange={setDataSourceSql}
          label={t("sqlQuery")}
        />
        <div className="rounded-md bg-amber-50 border border-amber-200 p-3">
          <p className="text-xs text-amber-700">
            <strong>Note:</strong> {t("sqlRestrictions")}
          </p>
        </div>
      </div>
    </Modal>
  );
}
