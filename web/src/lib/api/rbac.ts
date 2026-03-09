import apiClient from "./client";

// --- Types ---

export interface Role {
  id: string;
  name: string;
  description: string | null;
  is_system: boolean;
  parent_id: string | null;
  sort_order: number;
  created_at: string;
}

export interface RoleWithPermissions extends Role {
  permissions: Permission[];
}

export interface Permission {
  id: string;
  module: string;
  action: string;
  description: string | null;
}

export interface UserWithRoles {
  id: string;
  username: string;
  email: string;
  display_name: string | null;
  is_active: boolean;
  roles: Role[];
}

export interface MeResponse {
  id: string;
  username: string;
  email: string;
  display_name: string | null;
  roles: string[];
  permissions: string[];
}

// --- API ---

export const rbacApi = {
  // Roles
  listRoles: async (): Promise<Role[]> => {
    const res = await apiClient.get("/auth/roles");
    return res.data;
  },

  getRole: async (id: string): Promise<RoleWithPermissions> => {
    const res = await apiClient.get(`/auth/roles/${id}`);
    return res.data;
  },

  createRole: async (data: {
    name: string;
    description?: string;
    parent_id?: string | null;
  }): Promise<Role> => {
    const res = await apiClient.post("/auth/roles", data);
    return res.data;
  },

  updateRole: async (
    id: string,
    data: {
      name?: string;
      description?: string | null;
      parent_id?: string | null;
      sort_order?: number;
    }
  ): Promise<Role> => {
    const res = await apiClient.put(`/auth/roles/${id}`, data);
    return res.data;
  },

  deleteRole: async (id: string): Promise<void> => {
    await apiClient.delete(`/auth/roles/${id}`);
  },

  // Role permissions
  getRolePermissions: async (id: string): Promise<Permission[]> => {
    const res = await apiClient.get(`/auth/roles/${id}/permissions`);
    return res.data;
  },

  setRolePermissions: async (
    id: string,
    permissionIds: string[]
  ): Promise<Permission[]> => {
    const res = await apiClient.put(`/auth/roles/${id}/permissions`, {
      permission_ids: permissionIds,
    });
    return res.data;
  },

  // All permissions
  listPermissions: async (): Promise<Permission[]> => {
    const res = await apiClient.get("/auth/permissions");
    return res.data;
  },

  // Users
  listUsersWithRoles: async (params?: {
    search?: string;
    page?: number;
    per_page?: number;
  }): Promise<{ items: UserWithRoles[]; total: number; page: number; page_size: number; total_pages: number }> => {
    const res = await apiClient.get("/auth/users", { params });
    return res.data;
  },

  assignUserRole: async (
    userId: string,
    roleId: string
  ): Promise<void> => {
    await apiClient.post(`/auth/users/${userId}/roles`, {
      role_id: roleId,
    });
  },

  removeUserRole: async (
    userId: string,
    roleId: string
  ): Promise<void> => {
    await apiClient.delete(`/auth/users/${userId}/roles/${roleId}`);
  },

  // Current user
  getMe: async (): Promise<MeResponse> => {
    const res = await apiClient.get("/auth/me");
    return res.data;
  },
};
