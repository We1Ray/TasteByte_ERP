import { describe, it, expect } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { usePagination } from "@/lib/hooks/use-pagination";

describe("usePagination", () => {
  it("starts with default page=1 and pageSize=20", () => {
    const { result } = renderHook(() => usePagination());

    expect(result.current.page).toBe(1);
    expect(result.current.pageSize).toBe(20);
    expect(result.current.paginationParams).toEqual({ page: 1, page_size: 20 });
  });

  it("accepts custom initial values", () => {
    const { result } = renderHook(() =>
      usePagination({ initialPage: 3, initialPageSize: 50 })
    );

    expect(result.current.page).toBe(3);
    expect(result.current.pageSize).toBe(50);
  });

  describe("nextPage", () => {
    it("increments the page by 1", () => {
      const { result } = renderHook(() => usePagination());

      act(() => result.current.nextPage());

      expect(result.current.page).toBe(2);
    });

    it("increments multiple times", () => {
      const { result } = renderHook(() => usePagination());

      act(() => result.current.nextPage());
      act(() => result.current.nextPage());
      act(() => result.current.nextPage());

      expect(result.current.page).toBe(4);
    });
  });

  describe("prevPage", () => {
    it("decrements the page by 1", () => {
      const { result } = renderHook(() =>
        usePagination({ initialPage: 5 })
      );

      act(() => result.current.prevPage());

      expect(result.current.page).toBe(4);
    });

    it("does not go below page 1", () => {
      const { result } = renderHook(() => usePagination());

      act(() => result.current.prevPage());
      act(() => result.current.prevPage());

      expect(result.current.page).toBe(1);
    });
  });

  describe("goToPage", () => {
    it("navigates to the specified page", () => {
      const { result } = renderHook(() => usePagination());

      act(() => result.current.goToPage(10));

      expect(result.current.page).toBe(10);
    });

    it("clamps to page 1 when given zero or negative", () => {
      const { result } = renderHook(() => usePagination());

      act(() => result.current.goToPage(0));
      expect(result.current.page).toBe(1);

      act(() => result.current.goToPage(-5));
      expect(result.current.page).toBe(1);
    });
  });

  describe("changePageSize", () => {
    it("updates pageSize and resets page to 1", () => {
      const { result } = renderHook(() =>
        usePagination({ initialPage: 5 })
      );

      act(() => result.current.changePageSize(50));

      expect(result.current.pageSize).toBe(50);
      expect(result.current.page).toBe(1);
    });
  });

  describe("resetPage", () => {
    it("resets page to 1", () => {
      const { result } = renderHook(() => usePagination());

      act(() => result.current.goToPage(7));
      expect(result.current.page).toBe(7);

      act(() => result.current.resetPage());
      expect(result.current.page).toBe(1);
    });
  });

  describe("paginationParams", () => {
    it("reflects current page and pageSize", () => {
      const { result } = renderHook(() => usePagination());

      act(() => result.current.goToPage(3));
      act(() => result.current.changePageSize(10));

      expect(result.current.paginationParams).toEqual({
        page: 1, // changePageSize resets page
        page_size: 10,
      });
    });
  });
});
