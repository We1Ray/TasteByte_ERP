"use client";

import { useState, useCallback } from "react";
import { useTranslations } from "next-intl";
import { Upload, FileText, CheckCircle, AlertCircle } from "lucide-react";
import Papa from "papaparse";
import { Modal } from "@/components/ui/modal";
import { Button } from "@/components/ui/button";
import { Select } from "@/components/ui/select";
import { importExportApi } from "@/lib/api/lowcode";
import { toast } from "sonner";
import type { ListColumn } from "@/lib/types/lowcode";

interface CsvImportModalProps {
  open: boolean;
  onClose: () => void;
  operationCode: string;
  columns: ListColumn[];
  onImportComplete?: () => void;
}

type ImportStep = "upload" | "mapping" | "preview" | "result";

interface ImportResult {
  inserted: number;
  skipped?: number;
  errors: string[];
  row_errors?: { row_index: number; errors: { field_name: string; field_label: string; message: string }[] }[];
}

export function CsvImportModal({
  open,
  onClose,
  operationCode,
  columns,
  onImportComplete,
}: CsvImportModalProps) {
  const t = useTranslations("lowcode");
  const tCommon = useTranslations("common");
  const [step, setStep] = useState<ImportStep>("upload");
  const [csvHeaders, setCsvHeaders] = useState<string[]>([]);
  const [csvData, setCsvData] = useState<Record<string, string>[]>([]);
  const [columnMapping, setColumnMapping] = useState<Record<string, string>>(
    {}
  );
  const [importing, setImporting] = useState(false);
  const [result, setResult] = useState<ImportResult | null>(null);
  const [fileName, setFileName] = useState("");
  const [dryRun, setDryRun] = useState(true);
  const [skipInvalid, setSkipInvalid] = useState(false);

  const resetState = () => {
    setStep("upload");
    setCsvHeaders([]);
    setCsvData([]);
    setColumnMapping({});
    setImporting(false);
    setResult(null);
    setFileName("");
    setDryRun(true);
    setSkipInvalid(false);
  };

  const handleClose = () => {
    resetState();
    onClose();
  };

  const handleFileUpload = useCallback(
    (file: File) => {
      setFileName(file.name);
      Papa.parse(file, {
        header: true,
        skipEmptyLines: true,
        complete: (results) => {
          const headers = results.meta.fields || [];
          const data = results.data as Record<string, string>[];

          setCsvHeaders(headers);
          setCsvData(data);

          // Auto-map columns by matching names
          const autoMapping: Record<string, string> = {};
          headers.forEach((header) => {
            const match = columns.find(
              (col) =>
                col.field_key.toLowerCase() === header.toLowerCase() ||
                col.label.toLowerCase() === header.toLowerCase()
            );
            if (match) {
              autoMapping[header] = match.field_key;
            }
          });
          setColumnMapping(autoMapping);
          setStep("mapping");
        },
        error: () => {
          toast.error(t("csvParseFailed"));
        },
      });
    },
    [columns]
  );

  const handleDrop = useCallback(
    (e: React.DragEvent) => {
      e.preventDefault();
      const file = e.dataTransfer.files[0];
      if (file && file.name.endsWith(".csv")) {
        handleFileUpload(file);
      } else {
        toast.error(t("csvFileRequired"));
      }
    },
    [handleFileUpload]
  );

  const handleFileInput = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const file = e.target.files?.[0];
      if (file) {
        handleFileUpload(file);
      }
    },
    [handleFileUpload]
  );

  const mappedData = csvData.map((row) => {
    const mapped: Record<string, unknown> = {};
    Object.entries(columnMapping).forEach(([csvCol, targetCol]) => {
      if (targetCol) {
        mapped[targetCol] = row[csvCol];
      }
    });
    return mapped;
  });

  const previewRows = mappedData.slice(0, 5);

  const handleImport = async () => {
    setImporting(true);
    try {
      const importResult = await importExportApi.bulkImport(
        operationCode,
        mappedData,
        { dry_run: dryRun, skip_invalid: skipInvalid }
      );
      setResult(importResult);
      setStep("result");
      if (importResult.inserted > 0) {
        toast.success(t("importSuccessMsg", { count: importResult.inserted }));
        onImportComplete?.();
      }
    } catch {
      toast.error(t("exportFailed"));
    } finally {
      setImporting(false);
    }
  };

  const columnOptions = [
    { value: "", label: t("skipColumn") },
    ...columns.map((c) => ({
      value: c.field_key,
      label: `${c.label} (${c.field_key})`,
    })),
  ];

  return (
    <Modal
      open={open}
      onClose={handleClose}
      title={t("csvImportTitle")}
      size="xl"
      footer={
        <div className="flex items-center gap-2">
          {step === "mapping" && (
            <>
              <Button variant="secondary" onClick={() => setStep("upload")}>
                {tCommon("back")}
              </Button>
              <Button onClick={() => setStep("preview")}>
                {t("nextPreview")}
              </Button>
            </>
          )}
          {step === "preview" && (
            <>
              <Button variant="secondary" onClick={() => setStep("mapping")}>
                {tCommon("back")}
              </Button>
              <Button
                onClick={handleImport}
                loading={importing}
                disabled={mappedData.length === 0}
              >
                {t("importRecords", { count: mappedData.length })}
              </Button>
            </>
          )}
          {step === "result" && result && dryRun && (
            <Button
              onClick={() => {
                setDryRun(false);
                setStep("preview");
                setResult(null);
              }}
            >
              {t("proceedImport")}
            </Button>
          )}
          {step === "result" && (
            <Button onClick={handleClose}>{tCommon("close")}</Button>
          )}
        </div>
      }
    >
      {/* Step: Upload */}
      {step === "upload" && (
        <div
          className="rounded-lg border-2 border-dashed border-gray-300 p-12 text-center transition-colors hover:border-blue-400"
          onDragOver={(e) => e.preventDefault()}
          onDrop={handleDrop}
        >
          <Upload className="mx-auto h-10 w-10 text-gray-400" />
          <h3 className="mt-3 text-sm font-semibold text-gray-700">
            {t("uploadCsv")}
          </h3>
          <p className="mt-1 text-xs text-gray-500">
            {t("csvDragDrop")}
          </p>
          <label className="mt-4 inline-block">
            <input
              type="file"
              accept=".csv"
              onChange={handleFileInput}
              className="hidden"
            />
            <span className="cursor-pointer rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700">
              {t("chooseFile")}
            </span>
          </label>
        </div>
      )}

      {/* Step: Column Mapping */}
      {step === "mapping" && (
        <div className="space-y-4">
          <div className="flex items-center gap-2 rounded-md bg-blue-50 p-3">
            <FileText className="h-4 w-4 text-blue-600" />
            <p className="text-sm text-blue-700">
              {t("csvFileDetails", { fileName, rows: csvData.length, cols: csvHeaders.length })}
            </p>
          </div>

          <h4 className="text-sm font-semibold text-gray-900">
            {t("columnMapping")}
          </h4>
          <p className="text-xs text-gray-500">
            {t("columnMappingDesc")}
          </p>

          <div className="max-h-80 space-y-2 overflow-y-auto">
            {csvHeaders.map((header) => (
              <div
                key={header}
                className="flex items-center gap-3 rounded-md border border-gray-200 bg-white p-3"
              >
                <div className="w-1/3">
                  <p className="text-sm font-medium text-gray-700">
                    {header}
                  </p>
                  <p className="text-xs text-gray-400">
                    e.g., {csvData[0]?.[header] ?? ""}
                  </p>
                </div>
                <span className="text-gray-400">-&gt;</span>
                <div className="flex-1">
                  <Select
                    value={columnMapping[header] ?? ""}
                    onChange={(e) =>
                      setColumnMapping((prev) => ({
                        ...prev,
                        [header]: e.target.value,
                      }))
                    }
                    options={columnOptions}
                  />
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Step: Preview */}
      {step === "preview" && (
        <div className="space-y-4">
          <h4 className="text-sm font-semibold text-gray-900">
            {t("previewTitle")}
          </h4>
          <div className="overflow-x-auto rounded-md border border-gray-200">
            <table className="min-w-full text-xs">
              <thead className="bg-gray-50">
                <tr>
                  {Object.values(columnMapping)
                    .filter(Boolean)
                    .map((col) => {
                      const colDef = columns.find(
                        (c) => c.field_key === col
                      );
                      return (
                        <th
                          key={col}
                          className="px-3 py-2 text-left font-semibold text-gray-600"
                        >
                          {colDef?.label || col}
                        </th>
                      );
                    })}
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-100">
                {previewRows.map((row, i) => (
                  <tr key={i}>
                    {Object.values(columnMapping)
                      .filter(Boolean)
                      .map((col) => (
                        <td key={col} className="px-3 py-2 text-gray-700">
                          {String(row[col] ?? "")}
                        </td>
                      ))}
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
          <p className="text-xs text-gray-500">
            {t("totalRecords", { count: mappedData.length })}
          </p>

          <div className="flex items-center gap-4 rounded-md bg-gray-50 p-3">
            <label className="flex items-center gap-2">
              <input
                type="checkbox"
                checked={dryRun}
                onChange={(e) => setDryRun(e.target.checked)}
                className="h-4 w-4 rounded border-gray-300 text-blue-600"
              />
              <span className="text-sm text-gray-700">{t("dryRun")}</span>
            </label>
            <label className="flex items-center gap-2">
              <input
                type="checkbox"
                checked={skipInvalid}
                onChange={(e) => setSkipInvalid(e.target.checked)}
                className="h-4 w-4 rounded border-gray-300 text-blue-600"
              />
              <span className="text-sm text-gray-700">{t("skipInvalid")}</span>
            </label>
          </div>

          <div className="rounded-md bg-blue-50 p-3">
            <p className="text-sm text-blue-700">
              {t("importSummary", { total: mappedData.length })}
            </p>
          </div>
        </div>
      )}

      {/* Step: Result */}
      {step === "result" && result && (
        <div className="space-y-4">
          {result && dryRun && result.inserted > 0 && (
            <div className="flex items-start gap-3 rounded-md bg-blue-50 p-4">
              <CheckCircle className="h-5 w-5 text-blue-600 mt-0.5" />
              <div>
                <p className="text-sm font-medium text-blue-800">{t("dryRunComplete")}</p>
                <p className="text-sm text-blue-700">
                  {t("dryRunSummary", { valid: result.inserted, invalid: result.skipped || 0 })}
                </p>
              </div>
            </div>
          )}

          {result.inserted > 0 && !dryRun && (
            <div className="flex items-start gap-3 rounded-md bg-green-50 p-4">
              <CheckCircle className="h-5 w-5 text-green-600 mt-0.5" />
              <div>
                <p className="text-sm font-medium text-green-800">
                  {t("importSuccess")}
                </p>
                <p className="text-sm text-green-700">
                  {t("importSuccessMsg", { count: result.inserted })}
                </p>
              </div>
            </div>
          )}

          {result.errors.length > 0 && (
            <div className="rounded-md bg-red-50 p-4">
              <div className="flex items-start gap-3">
                <AlertCircle className="h-5 w-5 text-red-600 mt-0.5" />
                <div>
                  <p className="text-sm font-medium text-red-800">
                    {t("importErrors", { count: result.errors.length })}
                  </p>
                  <ul className="mt-2 max-h-40 space-y-1 overflow-y-auto text-xs text-red-700">
                    {result.errors.map((err, i) => (
                      <li key={i}>{err}</li>
                    ))}
                  </ul>
                </div>
              </div>
            </div>
          )}

          {result.row_errors && result.row_errors.length > 0 && (
            <div className="mt-3 max-h-60 space-y-2 overflow-y-auto">
              {result.row_errors.map((re: { row_index: number; errors: { field_name: string; field_label: string; message: string }[] }, i: number) => (
                <div key={i} className="rounded-md border border-red-200 bg-red-50 p-2">
                  <p className="text-xs font-medium text-red-700">
                    {t("rowError", { row: re.row_index + 1 })}
                  </p>
                  <ul className="mt-1 space-y-0.5">
                    {re.errors.map((err: { field_label: string; message: string }, j: number) => (
                      <li key={j} className="text-xs text-red-600">
                        {err.field_label}: {err.message}
                      </li>
                    ))}
                  </ul>
                </div>
              ))}
            </div>
          )}
        </div>
      )}
    </Modal>
  );
}
