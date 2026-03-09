"use client";

import { useTranslations } from "next-intl";
import { Plus, Trash2, GripVertical } from "lucide-react";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { Modal } from "@/components/ui/modal";
import { useListBuilderStore } from "@/lib/stores/list-builder-store";
import type { ListAction } from "@/lib/types/lowcode";

interface ActionEditorProps {
  open: boolean;
  onClose: () => void;
}

function ActionItem({
  action,
  onUpdate,
  onDelete,
}: {
  action: ListAction;
  onUpdate: (updates: Partial<ListAction>) => void;
  onDelete: () => void;
}) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  return (
    <div className="rounded-lg border border-gray-200 bg-white p-4">
      <div className="mb-3 flex items-center justify-between">
        <div className="flex items-center gap-2">
          <GripVertical className="h-4 w-4 text-gray-300" />
          <span className="text-sm font-medium text-gray-700">
            {action.label || t("untitledAction")}
          </span>
        </div>
        <button
          onClick={onDelete}
          className="text-gray-400 hover:text-red-500"
        >
          <Trash2 className="h-4 w-4" />
        </button>
      </div>

      <div className="grid grid-cols-2 gap-3">
        <Input
          label={t("actionKey")}
          value={action.action_key}
          onChange={(e) => onUpdate({ action_key: e.target.value })}
          placeholder={t("actionKeyExample")}
        />
        <Input
          label={t("label")}
          value={action.label}
          onChange={(e) => onUpdate({ label: e.target.value })}
          placeholder={t("actionLabelExample")}
        />
        <Input
          label={t("icon")}
          value={action.icon ?? ""}
          onChange={(e) => onUpdate({ icon: e.target.value || undefined })}
          placeholder={t("iconExample")}
        />
        <Select
          label={t("actionType")}
          value={action.action_type}
          onChange={(e) =>
            onUpdate({
              action_type: e.target.value as ListAction["action_type"],
            })
          }
          options={[
            { value: "navigate", label: t("actionNavigate") },
            { value: "modal", label: t("actionModal") },
            { value: "api_call", label: t("actionApiCall") },
            { value: "delete", label: tCommon("delete") },
          ]}
        />
      </div>

      {action.action_type === "navigate" && (
        <div className="mt-3">
          <Input
            label={t("targetUrl")}
            value={action.target_url ?? ""}
            onChange={(e) => onUpdate({ target_url: e.target.value })}
            placeholder={t("targetUrlPlaceholder")}
            helperText={t("targetUrlHint")}
          />
        </div>
      )}

      {action.action_type === "delete" && (
        <div className="mt-3">
          <Input
            label={t("confirmMessage")}
            value={action.confirm_message ?? ""}
            onChange={(e) => onUpdate({ confirm_message: e.target.value })}
            placeholder={t("deleteConfirmExample")}
          />
        </div>
      )}
    </div>
  );
}

export function ActionEditor({ open, onClose }: ActionEditorProps) {
  const t = useTranslations("lowcode");
  const { actions, addAction, updateAction, deleteAction } =
    useListBuilderStore();

  const handleAddAction = () => {
    addAction({
      action_key: `action_${Date.now()}`,
      label: "New Action",
      action_type: "navigate",
      sort_order: actions.length,
    });
  };

  return (
    <Modal
      open={open}
      onClose={onClose}
      title={t("rowActions")}
      size="xl"
      footer={<Button onClick={onClose}>{t("done")}</Button>}
    >
      <div className="space-y-4">
        <p className="text-sm text-gray-500">
          {t("rowActionsDesc")}
        </p>

        <div className="space-y-3">
          {actions.map((action) => (
            <ActionItem
              key={action.id}
              action={action}
              onUpdate={(updates) => updateAction(action.id, updates)}
              onDelete={() => deleteAction(action.id)}
            />
          ))}
        </div>

        {actions.length === 0 && (
          <div className="rounded-md border-2 border-dashed border-gray-200 px-4 py-8 text-center">
            <p className="text-sm text-gray-400">{t("noActionsConfigured")}</p>
            <p className="mt-1 text-xs text-gray-400">
              {t("addActionsHint")}
            </p>
          </div>
        )}

        <Button
          variant="secondary"
          className="w-full"
          onClick={handleAddAction}
        >
          <Plus className="h-4 w-4" />
          {t("addAction")}
        </Button>
      </div>
    </Modal>
  );
}
