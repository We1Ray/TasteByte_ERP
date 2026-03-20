"use client";

import { useState, useEffect } from "react";
import { useTranslations } from "next-intl";
import { Globe, Bell, Palette } from "lucide-react";
import { PageHeader } from "@/components/layout/page-header";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { useApiQuery } from "@/lib/hooks/use-api-query";
import { useToastMutation } from "@/lib/hooks/use-toast-mutation";
import { preferencesApi, type UserPreference } from "@/lib/api/system";

export default function PreferencesPage() {
  const t = useTranslations("admin");
  const tCommon = useTranslations("common");
  const [prefs, setPrefs] = useState<Partial<UserPreference>>({});
  const [synced, setSynced] = useState(false);

  const { data } = useApiQuery(
    ["system", "preferences"],
    () => preferencesApi.get()
  );

  useEffect(() => {
    if (data && !synced) {
      setPrefs(data);
      setSynced(true);
    }
  }, [data, synced]);

  const saveMutation = useToastMutation(
    () => preferencesApi.update(prefs),
    {
      successMessage: tCommon("saveSuccess"),
      invalidateKeys: ["system", "preferences"],
    }
  );

  return (
    <div>
      <PageHeader
        title={t("userPreferences")}
        description={t("userPreferencesDesc")}
      />

      <Card className="mb-6">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Globe className="h-4 w-4" />
            {t("localization")}
          </CardTitle>
        </CardHeader>
        <div className="space-y-4">
          <Select
            label={t("language")}
            value={prefs.language || "zh-TW"}
            onChange={(e) =>
              setPrefs({ ...prefs, language: e.target.value })
            }
            options={[
              { value: "zh-TW", label: "繁體中文" },
              { value: "en", label: "English" },
            ]}
          />
          <Select
            label={t("timezone")}
            value={prefs.timezone || "Asia/Taipei"}
            onChange={(e) =>
              setPrefs({ ...prefs, timezone: e.target.value })
            }
            options={[
              { value: "Asia/Taipei", label: "Asia/Taipei (UTC+8)" },
              { value: "Asia/Tokyo", label: "Asia/Tokyo (UTC+9)" },
              { value: "America/New_York", label: "America/New_York (UTC-5)" },
              { value: "Europe/London", label: "Europe/London (UTC+0)" },
            ]}
          />
          <Select
            label={t("dateFormat")}
            value={prefs.date_format || "YYYY-MM-DD"}
            onChange={(e) =>
              setPrefs({ ...prefs, date_format: e.target.value })
            }
            options={[
              { value: "YYYY-MM-DD", label: "2024-03-20" },
              { value: "DD/MM/YYYY", label: "20/03/2024" },
              { value: "MM/DD/YYYY", label: "03/20/2024" },
              { value: "YYYY/MM/DD", label: "2024/03/20" },
            ]}
          />
        </div>
      </Card>

      <Card className="mb-6">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Palette className="h-4 w-4" />
            {t("appearance")}
          </CardTitle>
        </CardHeader>
        <div className="space-y-4">
          <Select
            label={t("theme")}
            value={prefs.theme || "light"}
            onChange={(e) =>
              setPrefs({ ...prefs, theme: e.target.value })
            }
            options={[
              { value: "light", label: t("themeLight") },
              { value: "dark", label: t("themeDark") },
              { value: "system", label: t("themeSystem") },
            ]}
          />
          <Input
            label={t("defaultPageSize")}
            type="number"
            value={String(prefs.page_size || 20)}
            onChange={(e) =>
              setPrefs({
                ...prefs,
                page_size: Number(e.target.value) || 20,
              })
            }
          />
        </div>
      </Card>

      <Card className="mb-6">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Bell className="h-4 w-4" />
            {t("notificationSettings")}
          </CardTitle>
        </CardHeader>
        <div className="space-y-3">
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={prefs.notifications_enabled ?? true}
              onChange={(e) =>
                setPrefs({
                  ...prefs,
                  notifications_enabled: e.target.checked,
                })
              }
              className="h-4 w-4 rounded border-gray-300 text-blue-600"
            />
            <span className="text-sm text-gray-700">
              {t("enableNotifications")}
            </span>
          </label>
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={prefs.email_notifications ?? true}
              onChange={(e) =>
                setPrefs({
                  ...prefs,
                  email_notifications: e.target.checked,
                })
              }
              className="h-4 w-4 rounded border-gray-300 text-blue-600"
            />
            <span className="text-sm text-gray-700">
              {t("enableEmailNotifications")}
            </span>
          </label>
        </div>
      </Card>

      <div className="flex justify-end">
        <Button
          onClick={() => saveMutation.mutate(undefined)}
          loading={saveMutation.isPending}
        >
          {tCommon("save")}
        </Button>
      </div>
    </div>
  );
}
