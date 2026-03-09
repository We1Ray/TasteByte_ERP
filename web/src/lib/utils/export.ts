import * as XLSX from "xlsx";

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type ExportRow = Record<string, any>;

export function exportToExcel(
  data: ExportRow[],
  filename: string,
  sheetName = "Sheet1"
) {
  const ws = XLSX.utils.json_to_sheet(data);
  const wb = XLSX.utils.book_new();
  XLSX.utils.book_append_sheet(wb, ws, sheetName);
  XLSX.writeFile(wb, `${filename}.xlsx`);
}

export function exportToCsv(data: ExportRow[], filename: string) {
  if (data.length === 0) return;

  const headers = Object.keys(data[0]);
  const csvRows = [
    headers.join(","),
    ...data.map((row) =>
      headers
        .map((h) => {
          const val = row[h];
          const str = val == null ? "" : String(val);
          // Escape fields containing comma, quote, or newline
          if (str.includes(",") || str.includes('"') || str.includes("\n")) {
            return `"${str.replace(/"/g, '""')}"`;
          }
          return str;
        })
        .join(",")
    ),
  ];

  const blob = new Blob([csvRows.join("\n")], {
    type: "text/csv;charset=utf-8;",
  });
  downloadFile(blob, `${filename}.csv`);
}

export function downloadFile(blob: Blob, filename: string) {
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}

export async function downloadFromApi(url: string, filename: string) {
  const token = localStorage.getItem("access_token");
  const baseUrl =
    process.env.NEXT_PUBLIC_API_URL || "http://localhost:8000/api/v1";
  const response = await fetch(`${baseUrl}${url}`, {
    headers: token ? { Authorization: `Bearer ${token}` } : {},
  });
  if (!response.ok) {
    throw new Error(`Export failed: ${response.statusText}`);
  }
  const blob = await response.blob();
  downloadFile(blob, filename);
}
