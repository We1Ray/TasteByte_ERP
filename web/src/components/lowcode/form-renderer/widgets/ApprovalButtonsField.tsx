"use client";

import { useState } from "react";
import { useParams } from "next/navigation";
import { useFormContext } from "react-hook-form";
import { toast } from "sonner";
import { Loader2 } from "lucide-react";
import { useTranslations } from "next-intl";
import { Button } from "@/components/ui/button";
import { Modal } from "@/components/ui/modal";
import { cn } from "@/lib/utils";
import { executorApi } from "@/lib/api/lowcode";
import type { FieldDefinition, ApprovalButtonsFieldConfig } from "@/lib/types/lowcode";

interface ApprovalButtonsFieldProps {
  field: FieldDefinition;
  value: unknown;
  onChange: (value: unknown) => void;
  error?: string;
  disabled?: boolean;
}

const colorMap: Record<string, string> = {
  green: "bg-green-600 text-white hover:bg-green-700 focus:ring-green-500",
  red: "bg-red-600 text-white hover:bg-red-700 focus:ring-red-500",
  blue: "bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500",
  yellow: "bg-yellow-500 text-white hover:bg-yellow-600 focus:ring-yellow-500",
  gray: "bg-gray-600 text-white hover:bg-gray-700 focus:ring-gray-500",
};

export function ApprovalButtonsField({
  field,
  value: _value,
  onChange,
  error,
  disabled,
}: ApprovalButtonsFieldProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const config = (field.field_config ?? {}) as ApprovalButtonsFieldConfig;
  const actions = config.actions ?? [];
  const statusField = config.statusField;

  const params = useParams();
  const recordId = params?.id as string | undefined;
  const operationCode = params?.code as string | undefined;

  // Try to read current status from form context
  let currentStatus: string | undefined;
  try {
    // useFormContext may not be available in all rendering contexts
    const formContext = useFormContext();
    if (formContext && statusField) {
      currentStatus = formContext.watch(statusField) as string | undefined;
    }
  } catch {
    // Form context not available
  }

  const [commentModalOpen, setCommentModalOpen] = useState(false);
  const [comment, setComment] = useState("");
  const [pendingAction, setPendingAction] = useState<{
    label: string;
    targetStatus: string;
  } | null>(null);
  const [submitting, setSubmitting] = useState(false);

  const isCreateMode = !recordId || recordId === "new";

  const handleAction = async (action: {
    label: string;
    targetStatus: string;
    requireComment?: boolean;
    color?: string;
  }) => {
    if (action.requireComment) {
      setPendingAction({ label: action.label, targetStatus: action.targetStatus });
      setComment("");
      setCommentModalOpen(true);
      return;
    }

    await executeTransition(action.targetStatus);
  };

  const executeTransition = async (targetStatus: string, transitionComment?: string) => {
    if (!operationCode || !recordId) return;

    setSubmitting(true);
    try {
      const updateData: Record<string, unknown> = {};
      if (statusField) {
        updateData[statusField] = targetStatus;
      }
      if (transitionComment) {
        updateData._approval_comment = transitionComment;
      }

      await executorApi.update(operationCode, recordId, updateData);

      onChange(targetStatus);
      toast.success(t("recordUpdated"));
      setCommentModalOpen(false);
    } catch (err) {
      const message = err instanceof Error ? err.message : t("formSubmitFailed");
      toast.error(message);
    } finally {
      setSubmitting(false);
    }
  };

  const handleCommentSubmit = () => {
    if (!pendingAction) return;
    executeTransition(pendingAction.targetStatus, comment);
  };

  if (isCreateMode) {
    return (
      <div className="w-full">
        {field.label && (
          <label className="mb-1 block text-sm font-medium text-gray-700">
            {field.label}
          </label>
        )}
        <p className="text-sm text-gray-400">
          {t("noFormDefinition")}
        </p>
      </div>
    );
  }

  return (
    <div className="w-full">
      {field.label && (
        <label className="mb-1 block text-sm font-medium text-gray-700">
          {field.label}
        </label>
      )}

      {currentStatus && (
        <p className="mb-2 text-xs text-gray-500">
          {tCommon("status")}: <span className="font-medium text-gray-700">{currentStatus}</span>
        </p>
      )}

      <div className="flex flex-wrap gap-2">
        {actions.map((action, index) => {
          const colorClass = colorMap[action.color ?? "blue"] ?? colorMap.blue;
          return (
            <button
              key={index}
              type="button"
              disabled={disabled || submitting}
              onClick={() => handleAction(action)}
              className={cn(
                "inline-flex items-center gap-2 rounded-md px-4 py-2 text-sm font-medium transition-colors",
                "focus:outline-none focus:ring-2 focus:ring-offset-2",
                "disabled:pointer-events-none disabled:opacity-50",
                colorClass
              )}
            >
              {submitting && <Loader2 className="h-4 w-4 animate-spin" />}
              {action.label}
            </button>
          );
        })}
        {actions.length === 0 && (
          <p className="text-sm text-gray-400">{t("noActionsDefined")}</p>
        )}
      </div>

      {field.help_text && !error && (
        <p className="mt-1 text-sm text-gray-500">{field.help_text}</p>
      )}
      {error && <p className="mt-1 text-sm text-red-600">{error}</p>}

      {/* Comment Modal */}
      <Modal
        open={commentModalOpen}
        onClose={() => setCommentModalOpen(false)}
        title={`${pendingAction?.label ?? t("actions")} - ${tCommon("comments")}`}
        size="md"
        footer={
          <>
            <Button
              variant="secondary"
              onClick={() => setCommentModalOpen(false)}
              disabled={submitting}
            >
              {tCommon("cancel")}
            </Button>
            <Button onClick={handleCommentSubmit} loading={submitting}>
              {tCommon("confirm")}
            </Button>
          </>
        }
      >
        <div className="space-y-3">
          <p className="text-sm text-gray-600">
            {t("requireComment")}: <span className="font-medium">&quot;{pendingAction?.targetStatus}&quot;</span>
          </p>
          <textarea
            value={comment}
            onChange={(e) => setComment(e.target.value)}
            rows={4}
            placeholder={tCommon("comments")}
            className="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
          />
        </div>
      </Modal>
    </div>
  );
}
