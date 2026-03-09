"use client";

import { create } from "zustand";
import type { AiChatMessage, ProposedChanges } from "../api/ai-chat";

interface AiChatState {
  isOpen: boolean;
  messages: AiChatMessage[];
  pendingChanges: ProposedChanges | null;
  isLoading: boolean;

  toggleOpen: () => void;
  setOpen: (open: boolean) => void;
  addUserMessage: (content: string) => void;
  addAssistantMessage: (content: string, proposedChanges?: ProposedChanges | null) => void;
  setPendingChanges: (changes: ProposedChanges | null) => void;
  clearPendingChanges: () => void;
  setLoading: (loading: boolean) => void;
  clearChat: () => void;
}

export const useAiChatStore = create<AiChatState>((set) => ({
  isOpen: false,
  messages: [],
  pendingChanges: null,
  isLoading: false,

  toggleOpen: () => set((s) => ({ isOpen: !s.isOpen })),
  setOpen: (open) => set({ isOpen: open }),

  addUserMessage: (content) =>
    set((s) => ({
      messages: [...s.messages, { role: "user", content }],
    })),

  addAssistantMessage: (content, proposedChanges) =>
    set((s) => ({
      messages: [
        ...s.messages,
        {
          role: "assistant",
          content,
          proposed_changes: proposedChanges ?? undefined,
        },
      ],
      pendingChanges: proposedChanges ?? s.pendingChanges,
    })),

  setPendingChanges: (changes) => set({ pendingChanges: changes }),
  clearPendingChanges: () => set({ pendingChanges: null }),
  setLoading: (loading) => set({ isLoading: loading }),
  clearChat: () => set({ messages: [], pendingChanges: null, isLoading: false }),
}));
