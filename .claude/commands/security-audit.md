# /security-audit - 安全審計

使用 security-engineer Agent 執行全面安全審計。

## Usage
```
/security-audit [all|auth|rbac|api|deps|db]
```

---

## 審計範圍

### 1. 認證 (Authentication)
- JWT token 配置（有效期、算法）
- 密碼雜湊（argon2）
- 帳號鎖定機制
- Token refresh 流程
- 登出 token 失效

### 2. 授權 (Authorization / RBAC)
- SAP-like 授權物件定義完整性
- 角色定義覆蓋所有 API
- RBAC middleware 正確性
- 最小權限原則

### 3. API 安全
- Rate Limiting 配置
- CORS 白名單
- 請求大小限制
- 輸入驗證（Zod + validator）

### 4. 資料安全
- SQL Injection 防護（參數化查詢）
- XSS 防護
- 敏感資料加密
- 稽核日誌覆蓋

### 5. 依賴安全
```bash
cd backend && cargo audit
cd web && pnpm audit
# iOS: 檢查 Swift Package 或 CocoaPods 依賴是否有已知漏洞
# Android: 檢查 Gradle 依賴是否有已知漏洞
cd android && ./gradlew dependencyCheckAnalyze 2>/dev/null || echo "OWASP dependency check not configured"
```

### 6. 資料庫安全
- 連線加密
- 使用者權限最小化
- 備份加密
- DB 連線: `postgres://postgres:postgres@localhost:5432/TastyByte`

---

## 輸出格式

```
═══ Security Audit Report ═══

Authentication:
├── JWT Config: [PASS/FAIL]
├── Password Hashing: [PASS/FAIL]
├── Account Lockout: [PASS/FAIL]
└── Token Management: [PASS/FAIL]

Authorization (RBAC):
├── Auth Objects: [PASS/FAIL]
├── Role Coverage: [PASS/FAIL]
└── Middleware: [PASS/FAIL]

API Security:
├── Rate Limiting: [PASS/FAIL]
├── CORS: [PASS/FAIL]
├── Input Validation: [PASS/FAIL]
└── SQL Injection: [PASS/FAIL]

Dependencies:
├── cargo audit: [PASS/FAIL] (<n> vulnerabilities)
├── pnpm audit: [PASS/FAIL] (<n> vulnerabilities)
├── iOS deps: [PASS/FAIL]
└── Android deps: [PASS/FAIL]

Overall Risk: [LOW/MEDIUM/HIGH/CRITICAL]

═══════════════════════════════
```

---

**執行參數:** $ARGUMENTS
