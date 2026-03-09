"use client";

import { useState } from "react";
import { Plus } from "lucide-react";
import { useTranslations } from "next-intl";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { Modal } from "@/components/ui/modal";
import { FeedbackForm } from "@/components/lowcode/feedback/FeedbackForm";
import { FeedbackList } from "@/components/lowcode/feedback/FeedbackList";
import type { Feedback } from "@/lib/types/lowcode";

export default function DeveloperFeedbackPage() {
  const t = useTranslations("developer");
  const tCommon = useTranslations("common");
  const [showCreate, setShowCreate] = useState(false);
  const [selectedFeedback, setSelectedFeedback] = useState<Feedback | null>(null);

  return (
    <div>
      <PageHeader
        title={t("feedbackInbox")}
        description={t("feedbackDesc")}
        actions={
          <Button onClick={() => setShowCreate(true)}>
            <Plus className="h-4 w-4" />
            {t("submitFeedback")}
          </Button>
        }
      />

      <FeedbackList onSelect={setSelectedFeedback} />

      <Modal
        open={showCreate}
        onClose={() => setShowCreate(false)}
        title={t("submitFeedback")}
        size="lg"
      >
        <FeedbackForm onSuccess={() => setShowCreate(false)} />
      </Modal>

      <Modal
        open={!!selectedFeedback}
        onClose={() => setSelectedFeedback(null)}
        title={selectedFeedback?.title || ""}
        size="lg"
      >
        {selectedFeedback && (
          <div className="space-y-4">
            <div>
              <h4 className="text-sm font-medium text-gray-700">{tCommon("type")}</h4>
              <p className="text-sm capitalize text-gray-900">{selectedFeedback.feedback_type}</p>
            </div>
            <div>
              <h4 className="text-sm font-medium text-gray-700">{tCommon("priority")}</h4>
              <p className="text-sm capitalize text-gray-900">{selectedFeedback.priority}</p>
            </div>
            <div>
              <h4 className="text-sm font-medium text-gray-700">{tCommon("description")}</h4>
              <p className="whitespace-pre-wrap text-sm text-gray-900">{selectedFeedback.description}</p>
            </div>

            {selectedFeedback.comments && selectedFeedback.comments.length > 0 && (
              <div>
                <h4 className="mb-2 text-sm font-medium text-gray-700">{tCommon("comments")}</h4>
                <div className="space-y-3">
                  {selectedFeedback.comments.map((comment) => (
                    <div key={comment.id} className="rounded-md border border-gray-100 bg-gray-50 p-3">
                      <div className="flex items-center gap-2">
                        <span className="text-sm font-medium text-gray-900">{comment.user_name}</span>
                        <span className="text-xs text-gray-400">
                          {new Date(comment.created_at).toLocaleString()}
                        </span>
                      </div>
                      <p className="mt-1 text-sm text-gray-700">{comment.content}</p>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>
        )}
      </Modal>
    </div>
  );
}
