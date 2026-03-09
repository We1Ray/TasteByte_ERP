# /test - TasteByte ERP 全功能測試與自動修復指令

## 概述
執行 TasteByte ERP 的完整測試流程，並在發現問題時由 Agent 分析並決定修復策略。

## Usage
```
/test [all|backend|web|ios|android|security]
```

---

## 測試流程

### Phase 1: 編譯檢查

```bash
# Rust 編譯
cd backend && cargo build 2>&1

# TypeScript 型別檢查
cd web && pnpm run type-check 2>&1

# iOS 編譯檢查
cd ios && xcodebuild -scheme TasteByte -destination 'platform=iOS Simulator,name=iPhone 16' build 2>&1

# Android 編譯檢查
cd android && ./gradlew assembleDebug 2>&1
```

### Phase 2: 單元測試

```bash
# Rust
cd backend && cargo test 2>&1

# Next.js (vitest)
cd web && pnpm test 2>&1

# iOS (XCTest)
cd ios && xcodebuild test -scheme TasteByte -destination 'platform=iOS Simulator,name=iPhone 16' 2>&1

# Android (JUnit/Compose)
cd android && ./gradlew test 2>&1
```

### Phase 3: Lint 檢查

```bash
# Rust
cd backend && cargo clippy -- -D warnings 2>&1
cd backend && cargo fmt --check 2>&1

# Next.js
cd web && pnpm run lint 2>&1

# iOS (SwiftLint, if configured)
cd ios && swiftlint lint 2>/dev/null || echo "SwiftLint not configured"

# Android (Kotlin lint)
cd android && ./gradlew lint 2>&1
```

### Phase 4: 安全掃描

```bash
cd backend && cargo audit 2>&1
cd web && pnpm audit 2>&1
```

---

## 自動修復策略

當測試失敗時，Agent 依以下策略修復：

### 編譯錯誤 (Critical)
- 讀取錯誤訊息
- 定位問題檔案
- 修復型別錯誤、語法錯誤
- 重新編譯驗證

### 測試失敗 (High)
- 分析失敗測試的斷言
- 檢查相關業務邏輯
- 修復程式碼或更新測試
- 重新執行驗證

### Lint 警告 (Medium)
- 依 clippy/eslint/swiftlint 建議修復
- 格式化程式碼
- 重新檢查

### 安全漏洞 (High)
- 更新有漏洞的依賴
- 如需 breaking change 先警告使用者

---

## 輸出格式

```
═══ Test Results ═══

Backend (Rust):
├── Build: [PASS/FAIL]
├── Tests: <passed>/<total> [PASS/FAIL]
├── Clippy: [PASS/FAIL]
└── Audit: [PASS/FAIL]

Web (Next.js):
├── Type Check: [PASS/FAIL]
├── Tests: <passed>/<total> [PASS/FAIL]
├── ESLint: [PASS/FAIL]
└── Audit: [PASS/FAIL]

iOS (Swift/SwiftUI):
├── Build: [PASS/FAIL]
└── Tests: <passed>/<total> [PASS/FAIL]

Android (Kotlin/Compose):
├── Build: [PASS/FAIL]
├── Tests: <passed>/<total> [PASS/FAIL]
└── Lint: [PASS/FAIL]

Auto-fixes applied: <count>
═══════════════════
```

---

**執行參數:** $ARGUMENTS
