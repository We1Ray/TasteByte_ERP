"use client";

import { Printer } from "lucide-react";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";

interface PrintButtonProps {
  className?: string;
}

export function PrintButton({ className }: PrintButtonProps) {
  return (
    <Button
      variant="secondary"
      onClick={() => window.print()}
      className={cn(className)}
    >
      <Printer className="h-4 w-4" />
      Print
    </Button>
  );
}
