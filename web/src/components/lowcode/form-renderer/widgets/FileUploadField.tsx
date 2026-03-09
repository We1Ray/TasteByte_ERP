"use client";

import { useState, useRef } from "react";
import { Upload, X, FileIcon } from "lucide-react";
import { useTranslations } from "next-intl";
import { cn } from "@/lib/utils";
import { filesApi } from "@/lib/api/lowcode";
import type { FieldDefinition, FileAttachment } from "@/lib/types/lowcode";

interface FileUploadFieldProps {
  field: FieldDefinition;
  value: string;
  onChange: (value: string) => void;
  error?: string;
  disabled?: boolean;
}

export function FileUploadField({ field, value, onChange, error, disabled }: FileUploadFieldProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const [uploading, setUploading] = useState(false);
  const [fileInfo, setFileInfo] = useState<FileAttachment | null>(null);
  const [dragOver, setDragOver] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  const handleFile = async (file: File) => {
    const v = field.validation;
    if (v.max_file_size && file.size > v.max_file_size) {
      return;
    }
    if (v.allowed_extensions?.length) {
      const ext = file.name.split(".").pop()?.toLowerCase();
      if (ext && !v.allowed_extensions.includes(ext)) {
        return;
      }
    }

    setUploading(true);
    try {
      const result = await filesApi.upload(file);
      setFileInfo(result);
      onChange(result.id);
    } catch {
      // upload failed
    } finally {
      setUploading(false);
    }
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    setDragOver(false);
    if (disabled) return;
    const file = e.dataTransfer.files[0];
    if (file) handleFile(file);
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) handleFile(file);
  };

  const handleRemove = () => {
    setFileInfo(null);
    onChange("");
    if (inputRef.current) inputRef.current.value = "";
  };

  return (
    <div className="w-full">
      {field.label && (
        <label className="mb-1 block text-sm font-medium text-gray-700">
          {field.label}
          {field.validation.required && <span className="ml-0.5 text-red-500">*</span>}
        </label>
      )}

      {!value ? (
        <div
          className={cn(
            "flex flex-col items-center justify-center rounded-md border-2 border-dashed px-4 py-6 text-center transition-colors",
            dragOver ? "border-blue-500 bg-blue-50" : "border-gray-300",
            disabled ? "cursor-not-allowed bg-gray-50" : "cursor-pointer hover:border-gray-400"
          )}
          onDragOver={(e) => { e.preventDefault(); setDragOver(true); }}
          onDragLeave={() => setDragOver(false)}
          onDrop={handleDrop}
          onClick={() => !disabled && inputRef.current?.click()}
        >
          <Upload className="mb-2 h-6 w-6 text-gray-400" />
          {uploading ? (
            <p className="text-sm text-gray-500">{tCommon("loading")}</p>
          ) : (
            <>
              <p className="text-sm text-gray-600">{t("csvDragDrop")}</p>
              {field.validation.allowed_extensions && (
                <p className="mt-1 text-xs text-gray-400">
                  {t("allowedExtensions")}: {field.validation.allowed_extensions.join(", ")}
                </p>
              )}
            </>
          )}
          <input
            ref={inputRef}
            type="file"
            className="hidden"
            onChange={handleInputChange}
            disabled={disabled}
            accept={field.validation.allowed_extensions?.map((e) => `.${e}`).join(",")}
          />
        </div>
      ) : (
        <div className="flex items-center gap-3 rounded-md border border-gray-200 bg-gray-50 px-3 py-2">
          <FileIcon className="h-5 w-5 text-gray-400" />
          <span className="flex-1 truncate text-sm text-gray-700">
            {fileInfo?.filename || value}
          </span>
          {!disabled && (
            <button type="button" onClick={handleRemove} className="text-gray-400 hover:text-red-500">
              <X className="h-4 w-4" />
            </button>
          )}
        </div>
      )}

      {error && <p className="mt-1 text-sm text-red-600">{error}</p>}
      {field.help_text && !error && <p className="mt-1 text-sm text-gray-500">{field.help_text}</p>}
    </div>
  );
}
