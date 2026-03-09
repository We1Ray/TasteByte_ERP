import { type LucideIcon } from "lucide-react";
import { cn } from "@/lib/utils";
import { Card } from "../ui/card";

interface KpiCardProps {
  title: string;
  value: string;
  change?: string;
  changeType?: "positive" | "negative" | "neutral";
  icon: LucideIcon;
  iconColor?: string;
  className?: string;
}

export function KpiCard({
  title,
  value,
  change,
  changeType = "neutral",
  icon: Icon,
  iconColor = "bg-blue-100 text-blue-600",
  className,
}: KpiCardProps) {
  return (
    <Card className={className}>
      <div className="flex items-start justify-between">
        <div>
          <p className="text-sm font-medium text-gray-500">{title}</p>
          <p className="mt-1 text-2xl font-bold text-gray-900">{value}</p>
          {change && (
            <p
              className={cn(
                "mt-1 text-sm font-medium",
                changeType === "positive" && "text-green-600",
                changeType === "negative" && "text-red-600",
                changeType === "neutral" && "text-gray-500"
              )}
            >
              {change}
            </p>
          )}
        </div>
        <div className={cn("rounded-lg p-3", iconColor)}>
          <Icon className="h-6 w-6" />
        </div>
      </div>
    </Card>
  );
}
