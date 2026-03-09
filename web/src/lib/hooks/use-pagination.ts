"use client";

import { useState, useCallback } from "react";

interface UsePaginationOptions {
  initialPage?: number;
  initialPageSize?: number;
}

export function usePagination(options: UsePaginationOptions = {}) {
  const { initialPage = 1, initialPageSize = 20 } = options;
  const [page, setPage] = useState(initialPage);
  const [pageSize, setPageSize] = useState(initialPageSize);

  const nextPage = useCallback(() => {
    setPage((p) => p + 1);
  }, []);

  const prevPage = useCallback(() => {
    setPage((p) => Math.max(1, p - 1));
  }, []);

  const goToPage = useCallback((p: number) => {
    setPage(Math.max(1, p));
  }, []);

  const changePageSize = useCallback((size: number) => {
    setPageSize(size);
    setPage(1);
  }, []);

  const resetPage = useCallback(() => {
    setPage(1);
  }, []);

  return {
    page,
    pageSize,
    nextPage,
    prevPage,
    goToPage,
    changePageSize,
    resetPage,
    paginationParams: { page, page_size: pageSize },
  };
}
