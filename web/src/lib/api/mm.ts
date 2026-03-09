import apiClient, { type PaginatedResponse } from "./client";

export interface Material {
  id: string;
  material_number: string;
  name: string;
  description: string;
  material_type: string;
  material_group: string;
  base_unit: string;
  weight: number | null;
  weight_unit: string | null;
  price: number;
  currency: string;
  min_stock: number;
  max_stock: number;
  reorder_point: number;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface StockOverview {
  material_id: string;
  material_number: string;
  material_name: string;
  warehouse_id: string;
  warehouse_name: string;
  storage_bin: string;
  quantity: number;
  unit: string;
  value: number;
}

export interface PurchaseOrder {
  id: string;
  po_number: string;
  vendor_id: string;
  vendor_name: string;
  order_date: string;
  delivery_date: string;
  status: string;
  total_amount: number;
  currency: string;
  items: PurchaseOrderItem[];
  created_at: string;
}

export interface PurchaseOrderItem {
  id: string;
  material_id: string;
  material_number: string;
  material_name: string;
  quantity: number;
  unit: string;
  unit_price: number;
  total_price: number;
  received_quantity: number;
}

export interface StockValuation {
  items: { material_number: string; material_name: string; quantity: number; unit: string; unit_price: number; total_value: number; warehouse_name: string }[];
  total_value: number;
}

export interface MovementSummary {
  movements: { material_number: string; material_name: string; movement_type: string; total_quantity: number; unit: string }[];
}

export interface SlowMovingItem {
  material_number: string;
  material_name: string;
  quantity: number;
  unit: string;
  last_movement_date: string | null;
  days_since_movement: number;
  value: number;
}

export const mmApi = {
  getMaterials: async (params?: {
    page?: number;
    page_size?: number;
    search?: string;
    material_type?: string;
    material_group?: string;
  }): Promise<PaginatedResponse<Material>> => {
    const response = await apiClient.get("/mm/materials", { params });
    return response.data;
  },

  getMaterial: async (id: string): Promise<Material> => {
    const response = await apiClient.get(`/mm/materials/${id}`);
    return response.data;
  },

  createMaterial: async (data: Partial<Material>): Promise<Material> => {
    const response = await apiClient.post("/mm/materials", data);
    return response.data;
  },

  updateMaterial: async (id: string, data: Partial<Material>): Promise<Material> => {
    const response = await apiClient.put(`/mm/materials/${id}`, data);
    return response.data;
  },

  deleteMaterial: async (id: string): Promise<void> => {
    await apiClient.delete(`/mm/materials/${id}`);
  },

  getStock: async (params?: {
    page?: number;
    page_size?: number;
    material_id?: string;
    warehouse_id?: string;
  }): Promise<PaginatedResponse<StockOverview>> => {
    const response = await apiClient.get("/mm/plant-stock", { params });
    return response.data;
  },

  getPurchaseOrders: async (params?: {
    page?: number;
    page_size?: number;
    search?: string;
    status?: string;
  }): Promise<PaginatedResponse<PurchaseOrder>> => {
    const response = await apiClient.get("/mm/purchase-orders", { params });
    return response.data;
  },

  getPurchaseOrder: async (id: string): Promise<PurchaseOrder> => {
    const response = await apiClient.get(`/mm/purchase-orders/${id}`);
    return response.data;
  },

  createPurchaseOrder: async (data: Partial<PurchaseOrder>): Promise<PurchaseOrder> => {
    const response = await apiClient.post("/mm/purchase-orders", data);
    return response.data;
  },

  updatePurchaseOrder: async (id: string, data: Partial<PurchaseOrder>): Promise<PurchaseOrder> => {
    const response = await apiClient.put(`/mm/purchase-orders/${id}`, data);
    return response.data;
  },

  receivePurchaseOrder: async (poId: string, items: { po_item_id: string; quantity: number; warehouse_id: string }[]): Promise<void> => {
    await apiClient.post(`/mm/purchase-orders/${poId}/receive`, { items });
  },

  postGoodsIssue: async (data: {
    material_id: string;
    warehouse_id: string;
    quantity: number;
    reason: string;
  }): Promise<void> => {
    await apiClient.post("/mm/goods-issue", data);
  },

  getStockValuation: async (): Promise<StockValuation> => {
    const response = await apiClient.get("/mm/reports/stock-valuation");
    return response.data;
  },

  getMovementSummary: async (params: {
    start_date: string;
    end_date: string;
  }): Promise<MovementSummary> => {
    const response = await apiClient.get("/mm/reports/movement-summary", { params });
    return response.data;
  },

  getSlowMoving: async (params?: { days?: number }): Promise<SlowMovingItem[]> => {
    const response = await apiClient.get("/mm/reports/slow-moving", { params });
    return response.data;
  },
};
