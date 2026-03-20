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

// Approval Matrix
export interface ApprovalMatrix {
  id: string;
  name: string;
  operation_id?: string;
  description?: string;
  is_active: boolean;
  levels?: ApprovalLevel[];
}

export interface ApprovalLevel {
  id: string;
  matrix_id: string;
  level_order: number;
  name: string;
  condition_field?: string;
  condition_operator: string;
  condition_value?: number;
  approver_type: string;
  approver_role?: string;
  approver_user_id?: string;
  is_parallel: boolean;
  sla_hours?: number;
  auto_escalate: boolean;
}

export interface ApprovalInstance {
  id: string;
  matrix_id: string;
  operation_id: string;
  record_id: string;
  status: string;
  current_level: number;
  submitted_by?: string;
  submitted_at: string;
  completed_at?: string;
}

export const approvalApi = {
  listMatrices: async (operationId?: string): Promise<ApprovalMatrix[]> => {
    const response = await apiClient.get("/system/approvals/matrices", {
      params: operationId ? { operation_id: operationId } : {},
    });
    return response.data;
  },
  getMatrix: async (id: string): Promise<ApprovalMatrix> => {
    const response = await apiClient.get(`/system/approvals/matrices/${id}`);
    return response.data;
  },
  createMatrix: async (
    data: Partial<ApprovalMatrix> & { levels: Partial<ApprovalLevel>[] }
  ): Promise<ApprovalMatrix> => {
    const response = await apiClient.post(
      "/system/approvals/matrices",
      data
    );
    return response.data;
  },
  deleteMatrix: async (id: string): Promise<void> => {
    await apiClient.delete(`/system/approvals/matrices/${id}`);
  },
  submitForApproval: async (data: {
    matrix_id: string;
    operation_id: string;
    record_id: string;
  }): Promise<ApprovalInstance> => {
    const response = await apiClient.post(
      "/system/approvals/submit",
      data
    );
    return response.data;
  },
  processAction: async (
    instanceId: string,
    action: string,
    comment?: string
  ): Promise<ApprovalInstance> => {
    const response = await apiClient.post(
      `/system/approvals/instances/${instanceId}/action`,
      { action, comment }
    );
    return response.data;
  },
  getPending: async (): Promise<ApprovalInstance[]> => {
    const response = await apiClient.get("/system/approvals/pending");
    return response.data;
  },
};

// BPM Workflows
export interface WorkflowDefinition {
  id: string;
  name: string;
  operation_id?: string;
  trigger_event: string;
  definition: Record<string, unknown>;
  is_active: boolean;
  version: number;
}

export interface WorkflowInstance {
  id: string;
  definition_id: string;
  record_id: string;
  operation_id: string;
  current_node?: string;
  status: string;
  started_at: string;
  completed_at?: string;
}

export const workflowEngineApi = {
  listDefinitions: async (
    operationId?: string
  ): Promise<WorkflowDefinition[]> => {
    const response = await apiClient.get("/system/workflows", {
      params: operationId ? { operation_id: operationId } : {},
    });
    return response.data;
  },
  createDefinition: async (
    data: Partial<WorkflowDefinition>
  ): Promise<WorkflowDefinition> => {
    const response = await apiClient.post("/system/workflows", data);
    return response.data;
  },
  updateDefinition: async (
    id: string,
    data: Partial<WorkflowDefinition>
  ): Promise<WorkflowDefinition> => {
    const response = await apiClient.put(`/system/workflows/${id}`, data);
    return response.data;
  },
  deleteDefinition: async (id: string): Promise<void> => {
    await apiClient.delete(`/system/workflows/${id}`);
  },
  startInstance: async (
    defId: string,
    operationId: string,
    recordId: string
  ): Promise<WorkflowInstance> => {
    const response = await apiClient.post(
      `/system/workflows/${defId}/start`,
      { operation_id: operationId, record_id: recordId }
    );
    return response.data;
  },
  listInstances: async (
    operationId: string
  ): Promise<WorkflowInstance[]> => {
    const response = await apiClient.get("/system/workflows/instances", {
      params: { operation_id: operationId },
    });
    return response.data;
  },
};

// Cross-field Rules & Formulas
export interface CrossFieldRule {
  id: string;
  operation_id: string;
  rule_name: string;
  description?: string;
  source_field: string;
  operator: string;
  target_field?: string;
  target_value?: string;
  error_message: string;
  is_active: boolean;
}

export interface CalculationFormula {
  id: string;
  operation_id: string;
  target_field: string;
  formula: string;
  trigger_fields: string[];
  description?: string;
  is_active: boolean;
}

export const rulesApi = {
  listRules: async (operationId: string): Promise<CrossFieldRule[]> => {
    const response = await apiClient.get(
      `/system/rules/${operationId}`
    );
    return response.data;
  },
  createRule: async (
    operationId: string,
    data: Partial<CrossFieldRule>
  ): Promise<CrossFieldRule> => {
    const response = await apiClient.post(
      `/system/rules/${operationId}`,
      data
    );
    return response.data;
  },
  deleteRule: async (
    operationId: string,
    id: string
  ): Promise<void> => {
    await apiClient.delete(`/system/rules/${operationId}/${id}`);
  },
  listFormulas: async (
    operationId: string
  ): Promise<CalculationFormula[]> => {
    const response = await apiClient.get(
      `/system/formulas/${operationId}`
    );
    return response.data;
  },
  createFormula: async (
    operationId: string,
    data: Partial<CalculationFormula>
  ): Promise<CalculationFormula> => {
    const response = await apiClient.post(
      `/system/formulas/${operationId}`,
      data
    );
    return response.data;
  },
  deleteFormula: async (
    operationId: string,
    id: string
  ): Promise<void> => {
    await apiClient.delete(`/system/formulas/${operationId}/${id}`);
  },
};

// Number Range Config
export interface NumberRangeConfig {
  id: string;
  range_prefix: string;
  description?: string;
  current_value: number;
  start_value: number;
  end_value: number;
  padding: number;
  separator: string;
  fiscal_year_dependent: boolean;
  is_active: boolean;
}

export const numberRangeApi = {
  list: async (): Promise<NumberRangeConfig[]> => {
    const response = await apiClient.get("/system/number-ranges");
    return response.data;
  },
  update: async (
    id: string,
    data: Partial<NumberRangeConfig>
  ): Promise<NumberRangeConfig> => {
    const response = await apiClient.put(
      `/system/number-ranges/${id}`,
      data
    );
    return response.data;
  },
};

// Output Rules
export interface OutputRule {
  id: string;
  name: string;
  operation_id?: string;
  trigger_event: string;
  output_type: string;
  email_template_code?: string;
  print_layout_code?: string;
  is_active: boolean;
}

export const outputApi = {
  listRules: async (operationId?: string): Promise<OutputRule[]> => {
    const response = await apiClient.get("/system/outputs", {
      params: operationId ? { operation_id: operationId } : {},
    });
    return response.data;
  },
  createRule: async (data: Partial<OutputRule>): Promise<OutputRule> => {
    const response = await apiClient.post("/system/outputs", data);
    return response.data;
  },
  deleteRule: async (id: string): Promise<void> => {
    await apiClient.delete(`/system/outputs/${id}`);
  },
};

// Form Variants
export interface FormVariant {
  id: string;
  operation_id: string;
  variant_name: string;
  condition_field?: string;
  condition_value?: string;
  hidden_fields: string[];
  readonly_fields: string[];
  required_fields: string[];
  default_values: Record<string, unknown>;
  is_default: boolean;
}

export const formVariantsApi = {
  list: async (operationId: string): Promise<FormVariant[]> => {
    const response = await apiClient.get(
      `/system/form-variants/${operationId}`
    );
    return response.data;
  },
  create: async (
    operationId: string,
    data: Partial<FormVariant>
  ): Promise<FormVariant> => {
    const response = await apiClient.post(
      `/system/form-variants/${operationId}`,
      data
    );
    return response.data;
  },
  delete: async (
    operationId: string,
    id: string
  ): Promise<void> => {
    await apiClient.delete(
      `/system/form-variants/${operationId}/${id}`
    );
  },
};

// Auth Trace
export interface AuthTraceEntry {
  id: string;
  user_id: string;
  resource_type: string;
  action: string;
  result: string;
  reason?: string;
  created_at: string;
}

export const authTraceApi = {
  getTrace: async (userId: string): Promise<AuthTraceEntry[]> => {
    const response = await apiClient.get(
      `/system/auth-trace/${userId}`
    );
    return response.data;
  },
  getDenials: async (userId: string): Promise<AuthTraceEntry[]> => {
    const response = await apiClient.get(
      `/system/auth-trace/${userId}/denials`
    );
    return response.data;
  },
};

// Import Template
export const importTemplateApi = {
  download: async (code: string): Promise<Blob> => {
    const response = await apiClient.get(
      `/system/import-template/${code}`,
      { responseType: "blob" }
    );
    return response.data;
  },
};

// Report Builder
export interface ReportDefinition {
  id: string;
  report_code: string;
  name: string;
  description?: string;
  operation_id?: string;
  data_source_sql: string;
  columns: { key: string; label: string; type?: string }[];
  filters: { field: string; label: string; type?: string }[];
  chart_config?: Record<string, unknown>;
  page_size: number;
  is_public: boolean;
}

export const reportsApi = {
  list: async (): Promise<ReportDefinition[]> => {
    const response = await apiClient.get("/system/reports");
    return response.data;
  },
  get: async (id: string): Promise<ReportDefinition> => {
    const response = await apiClient.get(`/system/reports/${id}`);
    return response.data;
  },
  create: async (
    data: Partial<ReportDefinition>
  ): Promise<ReportDefinition> => {
    const response = await apiClient.post("/system/reports", data);
    return response.data;
  },
  delete: async (id: string): Promise<void> => {
    await apiClient.delete(`/system/reports/${id}`);
  },
};

// Usage Analytics
export interface UsageSummary {
  event_type: string;
  count: number;
  unique_users: number;
}

export const analyticsApi = {
  track: async (data: {
    event_type: string;
    operation_id?: string;
    page_url?: string;
    duration_ms?: number;
  }): Promise<void> => {
    await apiClient.post("/system/analytics/track", data);
  },
  getSummary: async (days?: number): Promise<UsageSummary[]> => {
    const response = await apiClient.get("/system/analytics/summary", {
      params: { days },
    });
    return response.data;
  },
  getOperationStats: async (
    operationId: string
  ): Promise<UsageSummary[]> => {
    const response = await apiClient.get(
      `/system/analytics/operation/${operationId}`
    );
    return response.data;
  },
};

// Dashboard Templates
export interface DashboardTemplate {
  id: string;
  template_code: string;
  name: string;
  description?: string;
  category: string;
  definition: Record<string, unknown>;
}

export const dashboardTemplatesApi = {
  list: async (): Promise<DashboardTemplate[]> => {
    const response = await apiClient.get("/system/dashboard-templates");
    return response.data;
  },
};

// Exchange Rates
export interface ExchangeRate {
  id: string;
  from_currency: string;
  to_currency: string;
  rate: string;
  valid_from: string;
  valid_until?: string;
  source: string;
}

export const exchangeRateApi = {
  list: async (): Promise<ExchangeRate[]> => {
    const response = await apiClient.get("/system/exchange-rates");
    return response.data;
  },
  create: async (data: {
    from_currency: string;
    to_currency: string;
    rate: number;
    valid_from: string;
  }): Promise<ExchangeRate> => {
    const response = await apiClient.post("/system/exchange-rates", data);
    return response.data;
  },
  delete: async (id: string): Promise<void> => {
    await apiClient.delete(`/system/exchange-rates/${id}`);
  },
  convert: async (
    from: string,
    to: string,
    amount: number
  ): Promise<{
    from: string;
    to: string;
    amount: string;
    result: string;
  }> => {
    const response = await apiClient.get(
      "/system/exchange-rates/convert",
      { params: { from, to, amount } }
    );
    return response.data;
  },
};
