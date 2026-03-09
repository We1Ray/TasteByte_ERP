"use client";

import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import type { AxiosError } from "axios";
import type { ApiError } from "../api/client";
import { formApi, executorApi, operationsApi } from "../api/lowcode";
import type { FormDefinition, FormRecord } from "../types/lowcode";

export function useDynamicForm(operationCode: string) {
  const queryClient = useQueryClient();

  const operationQuery = useQuery({
    queryKey: ["lowcode", "operations", "by-code", operationCode],
    queryFn: () => operationsApi.list({ search: operationCode, page_size: 1 }),
    select: (data) => data.items.find((op) => op.code === operationCode),
  });

  const operationId = operationQuery.data?.id;

  const formQuery = useQuery<FormDefinition, AxiosError<ApiError>>({
    queryKey: ["lowcode", "form", operationId],
    queryFn: () => formApi.getDefinition(operationId!),
    enabled: !!operationId,
  });

  const createMutation = useMutation({
    mutationFn: (data: Record<string, unknown>) =>
      executorApi.create(operationCode, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["lowcode", "data", operationCode] });
    },
  });

  const updateMutation = useMutation({
    mutationFn: ({ id, data }: { id: string; data: Record<string, unknown> }) =>
      executorApi.update(operationCode, id, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["lowcode", "data", operationCode] });
    },
  });

  return {
    operation: operationQuery.data,
    formDefinition: formQuery.data,
    isLoading: operationQuery.isLoading || formQuery.isLoading,
    error: operationQuery.error || formQuery.error,
    createRecord: createMutation.mutateAsync,
    updateRecord: updateMutation.mutateAsync,
    isSubmitting: createMutation.isPending || updateMutation.isPending,
  };
}

export function useFormRecords(code: string, params?: { page?: number; page_size?: number; search?: string }) {
  return useQuery({
    queryKey: ["lowcode", "data", code, params],
    queryFn: () => executorApi.list(code, params),
  });
}

export function useFormRecord(code: string, id: string) {
  return useQuery<FormRecord, AxiosError<ApiError>>({
    queryKey: ["lowcode", "data", code, id],
    queryFn: () => executorApi.get(code, id),
    enabled: !!id,
  });
}
