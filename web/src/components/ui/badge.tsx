import { cn } from "@/lib/utils";

const colorClasses: Record<string, string> = {
  gray: "bg-gray-100 text-gray-700 ring-gray-500/20",
  blue: "bg-blue-50 text-blue-700 ring-blue-600/20",
  green: "bg-green-50 text-green-700 ring-green-600/20",
  red: "bg-red-50 text-red-700 ring-red-600/20",
  amber: "bg-amber-50 text-amber-700 ring-amber-600/20",
  purple: "bg-purple-50 text-purple-700 ring-purple-600/20",
};

interface BadgeProps {
  children: React.ReactNode;
  color?: keyof typeof colorClasses;
  className?: string;
}

export function Badge({ children, color = "gray", className }: BadgeProps) {
  return (
    <span
      className={cn(
        "inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ring-1 ring-inset",
        colorClasses[color] || colorClasses.gray,
        className
      )}
    >
      {children}
    </span>
  );
}

export function StatusBadge({ status }: { status: string }) {
  const s = (status ?? "").toLowerCase().replace(/_/g, " ");
  let color: keyof typeof colorClasses = "gray";

  if (["draft", "inactive"].includes(s)) color = "gray";
  else if (["released", "open", "active", "confirmed"].includes(s)) color = "blue";
  else if (["in progress", "pending", "processing", "submitted"].includes(s)) color = "amber";
  else if (["completed", "done", "approved", "paid", "delivered"].includes(s)) color = "green";
  else if (["closed", "cancelled", "rejected", "overdue"].includes(s)) color = "red";

  return (
    <Badge color={color}>
      {s.charAt(0).toUpperCase() + s.slice(1)}
    </Badge>
  );
}
