"use client";

import { useEffect } from "react";
import { useEditor, EditorContent } from "@tiptap/react";
import StarterKit from "@tiptap/starter-kit";
import Link from "@tiptap/extension-link";
import Placeholder from "@tiptap/extension-placeholder";
import {
  Bold,
  Italic,
  Strikethrough,
  List,
  ListOrdered,
  Link as LinkIcon,
  Heading2,
  Code,
} from "lucide-react";
import { useTranslations } from "next-intl";
import { cn } from "@/lib/utils";
import type { FieldDefinition, RichTextFieldConfig } from "@/lib/types/lowcode";

interface RichTextFieldProps {
  field: FieldDefinition;
  value: string;
  onChange: (value: string) => void;
  error?: string;
  disabled?: boolean;
}

const DEFAULT_TOOLBAR = ["bold", "italic", "strike", "bulletList", "orderedList", "link"];

const toolbarButtons: Record<
  string,
  { icon: React.ElementType; label: string; action: (editor: ReturnType<typeof useEditor>) => void; isActive?: (editor: ReturnType<typeof useEditor>) => boolean }
> = {
  bold: {
    icon: Bold,
    label: "Bold",
    action: (editor) => editor?.chain().focus().toggleBold().run(),
    isActive: (editor) => editor?.isActive("bold") ?? false,
  },
  italic: {
    icon: Italic,
    label: "Italic",
    action: (editor) => editor?.chain().focus().toggleItalic().run(),
    isActive: (editor) => editor?.isActive("italic") ?? false,
  },
  strike: {
    icon: Strikethrough,
    label: "Strikethrough",
    action: (editor) => editor?.chain().focus().toggleStrike().run(),
    isActive: (editor) => editor?.isActive("strike") ?? false,
  },
  bulletList: {
    icon: List,
    label: "Bullet List",
    action: (editor) => editor?.chain().focus().toggleBulletList().run(),
    isActive: (editor) => editor?.isActive("bulletList") ?? false,
  },
  orderedList: {
    icon: ListOrdered,
    label: "Ordered List",
    action: (editor) => editor?.chain().focus().toggleOrderedList().run(),
    isActive: (editor) => editor?.isActive("orderedList") ?? false,
  },
  link: {
    icon: LinkIcon,
    label: "Link",
    action: (editor) => {
      if (!editor) return;
      if (editor.isActive("link")) {
        editor.chain().focus().unsetLink().run();
        return;
      }
      const url = window.prompt("Enter URL:");
      if (url) {
        editor.chain().focus().extendMarkRange("link").setLink({ href: url }).run();
      }
    },
    isActive: (editor) => editor?.isActive("link") ?? false,
  },
  heading: {
    icon: Heading2,
    label: "Heading",
    action: (editor) => editor?.chain().focus().toggleHeading({ level: 2 }).run(),
    isActive: (editor) => editor?.isActive("heading", { level: 2 }) ?? false,
  },
  code: {
    icon: Code,
    label: "Code",
    action: (editor) => editor?.chain().focus().toggleCodeBlock().run(),
    isActive: (editor) => editor?.isActive("codeBlock") ?? false,
  },
};

export function RichTextField({ field, value, onChange, error, disabled }: RichTextFieldProps) {
  const t = useTranslations("lowcode");
  const config = (field.field_config ?? {}) as RichTextFieldConfig;
  const toolbar = config.toolbar ?? DEFAULT_TOOLBAR;
  const maxLength = config.maxLength;

  const editor = useEditor({
    extensions: [
      StarterKit,
      Link.configure({ openOnClick: false }),
      Placeholder.configure({
        placeholder: field.placeholder || `${t("richText")}...`,
      }),
    ],
    content: value || "",
    editable: !disabled && !field.is_readonly,
    onUpdate: ({ editor: e }) => {
      onChange(e.getHTML());
    },
  });

  // Sync external value changes into editor
  useEffect(() => {
    if (editor && value !== editor.getHTML()) {
      editor.commands.setContent(value || "");
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [value]);

  const charCount = editor?.storage.characterCount?.characters?.() ?? editor?.getText().length ?? 0;

  return (
    <div className="w-full">
      {field.label && (
        <label className="mb-1 block text-sm font-medium text-gray-700">
          {field.label}
          {field.validation.required && <span className="ml-0.5 text-red-500">*</span>}
        </label>
      )}

      <div
        className={cn(
          "overflow-hidden rounded-md border shadow-sm",
          error ? "border-red-300" : "border-gray-300",
          disabled ? "bg-gray-50" : "bg-white"
        )}
      >
        {/* Toolbar */}
        {!disabled && !field.is_readonly && (
          <div className="flex flex-wrap gap-1 border-b border-gray-200 bg-gray-50 px-2 py-1.5">
            {toolbar.map((key) => {
              const btn = toolbarButtons[key];
              if (!btn) return null;
              const Icon = btn.icon;
              const active = btn.isActive?.(editor) ?? false;
              return (
                <button
                  key={key}
                  type="button"
                  title={btn.label}
                  onClick={() => btn.action(editor)}
                  className={cn(
                    "rounded p-1.5 text-sm transition-colors",
                    active
                      ? "bg-blue-100 text-blue-700"
                      : "text-gray-600 hover:bg-gray-200 hover:text-gray-900"
                  )}
                >
                  <Icon className="h-4 w-4" />
                </button>
              );
            })}
          </div>
        )}

        {/* Editor Area */}
        <EditorContent
          editor={editor}
          className={cn(
            "prose prose-sm max-w-none px-3 py-2",
            "min-h-[120px]",
            "[&_.tiptap]:min-h-[100px] [&_.tiptap]:outline-none",
            "[&_.tiptap_p.is-editor-empty:first-child::before]:text-gray-400 [&_.tiptap_p.is-editor-empty:first-child::before]:content-[attr(data-placeholder)]",
            "[&_.tiptap_p.is-editor-empty:first-child::before]:float-left [&_.tiptap_p.is-editor-empty:first-child::before]:pointer-events-none [&_.tiptap_p.is-editor-empty:first-child::before]:h-0"
          )}
        />

        {/* Character count */}
        {maxLength && (
          <div className="border-t border-gray-100 px-3 py-1 text-right">
            <span
              className={cn(
                "text-xs",
                charCount > maxLength ? "font-medium text-red-500" : "text-gray-400"
              )}
            >
              {charCount} / {maxLength}
            </span>
          </div>
        )}
      </div>

      {field.help_text && !error && (
        <p className="mt-1 text-sm text-gray-500">{field.help_text}</p>
      )}
      {error && <p className="mt-1 text-sm text-red-600">{error}</p>}
    </div>
  );
}
