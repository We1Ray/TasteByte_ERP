"use client";

import {
  createContext,
  useContext,
  useState,
  useRef,
  useEffect,
  useCallback,
  type ReactNode,
  type ReactElement,
} from "react";
import { cn } from "@/lib/utils";

/* ------------------------------------------------------------------ */
/*  Context                                                            */
/* ------------------------------------------------------------------ */

const DropdownMenuContext = createContext<{ close: () => void }>({
  close: () => {},
});

/* ------------------------------------------------------------------ */
/*  DropdownMenu                                                       */
/* ------------------------------------------------------------------ */

interface DropdownMenuProps {
  trigger: ReactElement;
  children: ReactNode;
  align?: "left" | "right";
  className?: string;
}

export function DropdownMenu({
  trigger,
  children,
  align = "right",
  className,
}: DropdownMenuProps) {
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);

  const close = useCallback(() => setOpen(false), []);

  // Close on click outside
  useEffect(() => {
    function handleClickOutside(e: MouseEvent) {
      if (ref.current && !ref.current.contains(e.target as Node)) {
        setOpen(false);
      }
    }
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  // Close on Escape
  useEffect(() => {
    function handleKeyDown(e: KeyboardEvent) {
      if (e.key === "Escape") {
        setOpen(false);
      }
    }
    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, []);

  return (
    <DropdownMenuContext.Provider value={{ close }}>
      <div className="relative inline-block" ref={ref}>
        <div onClick={() => setOpen((prev) => !prev)}>{trigger}</div>
        {open && (
          <div
            className={cn(
              "absolute z-10 mt-1 w-48 rounded-md border border-gray-200 bg-white py-1 shadow-lg",
              align === "right" ? "right-0" : "left-0",
              className
            )}
          >
            {children}
          </div>
        )}
      </div>
    </DropdownMenuContext.Provider>
  );
}

/* ------------------------------------------------------------------ */
/*  DropdownMenuItem                                                   */
/* ------------------------------------------------------------------ */

interface DropdownMenuItemProps {
  children: ReactNode;
  onClick: () => void;
  icon?: ReactNode;
  variant?: "default" | "danger";
  disabled?: boolean;
  className?: string;
}

export function DropdownMenuItem({
  children,
  onClick,
  icon,
  variant = "default",
  disabled = false,
  className,
}: DropdownMenuItemProps) {
  const { close } = useContext(DropdownMenuContext);

  return (
    <button
      type="button"
      disabled={disabled}
      className={cn(
        "flex w-full items-center gap-2 px-4 py-2 text-left text-sm transition-colors",
        variant === "danger"
          ? "text-red-600 hover:bg-red-50"
          : "text-gray-700 hover:bg-gray-100",
        disabled && "cursor-not-allowed opacity-50",
        className
      )}
      onClick={() => {
        if (disabled) return;
        onClick();
        close();
      }}
    >
      {icon}
      {children}
    </button>
  );
}

/* ------------------------------------------------------------------ */
/*  DropdownMenuSeparator                                              */
/* ------------------------------------------------------------------ */

export function DropdownMenuSeparator() {
  return <div className="my-1 border-t border-gray-100" />;
}
