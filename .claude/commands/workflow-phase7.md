# /workflow-phase7 - Phase 7: 服務部署 Team

**Lead: system-admin**

編譯、啟動並驗證所有服務。

---

## Team 組成

| 角色 | Agent | 職責 |
|------|-------|------|
| **Lead** | system-admin | 部署協調、服務管理 |
| **Member** | backend-developer | Rust 後端部署支援 |

---

## 執行任務

你是 **system-admin**，作為 Phase 7 Team Lead，執行以下任務：

### Step 1: 環境檢查

```bash
# 檢查 PostgreSQL
pg_isready -h localhost -p 5432

# 檢查 Rust toolchain
rustc --version
cargo --version

# 檢查 Node.js
node --version
pnpm --version

# 檢查 Xcode (iOS)
xcodebuild -version

# 檢查 Android SDK
./android/gradlew --version 2>/dev/null || echo "Check Android Studio installation"
```

### Step 2: 資料庫遷移

```bash
cd backend
cargo run -- migrate
```

### Step 3: 後端部署

```bash
cd backend
cargo build --release
cargo run --release &

# 健康檢查
sleep 3
curl http://localhost:8000/health
```

### Step 4: Web 前端部署

```bash
cd web
pnpm install
pnpm build
pnpm start &

# 健康檢查
sleep 3
curl http://localhost:3000
```

### Step 5: 全服務健康檢查

```bash
echo "=== PostgreSQL ===" && pg_isready -h localhost -p 5432
echo "=== Backend (8000) ===" && curl -s http://localhost:8000/health
echo "=== Web (3000) ===" && curl -s -o /dev/null -w "%{http_code}" http://localhost:3000
```

### Step 6: 輸出部署報告

```
═══ Phase 7: 部署報告 ═══

服務狀態:
├── PostgreSQL 17 (5432): [UP/DOWN]
├── Rust Backend (8000): [UP/DOWN]
├── Next.js Web (3000): [UP/DOWN]
├── iOS App: [N/A - 透過 Xcode/TestFlight 部署]
└── Android App: [N/A - 透過 Android Studio/Play Console 部署]

資料庫遷移:
├── 版本: <latest_migration>
└── 狀態: [OK/ERROR]

部署結論: [SUCCESS/PARTIAL/FAILED]

═══════════════════════════════
```

---

**執行參數:** $ARGUMENTS
