import { cn } from "@/lib/utils";

interface DescriptionItem {
  label: string;
  value: React.ReactNode;
  hidden?: boolean;
}

interface DescriptionListProps {
  items: DescriptionItem[];
  layout?: "stacked" | "grid";
  columns?: 2 | 3;
  className?: string;
}

const gridColsClass = {
  2: "grid-cols-2",
  3: "grid-cols-3",
} as const;

export function DescriptionList({
  items,
  layout = "stacked",
  columns = 2,
  className,
}: DescriptionListProps) {
  const visibleItems = items.filter((item) => !item.hidden);

  if (layout === "grid") {
    return (
      <dl className={cn("grid gap-4 text-sm", gridColsClass[columns], className)}>
        {visibleItems.map((item) => (
          <div key={item.label}>
            <dt className="text-gray-500">{item.label}</dt>
            <dd className="mt-1 font-medium text-gray-900">{item.value}</dd>
          </div>
        ))}
      </dl>
    );
  }

  return (
    <dl className={cn("space-y-3 text-sm", className)}>
      {visibleItems.map((item) => (
        <div key={item.label} className="flex justify-between">
          <dt className="text-gray-500">{item.label}</dt>
          <dd className="text-gray-900">{item.value}</dd>
        </div>
      ))}
    </dl>
  );
}
