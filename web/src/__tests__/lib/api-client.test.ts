import { describe, it, expect, beforeEach, vi } from "vitest";
import axios from "axios";
import type { AxiosInstance, InternalAxiosRequestConfig, AxiosResponse, AxiosError } from "axios";

// We need to test the interceptors on the actual apiClient module.
// Since the module sets up interceptors at import time, we test by importing it fresh.

// Mock the auth store before importing client
vi.mock("@/lib/stores/auth-store", () => {
  const state = {
    logout: vi.fn(),
    setToken: vi.fn(),
  };
  return {
    useAuthStore: Object.assign(
      vi.fn((selector?: (s: typeof state) => unknown) =>
        selector ? selector(state) : state
      ),
      {
        getState: vi.fn(() => state),
        setState: vi.fn(),
        subscribe: vi.fn(),
      }
    ),
  };
});

// Mock the auth API module
vi.mock("@/lib/api/auth", () => ({
  authApi: {
    refresh: vi.fn(),
    logout: vi.fn(),
    me: vi.fn(),
  },
}));

describe("apiClient", () => {
  let apiClient: AxiosInstance;
  let requestInterceptors: Array<{
    fulfilled: (config: InternalAxiosRequestConfig) => InternalAxiosRequestConfig;
  }>;

  beforeEach(async () => {
    vi.resetModules();
    localStorage.clear();

    // Re-import to get fresh module
    const clientModule = await import("@/lib/api/client");
    apiClient = clientModule.default;

    // Extract the request interceptors for direct testing
    requestInterceptors = (
      apiClient.interceptors.request as unknown as {
        handlers: Array<{
          fulfilled: (config: InternalAxiosRequestConfig) => InternalAxiosRequestConfig;
        }>;
      }
    ).handlers.filter(Boolean);
  });

  describe("request interceptor", () => {
    it("adds Authorization header when token exists in localStorage", () => {
      localStorage.setItem("access_token", "my-jwt-token");

      const config = {
        headers: new axios.AxiosHeaders(),
      } as InternalAxiosRequestConfig;

      // Apply the first request interceptor (the one that adds auth)
      const result = requestInterceptors[0].fulfilled(config);

      expect(result.headers.Authorization).toBe("Bearer my-jwt-token");
    });

    it("does not add Authorization header when no token", () => {
      const config = {
        headers: new axios.AxiosHeaders(),
      } as InternalAxiosRequestConfig;

      const result = requestInterceptors[0].fulfilled(config);

      expect(result.headers.Authorization).toBeUndefined();
    });
  });

  describe("base configuration", () => {
    it("has correct base URL", () => {
      expect(apiClient.defaults.baseURL).toBe(
        "http://localhost:8000/api/v1"
      );
    });

    it("has JSON content type header", () => {
      expect(apiClient.defaults.headers["Content-Type"]).toBe(
        "application/json"
      );
    });
  });

  describe("response interceptor - 401 handling", () => {
    it("attempts token refresh on 401 response", async () => {
      const { authApi } = await import("@/lib/api/auth");
      const { useAuthStore } = await import("@/lib/stores/auth-store");

      localStorage.setItem("refresh_token", "old-refresh");
      localStorage.setItem("access_token", "expired-token");

      // Mock refresh to return new tokens
      (authApi.refresh as ReturnType<typeof vi.fn>).mockResolvedValue({
        access_token: "new-access",
        refresh_token: "new-refresh",
        token_type: "bearer",
        expires_in: 900,
      });

      // Create a mock 401 error
      const error401 = {
        response: { status: 401 },
        config: {
          url: "/mm/materials",
          headers: new axios.AxiosHeaders(),
          _retry: false,
        },
      } as unknown as AxiosError;

      // Extract the response error interceptor
      const responseInterceptors = (
        apiClient.interceptors.response as unknown as {
          handlers: Array<{
            rejected: (error: AxiosError) => Promise<unknown>;
          }>;
        }
      ).handlers.filter(Boolean);

      // The interceptor will try to call apiClient again after refresh,
      // which will fail since we don't have a server. But we can verify
      // that refresh was called.
      try {
        await responseInterceptors[0].rejected(error401);
      } catch {
        // Expected - the retry will fail since there's no server
      }

      expect(authApi.refresh).toHaveBeenCalledWith("old-refresh");
      expect(useAuthStore.getState().setToken).toHaveBeenCalledWith("new-access");
    });

    it("redirects to login when no refresh token available", async () => {
      const { useAuthStore } = await import("@/lib/stores/auth-store");

      // No refresh token in localStorage
      localStorage.removeItem("refresh_token");

      const error401 = {
        response: { status: 401 },
        config: {
          url: "/mm/materials",
          headers: new axios.AxiosHeaders(),
          _retry: false,
        },
      } as unknown as AxiosError;

      const responseInterceptors = (
        apiClient.interceptors.response as unknown as {
          handlers: Array<{
            rejected: (error: AxiosError) => Promise<unknown>;
          }>;
        }
      ).handlers.filter(Boolean);

      await expect(
        responseInterceptors[0].rejected(error401)
      ).rejects.toBeDefined();

      expect(useAuthStore.getState().logout).toHaveBeenCalled();
    });

    it("does not retry auth endpoints (login/refresh/logout)", async () => {
      const { authApi } = await import("@/lib/api/auth");
      (authApi.refresh as ReturnType<typeof vi.fn>).mockClear();

      localStorage.setItem("refresh_token", "some-token");

      const authUrls = ["/auth/refresh", "/auth/login", "/auth/logout"];

      const responseInterceptors = (
        apiClient.interceptors.response as unknown as {
          handlers: Array<{
            rejected: (error: AxiosError) => Promise<unknown>;
          }>;
        }
      ).handlers.filter(Boolean);

      for (const url of authUrls) {
        const error = {
          response: { status: 401 },
          config: {
            url,
            headers: new axios.AxiosHeaders(),
            _retry: false,
          },
        } as unknown as AxiosError;

        await expect(
          responseInterceptors[0].rejected(error)
        ).rejects.toBeDefined();
      }

      // refresh should never be called for auth endpoints
      expect(authApi.refresh).not.toHaveBeenCalled();
    });

    it("passes through non-401 errors", async () => {
      const error500 = {
        response: { status: 500 },
        config: {
          url: "/mm/materials",
          headers: new axios.AxiosHeaders(),
        },
      } as unknown as AxiosError;

      const responseInterceptors = (
        apiClient.interceptors.response as unknown as {
          handlers: Array<{
            rejected: (error: AxiosError) => Promise<unknown>;
          }>;
        }
      ).handlers.filter(Boolean);

      await expect(
        responseInterceptors[0].rejected(error500)
      ).rejects.toBeDefined();
    });
  });

  describe("exports", () => {
    it("exports PaginatedResponse type", async () => {
      const mod = await import("@/lib/api/client");
      // Type check - if this compiles the interface exists
      const _test: import("@/lib/api/client").PaginatedResponse<string> = {
        items: ["a"],
        total: 1,
        page: 1,
        page_size: 20,
        total_pages: 1,
      };
      expect(_test.items).toEqual(["a"]);
    });
  });
});
