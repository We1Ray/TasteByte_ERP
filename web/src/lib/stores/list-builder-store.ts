"use client";

import { create } from "zustand";
import type { ListColumn, ListAction, ListDefinition } from "../types/lowcode";

function generateId() {
  return `${Date.now()}-${Math.random().toString(36).slice(2, 9)}`;
}

interface ListBuilderState {
  columns: ListColumn[];
  actions: ListAction[];
  dataSourceSql: string;
  settings: {
    pageSize: number;
    enableSearch: boolean;
    enableExport: boolean;
    enableImport: boolean;
  };
  selectedColumnId: string | null;
  isDirty: boolean;

  setColumns: (columns: ListColumn[]) => void;
  addColumn: (column: Omit<ListColumn, "id" | "list_id">) => void;
  updateColumn: (id: string, updates: Partial<ListColumn>) => void;
  deleteColumn: (id: string) => void;
  reorderColumns: (fromIndex: number, toIndex: number) => void;
  selectColumn: (id: string | null) => void;
  setDataSourceSql: (sql: string) => void;
  updateSettings: (settings: Partial<ListBuilderState["settings"]>) => void;
  addAction: (action: Omit<ListAction, "id" | "list_id">) => void;
  updateAction: (id: string, updates: Partial<ListAction>) => void;
  deleteAction: (id: string) => void;
  loadFromDefinition: (def: ListDefinition) => void;
  markClean: () => void;
  reset: () => void;
}

export const useListBuilderStore = create<ListBuilderState>((set) => ({
  columns: [],
  actions: [],
  dataSourceSql: "",
  settings: {
    pageSize: 20,
    enableSearch: true,
    enableExport: true,
    enableImport: false,
  },
  selectedColumnId: null,
  isDirty: false,

  setColumns: (columns) => set({ columns, isDirty: true }),

  addColumn: (column) =>
    set((state) => ({
      columns: [
        ...state.columns,
        { ...column, id: generateId(), list_id: "" } as ListColumn,
      ],
      isDirty: true,
    })),

  updateColumn: (id, updates) =>
    set((state) => ({
      columns: state.columns.map((c) =>
        c.id === id ? { ...c, ...updates } : c
      ),
      isDirty: true,
    })),

  deleteColumn: (id) =>
    set((state) => ({
      columns: state.columns.filter((c) => c.id !== id),
      selectedColumnId:
        state.selectedColumnId === id ? null : state.selectedColumnId,
      isDirty: true,
    })),

  reorderColumns: (fromIndex, toIndex) =>
    set((state) => {
      const newColumns = [...state.columns];
      const [moved] = newColumns.splice(fromIndex, 1);
      newColumns.splice(toIndex, 0, moved);
      return {
        columns: newColumns.map((c, i) => ({ ...c, sort_order: i })),
        isDirty: true,
      };
    }),

  selectColumn: (id) => set({ selectedColumnId: id }),

  setDataSourceSql: (sql) => set({ dataSourceSql: sql, isDirty: true }),

  updateSettings: (settings) =>
    set((state) => ({
      settings: { ...state.settings, ...settings },
      isDirty: true,
    })),

  addAction: (action) =>
    set((state) => ({
      actions: [
        ...state.actions,
        { ...action, id: generateId(), list_id: "" } as ListAction,
      ],
      isDirty: true,
    })),

  updateAction: (id, updates) =>
    set((state) => ({
      actions: state.actions.map((a) =>
        a.id === id ? { ...a, ...updates } : a
      ),
      isDirty: true,
    })),

  deleteAction: (id) =>
    set((state) => ({
      actions: state.actions.filter((a) => a.id !== id),
      isDirty: true,
    })),

  loadFromDefinition: (def) =>
    set({
      columns: def.columns || [],
      actions: def.actions || [],
      dataSourceSql: def.data_source_sql || "",
      settings: {
        pageSize: def.default_page_size || 20,
        enableSearch: def.enable_search ?? true,
        enableExport: def.enable_export ?? true,
        enableImport: def.enable_import ?? false,
      },
      selectedColumnId: null,
      isDirty: false,
    }),

  markClean: () => set({ isDirty: false }),

  reset: () =>
    set({
      columns: [],
      actions: [],
      dataSourceSql: "",
      settings: {
        pageSize: 20,
        enableSearch: true,
        enableExport: true,
        enableImport: false,
      },
      selectedColumnId: null,
      isDirty: false,
    }),
}));
