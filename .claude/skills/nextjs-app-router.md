# Next.js 15 App Router 開發指南

## 目錄
1. [Server vs Client Components](#server-vs-client-components)
2. [Route Groups](#route-groups)
3. [Data Fetching](#data-fetching)
4. [Forms & Mutations](#forms--mutations)
5. [Layout & Loading](#layout--loading)

---

## Server vs Client Components

### Server Components (預設)
```tsx
// app/(dashboard)/fi/accounts/page.tsx
// 預設就是 Server Component，不需要 'use client'
import { AccountsTable } from './components/accounts-table';

export default async function AccountsPage() {
  // 可以直接做 async 操作
  return (
    <div className="space-y-4">
      <h1 className="text-2xl font-bold">Chart of Accounts</h1>
      <AccountsTable />
    </div>
  );
}
```

### Client Components (需要互動)
```tsx
// app/(dashboard)/fi/accounts/components/accounts-table.tsx
'use client';

import { useQuery } from '@tanstack/react-query';
import { useReactTable, getCoreRowModel, flexRender } from '@tanstack/react-table';

export function AccountsTable() {
  const { data, isLoading } = useQuery({
    queryKey: ['fi', 'accounts'],
    queryFn: () => apiClient<PaginatedResponse<Account>>('/fi/accounts'),
  });

  if (isLoading) return <TableSkeleton />;

  return (
    <Table>
      {/* ... Tanstack Table render */}
    </Table>
  );
}
```

### 判斷原則
| 情境 | 使用 |
|------|------|
| 靜態內容、佈局 | Server Component |
| 資料展示（no interaction） | Server Component |
| 表單、按鈕互動 | Client Component |
| 使用 hooks (useState, useEffect) | Client Component |
| 使用 Tanstack Query/Table | Client Component |
| 使用 Zustand | Client Component |
| 使用瀏覽器 API | Client Component |

---

## Route Groups

### ERP 模組路由組織
```
app/
├── layout.tsx                    # Root layout (providers)
├── page.tsx                      # Landing / redirect
├── (auth)/                       # Auth group (no sidebar)
│   ├── layout.tsx                # Auth-specific layout
│   ├── login/page.tsx
│   └── forgot-password/page.tsx
├── (dashboard)/                  # Dashboard group (with sidebar)
│   ├── layout.tsx                # Dashboard layout (sidebar + header)
│   ├── page.tsx                  # Dashboard home
│   ├── fi/                       # Financial Accounting
│   │   ├── accounts/
│   │   │   ├── page.tsx          # 列表頁
│   │   │   ├── [id]/page.tsx     # 詳情頁
│   │   │   └── new/page.tsx      # 新增頁
│   │   ├── journal-entries/
│   │   └── reports/
│   ├── mm/                       # Materials Management
│   │   ├── materials/
│   │   ├── purchase-orders/
│   │   └── inventory/
│   ├── sd/                       # Sales & Distribution
│   │   ├── customers/
│   │   ├── sales-orders/
│   │   └── invoices/
│   ├── pp/                       # Production Planning
│   ├── hr/                       # Human Resources
│   ├── wm/                       # Warehouse Management
│   ├── qm/                       # Quality Management
│   └── settings/                 # 系統設定
```

### Route Group Layout
```tsx
// app/(dashboard)/layout.tsx
import { Sidebar } from '@/components/layout/sidebar';
import { Header } from '@/components/layout/header';

export default function DashboardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="flex h-screen">
      <Sidebar />
      <div className="flex flex-1 flex-col">
        <Header />
        <main className="flex-1 overflow-auto p-6">
          {children}
        </main>
      </div>
    </div>
  );
}
```

---

## Data Fetching

### Tanstack Query Pattern
```tsx
'use client';

import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';

// Query keys 統一管理
export const queryKeys = {
  salesOrders: {
    all: ['sd', 'sales-orders'] as const,
    detail: (id: string) => ['sd', 'sales-orders', id] as const,
  },
  materials: {
    all: ['mm', 'materials'] as const,
    detail: (id: string) => ['mm', 'materials', id] as const,
  },
};

// 列表查詢
export function useSalesOrders(params?: SalesOrderQuery) {
  return useQuery({
    queryKey: [...queryKeys.salesOrders.all, params],
    queryFn: () => apiClient<PaginatedResponse<SalesOrder>>('/sd/sales-orders', {
      params,
    }),
  });
}

// 詳情查詢
export function useSalesOrder(id: string) {
  return useQuery({
    queryKey: queryKeys.salesOrders.detail(id),
    queryFn: () => apiClient<SalesOrder>(`/sd/sales-orders/${id}`),
    enabled: !!id,
  });
}

// 建立 mutation
export function useCreateSalesOrder() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateSalesOrderInput) =>
      apiClient<SalesOrder>('/sd/sales-orders', {
        method: 'POST',
        body: JSON.stringify(data),
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.salesOrders.all,
      });
    },
  });
}
```

---

## Forms & Mutations

### React Hook Form + Zod
```tsx
'use client';

import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';

const salesOrderSchema = z.object({
  customer_id: z.string().uuid('Invalid customer'),
  order_date: z.string().date(),
  items: z.array(z.object({
    material_id: z.string().uuid(),
    quantity: z.number().positive('Quantity must be positive'),
    unit_price: z.number().nonnegative(),
  })).min(1, 'At least one item required'),
});

type SalesOrderForm = z.infer<typeof salesOrderSchema>;

export function CreateSalesOrderForm() {
  const { register, handleSubmit, formState: { errors } } = useForm<SalesOrderForm>({
    resolver: zodResolver(salesOrderSchema),
  });

  const createMutation = useCreateSalesOrder();

  const onSubmit = (data: SalesOrderForm) => {
    createMutation.mutate(data);
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      {/* form fields */}
    </form>
  );
}
```

---

## Layout & Loading

### Loading States
```tsx
// app/(dashboard)/sd/sales-orders/loading.tsx
export default function Loading() {
  return <TableSkeleton rows={10} columns={6} />;
}
```

### Error Handling
```tsx
// app/(dashboard)/sd/sales-orders/error.tsx
'use client';

export default function Error({
  error,
  reset,
}: {
  error: Error;
  reset: () => void;
}) {
  return (
    <div className="flex flex-col items-center gap-4 p-8">
      <h2 className="text-xl font-semibold">Something went wrong</h2>
      <p className="text-muted-foreground">{error.message}</p>
      <Button onClick={reset}>Try again</Button>
    </div>
  );
}
```

### Providers Setup
```tsx
// app/layout.tsx
import { QueryProvider } from '@/components/providers/query-provider';

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="zh-TW">
      <body>
        <QueryProvider>
          {children}
        </QueryProvider>
      </body>
    </html>
  );
}

// components/providers/query-provider.tsx
'use client';

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { useState } from 'react';

export function QueryProvider({ children }: { children: React.ReactNode }) {
  const [queryClient] = useState(() => new QueryClient({
    defaultOptions: {
      queries: {
        staleTime: 5 * 60 * 1000,  // 5 minutes
        retry: 1,
      },
    },
  }));

  return (
    <QueryClientProvider client={queryClient}>
      {children}
    </QueryClientProvider>
  );
}
```
