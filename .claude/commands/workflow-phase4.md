# /workflow-phase4 - Phase 4: 測試執行 Team

**Lead: qa-engineer**

執行全面測試套件並驗證 Quality Gates。

---

## Team 組成

| 角色 | Agent | 職責 |
|------|-------|------|
| **Lead** | qa-engineer | 測試協調、Quality Gates 驗證 |
| **Member** | backend-developer | Rust 測試執行 |
| **Member** | frontend-developer | Next.js 測試執行 |

---

## 執行任務

你是 **qa-engineer**，作為 Phase 4 Team Lead，執行以下任務：

### Step 1: Team Discussion

```
┌─────────────────────────────────────────────────────────────────┐
│ 【Phase 4: 測試執行 Team Discussion】                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ [Lead: qa-engineer] 召集討論                                     │
│ ├── 議題: 測試範圍與策略                                        │
│ ├── 參與: backend-developer, frontend-developer                 │
│ └── 目標: 並行測試 → 結果彙整 → Quality Gates 驗證             │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Step 2: 並行測試執行

```bash
# Rust Backend 測試
cd backend
cargo test 2>&1
cargo test -- --test-threads=1 2>&1  # DB 相關測試

# Next.js Web 測試
cd web
pnpm test 2>&1
pnpm run test:coverage 2>&1

# iOS 測試 (if test targets exist)
cd ios
xcodebuild test -scheme TasteByte -destination 'platform=iOS Simulator,name=iPhone 16' 2>&1

# Android 測試 (if test targets exist)
cd android
./gradlew test 2>&1

# 安全測試
cd backend && cargo audit 2>&1
cd web && pnpm audit 2>&1
```

### Step 3: Quality Gates 驗證

| Gate | 指標 | 目標 | 狀態 |
|------|------|------|------|
| Gate 1 | 程式碼品質 | Lint 全通過 | [PASS/FAIL] |
| Gate 2 | 單元測試 | 覆蓋率 > 85% | [PASS/FAIL] |
| Gate 3 | 整合測試 | 全通過 | [PASS/FAIL] |
| Gate 4 | 安全掃描 | 無高危 | [PASS/FAIL] |
| Gate 5 | 效能達標 | P95 < 200ms | [PASS/FAIL] |

### Step 4: 輸出測試報告

```
═══ Phase 4: 測試報告 ═══

測試結果:
| 服務 | 通過 | 失敗 | 跳過 | 覆蓋率 |
|------|------|------|------|--------|
| Rust Backend | <n> | <n> | <n> | <x>% |
| Next.js Web | <n> | <n> | <n> | <x>% |
| iOS (Swift) | <n> | <n> | <n> | N/A |
| Android (Kotlin) | <n> | <n> | <n> | N/A |

Quality Gates:
├── Gate 1: 程式碼品質    [PASS/FAIL]
├── Gate 2: 單元測試      [PASS/FAIL] 覆蓋率: <x>%
├── Gate 3: 整合測試      [PASS/FAIL]
├── Gate 4: 安全掃描      [PASS/FAIL]
└── Gate 5: 效能達標      [PASS/FAIL]

失敗測試列表:
├── <test_name>: <error>
└── ...

═══════════════════════════════
```

---

**執行參數:** $ARGUMENTS
