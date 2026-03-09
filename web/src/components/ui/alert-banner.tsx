import { Info, AlertTriangle, AlertCircle, CheckCircle, X } from "lucide-react";
import { cn } from "@/lib/utils";

interface AlertBannerProps {
  variant: "info" | "warning" | "error" | "success";
  title?: string;
  children: React.ReactNode;
  dismissible?: boolean;
  onDismiss?: () => void;
  action?: {
    label: string;
    onClick: () => void;
  };
  className?: string;
}

const variantConfig = {
  info: {
    classes: "bg-blue-50 border-blue-200 text-blue-800",
    Icon: Info,
  },
  warning: {
    classes: "bg-amber-50 border-amber-200 text-amber-800",
    Icon: AlertTriangle,
  },
  error: {
    classes: "bg-red-50 border-red-200 text-red-800",
    Icon: AlertCircle,
  },
  success: {
    classes: "bg-green-50 border-green-200 text-green-800",
    Icon: CheckCircle,
  },
} as const;

export function AlertBanner({
  variant,
  title,
  children,
  dismissible,
  onDismiss,
  action,
  className,
}: AlertBannerProps) {
  const { classes, Icon } = variantConfig[variant];

  return (
    <div
      className={cn(
        "rounded-lg border px-4 py-3 flex items-start gap-3",
        classes,
        className
      )}
    >
      <Icon className="h-5 w-5 flex-shrink-0 mt-0.5" />
      <div className="flex-1 min-w-0">
        {title && <p className="font-medium">{title}</p>}
        <div className={cn("text-sm", title && "mt-0.5")}>{children}</div>
        {action && (
          <button
            className="text-sm font-medium underline hover:no-underline mt-1"
            onClick={action.onClick}
          >
            {action.label}
          </button>
        )}
      </div>
      {dismissible && (
        <button onClick={onDismiss} className="flex-shrink-0 ml-auto">
          <X className="h-4 w-4" />
        </button>
      )}
    </div>
  );
}
