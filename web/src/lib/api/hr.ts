import apiClient, { type PaginatedResponse } from "./client";

export interface Employee {
  id: string;
  employee_number: string;
  first_name: string;
  last_name: string;
  email: string;
  phone: string;
  department_id: string;
  department_name: string;
  position: string;
  hire_date: string;
  status: string;
  manager_id: string | null;
  salary: number;
  currency: string;
  created_at: string;
}

export interface Department {
  id: string;
  name: string;
  code: string;
  manager_id: string | null;
  manager_name: string | null;
  employee_count: number;
}

export interface AttendanceRecord {
  id: string;
  employee_id: string;
  employee_name: string;
  date: string;
  check_in: string | null;
  check_out: string | null;
  status: string;
  hours_worked: number;
  overtime_hours: number;
}

export const hrApi = {
  getEmployees: async (params?: {
    page?: number;
    page_size?: number;
    search?: string;
    department_id?: string;
    status?: string;
  }): Promise<PaginatedResponse<Employee>> => {
    const response = await apiClient.get("/hr/employees", { params });
    return response.data;
  },

  getEmployee: async (id: string): Promise<Employee> => {
    const response = await apiClient.get(`/hr/employees/${id}`);
    return response.data;
  },

  createEmployee: async (data: Partial<Employee>): Promise<Employee> => {
    const response = await apiClient.post("/hr/employees", data);
    return response.data;
  },

  updateEmployee: async (id: string, data: Partial<Employee>): Promise<Employee> => {
    const response = await apiClient.put(`/hr/employees/${id}`, data);
    return response.data;
  },

  getDepartments: async (): Promise<Department[]> => {
    const response = await apiClient.get("/hr/departments");
    return response.data;
  },

  getAttendance: async (params?: {
    page?: number;
    page_size?: number;
    employee_id?: string;
    date_from?: string;
    date_to?: string;
  }): Promise<PaginatedResponse<AttendanceRecord>> => {
    const response = await apiClient.get("/hr/attendance", { params });
    return response.data;
  },

  checkIn: async (employeeId: string): Promise<AttendanceRecord> => {
    const response = await apiClient.post(`/hr/attendance/${employeeId}/check-in`);
    return response.data;
  },

  checkOut: async (employeeId: string): Promise<AttendanceRecord> => {
    const response = await apiClient.post(`/hr/attendance/${employeeId}/check-out`);
    return response.data;
  },
};
