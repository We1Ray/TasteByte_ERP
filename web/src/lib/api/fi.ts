import apiClient, { type PaginatedResponse } from "./client";

export interface Account {
  id: string;
  account_number: string;
  name: string;
  account_type: string;
  parent_id: string | null;
  account_group_id: string | null;
  currency?: string;
  is_active: boolean;
  balance?: number | null;
  created_at: string;
}

export interface JournalEntry {
  id: string;
  document_number: string;
  posting_date: string;
  document_date: string;
  description: string | null;
  reference: string | null;
  status: string;
  total_debit?: number | null;
  total_credit?: number | null;
  line_items?: JournalLineItem[];
  created_at: string;
}

export interface JournalLineItem {
  id: string;
  account_id: string;
  account_number: string;
  account_name: string;
  debit: number;
  credit: number;
  description: string;
  cost_center_id?: string;
}

export interface CreateJournalEntryInput {
  posting_date: string;
  document_date: string;
  reference?: string;
  description?: string;
  line_items: {
    account_id: string;
    debit: number;
    credit: number;
    description?: string;
  }[];
}

export interface TrialBalance {
  accounts: TrialBalanceEntry[];
  total_debit: number;
  total_credit: number;
  as_of_date: string;
}

export interface TrialBalanceEntry {
  account_id: string;
  account_number: string;
  account_name: string;
  account_type: string;
  debit_balance: number;
  credit_balance: number;
}

export interface IncomeStatement {
  start_date: string;
  end_date: string;
  revenue: { account_number: string; account_name: string; amount: number }[];
  expenses: { account_number: string; account_name: string; amount: number }[];
  total_revenue: number;
  total_expenses: number;
  net_income: number;
}

export interface BalanceSheet {
  as_of_date: string;
  assets: { account_number: string; account_name: string; amount: number }[];
  liabilities: { account_number: string; account_name: string; amount: number }[];
  equity: { account_number: string; account_name: string; amount: number }[];
  total_assets: number;
  total_liabilities: number;
  total_equity: number;
}

export interface AgingEntry {
  id: string;
  document_number: string;
  party_name: string;
  due_date: string;
  amount: number;
  days_overdue: number;
  aging_bucket: string;
}

export interface AgingReport {
  entries: AgingEntry[];
  total_amount: number;
  buckets: { bucket: string; amount: number }[];
}

export const fiApi = {
  getAccounts: async (params?: {
    page?: number;
    page_size?: number;
    search?: string;
    account_type?: string;
  }): Promise<PaginatedResponse<Account>> => {
    const response = await apiClient.get("/fi/accounts", { params });
    return response.data;
  },

  getAccount: async (id: string): Promise<Account> => {
    const response = await apiClient.get(`/fi/accounts/${id}`);
    return response.data;
  },

  createAccount: async (data: Partial<Account>): Promise<Account> => {
    const response = await apiClient.post("/fi/accounts", data);
    return response.data;
  },

  updateAccount: async (id: string, data: Partial<Account>): Promise<Account> => {
    const response = await apiClient.put(`/fi/accounts/${id}`, data);
    return response.data;
  },

  getJournalEntries: async (params?: {
    page?: number;
    page_size?: number;
    search?: string;
    status?: string;
    date_from?: string;
    date_to?: string;
  }): Promise<PaginatedResponse<JournalEntry>> => {
    const response = await apiClient.get("/fi/journal-entries", { params });
    return response.data;
  },

  getJournalEntry: async (id: string): Promise<JournalEntry> => {
    const response = await apiClient.get(`/fi/journal-entries/${id}`);
    return response.data;
  },

  createJournalEntry: async (data: CreateJournalEntryInput): Promise<JournalEntry> => {
    const response = await apiClient.post("/fi/journal-entries", data);
    return response.data;
  },

  postJournalEntry: async (id: string): Promise<JournalEntry> => {
    const response = await apiClient.post(`/fi/journal-entries/${id}/post`);
    return response.data;
  },

  getTrialBalance: async (params?: {
    as_of_date?: string;
  }): Promise<TrialBalance> => {
    const response = await apiClient.get("/fi/reports/trial-balance", { params });
    return response.data;
  },

  getIncomeStatement: async (params: {
    start_date: string;
    end_date: string;
  }): Promise<IncomeStatement> => {
    const response = await apiClient.get("/fi/reports/income-statement", { params });
    return response.data;
  },

  getBalanceSheet: async (params: {
    as_of_date: string;
  }): Promise<BalanceSheet> => {
    const response = await apiClient.get("/fi/reports/balance-sheet", { params });
    return response.data;
  },

  getArAging: async (): Promise<AgingReport> => {
    const response = await apiClient.get("/fi/reports/ar-aging");
    return response.data;
  },

  getApAging: async (): Promise<AgingReport> => {
    const response = await apiClient.get("/fi/reports/ap-aging");
    return response.data;
  },
};
