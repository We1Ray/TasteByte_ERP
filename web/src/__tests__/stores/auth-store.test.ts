import { describe, it, expect, beforeEach, vi, type Mock } from "vitest";
import { useAuthStore } from "@/lib/stores/auth-store";

// Mock authApi
vi.mock("@/lib/api/auth", () => ({
  authApi: {
    logout: vi.fn().mockResolvedValue(undefined),
    me: vi.fn().mockResolvedValue({
      id: "fresh-id",
      username: "admin",
      email: "admin@test.com",
      display_name: "Admin",
      roles: ["ADMIN"],
      permissions: ["fi:read"],
    }),
  },
}));

const mockUser = {
  id: "user-1",
  username: "testuser",
  email: "test@example.com",
  display_name: "Test User",
  roles: ["USER"],
  permissions: ["fi:read", "mm:read"],
};

describe("auth-store", () => {
  beforeEach(() => {
    localStorage.clear();
    useAuthStore.setState({
      user: null,
      token: null,
      refreshToken: null,
      isAuthenticated: false,
      isLoading: true,
    });
  });

  describe("setAuth", () => {
    it("stores user, tokens, and sets isAuthenticated", () => {
      useAuthStore.getState().setAuth(mockUser, "access-tok", "refresh-tok");

      const state = useAuthStore.getState();
      expect(state.user).toEqual(mockUser);
      expect(state.token).toBe("access-tok");
      expect(state.refreshToken).toBe("refresh-tok");
      expect(state.isAuthenticated).toBe(true);
      expect(state.isLoading).toBe(false);
    });

    it("persists tokens and user to localStorage", () => {
      useAuthStore.getState().setAuth(mockUser, "access-tok", "refresh-tok");

      expect(localStorage.getItem("access_token")).toBe("access-tok");
      expect(localStorage.getItem("refresh_token")).toBe("refresh-tok");
      expect(JSON.parse(localStorage.getItem("user")!)).toEqual(mockUser);
    });
  });

  describe("setToken", () => {
    it("updates token in state and localStorage", () => {
      useAuthStore.getState().setToken("new-token");

      expect(useAuthStore.getState().token).toBe("new-token");
      expect(localStorage.getItem("access_token")).toBe("new-token");
    });
  });

  describe("setUser", () => {
    it("updates user in state and localStorage", () => {
      useAuthStore.getState().setUser(mockUser);

      expect(useAuthStore.getState().user).toEqual(mockUser);
      expect(JSON.parse(localStorage.getItem("user")!)).toEqual(mockUser);
    });
  });

  describe("logout", () => {
    it("clears all auth state", () => {
      useAuthStore.getState().setAuth(mockUser, "access-tok", "refresh-tok");

      useAuthStore.getState().logout();

      const state = useAuthStore.getState();
      expect(state.user).toBeNull();
      expect(state.token).toBeNull();
      expect(state.refreshToken).toBeNull();
      expect(state.isAuthenticated).toBe(false);
      expect(state.isLoading).toBe(false);
    });

    it("removes tokens from localStorage", () => {
      useAuthStore.getState().setAuth(mockUser, "access-tok", "refresh-tok");

      useAuthStore.getState().logout();

      expect(localStorage.getItem("access_token")).toBeNull();
      expect(localStorage.getItem("refresh_token")).toBeNull();
      expect(localStorage.getItem("user")).toBeNull();
    });

    it("calls server logout with refresh token", async () => {
      const { authApi } = await import("@/lib/api/auth");

      useAuthStore.getState().setAuth(mockUser, "access-tok", "refresh-tok");
      useAuthStore.getState().logout();

      expect(authApi.logout).toHaveBeenCalledWith("refresh-tok");
    });

    it("does not call server logout when no refresh token", async () => {
      const { authApi } = await import("@/lib/api/auth");
      (authApi.logout as Mock).mockClear();

      // State has no refreshToken by default
      useAuthStore.getState().logout();

      expect(authApi.logout).not.toHaveBeenCalled();
    });
  });

  describe("setLoading", () => {
    it("sets isLoading flag", () => {
      useAuthStore.getState().setLoading(false);
      expect(useAuthStore.getState().isLoading).toBe(false);

      useAuthStore.getState().setLoading(true);
      expect(useAuthStore.getState().isLoading).toBe(true);
    });
  });

  describe("hydrate", () => {
    it("sets isLoading=false when no token in localStorage", () => {
      useAuthStore.getState().hydrate();

      const state = useAuthStore.getState();
      expect(state.isLoading).toBe(false);
      expect(state.isAuthenticated).toBe(false);
    });

    it("restores cached user from localStorage", () => {
      localStorage.setItem("access_token", "stored-token");
      localStorage.setItem("refresh_token", "stored-refresh");
      localStorage.setItem("user", JSON.stringify(mockUser));

      useAuthStore.getState().hydrate();

      const state = useAuthStore.getState();
      expect(state.user).toEqual(mockUser);
      expect(state.token).toBe("stored-token");
      expect(state.refreshToken).toBe("stored-refresh");
      expect(state.isAuthenticated).toBe(true);
      expect(state.isLoading).toBe(false);
    });

    it("sets authenticated without user when cachedUser is missing", () => {
      localStorage.setItem("access_token", "stored-token");
      localStorage.setItem("refresh_token", "stored-refresh");

      useAuthStore.getState().hydrate();

      const state = useAuthStore.getState();
      expect(state.user).toBeNull();
      expect(state.token).toBe("stored-token");
      expect(state.isAuthenticated).toBe(true);
    });

    it("handles corrupted cached user JSON gracefully", () => {
      localStorage.setItem("access_token", "stored-token");
      localStorage.setItem("refresh_token", "stored-refresh");
      localStorage.setItem("user", "not-valid-json");

      useAuthStore.getState().hydrate();

      const state = useAuthStore.getState();
      expect(state.user).toBeNull();
      expect(state.isAuthenticated).toBe(true);
      expect(state.isLoading).toBe(false);
    });

    it("fetches fresh user data from server after hydrating", async () => {
      const { authApi } = await import("@/lib/api/auth");

      localStorage.setItem("access_token", "stored-token");
      localStorage.setItem("refresh_token", "stored-refresh");
      localStorage.setItem("user", JSON.stringify(mockUser));

      useAuthStore.getState().hydrate();

      // Wait for async me() call to resolve
      await vi.waitFor(() => {
        expect(authApi.me).toHaveBeenCalled();
      });
    });

    it("clears auth on server me() failure", async () => {
      const { authApi } = await import("@/lib/api/auth");
      (authApi.me as Mock).mockRejectedValueOnce(new Error("Unauthorized"));

      localStorage.setItem("access_token", "expired-token");
      localStorage.setItem("refresh_token", "stored-refresh");

      useAuthStore.getState().hydrate();

      await vi.waitFor(() => {
        const state = useAuthStore.getState();
        expect(state.isAuthenticated).toBe(false);
        expect(state.user).toBeNull();
        expect(state.token).toBeNull();
      });
    });
  });
});
