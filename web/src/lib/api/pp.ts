import apiClient, { type PaginatedResponse } from "./client";

export interface BillOfMaterial {
  id: string;
  bom_number: string;
  material_id: string;
  material_number?: string;
  material_name?: string;
  name?: string;
  quantity?: number | null;
  unit?: string;
  version?: number;
  status: string;
  valid_from: string | null;
  valid_to: string | null;
  items?: BomItem[];
  created_at: string;
}

export interface BomItem {
  id: string;
  component_material_id: string;
  component_material_number: string;
  component_material_name: string;
  quantity: number;
  unit: string;
  position: number;
}

export interface Routing {
  id: string;
  routing_number: string;
  material_id: string;
  description: string;
  operations: RoutingOperation[];
}

export interface RoutingOperation {
  id: string;
  operation_number: number;
  work_center: string;
  description: string;
  setup_time: number;
  run_time: number;
  time_unit: string;
}

export interface ProductionOrder {
  id: string;
  order_number: string;
  material_id: string;
  material_number?: string;
  material_name?: string;
  bom_id: string;
  routing_id?: string | null;
  quantity?: number | null;
  planned_quantity?: number | null;
  actual_quantity?: number | null;
  unit?: string;
  uom_id?: string | null;
  planned_start: string | null;
  planned_end: string | null;
  actual_start: string | null;
  actual_end: string | null;
  status: string;
  completed_quantity?: number | null;
  created_at: string;
}

export const ppApi = {
  getBoms: async (params?: {
    page?: number;
    page_size?: number;
    search?: string;
  }): Promise<PaginatedResponse<BillOfMaterial>> => {
    const response = await apiClient.get("/pp/boms", { params });
    return response.data;
  },

  getBom: async (id: string): Promise<BillOfMaterial> => {
    const response = await apiClient.get(`/pp/boms/${id}`);
    return response.data;
  },

  createBom: async (data: Partial<BillOfMaterial>): Promise<BillOfMaterial> => {
    const response = await apiClient.post("/pp/boms", data);
    return response.data;
  },

  getRoutings: async (params?: {
    page?: number;
    page_size?: number;
  }): Promise<PaginatedResponse<Routing>> => {
    const response = await apiClient.get("/pp/routings", { params });
    return response.data;
  },

  getProductionOrders: async (params?: {
    page?: number;
    page_size?: number;
    search?: string;
    status?: string;
  }): Promise<PaginatedResponse<ProductionOrder>> => {
    const response = await apiClient.get("/pp/production-orders", { params });
    return response.data;
  },

  getProductionOrder: async (id: string): Promise<ProductionOrder> => {
    const response = await apiClient.get(`/pp/production-orders/${id}`);
    return response.data;
  },

  createProductionOrder: async (data: Partial<ProductionOrder>): Promise<ProductionOrder> => {
    const response = await apiClient.post("/pp/production-orders", data);
    return response.data;
  },

  releaseProductionOrder: async (id: string): Promise<ProductionOrder> => {
    const response = await apiClient.post(`/pp/production-orders/${id}/release`);
    return response.data;
  },

  confirmProductionOrder: async (
    id: string,
    data: { quantity: number }
  ): Promise<ProductionOrder> => {
    const response = await apiClient.post(`/pp/production-orders/${id}/confirm`, data);
    return response.data;
  },

  updateProductionOrderStatus: async (
    id: string,
    data: { status: string }
  ): Promise<ProductionOrder> => {
    const response = await apiClient.put(`/pp/production-orders/${id}/status`, data);
    return response.data;
  },
};
