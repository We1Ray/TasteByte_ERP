import Foundation

struct DashboardKPIs: Codable {
    let totalRevenue: Double
    let totalOrderCount: Int
    let totalInventoryQuantity: Double
    let pendingProductionOrders: Int
    let openArAmount: Double
    let openApAmount: Double
}

struct RecentActivity: Codable, Identifiable {
    let id: String
    let type: String
    let description: String
    let timestamp: Date
    let referenceNumber: String?
}

@MainActor
final class DashboardViewModel: ObservableObject {
    @Published var kpis: DashboardKPIs?
    @Published var recentActivities: [RecentActivity] = []
    @Published var isLoading = false
    @Published var errorMessage: String?

    private let fallbackKPIs = DashboardKPIs(
        totalRevenue: 0,
        totalOrderCount: 0,
        totalInventoryQuantity: 0,
        pendingProductionOrders: 0,
        openArAmount: 0,
        openApAmount: 0
    )

    func loadDashboard() async {
        isLoading = true
        errorMessage = nil

        // Load KPIs
        do {
            let response: APIResponse<DashboardKPIs> = try await APIClient.shared.get(
                APIEndpoints.dashboardKPIs
            )
            if response.success, let data = response.data {
                kpis = data
            } else {
                kpis = fallbackKPIs
            }
        } catch {
            kpis = fallbackKPIs
        }

        // Load recent activity
        do {
            let response: APIResponse<[RecentActivity]> = try await APIClient.shared.get(
                APIEndpoints.dashboardRecent
            )
            if response.success, let data = response.data {
                recentActivities = data
            }
        } catch {
            // Recent activities are optional, no error shown
        }

        isLoading = false
    }
}
