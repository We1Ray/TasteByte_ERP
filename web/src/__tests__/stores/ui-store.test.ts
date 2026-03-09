import { describe, it, expect, beforeEach } from "vitest";
import { useUiStore } from "@/lib/stores/ui-store";

describe("ui-store", () => {
  beforeEach(() => {
    useUiStore.setState({
      sidebarOpen: true,
      sidebarCollapsed: false,
      currentModule: null,
    });
  });

  describe("toggleSidebar", () => {
    it("toggles sidebarOpen from true to false", () => {
      expect(useUiStore.getState().sidebarOpen).toBe(true);

      useUiStore.getState().toggleSidebar();

      expect(useUiStore.getState().sidebarOpen).toBe(false);
    });

    it("toggles sidebarOpen from false to true", () => {
      useUiStore.setState({ sidebarOpen: false });

      useUiStore.getState().toggleSidebar();

      expect(useUiStore.getState().sidebarOpen).toBe(true);
    });
  });

  describe("setSidebarOpen", () => {
    it("sets sidebarOpen to the given value", () => {
      useUiStore.getState().setSidebarOpen(false);
      expect(useUiStore.getState().sidebarOpen).toBe(false);

      useUiStore.getState().setSidebarOpen(true);
      expect(useUiStore.getState().sidebarOpen).toBe(true);
    });
  });

  describe("toggleSidebarCollapsed", () => {
    it("toggles sidebarCollapsed from false to true", () => {
      expect(useUiStore.getState().sidebarCollapsed).toBe(false);

      useUiStore.getState().toggleSidebarCollapsed();

      expect(useUiStore.getState().sidebarCollapsed).toBe(true);
    });

    it("toggles sidebarCollapsed from true to false", () => {
      useUiStore.setState({ sidebarCollapsed: true });

      useUiStore.getState().toggleSidebarCollapsed();

      expect(useUiStore.getState().sidebarCollapsed).toBe(false);
    });
  });

  describe("setCurrentModule", () => {
    it("sets the current module", () => {
      useUiStore.getState().setCurrentModule("fi");

      expect(useUiStore.getState().currentModule).toBe("fi");
    });

    it("clears the current module with null", () => {
      useUiStore.setState({ currentModule: "mm" });

      useUiStore.getState().setCurrentModule(null);

      expect(useUiStore.getState().currentModule).toBeNull();
    });
  });
});
