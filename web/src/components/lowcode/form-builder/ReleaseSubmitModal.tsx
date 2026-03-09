"use client";

import { useState } from "react";
import { useTranslations } from "next-intl";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Modal } from "@/components/ui/modal";
import { useApiMutation, useInvalidateQueries } from "@/lib/hooks/use-api-query";
import { releasesApi } from "@/lib/api/lowcode";

interface ReleaseSubmitModalProps {
  open: boolean;
  onClose: () => void;
  operationId: string;
}

export function ReleaseSubmitModal({ open, onClose, operationId }: ReleaseSubmitModalProps) {
  const invalidate = useInvalidateQueries();
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");

  const createMutation = useApiMutation(
    async ({ submit }: { submit: boolean }) => {
      const release = await releasesApi.create({
        operation_id: operationId,
        title,
        description,
      } as never);
      if (submit) {
        await releasesApi.submit(release.id);
      }
      return release;
    },
    {
      onSuccess: () => {
        invalidate(["lowcode", "releases"]);
        setTitle("");
        setDescription("");
        onClose();
      },
    }
  );

  return (
    <Modal
      open={open}
      onClose={onClose}
      title={t("releaseTitle")}
      size="lg"
      footer={
        <>
          <Button variant="secondary" onClick={onClose}>
            {tCommon("cancel")}
          </Button>
          <Button
            variant="secondary"
            onClick={() => createMutation.mutate({ submit: false })}
            loading={createMutation.isPending}
            disabled={!title.trim()}
          >
            {t("saveDraft")}
          </Button>
          <Button
            onClick={() => createMutation.mutate({ submit: true })}
            loading={createMutation.isPending}
            disabled={!title.trim()}
          >
            {t("createSubmit")}
          </Button>
        </>
      }
    >
      <div className="space-y-4">
        <p className="text-sm text-gray-500">
          {t("releaseSubmitDescription")}
        </p>
        <Input
          label={t("releaseTitleLabel")}
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          placeholder={t("releaseTitleExample")}
        />
        <div>
          <label className="mb-1 block text-sm font-medium text-gray-700">
            {tCommon("description")}
          </label>
          <textarea
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            rows={3}
            placeholder={t("releaseDescriptionPlaceholder")}
            className="block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
          />
        </div>
      </div>
    </Modal>
  );
}
