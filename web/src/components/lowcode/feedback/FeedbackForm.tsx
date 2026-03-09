"use client";

import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { useTranslations } from "next-intl";
import { Input } from "@/components/ui/input";
import { Select } from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { feedbackApi } from "@/lib/api/lowcode";
import type { Feedback } from "@/lib/types/lowcode";

const feedbackSchema = z.object({
  feedback_type: z.string().min(1, "Type is required"),
  title: z.string().min(1, "Title is required").max(200),
  description: z.string().min(1, "Description is required"),
  priority: z.string().min(1, "Priority is required"),
  project_id: z.string().optional(),
  operation_id: z.string().optional(),
});

type FeedbackFormData = z.infer<typeof feedbackSchema>;

interface FeedbackFormProps {
  projectId?: string;
  operationId?: string;
  onSuccess?: () => void;
}

export function FeedbackForm({ projectId, operationId, onSuccess }: FeedbackFormProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const invalidate = useInvalidateQueries();

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors },
  } = useForm<FeedbackFormData>({
    resolver: zodResolver(feedbackSchema),
    defaultValues: {
      feedback_type: "",
      title: "",
      description: "",
      priority: "medium",
      project_id: projectId,
      operation_id: operationId,
    },
  });

  const { mutateAsync, isPending } = useApiMutation(
    (data: FeedbackFormData) => feedbackApi.create(data as unknown as Partial<Feedback>),
    {
      onSuccess: () => {
        invalidate(["lowcode", "feedback"]);
        reset();
        onSuccess?.();
      },
    }
  );

  return (
    <form onSubmit={handleSubmit((data) => mutateAsync(data))} className="space-y-4">
      <Select
        {...register("feedback_type")}
        label={tCommon("type")}
        required
        error={errors.feedback_type?.message}
        placeholder="Select type"
        options={[
          { value: "bug", label: t("feedbackTypeBug") },
          { value: "feature", label: t("feedbackTypeFeature") },
          { value: "improvement", label: t("feedbackTypeImprovement") },
        ]}
      />

      <Input
        {...register("title")}
        label={tCommon("name")}
        required
        error={errors.title?.message}
        placeholder={t("feedbackTitlePlaceholder")}
      />

      <div className="w-full">
        <label className="mb-1 block text-sm font-medium text-gray-700">
          {tCommon("description")} <span className="text-red-500">*</span>
        </label>
        <textarea
          {...register("description")}
          rows={4}
          placeholder={t("feedbackDescPlaceholder")}
          className="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
        />
        {errors.description && (
          <p className="mt-1 text-sm text-red-600">{errors.description.message}</p>
        )}
      </div>

      <Select
        {...register("priority")}
        label={tCommon("priority")}
        required
        error={errors.priority?.message}
        options={[
          { value: "low", label: t("priorityLow") },
          { value: "medium", label: t("priorityMedium") },
          { value: "high", label: t("priorityHigh") },
          { value: "critical", label: t("priorityCritical") },
        ]}
      />

      <div className="flex justify-end gap-3">
        <Button type="button" variant="secondary" onClick={() => reset()}>
          {tCommon("cancel")}
        </Button>
        <Button type="submit" loading={isPending}>
          {t("submitFeedback")}
        </Button>
      </div>
    </form>
  );
}
