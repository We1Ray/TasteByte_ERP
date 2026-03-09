"use client";

import { create } from "zustand";
import type { FormSection, FieldDefinition, FormSettings, LayoutConfig } from "../types/lowcode";

interface BuilderState {
  sections: FormSection[];
  formSettings: FormSettings;
  layoutConfig: LayoutConfig;
  selectedFieldId: string | null;
  selectedSectionId: string | null;
  isDirty: boolean;
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
}

export const useBuilderStore = create<BuilderState>((set) => ({
  sections: [],
  formSettings: {},
  layoutConfig: {},
  selectedFieldId: null,
  selectedSectionId: null,
  isDirty: false,

  setSections: (sections) => set({ sections, isDirty: false }),

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
      sections: [...state.sections, section],
      isDirty: true,
    })),

  updateSection: (id, updates) =>
    set((state) => ({
      sections: state.sections.map((s) =>
        s.id === id ? { ...s, ...updates } : s
      ),
      isDirty: true,
    })),

  deleteSection: (id) =>
    set((state) => ({
      sections: state.sections.filter((s) => s.id !== id),
      selectedSectionId: state.selectedSectionId === id ? null : state.selectedSectionId,
      selectedFieldId: null,
      isDirty: true,
    })),

  addField: (sectionId, field) =>
    set((state) => ({
      sections: state.sections.map((s) =>
        s.id === sectionId ? { ...s, fields: [...s.fields, field] } : s
      ),
      isDirty: true,
    })),

  updateField: (id, updates) =>
    set((state) => ({
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
      return { sections: newSections.map((s, i) => ({ ...s, sort_order: i })), isDirty: true };
    }),

  reorderFields: (sectionId, activeId, overId) =>
    set((state) => ({
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

  markClean: () => set({ isDirty: false }),
}));
