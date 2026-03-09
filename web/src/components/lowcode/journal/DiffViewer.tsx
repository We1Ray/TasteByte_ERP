"use client";

import dynamic from "next/dynamic";
import { useTranslations } from "next-intl";
import { Modal } from "@/components/ui/modal";
import type { JournalEntry } from "@/lib/types/lowcode";

const ReactDiffViewer = dynamic(() => import("react-diff-viewer-continued"), {
  ssr: false,
  loading: () => <div className="py-8 text-center text-sm text-gray-500">Loading diff viewer...</div>,
});

interface DiffViewerProps {
  open: boolean;
  onClose: () => void;
  entry: JournalEntry | null;
}

export function DiffViewer({ open, onClose, entry }: DiffViewerProps) {
  const t = useTranslations("lowcode");

  if (!entry) return null;

  const oldValue = entry.old_values ? JSON.stringify(entry.old_values, null, 2) : "";
  const newValue = entry.new_values ? JSON.stringify(entry.new_values, null, 2) : "";

  return (
    <Modal open={open} onClose={onClose} title={`${t("diffTitle")} - ${entry.change_type}`} size="xl">
      <div className="max-h-96 overflow-auto">
        <ReactDiffViewer
          oldValue={oldValue}
          newValue={newValue}
          splitView
          leftTitle={t("previousVersion")}
          rightTitle={t("currentVersion")}
        />
      </div>
      <div className="mt-3 text-xs text-gray-500">
        Changed by {entry.user_name ?? entry.changed_by} on {new Date(entry.created_at).toLocaleString()}
      </div>
    </Modal>
  );
}
