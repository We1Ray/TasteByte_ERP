"use client";

import { useState, useCallback } from "react";
import { useQuery } from "@tanstack/react-query";
import { datasourceApi } from "../api/lowcode";

interface UseLookupOptions {
  sql: string;
  valueColumn: string;
  labelColumn: string;
  enabled?: boolean;
}

export function useLookup({ sql, valueColumn, labelColumn, enabled = true }: UseLookupOptions) {
  const [search, setSearch] = useState("");

  const query = useQuery({
    queryKey: ["lowcode", "lookup", sql, search],
    queryFn: () => datasourceApi.query(sql, { search }),
    enabled: enabled && !!sql,
  });

  const options = (query.data?.rows ?? []).map((row) => ({
    value: String(row[valueColumn] ?? ""),
    label: String(row[labelColumn] ?? ""),
  }));

  const handleSearch = useCallback((value: string) => {
    setSearch(value);
  }, []);

  return {
    options,
    isLoading: query.isLoading,
    search,
    setSearch: handleSearch,
  };
}

export function useDropdownOptions(dataSource?: { type: string; sql_query?: string; value_column?: string; label_column?: string; static_options?: { label: string; value: string }[] }) {
  const isSql = dataSource?.type === "sql" && !!dataSource.sql_query;

  const query = useQuery({
    queryKey: ["lowcode", "dropdown-options", dataSource?.sql_query],
    queryFn: () => datasourceApi.query(dataSource!.sql_query!),
    enabled: isSql,
  });

  if (!dataSource) return [];

  if (dataSource.type === "static" && dataSource.static_options) {
    return dataSource.static_options;
  }

  if (isSql && query.data) {
    return query.data.rows.map((row) => ({
      value: String(row[dataSource.value_column || "value"] ?? ""),
      label: String(row[dataSource.label_column || "label"] ?? ""),
    }));
  }

  return [];
}
