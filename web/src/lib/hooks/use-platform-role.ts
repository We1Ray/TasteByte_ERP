"use client";

import { useApiQuery } from "./use-api-query";
import { userProfileApi } from "../api/lowcode";
import { useAuthStore } from "../stores/auth-store";

export function usePlatformRole() {
  const { isAuthenticated, user } = useAuthStore();

  const { data, isLoading } = useApiQuery(
    ["lowcode", "user", "me"],
    () => userProfileApi.getMyProfile(),
    {
      enabled: isAuthenticated,
      staleTime: 30 * 1000, // 30 seconds
      retry: 1,
    }
  );

  const roles = data?.platform_roles ?? [];
  const isTraditionalAdmin = user?.roles?.includes("ADMIN") ?? false;

  return {
    roles,
    isAdmin: isTraditionalAdmin || roles.includes("PLATFORM_ADMIN"),
    isDeveloper: isTraditionalAdmin || roles.includes("PLATFORM_ADMIN") || roles.includes("DEVELOPER"),
    isUser: isTraditionalAdmin || roles.length > 0,
    projects: data?.projects ?? [],
    isLoading: isAuthenticated && isLoading,
    isTraditionalAdmin,
  };
}
