"use client";

import dynamic from "next/dynamic";
import { Modal } from "@/components/ui/modal";
import { Button } from "@/components/ui/button";
import { useTranslations } from "next-intl";
import type { ProposedChanges } from "@/lib/api/ai-chat";

const ReactDiffViewer = dynamic(() => import("react-diff-viewer-continued"), {
  ssr: false,
  loading: () => (
    <div className="py-8 text-center text-sm text-gray-500">
      Loading diff viewer...
    </div>
  ),
});

interface AiChangePreviewProps {
  open: boolean;
  onClose: () => void;
  changes: ProposedChanges | null;
  onApply: () => void;
}

export function AiChangePreview({
  open,
  onClose,
  changes,
  onApply,
}: AiChangePreviewProps) {
  const t = useTranslations("developer");

  if (!changes) return null;

  const oldValue = JSON.stringify(changes.current, null, 2);
  const newValue = JSON.stringify(changes.proposed, null, 2);

  return (
    <Modal
      open={open}
      onClose={onClose}
      title={t("aiProposedChanges")}
      size="xl"
      footer={
        <>
          <Button variant="secondary" onClick={onClose}>
            {t("aiDismiss")}
          </Button>
          <Button
            onClick={() => {
              onApply();
              onClose();
            }}
          >
            {t("aiApplyChanges")}
          </Button>
        </>
      }
    >
      {changes.summary.length > 0 && (
        <div className="mb-4">
          <h4 className="mb-2 text-sm font-medium text-gray-700">Summary</h4>
          <ul className="list-inside list-disc space-y-1 text-sm text-gray-600">
            {changes.summary.map((item, i) => (
              <li key={i}>{item}</li>
            ))}
          </ul>
        </div>
      )}
      <div className="max-h-96 overflow-auto rounded border">
        <ReactDiffViewer
          oldValue={oldValue}
          newValue={newValue}
          splitView
          leftTitle="Current"
          rightTitle="Proposed"
        />
      </div>
    </Modal>
  );
}
