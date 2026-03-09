"use client";

import { use } from "react";
import { useRouter } from "next/navigation";
import { useTranslations } from "next-intl";
import { ArrowLeft } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { StatusBadge } from "@/components/ui/badge";
import { PageLoading } from "@/components/ui/loading";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { hrApi } from "@/lib/api/hr";
import { DescriptionList } from "@/components/ui/description-list";
import { formatDate, formatCurrency } from "@/lib/utils";

export default function EmployeeDetailPage({ params }: { params: Promise<{ id: string }> }) {
  const { id } = use(params);
  const router = useRouter();
  const t = useTranslations("hr");
  const tc = useTranslations("common");

  const { data: employee, isLoading } = useApiQuery(
    ["hr", "employees", id],
    () => hrApi.getEmployee(id)
  );

  if (isLoading) {
    return <PageLoading />;
  }

  if (!employee) {
    return (
      <div className="py-12 text-center">
        <p className="text-gray-500">{t("noEmployeesFound")}</p>
        <Button variant="link" onClick={() => router.push("/hr/employees")} className="mt-2">
          {tc("back")}
        </Button>
      </div>
    );
  }

  return (
    <div>
      <PageHeader
        title={`${employee.first_name} ${employee.last_name}`}
        description={`${t("employees")} ${employee.employee_number}`}
        actions={
          <Button variant="secondary" onClick={() => router.push("/hr/employees")}>
            <ArrowLeft className="h-4 w-4" />
            {tc("back")}
          </Button>
        }
      />

      <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle>{t("employees")}</CardTitle>
          </CardHeader>
          <DescriptionList items={[
            { label: t("employeeNo"), value: employee.employee_number },
            { label: t("firstName"), value: employee.first_name },
            { label: t("lastName"), value: employee.last_name },
            { label: t("email"), value: employee.email || "-" },
            { label: t("phone"), value: employee.phone || "-" },
          ]} />
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>{t("department")}</CardTitle>
          </CardHeader>
          <DescriptionList items={[
            { label: tc("status"), value: <StatusBadge status={employee.status} /> },
            { label: t("department"), value: employee.department_name || "-" },
            { label: t("position"), value: employee.position || "-" },
            { label: t("hireDate"), value: formatDate(employee.hire_date) },
            { label: t("salary"), value: <span className="font-mono">{formatCurrency(employee.salary, employee.currency)}</span> },
            { label: tc("createdAt"), value: formatDate(employee.created_at) },
          ]} />
        </Card>
      </div>
    </div>
  );
}
