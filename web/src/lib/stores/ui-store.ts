"use client";

import { create } from "zustand";

interface UiState {
  sidebarOpen: boolean;
  sidebarCollapsed: boolean;
  currentModule: string | null;
  toggleSidebar: () => void;
  setSidebarOpen: (open: boolean) => void;
  toggleSidebarCollapsed: () => void;
  setCurrentModule: (module: string | null) => void;
}

export const useUiStore = create<UiState>((set) => ({
  sidebarOpen: true,
  sidebarCollapsed: false,
  currentModule: null,

  toggleSidebar: () => set((state) => ({ sidebarOpen: !state.sidebarOpen })),
  setSidebarOpen: (open) => set({ sidebarOpen: open }),
  toggleSidebarCollapsed: () =>
    set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed })),
  setCurrentModule: (module) => set({ currentModule: module }),
}));
