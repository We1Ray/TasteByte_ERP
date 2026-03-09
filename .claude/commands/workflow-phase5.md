# /workflow-phase5 - Phase 5: 問題修復 Team

**Lead: backend-developer**

根據 Phase 3-4 發現的問題進行修復。

---

## Team 組成

| 角色 | Agent | 職責 |
|------|-------|------|
| **Lead** | backend-developer | 修復協調、Rust 修復 |
| **Member** | frontend-developer | Next.js 修復 |
| **Member** | data-processor | DB Schema 修復 |

---

## 執行任務

你是 **backend-developer**，作為 Phase 5 Team Lead，執行以下任務：

### Step 1: Team Discussion

```
┌─────────────────────────────────────────────────────────────────┐
│ 【Phase 5: 問題修復 Team Discussion】                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ [Lead: backend-developer] 召集討論                               │
│ ├── 議題: Phase 3-4 問題分類與修復分工                           │
│ ├── 參與: frontend-developer, data-processor                    │
│ └── 目標: 並行修復 → 驗證修復結果                               │
│                                                                 │
│ 問題分類:                                                       │
│ ├── Critical: 必須立即修復                                      │
│ ├── High: 本輪修復                                              │
│ ├── Medium: 可延後                                              │
│ └── Low: 記錄追蹤                                               │
│                                                                 │
│ 修復分工:                                                       │
│ ├── Rust 問題 → backend-developer                               │
│ ├── Next.js 問題 → frontend-developer                           │
│ ├── iOS 問題 → ios-developer (如需要可召集)                     │
│ ├── Android 問題 → android-developer (如需要可召集)             │
│ └── DB 問題 → data-processor                                    │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Step 2: 並行修復

各 Agent 依分工修復各自負責的問題。

### Step 3: 修復驗證

```bash
# 修復後重新檢查
cd backend && cargo clippy -- -D warnings && cargo test
cd web && pnpm run lint && pnpm test

# iOS 修復驗證 (如有修改)
cd ios && xcodebuild -scheme TasteByte -destination 'platform=iOS Simulator,name=iPhone 16' build

# Android 修復驗證 (如有修改)
cd android && ./gradlew assembleDebug && ./gradlew test
```

### Step 4: 輸出修復報告

```
═══ Phase 5: 修復報告 ═══

修復結果:
| 問題 | 嚴重程度 | 負責人 | 狀態 |
|------|---------|--------|------|
| <issue> | Critical | <agent> | [FIXED/PENDING] |
| ... | ... | ... | ... |

修復後測試:
├── cargo test: [PASS/FAIL]
├── pnpm test: [PASS/FAIL]
├── iOS build: [PASS/FAIL]
└── Android build: [PASS/FAIL]

═══════════════════════════════
```

---

**執行參數:** $ARGUMENTS
