import apiClient, { type PaginatedResponse } from "./client";

export interface Warehouse {
  id: string;
  warehouse_number: string;
  name: string;
  address: string;
  city: string;
  country: string;
  is_active: boolean;
  storage_bin_count: number;
  total_capacity: number;
  used_capacity: number;
  created_at: string;
}

export interface StorageBin {
  id: string;
  warehouse_id: string;
  bin_number: string;
  zone: string;
  bin_type: string;
  capacity: number;
  current_stock: number;
  is_active: boolean;
}

export interface TransferOrder {
  id: string;
  transfer_number: string;
  from_warehouse_id: string;
  from_warehouse_name: string;
  to_warehouse_id: string;
  to_warehouse_name: string;
  status: string;
  items: TransferItem[];
  created_at: string;
}

export interface TransferItem {
  material_id: string;
  material_name: string;
  quantity: number;
  unit: string;
  from_bin: string;
  to_bin: string;
}

export interface StockCount {
  id: string;
  count_number: string;
  warehouse_id: string;
  warehouse_name: string;
  count_date: string;
  status: string;
  items: StockCountItem[];
}

export interface StockCountItem {
  material_id: string;
  material_name: string;
  bin_number: string;
  system_quantity: number;
  counted_quantity: number;
  difference: number;
  unit: string;
}

export interface CreateWarehouseInput {
  code: string;
  name: string;
  address?: string;
  warehouse_type?: string;
}

export const wmApi = {
  createWarehouse: async (data: CreateWarehouseInput): Promise<Warehouse> => {
    const response = await apiClient.post("/wm/warehouses", data);
    return response.data;
  },

  getWarehouses: async (params?: {
    page?: number;
    page_size?: number;
    search?: string;
  }): Promise<PaginatedResponse<Warehouse>> => {
    const response = await apiClient.get("/wm/warehouses", { params });
    return response.data;
  },

  getWarehouse: async (id: string): Promise<Warehouse> => {
    const response = await apiClient.get(`/wm/warehouses/${id}`);
    return response.data;
  },

  getStorageBins: async (params?: {
    page?: number;
    page_size?: number;
    warehouse_id?: string;
    zone?: string;
  }): Promise<PaginatedResponse<StorageBin>> => {
    const response = await apiClient.get("/wm/storage-bins", { params });
    return response.data;
  },

  getTransferOrders: async (params?: {
    page?: number;
    page_size?: number;
    status?: string;
  }): Promise<PaginatedResponse<TransferOrder>> => {
    const response = await apiClient.get("/wm/stock-transfers", { params });
    return response.data;
  },

  createTransferOrder: async (data: Partial<TransferOrder>): Promise<TransferOrder> => {
    const response = await apiClient.post("/wm/stock-transfers", data);
    return response.data;
  },

  getStockCounts: async (params?: {
    page?: number;
    page_size?: number;
    warehouse_id?: string;
    status?: string;
  }): Promise<PaginatedResponse<StockCount>> => {
    const response = await apiClient.get("/wm/stock-counts", { params });
    return response.data;
  },

  createStockCount: async (data: Partial<StockCount>): Promise<StockCount> => {
    const response = await apiClient.post("/wm/stock-counts", data);
    return response.data;
  },
};
