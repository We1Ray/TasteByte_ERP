import apiClient, { type PaginatedResponse } from "./client";

export interface Customer {
  id: string;
  customer_number: string;
  name: string;
  email: string;
  phone: string;
  address: string;
  city: string;
  country: string;
  credit_limit: number;
  payment_terms: string;
  is_active: boolean;
  created_at: string;
}

export interface SalesOrder {
  id: string;
  order_number: string;
  customer_id: string;
  customer_name?: string;
  order_date: string;
  delivery_date?: string | null;
  requested_delivery_date?: string | null;
  status: string;
  total_amount: number;
  currency: string;
  notes?: string | null;
  items?: SalesOrderItem[];
  created_at: string;
}

export interface SalesOrderItem {
  id: string;
  material_id: string;
  material_number: string;
  material_name: string;
  quantity: number;
  unit: string;
  unit_price: number;
  total_price: number;
  delivered_quantity: number;
}

export interface CreateSalesOrderInput {
  customer_id: string;
  order_date: string;
  delivery_date?: string;
  notes?: string;
  items: {
    material_id: string;
    quantity: number;
    unit_price: number;
    uom_id?: string;
  }[];
}

export interface Delivery {
  id: string;
  delivery_number: string;
  sales_order_id: string;
  delivery_date: string;
  status: string;
  items: DeliveryItem[];
}

export interface DeliveryItem {
  material_id: string;
  material_name: string;
  quantity: number;
  unit: string;
}

export interface Invoice {
  id: string;
  invoice_number: string;
  sales_order_id: string;
  delivery_id?: string | null;
  customer_id: string;
  customer_name?: string;
  invoice_date: string;
  due_date: string;
  status: string;
  total_amount: number;
  paid_amount?: number | null;
  currency?: string;
  created_at: string;
}

export interface SalesSummary {
  total_revenue?: number | null;
  total_orders?: number | null;
  average_order_value?: number | null;
  items?: { date: string; revenue: number; order_count: number }[];
}

export interface OrderFulfillment {
  total_orders: number;
  fully_delivered: number;
  partially_delivered: number;
  not_delivered: number;
  fulfillment_rate: number;
  orders: { order_number: string; customer_name: string; total_items: number; delivered_items: number; status: string }[];
}

export interface TopCustomer {
  customer_id: string;
  customer_name: string;
  customer_number: string;
  total_revenue: number;
  order_count: number;
}

export const sdApi = {
  getCustomers: async (params?: {
    page?: number;
    page_size?: number;
    search?: string;
  }): Promise<PaginatedResponse<Customer>> => {
    const response = await apiClient.get("/sd/customers", { params });
    return response.data;
  },

  getCustomer: async (id: string): Promise<Customer> => {
    const response = await apiClient.get(`/sd/customers/${id}`);
    return response.data;
  },

  createCustomer: async (data: Partial<Customer>): Promise<Customer> => {
    const response = await apiClient.post("/sd/customers", data);
    return response.data;
  },

  updateCustomer: async (id: string, data: Partial<Customer>): Promise<Customer> => {
    const response = await apiClient.put(`/sd/customers/${id}`, data);
    return response.data;
  },

  getSalesOrders: async (params?: {
    page?: number;
    page_size?: number;
    search?: string;
    status?: string;
  }): Promise<PaginatedResponse<SalesOrder>> => {
    const response = await apiClient.get("/sd/sales-orders", { params });
    return response.data;
  },

  getSalesOrder: async (id: string): Promise<SalesOrder> => {
    const response = await apiClient.get(`/sd/sales-orders/${id}`);
    return response.data;
  },

  createSalesOrder: async (data: CreateSalesOrderInput): Promise<SalesOrder> => {
    const response = await apiClient.post("/sd/sales-orders", data);
    return response.data;
  },

  getDeliveries: async (params?: {
    page?: number;
    page_size?: number;
    sales_order_id?: string;
  }): Promise<PaginatedResponse<Delivery>> => {
    const response = await apiClient.get("/sd/deliveries", { params });
    return response.data;
  },

  getInvoices: async (params?: {
    page?: number;
    page_size?: number;
    search?: string;
    status?: string;
  }): Promise<PaginatedResponse<Invoice>> => {
    const response = await apiClient.get("/sd/invoices", { params });
    return response.data;
  },

  getInvoice: async (id: string): Promise<Invoice> => {
    const response = await apiClient.get(`/sd/invoices/${id}`);
    return response.data;
  },

  confirmSalesOrder: async (id: string): Promise<SalesOrder> => {
    const response = await apiClient.post(`/sd/sales-orders/${id}/confirm`);
    return response.data;
  },

  getSalesSummary: async (params: {
    start_date: string;
    end_date: string;
  }): Promise<SalesSummary> => {
    const response = await apiClient.get("/sd/reports/sales-summary", { params });
    return response.data;
  },

  getOrderFulfillment: async (): Promise<OrderFulfillment> => {
    const response = await apiClient.get("/sd/reports/order-fulfillment");
    return response.data;
  },

  getTopCustomers: async (params?: { limit?: number }): Promise<TopCustomer[]> => {
    const response = await apiClient.get("/sd/reports/top-customers", { params });
    return response.data;
  },
};
