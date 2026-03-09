"use client";

import { useState, useMemo } from "react";
import { type ColumnDef } from "@tanstack/react-table";
import { useTranslations } from "next-intl";
import { Plus } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { DataTable } from "@/components/ui/data-table";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { SearchBar } from "@/components/forms/search-bar";
import { ExportButton } from "@/components/shared/export-button";
import { PrintButton } from "@/components/shared/print-button";
import { Modal } from "@/components/ui/modal";
import { Input } from "@/components/ui/input";
import { useApiQuery, useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { usePagination } from "@/lib/hooks/use-pagination";
import { sdApi, type Customer } from "@/lib/api/sd";
import { formatCurrency } from "@/lib/utils";

export default function CustomersPage() {
  const t = useTranslations("sd");
  const tc = useTranslations("common");
  const ts = useTranslations("shared");
  const columns = useMemo<ColumnDef<Customer, unknown>[]>(() => [
    {
      accessorKey: "customer_number",
      header: t("customerNo"),
      cell: ({ row }) => (
        <span className="font-medium text-blue-600">{row.original.customer_number}</span>
      ),
    },
    { accessorKey: "name", header: tc("name") },
    { accessorKey: "email", header: t("email") },
    { accessorKey: "city", header: t("city") },
    { accessorKey: "country", header: t("country") },
    {
      accessorKey: "credit_limit",
      header: t("creditLimit"),
      cell: ({ row }) => (
        <span className="font-mono">{formatCurrency(row.original.credit_limit)}</span>
      ),
    },
    {
      accessorKey: "is_active",
      header: tc("status"),
      cell: ({ row }) => (
        <Badge color={row.original.is_active ? "green" : "gray"}>
          {row.original.is_active ? tc("active") : ts("inactive")}
        </Badge>
      ),
    },
  ], [t, tc, ts]);

  const { page, pageSize, goToPage } = usePagination();
  const [search, setSearch] = useState("");
  const [showCreate, setShowCreate] = useState(false);
  const [newCustomer, setNewCustomer] = useState({
    customer_number: "",
    name: "",
    email: "",
    phone: "",
    address: "",
    city: "",
    country: "",
    credit_limit: 0,
    payment_terms: "Net 30",
  });
  const invalidate = useInvalidateQueries();

  const { data, isLoading } = useApiQuery(
    ["sd", "customers", String(page), search],
    () => sdApi.getCustomers({ page, page_size: pageSize, search: search || undefined })
  );

  const createMutation = useApiMutation(
    (data: Partial<Customer>) => sdApi.createCustomer(data),
    {
      onSuccess: () => {
        invalidate(["sd", "customers"]);
        setShowCreate(false);
      },
    }
  );

  return (
    <div>
      <PageHeader
        title={t("customers")}
        description={t("manageCustomers")}
        actions={
          <>
            <PrintButton />
            <ExportButton
              data={data?.items || []}
              filename="customers"
              sheetName="Customers"
            />
            <Button onClick={() => setShowCreate(true)}>
              <Plus className="h-4 w-4" />
              {t("newCustomer")}
            </Button>
          </>
        }
      />

      <div className="mb-4">
        <SearchBar placeholder={t("searchCustomers")} onSearch={setSearch} />
      </div>

      <DataTable
        columns={columns}
        data={data?.items || []}
        page={page}
        pageSize={pageSize}
        total={data?.total || 0}
        totalPages={data?.total_pages || 1}
        onPageChange={goToPage}
        isLoading={isLoading}
        emptyTitle={t("noCustomersFound")}
        emptyDescription={t("addFirstCustomer")}
      />

      <Modal
        open={showCreate}
        onClose={() => setShowCreate(false)}
        title={t("createCustomer")}
        size="lg"
        footer={
          <>
            <Button variant="secondary" onClick={() => setShowCreate(false)}>{tc("cancel")}</Button>
            <Button loading={createMutation.isPending} onClick={() => createMutation.mutate(newCustomer)}>
              {tc("create")}
            </Button>
          </>
        }
      >
        <div className="grid grid-cols-2 gap-4">
          <Input
            label={t("customerNumber")}
            required
            value={newCustomer.customer_number}
            onChange={(e) => setNewCustomer({ ...newCustomer, customer_number: e.target.value })}
          />
          <Input
            label={tc("name")}
            required
            value={newCustomer.name}
            onChange={(e) => setNewCustomer({ ...newCustomer, name: e.target.value })}
          />
          <Input
            label={t("email")}
            type="email"
            value={newCustomer.email}
            onChange={(e) => setNewCustomer({ ...newCustomer, email: e.target.value })}
          />
          <Input
            label={t("phone")}
            value={newCustomer.phone}
            onChange={(e) => setNewCustomer({ ...newCustomer, phone: e.target.value })}
          />
          <div className="col-span-2">
            <Input
              label={t("address")}
              value={newCustomer.address}
              onChange={(e) => setNewCustomer({ ...newCustomer, address: e.target.value })}
            />
          </div>
          <Input
            label={t("city")}
            value={newCustomer.city}
            onChange={(e) => setNewCustomer({ ...newCustomer, city: e.target.value })}
          />
          <Input
            label={t("country")}
            value={newCustomer.country}
            onChange={(e) => setNewCustomer({ ...newCustomer, country: e.target.value })}
          />
          <Input
            label={t("creditLimit")}
            type="number"
            value={newCustomer.credit_limit}
            onChange={(e) => setNewCustomer({ ...newCustomer, credit_limit: Number(e.target.value) })}
          />
          <Input
            label={t("paymentTerms")}
            value={newCustomer.payment_terms}
            onChange={(e) => setNewCustomer({ ...newCustomer, payment_terms: e.target.value })}
          />
        </div>
      </Modal>
    </div>
  );
}
