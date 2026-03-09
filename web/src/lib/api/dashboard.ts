import apiClient from "./client";

export interface DashboardKpis {
  total_revenue: number;
  total_order_count: number;
  total_inventory_quantity: number;
  pending_production_orders: number;
  open_ar_amount: number;
  open_ap_amount: number;
}

export interface MonthlyRevenue {
  month: string;
  revenue: number;
  costs: number;
}

export interface MonthlyOrders {
  month: string;
  orders: number;
  delivered: number;
}

export interface DashboardCharts {
  monthly_revenue: MonthlyRevenue[];
  monthly_orders: MonthlyOrders[];
}

export const dashboardApi = {
  getKpis: async (): Promise<DashboardKpis> => {
    const response = await apiClient.get("/dashboard/kpis");
    return response.data;
  },

  getCharts: async (): Promise<DashboardCharts> => {
    const response = await apiClient.get("/dashboard/charts");
    return response.data;
  },
};
