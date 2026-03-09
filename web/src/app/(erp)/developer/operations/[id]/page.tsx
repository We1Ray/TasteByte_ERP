"use client";

import { useState, useEffect } from "react";
import { useParams, useRouter } from "next/navigation";
import { Save, Eye, EyeOff, ArrowLeft, Settings, Rocket, Layout } from "lucide-react";
import { DndContext, closestCenter } from "@dnd-kit/core";
import { Button } from "@/components/ui/button";
import { Modal } from "@/components/ui/modal";
import { PageLoading } from "@/components/ui/loading";
import { FieldPalette } from "@/components/lowcode/form-builder/FieldPalette";
import { BuilderCanvas, BuilderDragOverlay, useBuilderDnd } from "@/components/lowcode/form-builder/BuilderCanvas";
import { PropertyPanel } from "@/components/lowcode/form-builder/PropertyPanel";
import { PreviewPanel } from "@/components/lowcode/form-builder/PreviewPanel";
import { OperationSettingsModal } from "@/components/lowcode/form-builder/OperationSettingsModal";
import { ReleaseSubmitModal } from "@/components/lowcode/form-builder/ReleaseSubmitModal";
import { WorkflowStatusIndicator } from "@/components/lowcode/form-builder/WorkflowStatusIndicator";
import { WizardConfigPanel } from "@/components/lowcode/form-builder/WizardConfigPanel";
import { TabGroupEditor } from "@/components/lowcode/form-builder/TabGroupEditor";
import { ListBuilder } from "@/components/lowcode/list-builder/ListBuilder";
import { DashboardBuilder } from "@/components/lowcode/dashboard-builder/DashboardBuilder";
import { useApiQuery, useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { useBuilderStore } from "@/lib/stores/builder-store";
import { operationsApi, formApi } from "@/lib/api/lowcode";
import { AiChatToggle } from "@/components/lowcode/ai-chat/AiChatToggle";
import { AiChatPanel } from "@/components/lowcode/ai-chat/AiChatPanel";
import { useAiChatStore } from "@/lib/stores/ai-chat-store";
import { aiChatApi, type ProposedChanges } from "@/lib/api/ai-chat";
import { toast } from "sonner";
import type { WizardConfig, FormSection } from "@/lib/types/lowcode";

export default function OperationBuilderPage() {
  const params = useParams();
  const id = params.id as string;

  const { data: operation, isLoading: opLoading } = useApiQuery(
    ["lowcode", "operations", id],
    () => operationsApi.get(id),
    { enabled: id !== "new" }
  );

  // Branch on operation_type: LIST and DASHBOARD have their own builders
  const operationType = operation?.operation_type?.toLowerCase();

  if (opLoading && id !== "new") {
    return <PageLoading />;
  }

  if (operationType === "list") {
    return <ListBuilder />;
  }

  if (operationType === "dashboard") {
    return <DashboardBuilder />;
  }

  // Default: FORM builder
  return <FormBuilderContent id={id} operation={operation} />;
}

function FormBuilderContent({ id, operation }: { id: string; operation?: { name?: string; code?: string; operation_type?: string } }) {
  const router = useRouter();
  const invalidate = useInvalidateQueries();
  const {
    sections, setSections,
    formSettings, setFormSettings,
    layoutConfig, setLayoutConfig,
    updateFormSettings, updateLayoutConfig,
    isDirty, markClean,
  } = useBuilderStore();
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [releaseOpen, setReleaseOpen] = useState(false);
  const [layoutOpen, setLayoutOpen] = useState(false);
  const [showPreview, setShowPreview] = useState(false);

  // AI assistant
  const { data: aiStatus } = useApiQuery(
    ["ai", "status"],
    () => aiChatApi.getStatus(),
    { staleTime: 60000, retry: false }
  );

  useEffect(() => {
    return () => {
      useAiChatStore.getState().clearChat();
    };
  }, []);

  const handleApplyAiChanges = (changes: ProposedChanges) => {
    if (changes.change_type === "form" && changes.proposed) {
      const proposed = changes.proposed as { sections?: FormSection[] };
      if (proposed.sections) {
        setSections(proposed.sections);
      }
      toast.success("AI changes applied. Review and save when ready.");
    }
    useAiChatStore.getState().clearPendingChanges();
  };

  // Shared DnD context for palette + canvas
  const { sensors, handleDragStart, handleDragEnd, activeDragItem } = useBuilderDnd();

  const { data: formDef, isLoading: formLoading } = useApiQuery(
    ["lowcode", "form", id],
    () => formApi.getDefinition(id),
    { enabled: id !== "new" }
  );

  useEffect(() => {
    if (formDef?.sections) {
      setSections(formDef.sections);
    }
    if (formDef?.form_settings) {
      setFormSettings(formDef.form_settings);
    }
    if (formDef?.layout_config) {
      setLayoutConfig(formDef.layout_config);
    }
  }, [formDef, setSections, setFormSettings, setLayoutConfig]);

  // Warn before leaving with unsaved changes
  useEffect(() => {
    const handler = (e: BeforeUnloadEvent) => {
      if (isDirty) {
        e.preventDefault();
      }
    };
    window.addEventListener("beforeunload", handler);
    return () => window.removeEventListener("beforeunload", handler);
  }, [isDirty]);

  const saveMutation = useApiMutation(
    () =>
      formApi.saveDefinition(id, {
        sections,
        form_settings: formSettings,
        layout_config: layoutConfig,
      }),
    {
      onSuccess: () => {
        invalidate(["lowcode", "form", id]);
        markClean();
      },
    }
  );

  // Keyboard shortcut: Ctrl+S / Cmd+S to save
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === "s") {
        e.preventDefault();
        if (isDirty && !saveMutation.isPending) {
          saveMutation.mutateAsync(undefined);
        }
      }
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [isDirty, saveMutation]);

  if (id !== "new" && formLoading) {
    return <PageLoading />;
  }

  const hasWizard = formSettings.wizard?.steps && formSettings.wizard.steps.length > 0;
  const hasTabs = layoutConfig.tabGroups && layoutConfig.tabGroups.length > 0;

  return (
    <div className="-m-6 flex h-[calc(100vh-3.5rem)] flex-col">
      <div className="flex items-center justify-between border-b bg-white px-6 py-3">
        <div className="flex items-center gap-3">
          <Button variant="ghost" size="icon" onClick={() => router.push("/developer/operations")}>
            <ArrowLeft className="h-4 w-4" />
          </Button>
          <div>
            <div className="flex items-center gap-2">
              <h1 className="text-lg font-semibold text-gray-900">
                {operation?.name || "New Operation"}
              </h1>
              {id !== "new" && <WorkflowStatusIndicator operationId={id} />}
            </div>
            <p className="text-xs text-gray-500">
              {operation?.code || "Form Builder"}
              {isDirty && " (unsaved changes)"}
              {hasWizard && " | Wizard Mode"}
              {hasTabs && " | Tab Layout"}
            </p>
          </div>
        </div>
        <div className="flex items-center gap-2">
          {id !== "new" && (
            <>
              <Button
                variant="ghost"
                size="icon"
                onClick={() => setLayoutOpen(true)}
                title="Layout Mode (Wizard / Tabs)"
              >
                <Layout className="h-4 w-4" />
              </Button>
              <Button
                variant="ghost"
                size="icon"
                onClick={() => setSettingsOpen(true)}
                title="Operation Settings"
              >
                <Settings className="h-4 w-4" />
              </Button>
            </>
          )}
          <Button
            variant={showPreview ? "primary" : "secondary"}
            onClick={() => setShowPreview(!showPreview)}
            title={showPreview ? "Hide live preview" : "Show live preview"}
          >
            {showPreview ? <EyeOff className="h-4 w-4" /> : <Eye className="h-4 w-4" />}
            {showPreview ? "Editor" : "Preview"}
          </Button>
          {id !== "new" && (
            <Button
              variant="secondary"
              onClick={() => setReleaseOpen(true)}
              disabled={isDirty}
              title={isDirty ? "Save changes before releasing" : "Create a release"}
            >
              <Rocket className="h-4 w-4" />
              Release
            </Button>
          )}
          <Button
            onClick={() => saveMutation.mutateAsync(undefined)}
            loading={saveMutation.isPending}
            disabled={!isDirty}
          >
            <Save className="h-4 w-4" />
            Save
          </Button>
        </div>
      </div>

      <DndContext
        sensors={sensors}
        collisionDetection={closestCenter}
        onDragStart={handleDragStart}
        onDragEnd={handleDragEnd}
      >
        <div className="flex flex-1 overflow-hidden">
          <FieldPalette />
          <BuilderCanvas />
          {showPreview ? (
            <div className="w-[420px] shrink-0 overflow-y-auto border-l border-gray-200 bg-white">
              <PreviewPanel operationCode={operation?.code || ""} />
            </div>
          ) : (
            <PropertyPanel />
          )}
        </div>
        <BuilderDragOverlay activeDragItem={activeDragItem} />
      </DndContext>

      {aiStatus?.enabled && (
        <>
          <AiChatToggle />
          <AiChatPanel
            operationId={id}
            contextType="form"
            onApplyChanges={handleApplyAiChanges}
          />
        </>
      )}

      {id !== "new" && (
        <>
          <OperationSettingsModal
            open={settingsOpen}
            onClose={() => setSettingsOpen(false)}
            operationId={id}
          />
          <ReleaseSubmitModal
            open={releaseOpen}
            onClose={() => setReleaseOpen(false)}
            operationId={id}
          />
          <LayoutConfigModal
            open={layoutOpen}
            onClose={() => setLayoutOpen(false)}
            sections={sections}
            wizardConfig={formSettings.wizard}
            tabGroups={layoutConfig.tabGroups || []}
            onWizardChange={(wizard) => updateFormSettings({ wizard })}
            onTabGroupsChange={(tabGroups) => updateLayoutConfig({ tabGroups })}
          />
        </>
      )}
    </div>
  );
}

// ── Layout Config Modal ─────────────────────────────────────────────────────

function LayoutConfigModal({
  open,
  onClose,
  sections,
  wizardConfig,
  tabGroups,
  onWizardChange,
  onTabGroupsChange,
}: {
  open: boolean;
  onClose: () => void;
  sections: { id: string; title: string; sort_order: number; fields: unknown[] }[];
  wizardConfig?: WizardConfig;
  tabGroups: { id: string; label: string; icon?: string; sort_order: number }[];
  onWizardChange: (config: WizardConfig | undefined) => void;
  onTabGroupsChange: (groups: { id: string; label: string; icon?: string; sort_order: number }[]) => void;
}) {
  const [activeTab, setActiveTab] = useState<"wizard" | "tabs">("wizard");

  return (
    <Modal
      open={open}
      onClose={onClose}
      title="Form Layout Configuration"
      size="xl"
    >
      <div className="space-y-4">
        {/* Tab selector */}
        <div className="flex items-center gap-1 border-b border-gray-200">
          <button
            type="button"
            onClick={() => setActiveTab("wizard")}
            className={`border-b-2 px-4 py-2 text-sm font-medium transition-colors -mb-px ${
              activeTab === "wizard"
                ? "border-blue-600 text-blue-600"
                : "border-transparent text-gray-500 hover:text-gray-700"
            }`}
          >
            Wizard / Stepper
          </button>
          <button
            type="button"
            onClick={() => setActiveTab("tabs")}
            className={`border-b-2 px-4 py-2 text-sm font-medium transition-colors -mb-px ${
              activeTab === "tabs"
                ? "border-blue-600 text-blue-600"
                : "border-transparent text-gray-500 hover:text-gray-700"
            }`}
          >
            Tab Groups
          </button>
        </div>

        {/* Wizard config */}
        {activeTab === "wizard" && (
          <div>
            <p className="mb-3 text-xs text-gray-500">
              Configure wizard steps to guide users through the form in multiple steps.
              Each step can contain one or more sections.
            </p>
            <WizardConfigPanel
              wizardConfig={wizardConfig || { steps: [] }}
              onChange={onWizardChange}
              sections={sections as Parameters<typeof WizardConfigPanel>[0]["sections"]}
            />
          </div>
        )}

        {/* Tab groups config */}
        {activeTab === "tabs" && (
          <div>
            <p className="mb-3 text-xs text-gray-500">
              Create tab groups to organize form sections into tabs. Sections can be
              assigned to tabs using the section&apos;s tab_group_id property.
            </p>
            <TabGroupEditor
              tabGroups={tabGroups}
              onUpdate={onTabGroupsChange}
            />
          </div>
        )}
      </div>
    </Modal>
  );
}
