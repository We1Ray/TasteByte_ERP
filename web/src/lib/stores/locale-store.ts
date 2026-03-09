"use client";

import { create } from "zustand";
import { type Locale, defaultLocale } from "@/i18n/config";

interface LocaleState {
  locale: Locale;
  setLocale: (locale: Locale) => void;
  hydrate: () => void;
}

const STORAGE_KEY = "tastebyte-locale";

function setCookie(name: string, value: string) {
  document.cookie = `${name}=${value};path=/;max-age=${365 * 24 * 60 * 60}`;
}

function getCookie(name: string): string | null {
  const match = document.cookie.match(new RegExp(`(^| )${name}=([^;]+)`));
  return match ? match[2] : null;
}

export const useLocaleStore = create<LocaleState>((set) => ({
  locale: defaultLocale,
  setLocale: (locale) => {
    setCookie(STORAGE_KEY, locale);
    set({ locale });
    window.location.reload();
  },
  hydrate: () => {
    const stored = getCookie(STORAGE_KEY);
    if (stored === "zh-TW" || stored === "en") {
      set({ locale: stored });
    }
  },
}));
