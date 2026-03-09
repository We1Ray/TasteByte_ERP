"use client";

import { create } from "zustand";
import { type User, authApi } from "../api/auth";

interface AuthState {
  user: User | null;
  token: string | null;
  refreshToken: string | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  setAuth: (user: User, token: string, refreshToken: string) => void;
  setToken: (token: string) => void;
  setUser: (user: User) => void;
  logout: () => void;
  setLoading: (loading: boolean) => void;
  hydrate: () => void;
}

export const useAuthStore = create<AuthState>((set, get) => ({
  user: null,
  token: null,
  refreshToken: null,
  isAuthenticated: false,
  isLoading: true,

  setAuth: (user, token, refreshToken) => {
    localStorage.setItem("access_token", token);
    localStorage.setItem("refresh_token", refreshToken);
    localStorage.setItem("user", JSON.stringify(user));
    set({
      user,
      token,
      refreshToken,
      isAuthenticated: true,
      isLoading: false,
    });
  },

  setToken: (token) => {
    localStorage.setItem("access_token", token);
    set({ token });
  },

  setUser: (user) => {
    localStorage.setItem("user", JSON.stringify(user));
    set({ user });
  },

  logout: () => {
    const { refreshToken } = get();
    if (refreshToken) {
      authApi.logout(refreshToken).catch(() => {
        // Best-effort server-side revocation
      });
    }
    localStorage.removeItem("access_token");
    localStorage.removeItem("refresh_token");
    localStorage.removeItem("user");
    set({
      user: null,
      token: null,
      refreshToken: null,
      isAuthenticated: false,
      isLoading: false,
    });
  },

  setLoading: (loading) => {
    set({ isLoading: loading });
  },

  hydrate: () => {
    const token = localStorage.getItem("access_token");
    const refreshToken = localStorage.getItem("refresh_token");
    if (!token) {
      set({ isLoading: false });
      return;
    }

    // Restore cached user immediately for instant render
    const cachedUser = localStorage.getItem("user");
    if (cachedUser) {
      try {
        const user = JSON.parse(cachedUser) as User;
        set({
          user,
          token,
          refreshToken,
          isAuthenticated: true,
          isLoading: false,
        });
      } catch {
        set({ token, refreshToken, isAuthenticated: true, isLoading: false });
      }
    } else {
      set({ token, refreshToken, isAuthenticated: true, isLoading: false });
    }

    // Async refresh user data from server for fresh roles/permissions
    authApi
      .me()
      .then((freshUser) => {
        localStorage.setItem("user", JSON.stringify(freshUser));
        set({ user: freshUser });
      })
      .catch(() => {
        // Token invalid - clear auth
        localStorage.removeItem("access_token");
        localStorage.removeItem("refresh_token");
        localStorage.removeItem("user");
        set({
          user: null,
          token: null,
          refreshToken: null,
          isAuthenticated: false,
          isLoading: false,
        });
        if (typeof window !== "undefined") {
          window.location.href = "/login";
        }
      });
  },
}));
