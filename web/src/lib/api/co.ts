import apiClient, { type PaginatedResponse } from "./client";

export interface CostCenter {
  id: string;
  cost_center_number: string;
  name: string;
  description: string;
  department_id: string | null;
  responsible_person: string;
  category: string;
  is_active: boolean;
  actual_costs: number;
  planned_costs: number;
  currency: string;
  created_at: string;
}

export interface ProfitCenter {
  id: string;
  profit_center_number: string;
  name: string;
  description: string;
  responsible_person: string;
  is_active: boolean;
  revenue: number;
  costs: number;
  profit: number;
  currency: string;
  created_at: string;
}

export interface InternalOrder {
  id: string;
  order_number: string;
  description: string;
  order_type: string;
  status: string;
  cost_center_id: string | null;
  responsible_person: string;
  budget: number;
  actual_costs: number;
  currency: string;
  start_date: string;
  end_date: string | null;
  created_at: string;
}

export const coApi = {
  getCostCenters: async (params?: {
    page?: number;
    page_size?: number;
    search?: string;
  }): Promise<PaginatedResponse<CostCenter>> => {
    const response = await apiClient.get("/co/cost-centers", { params });
    return response.data;
  },

  getCostCenter: async (id: string): Promise<CostCenter> => {
    const response = await apiClient.get(`/co/cost-centers/${id}`);
    return response.data;
  },

  createCostCenter: async (data: Partial<CostCenter>): Promise<CostCenter> => {
    const response = await apiClient.post("/co/cost-centers", data);
    return response.data;
  },

  getProfitCenters: async (params?: {
    page?: number;
    page_size?: number;
    search?: string;
  }): Promise<PaginatedResponse<ProfitCenter>> => {
    const response = await apiClient.get("/co/profit-centers", { params });
    return response.data;
  },

  getProfitCenter: async (id: string): Promise<ProfitCenter> => {
    const response = await apiClient.get(`/co/profit-centers/${id}`);
    return response.data;
  },

  getInternalOrders: async (params?: {
    page?: number;
    page_size?: number;
    search?: string;
    status?: string;
  }): Promise<PaginatedResponse<InternalOrder>> => {
    const response = await apiClient.get("/co/internal-orders", { params });
    return response.data;
  },

  getInternalOrder: async (id: string): Promise<InternalOrder> => {
    const response = await apiClient.get(`/co/internal-orders/${id}`);
    return response.data;
  },

  createInternalOrder: async (data: Partial<InternalOrder>): Promise<InternalOrder> => {
    const response = await apiClient.post("/co/internal-orders", data);
    return response.data;
  },
};
