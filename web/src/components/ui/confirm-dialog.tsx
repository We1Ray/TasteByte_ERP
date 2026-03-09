"use client";

import { useTranslations } from "next-intl";
import { Modal } from "./modal";
import { Button } from "./button";

interface ConfirmDialogProps {
  open: boolean;
  onClose: () => void;
  onConfirm: () => void;
  title: string;
  message: string;
  confirmLabel?: string;
  variant?: "danger" | "warning" | "default";
  loading?: boolean;
}

export function ConfirmDialog({
  open,
  onClose,
  onConfirm,
  title,
  message,
  confirmLabel,
  variant = "default",
  loading = false,
}: ConfirmDialogProps) {
  const t = useTranslations("common");
  const tShared = useTranslations("shared");

  const variantStyles = {
    danger: "bg-red-600 hover:bg-red-700 text-white",
    warning: "bg-amber-600 hover:bg-amber-700 text-white",
    default: "bg-blue-600 hover:bg-blue-700 text-white",
  };

  const resolvedConfirmLabel = confirmLabel ?? t("confirm");

  return (
    <Modal open={open} onClose={onClose} title={title}>
      <p className="text-sm text-gray-600">{message}</p>
      <div className="mt-4 flex justify-end gap-3">
        <Button variant="secondary" onClick={onClose} disabled={loading}>
          {t("cancel")}
        </Button>
        <button
          onClick={onConfirm}
          disabled={loading}
          className={`rounded-md px-4 py-2 text-sm font-medium ${variantStyles[variant]} disabled:opacity-50`}
        >
          {loading ? tShared("processing") : resolvedConfirmLabel}
        </button>
      </div>
    </Modal>
  );
}
