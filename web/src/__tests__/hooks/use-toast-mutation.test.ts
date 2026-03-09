import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import React from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { AxiosError, type AxiosResponse } from "axios";
import { useToastMutation } from "@/lib/hooks/use-toast-mutation";

// Mock sonner
const mockSuccess = vi.fn();
const mockError = vi.fn();
vi.mock("sonner", () => ({
  toast: {
    success: (...args: unknown[]) => mockSuccess(...args),
    error: (...args: unknown[]) => mockError(...args),
  },
}));

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: { retry: false },
      mutations: { retry: false },
    },
  });
  return function Wrapper({ children }: { children: React.ReactNode }) {
    return React.createElement(
      QueryClientProvider,
      { client: queryClient },
      children
    );
  };
}

describe("useToastMutation", () => {
  beforeEach(() => {
    mockSuccess.mockClear();
    mockError.mockClear();
  });

  it("shows success toast on mutation success", async () => {
    const mutationFn = vi.fn().mockResolvedValue({ id: "1" });

    const { result } = renderHook(
      () =>
        useToastMutation(mutationFn, {
          successMessage: "Created successfully",
        }),
      { wrapper: createWrapper() }
    );

    result.current.mutate(undefined as never);

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true);
    });

    expect(mockSuccess).toHaveBeenCalledWith("Created successfully");
  });

  it("shows error toast on mutation failure", async () => {
    const mutationFn = vi
      .fn()
      .mockRejectedValue(new Error("Something went wrong"));

    const { result } = renderHook(
      () =>
        useToastMutation(mutationFn, {
          successMessage: "Created",
          errorMessage: "Failed to create",
        }),
      { wrapper: createWrapper() }
    );

    result.current.mutate(undefined as never);

    await waitFor(() => {
      expect(result.current.isError).toBe(true);
    });

    expect(mockError).toHaveBeenCalledWith("Failed to create");
  });

  it("shows permission error toast on 403 response", async () => {
    const axiosError = new AxiosError("Forbidden", "ERR_BAD_REQUEST", undefined, undefined, {
      status: 403,
      data: { detail: "Forbidden" },
    } as AxiosResponse);

    const mutationFn = vi.fn().mockRejectedValue(axiosError);

    const { result } = renderHook(
      () =>
        useToastMutation(mutationFn, {
          successMessage: "Done",
          errorMessage: "Custom error",
        }),
      { wrapper: createWrapper() }
    );

    result.current.mutate(undefined as never);

    await waitFor(() => {
      expect(result.current.isError).toBe(true);
    });

    expect(mockError).toHaveBeenCalledWith(
      "You do not have permission to perform this action"
    );
  });

  it("falls back to error.message when no errorMessage provided", async () => {
    const mutationFn = vi
      .fn()
      .mockRejectedValue(new Error("Network timeout"));

    const { result } = renderHook(
      () =>
        useToastMutation(mutationFn, {
          successMessage: "Done",
        }),
      { wrapper: createWrapper() }
    );

    result.current.mutate(undefined as never);

    await waitFor(() => {
      expect(result.current.isError).toBe(true);
    });

    expect(mockError).toHaveBeenCalledWith("Network timeout");
  });

  it("calls onSuccess callback after success", async () => {
    const onSuccess = vi.fn();
    const mutationFn = vi.fn().mockResolvedValue({ id: "42" });

    const { result } = renderHook(
      () =>
        useToastMutation(mutationFn, {
          successMessage: "Done",
          onSuccess,
        }),
      { wrapper: createWrapper() }
    );

    result.current.mutate(undefined as never);

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true);
    });

    expect(onSuccess).toHaveBeenCalledWith({ id: "42" });
  });

  it("invalidates query keys on success", async () => {
    const queryClient = new QueryClient({
      defaultOptions: {
        queries: { retry: false },
        mutations: { retry: false },
      },
    });
    const invalidateSpy = vi.spyOn(queryClient, "invalidateQueries");

    const wrapper = ({ children }: { children: React.ReactNode }) =>
      React.createElement(QueryClientProvider, { client: queryClient }, children);

    const mutationFn = vi.fn().mockResolvedValue({ id: "1" });

    const { result } = renderHook(
      () =>
        useToastMutation(mutationFn, {
          successMessage: "Done",
          invalidateKeys: ["materials"],
        }),
      { wrapper }
    );

    result.current.mutate(undefined as never);

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true);
    });

    expect(invalidateSpy).toHaveBeenCalledWith({
      queryKey: ["materials"],
    });
  });
});
