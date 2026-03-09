"use client";

import { type ReactNode, useCallback, useEffect, useRef, useState } from "react";
import { X } from "lucide-react";
import { cn } from "@/lib/utils";

interface ModalProps {
  open: boolean;
  onClose: () => void;
  title: string;
  children: ReactNode;
  size?: "sm" | "md" | "lg" | "xl";
  footer?: ReactNode;
}

const sizeClasses = {
  sm: "max-w-md",
  md: "max-w-lg",
  lg: "max-w-2xl",
  xl: "max-w-4xl",
};

/**
 * Manages mount/unmount lifecycle for CSS transition animations.
 * Returns [shouldRender, isVisible] where shouldRender controls DOM presence
 * and isVisible controls the CSS transition classes.
 */
function useAnimationMount(open: boolean, duration = 150): [boolean, boolean] {
  const [shouldRender, setShouldRender] = useState(open);
  const [isVisible, setIsVisible] = useState(false);
  const timerRef = useRef<ReturnType<typeof setTimeout>>(undefined);
  const rafRef = useRef<number>(undefined);

  const handleOpen = useCallback(() => {
    setShouldRender(true);
    // Double rAF ensures the DOM has painted before triggering the enter transition
    rafRef.current = requestAnimationFrame(() => {
      rafRef.current = requestAnimationFrame(() => {
        setIsVisible(true);
      });
    });
  }, []);

  const handleClose = useCallback(() => {
    setIsVisible(false);
    timerRef.current = setTimeout(() => setShouldRender(false), duration);
  }, [duration]);

  /* eslint-disable react-hooks/set-state-in-effect -- mount/unmount animation requires setState in effect */
  useEffect(() => {
    if (open) {
      handleOpen();
    } else {
      handleClose();
    }
    return () => {
      if (timerRef.current) clearTimeout(timerRef.current);
      if (rafRef.current) cancelAnimationFrame(rafRef.current);
    };
  }, [open, handleOpen, handleClose]);
  /* eslint-enable react-hooks/set-state-in-effect */

  return [shouldRender, isVisible];
}

export function Modal({ open, onClose, title, children, size = "md", footer }: ModalProps) {
  const overlayRef = useRef<HTMLDivElement>(null);
  const [shouldRender, isVisible] = useAnimationMount(open);

  useEffect(() => {
    const handleEsc = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    if (open) {
      document.addEventListener("keydown", handleEsc);
      document.body.style.overflow = "hidden";
    }
    return () => {
      document.removeEventListener("keydown", handleEsc);
      document.body.style.overflow = "";
    };
  }, [open, onClose]);

  if (!shouldRender) return null;

  return (
    <div
      ref={overlayRef}
      className={cn(
        "fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4 transition-opacity duration-150",
        isVisible ? "opacity-100" : "opacity-0"
      )}
      onClick={(e) => {
        if (e.target === overlayRef.current) onClose();
      }}
    >
      <div
        className={cn(
          "w-full rounded-lg bg-white shadow-xl transition-all duration-150",
          sizeClasses[size],
          isVisible
            ? "scale-100 opacity-100"
            : "scale-95 opacity-0"
        )}
      >
        <div className="flex items-center justify-between border-b px-6 py-4">
          <h2 className="text-lg font-semibold text-gray-900">{title}</h2>
          <button
            onClick={onClose}
            className="rounded-md p-1 text-gray-400 hover:bg-gray-100 hover:text-gray-600"
          >
            <X className="h-5 w-5" />
          </button>
        </div>
        <div className="px-6 py-4">{children}</div>
        {footer && (
          <div className="flex items-center justify-end gap-3 border-t px-6 py-4">
            {footer}
          </div>
        )}
      </div>
    </div>
  );
}
