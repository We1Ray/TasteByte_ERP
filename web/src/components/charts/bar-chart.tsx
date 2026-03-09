"use client";

import {
  BarChart as RechartsBarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  Legend,
} from "recharts";
import { Card, CardHeader, CardTitle } from "../ui/card";

interface BarChartProps<T extends object = object> {
  title: string;
  data: T[];
  dataKey: string;
  xAxisKey: string;
  color?: string;
  secondaryDataKey?: string;
  secondaryColor?: string;
  height?: number;
}

export function BarChartCard({
  title,
  data,
  dataKey,
  xAxisKey,
  color = "#2563eb",
  secondaryDataKey,
  secondaryColor = "#7c3aed",
  height = 300,
}: BarChartProps) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>{title}</CardTitle>
      </CardHeader>
      <div style={{ height }}>
        <ResponsiveContainer width="100%" height="100%">
          <RechartsBarChart data={data} margin={{ top: 5, right: 20, left: 0, bottom: 5 }}>
            <CartesianGrid strokeDasharray="3 3" stroke="#f0f0f0" />
            <XAxis
              dataKey={xAxisKey}
              tick={{ fontSize: 12 }}
              tickLine={false}
              axisLine={false}
            />
            <YAxis tick={{ fontSize: 12 }} tickLine={false} axisLine={false} />
            <Tooltip
              contentStyle={{
                borderRadius: "8px",
                border: "1px solid #e5e7eb",
                boxShadow: "0 4px 6px -1px rgba(0,0,0,0.1)",
              }}
            />
            {secondaryDataKey && <Legend />}
            <Bar dataKey={dataKey} fill={color} radius={[4, 4, 0, 0]} />
            {secondaryDataKey && (
              <Bar dataKey={secondaryDataKey} fill={secondaryColor} radius={[4, 4, 0, 0]} />
            )}
          </RechartsBarChart>
        </ResponsiveContainer>
      </div>
    </Card>
  );
}
