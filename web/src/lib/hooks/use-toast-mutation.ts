"use client";

import { toast } from "sonner";
import { useMutation, useQueryClient, type InvalidateQueryFilters } from "@tanstack/react-query";
import { AxiosError } from "axios";

export function useToastMutation<TData, TVariables>(
  mutationFn: (variables: TVariables) => Promise<TData>,
  options: {
    successMessage: string;
    errorMessage?: string;
    invalidateKeys?: InvalidateQueryFilters["queryKey"];
    onSuccess?: (data: TData) => void;
  }
) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn,
    onSuccess: (data) => {
      toast.success(options.successMessage);
      if (options.invalidateKeys) {
        queryClient.invalidateQueries({ queryKey: options.invalidateKeys });
      }
      options.onSuccess?.(data);
    },
    onError: (error: Error) => {
      if (error instanceof AxiosError && error.response?.status === 403) {
        toast.error("You do not have permission to perform this action");
        return;
      }
      toast.error(options.errorMessage ?? error.message ?? "An error occurred");
    },
  });
}
