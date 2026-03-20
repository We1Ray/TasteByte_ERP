"use client";

import { useState } from "react";
import { useTranslations } from "next-intl";
import { DollarSign, Plus, Trash2, ArrowRight } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { PageLoading } from "@/components/ui/loading";
import {
  useApiQuery,
  useApiMutation,
  useInvalidateQueries,
} from "@/lib/hooks/use-api-query";
import { exchangeRateApi, type ExchangeRate } from "@/lib/api/system";

const CURRENCIES = ["TWD", "USD", "EUR", "JPY", "GBP", "CNY"];

export default function ExchangeRatesPage() {
  const t = useTranslations("admin");
  const tCommon = useTranslations("common");
  const invalidate = useInvalidateQueries();
  const [showAdd, setShowAdd] = useState(false);
  const [form, setForm] = useState({
    from_currency: "USD",
    to_currency: "TWD",
    rate: "",
    valid_from: "",
  });
  const [convertForm, setConvertForm] = useState({
    from: "USD",
    to: "TWD",
    amount: "100",
  });
  const [convertResult, setConvertResult] = useState<string | null>(null);

  const { data: rates, isLoading } = useApiQuery(
    ["system", "exchange-rates"],
    () => exchangeRateApi.list()
  );

  const createMutation = useApiMutation(
    () =>
      exchangeRateApi.create({
        ...form,
        rate: Number(form.rate),
        valid_from: form.valid_from,
      }),
    {
      onSuccess: () => {
        invalidate(["system", "exchange-rates"]);
        setShowAdd(false);
        setForm({
          from_currency: "USD",
          to_currency: "TWD",
          rate: "",
          valid_from: "",
        });
      },
    }
  );

  const deleteMutation = useApiMutation(
    (id: string) => exchangeRateApi.delete(id),
    {
      onSuccess: () => invalidate(["system", "exchange-rates"]),
    }
  );

  const handleConvert = async () => {
    try {
      const result = await exchangeRateApi.convert(
        convertForm.from,
        convertForm.to,
        Number(convertForm.amount)
      );
      setConvertResult(result.result);
    } catch {
      setConvertResult(null);
    }
  };

  if (isLoading) return <PageLoading />;

  return (
    <div>
      <PageHeader
        title={t("exchangeRates")}
        description={t("exchangeRatesDesc")}
        actions={
          <Button onClick={() => setShowAdd(!showAdd)}>
            <Plus className="h-4 w-4" />
            {t("addRate")}
          </Button>
        }
      />

      {/* Currency Converter */}
      <Card className="mb-6">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <DollarSign className="h-4 w-4" />
            {t("currencyConverter")}
          </CardTitle>
        </CardHeader>
        <div className="flex items-end gap-3">
          <Input
            label={t("amount")}
            value={convertForm.amount}
            onChange={(e) =>
              setConvertForm({ ...convertForm, amount: e.target.value })
            }
            type="number"
          />
          <Select
            label={t("from")}
            value={convertForm.from}
            onChange={(e) =>
              setConvertForm({ ...convertForm, from: e.target.value })
            }
            options={CURRENCIES.map((c) => ({ value: c, label: c }))}
          />
          <ArrowRight className="mb-2 h-5 w-5 shrink-0 text-gray-400" />
          <Select
            label={t("to")}
            value={convertForm.to}
            onChange={(e) =>
              setConvertForm({ ...convertForm, to: e.target.value })
            }
            options={CURRENCIES.map((c) => ({ value: c, label: c }))}
          />
          <Button onClick={handleConvert}>{t("convert")}</Button>
          {convertResult && (
            <div className="mb-1 rounded-md bg-green-50 px-4 py-2">
              <p className="text-lg font-bold text-green-700">
                {convertResult} {convertForm.to}
              </p>
            </div>
          )}
        </div>
      </Card>

      {/* Add Rate Form */}
      {showAdd && (
        <Card className="mb-6 border-blue-200 bg-blue-50">
          <div className="flex items-end gap-3">
            <Select
              label={t("from")}
              value={form.from_currency}
              onChange={(e) =>
                setForm({ ...form, from_currency: e.target.value })
              }
              options={CURRENCIES.map((c) => ({ value: c, label: c }))}
            />
            <Select
              label={t("to")}
              value={form.to_currency}
              onChange={(e) =>
                setForm({ ...form, to_currency: e.target.value })
              }
              options={CURRENCIES.map((c) => ({ value: c, label: c }))}
            />
            <Input
              label={t("rate")}
              value={form.rate}
              onChange={(e) => setForm({ ...form, rate: e.target.value })}
              type="number"
            />
            <Input
              label={t("validFrom")}
              value={form.valid_from}
              onChange={(e) =>
                setForm({ ...form, valid_from: e.target.value })
              }
              type="date"
            />
            <Button
              onClick={() => createMutation.mutateAsync(undefined)}
              loading={createMutation.isPending}
              disabled={!form.rate || !form.valid_from}
            >
              {tCommon("save")}
            </Button>
          </div>
        </Card>
      )}

      {/* Rates Table */}
      <Card>
        <div className="overflow-x-auto">
          <table className="min-w-full text-sm">
            <thead className="border-b bg-gray-50">
              <tr>
                <th className="px-4 py-3 text-left font-medium text-gray-600">
                  {t("from")}
                </th>
                <th className="px-4 py-3 text-left font-medium text-gray-600">
                  {t("to")}
                </th>
                <th className="px-4 py-3 text-right font-medium text-gray-600">
                  {t("rate")}
                </th>
                <th className="px-4 py-3 text-left font-medium text-gray-600">
                  {t("validFrom")}
                </th>
                <th className="px-4 py-3"></th>
              </tr>
            </thead>
            <tbody className="divide-y">
              {(rates || []).map((rate: ExchangeRate) => (
                <tr key={rate.id} className="hover:bg-gray-50">
                  <td className="px-4 py-3 font-medium">
                    {rate.from_currency}
                  </td>
                  <td className="px-4 py-3 font-medium">
                    {rate.to_currency}
                  </td>
                  <td className="px-4 py-3 text-right font-mono">
                    {rate.rate}
                  </td>
                  <td className="px-4 py-3 text-gray-500">
                    {rate.valid_from}
                  </td>
                  <td className="px-4 py-3 text-right">
                    <button
                      onClick={() => deleteMutation.mutateAsync(rate.id)}
                      className="text-gray-400 hover:text-red-500"
                    >
                      <Trash2 className="h-4 w-4" />
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </Card>
    </div>
  );
}
