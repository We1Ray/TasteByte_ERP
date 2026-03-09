# /workflow-phase2 - Phase 2: 文件同步 Team

**Lead: system-architect**

執行跨層文件版本同步與一致性驗證。

---

## Team 組成

| 角色 | Agent | 職責 |
|------|-------|------|
| **Lead** | system-architect | 同步策略、一致性審查 |
| **Member** | data-processor | DB Schema 與 Migration 同步 |
| **Member** | frontend-developer | TypeScript 型別同步 |

---

## 執行任務

你是 **system-architect**，作為 Phase 2 Team Lead，執行以下任務：

### Step 1: Team Discussion

```
┌─────────────────────────────────────────────────────────────────┐
│ 【Phase 2: 文件同步 Team Discussion】                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ [Lead: system-architect] 召集討論                                │
│ ├── 議題: 跨層資料模型一致性                                     │
│ ├── 參與: data-processor, frontend-developer                    │
│ └── 目標: 確保 DB ↔ Rust ↔ TypeScript ↔ Swift ↔ Kotlin 同步    │
│                                                                 │
│ [data-processor] DB Schema 狀態                                  │
│ ├── 檢查最新 migration 版本                                     │
│ ├── 確認欄位定義                                                │
│ └── 回報不一致項目                                              │
│                                                                 │
│ [frontend-developer] TypeScript 型別狀態                         │
│ ├── 檢查 web/src/types/ 定義                                    │
│ ├── 對比 API response 型別                                      │
│ └── 回報不一致項目                                              │
│                                                                 │
│ [Lead] 總結                                                      │
│ ├── 彙整所有不一致項目                                          │
│ ├── 同步順序: DB → Rust → TypeScript → Swift → Kotlin           │
│ └── 分派同步任務                                                │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Step 2: Schema 對比

```bash
# DB Migration 檔案
ls -la backend/migrations/*.sql

# Rust Models
find backend/src -name "*.rs" | head -50

# TypeScript Types
find web/src/types -name "*.ts" 2>/dev/null

# iOS Swift Models
find ios -name "*.swift" -path "*/Models/*" 2>/dev/null

# Android Kotlin Models
find android -name "*.kt" -path "*/model/*" 2>/dev/null
```

### Step 3: 同步執行

1. **DB Schema** (source of truth) → 確認 migration 正確
2. **Rust Models** → 對齊 DB 欄位
3. **TypeScript Types** → 對齊 API response
4. **Swift Models** → 對齊 API response
5. **Kotlin Models** → 對齊 API response

### Step 4: 一致性驗證

```bash
# Rust 編譯驗證
cd backend && cargo build

# TypeScript 型別驗證
cd web && pnpm run type-check

# iOS 編譯驗證
cd ios && xcodebuild -scheme TasteByte -destination 'platform=iOS Simulator,name=iPhone 16' build

# Android 編譯驗證
cd android && ./gradlew assembleDebug
```

---

**執行參數:** $ARGUMENTS
