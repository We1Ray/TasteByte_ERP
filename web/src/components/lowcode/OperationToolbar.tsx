"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import { ConfirmDialog } from "@/components/ui/confirm-dialog";
import { Modal } from "@/components/ui/modal";
import { Button } from "@/components/ui/button";
import apiClient from "@/lib/api/client";
import type { OperationButton } from "@/lib/types/lowcode";
import { toast } from "sonner";

interface Props {
  buttons: OperationButton[];
  operationCode: string;
}

export function OperationToolbar({ buttons, operationCode }: Props) {
  const router = useRouter();
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const [confirmBtn, setConfirmBtn] = useState<OperationButton | null>(null);
  const [modalBtn, setModalBtn] = useState<OperationButton | null>(null);
  const [loading, setLoading] = useState<string | null>(null);

  const executeAction = async (btn: OperationButton) => {
    setLoading(btn.button_key);
    try {
      switch (btn.action_type) {
        case "NAVIGATE": {
          const url = (btn.action_config.url as string) || "/";
          router.push(url);
          break;
        }
        case "API_CALL": {
          const method = ((btn.action_config.method as string) || "POST").toLowerCase();
          const url = (btn.action_config.url as string) || "";
          if (method === "get") {
            await apiClient.get(url);
          } else if (method === "put") {
            await apiClient.put(url, btn.action_config.body || {});
          } else if (method === "delete") {
            await apiClient.delete(url);
          } else {
            await apiClient.post(url, btn.action_config.body || {});
          }
          toast.success(t("actionSuccess", { action: btn.label }));
          break;
        }
        case "MODAL": {
          setModalBtn(btn);
          break;
        }
        case "CUSTOM_JS": {
          const jsCode = (btn.action_config.code as string) || "";
          if (jsCode) {
            try {
              const fn = new Function("operationCode", "toast", jsCode);
              fn(operationCode, toast);
            } catch (jsErr) {
              toast.error(t("actionFailed", { action: btn.label }));
            }
          }
          break;
        }
      }
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : t("actionFailed", { action: btn.label });
      toast.error(message);
    } finally {
      setLoading(null);
    }
  };

  const handleClick = (btn: OperationButton) => {
    if (btn.confirm_message) {
      setConfirmBtn(btn);
    } else {
      executeAction(btn);
    }
  };

  if (buttons.length === 0) return null;

  const variantMap: Record<string, "primary" | "secondary" | "danger" | "ghost"> = {
    primary: "primary",
    secondary: "secondary",
    danger: "danger",
    ghost: "ghost",
  };

  return (
    <>
      <div className="mb-4 flex flex-wrap gap-2">
        {buttons.map((btn) => (
          <Button
            key={btn.button_key}
            variant={variantMap[btn.variant] || "secondary"}
            onClick={() => handleClick(btn)}
            loading={loading === btn.button_key}
          >
            {btn.label}
          </Button>
        ))}
      </div>

      <ConfirmDialog
        open={!!confirmBtn}
        onClose={() => setConfirmBtn(null)}
        onConfirm={() => {
          if (confirmBtn) {
            executeAction(confirmBtn);
            setConfirmBtn(null);
          }
        }}
        title={confirmBtn?.label || tCommon("confirm")}
        message={confirmBtn?.confirm_message || tCommon("areYouSure")}
        confirmLabel={tCommon("confirm")}
        variant="danger"
      />

      {modalBtn && (
        <Modal
          open={true}
          onClose={() => setModalBtn(null)}
          title={modalBtn.label}
          size={(modalBtn.action_config.size as "sm" | "md" | "lg" | "xl") || "md"}
        >
          {modalBtn.action_config.content ? (
            <div
              className="prose prose-sm max-w-none"
              dangerouslySetInnerHTML={{ __html: String(modalBtn.action_config.content) }}
            />
          ) : modalBtn.action_config.url ? (
            <iframe
              src={String(modalBtn.action_config.url)}
              className="h-96 w-full rounded-md border-0"
              title={modalBtn.label}
            />
          ) : (
            <p className="text-sm text-gray-500">{t("noContentConfigured")}</p>
          )}
        </Modal>
      )}
    </>
  );
}
