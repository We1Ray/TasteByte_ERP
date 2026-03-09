"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { ConfirmDialog } from "@/components/ui/confirm-dialog";
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
  const [confirmBtn, setConfirmBtn] = useState<OperationButton | null>(null);
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
          toast.success(`${btn.label} executed successfully`);
          break;
        }
        case "MODAL":
        case "CUSTOM_JS":
          toast.info(`${btn.action_type} actions coming soon`);
          break;
      }
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : "Action failed";
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
        title={confirmBtn?.label || "Confirm"}
        message={confirmBtn?.confirm_message || "Are you sure?"}
        confirmLabel="Confirm"
        variant="danger"
      />
    </>
  );
}
