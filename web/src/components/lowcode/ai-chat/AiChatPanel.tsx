"use client";

import { useRef, useEffect, useState, useCallback, type KeyboardEvent } from "react";
import { X, Send, Eye } from "lucide-react";
import { useTranslations } from "next-intl";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { useAiChatStore } from "@/lib/stores/ai-chat-store";
import { aiChatApi, type ProposedChanges } from "@/lib/api/ai-chat";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { AiChangePreview } from "./AiChangePreview";

interface AiChatPanelProps {
  operationId: string;
  contextType: string;
  onApplyChanges: (changes: ProposedChanges) => void;
}

export function AiChatPanel({
  operationId,
  contextType,
  onApplyChanges,
}: AiChatPanelProps) {
  const t = useTranslations("developer");
  const {
    isOpen,
    setOpen,
    messages,
    isLoading,
    setLoading,
    addUserMessage,
    addAssistantMessage,
    clearPendingChanges,
  } = useAiChatStore();

  const [input, setInput] = useState("");
  const [previewChanges, setPreviewChanges] = useState<ProposedChanges | null>(null);
  const [previewOpen, setPreviewOpen] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  const { data: aiStatus } = useApiQuery(
    ["ai", "status"],
    () => aiChatApi.getStatus(),
    { staleTime: 60000, retry: false }
  );

  // Auto-scroll to bottom on new messages
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages, isLoading]);

  // Focus textarea when panel opens
  useEffect(() => {
    if (isOpen) {
      textareaRef.current?.focus();
    }
  }, [isOpen]);

  const handleSend = useCallback(async () => {
    const trimmed = input.trim();
    if (!trimmed || isLoading) return;

    setInput("");
    addUserMessage(trimmed);
    setLoading(true);

    try {
      const response = await aiChatApi.sendMessage(
        operationId,
        trimmed,
        useAiChatStore.getState().messages,
        contextType
      );
      addAssistantMessage(response.message, response.proposed_changes);
    } catch {
      addAssistantMessage("Sorry, an error occurred. Please try again.");
    } finally {
      setLoading(false);
    }
  }, [input, isLoading, operationId, contextType, addUserMessage, addAssistantMessage, setLoading]);

  const handleKeyDown = (e: KeyboardEvent<HTMLTextAreaElement>) => {
    if ((e.ctrlKey || e.metaKey) && e.key === "Enter") {
      e.preventDefault();
      handleSend();
    }
  };

  const handleApplyFromMessage = (changes: ProposedChanges) => {
    onApplyChanges(changes);
    clearPendingChanges();
  };

  if (!isOpen) return null;

  return (
    <>
      <div
        className={cn(
          "fixed right-0 top-14 bottom-0 z-50 flex w-[380px] flex-col border-l border-gray-200 bg-white shadow-xl transition-transform duration-200",
          isOpen ? "translate-x-0" : "translate-x-full"
        )}
      >
        {/* Header */}
        <div className="flex items-center justify-between border-b px-4 py-3">
          <div className="flex items-center gap-2">
            <h3 className="text-sm font-semibold text-gray-900">
              {t("aiAssistant")}
            </h3>
            {aiStatus?.provider && (
              <span className="rounded bg-gray-100 px-1.5 py-0.5 text-xs text-gray-500">
                {aiStatus.model || aiStatus.provider}
              </span>
            )}
          </div>
          <button
            type="button"
            onClick={() => setOpen(false)}
            className="rounded-md p-1 text-gray-400 hover:bg-gray-100 hover:text-gray-600"
          >
            <X className="h-4 w-4" />
          </button>
        </div>

        {/* Messages */}
        <div className="flex-1 overflow-y-auto p-4 space-y-4">
          {messages.length === 0 && (
            <div className="flex h-full items-center justify-center">
              <p className="text-center text-sm text-gray-400">
                {t("aiSendMessage")}
              </p>
            </div>
          )}

          {messages.map((msg, i) => (
            <div
              key={i}
              className={cn(
                "flex",
                msg.role === "user" ? "justify-end" : "justify-start"
              )}
            >
              <div
                className={cn(
                  "max-w-[85%] rounded-lg px-3 py-2 text-sm",
                  msg.role === "user"
                    ? "bg-blue-600 text-white"
                    : "bg-gray-100 text-gray-800"
                )}
              >
                <p className="whitespace-pre-wrap">{msg.content}</p>

                {/* Proposed changes actions */}
                {msg.proposed_changes && (
                  <div className="mt-2 space-y-2">
                    {msg.proposed_changes.summary.length > 0 && (
                      <ul className="list-inside list-disc space-y-0.5 text-xs opacity-80">
                        {msg.proposed_changes.summary.map((s, j) => (
                          <li key={j}>{s}</li>
                        ))}
                      </ul>
                    )}
                    <div className="flex gap-2">
                      <button
                        type="button"
                        onClick={() => {
                          setPreviewChanges(msg.proposed_changes!);
                          setPreviewOpen(true);
                        }}
                        className="inline-flex items-center gap-1 rounded bg-white/20 px-2 py-1 text-xs font-medium hover:bg-white/30"
                      >
                        <Eye className="h-3 w-3" />
                        {t("aiPreviewDiff")}
                      </button>
                      <button
                        type="button"
                        onClick={() => handleApplyFromMessage(msg.proposed_changes!)}
                        className="inline-flex items-center gap-1 rounded bg-green-600 px-2 py-1 text-xs font-medium text-white hover:bg-green-700"
                      >
                        {t("aiApplyChanges")}
                      </button>
                      <button
                        type="button"
                        onClick={() => clearPendingChanges()}
                        className="inline-flex items-center gap-1 rounded bg-gray-300 px-2 py-1 text-xs font-medium text-gray-700 hover:bg-gray-400"
                      >
                        {t("aiDismiss")}
                      </button>
                    </div>
                  </div>
                )}
              </div>
            </div>
          ))}

          {/* Loading indicator */}
          {isLoading && (
            <div className="flex justify-start">
              <div className="rounded-lg bg-gray-100 px-3 py-2">
                <div className="flex items-center gap-1.5">
                  <span className="text-xs text-gray-500">{t("aiThinking")}</span>
                  <span className="flex gap-0.5">
                    <span className="h-1.5 w-1.5 animate-bounce rounded-full bg-gray-400 [animation-delay:0ms]" />
                    <span className="h-1.5 w-1.5 animate-bounce rounded-full bg-gray-400 [animation-delay:150ms]" />
                    <span className="h-1.5 w-1.5 animate-bounce rounded-full bg-gray-400 [animation-delay:300ms]" />
                  </span>
                </div>
              </div>
            </div>
          )}

          <div ref={messagesEndRef} />
        </div>

        {/* Input area */}
        <div className="border-t p-3">
          <div className="flex items-end gap-2">
            <textarea
              ref={textareaRef}
              value={input}
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={handleKeyDown}
              placeholder={t("aiSendMessage")}
              rows={2}
              className="flex-1 resize-none rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
              disabled={isLoading}
            />
            <Button
              size="icon"
              onClick={handleSend}
              disabled={!input.trim() || isLoading}
              title="Send (Ctrl+Enter)"
            >
              <Send className="h-4 w-4" />
            </Button>
          </div>
          <p className="mt-1 text-xs text-gray-400">Ctrl+Enter to send</p>
        </div>
      </div>

      {/* Diff preview modal */}
      <AiChangePreview
        open={previewOpen}
        onClose={() => setPreviewOpen(false)}
        changes={previewChanges}
        onApply={() => {
          if (previewChanges) {
            onApplyChanges(previewChanges);
            clearPendingChanges();
          }
        }}
      />
    </>
  );
}
