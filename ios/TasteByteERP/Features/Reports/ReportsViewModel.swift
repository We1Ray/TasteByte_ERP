import Foundation

// MARK: - FI Report Models

struct TrialBalanceEntry: Codable, Identifiable {
    var id: String { accountNumber }
    let accountNumber: String
    let accountName: String
    let debit: Double
    let credit: Double
    let balance: Double
}

struct IncomeStatementEntry: Codable, Identifiable {
    var id: String { category }
    let category: String
    let amount: Double
}

struct BalanceSheetEntry: Codable, Identifiable {
    var id: String { category }
    let category: String
    let amount: Double
}

struct ArAgingEntry: Codable, Identifiable {
    var id: String { customerName }
    let customerName: String
    let current: Double
    let days30: Double
    let days60: Double
    let days90Plus: Double
    let total: Double
}

struct ApAgingEntry: Codable, Identifiable {
    var id: String { vendorName }
    let vendorName: String
    let current: Double
    let days30: Double
    let days60: Double
    let days90Plus: Double
    let total: Double
}

// MARK: - MM Report Models

struct StockValuationEntry: Codable, Identifiable {
    var id: String { materialNumber }
    let materialNumber: String
    let materialName: String
    let quantity: Double
    let unitCost: Double
    let totalValue: Double
}

struct MovementSummaryEntry: Codable, Identifiable {
    var id: String { movementType }
    let movementType: String
    let count: Int
    let totalQuantity: Double
}

struct SlowMovingEntry: Codable, Identifiable {
    var id: String { materialNumber }
    let materialNumber: String
    let materialName: String
    let lastMovementDate: Date?
    let quantity: Double
    let daysSinceLastMovement: Int
}

// MARK: - SD Report Models

struct SalesSummaryEntry: Codable, Identifiable {
    var id: String { period }
    let period: String
    let orderCount: Int
    let totalAmount: Double
}

struct OrderFulfillmentEntry: Codable, Identifiable {
    var id: String { status }
    let status: String
    let count: Int
    let percentage: Double
}

struct TopCustomerEntry: Codable, Identifiable {
    var id: String { customerName }
    let customerName: String
    let orderCount: Int
    let totalAmount: Double
}

// MARK: - ViewModel

@MainActor
final class ReportsViewModel: ObservableObject {
    @Published var isLoading = false
    @Published var errorMessage: String?

    // FI Reports
    @Published var trialBalance: [TrialBalanceEntry] = []
    @Published var incomeStatement: [IncomeStatementEntry] = []
    @Published var balanceSheet: [BalanceSheetEntry] = []
    @Published var arAging: [ArAgingEntry] = []
    @Published var apAging: [ApAgingEntry] = []

    // MM Reports
    @Published var stockValuation: [StockValuationEntry] = []
    @Published var movementSummary: [MovementSummaryEntry] = []
    @Published var slowMoving: [SlowMovingEntry] = []

    // SD Reports
    @Published var salesSummary: [SalesSummaryEntry] = []
    @Published var orderFulfillment: [OrderFulfillmentEntry] = []
    @Published var topCustomers: [TopCustomerEntry] = []

    // MARK: - FI Reports

    func loadTrialBalance() async {
        isLoading = true
        errorMessage = nil
        do {
            let response: APIResponse<[TrialBalanceEntry]> = try await APIClient.shared.get(APIEndpoints.fiTrialBalance)
            if response.success, let data = response.data {
                trialBalance = data
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to load trial balance"
        }
        isLoading = false
    }

    func loadIncomeStatement() async {
        isLoading = true
        errorMessage = nil
        do {
            let response: APIResponse<[IncomeStatementEntry]> = try await APIClient.shared.get(APIEndpoints.fiIncomeStatement)
            if response.success, let data = response.data {
                incomeStatement = data
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to load income statement"
        }
        isLoading = false
    }

    func loadBalanceSheet() async {
        isLoading = true
        errorMessage = nil
        do {
            let response: APIResponse<[BalanceSheetEntry]> = try await APIClient.shared.get(APIEndpoints.fiBalanceSheet)
            if response.success, let data = response.data {
                balanceSheet = data
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to load balance sheet"
        }
        isLoading = false
    }

    func loadArAging() async {
        isLoading = true
        errorMessage = nil
        do {
            let response: APIResponse<[ArAgingEntry]> = try await APIClient.shared.get(APIEndpoints.fiArAging)
            if response.success, let data = response.data {
                arAging = data
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to load AR aging"
        }
        isLoading = false
    }

    func loadApAging() async {
        isLoading = true
        errorMessage = nil
        do {
            let response: APIResponse<[ApAgingEntry]> = try await APIClient.shared.get(APIEndpoints.fiApAging)
            if response.success, let data = response.data {
                apAging = data
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to load AP aging"
        }
        isLoading = false
    }

    // MARK: - MM Reports

    func loadStockValuation() async {
        isLoading = true
        errorMessage = nil
        do {
            let response: APIResponse<[StockValuationEntry]> = try await APIClient.shared.get(APIEndpoints.mmStockValuation)
            if response.success, let data = response.data {
                stockValuation = data
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to load stock valuation"
        }
        isLoading = false
    }

    func loadMovementSummary() async {
        isLoading = true
        errorMessage = nil
        do {
            let response: APIResponse<[MovementSummaryEntry]> = try await APIClient.shared.get(APIEndpoints.mmMovementSummary)
            if response.success, let data = response.data {
                movementSummary = data
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to load movement summary"
        }
        isLoading = false
    }

    func loadSlowMoving() async {
        isLoading = true
        errorMessage = nil
        do {
            let response: APIResponse<[SlowMovingEntry]> = try await APIClient.shared.get(APIEndpoints.mmSlowMoving)
            if response.success, let data = response.data {
                slowMoving = data
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to load slow-moving report"
        }
        isLoading = false
    }

    // MARK: - SD Reports

    func loadSalesSummary() async {
        isLoading = true
        errorMessage = nil
        do {
            let response: APIResponse<[SalesSummaryEntry]> = try await APIClient.shared.get(APIEndpoints.sdSalesSummary)
            if response.success, let data = response.data {
                salesSummary = data
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to load sales summary"
        }
        isLoading = false
    }

    func loadOrderFulfillment() async {
        isLoading = true
        errorMessage = nil
        do {
            let response: APIResponse<[OrderFulfillmentEntry]> = try await APIClient.shared.get(APIEndpoints.sdOrderFulfillment)
            if response.success, let data = response.data {
                orderFulfillment = data
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to load order fulfillment"
        }
        isLoading = false
    }

    func loadTopCustomers() async {
        isLoading = true
        errorMessage = nil
        do {
            let response: APIResponse<[TopCustomerEntry]> = try await APIClient.shared.get(APIEndpoints.sdTopCustomers)
            if response.success, let data = response.data {
                topCustomers = data
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to load top customers"
        }
        isLoading = false
    }
}
