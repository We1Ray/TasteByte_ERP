"use client";

import { create } from "zustand";
import type { FormSection, FieldDefinition, FormSettings, LayoutConfig } from "../types/lowcode";

const MAX_HISTORY = 50;

const SESSION_KEY_PREFIX = "builder-state-";

function getSessionKey(operationId: string) {
  return `${SESSION_KEY_PREFIX}${operationId}`;
}

interface PersistedState {
  sections: FormSection[];
  formSettings: FormSettings;
  layoutConfig: LayoutConfig;
  history: FormSection[][];
  future: FormSection[][];
  isDirty: boolean;
}

function saveToSession(operationId: string | null, state: BuilderState) {
  if (!operationId) return;
  try {
    const data: PersistedState = {
      sections: state.sections,
      formSettings: state.formSettings,
      layoutConfig: state.layoutConfig,
      history: state.history,
      future: state.future,
      isDirty: state.isDirty,
    };
    sessionStorage.setItem(getSessionKey(operationId), JSON.stringify(data));
  } catch {
    // sessionStorage may be full or unavailable
  }
}

function loadFromSession(operationId: string): PersistedState | null {
  try {
    const raw = sessionStorage.getItem(getSessionKey(operationId));
    if (raw) return JSON.parse(raw);
  } catch {
    // ignore parse errors
  }
  return null;
}

function clearSession(operationId: string | null) {
  if (!operationId) return;
  try {
    sessionStorage.removeItem(getSessionKey(operationId));
  } catch {
    // ignore
  }
}

interface BuilderState {
  sections: FormSection[];
  formSettings: FormSettings;
  layoutConfig: LayoutConfig;
  selectedFieldId: string | null;
  selectedSectionId: string | null;
  isDirty: boolean;
  history: FormSection[][];
  future: FormSection[][];
  canUndo: boolean;
  canRedo: boolean;
  operationId: string | null;
  setSections: (sections: FormSection[]) => void;
  setFormSettings: (settings: FormSettings) => void;
  setLayoutConfig: (config: LayoutConfig) => void;
  updateFormSettings: (updates: Partial<FormSettings>) => void;
  updateLayoutConfig: (updates: Partial<LayoutConfig>) => void;
  addSection: (section: FormSection) => void;
  updateSection: (id: string, updates: Partial<FormSection>) => void;
  deleteSection: (id: string) => void;
  addField: (sectionId: string, field: FieldDefinition) => void;
  updateField: (id: string, updates: Partial<FieldDefinition>) => void;
  deleteField: (id: string) => void;
  moveField: (fieldId: string, fromSectionId: string, toSectionId: string, newIndex: number) => void;
  selectField: (id: string | null) => void;
  selectSection: (id: string | null) => void;
  reorderSections: (activeId: string, overId: string) => void;
  reorderFields: (sectionId: string, activeId: string, overId: string) => void;
  markClean: () => void;
  undo: () => void;
  redo: () => void;
  setOperationId: (id: string) => void;
}

function pushHistory(state: BuilderState): Pick<BuilderState, "history" | "future" | "canUndo" | "canRedo"> {
  const newHistory = [...state.history, state.sections].slice(-MAX_HISTORY);
  return {
    history: newHistory,
    future: [],
    canUndo: true,
    canRedo: false,
  };
}

export const useBuilderStore = create<BuilderState>((set) => ({
  sections: [],
  formSettings: {},
  layoutConfig: {},
  selectedFieldId: null,
  selectedSectionId: null,
  isDirty: false,
  history: [],
  future: [],
  canUndo: false,
  canRedo: false,
  operationId: null,

  setSections: (sections) => set((state) => {
    // If we have unsaved work in session, don't overwrite
    if (state.operationId) {
      const saved = loadFromSession(state.operationId);
      if (saved && saved.isDirty) {
        return {}; // Keep current state (already restored via setOperationId)
      }
    }
    return { sections, isDirty: false, history: [], future: [], canUndo: false, canRedo: false };
  }),

  setFormSettings: (settings) => set({ formSettings: settings }),
  setLayoutConfig: (config) => set({ layoutConfig: config }),
  updateFormSettings: (updates) =>
    set((state) => ({
      formSettings: { ...state.formSettings, ...updates },
      isDirty: true,
    })),
  updateLayoutConfig: (updates) =>
    set((state) => ({
      layoutConfig: { ...state.layoutConfig, ...updates },
      isDirty: true,
    })),

  addSection: (section) =>
    set((state) => ({
      ...pushHistory(state),
      sections: [...state.sections, section],
      isDirty: true,
    })),

  updateSection: (id, updates) =>
    set((state) => ({
      ...pushHistory(state),
      sections: state.sections.map((s) =>
        s.id === id ? { ...s, ...updates } : s
      ),
      isDirty: true,
    })),

  deleteSection: (id) =>
    set((state) => ({
      ...pushHistory(state),
      sections: state.sections.filter((s) => s.id !== id),
      selectedSectionId: state.selectedSectionId === id ? null : state.selectedSectionId,
      selectedFieldId: null,
      isDirty: true,
    })),

  addField: (sectionId, field) =>
    set((state) => ({
      ...pushHistory(state),
      sections: state.sections.map((s) =>
        s.id === sectionId ? { ...s, fields: [...s.fields, field] } : s
      ),
      isDirty: true,
    })),

  updateField: (id, updates) =>
    set((state) => ({
      ...pushHistory(state),
      sections: state.sections.map((s) => ({
        ...s,
        fields: s.fields.map((f) =>
          f.id === id ? { ...f, ...updates } : f
        ),
      })),
      isDirty: true,
    })),

  deleteField: (id) =>
    set((state) => ({
      ...pushHistory(state),
      sections: state.sections.map((s) => ({
        ...s,
        fields: s.fields.filter((f) => f.id !== id),
      })),
      selectedFieldId: state.selectedFieldId === id ? null : state.selectedFieldId,
      isDirty: true,
    })),

  moveField: (fieldId, fromSectionId, toSectionId, newIndex) =>
    set((state) => {
      let movedField: FieldDefinition | null = null;
      const sections = state.sections.map((s) => {
        if (s.id === fromSectionId) {
          const field = s.fields.find((f) => f.id === fieldId);
          if (field) movedField = { ...field, section_id: toSectionId };
          return { ...s, fields: s.fields.filter((f) => f.id !== fieldId) };
        }
        return s;
      });
      if (!movedField) return state;
      return {
        ...pushHistory(state),
        sections: sections.map((s) => {
          if (s.id === toSectionId) {
            const newFields = [...s.fields];
            newFields.splice(newIndex, 0, movedField!);
            return { ...s, fields: newFields };
          }
          return s;
        }),
        isDirty: true,
      };
    }),

  selectField: (id) => set({ selectedFieldId: id, selectedSectionId: null }),

  selectSection: (id) => set({ selectedSectionId: id, selectedFieldId: null }),

  reorderSections: (activeId, overId) =>
    set((state) => {
      const oldIndex = state.sections.findIndex((s) => s.id === activeId);
      const newIndex = state.sections.findIndex((s) => s.id === overId);
      if (oldIndex === -1 || newIndex === -1) return state;
      const newSections = [...state.sections];
      const [moved] = newSections.splice(oldIndex, 1);
      newSections.splice(newIndex, 0, moved);
      return {
        ...pushHistory(state),
        sections: newSections.map((s, i) => ({ ...s, sort_order: i })),
        isDirty: true,
      };
    }),

  reorderFields: (sectionId, activeId, overId) =>
    set((state) => ({
      ...pushHistory(state),
      sections: state.sections.map((s) => {
        if (s.id !== sectionId) return s;
        const oldIndex = s.fields.findIndex((f) => f.id === activeId);
        const newIndex = s.fields.findIndex((f) => f.id === overId);
        if (oldIndex === -1 || newIndex === -1) return s;
        const newFields = [...s.fields];
        const [moved] = newFields.splice(oldIndex, 1);
        newFields.splice(newIndex, 0, moved);
        return { ...s, fields: newFields.map((f, i) => ({ ...f, sort_order: i })) };
      }),
      isDirty: true,
    })),

  markClean: () => set((state) => {
    clearSession(state.operationId);
    return { isDirty: false };
  }),

  undo: () =>
    set((state) => {
      if (state.history.length === 0) return state;
      const newHistory = [...state.history];
      const previous = newHistory.pop()!;
      return {
        history: newHistory,
        future: [state.sections, ...state.future],
        sections: previous,
        isDirty: true,
        canUndo: newHistory.length > 0,
        canRedo: true,
      };
    }),

  redo: () =>
    set((state) => {
      if (state.future.length === 0) return state;
      const newFuture = [...state.future];
      const next = newFuture.shift()!;
      return {
        history: [...state.history, state.sections],
        future: newFuture,
        sections: next,
        isDirty: true,
        canUndo: true,
        canRedo: newFuture.length > 0,
      };
    }),

  setOperationId: (id) => set((state) => {
    const saved = loadFromSession(id);
    if (saved && saved.isDirty) {
      // Restore unsaved work from session
      return {
        operationId: id,
        sections: saved.sections,
        formSettings: saved.formSettings,
        layoutConfig: saved.layoutConfig,
        history: saved.history,
        future: saved.future,
        isDirty: true,
        canUndo: saved.history.length > 0,
        canRedo: saved.future.length > 0,
      };
    }
    return { operationId: id };
  }),
}));

// Auto-save to sessionStorage on changes (debounced)
let saveTimeout: ReturnType<typeof setTimeout> | null = null;
useBuilderStore.subscribe((state) => {
  if (saveTimeout) clearTimeout(saveTimeout);
  saveTimeout = setTimeout(() => {
    saveToSession(state.operationId, state);
  }, 500);
});
