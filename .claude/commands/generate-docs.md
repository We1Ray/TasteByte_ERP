# /generate-docs - 生成文件

自動生成 TasteByte ERP 專案文件。

## Usage
```
/generate-docs [all|api|db|erp|architecture]
```

---

## 文件類型

### 1. API 文件
- Rust Backend API endpoint 清單
- Request/Response 型別定義
- 認證要求說明
- ERP 模組 API 分組

### 2. 資料庫文件
- ER Diagram (描述)
- 表結構說明
- Migration 歷史
- 索引策略
- 連線資訊: `postgres://postgres:postgres@localhost:5432/TastyByte`

### 3. ERP 模組文件
- 各模組功能說明 (FI, CO, MM, SD, PP, HR, WM, QM)
- 模組間整合流程
- 單據流定義
- 狀態管理圖

### 4. 架構文件
- 系統架構圖
- 技術棧說明 (Rust/Axum + Next.js + iOS Native + Android Native + PostgreSQL 17)
- 目錄結構
- 部署流程

---

## 生成來源

```bash
# API endpoints (from Rust routes)
grep -r "route\|Router\|get\|post\|put\|delete" backend/src/routes/

# DB Schema (from migrations)
cat backend/migrations/*.sql

# DB 即時 Schema
PGPASSWORD=postgres psql -h localhost -p 5432 -U postgres -d TastyByte -c "\dt"

# TypeScript types
find web/src/types -name "*.ts"

# iOS Swift models
find ios -name "*.swift" -path "*/Models/*" 2>/dev/null

# Android Kotlin models
find android -name "*.kt" -path "*/model/*" 2>/dev/null
```

---

**執行參數:** $ARGUMENTS
