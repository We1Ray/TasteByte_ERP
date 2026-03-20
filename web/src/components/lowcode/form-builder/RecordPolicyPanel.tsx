"use client";

import { useState } from "react";
import { useTranslations } from "next-intl";
import { Shield, Plus, Trash2 } from "lucide-react";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useApiQuery, useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { permissionsApi } from "@/lib/api/lowcode";
import type { RecordPermission } from "@/lib/types/lowcode";

interface RecordPolicyPanelProps {
  operationId: string;
}

export function RecordPolicyPanel({ operationId }: RecordPolicyPanelProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const invalidate = useInvalidateQueries();

  const { data: policies, isLoading } = useApiQuery(
    ["lowcode", "record-policies", operationId],
    () => permissionsApi.getRecordPermissions(operationId),
    { enabled: !!operationId }
  );

  const createMutation = useApiMutation(
    (data: { policy_name: string; filter_sql: string; is_active?: boolean; role_id?: string }) =>
      permissionsApi.createRecordPermission(operationId, data),
    { onSuccess: () => invalidate(["lowcode", "record-policies", operationId]) }
  );

  const deleteMutation = useApiMutation(
    (policyId: string) => permissionsApi.deleteRecordPermission(operationId, policyId),
    { onSuccess: () => invalidate(["lowcode", "record-policies", operationId]) }
  );

  const [policyName, setPolicyName] = useState("");
  const [filterSql, setFilterSql] = useState("");
  const [isActive, setIsActive] = useState(true);
  const [showForm, setShowForm] = useState(false);

  const handleCreate = () => {
    if (!policyName || !filterSql) return;
    createMutation.mutateAsync({
      policy_name: policyName,
      filter_sql: filterSql,
      is_active: isActive,
    }).then(() => {
      setPolicyName("");
      setFilterSql("");
      setIsActive(true);
      setShowForm(false);
    });
  };

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle className="flex items-center gap-2">
            <Shield className="h-4 w-4" />
            {t("recordPolicies")}
          </CardTitle>
          <Button size="sm" variant="secondary" onClick={() => setShowForm(!showForm)}>
            <Plus className="h-4 w-4" />
            {t("addPolicy")}
          </Button>
        </div>
      </CardHeader>

      <div className="space-y-3">
        {showForm && (
          <div className="rounded-md border border-blue-200 bg-blue-50 p-4 space-y-3">
            <Input
              label={t("policyName")}
              value={policyName}
              onChange={(e) => setPolicyName(e.target.value)}
              placeholder={t("policyNamePlaceholder")}
              required
            />
            <div>
              <label className="mb-1 block text-sm font-medium text-gray-700">{t("filterSql")}</label>
              <textarea
                value={filterSql}
                onChange={(e) => setFilterSql(e.target.value)}
                rows={3}
                className="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm font-mono shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
                placeholder={t("filterSqlPlaceholder")}
              />
              <p className="mt-1 text-xs text-gray-400">{t("filterSqlHint")}</p>
            </div>
            <label className="flex items-center gap-2">
              <input
                type="checkbox"
                checked={isActive}
                onChange={(e) => setIsActive(e.target.checked)}
                className="h-4 w-4 rounded border-gray-300 text-blue-600"
              />
              <span className="text-sm text-gray-700">{t("policyActive")}</span>
            </label>
            <div className="flex gap-2">
              <Button size="sm" onClick={handleCreate} disabled={!policyName || !filterSql} loading={createMutation.isPending}>
                {tCommon("save")}
              </Button>
              <Button size="sm" variant="secondary" onClick={() => setShowForm(false)}>
                {tCommon("cancel")}
              </Button>
            </div>
          </div>
        )}

        {isLoading ? (
          <p className="py-4 text-center text-sm text-gray-400">{tCommon("loading")}</p>
        ) : (policies || []).length === 0 ? (
          <p className="py-4 text-center text-sm text-gray-400">{t("noPolicies")}</p>
        ) : (
          <div className="space-y-2">
            {(policies || []).map((policy: RecordPermission) => (
              <div key={policy.id} className="flex items-center justify-between rounded-md border border-gray-200 bg-white p-3">
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2">
                    <p className="text-sm font-medium text-gray-900">{policy.policy_name}</p>
                    <span className={`inline-flex rounded-full px-2 py-0.5 text-xs font-medium ${
                      policy.is_active ? "bg-green-100 text-green-700" : "bg-gray-100 text-gray-500"
                    }`}>
                      {policy.is_active ? t("active") : t("inactive")}
                    </span>
                  </div>
                  <p className="mt-1 truncate font-mono text-xs text-gray-500">{policy.filter_sql}</p>
                </div>
                <button
                  onClick={() => deleteMutation.mutateAsync(policy.id)}
                  className="ml-2 text-red-400 hover:text-red-600"
                >
                  <Trash2 className="h-4 w-4" />
                </button>
              </div>
            ))}
          </div>
        )}
      </div>
    </Card>
  );
}
