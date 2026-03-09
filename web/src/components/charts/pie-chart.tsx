"use client";

import {
  PieChart as RechartsPieChart,
  Pie,
  Cell,
  Tooltip,
  ResponsiveContainer,
  Legend,
} from "recharts";
import { Card, CardHeader, CardTitle } from "../ui/card";

const DEFAULT_COLORS = [
  "#2563eb",
  "#7c3aed",
  "#10b981",
  "#f59e0b",
  "#ef4444",
  "#06b6d4",
  "#8b5cf6",
  "#ec4899",
  "#14b8a6",
  "#f97316",
];

interface PieChartProps {
  title: string;
  data: Record<string, unknown>[];
  dataKey: string;
  nameKey: string;
  colors?: string[];
  height?: number;
  innerRadius?: number;
  outerRadius?: number;
}

export function PieChartCard({
  title,
  data,
  dataKey,
  nameKey,
  colors = DEFAULT_COLORS,
  height = 300,
  innerRadius = 0,
  outerRadius = 100,
}: PieChartProps) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>{title}</CardTitle>
      </CardHeader>
      <div style={{ height }}>
        <ResponsiveContainer width="100%" height="100%">
          <RechartsPieChart>
            <Pie
              data={data}
              dataKey={dataKey}
              nameKey={nameKey}
              cx="50%"
              cy="50%"
              innerRadius={innerRadius}
              outerRadius={outerRadius}
              paddingAngle={2}
              label={({ name, percent }) =>
                `${name} ${((percent ?? 0) * 100).toFixed(0)}%`
              }
              labelLine={true}
            >
              {data.map((_, index) => (
                <Cell
                  key={`cell-${index}`}
                  fill={colors[index % colors.length]}
                />
              ))}
            </Pie>
            <Tooltip
              contentStyle={{
                borderRadius: "8px",
                border: "1px solid #e5e7eb",
                boxShadow: "0 4px 6px -1px rgba(0,0,0,0.1)",
              }}
            />
            <Legend />
          </RechartsPieChart>
        </ResponsiveContainer>
      </div>
    </Card>
  );
}
