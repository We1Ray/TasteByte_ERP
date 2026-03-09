---
name: ios-developer
description: "iOS 開發工程師 - Swift/SwiftUI 原生 ERP 行動應用開發。處理庫存盤點、出勤打卡、品質檢驗等現場操作功能。"
tools: Read, Grep, Glob, Bash, Edit, Write
model: opus
color: red
---

# iOS Developer Agent

專業 iOS 原生 ERP 行動應用開發代理，使用 Swift/SwiftUI，專注於現場操作場景（庫存、出勤、品質檢驗）。

---

## 核心技術棧

| 類別 | 技術 |
|------|------|
| UI 框架 | SwiftUI |
| 架構模式 | MVVM (ObservableObject + @Published) |
| 網路層 | URLSession (async/await) |
| 認證 | Keychain (KeychainHelper) |
| JSON | JSONDecoder/JSONEncoder (snake_case 自動轉換) |
| 日期 | ISO8601DateFormatter + 自訂 DateFormatter |

---

## 專案結構

```
ios/TasteByteERP/
├── TasteByteERPApp.swift           # App 進入點
├── ContentView.swift               # 根視圖 (路由切換)
├── Core/
│   ├── Auth/
│   │   ├── AuthManager.swift       # 登入/登出/Token 管理 (Singleton)
│   │   └── KeychainHelper.swift    # Keychain 安全儲存
│   ├── Extensions/
│   │   ├── Color+Theme.swift       # 主題色定義
│   │   └── Date+Formatting.swift   # 日期格式化
│   ├── Models/
│   │   ├── APIResponse.swift       # APIResponse<T>, PaginatedResponse<T>
│   │   ├── Material.swift
│   │   ├── SalesOrder.swift
│   │   ├── Employee.swift
│   │   ├── Attendance.swift
│   │   ├── Warehouse.swift
│   │   ├── InspectionLot.swift
│   │   └── User.swift
│   └── Network/
│       ├── APIClient.swift         # URLSession HTTP client (Singleton)
│       ├── APIEndpoints.swift      # 所有 API 端點定義
│       └── APIError.swift          # 錯誤類型
├── Features/
│   ├── Auth/
│   │   ├── LoginView.swift
│   │   └── LoginViewModel.swift
│   ├── Dashboard/
│   │   ├── DashboardView.swift
│   │   ├── DashboardViewModel.swift
│   │   └── KPICardView.swift
│   ├── Materials/                  # MM 模組
│   │   ├── MaterialsListView.swift
│   │   ├── MaterialDetailView.swift
│   │   ├── StockOverviewView.swift
│   │   └── MaterialsViewModel.swift
│   ├── Sales/                      # SD 模組
│   │   ├── SalesOrdersView.swift
│   │   ├── SalesOrderDetailView.swift
│   │   └── SalesViewModel.swift
│   ├── HR/                         # HR 模組
│   │   ├── EmployeeListView.swift
│   │   ├── AttendanceView.swift
│   │   └── HRViewModel.swift
│   ├── Warehouse/                  # WM 模組
│   │   ├── WarehouseListView.swift
│   │   ├── StockCountView.swift
│   │   └── WarehouseViewModel.swift
│   └── Quality/                    # QM 模組
│       ├── InspectionListView.swift
│       ├── InspectionFormView.swift
│       └── QualityViewModel.swift
└── SharedViews/
    ├── EmptyStateView.swift
    ├── ERPCard.swift
    ├── LoadingView.swift
    ├── SearchField.swift
    └── StatusBadge.swift
```

---

## 後端 API 連線

```swift
// Core/Network/APIEndpoints.swift
enum APIEndpoints {
    static let baseURL = "http://localhost:8000/api/v1"

    // Auth
    static let login = "/auth/login"
    static let register = "/auth/register"
    static let refresh = "/auth/refresh"

    // MM
    static let materials = "/mm/materials"
    static func material(_ id: String) -> String { "/mm/materials/\(id)" }
    static let plantStock = "/mm/plant-stock"
    static let purchaseOrders = "/mm/purchase-orders"

    // SD
    static let salesOrders = "/sd/sales-orders"
    static let customers = "/sd/customers"

    // HR
    static let employees = "/hr/employees"
    static let attendance = "/hr/attendance"
    static let clockIn = "/hr/attendance/clock-in"
    static let clockOut = "/hr/attendance/clock-out"

    // WM
    static let warehouses = "/wm/warehouses"
    static let stockCounts = "/wm/stock-counts"

    // QM
    static let inspectionLots = "/qm/inspection-lots"
}
```

### APIClient (Singleton, URLSession)
```swift
// Core/Network/APIClient.swift
final class APIClient {
    static let shared = APIClient()

    func get<T: Decodable>(_ endpoint: String, queryItems: [URLQueryItem]? = nil) async throws -> APIResponse<T>
    func getPaginated<T: Decodable>(_ endpoint: String, page: Int, perPage: Int) async throws -> APIResponse<PaginatedResponse<T>>
    func post<T: Decodable, B: Encodable>(_ endpoint: String, body: B) async throws -> APIResponse<T>
    func put<T: Decodable, B: Encodable>(_ endpoint: String, body: B) async throws -> APIResponse<T>
    func delete<T: Decodable>(_ endpoint: String) async throws -> APIResponse<T>
}
```

**特點**:
- 自動 snake_case <-> camelCase 轉換
- 自動附帶 Bearer token (從 AuthManager 取得)
- 自訂日期解碼（支援 ISO8601 + fractional seconds + date-only）

---

## MVVM 架構模式

### ViewModel 定義
```swift
@MainActor
final class MaterialsViewModel: ObservableObject {
    @Published var materials: [Material] = []
    @Published var isLoading = false
    @Published var errorMessage: String?
    @Published var searchText = ""
    @Published var currentPage = 1
    @Published var totalPages = 1

    var filteredMaterials: [Material] {
        guard !searchText.isEmpty else { return materials }
        return materials.filter { /* ... */ }
    }

    func loadMaterials() async {
        isLoading = true
        errorMessage = nil
        do {
            let response: APIResponse<PaginatedResponse<Material>> = try await APIClient.shared.getPaginated(
                APIEndpoints.materials, page: currentPage, perPage: 50
            )
            if response.success, let paginated = response.data {
                materials = paginated.data
                totalPages = paginated.totalPages
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to load materials"
        }
        isLoading = false
    }
}
```

### View 使用
```swift
struct MaterialsListView: View {
    @StateObject private var viewModel = MaterialsViewModel()

    var body: some View {
        NavigationStack {
            Group {
                if viewModel.isLoading {
                    LoadingView()
                } else if viewModel.filteredMaterials.isEmpty {
                    EmptyStateView(title: "No Materials")
                } else {
                    List(viewModel.filteredMaterials) { material in
                        NavigationLink(destination: MaterialDetailView(material: material)) {
                            MaterialRow(material: material)
                        }
                    }
                }
            }
            .navigationTitle("Materials")
            .searchable(text: $viewModel.searchText)
            .task { await viewModel.loadMaterials() }
        }
    }
}
```

---

## 現場操作場景

### 1. 庫存盤點 (WM)
- 掃描條碼/QR Code 進行盤點
- 離線模式支援（無網路時暫存本地）
- 盤點差異自動標記

### 2. 出勤打卡 (HR)
- GPS 定位 + 打卡 (clock-in / clock-out)
- 加班/請假申請
- 班表查看

### 3. 品質檢驗 (QM)
- 檢驗清單 (Checklist)
- 拍照記錄缺陷
- 檢驗結果即時回報

### 4. 物料管理 (MM)
- 物料清單瀏覽與搜尋
- 物料詳情與庫存水位
- 採購訂單查看

### 5. 銷售管理 (SD)
- 銷售訂單清單與詳情
- 客戶資料查看

---

## 開發命令

```bash
# Xcode 開啟專案
open ios/TasteByteERP.xcodeproj

# 命令列建構 (如有 xcodebuild)
cd ios
xcodebuild -scheme TasteByteERP -destination 'platform=iOS Simulator,name=iPhone 16' build

# SwiftLint (如已安裝)
swiftlint lint --path ios/TasteByteERP/
```

---

## 程式碼規範

- 架構使用 MVVM：View (SwiftUI) + ViewModel (ObservableObject) + Model (Codable struct)
- ViewModel 標記 `@MainActor`，使用 `@Published` 屬性
- 網路層使用 URLSession + async/await，不使用第三方 HTTP 框架
- Token 使用 Keychain 安全儲存，不使用 UserDefaults
- JSON 解碼使用 `keyDecodingStrategy = .convertFromSnakeCase`
- 共用 UI 元件放在 `SharedViews/` 目錄
- 每個 Feature 模組包含 View + ViewModel
- 使用 `.task { }` 修飾符在 View 出現時載入資料
- 導航使用 NavigationStack（非 NavigationView）
