"use client";

import { useState, useMemo } from "react";
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
import { GripVertical, Plus, Trash2, Edit2, Check, X, ChevronRight } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { Modal } from "@/components/ui/modal";
import { ConfirmDialog } from "@/components/ui/confirm-dialog";
import { PageLoading } from "@/components/ui/loading";
import { useApiQuery, useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { navigationApi } from "@/lib/api/lowcode";
import type { NavigationItem } from "@/lib/types/lowcode";

// ── Tree utilities ────────────────────────────────────────────────────

interface TreeNavItem extends NavigationItem {
  children: TreeNavItem[];
  depth: number;
}

function buildTree(items: NavigationItem[]): TreeNavItem[] {
  const map = new Map<string, TreeNavItem>();
  const roots: TreeNavItem[] = [];

  // Initialize all items
  for (const item of items) {
    map.set(item.id, { ...item, children: [], depth: 0 });
  }

  // Build parent-child relationships
  for (const item of items) {
    const treeItem = map.get(item.id)!;
    if (item.parent_id && map.has(item.parent_id)) {
      const parent = map.get(item.parent_id)!;
      treeItem.depth = parent.depth + 1;
      parent.children.push(treeItem);
    } else {
      roots.push(treeItem);
    }
  }

  // Sort children by sort_order
  function sortChildren(nodes: TreeNavItem[]) {
    nodes.sort((a, b) => a.sort_order - b.sort_order);
    for (const node of nodes) {
      sortChildren(node.children);
    }
  }
  sortChildren(roots);

  return roots;
}

function flattenTree(roots: TreeNavItem[]): TreeNavItem[] {
  const result: TreeNavItem[] = [];
  function walk(nodes: TreeNavItem[], depth: number) {
    for (const node of nodes) {
      node.depth = depth;
      result.push(node);
      walk(node.children, depth + 1);
    }
  }
  walk(roots, 0);
  return result;
}

// ── Sortable Item Component ───────────────────────────────────────────

function SortableNavItem({
  item,
  depth,
  editingId,
  editLabel,
  setEditLabel,
  onSaveEdit,
  onCancelEdit,
  onStartEdit,
  onDelete,
}: {
  item: NavigationItem;
  depth: number;
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
      style={{ ...style, paddingLeft: `${depth * 24 + 12}px` }}
      className="flex items-center gap-3 rounded-md border border-gray-100 bg-white py-2.5 pr-3 hover:bg-gray-50"
    >
      <button
        {...attributes}
        {...listeners}
        className="cursor-grab touch-none text-gray-400 hover:text-gray-600 active:cursor-grabbing"
      >
        <GripVertical className="h-4 w-4" />
      </button>

      {depth > 0 && (
        <ChevronRight className="h-3 w-3 text-gray-300" />
      )}

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

// ── Page Component ────────────────────────────────────────────────────

export default function AdminNavigationPage() {
  const t = useTranslations("admin");
  const tCommon = useTranslations("common");
  const invalidate = useInvalidateQueries();
  const [showCreate, setShowCreate] = useState(false);
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editLabel, setEditLabel] = useState("");
  const [deleteTarget, setDeleteTarget] = useState<string | null>(null);
  const [formData, setFormData] = useState({
    label: "",
    href: "",
    icon: "",
    operation_code: "",
    parent_id: "",
  });

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
        setFormData({ label: "", href: "", icon: "", operation_code: "", parent_id: "" });
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

  const items = navItems || [];

  // Build tree and flatten for rendering
  const flatItems = useMemo(() => {
    return flattenTree(buildTree(items));
  }, [items]);

  // Parent options for the create form (exclude self and children when editing)
  const parentOptions = useMemo(() => {
    return [
      { value: "", label: t("noParent") },
      ...items.map((item) => ({ value: item.id, label: item.label })),
    ];
  }, [items, t]);

  if (isLoading) return <PageLoading />;

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

    const oldIndex = flatItems.findIndex((i) => i.id === active.id);
    const newIndex = flatItems.findIndex((i) => i.id === over.id);
    if (oldIndex === -1 || newIndex === -1) return;

    const reordered = [...flatItems];
    const [moved] = reordered.splice(oldIndex, 1);
    reordered.splice(newIndex, 0, moved);

    reorderMutation.mutateAsync(
      reordered.map((item, idx) => ({
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
          {flatItems.length === 0 ? (
            <p className="py-8 text-center text-sm text-gray-500">{t("noNavItems")}</p>
          ) : (
            <DndContext sensors={sensors} collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
              <SortableContext items={flatItems.map((i) => i.id)} strategy={verticalListSortingStrategy}>
                {flatItems.map((item) => (
                  <SortableNavItem
                    key={item.id}
                    item={item}
                    depth={item.depth}
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
                  parent_id: formData.parent_id || null,
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
          <Select
            label={t("parentItem")}
            value={formData.parent_id}
            onChange={(e) => setFormData({ ...formData, parent_id: e.target.value })}
            options={parentOptions}
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
