"use client";

import { useEffect } from "react";
import { useRouter } from "next/navigation";
import { useAuthStore } from "../stores/auth-store";
import { authApi } from "../api/auth";

export function useAuth() {
  const { user, token, isAuthenticated, isLoading, setAuth, setUser, logout, hydrate, setLoading } =
    useAuthStore();
  const router = useRouter();

  useEffect(() => {
    hydrate();
  }, [hydrate]);

  useEffect(() => {
    if (token && !user) {
      authApi
        .me()
        .then((userData) => {
          setUser(userData);
        })
        .catch(() => {
          logout();
        });
    }
  }, [token, user, setUser, logout]);

  const login = async (username: string, password: string) => {
    setLoading(true);
    try {
      const data = await authApi.login({ username, password });
      setAuth(data.user, data.access_token, data.refresh_token);
      router.push("/dashboard");
    } catch (error) {
      setLoading(false);
      throw error;
    }
  };

  const handleLogout = () => {
    logout();
    router.push("/login");
  };

  return {
    user,
    isAuthenticated,
    isLoading,
    login,
    logout: handleLogout,
  };
}
