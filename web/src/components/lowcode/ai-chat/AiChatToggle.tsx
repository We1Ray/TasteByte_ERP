"use client";

import { Sparkles } from "lucide-react";
import { useAiChatStore } from "@/lib/stores/ai-chat-store";
import { cn } from "@/lib/utils";

export function AiChatToggle() {
  const { isOpen, toggleOpen, messages, pendingChanges } = useAiChatStore();
  const messageCount = messages.length;

  return (
    <button
      type="button"
      onClick={toggleOpen}
      className={cn(
        "fixed bottom-6 right-6 z-50 flex h-12 w-12 items-center justify-center rounded-full shadow-lg transition-all hover:scale-105",
        isOpen
          ? "bg-gray-600 text-white hover:bg-gray-700"
          : "bg-blue-600 text-white hover:bg-blue-700",
        pendingChanges && !isOpen && "animate-pulse"
      )}
      title="AI Assistant"
    >
      <Sparkles className="h-5 w-5" />
      {!isOpen && messageCount > 0 && (
        <span className="absolute -right-1 -top-1 flex h-5 w-5 items-center justify-center rounded-full bg-red-500 text-xs font-bold text-white">
          {messageCount > 9 ? "9+" : messageCount}
        </span>
      )}
    </button>
  );
}
