"use client";

import { useEffect } from "react";
import { Globe } from "lucide-react";
import { useLocaleStore } from "@/lib/stores/locale-store";
import { type Locale } from "@/i18n/config";

const localeLabels: Record<Locale, string> = {
  "zh-TW": "中文",
  en: "EN",
};

export function LocaleSwitcher() {
  const { locale, setLocale, hydrate } = useLocaleStore();

  useEffect(() => {
    hydrate();
  }, [hydrate]);

  const toggle = () => {
    const next: Locale = locale === "zh-TW" ? "en" : "zh-TW";
    setLocale(next);
  };

  return (
    <button
      onClick={toggle}
      className="flex items-center gap-1.5 rounded-md px-2 py-1.5 text-sm text-gray-500 hover:bg-gray-100"
      title={locale === "zh-TW" ? "Switch to English" : "切換為中文"}
    >
      <Globe className="h-4 w-4" />
      <span>{localeLabels[locale]}</span>
    </button>
  );
}
