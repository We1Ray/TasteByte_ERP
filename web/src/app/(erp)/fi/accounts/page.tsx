"use client";

import { useState, useMemo } from "react";
import { type ColumnDef } from "@tanstack/react-table";
import { Plus } from "lucide-react";
import { useTranslations } from "next-intl";
import { PageHeader } from "@/components/layout/page-header";
import { DataTable } from "@/components/ui/data-table";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { SearchBar } from "@/components/forms/search-bar";
import { Modal } from "@/components/ui/modal";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { useApiQuery, useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { usePagination } from "@/lib/hooks/use-pagination";
import { fiApi, type Account } from "@/lib/api/fi";
import { formatCurrency } from "@/lib/utils";

export default function AccountsPage() {
  const t = useTranslations("fi");
  const tCommon = useTranslations("common");
  const tShared = useTranslations("shared");

  const columns = useMemo<ColumnDef<Account, unknown>[]>(() => [
    {
      accessorKey: "account_number",
      header: t("accountNo"),
      cell: ({ row }) => (
        <span className="font-medium text-blue-600">{row.original.account_number}</span>
      ),
    },
    { accessorKey: "name", header: t("accountName") },
    {
      accessorKey: "account_type",
      header: tCommon("type"),
      cell: ({ row }) => (
        <Badge color="blue">{row.original.account_type}</Badge>
      ),
    },
    { accessorKey: "currency", header: t("currency") },
    {
      accessorKey: "balance",
      header: t("balance"),
      cell: ({ row }) => (
        <span className="font-mono">{formatCurrency(row.original.balance, row.original.currency)}</span>
      ),
    },
    {
      accessorKey: "is_active",
      header: tCommon("status"),
      cell: ({ row }) => (
        <Badge color={row.original.is_active ? "green" : "gray"}>
          {row.original.is_active ? tCommon("active") : tShared("inactive")}
        </Badge>
      ),
    },
  ], [t, tCommon, tShared]);

  const { page, pageSize, goToPage } = usePagination();
  const [search, setSearch] = useState("");
  const [showCreate, setShowCreate] = useState(false);
  const [newAccount, setNewAccount] = useState({
    account_number: "",
    name: "",
    account_type: "Asset",
    currency: "USD",
  });
  const invalidate = useInvalidateQueries();

  const { data, isLoading } = useApiQuery(
    ["fi", "accounts", String(page), search],
    () => fiApi.getAccounts({ page, page_size: pageSize, search: search || undefined })
  );

  const createMutation = useApiMutation(
    (data: Partial<Account>) => fiApi.createAccount(data),
    {
      onSuccess: () => {
        invalidate(["fi", "accounts"]);
        setShowCreate(false);
        setNewAccount({ account_number: "", name: "", account_type: "Asset", currency: "USD" });
      },
    }
  );

  return (
    <div>
      <PageHeader
        title={t("chartOfAccounts")}
        description={t("manageAccounts")}
        actions={
          <Button onClick={() => setShowCreate(true)}>
            <Plus className="h-4 w-4" />
            {t("newAccount")}
          </Button>
        }
      />

      <div className="mb-4">
        <SearchBar
          placeholder={t("searchAccounts")}
          onSearch={setSearch}
        />
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
        emptyTitle={t("noAccountsFound")}
        emptyDescription={t("createFirstAccount")}
      />

      <Modal
        open={showCreate}
        onClose={() => setShowCreate(false)}
        title={t("createAccount")}
        footer={
          <>
            <Button variant="secondary" onClick={() => setShowCreate(false)}>
              {tCommon("cancel")}
            </Button>
            <Button
              loading={createMutation.isPending}
              onClick={() => createMutation.mutate(newAccount)}
            >
              {tCommon("create")}
            </Button>
          </>
        }
      >
        <div className="space-y-4">
          <Input
            label={t("accountNumber")}
            required
            value={newAccount.account_number}
            onChange={(e) => setNewAccount({ ...newAccount, account_number: e.target.value })}
            placeholder={t("accountNumberPlaceholder")}
          />
          <Input
            label={t("accountName")}
            required
            value={newAccount.name}
            onChange={(e) => setNewAccount({ ...newAccount, name: e.target.value })}
            placeholder={t("accountNamePlaceholder")}
          />
          <Select
            label={t("accountType")}
            required
            value={newAccount.account_type}
            onChange={(e) => setNewAccount({ ...newAccount, account_type: e.target.value })}
            options={[
              { value: "Asset", label: t("asset") },
              { value: "Liability", label: t("liability") },
              { value: "Equity", label: t("equity") },
              { value: "Revenue", label: t("revenue") },
              { value: "Expense", label: t("expense") },
            ]}
          />
          <Select
            label={t("currency")}
            value={newAccount.currency}
            onChange={(e) => setNewAccount({ ...newAccount, currency: e.target.value })}
            options={[
              { value: "USD", label: t("currencyUsd") },
              { value: "EUR", label: t("currencyEur") },
              { value: "GBP", label: t("currencyGbp") },
              { value: "TWD", label: t("currencyTwd") },
            ]}
          />
        </div>
      </Modal>
    </div>
  );
}
