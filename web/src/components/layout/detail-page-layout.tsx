"use client";

import { ArrowLeft } from "lucide-react";
import { useTranslations } from "next-intl";
import { PageHeader } from "@/components/layout/page-header";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { PageLoading } from "@/components/ui/loading";
import { cn } from "@/lib/utils";

interface SidebarSection {
  title: string;
  content: React.ReactNode;
}

interface DetailPageLayoutProps {
  title: string;
  subtitle?: string;
  isLoading?: boolean;
  onBack: () => void;
  actions?: React.ReactNode;
  children: React.ReactNode;
  sidebar?: SidebarSection[];
}

export function DetailPageLayout({
  title,
  subtitle,
  isLoading,
  onBack,
  actions,
  children,
  sidebar,
}: DetailPageLayoutProps) {
  const t = useTranslations("layout");

  if (isLoading) {
    return <PageLoading />;
  }

  return (
    <div>
      <PageHeader
        title={title}
        description={subtitle}
        actions={
          <div className="flex gap-2">
            <Button variant="secondary" onClick={onBack}>
              <ArrowLeft className="h-4 w-4" />
              {t("back")}
            </Button>
            {actions}
          </div>
        }
      />

      <div
        className={cn(
          "grid grid-cols-1 gap-6",
          sidebar && sidebar.length > 0 && "lg:grid-cols-3"
        )}
      >
        <div className={cn(sidebar && sidebar.length > 0 && "lg:col-span-2")}>
          {children}
        </div>

        {sidebar && sidebar.length > 0 && (
          <div className="space-y-6">
            {sidebar.map((section) => (
              <Card key={section.title}>
                <CardHeader>
                  <CardTitle>{section.title}</CardTitle>
                </CardHeader>
                {section.content}
              </Card>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
