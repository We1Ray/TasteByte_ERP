"use client";

import { type ColumnDef } from "@tanstack/react-table";
import { Badge } from "@/components/ui/badge";
import { type Material } from "@/lib/api/mm";
import { formatCurrency } from "@/lib/utils";

export function getMaterialColumns(
  t: (key: string) => string,
  tCommon: (key: string) => string,
  tShared: (key: string) => string
): ColumnDef<Material, unknown>[] {
  return [
    {
      accessorKey: "material_number",
      header: t("materialNo"),
      cell: ({ row }) => (
        <span className="font-medium text-blue-600">{row.original.material_number}</span>
      ),
    },
    {
      accessorKey: "name",
      header: tCommon("name"),
      cell: ({ row }) => (
        <div>
          <p className="font-medium text-gray-900">{row.original.name}</p>
          <p className="text-xs text-gray-500">{row.original.description}</p>
        </div>
      ),
    },
    {
      accessorKey: "material_type",
      header: tCommon("type"),
      cell: ({ row }) => <Badge color="blue">{row.original.material_type}</Badge>,
    },
    {
      accessorKey: "material_group",
      header: t("materialGroup"),
    },
    {
      accessorKey: "base_unit",
      header: tCommon("unit"),
    },
    {
      accessorKey: "price",
      header: tCommon("price"),
      cell: ({ row }) => (
        <span className="font-mono">{formatCurrency(row.original.price, row.original.currency)}</span>
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
  ];
}
