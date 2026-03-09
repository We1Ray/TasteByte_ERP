# /workflow-phase3 - Phase 3: 程式碼檢查 Team

**Lead: qa-engineer**

執行全面程式碼品質檢查。

---

## Team 組成

| 角色 | Agent | 職責 |
|------|-------|------|
| **Lead** | qa-engineer | 檢查協調、結果彙整 |
| **Member** | backend-developer | Rust 程式碼檢查 |
| **Member** | frontend-developer | Next.js 程式碼檢查 |
| **Member** | ios-developer | iOS (Swift) 程式碼檢查 |
| **Member** | android-developer | Android (Kotlin) 程式碼檢查 |
| **Member** | security-engineer | 安全性檢查 |

---

## 執行任務

你是 **qa-engineer**，作為 Phase 3 Team Lead，執行以下任務：

### Step 1: Team Discussion

```
┌─────────────────────────────────────────────────────────────────┐
│ 【Phase 3: 程式碼檢查 Team Discussion】                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ [Lead: qa-engineer] 召集討論                                     │
│ ├── 議題: 確認檢查範圍與標準                                     │
│ ├── 參與: backend-developer, frontend-developer,                │
│ │         ios-developer, android-developer, security-engineer   │
│ └── 目標: 並行執行各層程式碼檢查                                 │
│                                                                 │
│ [backend-developer] Rust 檢查                                    │
│ ├── cargo clippy -- -D warnings                                 │
│ ├── cargo fmt --check                                           │
│ └── 商業邏輯複雜度檢查                                          │
│                                                                 │
│ [frontend-developer] Next.js 檢查                                │
│ ├── pnpm run lint (ESLint)                                      │
│ ├── pnpm run type-check (TypeScript)                            │
│ └── 元件結構檢查                                                │
│                                                                 │
│ [ios-developer] iOS 檢查                                         │
│ ├── xcodebuild build (編譯檢查)                                 │
│ ├── SwiftLint (如已配置)                                        │
│ └── SwiftUI 架構檢查                                            │
│                                                                 │
│ [android-developer] Android 檢查                                 │
│ ├── ./gradlew lint                                              │
│ ├── ./gradlew assembleDebug (編譯檢查)                          │
│ └── Compose 架構檢查                                            │
│                                                                 │
│ [security-engineer] 安全檢查                                     │
│ ├── cargo audit                                                 │
│ ├── pnpm audit                                                  │
│ ├── SQL injection 風險掃描                                      │
│ └── 認證/授權邏輯審查                                           │
│                                                                 │
│ [Lead: qa-engineer] 彙整結果                                     │
│ ├── 按嚴重程度分類 (Critical/High/Medium/Low)                   │
│ └── 產出檢查報告                                                │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Step 2: 並行檢查執行

```bash
# Rust 檢查
cd backend
cargo clippy -- -D warnings
cargo fmt --check

# Next.js 檢查
cd web
pnpm run lint
pnpm run type-check

# iOS 檢查
cd ios
xcodebuild -scheme TasteByte -destination 'platform=iOS Simulator,name=iPhone 16' build

# Android 檢查
cd android
./gradlew lint
./gradlew assembleDebug

# 安全掃描
cd backend && cargo audit
cd web && pnpm audit
```

### Step 3: 輸出檢查報告

```
═══ Phase 3: 程式碼檢查報告 ═══

Rust Backend:
├── clippy: [PASS/FAIL] (<n> warnings)
├── fmt: [PASS/FAIL]
└── 問題: <list>

Next.js Web:
├── ESLint: [PASS/FAIL] (<n> errors, <n> warnings)
├── TypeScript: [PASS/FAIL]
└── 問題: <list>

iOS (Swift/SwiftUI):
├── Build: [PASS/FAIL] (<n> warnings)
└── 問題: <list>

Android (Kotlin/Compose):
├── Lint: [PASS/FAIL] (<n> issues)
├── Build: [PASS/FAIL]
└── 問題: <list>

Security:
├── cargo audit: [PASS/FAIL]
├── pnpm audit: [PASS/FAIL]
└── 問題: <list>

═══════════════════════════════
```

---

**執行參數:** $ARGUMENTS
