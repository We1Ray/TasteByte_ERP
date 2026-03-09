# /dev-setup - 開發環境設置

設置 TasteByte ERP 完整的開發環境。

## Usage
```
/dev-setup [all|backend|web|ios|android|db]
```

---

## 前置需求

```bash
# macOS (Homebrew)
brew install postgresql@17
brew install node
brew install pnpm
brew install rustup
rustup default stable

# iOS: 需安裝 Xcode (App Store) 及 Command Line Tools
xcode-select --install

# Android: 需安裝 Android Studio
# 下載: https://developer.android.com/studio
```

---

## 完整設置流程

### 1. PostgreSQL 17

```bash
# 啟動 PostgreSQL
brew services start postgresql@17

# 確認運行
pg_isready -h localhost -p 5432

# 建立資料庫
PGPASSWORD=postgres createdb -h localhost -p 5432 -U postgres TastyByte

# 驗證
PGPASSWORD=postgres psql -h localhost -p 5432 -U postgres -d TastyByte -c "SELECT version();"
```

### 2. Rust Backend

```bash
cd backend

# 安裝依賴並編譯 (自動執行 migration)
cargo build

# 啟動服務
cargo run
# → http://localhost:8000
# → http://localhost:8000/health
```

### 3. Next.js Web

```bash
cd web

# 安裝依賴
pnpm install

# 設定環境變數
cp .env.example .env.local
# 確認 NEXT_PUBLIC_API_URL=http://localhost:8000/api/v1

# 啟動開發伺服器
pnpm dev
# → http://localhost:3000
```

### 4. iOS (Swift/SwiftUI)

```bash
# 開啟 Xcode 專案
open ios/TasteByte.xcodeproj
# 或 open ios/TasteByte.xcworkspace (如使用 CocoaPods)

# 命令列編譯
cd ios
xcodebuild -scheme TasteByte -destination 'platform=iOS Simulator,name=iPhone 16' build

# 在模擬器上執行
xcodebuild -scheme TasteByte -destination 'platform=iOS Simulator,name=iPhone 16' build
```

### 5. Android (Kotlin/Jetpack Compose)

```bash
# 開啟 Android Studio 專案
open -a "Android Studio" android/

# 命令列編譯
cd android
./gradlew assembleDebug

# 執行測試
./gradlew test
```

---

## 環境變數

### Backend (.env)
```
DATABASE_URL=postgres://postgres:postgres@localhost:5432/TastyByte
JWT_SECRET=your-secret-key
RUST_LOG=debug
```

### Web (.env.local)
```
NEXT_PUBLIC_API_URL=http://localhost:8000/api/v1
```

---

## 驗證

```bash
# 全服務檢查
echo "=== PostgreSQL ===" && pg_isready -h localhost -p 5432
echo "=== Backend ===" && curl -s http://localhost:8000/health
echo "=== Web ===" && curl -s -o /dev/null -w "%{http_code}" http://localhost:3000
```

---

**執行參數:** $ARGUMENTS
