"use client";

import {
  useQuery,
  useMutation,
  useQueryClient,
  type UseQueryOptions,
  type UseMutationOptions,
} from "@tanstack/react-query";
import { type AxiosError } from "axios";
import type { ApiError } from "../api/client";

export function useApiQuery<TData>(
  queryKey: string[],
  queryFn: () => Promise<TData>,
  options?: Omit<UseQueryOptions<TData, AxiosError<ApiError>>, "queryKey" | "queryFn">
) {
  return useQuery<TData, AxiosError<ApiError>>({
    queryKey,
    queryFn,
    ...options,
  });
}

export function useApiMutation<TData, TVariables>(
  mutationFn: (variables: TVariables) => Promise<TData>,
  options?: Omit<UseMutationOptions<TData, AxiosError<ApiError>, TVariables>, "mutationFn">
) {
  return useMutation<TData, AxiosError<ApiError>, TVariables>({
    mutationFn,
    ...options,
  });
}

export function useInvalidateQueries() {
  const queryClient = useQueryClient();
  return (queryKey: string[]) => queryClient.invalidateQueries({ queryKey });
}
