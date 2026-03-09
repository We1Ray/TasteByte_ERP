# /code-review - 整合式程式碼審查

使用專業 Agents 並行執行全面程式碼審查。

## Usage
```
/code-review [all|backend|web|ios|android|<file_path>]
```

---

## 審查範圍

### Rust Backend (backend-developer)
- 程式碼風格與 clippy 規範
- 錯誤處理完整性
- SQL 查詢安全性與效能
- ERP 模組邊界是否清晰
- API 回應格式一致性

### Next.js Web (frontend-developer)
- TypeScript 型別安全
- Server/Client Component 使用正確
- 狀態管理模式（Zustand + Tanstack Query）
- Tailwind CSS 使用一致性
- 元件拆分合理性

### iOS (ios-developer)
- Swift 程式碼風格與命名慣例
- SwiftUI View 架構與性能
- 網路層封裝（URLSession / async-await）
- 資料模型與 Codable 一致性
- 離線支援邏輯

### Android (android-developer)
- Kotlin 程式碼風格與命名慣例
- Jetpack Compose UI 架構與性能
- ViewModel / Repository 模式
- Retrofit 網路層封裝
- 資料模型與 Serialization 一致性

### Security (security-engineer)
- JWT 認證流程
- RBAC 授權物件檢查
- SQL Injection 防護
- 敏感資料處理
- 稽核日誌覆蓋

---

## 審查檢查清單

```
通用:
- [ ] 命名規範一致
- [ ] 錯誤處理完整
- [ ] 無硬編碼敏感資訊
- [ ] 有適當的日誌記錄

Rust:
- [ ] cargo clippy 無警告
- [ ] 使用 Result<T, AppError> 統一錯誤
- [ ] SQLx 查詢使用參數化
- [ ] 金額使用 Decimal

Next.js:
- [ ] ESLint 無錯誤
- [ ] TypeScript strict mode 通過
- [ ] 正確使用 Server/Client Components
- [ ] Zod schema 驗證輸入

iOS (Swift):
- [ ] Xcode build 無 warning
- [ ] SwiftUI View 正確使用 @State/@Binding/@ObservedObject
- [ ] API Models 符合 Codable 協議
- [ ] 正確處理 async/await 錯誤

Android (Kotlin):
- [ ] Gradle lint 無 error
- [ ] Compose 正確使用 remember/State
- [ ] ViewModel 遵循 MVVM 模式
- [ ] Coroutine 正確處理例外
```

---

## 輸出格式

```
═══ Code Review Report ═══

Backend: <n> issues (<critical>/<high>/<medium>/<low>)
Web: <n> issues (<critical>/<high>/<medium>/<low>)
iOS: <n> issues (<critical>/<high>/<medium>/<low>)
Android: <n> issues (<critical>/<high>/<medium>/<low>)
Security: <n> issues (<critical>/<high>/<medium>/<low>)

Critical Issues:
├── <file>:<line> - <description>
└── ...

═══════════════════════════
```

---

**執行參數:** $ARGUMENTS
