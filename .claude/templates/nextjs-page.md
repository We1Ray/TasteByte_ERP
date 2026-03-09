# Next.js ERP Page Template

> Use this template when creating a new ERP module page in the Next.js web app.

## Page Structure per Module

```
web/src/app/(dashboard)/{module}/
├── {resource}/
│   ├── page.tsx              # 列表頁 (Server Component)
│   ├── loading.tsx           # Loading skeleton
│   ├── error.tsx             # Error boundary
│   ├── [id]/
│   │   └── page.tsx          # 詳情頁
│   ├── new/
│   │   └── page.tsx          # 新增頁
│   └── components/
│       ├── {resource}-table.tsx    # Client: Tanstack Table
│       ├── {resource}-form.tsx     # Client: React Hook Form + Zod
│       └── {resource}-filters.tsx  # Client: 篩選器
```

## 1. List Page (Server Component)

```tsx
// app/(dashboard)/{module}/{resource}/page.tsx
import { {Resource}Table } from './components/{resource}-table';

export default function {Resource}Page() {
  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">{Resource} List</h1>
        <Link href="/{module}/{resource}/new">
          <Button>Create {Resource}</Button>
        </Link>
      </div>
      <{Resource}Table />
    </div>
  );
}
```

## 2. Table Component (Client Component)

```tsx
// app/(dashboard)/{module}/{resource}/components/{resource}-table.tsx
'use client';

import { useQuery } from '@tanstack/react-query';
import {
  useReactTable,
  getCoreRowModel,
  getSortedRowModel,
  getPaginationRowModel,
  flexRender,
  type ColumnDef,
} from '@tanstack/react-table';
import { apiClient } from '@/lib/api/client';

const columns: ColumnDef<{Resource}>[] = [
  {
    accessorKey: 'document_number',
    header: 'Document No.',
  },
  {
    accessorKey: 'status',
    header: 'Status',
    cell: ({ row }) => <StatusBadge status={row.original.status} />,
  },
  {
    accessorKey: 'total_amount',
    header: 'Amount',
    cell: ({ row }) => formatCurrency(row.original.total_amount),
  },
  {
    accessorKey: 'posting_date',
    header: 'Date',
    cell: ({ row }) => formatDate(row.original.posting_date),
  },
  {
    id: 'actions',
    cell: ({ row }) => (
      <Link href={`/{module}/{resource}/${row.original.id}`}>
        View
      </Link>
    ),
  },
];

export function {Resource}Table() {
  const { data, isLoading } = useQuery({
    queryKey: ['{module}', '{resource}'],
    queryFn: () => apiClient<PaginatedResponse<{Resource}>>('/{module}/{resource}'),
  });

  const table = useReactTable({
    data: data?.items ?? [],
    columns,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
  });

  if (isLoading) return <TableSkeleton />;

  return (
    <div>
      <Table>
        <TableHeader>
          {table.getHeaderGroups().map((headerGroup) => (
            <TableRow key={headerGroup.id}>
              {headerGroup.headers.map((header) => (
                <TableHead key={header.id}>
                  {flexRender(header.column.columnDef.header, header.getContext())}
                </TableHead>
              ))}
            </TableRow>
          ))}
        </TableHeader>
        <TableBody>
          {table.getRowModel().rows.map((row) => (
            <TableRow key={row.id}>
              {row.getVisibleCells().map((cell) => (
                <TableCell key={cell.id}>
                  {flexRender(cell.column.columnDef.cell, cell.getContext())}
                </TableCell>
              ))}
            </TableRow>
          ))}
        </TableBody>
      </Table>
      <DataTablePagination table={table} />
    </div>
  );
}
```

## 3. Form Component (Client Component)

```tsx
// app/(dashboard)/{module}/{resource}/components/{resource}-form.tsx
'use client';

import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { useMutation, useQueryClient } from '@tanstack/react-query';

const {resource}Schema = z.object({
  // Define fields with Zod validation
  // Use z.number() for amounts, never z.string() for monetary values
});

type {Resource}FormData = z.infer<typeof {resource}Schema>;

export function {Resource}Form({ defaultValues }: { defaultValues?: {Resource}FormData }) {
  const queryClient = useQueryClient();
  const form = useForm<{Resource}FormData>({
    resolver: zodResolver({resource}Schema),
    defaultValues,
  });

  const mutation = useMutation({
    mutationFn: (data: {Resource}FormData) =>
      apiClient('/{module}/{resource}', {
        method: defaultValues ? 'PUT' : 'POST',
        body: JSON.stringify(data),
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['{module}', '{resource}'] });
      // redirect or show success
    },
  });

  return (
    <form onSubmit={form.handleSubmit((data) => mutation.mutate(data))}>
      {/* Form fields using shadcn/ui Form components */}
      <Button type="submit" disabled={mutation.isPending}>
        {mutation.isPending ? 'Saving...' : 'Save'}
      </Button>
    </form>
  );
}
```

## 4. Loading Skeleton

```tsx
// app/(dashboard)/{module}/{resource}/loading.tsx
export default function Loading() {
  return <TableSkeleton rows={10} columns={5} />;
}
```

## 5. Error Boundary

```tsx
// app/(dashboard)/{module}/{resource}/error.tsx
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
      <h2 className="text-xl font-semibold">Failed to load data</h2>
      <p className="text-muted-foreground">{error.message}</p>
      <Button onClick={reset}>Try again</Button>
    </div>
  );
}
```

## Checklist

- [ ] List page is a Server Component (no 'use client')
- [ ] Table/Form components are Client Components ('use client')
- [ ] Uses Tanstack Query for data fetching
- [ ] Uses Tanstack Table for data display
- [ ] Uses React Hook Form + Zod for forms
- [ ] Has loading.tsx skeleton
- [ ] Has error.tsx boundary
- [ ] Monetary values formatted correctly
- [ ] Status displayed with colored badges
- [ ] Pagination implemented
