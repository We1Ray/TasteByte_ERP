import Foundation

enum APIEndpoints {
    static let baseURL = "http://localhost:8000/api/v1"

    // Auth
    static let login = "/auth/login"
    static let register = "/auth/register"
    static let refresh = "/auth/refresh"
    static let logout = "/auth/logout"

    // Materials Management (MM)
    static let materials = "/mm/materials"
    static func material(_ id: String) -> String { "/mm/materials/\(id)" }
    static let materialGroups = "/mm/material-groups"
    static let uoms = "/mm/uoms"
    static let vendors = "/mm/vendors"
    static func vendor(_ id: String) -> String { "/mm/vendors/\(id)" }
    static let plantStock = "/mm/plant-stock"
    static let materialMovements = "/mm/material-movements"
    static let purchaseOrders = "/mm/purchase-orders"
    static func purchaseOrder(_ id: String) -> String { "/mm/purchase-orders/\(id)" }

    // Sales & Distribution (SD)
    static let customers = "/sd/customers"
    static func customer(_ id: String) -> String { "/sd/customers/\(id)" }
    static let salesOrders = "/sd/sales-orders"
    static func salesOrder(_ id: String) -> String { "/sd/sales-orders/\(id)" }
    static let deliveries = "/sd/deliveries"
    static func delivery(_ id: String) -> String { "/sd/deliveries/\(id)" }
    static let sdInvoices = "/sd/invoices"

    // Human Resources (HR)
    static let employees = "/hr/employees"
    static func employee(_ id: String) -> String { "/hr/employees/\(id)" }
    static let attendance = "/hr/attendance"
    static let clockIn = "/hr/attendance/clock-in"
    static let clockOut = "/hr/attendance/clock-out"
    static let attendanceToday = "/hr/attendance/today"

    // Warehouse Management (WM)
    static let warehouses = "/wm/warehouses"
    static func warehouse(_ id: String) -> String { "/wm/warehouses/\(id)" }
    static let stockCounts = "/wm/stock-counts"

    // Quality Management (QM)
    static let inspectionLots = "/qm/inspection-lots"
    static func inspectionLot(_ id: String) -> String { "/qm/inspection-lots/\(id)" }
    static func inspectionResults(_ lotId: String) -> String { "/qm/inspection-lots/\(lotId)/results" }

    // Purchase Orders
    static func purchaseOrderReceive(_ id: String) -> String { "/mm/purchase-orders/\(id)/receive" }

    // Sales Order Actions
    static func salesOrderConfirm(_ id: String) -> String { "/sd/sales-orders/\(id)/confirm" }

    // Production Planning (PP)
    static let productionOrders = "/pp/production-orders"
    static func productionOrder(_ id: String) -> String { "/pp/production-orders/\(id)" }
    static func productionOrderStatus(_ id: String) -> String { "/pp/production-orders/\(id)/status" }

    // Reports
    static let fiTrialBalance = "/fi/reports/trial-balance"
    static let fiIncomeStatement = "/fi/reports/income-statement"
    static let fiBalanceSheet = "/fi/reports/balance-sheet"
    static let fiArAging = "/fi/reports/ar-aging"
    static let fiApAging = "/fi/reports/ap-aging"
    static let mmStockValuation = "/mm/reports/stock-valuation"
    static let mmMovementSummary = "/mm/reports/movement-summary"
    static let mmSlowMoving = "/mm/reports/slow-moving"
    static let sdSalesSummary = "/sd/reports/sales-summary"
    static let sdOrderFulfillment = "/sd/reports/order-fulfillment"
    static let sdTopCustomers = "/sd/reports/top-customers"

    // Dashboard
    static let dashboardKPIs = "/dashboard/kpis"
    static let dashboardRecent = "/dashboard/recent-activity"
}
