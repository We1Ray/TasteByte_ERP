import apiClient, { type PaginatedResponse } from "./client";
import type {
  LowCodeProject,
  LowCodeOperation,
  FormDefinition,
  FormRecord,
  ListDefinition,
  DashboardDefinition,
  OperationPermission,
  FieldPermission,
  RecordPermission,
  Release,
  Feedback,
  FeedbackComment,
  JournalEntry,
  NavigationItem,
  FileAttachment,
  Notification,
  DatasourceQueryResult,
  TableInfo,
  ColumnInfo,
  SqlValidationResult,
  OperationButton,
  ModuleOperationSummary,
} from "../types/lowcode";

// Projects API
export const projectsApi = {
  list: async (params?: {
    page?: number;
    page_size?: number;
    search?: string;
    status?: string;
  }): Promise<PaginatedResponse<LowCodeProject>> => {
    const response = await apiClient.get("/lowcode/projects", { params });
    return response.data;
  },

  get: async (id: string): Promise<LowCodeProject> => {
    const response = await apiClient.get(`/lowcode/projects/${id}`);
    return response.data;
  },

  create: async (data: Partial<LowCodeProject>): Promise<LowCodeProject> => {
    const response = await apiClient.post("/lowcode/projects", data);
    return response.data;
  },

  update: async (id: string, data: Partial<LowCodeProject>): Promise<LowCodeProject> => {
    const response = await apiClient.put(`/lowcode/projects/${id}`, data);
    return response.data;
  },

  delete: async (id: string): Promise<void> => {
    await apiClient.delete(`/lowcode/projects/${id}`);
  },
};

// Operations API
export const operationsApi = {
  list: async (params?: {
    page?: number;
    page_size?: number;
    project_id?: string;
    search?: string;
    status?: string;
  }): Promise<PaginatedResponse<LowCodeOperation>> => {
    const response = await apiClient.get("/lowcode/operations", { params });
    return response.data;
  },

  get: async (id: string): Promise<LowCodeOperation> => {
    const response = await apiClient.get(`/lowcode/operations/${id}`);
    return response.data;
  },

  create: async (data: Partial<LowCodeOperation>): Promise<LowCodeOperation> => {
    const response = await apiClient.post("/lowcode/operations", data);
    return response.data;
  },

  update: async (id: string, data: Partial<LowCodeOperation>): Promise<LowCodeOperation> => {
    const response = await apiClient.put(`/lowcode/operations/${id}`, data);
    return response.data;
  },

  delete: async (id: string): Promise<void> => {
    await apiClient.delete(`/lowcode/operations/${id}`);
  },
};

// Form API
export const formApi = {
  getDefinition: async (operationId: string): Promise<FormDefinition> => {
    const response = await apiClient.get(`/lowcode/operations/${operationId}/form`);
    return response.data;
  },

  saveDefinition: async (
    operationId: string,
    data: {
      sections: FormDefinition["sections"];
      form_settings?: FormDefinition["form_settings"];
      layout_config?: FormDefinition["layout_config"];
    }
  ): Promise<FormDefinition> => {
    const response = await apiClient.put(`/lowcode/operations/${operationId}/form`, data);
    return response.data;
  },
};

// Executor API (user-facing form data CRUD)
export const executorApi = {
  list: async (
    code: string,
    params?: { page?: number; page_size?: number; search?: string }
  ): Promise<PaginatedResponse<FormRecord>> => {
    const response = await apiClient.get(`/lowcode/data/${code}`, { params });
    return response.data;
  },

  get: async (code: string, id: string): Promise<FormRecord> => {
    const response = await apiClient.get(`/lowcode/data/${code}/${id}`);
    return response.data;
  },

  create: async (code: string, data: Record<string, unknown>): Promise<FormRecord> => {
    const response = await apiClient.post(`/lowcode/data/${code}`, data);
    return response.data;
  },

  update: async (code: string, id: string, data: Record<string, unknown>): Promise<FormRecord> => {
    const response = await apiClient.put(`/lowcode/data/${code}/${id}`, data);
    return response.data;
  },

  delete: async (code: string, id: string): Promise<void> => {
    await apiClient.delete(`/lowcode/data/${code}/${id}`);
  },
};

// Datasource API
export const datasourceApi = {
  query: async (sql: string, params?: Record<string, unknown>): Promise<DatasourceQueryResult> => {
    const response = await apiClient.post("/lowcode/datasource/query", { sql, params });
    return response.data;
  },

  tables: async (): Promise<TableInfo[]> => {
    const response = await apiClient.get("/lowcode/datasource/tables");
    return response.data;
  },

  columns: async (tableName: string): Promise<ColumnInfo[]> => {
    const response = await apiClient.get(`/lowcode/datasource/tables/${tableName}/columns`);
    return response.data;
  },

  validateSql: async (sql: string): Promise<SqlValidationResult> => {
    const response = await apiClient.post("/lowcode/datasource/validate-sql", { sql });
    return response.data;
  },
};

// Permissions API
export const permissionsApi = {
  // ── Operation Permissions ──────────────────────────────────────────────
  getOperationPermissions: async (operationId: string): Promise<OperationPermission[]> => {
    const response = await apiClient.get(`/lowcode/permissions/operations/${operationId}`);
    return response.data;
  },

  createOperationPermission: async (
    operationId: string,
    permission: {
      role_id?: string;
      user_id?: string;
      can_create?: boolean;
      can_read?: boolean;
      can_update?: boolean;
      can_delete?: boolean;
      custom_permissions?: Record<string, unknown>;
    }
  ): Promise<OperationPermission> => {
    const response = await apiClient.post(
      `/lowcode/permissions/operations/${operationId}`,
      permission
    );
    return response.data;
  },

  updateOperationPermission: async (
    operationId: string,
    permissionId: string,
    permission: {
      can_create?: boolean;
      can_read?: boolean;
      can_update?: boolean;
      can_delete?: boolean;
      custom_permissions?: Record<string, unknown>;
    }
  ): Promise<OperationPermission> => {
    const response = await apiClient.put(
      `/lowcode/permissions/operations/${operationId}/${permissionId}`,
      permission
    );
    return response.data;
  },

  deleteOperationPermission: async (
    operationId: string,
    permissionId: string
  ): Promise<void> => {
    await apiClient.delete(
      `/lowcode/permissions/operations/${operationId}/${permissionId}`
    );
  },

  /** Convenience: sync a list of permissions (create new ones, update existing ones) */
  saveOperationPermissions: async (
    operationId: string,
    permissions: Partial<OperationPermission>[]
  ): Promise<OperationPermission[]> => {
    const results: OperationPermission[] = [];
    for (const perm of permissions) {
      if (perm.id) {
        // existing permission -- update
        const updated = await permissionsApi.updateOperationPermission(
          operationId,
          perm.id,
          {
            can_create: perm.can_create,
            can_read: perm.can_read,
            can_update: perm.can_update,
            can_delete: perm.can_delete,
          }
        );
        results.push(updated);
      } else {
        // new permission -- create
        const created = await permissionsApi.createOperationPermission(
          operationId,
          {
            role_id: perm.role_id ?? undefined,
            user_id: perm.user_id ?? undefined,
            can_create: perm.can_create,
            can_read: perm.can_read,
            can_update: perm.can_update,
            can_delete: perm.can_delete,
          }
        );
        results.push(created);
      }
    }
    return results;
  },

  // ── Field Permissions ──────────────────────────────────────────────────
  getFieldPermissions: async (fieldId: string): Promise<FieldPermission[]> => {
    const response = await apiClient.get(`/lowcode/permissions/fields/${fieldId}`);
    return response.data;
  },

  createFieldPermission: async (
    fieldId: string,
    permission: {
      role_id?: string;
      user_id?: string;
      visibility?: string;
      is_editable?: boolean;
    }
  ): Promise<FieldPermission> => {
    const response = await apiClient.post(
      `/lowcode/permissions/fields/${fieldId}`,
      permission
    );
    return response.data;
  },

  deleteFieldPermission: async (
    fieldId: string,
    permissionId: string
  ): Promise<void> => {
    await apiClient.delete(
      `/lowcode/permissions/fields/${fieldId}/${permissionId}`
    );
  },

  /** Convenience: sync a list of field permissions (create new, delete removed) */
  saveFieldPermissions: async (
    fieldId: string,
    permissions: Partial<FieldPermission>[]
  ): Promise<FieldPermission[]> => {
    const results: FieldPermission[] = [];
    for (const perm of permissions) {
      if (!perm.id) {
        const created = await permissionsApi.createFieldPermission(fieldId, {
          role_id: perm.role_id ?? undefined,
          user_id: perm.user_id ?? undefined,
          visibility: perm.visibility,
          is_editable: perm.is_editable,
        });
        results.push(created);
      }
    }
    return results;
  },

  // ── Record Policies ────────────────────────────────────────────────────
  getRecordPermissions: async (operationId: string): Promise<RecordPermission[]> => {
    const response = await apiClient.get(`/lowcode/permissions/records/${operationId}`);
    return response.data;
  },

  createRecordPermission: async (
    operationId: string,
    policy: {
      role_id?: string;
      user_id?: string;
      policy_name: string;
      filter_sql: string;
      is_active?: boolean;
    }
  ): Promise<RecordPermission> => {
    const response = await apiClient.post(
      `/lowcode/permissions/records/${operationId}`,
      policy
    );
    return response.data;
  },

  deleteRecordPermission: async (
    operationId: string,
    policyId: string
  ): Promise<void> => {
    await apiClient.delete(
      `/lowcode/permissions/records/${operationId}/${policyId}`
    );
  },

  /** Convenience: sync a list of record policies (create new ones) */
  saveRecordPermissions: async (
    operationId: string,
    permissions: Partial<RecordPermission>[]
  ): Promise<RecordPermission[]> => {
    const results: RecordPermission[] = [];
    for (const perm of permissions) {
      if (!perm.id) {
        const created = await permissionsApi.createRecordPermission(operationId, {
          role_id: perm.role_id ?? undefined,
          user_id: perm.user_id ?? undefined,
          policy_name: perm.policy_name ?? "default",
          filter_sql: perm.filter_sql ?? "",
          is_active: perm.is_active,
        });
        results.push(created);
      }
    }
    return results;
  },
};

// Releases API
export const releasesApi = {
  list: async (params?: {
    page?: number;
    page_size?: number;
    project_id?: string;
    status?: string;
  }): Promise<PaginatedResponse<Release>> => {
    const response = await apiClient.get("/lowcode/releases", { params });
    return response.data;
  },

  get: async (id: string): Promise<Release> => {
    const response = await apiClient.get(`/lowcode/releases/${id}`);
    return response.data;
  },

  create: async (data: Partial<Release>): Promise<Release> => {
    const response = await apiClient.post("/lowcode/releases", data);
    return response.data;
  },

  update: async (id: string, data: Partial<Release>): Promise<Release> => {
    const response = await apiClient.put(`/lowcode/releases/${id}`, data);
    return response.data;
  },

  submit: async (id: string): Promise<Release> => {
    const response = await apiClient.put(`/lowcode/releases/${id}/submit`);
    return response.data;
  },

  approve: async (id: string, notes?: string): Promise<Release> => {
    const response = await apiClient.put(`/lowcode/releases/${id}/approve`, { notes });
    return response.data;
  },

  reject: async (id: string, notes: string): Promise<Release> => {
    const response = await apiClient.put(`/lowcode/releases/${id}/reject`, { notes });
    return response.data;
  },

  publish: async (id: string): Promise<Release> => {
    const response = await apiClient.put(`/lowcode/releases/${id}/publish`);
    return response.data;
  },
};

// Feedback API
export const feedbackApi = {
  list: async (params?: {
    page?: number;
    page_size?: number;
    project_id?: string;
    status?: string;
    feedback_type?: string;
    priority?: string;
  }): Promise<PaginatedResponse<Feedback>> => {
    const response = await apiClient.get("/lowcode/feedback", { params });
    return response.data;
  },

  get: async (id: string): Promise<Feedback> => {
    const response = await apiClient.get(`/lowcode/feedback/${id}`);
    return response.data;
  },

  create: async (data: Partial<Feedback>): Promise<Feedback> => {
    const response = await apiClient.post("/lowcode/feedback", data);
    return response.data;
  },

  update: async (id: string, data: Partial<Feedback>): Promise<Feedback> => {
    const response = await apiClient.put(`/lowcode/feedback/${id}`, data);
    return response.data;
  },

  addComment: async (id: string, content: string): Promise<FeedbackComment> => {
    const response = await apiClient.post(`/lowcode/feedback/${id}/comments`, { content });
    return response.data;
  },
};

// Journal API
export const journalApi = {
  list: async (
    operationId: string,
    params?: {
      page?: number;
      page_size?: number;
      change_type?: string;
    }
  ): Promise<PaginatedResponse<JournalEntry>> => {
    const response = await apiClient.get(
      `/lowcode/operations/${operationId}/journal`,
      { params }
    );
    // Backend returns Vec<DevJournalEntry>, normalize to paginated shape
    const data = response.data;
    if (Array.isArray(data)) {
      return { items: data, total: data.length, page: 1, page_size: data.length, total_pages: 1 };
    }
    return data;
  },

  rollback: async (operationId: string, version: number): Promise<JournalEntry> => {
    const response = await apiClient.post(
      `/lowcode/operations/${operationId}/journal/rollback/${version}`
    );
    return response.data;
  },
};

// Files API
export const filesApi = {
  upload: async (file: File, context?: string): Promise<FileAttachment> => {
    const formData = new FormData();
    formData.append("file", file);
    if (context) formData.append("context", context);
    const response = await apiClient.post("/lowcode/files/upload", formData, {
      headers: { "Content-Type": "multipart/form-data" },
    });
    return response.data;
  },

  download: async (id: string): Promise<Blob> => {
    const response = await apiClient.get(`/lowcode/files/${id}`, {
      responseType: "blob",
    });
    return response.data;
  },
};

// Navigation API
export const navigationApi = {
  list: async (): Promise<NavigationItem[]> => {
    const response = await apiClient.get("/lowcode/navigation");
    return response.data;
  },

  create: async (data: Partial<NavigationItem>): Promise<NavigationItem> => {
    const response = await apiClient.post("/lowcode/navigation", data);
    return response.data;
  },

  update: async (id: string, data: Partial<NavigationItem>): Promise<NavigationItem> => {
    const response = await apiClient.put(`/lowcode/navigation/${id}`, data);
    return response.data;
  },

  delete: async (id: string): Promise<void> => {
    await apiClient.delete(`/lowcode/navigation/${id}`);
  },

  reorder: async (items: { id: string; sort_order: number; parent_id: string | null }[]): Promise<void> => {
    await apiClient.put("/lowcode/navigation/reorder", { items });
  },
};

// User Profile API
export const userProfileApi = {
  getMyProfile: async (): Promise<{
    platform_roles: string[];
    projects: { id: string; name: string; role: string }[];
  }> => {
    const response = await apiClient.get("/lowcode/user/me");
    return response.data;
  },
};

// Role Management API (Admin)
export const roleManagementApi = {
  listUsersWithRoles: async (params?: {
    search?: string;
    page?: number;
    per_page?: number;
  }) => {
    const response = await apiClient.get("/lowcode/users", { params });
    return response.data;
  },

  assignRole: async (userId: string, roleName: string): Promise<void> => {
    await apiClient.post(`/lowcode/users/${userId}/roles`, { role_name: roleName });
  },

  revokeRole: async (userId: string, roleName: string): Promise<void> => {
    await apiClient.delete(`/lowcode/users/${userId}/roles/${roleName}`);
  },

  listProjectDevelopers: async (projectId: string) => {
    const response = await apiClient.get(`/lowcode/projects/${projectId}/developers`);
    return response.data;
  },

  assignProjectDeveloper: async (
    projectId: string,
    userId: string,
    role?: string
  ): Promise<void> => {
    await apiClient.post(`/lowcode/projects/${projectId}/developers`, {
      user_id: userId,
      role: role || "DEVELOPER",
    });
  },

  removeProjectDeveloper: async (projectId: string, userId: string): Promise<void> => {
    await apiClient.delete(`/lowcode/projects/${projectId}/developers/${userId}`);
  },
};

// Notifications API
export const notificationsApi = {
  list: async (params?: {
    page?: number;
    page_size?: number;
    unread_only?: boolean;
  }): Promise<PaginatedResponse<Notification>> => {
    const response = await apiClient.get("/lowcode/notifications", { params });
    return response.data;
  },

  markRead: async (id: string): Promise<void> => {
    await apiClient.put(`/lowcode/notifications/${id}/read`);
  },

  markAllRead: async (): Promise<void> => {
    await apiClient.put("/lowcode/notifications/read-all");
  },
};

// List API
export const listApi = {
  getDefinition: async (operationId: string): Promise<ListDefinition> => {
    const response = await apiClient.get(
      `/lowcode/operations/${operationId}/list`
    );
    return response.data;
  },

  saveDefinition: async (
    operationId: string,
    data: unknown
  ): Promise<ListDefinition> => {
    const response = await apiClient.put(
      `/lowcode/operations/${operationId}/list`,
      data
    );
    return response.data;
  },
};

// Dashboard API
export const dashboardApi = {
  getDefinition: async (
    operationId: string
  ): Promise<DashboardDefinition> => {
    const response = await apiClient.get(
      `/lowcode/operations/${operationId}/dashboard`
    );
    return response.data;
  },

  saveDefinition: async (
    operationId: string,
    data: unknown
  ): Promise<DashboardDefinition> => {
    const response = await apiClient.put(
      `/lowcode/operations/${operationId}/dashboard`,
      data
    );
    return response.data;
  },
};

// List Executor API (user-facing list query)
export const listExecutorApi = {
  query: async (
    code: string,
    params?: Record<string, unknown>
  ): Promise<PaginatedResponse<Record<string, unknown>>> => {
    const response = await apiClient.get(`/lowcode/exec/${code}/list-query`, {
      params,
    });
    return response.data;
  },
};

// Import / Export API
export const importExportApi = {
  bulkImport: async (
    code: string,
    records: Record<string, unknown>[]
  ): Promise<{ inserted: number; errors: string[] }> => {
    const response = await apiClient.post(`/lowcode/exec/${code}/bulk-import`, {
      records,
    });
    return response.data;
  },
};

// Workflow API
export const workflowApi = {
  transition: async (
    code: string,
    recordId: string,
    data: {
      target_status: string;
      status_field?: string;
      comment?: string;
    }
  ): Promise<void> => {
    await apiClient.post(
      `/lowcode/exec/${code}/data/${recordId}/transition`,
      data
    );
  },
};

// Document Flow API
export const documentFlowApi = {
  getFlow: async (sourceType: string, sourceId: string) => {
    const response = await apiClient.get(
      `/lowcode/document-flow/${sourceType}/${sourceId}`
    );
    return response.data;
  },
};

// Module Operations API (user-facing)
export const moduleOpsApi = {
  listByModule: async (module: string): Promise<ModuleOperationSummary[]> => {
    const response = await apiClient.get(`/lowcode/modules/${module}/operations`);
    return response.data;
  },
};

// Operation Buttons API
export const buttonsApi = {
  get: async (operationId: string): Promise<OperationButton[]> => {
    const response = await apiClient.get(`/lowcode/operations/${operationId}/buttons`);
    return response.data;
  },

  save: async (operationId: string, buttons: Partial<OperationButton>[]): Promise<OperationButton[]> => {
    const response = await apiClient.put(`/lowcode/operations/${operationId}/buttons`, { buttons });
    return response.data;
  },

  getByCode: async (code: string): Promise<OperationButton[]> => {
    const response = await apiClient.get(`/lowcode/exec/${code}/buttons`);
    return response.data;
  },
};
