import { describe, it, expect, vi } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import React from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { useApiQuery, useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";

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

describe("useApiQuery", () => {
  it("returns data on successful query", async () => {
    const queryFn = vi.fn().mockResolvedValue({ id: "1", name: "Test" });

    const { result } = renderHook(
      () => useApiQuery(["test", "item"], queryFn),
      { wrapper: createWrapper() }
    );

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true);
    });

    expect(result.current.data).toEqual({ id: "1", name: "Test" });
    expect(queryFn).toHaveBeenCalledTimes(1);
  });

  it("returns error on failed query", async () => {
    const queryFn = vi.fn().mockRejectedValue(new Error("Network error"));

    const { result } = renderHook(
      () => useApiQuery(["test", "fail"], queryFn),
      { wrapper: createWrapper() }
    );

    await waitFor(() => {
      expect(result.current.isError).toBe(true);
    });

    expect(result.current.error).toBeDefined();
  });

  it("respects enabled option", async () => {
    const queryFn = vi.fn().mockResolvedValue({ id: "1" });

    const { result } = renderHook(
      () => useApiQuery(["test", "disabled"], queryFn, { enabled: false }),
      { wrapper: createWrapper() }
    );

    // Should not fetch when disabled
    expect(result.current.fetchStatus).toBe("idle");
    expect(queryFn).not.toHaveBeenCalled();
  });
});

describe("useApiMutation", () => {
  it("calls mutation function and returns data", async () => {
    const mutationFn = vi
      .fn()
      .mockResolvedValue({ id: "new-1", name: "Created" });

    const { result } = renderHook(
      () => useApiMutation(mutationFn),
      { wrapper: createWrapper() }
    );

    result.current.mutate({ name: "Created" });

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true);
    });

    expect(result.current.data).toEqual({ id: "new-1", name: "Created" });
    expect(mutationFn.mock.calls[0][0]).toEqual({ name: "Created" });
  });

  it("handles mutation error", async () => {
    const mutationFn = vi.fn().mockRejectedValue(new Error("Create failed"));

    const { result } = renderHook(
      () => useApiMutation(mutationFn),
      { wrapper: createWrapper() }
    );

    result.current.mutate({ name: "Bad" });

    await waitFor(() => {
      expect(result.current.isError).toBe(true);
    });
  });
});

describe("useInvalidateQueries", () => {
  it("returns a function that invalidates queries", async () => {
    const queryClient = new QueryClient({
      defaultOptions: { queries: { retry: false } },
    });
    const invalidateSpy = vi.spyOn(queryClient, "invalidateQueries");

    const wrapper = ({ children }: { children: React.ReactNode }) =>
      React.createElement(QueryClientProvider, { client: queryClient }, children);

    const { result } = renderHook(() => useInvalidateQueries(), { wrapper });

    result.current(["materials"]);

    expect(invalidateSpy).toHaveBeenCalledWith({ queryKey: ["materials"] });
  });
});
