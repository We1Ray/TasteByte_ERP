"use client";

import { useState, useRef, useCallback, type ReactNode, type ReactElement } from "react";
import { cn } from "@/lib/utils";

interface TooltipProps {
  content: ReactNode;
  children: ReactElement;
  side?: "top" | "right" | "bottom" | "left";
  delayMs?: number;
  className?: string;
}

const positionClasses: Record<string, string> = {
  top: "bottom-full left-1/2 -translate-x-1/2 mb-2",
  right: "left-full top-1/2 -translate-y-1/2 ml-2",
  bottom: "top-full left-1/2 -translate-x-1/2 mt-2",
  left: "right-full top-1/2 -translate-y-1/2 mr-2",
};

const arrowClasses: Record<string, string> = {
  top: "absolute left-1/2 -translate-x-1/2 top-full border-4 border-transparent border-t-gray-900",
  bottom: "absolute left-1/2 -translate-x-1/2 bottom-full border-4 border-transparent border-b-gray-900",
  left: "absolute left-full top-1/2 -translate-y-1/2 border-4 border-transparent border-l-gray-900",
  right: "absolute right-full top-1/2 -translate-y-1/2 border-4 border-transparent border-r-gray-900",
};

export function Tooltip({
  content,
  children,
  side = "top",
  delayMs = 0,
  className,
}: TooltipProps) {
  const [visible, setVisible] = useState(false);
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const show = useCallback(() => {
    if (delayMs > 0) {
      timeoutRef.current = setTimeout(() => setVisible(true), delayMs);
    } else {
      setVisible(true);
    }
  }, [delayMs]);

  const hide = useCallback(() => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
      timeoutRef.current = null;
    }
    setVisible(false);
  }, []);

  return (
    <div
      className="relative inline-flex"
      onMouseEnter={show}
      onMouseLeave={hide}
      onFocus={show}
      onBlur={hide}
    >
      {children}
      {visible && (
        <div
          className={cn(
            "absolute z-50 whitespace-nowrap rounded-md bg-gray-900 px-2.5 py-1.5 text-xs font-medium text-white shadow-lg pointer-events-none",
            positionClasses[side],
            className
          )}
        >
          {content}
          <div className={arrowClasses[side]} />
        </div>
      )}
    </div>
  );
}
