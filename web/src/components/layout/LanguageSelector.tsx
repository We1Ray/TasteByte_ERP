"use client";

import { useRouter } from "next/navigation";
import { Globe } from "lucide-react";

export function LanguageSelector() {
  const router = useRouter();

  const toggleLocale = () => {
    // Toggle between zh-TW and en by setting cookie
    const current =
      document.cookie.match(/NEXT_LOCALE=(\w+(-\w+)?)/)?.[1] || "zh-TW";
    const next = current === "zh-TW" ? "en" : "zh-TW";
    document.cookie = `NEXT_LOCALE=${next};path=/;max-age=31536000`;
    router.refresh();
  };

  return (
    <button
      onClick={toggleLocale}
      className="flex items-center gap-1.5 rounded-md px-2 py-1.5 text-xs text-gray-500 hover:bg-gray-100 hover:text-gray-700"
      title="Toggle Language"
    >
      <Globe className="h-4 w-4" />
      <span>中/EN</span>
    </button>
  );
}
