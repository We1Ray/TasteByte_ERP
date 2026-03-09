"use client";

import { useParams } from "next/navigation";
import { ModuleOperationView } from "@/components/lowcode/ModuleOperationView";

export default function Page() {
  const { code } = useParams();
  return <ModuleOperationView module="PP" code={code as string} />;
}
