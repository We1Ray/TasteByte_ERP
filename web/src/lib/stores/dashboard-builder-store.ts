"use client";

import { create } from "zustand";
import type { DashboardWidget, DashboardDefinition } from "../types/lowcode";

function generateId() {
  return `${Date.now()}-${Math.random().toString(36).slice(2, 9)}`;
}

interface DashboardBuilderState {
  widgets: DashboardWidget[];
  gridColumns: number;
  refreshInterval: number | null;
  selectedWidgetId: string | null;
  isDirty: boolean;
  isDragging: boolean;

  addWidget: (widget: Omit<DashboardWidget, "id" | "dashboard_id">) => void;
  updateWidget: (id: string, updates: Partial<DashboardWidget>) => void;
  deleteWidget: (id: string) => void;
  selectWidget: (id: string | null) => void;
  moveWidget: (id: string, grid_x: number, grid_y: number) => void;
  resizeWidget: (id: string, grid_w: number, grid_h: number) => void;
  setDragging: (v: boolean) => void;
  setGridColumns: (columns: number) => void;
  setRefreshInterval: (interval: number | null) => void;
  loadFromDefinition: (def: DashboardDefinition) => void;
  markClean: () => void;
  reset: () => void;
}

export const useDashboardBuilderStore = create<DashboardBuilderState>((set) => ({
  widgets: [],
  gridColumns: 12,
  refreshInterval: null,
  selectedWidgetId: null,
  isDirty: false,
  isDragging: false,

  addWidget: (widget) =>
    set((state) => ({
      widgets: [
        ...state.widgets,
        { ...widget, id: generateId(), dashboard_id: "" } as DashboardWidget,
      ],
      isDirty: true,
    })),

  updateWidget: (id, updates) =>
    set((state) => ({
      widgets: state.widgets.map((w) =>
        w.id === id ? { ...w, ...updates } : w
      ),
      isDirty: true,
    })),

  deleteWidget: (id) =>
    set((state) => ({
      widgets: state.widgets.filter((w) => w.id !== id),
      selectedWidgetId:
        state.selectedWidgetId === id ? null : state.selectedWidgetId,
      isDirty: true,
    })),

  selectWidget: (id) => set({ selectedWidgetId: id }),

  moveWidget: (id, grid_x, grid_y) =>
    set((state) => ({
      widgets: state.widgets.map((w) =>
        w.id === id ? { ...w, grid_x, grid_y } : w
      ),
      isDirty: true,
    })),

  resizeWidget: (id, grid_w, grid_h) =>
    set((state) => ({
      widgets: state.widgets.map((w) =>
        w.id === id ? { ...w, grid_w, grid_h } : w
      ),
      isDirty: true,
    })),

  setDragging: (v) => set({ isDragging: v }),

  setGridColumns: (columns) => set({ gridColumns: columns, isDirty: true }),

  setRefreshInterval: (interval) =>
    set({ refreshInterval: interval, isDirty: true }),

  loadFromDefinition: (def) =>
    set({
      widgets: def.widgets || [],
      gridColumns: def.grid_columns || 12,
      refreshInterval: def.refresh_interval ?? null,
      selectedWidgetId: null,
      isDirty: false,
    }),

  markClean: () => set({ isDirty: false }),

  reset: () =>
    set({
      widgets: [],
      gridColumns: 12,
      refreshInterval: null,
      selectedWidgetId: null,
      isDirty: false,
      isDragging: false,
    }),
}));
