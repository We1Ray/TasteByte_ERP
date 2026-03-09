# /run-tests - 執行測試套件

執行 TasteByte ERP 完整的測試套件。

## Usage
```
/run-tests [all|backend|web|ios|android]
```

---

## 測試命令

### Rust Backend
```bash
cd backend
cargo test                             # 全部測試
cargo test -- --test-threads=1         # 序列 (DB tests)
cargo clippy -- -D warnings            # Lint
cargo fmt --check                      # 格式
```

### Next.js Web
```bash
cd web
pnpm test                              # vitest
pnpm run test:coverage                 # 覆蓋率
pnpm run lint                          # ESLint
pnpm run type-check                    # TypeScript
```

### iOS (Swift/SwiftUI)
```bash
cd ios
# 編譯檢查
xcodebuild -scheme TasteByte -destination 'platform=iOS Simulator,name=iPhone 16' build

# 單元測試 (XCTest)
xcodebuild test -scheme TasteByte -destination 'platform=iOS Simulator,name=iPhone 16'

# SwiftLint (if configured)
swiftlint lint 2>/dev/null || echo "SwiftLint not configured"
```

### Android (Kotlin/Jetpack Compose)
```bash
cd android
./gradlew test                         # 單元測試 (JUnit)
./gradlew connectedAndroidTest         # 裝置測試
./gradlew lint                         # Lint 檢查
./gradlew assembleDebug                # 編譯檢查
```

### 安全掃描
```bash
cd backend && cargo audit
cd web && pnpm audit
```

---

## Quality Gates

| Gate | 指標 | 目標 |
|------|------|------|
| Lint | clippy + eslint + swiftlint + ktlint | 0 errors |
| Tests | 全部通過 | 100% pass |
| Coverage | 覆蓋率 | > 85% |
| Security | audit | 0 high/critical |

---

**執行參數:** $ARGUMENTS
