# /workflow - TasteByte ERP 專案自動更新與測試

**執行完整的開發流程，每階段透過 Agent Team 協作完成後提供進度報告。**

---

## Agent Team 協作架構

> **CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS** 已啟用，所有 Phase 將透過 Agent Team 協作執行。

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         Workflow Agent Team 總覽                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ Phase 0: DB Schema 跨層驗證 (每次開發前必執行)                        │   │
│  │   Lead: data-processor                                              │   │
│  │   Members: backend-developer                                        │   │
│  │   職責: DB Migration ↔ Rust Models ↔ TypeScript Types               │   │
│  │         ↔ Swift Models ↔ Kotlin Models                              │   │
│  │   Gate: 必須通過才能進入後續 Phase                                    │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ Phase 1: 專案狀態 Team                                               │   │
│  │   Lead: system-architect                                             │   │
│  │   Members: erp-domain-expert                                         │   │
│  │   職責: 架構審視、影響分析、任務分派規劃                               │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ Phase 2: 文件同步 Team                                               │   │
│  │   Lead: system-architect                                             │   │
│  │   Members: data-processor, frontend-developer                        │   │
│  │   討論: 文件版本確認 → 分工同步 → 一致性驗證                          │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ Phase 3: 程式碼檢查 Team                                             │   │
│  │   Lead: qa-engineer                                                  │   │
│  │   Members: backend-developer, frontend-developer, ios-developer,     │   │
│  │            android-developer, security-engineer                      │   │
│  │   討論: 檢查範圍 → 並行執行 → 問題彙整                                │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ Phase 4: 測試執行 Team                                               │   │
│  │   Lead: qa-engineer                                                  │   │
│  │   Members: backend-developer, frontend-developer                     │   │
│  │   討論: 測試範圍 → 並行測試 → 結果彙整                                │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ Phase 5: 問題修復 Team                                               │   │
│  │   Lead: backend-developer                                            │   │
│  │   Members: frontend-developer, data-processor                        │   │
│  │   討論: 問題分類 → 分工修復 → 修復驗證                                │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ Phase 6: 最終驗證 Team                                               │   │
│  │   Lead: qa-engineer                                                  │   │
│  │   Members: erp-domain-expert                                         │   │
│  │   職責: 驗收測試、Quality Gates 確認                                  │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │ Phase 7: 服務部署 Team                                               │   │
│  │   Lead: system-admin                                                 │   │
│  │   Members: backend-developer                                         │   │
│  │   職責: 資料庫遷移 → 後端部署 → 前端部署 → 健康檢查                   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 執行流程

### Phase 0: DB Schema 跨層驗證 (Gate)

**每次開發前必須執行，確保資料一致性。**

```bash
# 1. 確認 PostgreSQL 連線
pg_isready -h localhost -p 5432

# 2. 檢查 Migration 檔案
ls backend/migrations/*.sql

# 3. 驗證 DB Schema
PGPASSWORD=postgres psql -h localhost -p 5432 -U postgres -d TastyByte -c "\dt"

# 4. 驗證 Rust Models 與 DB Schema 一致
cd backend && cargo build

# 5. 驗證 TypeScript Types 與 API 一致
cd web && pnpm run type-check

# 6. 驗證 iOS 專案可編譯
cd ios && xcodebuild -scheme TasteByte -destination 'platform=iOS Simulator,name=iPhone 16' build

# 7. 驗證 Android 專案可編譯
cd android && ./gradlew assembleDebug
```

驗證項目:
- [ ] Migration SQL ↔ Rust SQLx models 欄位一致
- [ ] Rust API response ↔ TypeScript types 一致
- [ ] Rust API response ↔ Swift models 一致
- [ ] Rust API response ↔ Kotlin models 一致
- [ ] 金額欄位全部使用 DECIMAL / Decimal / number

### Phase 1-7: 依照各 Phase 指令執行

使用 `/workflow-phase1` 到 `/workflow-phase7`（跳過 5.5）個別執行各階段。

---

## 技術棧摘要

| Layer | 技術 | Port | 測試命令 |
|-------|------|------|---------|
| Database | PostgreSQL 17 | 5432 | `PGPASSWORD=postgres psql -h localhost -p 5432 -U postgres -d TastyByte` |
| Backend | Rust/Axum 0.8 | 8000 | `cargo test` |
| Web | Next.js 15 | 3000 | `pnpm test` |
| iOS | Swift/SwiftUI (Native) | N/A | `xcodebuild test` |
| Android | Kotlin/Jetpack Compose (Native) | N/A | `./gradlew test` |

---

## Quality Gates

| Gate | 指標 | 目標 |
|------|------|------|
| Gate 1 | 程式碼品質 | clippy + eslint + Swift build + Kotlin build 全通過 |
| Gate 2 | 單元測試 | 覆蓋率 > 85% |
| Gate 3 | 整合測試 | 跨模組流程全通過 |
| Gate 4 | 安全掃描 | cargo audit + pnpm audit 無高危 |
| Gate 5 | 效能達標 | P95 < 200ms |

---

**執行參數:** $ARGUMENTS
