"use client";

import { useParams } from "next/navigation";
import { ModuleOperationView } from "@/components/lowcode/ModuleOperationView";

export default function Page() {
  const { code } = useParams();
  return <ModuleOperationView module="MM" code={code as string} />;
}
