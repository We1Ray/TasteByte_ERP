"use client";

import { useState, useCallback, useRef, useEffect } from "react";
import { useRouter } from "next/navigation";
import { Search, FileText, Users, Folder, X } from "lucide-react";
import { useTranslations } from "next-intl";
import { systemApi } from "@/lib/api/system";

const categoryIcons: Record<string, React.ElementType> = {
  Operation: FileText,
  Project: Folder,
  User: Users,
};

export function GlobalSearch() {
  const t = useTranslations("common");
  const router = useRouter();
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<
    {
      category: string;
      id: string;
      title: string;
      subtitle?: string;
      url: string;
    }[]
  >([]);
  const [loading, setLoading] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);
  const debounceRef = useRef<ReturnType<typeof setTimeout>>(undefined);

  // Cmd+K to open
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === "k") {
        e.preventDefault();
        setOpen(true);
        setTimeout(() => inputRef.current?.focus(), 50);
      }
      if (e.key === "Escape") setOpen(false);
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, []);

  const search = useCallback(async (q: string) => {
    if (!q.trim()) {
      setResults([]);
      return;
    }
    setLoading(true);
    try {
      const data = await systemApi.search(q, 15);
      setResults(data);
    } catch {
      setResults([]);
    } finally {
      setLoading(false);
    }
  }, []);

  const handleChange = (value: string) => {
    setQuery(value);
    if (debounceRef.current) clearTimeout(debounceRef.current);
    debounceRef.current = setTimeout(() => search(value), 300);
  };

  const handleSelect = (url: string) => {
    router.push(url);
    setOpen(false);
    setQuery("");
    setResults([]);
  };

  if (!open) {
    return (
      <button
        onClick={() => {
          setOpen(true);
          setTimeout(() => inputRef.current?.focus(), 50);
        }}
        className="flex items-center gap-2 rounded-md border border-gray-200 bg-gray-50 px-3 py-1.5 text-sm text-gray-400 hover:bg-gray-100"
      >
        <Search className="h-4 w-4" />
        <span>{t("search")}...</span>
        <kbd className="ml-2 rounded bg-gray-200 px-1.5 py-0.5 text-[10px] font-medium text-gray-500">
          ⌘K
        </kbd>
      </button>
    );
  }

  return (
    <div
      className="fixed inset-0 z-50 flex items-start justify-center pt-[15vh]"
      onClick={() => setOpen(false)}
    >
      <div className="fixed inset-0 bg-black/30" />
      <div
        className="relative w-full max-w-lg rounded-xl bg-white shadow-2xl"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center gap-3 border-b px-4 py-3">
          <Search className="h-5 w-5 text-gray-400" />
          <input
            ref={inputRef}
            value={query}
            onChange={(e) => handleChange(e.target.value)}
            placeholder={t("globalSearchPlaceholder")}
            className="flex-1 bg-transparent text-sm outline-none placeholder:text-gray-400"
            autoFocus
          />
          <button
            onClick={() => setOpen(false)}
            className="text-gray-400 hover:text-gray-600"
          >
            <X className="h-4 w-4" />
          </button>
        </div>

        {results.length > 0 && (
          <div className="max-h-80 overflow-y-auto p-2">
            {results.map((r) => {
              const Icon = categoryIcons[r.category] || FileText;
              return (
                <button
                  key={`${r.category}-${r.id}`}
                  onClick={() => handleSelect(r.url)}
                  className="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-left hover:bg-gray-50"
                >
                  <Icon className="h-4 w-4 shrink-0 text-gray-400" />
                  <div className="min-w-0 flex-1">
                    <p className="truncate text-sm font-medium text-gray-900">
                      {r.title}
                    </p>
                    {r.subtitle && (
                      <p className="truncate text-xs text-gray-400">
                        {r.subtitle}
                      </p>
                    )}
                  </div>
                  <span className="shrink-0 rounded bg-gray-100 px-1.5 py-0.5 text-[10px] font-medium text-gray-500">
                    {r.category}
                  </span>
                </button>
              );
            })}
          </div>
        )}

        {query && results.length === 0 && !loading && (
          <div className="p-6 text-center text-sm text-gray-400">
            {t("noResults")}
          </div>
        )}

        {loading && (
          <div className="p-6 text-center text-sm text-gray-400">
            {t("searching")}...
          </div>
        )}
      </div>
    </div>
  );
}
