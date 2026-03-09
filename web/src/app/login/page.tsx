"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { useForm } from "react-hook-form";
import { z } from "zod/v4";
import { standardSchemaResolver } from "@hookform/resolvers/standard-schema";
import { useTranslations } from "next-intl";
import { authApi } from "@/lib/api/auth";
import { useAuthStore } from "@/lib/stores/auth-store";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

const loginSchema = z.object({
  username: z.string().min(1, "Username is required"),
  password: z.string().min(1, "Password is required"),
});

type LoginForm = z.infer<typeof loginSchema>;

export default function LoginPage() {
  const router = useRouter();
  const setAuth = useAuthStore((state) => state.setAuth);
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);
  const t = useTranslations("auth");
  const tNav = useTranslations("nav");

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<LoginForm>({
    resolver: standardSchemaResolver(loginSchema),
  });

  const onSubmit = async (data: LoginForm) => {
    setError("");
    setLoading(true);
    try {
      const response = await authApi.login(data);
      setAuth(response.user, response.access_token, response.refresh_token);
      router.push("/dashboard");
    } catch {
      setError(t("loginError"));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex min-h-screen">
      {/* Left panel - branding */}
      <div className="hidden w-1/2 bg-slate-900 lg:flex lg:flex-col lg:items-center lg:justify-center">
        <div className="text-center">
          <div className="mx-auto mb-6 flex h-20 w-20 items-center justify-center rounded-2xl bg-blue-600 text-3xl font-bold text-white">
            TB
          </div>
          <h1 className="text-4xl font-bold text-white">TasteByte ERP</h1>
          <p className="mt-3 text-lg text-slate-400">
            Enterprise Resource Planning System
          </p>
          <div className="mt-12 grid grid-cols-2 gap-4 text-sm text-slate-400">
            <div className="rounded-lg bg-slate-800 px-4 py-3">{tNav("fi")}</div>
            <div className="rounded-lg bg-slate-800 px-4 py-3">{tNav("mm")}</div>
            <div className="rounded-lg bg-slate-800 px-4 py-3">{tNav("sd")}</div>
            <div className="rounded-lg bg-slate-800 px-4 py-3">{tNav("pp")}</div>
            <div className="rounded-lg bg-slate-800 px-4 py-3">{tNav("hr")}</div>
            <div className="rounded-lg bg-slate-800 px-4 py-3">{tNav("qm")}</div>
          </div>
        </div>
      </div>

      {/* Right panel - login form */}
      <div className="flex w-full items-center justify-center bg-gray-50 px-6 lg:w-1/2">
        <div className="w-full max-w-md">
          <div className="mb-8 lg:hidden">
            <div className="mx-auto mb-4 flex h-14 w-14 items-center justify-center rounded-xl bg-blue-600 text-xl font-bold text-white">
              TB
            </div>
            <h1 className="text-center text-2xl font-bold text-gray-900">TasteByte ERP</h1>
          </div>

          <div className="rounded-xl bg-white p-8 shadow-sm ring-1 ring-gray-200">
            <h2 className="mb-1 text-xl font-semibold text-gray-900">{t("welcomeBack")}</h2>
            <p className="mb-6 text-sm text-gray-500">{t("signInDescription")}</p>

            {error && (
              <div className="mb-4 rounded-md bg-red-50 px-4 py-3 text-sm text-red-700">
                {error}
              </div>
            )}

            <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
              <Input
                label={t("username")}
                placeholder={t("usernamePlaceholder")}
                error={errors.username?.message}
                {...register("username")}
              />

              <Input
                label={t("password")}
                type="password"
                placeholder={t("passwordPlaceholder")}
                error={errors.password?.message}
                {...register("password")}
              />

              <Button type="submit" className="w-full" loading={loading}>
                {t("loginButton")}
              </Button>
            </form>

            <p className="mt-6 text-center text-xs text-gray-400">
              {t("version")}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
