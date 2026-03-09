import apiClient, { type PaginatedResponse } from "./client";

export interface Notification {
  id: string;
  user_id: string;
  title: string;
  message: string;
  notification_type: "info" | "warning" | "success" | "error";
  module: string | null;
  reference_id: string | null;
  is_read: boolean;
  created_at: string;
}

export interface UnreadCount {
  count: number;
}

export interface NotificationListParams {
  page?: number;
  per_page?: number;
  is_read?: boolean;
  notification_type?: string;
  module?: string;
}

export const notificationsApi = {
  getNotifications: async (
    params?: NotificationListParams
  ): Promise<PaginatedResponse<Notification>> => {
    const response = await apiClient.get("/notifications", { params });
    return response.data;
  },

  getUnreadCount: async (): Promise<UnreadCount> => {
    const response = await apiClient.get("/notifications/unread-count");
    return response.data;
  },

  markAsRead: async (id: string): Promise<void> => {
    await apiClient.put(`/notifications/${id}/read`);
  },

  markAllAsRead: async (): Promise<void> => {
    await apiClient.put("/notifications/read-all");
  },

  deleteNotification: async (id: string): Promise<void> => {
    await apiClient.delete(`/notifications/${id}`);
  },
};
