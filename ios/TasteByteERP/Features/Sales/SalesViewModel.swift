import Foundation

@MainActor
final class SalesViewModel: ObservableObject {
    @Published var salesOrders: [SalesOrder] = []
    @Published var customers: [Customer] = []
    @Published var isLoading = false
    @Published var errorMessage: String?
    @Published var searchText = ""
    @Published var currentPage = 1
    @Published var totalPages = 1
    @Published var selectedOrderItems: [SalesOrderItem] = []

    var filteredOrders: [SalesOrder] {
        guard !searchText.isEmpty else { return salesOrders }
        let query = searchText.lowercased()
        return salesOrders.filter {
            $0.orderNumber.lowercased().contains(query) ||
            $0.status.lowercased().contains(query) ||
            customerName(for: $0.customerId).lowercased().contains(query)
        }
    }

    func loadSalesOrders() async {
        isLoading = true
        errorMessage = nil

        do {
            let response: APIResponse<PaginatedResponse<SalesOrder>> = try await APIClient.shared.getPaginated(
                APIEndpoints.salesOrders,
                page: currentPage,
                perPage: 50
            )
            if response.success, let paginated = response.data {
                salesOrders = paginated.items
                totalPages = paginated.totalPages
            } else {
                errorMessage = response.error ?? "Failed to load sales orders"
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to load sales orders"
        }

        isLoading = false
    }

    func loadCustomers() async {
        do {
            let response: APIResponse<PaginatedResponse<Customer>> = try await APIClient.shared.getPaginated(
                APIEndpoints.customers,
                perPage: 100
            )
            if response.success, let paginated = response.data {
                customers = paginated.items
            }
        } catch {
            // Customers are supplementary data
        }
    }

    func loadOrderItems(orderId: String) async {
        do {
            let endpoint = APIEndpoints.salesOrder(orderId) + "/items"
            let response: APIResponse<[SalesOrderItem]> = try await APIClient.shared.get(endpoint)
            if response.success, let data = response.data {
                selectedOrderItems = data
            }
        } catch {
            // Items will show empty
        }
    }

    func confirmOrder(orderId: String) async {
        isLoading = true
        errorMessage = nil
        do {
            let _: APIResponse<SalesOrder> = try await APIClient.shared.postNoBody(
                APIEndpoints.salesOrderConfirm(orderId)
            )
            await loadSalesOrders()
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to confirm order"
        }
        isLoading = false
    }

    func customerName(for customerId: String) -> String {
        customers.first { $0.id == customerId }?.name ?? "Customer"
    }
}
