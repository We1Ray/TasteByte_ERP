import apiClient from "./client";

export interface AiChatMessage {
  role: "user" | "assistant";
  content: string;
  proposed_changes?: ProposedChanges | null;
}

export interface ProposedChanges {
  change_type: "form" | "list" | "dashboard" | "buttons";
  current: unknown;
  proposed: unknown;
  diff: {
    added: Record<string, unknown>;
    removed: Record<string, unknown>;
    changed: Record<string, unknown>;
  };
  summary: string[];
}

export interface AiChatResponse {
  message: string;
  proposed_changes: ProposedChanges | null;
  tool_calls_executed: string[];
}

export interface AiStatus {
  enabled: boolean;
  provider: string | null;
  model: string | null;
}

export const aiChatApi = {
  sendMessage: async (
    operationId: string,
    message: string,
    conversationHistory: AiChatMessage[],
    contextType?: string
  ): Promise<AiChatResponse> => {
    const response = await apiClient.post(
      `/lowcode/operations/${operationId}/ai/chat`,
      {
        message,
        conversation_history: conversationHistory.map((m) => ({
          role: m.role,
          content: m.content,
        })),
        context_type: contextType,
      }
    );
    return response.data;
  },

  getStatus: async (): Promise<AiStatus> => {
    const response = await apiClient.get("/lowcode/ai/status");
    return response.data;
  },
};
