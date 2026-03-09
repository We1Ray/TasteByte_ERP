"use client";

import { useState, useMemo } from "react";
import { useRouter } from "next/navigation";
import { type ColumnDef } from "@tanstack/react-table";
import { useTranslations } from "next-intl";
import { Plus } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { DataTable } from "@/components/ui/data-table";
import { Button } from "@/components/ui/button";
import { StatusBadge } from "@/components/ui/badge";
import { SearchBar } from "@/components/forms/search-bar";
import { ExportButton } from "@/components/shared/export-button";
import { PrintButton } from "@/components/shared/print-button";
import { Modal } from "@/components/ui/modal";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { useApiQuery, useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { usePagination } from "@/lib/hooks/use-pagination";
import { hrApi, type Employee } from "@/lib/api/hr";
import { formatDate, formatCurrency } from "@/lib/utils";

export default function EmployeesPage() {
  const t = useTranslations("hr");
  const tc = useTranslations("common");

  const columns = useMemo<ColumnDef<Employee, unknown>[]>(() => [
    {
      accessorKey: "employee_number",
      header: t("employeeNo"),
      cell: ({ row }) => (
        <span className="font-medium text-blue-600">{row.original.employee_number}</span>
      ),
    },
    {
      accessorKey: "first_name",
      header: tc("name"),
      cell: ({ row }) => (
        <span className="font-medium text-gray-900">
          {row.original.first_name} {row.original.last_name}
        </span>
      ),
    },
    { accessorKey: "email", header: t("email") },
    { accessorKey: "department_name", header: t("department") },
    { accessorKey: "position", header: t("position") },
    {
      accessorKey: "hire_date",
      header: t("hireDate"),
      cell: ({ row }) => formatDate(row.original.hire_date),
    },
    {
      accessorKey: "salary",
      header: t("salary"),
      cell: ({ row }) => (
        <span className="font-mono">{formatCurrency(row.original.salary, row.original.currency)}</span>
      ),
    },
    {
      accessorKey: "status",
      header: tc("status"),
      cell: ({ row }) => <StatusBadge status={row.original.status} />,
    },
  ], [t, tc]);

  const router = useRouter();
  const { page, pageSize, goToPage } = usePagination();
  const [search, setSearch] = useState("");
  const [showCreate, setShowCreate] = useState(false);
  const [newEmployee, setNewEmployee] = useState({
    first_name: "",
    last_name: "",
    email: "",
    phone: "",
    hire_date: new Date().toISOString().split("T")[0],
    department_id: "",
    position: "",
  });
  const invalidate = useInvalidateQueries();

  const { data, isLoading } = useApiQuery(
    ["hr", "employees", String(page), search],
    () =>
      hrApi.getEmployees({
        page,
        page_size: pageSize,
        search: search || undefined,
      })
  );

  const { data: departments } = useApiQuery(
    ["hr", "departments"],
    () => hrApi.getDepartments()
  );

  const createMutation = useApiMutation(
    (data: Partial<Employee>) => hrApi.createEmployee(data),
    {
      onSuccess: () => {
        invalidate(["hr", "employees"]);
        setShowCreate(false);
        setNewEmployee({
          first_name: "",
          last_name: "",
          email: "",
          phone: "",
          hire_date: new Date().toISOString().split("T")[0],
          department_id: "",
          position: "",
        });
      },
    }
  );

  const departmentList = Array.isArray(departments) ? departments : [];

  return (
    <div>
      <PageHeader
        title={t("employees")}
        description={t("manageEmployees")}
        actions={
          <>
            <PrintButton />
            <ExportButton
              data={data?.items || []}
              filename="employees"
              sheetName="Employees"
            />
            <Button onClick={() => setShowCreate(true)}>
              <Plus className="h-4 w-4" />
              {t("addEmployee")}
            </Button>
          </>
        }
      />

      <div className="mb-4">
        <SearchBar
          placeholder={t("searchEmployees")}
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
        onRowClick={(row) => router.push(`/hr/employees/${row.id}`)}
        isLoading={isLoading}
        emptyTitle={t("noEmployeesFound")}
        emptyDescription={t("addFirstEmployee")}
      />

      <Modal
        open={showCreate}
        onClose={() => setShowCreate(false)}
        title={t("addEmployee")}
        size="lg"
        footer={
          <>
            <Button variant="secondary" onClick={() => setShowCreate(false)}>{tc("cancel")}</Button>
            <Button loading={createMutation.isPending} onClick={() => createMutation.mutate(newEmployee)}>
              {tc("create")}
            </Button>
          </>
        }
      >
        <div className="grid grid-cols-2 gap-4">
          <Input
            label={t("firstName")}
            required
            value={newEmployee.first_name}
            onChange={(e) => setNewEmployee({ ...newEmployee, first_name: e.target.value })}
          />
          <Input
            label={t("lastName")}
            required
            value={newEmployee.last_name}
            onChange={(e) => setNewEmployee({ ...newEmployee, last_name: e.target.value })}
          />
          <Input
            label={t("email")}
            type="email"
            value={newEmployee.email}
            onChange={(e) => setNewEmployee({ ...newEmployee, email: e.target.value })}
          />
          <Input
            label={t("phone")}
            value={newEmployee.phone}
            onChange={(e) => setNewEmployee({ ...newEmployee, phone: e.target.value })}
          />
          <Input
            label={t("hireDate")}
            type="date"
            required
            value={newEmployee.hire_date}
            onChange={(e) => setNewEmployee({ ...newEmployee, hire_date: e.target.value })}
          />
          {departmentList.length > 0 ? (
            <Select
              label={t("department")}
              value={newEmployee.department_id}
              onChange={(e) => setNewEmployee({ ...newEmployee, department_id: e.target.value })}
              placeholder={t("selectDepartment")}
              options={departmentList.map((d) => ({
                value: d.id,
                label: d.name,
              }))}
            />
          ) : (
            <Input
              label={t("department")}
              value={newEmployee.department_id}
              onChange={(e) => setNewEmployee({ ...newEmployee, department_id: e.target.value })}
              placeholder={t("departmentId")}
            />
          )}
          <div className="col-span-2">
            <Input
              label={t("position")}
              value={newEmployee.position}
              onChange={(e) => setNewEmployee({ ...newEmployee, position: e.target.value })}
              placeholder={t("positionPlaceholder")}
            />
          </div>
        </div>
      </Modal>
    </div>
  );
}
