# /workflow-phase6 - Phase 6: 最終驗證 Team

**Lead: qa-engineer**

執行最終驗證並生成驗收報告。

---

## Team 組成

| 角色 | Agent | 職責 |
|------|-------|------|
| **Lead** | qa-engineer | 驗收測試、Quality Gates 確認 |
| **Member** | erp-domain-expert | ERP 商業邏輯最終確認 |

---

## 執行任務

你是 **qa-engineer**，作為 Phase 6 Team Lead，執行以下任務：

### Step 1: Team Discussion

```
┌─────────────────────────────────────────────────────────────────┐
│ 【Phase 6: 最終驗證 Team Discussion】                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ [Lead: qa-engineer] 召集討論                                     │
│ ├── 議題: 確認所有 Quality Gates 通過                             │
│ ├── 參與: erp-domain-expert                                     │
│ └── 目標: 生成驗收報告                                           │
│                                                                 │
│ [erp-domain-expert] ERP 品質確認                                 │
│ ├── 模組間整合邏輯正確性                                        │
│ ├── 單據流完整性                                                │
│ ├── 狀態管理正確性                                              │
│ └── 主資料/交易資料分離                                         │
│                                                                 │
│ [Lead: qa-engineer] 總結                                         │
│ ├── 重新執行所有測試                                              │
│ ├── 驗證 Quality Gates                                           │
│ └── 生成最終報告                                                  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Step 2: 重新執行測試

```bash
# Rust
cd backend && cargo test

# Next.js
cd web && pnpm test

# iOS
cd ios && xcodebuild test -scheme TasteByte -destination 'platform=iOS Simulator,name=iPhone 16' 2>&1

# Android
cd android && ./gradlew test 2>&1
```

### Step 3: Quality Gates 最終驗證

| Gate | 指標 | 目標 | 狀態 |
|------|------|------|------|
| Gate 1 | 程式碼品質 | Lint 全通過 | [PASS/FAIL] |
| Gate 2 | 單元測試 | 覆蓋率 > 85% | [PASS/FAIL] |
| Gate 3 | 整合測試 | 全通過 | [PASS/FAIL] |
| Gate 4 | 安全掃描 | 無高危 | [PASS/FAIL] |
| Gate 5 | 效能達標 | P95 < 200ms | [PASS/FAIL] |

### Step 4: 驗收結論判定

| 結論 | 條件 |
|------|------|
| PASS | 所有 Quality Gates 通過 |
| CONDITIONAL | 僅 Low 優先級問題未解決 |
| FAIL | 有 Critical/High 問題未解決 |

### Step 5: 輸出最終報告

```
═══════════════════════════════════════════════════════════════
                TasteByte ERP Workflow 完成報告
═══════════════════════════════════════════════════════════════

執行摘要:
├── 執行 Phases: 1-6
└── 總修復問題: <count>

測試結果:
| 服務 | 通過 | 失敗 | 通過率 |
|------|------|------|--------|
| Rust Backend | <n> | <n> | <x>% |
| Next.js Web | <n> | <n> | <x>% |
| iOS (Swift) | <n> | <n> | <x>% |
| Android (Kotlin) | <n> | <n> | <x>% |
| Security | <n> | <n> | <x>% |

Quality Gates:
├── Gate 1: 程式碼品質    [PASS/FAIL]
├── Gate 2: 單元測試      [PASS/FAIL] 覆蓋率: <x>%
├── Gate 3: 整合測試      [PASS/FAIL]
├── Gate 4: 安全掃描      [PASS/FAIL]
└── Gate 5: 效能達標      [PASS/FAIL]

驗收結論: [PASS / CONDITIONAL / FAIL]

建議:
- <recommendation_1>
- <recommendation_2>

═══════════════════════════════════════════════════════════════
```

---

**執行參數:** $ARGUMENTS
