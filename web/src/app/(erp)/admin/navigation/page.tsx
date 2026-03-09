"use client";

import { useState } from "react";
import { useTranslations } from "next-intl";
import {
  DndContext,
  closestCenter,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
  type DragEndEvent,
} from "@dnd-kit/core";
import {
  SortableContext,
  verticalListSortingStrategy,
  sortableKeyboardCoordinates,
  useSortable,
} from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { GripVertical, Plus, Trash2, Edit2, Check, X } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Modal } from "@/components/ui/modal";
import { ConfirmDialog } from "@/components/ui/confirm-dialog";
import { PageLoading } from "@/components/ui/loading";
import { useApiQuery, useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { navigationApi } from "@/lib/api/lowcode";
import type { NavigationItem } from "@/lib/types/lowcode";

function SortableNavItem({
  item,
  editingId,
  editLabel,
  setEditLabel,
  onSaveEdit,
  onCancelEdit,
  onStartEdit,
  onDelete,
}: {
  item: NavigationItem;
  editingId: string | null;
  editLabel: string;
  setEditLabel: (v: string) => void;
  onSaveEdit: (id: string) => void;
  onCancelEdit: () => void;
  onStartEdit: (item: NavigationItem) => void;
  onDelete: (id: string) => void;
}) {
  const { attributes, listeners, setNodeRef, transform, transition, isDragging } = useSortable({ id: item.id });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
  };

  return (
    <div
      ref={setNodeRef}
      style={style}
      className="flex items-center gap-3 rounded-md border border-gray-100 bg-white px-3 py-2.5 hover:bg-gray-50"
    >
      <button
        {...attributes}
        {...listeners}
        className="cursor-grab touch-none text-gray-400 hover:text-gray-600 active:cursor-grabbing"
      >
        <GripVertical className="h-4 w-4" />
      </button>

      {editingId === item.id ? (
        <div className="flex flex-1 items-center gap-2">
          <input
            value={editLabel}
            onChange={(e) => setEditLabel(e.target.value)}
            className="flex-1 rounded-md border border-gray-300 px-2 py-1 text-sm"
            autoFocus
          />
          <button onClick={() => onSaveEdit(item.id)} className="text-green-600 hover:text-green-700">
            <Check className="h-4 w-4" />
          </button>
          <button onClick={onCancelEdit} className="text-gray-400 hover:text-gray-600">
            <X className="h-4 w-4" />
          </button>
        </div>
      ) : (
        <>
          <div className="flex-1">
            <span className="text-sm font-medium text-gray-900">{item.label}</span>
            {item.href && (
              <span className="ml-2 text-xs text-gray-400">{item.href}</span>
            )}
            {item.operation_code && (
              <span className="ml-2 text-xs text-blue-500">[{item.operation_code}]</span>
            )}
          </div>
          <button onClick={() => onStartEdit(item)} className="text-gray-400 hover:text-blue-500">
            <Edit2 className="h-4 w-4" />
          </button>
          <button
            onClick={() => onDelete(item.id)}
            className="text-gray-400 hover:text-red-500"
          >
            <Trash2 className="h-4 w-4" />
          </button>
        </>
      )}
    </div>
  );
}

export default function AdminNavigationPage() {
  const t = useTranslations("admin");
  const tCommon = useTranslations("common");
  const invalidate = useInvalidateQueries();
  const [showCreate, setShowCreate] = useState(false);
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editLabel, setEditLabel] = useState("");
  const [deleteTarget, setDeleteTarget] = useState<string | null>(null);
  const [formData, setFormData] = useState({ label: "", href: "", icon: "", operation_code: "" });

  const { data: navItems, isLoading } = useApiQuery(
    ["lowcode", "navigation"],
    () => navigationApi.list()
  );

  const createMutation = useApiMutation(
    (data: Partial<NavigationItem>) => navigationApi.create(data),
    {
      onSuccess: () => {
        invalidate(["lowcode", "navigation"]);
        setShowCreate(false);
        setFormData({ label: "", href: "", icon: "", operation_code: "" });
      },
    }
  );

  const updateMutation = useApiMutation(
    ({ id, data }: { id: string; data: Partial<NavigationItem> }) => navigationApi.update(id, data),
    { onSuccess: () => invalidate(["lowcode", "navigation"]) }
  );

  const deleteMutation = useApiMutation(
    (id: string) => navigationApi.delete(id),
    { onSuccess: () => invalidate(["lowcode", "navigation"]) }
  );

  const reorderMutation = useApiMutation(
    (items: { id: string; sort_order: number; parent_id: string | null }[]) => navigationApi.reorder(items),
    { onSuccess: () => invalidate(["lowcode", "navigation"]) }
  );

  const sensors = useSensors(
    useSensor(PointerSensor, { activationConstraint: { distance: 5 } }),
    useSensor(KeyboardSensor, { coordinateGetter: sortableKeyboardCoordinates })
  );

  if (isLoading) return <PageLoading />;

  const items = navItems || [];

  const startEdit = (item: NavigationItem) => {
    setEditingId(item.id);
    setEditLabel(item.label);
  };

  const saveEdit = (id: string) => {
    updateMutation.mutateAsync({ id, data: { label: editLabel } });
    setEditingId(null);
  };

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event;
    if (!over || active.id === over.id) return;

    const oldIndex = items.findIndex((i) => i.id === active.id);
    const newIndex = items.findIndex((i) => i.id === over.id);
    if (oldIndex === -1 || newIndex === -1) return;

    const newItems = [...items];
    const [moved] = newItems.splice(oldIndex, 1);
    newItems.splice(newIndex, 0, moved);

    reorderMutation.mutateAsync(
      newItems.map((item, idx) => ({
        id: item.id,
        sort_order: idx,
        parent_id: item.parent_id ?? null,
      }))
    );
  };

  return (
    <div>
      <PageHeader
        title={t("navigationEditor")}
        description={t("configureNavItems")}
        actions={
          <Button onClick={() => setShowCreate(true)}>
            <Plus className="h-4 w-4" />
            {t("addItem")}
          </Button>
        }
      />

      <Card>
        <div className="space-y-1">
          {items.length === 0 ? (
            <p className="py-8 text-center text-sm text-gray-500">{t("noNavItems")}</p>
          ) : (
            <DndContext sensors={sensors} collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
              <SortableContext items={items.map((i) => i.id)} strategy={verticalListSortingStrategy}>
                {items.map((item) => (
                  <SortableNavItem
                    key={item.id}
                    item={item}
                    editingId={editingId}
                    editLabel={editLabel}
                    setEditLabel={setEditLabel}
                    onSaveEdit={saveEdit}
                    onCancelEdit={() => setEditingId(null)}
                    onStartEdit={startEdit}
                    onDelete={(id) => setDeleteTarget(id)}
                  />
                ))}
              </SortableContext>
            </DndContext>
          )}
        </div>
      </Card>

      <Modal
        open={showCreate}
        onClose={() => setShowCreate(false)}
        title={t("addNavItem")}
        footer={
          <>
            <Button variant="secondary" onClick={() => setShowCreate(false)}>{tCommon("cancel")}</Button>
            <Button
              loading={createMutation.isPending}
              onClick={() =>
                createMutation.mutateAsync({
                  label: formData.label,
                  href: formData.href || undefined,
                  icon: formData.icon || undefined,
                  operation_code: formData.operation_code || undefined,
                  sort_order: items.length,
                  visible: true,
                })
              }
              disabled={!formData.label}
            >
              {t("add")}
            </Button>
          </>
        }
      >
        <div className="space-y-4">
          <Input
            label={t("label")}
            required
            value={formData.label}
            onChange={(e) => setFormData({ ...formData, label: e.target.value })}
          />
          <Input
            label={t("linkHref")}
            value={formData.href}
            onChange={(e) => setFormData({ ...formData, href: e.target.value })}
            placeholder={t("linkPlaceholder")}
          />
          <Input
            label={t("iconName")}
            value={formData.icon}
            onChange={(e) => setFormData({ ...formData, icon: e.target.value })}
            placeholder={t("iconPlaceholder")}
          />
          <Input
            label={t("operationCode")}
            value={formData.operation_code}
            onChange={(e) => setFormData({ ...formData, operation_code: e.target.value })}
            placeholder={t("operationCodePlaceholder")}
          />
        </div>
      </Modal>

      <ConfirmDialog
        open={!!deleteTarget}
        onClose={() => setDeleteTarget(null)}
        onConfirm={() => {
          if (deleteTarget) {
            deleteMutation.mutateAsync(deleteTarget);
            setDeleteTarget(null);
          }
        }}
        title={tCommon("delete")}
        message={t("deleteNavItemConfirm")}
        confirmLabel={tCommon("delete")}
        variant="danger"
        loading={deleteMutation.isPending}
      />
    </div>
  );
}
