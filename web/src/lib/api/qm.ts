import apiClient, { type PaginatedResponse } from "./client";

export interface InspectionLot {
  id: string;
  lot_number: string;
  material_id: string;
  material_number: string;
  material_name: string;
  inspection_type: string;
  origin: string;
  quantity: number;
  unit: string;
  status: string;
  inspector_id: string | null;
  inspector_name: string | null;
  planned_date: string;
  completed_date: string | null;
  created_at: string;
}

export interface InspectionResult {
  id: string;
  lot_id: string;
  characteristic: string;
  specification: string;
  measured_value: string;
  unit: string;
  is_passed: boolean;
  remarks: string;
}

export interface QualityNotification {
  id: string;
  notification_number: string;
  type: string;
  material_id: string;
  material_name: string;
  description: string;
  priority: string;
  status: string;
  reported_by: string;
  assigned_to: string | null;
  created_at: string;
  closed_at: string | null;
}

export const qmApi = {
  getInspectionLots: async (params?: {
    page?: number;
    page_size?: number;
    search?: string;
    status?: string;
  }): Promise<PaginatedResponse<InspectionLot>> => {
    const response = await apiClient.get("/qm/inspection-lots", { params });
    return response.data;
  },

  getInspectionLot: async (id: string): Promise<InspectionLot> => {
    const response = await apiClient.get(`/qm/inspection-lots/${id}`);
    return response.data;
  },

  getInspectionResults: async (lotId: string): Promise<InspectionResult[]> => {
    const response = await apiClient.get(`/qm/inspection-lots/${lotId}/results`);
    return response.data;
  },

  recordInspectionResult: async (
    lotId: string,
    data: Partial<InspectionResult>
  ): Promise<InspectionResult> => {
    const response = await apiClient.post("/qm/inspection-results", { ...data, inspection_lot_id: lotId });
    return response.data;
  },

  completeInspection: async (lotId: string, decision: string): Promise<InspectionLot> => {
    const response = await apiClient.post(`/qm/inspection-lots/${lotId}/complete`, { decision });
    return response.data;
  },

  getQualityNotifications: async (params?: {
    page?: number;
    page_size?: number;
    search?: string;
    status?: string;
    type?: string;
  }): Promise<PaginatedResponse<QualityNotification>> => {
    const response = await apiClient.get("/qm/notifications", { params });
    return response.data;
  },

  createQualityNotification: async (
    data: Partial<QualityNotification>
  ): Promise<QualityNotification> => {
    const response = await apiClient.post("/qm/notifications", data);
    return response.data;
  },
};
