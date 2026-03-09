# /workflow-phase1 - Phase 1: 專案狀態 Team

**Lead: system-architect**

執行專案狀態檢視與架構審視。

---

## Team 組成

| 角色 | Agent | 職責 |
|------|-------|------|
| **Lead** | system-architect | 架構審視、影響分析、任務規劃 |
| **Member** | erp-domain-expert | ERP 模組邏輯驗證 |

---

## 執行任務

你是 **system-architect**，作為 Phase 1 Team Lead，執行以下任務：

### Step 1: 架構審視

```bash
# 檢查專案結構
ls -la backend/src/
ls -la web/src/app/
ls -la ios/
ls -la android/

# 檢查 Rust 依賴
cat backend/Cargo.toml

# 檢查 Next.js 依賴
cat web/package.json

# 檢查 iOS 專案
ls -la ios/*.xcodeproj 2>/dev/null || ls -la ios/*.xcworkspace 2>/dev/null

# 檢查 Android 專案
cat android/app/build.gradle.kts 2>/dev/null || cat android/app/build.gradle 2>/dev/null
```

### Step 2: 資料庫狀態

```bash
# 確認 PostgreSQL 連線
pg_isready -h localhost -p 5432

# 檢查 migration 檔案
ls backend/migrations/*.sql

# 驗證 DB 可存取
PGPASSWORD=postgres psql -h localhost -p 5432 -U postgres -d TastyByte -c "SELECT version();"
```

### Step 3: 變更影響分析

檢查最近的程式碼變更：

```bash
git log --oneline -20
git diff --stat HEAD~5
```

分析影響範圍：
- 哪些 ERP 模組受影響
- 是否需要新的 migration
- 前端是否需要同步更新

### Step 4: 任務分派規劃

根據分析結果，規劃各 Agent 的任務：

| Agent | 任務 |
|-------|------|
| backend-developer | Rust API 開發/修復 |
| frontend-developer | Next.js 頁面開發/修復 |
| ios-developer | iOS (Swift/SwiftUI) 行動端開發/修復 |
| android-developer | Android (Kotlin/Compose) 行動端開發/修復 |
| data-processor | DB Schema 與 Migration |
| erp-domain-expert | 商業邏輯驗證 |

### Step 5: 輸出狀態報告

```
═══ Phase 1: 專案狀態報告 ═══

架構狀態:
├── Backend (Rust/Axum 0.8): [OK/WARN/ERROR]
├── Web (Next.js 15): [OK/WARN/ERROR]
├── iOS (Swift/SwiftUI): [OK/WARN/ERROR]
├── Android (Kotlin/Compose): [OK/WARN/ERROR]
└── Database (PostgreSQL 17, port 5432): [OK/WARN/ERROR]

變更影響:
├── 影響模組: [FI/CO/MM/SD/PP/HR/WM/QM]
├── 需要 Migration: [Yes/No]
└── 前端同步: [Yes/No]

任務分派:
├── <agent>: <task>
└── ...

═══════════════════════════════
```

---

**執行參數:** $ARGUMENTS
