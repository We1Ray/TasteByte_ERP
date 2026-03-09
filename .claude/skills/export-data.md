# export-data Skill

將處理過的資料匯出為各種格式。

## Input
- `data`: 要匯出的資料
- `format`: 匯出格式（json, csv, excel, pdf, markdown）
- `options`: 格式特定選項

## Supported Formats

### JSON 匯出 (Rust)
```rust
use axum::{
    http::header,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

pub async fn export_json<T: Serialize>(data: &T, pretty: bool) -> Result<Response, AppError> {
    let json_bytes = if pretty {
        serde_json::to_vec_pretty(data)?
    } else {
        serde_json::to_vec(data)?
    };

    Ok((
        [
            (header::CONTENT_TYPE, "application/json; charset=utf-8"),
            (header::CONTENT_DISPOSITION, "attachment; filename=\"export.json\""),
        ],
        json_bytes,
    ).into_response())
}
```

### CSV 匯出 (Rust)
```rust
use csv::WriterBuilder;
use serde::Serialize;

pub fn export_csv<T: Serialize>(records: &[T], delimiter: u8) -> Result<Vec<u8>, AppError> {
    let mut wtr = WriterBuilder::new()
        .delimiter(delimiter)
        .from_writer(Vec::new());

    // Write BOM for Excel compatibility
    let mut output = vec![0xEF, 0xBB, 0xBF];

    for record in records {
        wtr.serialize(record)?;
    }

    output.extend(wtr.into_inner()?);
    Ok(output)
}

// Axum handler
pub async fn handle_csv_export(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ExportParams>,
) -> Result<Response, AppError> {
    let orders = state.sd_service.list_sales_orders(params.status.as_deref(), 1, 10000).await?;
    let csv_bytes = export_csv(&orders, b',')?;

    Ok((
        [
            (header::CONTENT_TYPE, "text/csv; charset=utf-8"),
            (header::CONTENT_DISPOSITION, "attachment; filename=\"sales_orders.csv\""),
        ],
        csv_bytes,
    ).into_response())
}
```

### Excel 匯出 (Rust)
```rust
use rust_xlsxwriter::{Workbook, Format};

pub fn export_excel(sheets: Vec<ExcelSheet>) -> Result<Vec<u8>, AppError> {
    let mut workbook = Workbook::new();

    let header_format = Format::new()
        .set_bold()
        .set_background_color(rust_xlsxwriter::Color::RGB(0xDAEEF3));

    for sheet_data in &sheets {
        let worksheet = workbook.add_worksheet();
        worksheet.set_name(&sheet_data.name)?;

        // Write headers
        for (col, header) in sheet_data.headers.iter().enumerate() {
            worksheet.write_string_with_format(0, col as u16, header, &header_format)?;
        }

        // Write data rows
        for (row_idx, row) in sheet_data.rows.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                match cell {
                    CellValue::String(s) => worksheet.write_string((row_idx + 1) as u32, col_idx as u16, s)?,
                    CellValue::Number(n) => worksheet.write_number((row_idx + 1) as u32, col_idx as u16, *n)?,
                    CellValue::Decimal(d) => worksheet.write_number((row_idx + 1) as u32, col_idx as u16, d.to_f64().unwrap_or(0.0))?,
                };
            }
        }
    }

    let buffer = workbook.save_to_buffer()?;
    Ok(buffer)
}

pub struct ExcelSheet {
    pub name: String,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<CellValue>>,
}

pub enum CellValue {
    String(String),
    Number(f64),
    Decimal(rust_decimal::Decimal),
}
```

### PDF 匯出 (Rust)
```rust
use genpdf::{
    elements::{Paragraph, TableLayout},
    fonts, Document, SimplePageDecorator,
};

pub fn export_pdf(title: &str, headers: &[String], rows: &[Vec<String>]) -> Result<Vec<u8>, AppError> {
    let font_family = fonts::from_files("assets/fonts", "NotoSansCJK", None)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let mut doc = Document::new(font_family);
    doc.set_title(title);

    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    // Title
    doc.push(Paragraph::new(title).styled(genpdf::style::Style::new().bold().with_font_size(16)));

    // Table
    let mut table = TableLayout::new(vec![1; headers.len()]);
    table.set_cell_decorator(genpdf::elements::FrameCellDecorator::new(true, true, false));

    // Header row
    let mut header_row = table.row();
    for h in headers {
        header_row.push_element(Paragraph::new(h).styled(genpdf::style::Style::new().bold()));
    }
    header_row.push().map_err(|e| AppError::Internal(e.to_string()))?;

    // Data rows
    for row in rows {
        let mut table_row = table.row();
        for cell in row {
            table_row.push_element(Paragraph::new(cell));
        }
        table_row.push().map_err(|e| AppError::Internal(e.to_string()))?;
    }

    doc.push(table);

    let mut buffer = Vec::new();
    doc.render(&mut buffer).map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(buffer)
}
```

### Markdown 匯出 (TypeScript - Next.js)
```typescript
export function exportMarkdown(data: {
  title: string;
  headers?: string[];
  rows?: string[][];
  text?: string;
}): string {
  const lines: string[] = [];

  lines.push(`# ${data.title}\n`);

  if (data.headers && data.rows) {
    lines.push("| " + data.headers.join(" | ") + " |");
    lines.push("| " + data.headers.map(() => "---").join(" | ") + " |");

    for (const row of data.rows) {
      lines.push("| " + row.join(" | ") + " |");
    }
  }

  if (data.text) {
    lines.push("\n## Content\n");
    lines.push(data.text);
  }

  return lines.join("\n");
}
```

## Output
```json
{
    "success": true,
    "format": "excel",
    "filename": "export_20240101_120000.xlsx",
    "size": 12345,
    "download_url": "/api/v1/exports/abc123/download"
}
```
