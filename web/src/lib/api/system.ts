import apiClient, { type PaginatedResponse } from "./client";

// User Preferences
export interface UserPreference {
  id: string;
  user_id: string;
  language: string;
  timezone: string;
  date_format: string;
  theme: string;
  notifications_enabled: boolean;
  email_notifications: boolean;
  page_size: number;
  sidebar_collapsed: boolean;
  custom_settings: Record<string, unknown>;
}

export const preferencesApi = {
  get: async (): Promise<UserPreference> => {
    const response = await apiClient.get("/system/preferences");
    return response.data;
  },
  update: async (data: Partial<UserPreference>): Promise<UserPreference> => {
    const response = await apiClient.put("/system/preferences", data);
    return response.data;
  },
};

// Saved Variants
export interface SavedVariant {
  id: string;
  user_id: string;
  context: string;
  variant_name: string;
  is_default: boolean;
  filters: Record<string, unknown>;
  columns?: Record<string, unknown>;
  sort_config?: Record<string, unknown>;
}

export const variantsApi = {
  list: async (context: string): Promise<SavedVariant[]> => {
    const response = await apiClient.get("/system/variants", { params: { context } });
    return response.data;
  },
  create: async (data: {
    context: string;
    variant_name: string;
    is_default?: boolean;
    filters: Record<string, unknown>;
  }): Promise<SavedVariant> => {
    const response = await apiClient.post("/system/variants", data);
    return response.data;
  },
  delete: async (id: string): Promise<void> => {
    await apiClient.delete(`/system/variants/${id}`);
  },
};

// System Health
export interface SystemHealth {
  status: string;
  version: string;
  uptime_seconds: number;
  database: {
    connected: boolean;
    pool_size: number;
    idle_connections: number;
  };
  stats: {
    total_users: number;
    total_operations: number;
    total_records: number;
    pending_releases: number;
    active_jobs: number;
  };
}

export const systemApi = {
  health: async (): Promise<SystemHealth> => {
    const response = await apiClient.get("/system/health");
    return response.data;
  },
  search: async (
    q: string,
    limit?: number
  ): Promise<
    {
      category: string;
      id: string;
      title: string;
      subtitle?: string;
      url: string;
    }[]
  > => {
    const response = await apiClient.get("/system/search", {
      params: { q, limit },
    });
    return response.data;
  },
};

// Email Management
export interface EmailTemplate {
  id: string;
  template_code: string;
  subject: string;
  body_html: string;
  variables: string[];
  is_active: boolean;
}

export interface EmailLog {
  id: string;
  template_code?: string;
  recipient: string;
  subject: string;
  status: string;
  sent_at?: string;
  created_at: string;
}

export const emailApi = {
  listTemplates: async (): Promise<EmailTemplate[]> => {
    const response = await apiClient.get("/system/email/templates");
    return response.data;
  },
  listLogs: async (): Promise<EmailLog[]> => {
    const response = await apiClient.get("/system/email/logs");
    return response.data;
  },
};

// Scheduled Jobs
export interface ScheduledJob {
  id: string;
  job_name: string;
  job_type: string;
  cron_expression: string;
  handler: string;
  config: Record<string, unknown>;
  is_active: boolean;
  last_run_at?: string;
  last_status?: string;
  last_error?: string;
  next_run_at?: string;
}

export interface JobExecutionLog {
  id: string;
  job_id: string;
  status: string;
  started_at: string;
  finished_at?: string;
  duration_ms?: number;
  error_message?: string;
}

export const jobsApi = {
  list: async (): Promise<ScheduledJob[]> => {
    const response = await apiClient.get("/system/jobs");
    return response.data;
  },
  create: async (data: {
    job_name: string;
    job_type: string;
    cron_expression: string;
    handler: string;
    config?: Record<string, unknown>;
  }): Promise<ScheduledJob> => {
    const response = await apiClient.post("/system/jobs", data);
    return response.data;
  },
  toggle: async (id: string, is_active: boolean): Promise<ScheduledJob> => {
    const response = await apiClient.put(`/system/jobs/${id}`, { is_active });
    return response.data;
  },
  delete: async (id: string): Promise<void> => {
    await apiClient.delete(`/system/jobs/${id}`);
  },
  getLogs: async (id: string): Promise<JobExecutionLog[]> => {
    const response = await apiClient.get(`/system/jobs/${id}/logs`);
    return response.data;
  },
};

// Webhooks
export interface Webhook {
  id: string;
  name: string;
  url: string;
  events: string[];
  is_active: boolean;
  retry_count: number;
}

export const webhooksApi = {
  list: async (): Promise<Webhook[]> => {
    const response = await apiClient.get("/system/webhooks");
    return response.data;
  },
  create: async (data: {
    name: string;
    url: string;
    events: string[];
    secret?: string;
  }): Promise<Webhook> => {
    const response = await apiClient.post("/system/webhooks", data);
    return response.data;
  },
  delete: async (id: string): Promise<void> => {
    await apiClient.delete(`/system/webhooks/${id}`);
  },
};

// Print Layouts
export interface PrintLayout {
  id: string;
  layout_code: string;
  name: string;
  description?: string;
  template_html: string;
  paper_size: string;
  orientation: string;
}

export const printApi = {
  listLayouts: async (): Promise<PrintLayout[]> => {
    const response = await apiClient.get("/system/print/layouts");
    return response.data;
  },
  saveLayout: async (data: Partial<PrintLayout>): Promise<PrintLayout> => {
    const response = await apiClient.post("/system/print/layouts", data);
    return response.data;
  },
  render: async (
    layoutCode: string,
    data: Record<string, unknown>
  ): Promise<string> => {
    const response = await apiClient.post(
      `/system/print/render/${layoutCode}`,
      data
    );
    return response.data;
  },
};

// Transport Orders
export interface TransportOrder {
  id: string;
  transport_number: string;
  description?: string;
  source_env: string;
  target_env: string;
  status: string;
  object_type: string;
  created_at: string;
}

export const transportApi = {
  list: async (): Promise<TransportOrder[]> => {
    const response = await apiClient.get("/system/transports");
    return response.data;
  },
  create: async (data: {
    description?: string;
    target_env: string;
    object_type: string;
    object_id?: string;
    payload: Record<string, unknown>;
  }): Promise<TransportOrder> => {
    const response = await apiClient.post("/system/transports", data);
    return response.data;
  },
};
