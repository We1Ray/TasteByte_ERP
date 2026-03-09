# /deploy - 部署服務

部署 TasteByte ERP 服務到本地開發環境。

## Usage
```
/deploy [all|backend|web|db]
```

---

## 部署流程

### 1. 資料庫

```bash
# 確認 PostgreSQL 運行
pg_isready -h localhost -p 5432

# 驗證連線
PGPASSWORD=postgres psql -h localhost -p 5432 -U postgres -d TastyByte -c "SELECT version();"

# 執行 migration
cd backend && cargo run -- migrate
```

### 2. Rust Backend

```bash
cd backend

# Release 編譯
cargo build --release

# 啟動
RUST_LOG=info cargo run --release &

# 健康檢查
sleep 3
curl http://localhost:8000/health
```

### 3. Next.js Web

```bash
cd web

# 安裝依賴
pnpm install

# 生產建構
pnpm build

# 啟動
pnpm start &

# 健康檢查
sleep 3
curl -s -o /dev/null -w "%{http_code}" http://localhost:3000
```

### 4. 行動端部署

#### iOS
- 透過 Xcode Archive → TestFlight 發佈測試版
- 或透過 `xcodebuild archive` 命令列打包

#### Android
- 透過 Android Studio Build → Generate Signed APK/AAB
- 或透過 `cd android && ./gradlew assembleRelease` 命令列打包

### 5. 全服務驗證

```bash
echo "=== PostgreSQL (5432) ===" && pg_isready -h localhost -p 5432
echo "=== Backend (8000) ===" && curl -s http://localhost:8000/health
echo "=== Web (3000) ===" && curl -s -o /dev/null -w "%{http_code}" http://localhost:3000
```

---

**執行參數:** $ARGUMENTS
